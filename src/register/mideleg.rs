use bit_field::BitField;

use crate::RegT;

/// mideleg register
#[derive(Clone, Copy, Debug)]
pub struct Mideleg {
    bits: RegT,
}
impl From<RegT> for Mideleg {
    fn from(r: RegT) -> Self {
        Self { bits: r }
    }
}

impl Mideleg {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> RegT {
        self.bits
    }

    /// User Software Interrupt Delegate
    #[inline]
    pub fn usoft(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Supervisor Software Interrupt Delegate
    #[inline]
    pub fn ssoft(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// User Timer Interrupt Delegate
    #[inline]
    pub fn utimer(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Supervisor Timer Interrupt Delegate
    #[inline]
    pub fn stimer(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// User External Interrupt Delegate
    #[inline]
    pub fn uext(&self) -> bool {
        self.bits.get_bit(8)
    }

    /// Supervisor External Interrupt Delegate
    #[inline]
    pub fn sext(&self) -> bool {
        self.bits.get_bit(9)
    }
}
