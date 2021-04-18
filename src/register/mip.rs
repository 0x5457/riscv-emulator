use bit_field::BitField;

use crate::RegT;

/// mip register
#[derive(Clone, Copy, Debug)]
pub struct Mip {
    bits: RegT,
}

impl From<RegT> for Mip {
    fn from(r: RegT) -> Self {
        Self { bits: r }
    }
}

impl Mip {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> RegT {
        self.bits
    }

    #[inline]
    pub fn set_msoft(&mut self, msoft: bool) {
        self.bits.set_bit(3, msoft);
    }

    #[inline]
    pub fn set_mtimer(&mut self, mtimer: bool) {
        self.bits.set_bit(7, mtimer);
    }

    #[inline]
    pub fn set_sext(&mut self, sext: bool) {
        self.bits.set_bit(9, sext);
    }
    #[inline]
    pub fn set_mext(&mut self, mext: bool) {
        self.bits.set_bit(11, mext);
    }
    #[inline]
    pub fn set_ssoft(&mut self, ssoft: bool) {
        self.bits.set_bit(1, ssoft);
    }

    #[inline]
    pub fn set_stimer(&mut self, stimer: bool) {
        self.bits.set_bit(5, stimer);
    }

    /// User Software Interrupt Pending
    #[inline]
    pub fn usoft(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Supervisor Software Interrupt Pending
    #[inline]
    pub fn ssoft(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// Machine Software Interrupt Pending
    #[inline]
    pub fn msoft(&self) -> bool {
        self.bits.get_bit(3)
    }

    /// User Timer Interrupt Pending
    #[inline]
    pub fn utimer(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Supervisor Timer Interrupt Pending
    #[inline]
    pub fn stimer(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// Machine Timer Interrupt Pending
    #[inline]
    pub fn mtimer(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// User External Interrupt Pending
    #[inline]
    pub fn uext(&self) -> bool {
        self.bits.get_bit(8)
    }

    /// Supervisor External Interrupt Pending
    #[inline]
    pub fn sext(&self) -> bool {
        self.bits.get_bit(9)
    }

    /// Machine External Interrupt Pending
    #[inline]
    pub fn mext(&self) -> bool {
        self.bits.get_bit(11)
    }
}
