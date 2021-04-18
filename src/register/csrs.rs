use crate::RegT;

use super::{
    medeleg::Medeleg, mideleg::Mideleg, mie::Mie, mip::Mip, mstatus::Mstatus, satp::Satp,
    sstatus::Sstatus, xtvec::Xtvec,
};

macro_rules! csr {
    ($fnname:ident, $csr_num:expr, $register:ty) => {
        pub fn $fnname(&self) -> $register {
            self.csrs[$csr_num].into()
        }
    };
    ($fnname:ident, $set_fnname:ident, $csr_num:expr, $register:ty) => {
        csr!($fnname, $csr_num, $register);
        pub fn $set_fnname(&mut self, value: RegT) {
            self.set_csr($csr_num, value);
        }
    };

    ($fnname:ident, $set_fnname:ident, $csr_num:expr) => {
        pub fn $fnname(&self) -> RegT {
            self.csrs[$csr_num].into()
        }

        pub fn $set_fnname(&mut self, value: RegT) {
            self.set_csr($csr_num, value);
        }
    };
}

pub struct Csrs {
    /// Control and status registers. RISC-V ISA sets aside a 12-bit encoding space (csr[11:0]) for
    /// up to 4096 CSRs.
    csrs: [RegT; 4096],
}

impl Csrs {
    pub fn new() -> Self {
        Self { csrs: [0; 4096] }
    }

    pub fn csr(&self, csr_num: u16) -> RegT {
        debug_assert!(
            csr_num < 4096,
            "csr_num must be one of [0~32). got: {}",
            csr_num
        );
        self.csrs[csr_num as usize]
    }

    pub fn set_csr(&mut self, csr_num: u16, value: RegT) {
        debug_assert!(
            csr_num < 4096,
            "csr_num must be one of [0~32). got: {}",
            csr_num
        );
        match csr_num {
            0x104 => {
                // SIE
                let mideleg = self.mideleg().bits();
                let mie = self.mie().bits();
                self.set_mie((mie & !mideleg) | (value & mideleg));
            }
            _ => self.csrs[csr_num as usize] = value,
        }
    }

    csr!(satp, set_satp, 0x180, Satp);
    csr!(sstatus, set_sstatus, 0x100, Sstatus);
    csr!(mstatus, set_mstatus, 0x300, Mstatus);
    csr!(mip, set_mip, 0x344, Mip);
    csr!(mie, set_mie, 0x304, Mie);
    csr!(mideleg, set_mideleg, 0x303, Mideleg);
    csr!(medeleg, set_medeleg, 0x302, Medeleg);
    csr!(mtvec, set_mtvec, 0x305, Xtvec);
    csr!(stvec, set_stvec, 0x105, Xtvec);
    csr!(mtval, set_mtval, 0x343);
    csr!(stval, set_stval, 0x143);
    csr!(sepc, set_sepc, 0x141);
    csr!(scause, set_scause, 0x142);
    csr!(mepc, set_mepc, 0x341);
    csr!(mcause, set_mcause, 0x342);
    csr!(time, set_time, 0xc01);
}
