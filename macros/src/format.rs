#[macro_export]
macro_rules! def_insn {
  ($(#[$attr:meta])*, $name:ident) => {
      $(#[$attr])*
      pub struct $name{
          code: u32
      }
  };
}

#[macro_export]
macro_rules! impl_format {
    ($name:ident, R) => {
        impl Format for $name {
            fn op(&self) -> u32 {
                self.code & 0x7f
            }
            fn rd(&self) -> u32 {
                (self.code >> 7) & 0x1f
            }
            fn rs1(&self) -> u32 {
                (self.code >> 15) & 0x1f
            }
            fn rs2(&self) -> u32 {
                (self.code >> 20) & 0x1f
            }
        }
    };
    ($name:ident, I) => {
        impl Format for $name {
            fn op(&self) -> u32 {
                self.code & 0x7f
            }
            fn rd(&self) -> u32 {
                (self.code >> 7) & 0x1f
            }
            fn rs1(&self) -> u32 {
                (self.code >> 15) & 0x1f
            }
            fn imm(&self) -> u32 {
                (self.code >> 20) & 0xfff
            }
            fn imm_len(&self) -> usize {
                12
            }
        }
    };
    ($name:ident, S) => {
        impl Format for $name {
            fn op(&self) -> u32 {
                self.code & 0x7f
            }
            fn rs1(&self) -> u32 {
                (self.code >> 15) & 0x1f
            }
            fn rs2(&self) -> u32 {
                (self.code >> 20) & 0x1f
            }
            fn imm(&self) -> u32 {
                ((self.code >> 7) & 0x1f) | ((self.code >> 25) & 0x7f) << 5
            }

            fn imm_len(&self) -> usize {
                12
            }
        }
    };
    ($name:ident, B) => {
        impl Format for $name {
            fn op(&self) -> u32 {
                self.code & 0x7f
            }
            fn rs2(&self) -> u32 {
                (self.code >> 20) & 0x1f
            }
            fn rs1(&self) -> u32 {
                (self.code >> 15) & 0x1f
            }
            fn imm(&self) -> u32 {
                ((self.code >> 31) & 0x1) << 12
                    | ((self.code >> 7) & 0x1) << 11
                    | ((self.code >> 25) & 0x3f) << 5
                    | ((self.code >> 8) & 0xf) << 1
            }
            fn imm_len(&self) -> usize {
                13
            }
        }
    };
    ($name:ident, U) => {
        impl Format for $name {
            fn op(&self) -> u32 {
                self.code & 0x7f
            }
            fn rd(&self) -> u32 {
                (self.code >> 7) & 0x1f
            }
            fn imm(&self) -> u32 {
                (self.code >> 12) << 12
            }
            fn imm_len(&self) -> usize {
                32
            }
        }
    };
    ($name:ident, J) => {
        impl Format for $name {
            fn op(&self) -> u32 {
                self.code & 0x7f
            }
            fn rd(&self) -> u32 {
                (self.code >> 7) & 0x1f
            }
            fn imm(&self) -> u32 {
                ((self.code >> 31) & 0x1) << 20
                    | ((self.code >> 12) & 0xff) << 12
                    | ((self.code >> 20) & 0x1) << 11
                    | ((self.code >> 21) & 0x3ff) << 1
            }

            fn imm_len(&self) -> usize {
                21
            }
        }
    };
}
