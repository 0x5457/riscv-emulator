use crate::trap::Exception;

// use crate::trap::Exception;

pub mod bus;
pub mod clint;
mod memory;
pub mod plic;
pub mod uart;
pub mod virtio;

/// Default dram base.
pub const DRAM_BASE: u64 = 0x80000000;
/// Default dram size (128MiB).
pub const DRAM_SIZE: usize = 128 * 1024 * 1024;
/// The address which DRAM ends.
const DRAM_END: u64 = DRAM_BASE + (DRAM_SIZE as u64);

/// The  start address of CLINT.
pub const CLINT_BASE: u64 = 0x200_0000;
/// The address which the core-local interruptor (CLINT) ends.
const CLINT_END: u64 = CLINT_BASE + 0x10000;

// The  start address of PLIC.
pub const PLIC_BASE: u64 = 0xc00_0000;
/// The address which the platform-level interrupt controller (PLIC) ends.
const PLIC_END: u64 = PLIC_BASE + 0x208000;

/// The address which UART starts, same as QEMU virt machine.
pub const UART_BASE: u64 = 0x1000_0000;
/// The size of UART.
pub const UART_SIZE: u64 = 0x100;
/// The address which UART ends.
const UART_END: u64 = UART_BASE + 0x100;

/// The address which virtio starts.
pub const VIRTIO_BASE: u64 = 0x1000_1000;
/// The address which virtio ends.
const VIRTIO_END: u64 = VIRTIO_BASE + 0x1000;

pub trait Device {
    fn read<T>(&self, addr: u64) -> Result<T, Exception>
    where
        T: Data,
        [(); <T as Data>::SIZE]: Sized;

    fn write<T>(&mut self, addr: u64, value: T) -> Result<(), Exception>
    where
        T: Data,
        [(); <T as Data>::SIZE]: Sized;
}

pub trait Data {
    const SIZE: usize;
    fn from_bytes(bytes: [u8; Self::SIZE]) -> Self;
    fn to_bytes(self) -> [u8; Self::SIZE];
    fn from_u8(v: u8) -> Self;
    fn from_u16(v: u16) -> Self;
    fn from_u32(v: u32) -> Self;
    fn from_u64(v: u64) -> Self;
    fn to_u8(self) -> u8;
    fn to_u16(self) -> u16;
    fn to_u32(self) -> u32;
    fn to_u64(self) -> u64;
}

macro_rules! data_impl {
    ($($x:ty),*) => {
        $(impl Data for $x {
            const SIZE: usize = std::mem::size_of::<$x>();
            fn from_bytes(bytes: [u8; Self::SIZE]) ->  Self {
                Self::from_le_bytes(bytes)
            }
            fn to_bytes(self) -> [u8; Self::SIZE] {
                self.to_le_bytes()
            }
            fn from_u8(v: u8) -> Self { v as $x }
            fn from_u16(v: u16) -> Self { v as $x }
            fn from_u32(v: u32) -> Self { v as $x }
            fn from_u64(v: u64) -> Self { v as $x }
            fn to_u8(self) -> u8 { self as u8 }
            fn to_u16(self) -> u16 { self as u16 }
            fn to_u32(self) -> u32 { self as u32 }
            fn to_u64(self) -> u64 { self as u64 }
        })*
    };
}

data_impl!(u8, u16, u32, u64);
