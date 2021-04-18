use bit_field::BitField;

use crate::RegT;

/// medeleg register
#[derive(Clone, Copy, Debug)]
pub struct Medeleg {
    bits: RegT,
}
impl From<RegT> for Medeleg {
    fn from(r: RegT) -> Self {
        Self { bits: r }
    }
}

impl Medeleg {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> RegT {
        self.bits
    }

    /// Instruction Address Misaligned Delegate
    #[inline]
    pub fn instruction_misaligned(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Instruction Access Fault Delegate
    #[inline]
    pub fn instruction_fault(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// Illegal Instruction Delegate
    #[inline]
    pub fn illegal_instruction(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// Breakpoint Delegate
    #[inline]
    pub fn breakpoint(&self) -> bool {
        self.bits.get_bit(3)
    }

    /// Load Address Misaligned Delegate
    #[inline]
    pub fn load_misaligned(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Load Access Fault Delegate
    #[inline]
    pub fn load_fault(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// Store/AMO Address Misaligned Delegate
    #[inline]
    pub fn store_misaligned(&self) -> bool {
        self.bits.get_bit(6)
    }

    /// Store/AMO Access Fault Delegate
    #[inline]
    pub fn store_fault(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// Environment Call from U-mode Delegate
    #[inline]
    pub fn user_env_call(&self) -> bool {
        self.bits.get_bit(8)
    }

    /// Environment Call from S-mode Delegate
    #[inline]
    pub fn supervisor_env_call(&self) -> bool {
        self.bits.get_bit(9)
    }

    /// Environment Call from M-mode Delegate
    #[inline]
    pub fn machine_env_call(&self) -> bool {
        self.bits.get_bit(11)
    }

    /// Instruction Page Fault Delegate
    #[inline]
    pub fn instruction_page_fault(&self) -> bool {
        self.bits.get_bit(12)
    }

    /// Load Page Fault Delegate
    #[inline]
    pub fn load_page_fault(&self) -> bool {
        self.bits.get_bit(13)
    }

    /// Store/AMO Page Fault Delegate
    #[inline]
    pub fn store_page_fault(&self) -> bool {
        self.bits.get_bit(15)
    }
}
