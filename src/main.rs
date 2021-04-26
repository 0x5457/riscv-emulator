#![feature(const_generics, const_evaluatable_checked)]
#![allow(incomplete_features)]

use std::{
    env,
    fs::File,
    io::{self, Read},
};

use cpu::Cpu;
use trap::Exception;

mod cpu;
mod device;
mod isa;
mod mmu;
mod page;
mod register;
mod trap;

#[macro_use]
extern crate macros;

pub type RegT = u64;
pub type SRegT = i64;

#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
pub enum PrivilegeMode {
    User = 0,
    Supervisor = 1,
    Machine = 2,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum XLen {
    X32 = 32,
    X64 = 64,
}

impl XLen {
    pub const fn len(&self) -> usize {
        match self {
            XLen::X32 => 32,
            XLen::X64 => 64,
        }
    }

    pub const fn size(&self) -> usize {
        self.len() >> 3
    }

    pub const fn mask(&self) -> RegT {
        match self {
            // 0xffffffff
            XLen::X32 => ((1 as RegT) << (self.len() as RegT)) - 1,
            // 0xffffffffffffffff
            XLen::X64 => -1i64 as RegT,
        }
    }
}

init_insn!(Cpu, Exception);

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if (args.len() != 2) && (args.len() != 3) {
        panic!("Usage: riscv-emulator <filename> [image]");
    }
    let mut file = File::open(&args[1])?;
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new(XLen::X64, binary, device::DRAM_BASE);

    if args.len() == 3 {
        let mut disk_image = Vec::new();
        let mut file = File::open(&args[2])?;
        file.read_to_end(&mut disk_image)?;
        cpu.setup_disk(disk_image);
    }

    loop {
        cpu.one_step();
    }
}
