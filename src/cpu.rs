use std::rc::Rc;

use crate::{
    device::{
        uart::UART_IRQ,
        virtio::{Virtio, VIRTIO_IRQ},
        DRAM_BASE, DRAM_SIZE,
    },
    mmu::Mmu,
    register::mip::Mip,
    trap::{Exception, Interrupt, Trap},
    Insn, InsnDecoder, PrivilegeMode, RegT,
};
use lru::LruCache;

use crate::{
    register::{csrs::Csrs, xs::Xs},
    XLen,
};
pub struct Cpu {
    pub state: CpuStatus,
    pub mmu: Mmu,
    pub xlen: XLen,
    insn_decoder: InsnDecoderWithLru,
}

impl Cpu {
    pub fn new(xlen: XLen, binary: Vec<u8>, start_address: u64) -> Self {
        let mut cpu_status = CpuStatus::new(start_address);
        cpu_status.reset();
        Self {
            state: cpu_status,
            mmu: Mmu::new(xlen, binary),
            xlen: xlen,
            insn_decoder: InsnDecoderWithLru::new(InsnDecoder::new()),
        }
    }

    pub fn setup_disk(&mut self, disk_img: Vec<u8>) {
        self.mmu.bus.virtio.initialize(disk_img);
    }

    pub fn one_step(&mut self) {
        if let Err(trap) = self.exec() {
            if let Trap::Exception(e) = trap {
                if e.is_fatal() {
                    panic!("{:?}", e);
                }
            }
            self.handle_trap(trap);
        }
        self.increment();
    }

    fn increment(&mut self) {
        // Increment the timer register (mtimer) in Clint.
        self.mmu.bus.clint.increment(&mut self.state);
        // Increment the value in the TIME register.
        let time = self.state.csrs.time();
        self.state.csrs.set_time(time.wrapping_add(1));
    }

    fn exec(&mut self) -> Result<(), Trap> {
        let code = self.fetch()?;
        let insn = self.decode(code)?;
        if let Some(interrupt) = self.take_interrupt() {
            return Err(interrupt.into());
        }
        insn.exec(self)?;
        Ok(())
    }

    fn fetch(&self) -> Result<u32, Exception> {
        let pc = self.state.pc;
        self.mmu.fetch(&self.state, pc)
    }

    fn decode(&mut self, code: u32) -> Result<Rc<Insn>, Exception> {
        let insn = self.insn_decoder.decode(code);
        insn.ok_or_else(|| {
            println!("IllegalInstruction code: {:x}", code);
            Exception::IllegalInstruction
        })
    }

    fn handle_trap(&mut self, trap: Trap) {
        let csrs = &mut self.state.csrs;
        let (deleg, mut cause, is_interrupt) = match trap {
            Trap::Interrupt(i) => (csrs.mideleg().bits(), i.code(), true),
            Trap::Exception(e) => (csrs.medeleg().bits(), e.code(), false),
        };

        if is_interrupt {
            cause = (1 << (self.xlen.len() - 1)) | cause;
        }

        let next_privilege =
            if self.state.privilege != PrivilegeMode::Machine && (deleg >> cause) & 1 == 1 {
                // deleg to s-mode
                PrivilegeMode::Supervisor
            } else {
                PrivilegeMode::Machine
            };

        let xtvec = match next_privilege {
            PrivilegeMode::Supervisor => {
                csrs.set_sepc(self.state.pc);
                csrs.set_scause(cause);
                csrs.set_stval(0);

                let mut sstatus = csrs.sstatus();
                // Set a privious interrupt-enable bit for supervisor mode (SPIE, 5) to the value
                // of a global interrupt-enable bit for supervisor mode (SIE, 1).
                sstatus.set_spie(sstatus.sie());
                // Set a global interrupt-enable bit for supervisor mode (SIE, 1) to 0.
                sstatus.set_sie(false);
                //SPP is set to 0 if the trap originated from user mode, or 1 otherwise."
                sstatus.set_spp(self.state.privilege);
                csrs.set_sstatus(sstatus.bits());
                csrs.stvec()
            }
            PrivilegeMode::Machine => {
                csrs.set_mepc(self.state.pc);
                csrs.set_mcause(cause);
                csrs.set_mtval(0);

                let mut mstatus = csrs.mstatus();
                // Set a privious interrupt-enable bit for supervisor mode (MPIE, 7) to the value
                // of a global interrupt-enable bit for supervisor mode (MIE, 3).
                mstatus.set_mpie(mstatus.mie());
                // Set a global interrupt-enable bit for supervisor mode (MIE, 3) to 0.
                mstatus.set_mie(false);
                // Set a privious privilege mode for supervisor mode (MPP, 11..13).
                mstatus.set_mpp(self.state.privilege);
                csrs.set_mstatus(mstatus.bits());
                csrs.mtvec()
            }
            _ => unreachable!(),
        };

        let trap_pc = xtvec
            .trap_mode()
            .trap_pc(xtvec.address(), cause, is_interrupt);

        self.state.update_pc(trap_pc);
        self.state.privilege = next_privilege;
    }

    fn take_interrupt(&mut self) -> Option<Interrupt> {
        // 检查中断使能
        match self.state.privilege {
            PrivilegeMode::Supervisor => {
                if !self.state.csrs.sstatus().sie() {
                    return None;
                }
            }
            PrivilegeMode::Machine => {
                if !self.state.csrs.mstatus().mie() {
                    return None;
                }
            }
            _ => {}
        }

        self.check_external_interrupts();

        let mip = self.state.csrs.mip();
        let mie = self.state.csrs.mie();
        let pendings = mip.bits() & mie.bits();
        let interrupts = Mip::from(pendings);
        let mut mip = self.state.csrs.mip();

        let ret = if interrupts.mext() {
            mip.set_mext(false);
            Some(Interrupt::MachineExternal)
        } else if interrupts.msoft() {
            mip.set_msoft(false);
            Some(Interrupt::MachineSoft)
        } else if interrupts.mtimer() {
            mip.set_mtimer(false);
            Some(Interrupt::MachineTimer)
        } else if interrupts.sext() {
            mip.set_sext(false);
            Some(Interrupt::SupervisorExternal)
        } else if interrupts.ssoft() {
            mip.set_ssoft(false);
            Some(Interrupt::SupervisorSoft)
        } else if interrupts.stimer() {
            mip.set_stimer(false);
            Some(Interrupt::SupervisorTimer)
        } else {
            None
        };
        self.state.csrs.set_mip(mip.bits());
        ret
    }

    fn check_external_interrupts(&mut self) {
        let irq = if self.mmu.bus.uart.is_interrupting() {
            UART_IRQ
        } else if self.mmu.bus.virtio.is_interrupting() {
            // An interrupt is raised after a disk access is done.
            Virtio::disk_access(&mut self.mmu.bus).expect("failed to access the disk");
            VIRTIO_IRQ
        } else {
            0
        };
        if irq != 0 {
            self.mmu.bus.plic.update_pending(irq);
            let mut mip = self.state.csrs.mip();
            mip.set_sext(true);
            self.state.csrs.set_mip(mip.bits());
        }
    }
}

pub struct CpuStatus {
    pub privilege: PrivilegeMode,
    pub xs: Xs,
    pub csrs: Csrs,
    pub pc: RegT,
}

impl CpuStatus {
    fn new(start_address: u64) -> Self {
        Self {
            privilege: PrivilegeMode::Machine,
            xs: Xs::new(),
            csrs: Csrs::new(),
            pc: start_address,
        }
    }

    fn reset(&mut self) {
        // The stack pointer (SP) must be set up at first.;
        self.xs.set_reg(2, DRAM_BASE + DRAM_SIZE as u64);
        self.privilege = PrivilegeMode::Machine;
    }

    pub fn update_pc(&mut self, value: RegT) {
        self.pc = value;
    }
}

struct InsnDecoderWithLru {
    inner: InsnDecoder,
    cache: LruCache<u32, Option<Rc<Insn>>>,
}

impl InsnDecoderWithLru {
    fn new(insn_decoder: InsnDecoder) -> Self {
        Self {
            inner: insn_decoder,
            cache: LruCache::new(127),
        }
    }
    fn decode(&mut self, code: u32) -> Option<Rc<Insn>> {
        match self.cache.get(&code) {
            Some(insn) => insn.clone(),
            None => {
                let insn = self.inner.decode(code).map(|insn| Rc::new(insn));
                self.cache.put(code, insn.clone());
                insn
            }
        }
    }
}
