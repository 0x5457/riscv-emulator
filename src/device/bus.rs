use crate::trap::Exception;

use super::{
    clint::Clint, memory::Memory, plic::Plic, uart::Uart, virtio::Virtio, Data, Device, CLINT_BASE,
    CLINT_END, DRAM_BASE, DRAM_END, DRAM_SIZE, PLIC_BASE, PLIC_END, UART_BASE, UART_END,
    VIRTIO_BASE, VIRTIO_END,
};

pub struct Bus {
    memory: Memory,
    pub clint: Clint,
    pub plic: Plic,
    pub uart: Uart,
    pub virtio: Virtio,
}

impl Device for Bus {
    fn read<T>(&self, addr: u64) -> Result<T, Exception>
    where
        T: Data,
        [(); <T as Data>::SIZE]: Sized,
    {
        match addr {
            CLINT_BASE..=CLINT_END => self.clint.read::<T>(addr),
            PLIC_BASE..=PLIC_END => self.plic.read::<T>(addr),
            UART_BASE..=UART_END => self.uart.read::<T>(addr),
            VIRTIO_BASE..=VIRTIO_END => self.virtio.read::<T>(addr),
            DRAM_BASE..=DRAM_END => self.memory.read::<T>(addr),
            _ => Err(Exception::LoadFault),
        }
    }

    fn write<T>(&mut self, addr: u64, value: T) -> Result<(), Exception>
    where
        T: Data,
        [(); <T as Data>::SIZE]: Sized,
    {
        match addr {
            CLINT_BASE..=CLINT_END => self.clint.write::<T>(addr, value),
            PLIC_BASE..=PLIC_END => self.plic.write::<T>(addr, value),
            UART_BASE..=UART_END => self.uart.write::<T>(addr, value),
            VIRTIO_BASE..=VIRTIO_END => self.virtio.write::<T>(addr, value),
            DRAM_BASE..=DRAM_END => self.memory.write::<T>(addr, value),
            _ => Err(Exception::StoreFault),
        }
    }
}

impl Bus {
    pub fn new(binary: Vec<u8>) -> Self {
        Self {
            memory: Memory::new_with_binary(DRAM_BASE, binary, DRAM_SIZE),
            clint: Clint::new(),
            plic: Plic::new(),
            uart: Uart::new(),
            virtio: Virtio::new(),
        }
    }
}
