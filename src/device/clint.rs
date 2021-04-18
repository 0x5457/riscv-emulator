use std::convert::TryInto;

use crate::{cpu::CpuStatus, trap::Exception};

use super::{Data, Device, CLINT_BASE};

/// The address that a msip register starts. A msip is a machine mode software interrupt pending
/// register, used to assert a software interrupt for a CPU.
const MSIP: u64 = CLINT_BASE;
/// The address that a msip register ends. `msip` is a 4-byte register.
const MSIP_END: u64 = MSIP + 0x4;

/// The address that a mtimecmp register starts. A mtimecmp is a memory mapped machine mode timer
/// compare register, used to trigger an interrupt when mtimecmp is greater than or equal to mtime.
const MTIMECMP: u64 = CLINT_BASE + 0x4000;
/// The address that a mtimecmp register ends. `mtimecmp` is a 8-byte register.
const MTIMECMP_END: u64 = MTIMECMP + 0x8;

/// The address that a timer register starts. A mtime is a machine mode timer register which runs
/// at a constant frequency.
const MTIME: u64 = CLINT_BASE + 0xbff8;
/// The address that a timer register ends. `mtime` is a 8-byte register.
const MTIME_END: u64 = MTIME + 0x8;

/// The core-local interruptor (CLINT).
pub struct Clint {
    /// Machine mode software interrupt pending register, used to assert a software interrupt for
    /// a CPU.
    msip: u32,
    /// Memory mapped machine mode timer compare register, used to trigger an interrupt when
    /// mtimecmp is greater than or equal to mtime. There is an mtimecmp dedicated to each CPU.
    mtimecmp: u64,
    /// Machine mode timer register which runs at a constant frequency.
    mtime: u64,
}
impl Device for Clint {
    fn read<T>(&self, addr: u64) -> Result<T, Exception>
    where
        T: Data,
        [(); <T as Data>::SIZE]: Sized,
    {
        // `reg` is the value of a target register in CLINT and `offset` is the byte of the start
        // position in the register.
        let (reg, offset) = match addr {
            MSIP..=MSIP_END => (self.msip as u64, addr - MSIP),
            MTIMECMP..=MTIMECMP_END => (self.mtimecmp, addr - MTIMECMP),
            MTIME..=MTIME_END => (self.mtime, addr - MTIME),
            _ => return Err(Exception::LoadFault),
        };
        let bytes = (reg >> (offset * 8)).to_le_bytes();
        let bytes: [u8; T::SIZE] = bytes[bytes.len() - T::SIZE..]
            .try_into()
            .map_err(|_| Exception::LoadFault)?;
        Ok(T::from_bytes(bytes))
    }

    fn write<T>(&mut self, addr: u64, value: T) -> Result<(), Exception>
    where
        T: Data,
        [(); <T as Data>::SIZE]: Sized,
    {
        // `reg` is the value of a target register in CLINT and `offset` is the byte of the start
        // position in the register.
        let (reg, offset) = match addr {
            MSIP..=MSIP_END => (self.msip as u64, addr - MSIP),
            MTIMECMP..=MTIMECMP_END => (self.mtimecmp, addr - MTIMECMP),
            MTIME..=MTIME_END => (self.mtime, addr - MTIME),
            _ => return Err(Exception::StoreFault),
        };
        let bytes = value.to_bytes();
        // Store the new value to the target register.
        let mut origin_bytes = reg.to_le_bytes();
        for (idx, bit) in bytes.iter().enumerate() {
            origin_bytes[offset as usize + idx] = *bit;
        }
        let reg = u64::from_le_bytes(origin_bytes);

        match addr {
            MSIP..=MSIP_END => self.msip = reg as u32,
            MTIMECMP..=MTIMECMP_END => self.mtimecmp = reg,
            MTIME..=MTIME_END => self.mtime = reg,
            _ => return Err(Exception::StoreFault),
        }
        Ok(())
    }
}

impl Clint {
    pub fn new() -> Self {
        Self {
            msip: 0,
            mtime: 0,
            mtimecmp: 0,
        }
    }
    /// Increment the mtimer register. It's not a real-time value. The MTIP bit (MIP, 7) is enabled
    /// when `mtime` is greater than or equal to `mtimecmp`.
    pub fn increment(&mut self, state: &mut CpuStatus) {
        self.mtime = self.mtime.wrapping_add(1);
        let mut mip = state.csrs.mip();
        if (self.msip & 1) != 0 {
            // Enable the MSIP bit (MIP, 3).
            mip.set_msoft(true);
        }

        // 3.1.10 Machine Timer Registers (mtime and mtimecmp)
        // "The interrupt remains posted until mtimecmp becomes greater than mtime (typically as a
        // result of writing mtimecmp)."
        if self.mtimecmp > self.mtime {
            // Clear the MTIP bit (MIP, 7).
            mip.set_mtimer(false);
        }
        // 3.1.10 Machine Timer Registers (mtime and mtimecmp)
        // "A timer interrupt becomes pending whenever mtime contains a value greater than or equal
        // to mtimecmp, treating the values as unsigned integers."
        if self.mtime >= self.mtimecmp {
            // Enable the MTIP bit (MIP, 7).
            mip.set_mtimer(true);
        }
        state.csrs.set_mip(mip.bits());
    }
}
