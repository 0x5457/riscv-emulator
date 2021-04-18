use crate::{RegT, XLen};

mod rva;
mod rvi;
mod rvm;

pub const fn reg_len() -> usize {
    std::mem::size_of::<RegT>() << 3
}

pub fn sext(value: RegT, len: usize) -> RegT {
    let bit_len = reg_len();
    assert!(len > 0 && len <= bit_len);
    if len == bit_len {
        return value;
    }
    let sign = value >> (len - 1) as RegT & 0x1;
    let mask = ((1 as RegT) << (len as RegT)) - 1 as RegT;
    if sign == 0 {
        value & mask
    } else {
        let high = (((1 as RegT) << (bit_len as RegT - len as RegT)) - 1 as RegT) << (len as RegT);
        value & mask | high
    }
}

impl XLen {
    fn shamt_mask(&self) -> u32 {
        match self {
            XLen::X32 => 0x1f,
            XLen::X64 => 0x3f,
        }
    }
}
