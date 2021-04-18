use crate::RegT;

/// mtvec register
#[derive(Clone, Copy, Debug)]
pub struct Xtvec {
    bits: RegT,
}

impl From<RegT> for Xtvec {
    fn from(r: RegT) -> Self {
        Self { bits: r }
    }
}
/// Trap mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrapMode {
    Direct = 0,
    Vectored = 1,
}

impl TrapMode {
    pub fn trap_pc(&self, base: RegT, cause: RegT, is_interrupt: bool) -> RegT {
        let offset = if is_interrupt && &TrapMode::Vectored == self {
            4 * cause
        } else {
            0
        };
        base + offset
    }
}

impl Xtvec {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> RegT {
        self.bits
    }

    /// Returns the trap-vector base-address
    pub fn address(&self) -> RegT {
        self.bits - (self.bits & 0b11)
    }

    /// Returns the trap-vector mode
    pub fn trap_mode(&self) -> TrapMode {
        let mode = self.bits & 0b11;
        if mode == 0 {
            TrapMode::Direct
        } else {
            TrapMode::Vectored
        }
    }
}
