use bit_field::BitField;

use crate::{PrivilegeMode, RegT};
/// mstatus register
#[derive(Clone, Copy, Debug)]
pub struct Mstatus {
    bits: RegT,
}

impl From<RegT> for Mstatus {
    fn from(r: RegT) -> Self {
        Self { bits: r }
    }
}

impl Mstatus {
    pub fn bits(&self) -> RegT {
        self.bits
    }

    pub fn set_mie(&mut self, mie: bool) {
        self.bits.set_bit(3, mie);
    }

    pub fn set_mpie(&mut self, mpie: bool) {
        self.bits.set_bit(7, mpie);
    }

    pub fn set_mpp(&mut self, pm: PrivilegeMode) {
        self.bits.set_bits(11..13, pm as RegT);
    }

    /// User Interrupt Enable
    #[inline]
    pub fn uie(&self) -> bool {
        self.bits.get_bit(0)
    }
    /// Supervisor Interrupt Enable
    #[inline]
    pub fn sie(&self) -> bool {
        self.bits.get_bit(1)
    }
    /// Machine Interrupt Enable
    #[inline]
    pub fn mie(&self) -> bool {
        self.bits.get_bit(3)
    }

    /// User Previous Interrupt Enable
    #[inline]
    pub fn upie(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Supervisor Previous Interrupt Enable
    #[inline]
    pub fn spie(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// Machine Previous Interrupt Enable
    #[inline]
    pub fn mpie(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> PrivilegeMode {
        match self.bits.get_bit(8) {
            true => PrivilegeMode::Supervisor,
            false => PrivilegeMode::User,
        }
    }

    /// Machine Previous Privilege Mode
    #[inline]
    pub fn mpp(&self) -> PrivilegeMode {
        match self.bits.get_bits(11..13) {
            0b00 => PrivilegeMode::User,
            0b01 => PrivilegeMode::Supervisor,
            0b11 => PrivilegeMode::Machine,
            _ => unreachable!(),
        }
    }
    /// Permit Supervisor User Memory access
    #[inline]
    pub fn sum(&self) -> bool {
        self.bits.get_bit(18)
    }

    /// Make eXecutable Readable
    #[inline]
    pub fn mxr(&self) -> bool {
        self.bits.get_bit(19)
    }

    /// Trap Virtual Memory
    ///
    /// If this bit is set, reads or writes to `satp` CSR or execute `sfence.vma`
    /// instruction when in S-mode will raise an illegal instruction exception.
    ///
    /// TVM is hard-wired to 0 when S-mode is not supported.
    #[inline]
    pub fn tvm(&self) -> bool {
        self.bits.get_bit(20)
    }

    /// Timeout Wait
    ///
    /// Indicates that if WFI instruction should be intercepted.
    ///
    /// If this bit is set, when WFI is executed in S-mode, and it does not complete
    /// within an implementation specific, bounded time limit, the WFI instruction will cause
    /// an illegal instruction trap; or could always cause trap then the time limit is zero.
    ///
    /// TW is hard-wired to 0 when S-mode is not supported.
    #[inline]
    pub fn tw(&self) -> bool {
        self.bits.get_bit(21)
    }

    /// Trap SRET
    ///
    /// Indicates that if SRET instruction should be trapped to raise illegal
    /// instruction exception.
    ///
    /// If S-mode is not supported, TSR bit is hard-wired to 0.
    #[inline]
    pub fn tsr(&self) -> bool {
        self.bits.get_bit(22)
    }

    /*
        FIXME: There are MBE and SBE bits in 1.12; once Privileged Specification version 1.12
        is ratified, there should be read functions of these bits as well.
    */

    /// Whether either the FS field or XS field
    /// signals the presence of some dirty state
    #[inline]
    pub fn sd(&self) -> bool {
        self.bits.get_bit(std::mem::size_of::<usize>() * 8 - 1)
    }
}
