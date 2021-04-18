use crate::{RegT, XLen};
use bit_field::BitField;

/// satp register
#[derive(Clone, Copy, Debug)]
pub struct Satp {
    bits: RegT,
}

impl From<RegT> for Satp {
    fn from(r: RegT) -> Self {
        Self { bits: r }
    }
}

impl Satp {
    /// Current address-translation scheme
    #[inline]
    pub fn mode(&self, xlen: &XLen) -> Mode {
        let mode = match xlen {
            XLen::X32 => self.bits.get_bit(31) as u64,
            XLen::X64 => self.bits.get_bits(60..64),
        };
        match mode {
            0 => Mode::Bare,
            1 => Mode::Sv32,
            8 => Mode::Sv39,
            9 => Mode::Sv48,
            10 => Mode::Sv57,
            11 => Mode::Sv64,
            _ => unreachable!(),
        }
    }

    /// Address space identifier
    #[inline]
    pub fn asid(&self, xlen: &XLen) -> u64 {
        match xlen {
            XLen::X32 => self.bits.get_bits(22..31),
            XLen::X64 => self.bits.get_bits(44..60),
        }
    }

    /// Physical page number
    #[inline]
    pub fn ppn(&self, xlen: &XLen) -> u64 {
        match xlen {
            XLen::X32 => self.bits.get_bits(0..22),
            XLen::X64 => self.bits.get_bits(0..44),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// No translation or protection
    Bare = 0,
    /// Page-based 32-bit virtual addressing
    Sv32 = 1,
    /// Page-based 39-bit virtual addressing
    Sv39 = 8,
    /// Page-based 48-bit virtual addressing
    Sv48 = 9,
    /// Page-based 57-bit virtual addressing
    Sv57 = 10,
    /// Page-based 64-bit virtual addressing
    Sv64 = 11,
}
