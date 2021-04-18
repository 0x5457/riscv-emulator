use crate::RegT;

pub struct Xs {
    regs: [RegT; 32],
}

impl Xs {
    pub fn new() -> Self {
        Self { regs: [0; 32] }
    }
    // Id must be one of [0~32).
    pub fn reg(&self, id: u8) -> RegT {
        debug_assert!(id < 32, "Id must be one of [0~32). got: {}", id);
        if id == 0 {
            0
        } else {
            self.regs[id as usize]
        }
    }
    // Id must be one of [0~32).
    pub fn set_reg(&mut self, id: u8, value: RegT) {
        debug_assert!(id < 32, "Id must be one of [0~32). got: {}", id);
        if id != 0 {
            self.regs[id as usize] = value
        }
    }
}
