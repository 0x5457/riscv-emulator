use crate::{
    cpu::CpuStatus,
    device::{bus::Bus, Data, Device},
    page::{PageTableEnty, VirtualAddress},
    register::satp::Mode,
    trap::Exception,
    XLen,
};

/// Page size (4 KiB).
pub const PAGE_SIZE: u64 = 4 * 1024;

pub struct Mmu {
    pub bus: Bus,
    xlen: XLen,
}

impl Mmu {
    pub fn new(xlen: XLen, binary: Vec<u8>) -> Self {
        Self {
            bus: Bus::new(binary),
            xlen: xlen,
        }
    }

    pub fn load<T>(&self, state: &CpuStatus, addr: u64) -> Result<T, Exception>
    where
        T: Data,
        [(); <T as Data>::SIZE]: Sized,
    {
        self.bus
            .read::<T>(self.translate(state, addr, AccessType::LOAD)?)
    }

    pub fn store<T>(&mut self, state: &CpuStatus, addr: u64, value: T) -> Result<(), Exception>
    where
        T: Data,
        [(); <T as Data>::SIZE]: Sized,
    {
        self.bus
            .write::<T>(self.translate(state, addr, AccessType::STORE)?, value)
    }

    pub fn fetch(&self, state: &CpuStatus, addr: u64) -> Result<u32, Exception> {
        self.bus
            .read::<u32>(self.translate(state, addr, AccessType::FETCH)?)
    }

    fn translate(
        &self,
        state: &CpuStatus,
        addr: u64,
        a_type: AccessType,
    ) -> Result<u64, Exception> {
        let satp = state.csrs.satp();
        let mode = satp.mode(&self.xlen);

        if mode == Mode::Bare {
            return Ok(addr);
        }

        let mut page_table_addr = satp.ppn(&self.xlen) * PAGE_SIZE;
        let v_addr = VirtualAddress(addr);

        // page-table entry
        let mut pte: PageTableEnty;
        let vpos = v_addr.virtual_page_offsets(&mode);
        let mut idx = (vpos.len() - 1) as i8;

        let exception = match a_type {
            AccessType::LOAD => Exception::LoadPageFault,
            AccessType::STORE => Exception::StorePageFault,
            AccessType::FETCH => Exception::InstructionPageFault,
        };

        loop {
            pte = PageTableEnty(self.bus.read::<u64>(page_table_addr + vpos[idx as usize])?);

            if !pte.v() || (!pte.r() && pte.w()) {
                return Err(exception);
            }

            if pte.r() || pte.w() {
                // Find leaf PTE
                break;
            }

            idx -= 1;

            // next page-table addr
            page_table_addr = pte.ppn(&mode) * PAGE_SIZE;

            if idx < 0 {
                return Err(exception);
            }
        }

        match a_type {
            AccessType::LOAD if !pte.r() => Err(Exception::LoadPageFault),
            AccessType::STORE if !pte.w() => Err(Exception::StorePageFault),
            AccessType::FETCH if !pte.x() => Err(Exception::InstructionPageFault),
            _ => {
                let offset = v_addr.offset();
                let ppns = pte.ppns(&mode);

                match idx {
                    0 => Ok(pte.ppn(&mode) << 12 | offset),
                    // Huge page.
                    1 => match mode {
                        Mode::Sv32 => Ok((ppns[1] << 22) | (vpos[0] << 9) | offset),
                        Mode::Sv39 => {
                            Ok((ppns[2] << 30) | (ppns[1] << 21) | (vpos[0] << 9) | offset)
                        }
                        _ => unimplemented!(),
                    },
                    // Huge page. only sv39
                    2 => Ok((ppns[2] << 30) | (vpos[1] << 18) | (vpos[0] << 9) | offset),
                    _ => Err(exception),
                }
            }
        }
    }
}

enum AccessType {
    LOAD,
    STORE,
    FETCH,
}
