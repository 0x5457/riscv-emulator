#[macro_export]
macro_rules! init_insn {
    ($cpu:ident, $exception:ident) => {
        pub trait Format {
            fn rs1(&self) -> u32 {
                0
            }
            fn rs2(&self) -> u32 {
                0
            }
            fn rd(&self) -> u32 {
                0
            }
            fn imm(&self) -> u32 {
                0
            }
            fn op(&self) -> u32 {
                0
            }
            fn imm_len(&self) -> usize {
                0
            }
        }

        pub trait Executable: std::fmt::Display {
            fn exec(&self, cpu: &mut $cpu) -> Result<(), $exception>;
        }

        pub struct Insn(Box<dyn Executable>);

        impl Insn {
            pub fn new<T: 'static + Executable>(e: T) -> Self {
                Self(Box::new(e))
            }
            fn exec(&self, cpu: &mut $cpu) -> Result<(), $exception> {
                self.0.exec(cpu)
            }
        }

        impl std::fmt::Display for Insn {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        // fn -> (match_code, mask, insn_creator)
        #[distributed_slice]
        pub static INSN_SLICE: [fn() -> (u32, u32, fn(u32) -> Insn)] = [..];

        use std::collections::HashMap;

        pub struct InsnDecoder {
            // HashMap<opcode, vec<(match_code, mask, insn_creator)>>
            insn_map: HashMap<u32, Vec<(u32, u32, fn(u32) -> Insn)>>,
        }

        impl InsnDecoder {
            fn new() -> Self {
                let mut insn_map = HashMap::new();
                for f in INSN_SLICE.iter() {
                    let (match_code, mask, insn_fn) = f();
                    let opcode = match_code & 0x7f;
                    insn_map
                        .entry(opcode)
                        .and_modify(|v: &mut Vec<(u32, u32, fn(u32) -> Insn)>| {
                            v.push((match_code, mask, insn_fn));
                        })
                        .or_insert_with(|| vec![(match_code, mask, insn_fn)]);
                }
                Self { insn_map: insn_map }
            }

            fn decode(&self, code: u32) -> Option<Insn> {
                let opcode = code & 0x7f;
                if let Some(v) = self.insn_map.get(&opcode) {
                    for (match_code, mask, insn_fn) in v {
                        if code & mask == *match_code {
                            return Some(insn_fn(code));
                        }
                    }
                    None
                } else {
                    None
                }
            }
        }
    };
}
