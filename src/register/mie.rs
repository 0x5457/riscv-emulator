use bit_field::BitField;

use crate::RegT;

/// mie register
#[derive(Clone, Copy, Debug)]
pub struct Mie {
    bits: RegT,
}

impl From<RegT> for Mie {
    fn from(r: RegT) -> Self {
        Self { bits: r }
    }
}

impl Mie {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> RegT {
        self.bits
    }

    /// User Software Interrupt Enable
    #[inline]
    pub fn usoft(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Supervisor Software Interrupt Enable
    #[inline]
    pub fn ssoft(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// Machine Software Interrupt Enable
    #[inline]
    pub fn msoft(&self) -> bool {
        self.bits.get_bit(3)
    }

    /// User Timer Interrupt Enable
    #[inline]
    pub fn utimer(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Supervisor Timer Interrupt Enable
    #[inline]
    pub fn stimer(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// Machine Timer Interrupt Enable
    #[inline]
    pub fn mtimer(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// User External Interrupt Enable
    #[inline]
    pub fn uext(&self) -> bool {
        self.bits.get_bit(8)
    }

    /// Supervisor External Interrupt Enable
    #[inline]
    pub fn sext(&self) -> bool {
        self.bits.get_bit(9)
    }

    /// Machine External Interrupt Enable
    #[inline]
    pub fn mext(&self) -> bool {
        self.bits.get_bit(11)
    }
}
