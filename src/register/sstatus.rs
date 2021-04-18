use crate::{PrivilegeMode, RegT};
use bit_field::BitField;

/// Supervisor Status Register
#[derive(Clone, Copy, Debug)]
pub struct Sstatus {
    bits: RegT,
}

impl From<RegT> for Sstatus {
    fn from(r: RegT) -> Self {
        Self { bits: r }
    }
}

impl Sstatus {
    pub fn bits(&self) -> RegT {
        self.bits
    }

    pub fn set_sie(&mut self, sie: bool) {
        self.bits.set_bit(1, sie);
    }

    pub fn set_spie(&mut self, spie: bool) {
        self.bits.set_bit(5, spie);
    }

    pub fn set_spp(&mut self, pm: PrivilegeMode) {
        if pm == PrivilegeMode::User {
            self.bits.set_bit(8, false);
        } else {
            self.bits.set_bit(8, true);
        }
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

    /// Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> PrivilegeMode {
        match self.bits.get_bit(8) {
            true => PrivilegeMode::Supervisor,
            false => PrivilegeMode::User,
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

    /// Whether either the FS field or XS field
    /// signals the presence of some dirty state
    #[inline]
    pub fn sd(&self) -> bool {
        self.bits.get_bit(std::mem::size_of::<usize>() * 8 - 1)
    }
}
