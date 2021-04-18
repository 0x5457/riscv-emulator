use std::convert::TryInto;

use crate::trap::Exception;

use super::{Data, Device};

pub struct Memory {
    data: Vec<u8>,
    dram_base: u64,
}

impl Device for Memory {
    fn read<T>(&self, addr: u64) -> Result<T, Exception>
    where
        T: Data,
        [(); <T as Data>::SIZE]: Sized,
    {
        let start_idx = (addr - self.dram_base) as usize;
        let v = self.data[start_idx..start_idx + std::mem::size_of::<T>()]
            .try_into()
            .map_err(|_| Exception::LoadFault)?;
        let x = T::from_bytes(v);
        Ok(x)
    }

    fn write<T>(&mut self, addr: u64, value: T) -> Result<(), Exception>
    where
        T: Data,
        [(); <T as Data>::SIZE]: Sized,
    {
        let bytes = value.to_bytes();
        let start_idx = (addr - self.dram_base) as usize;

        for (idx, bit) in bytes.iter().enumerate() {
            self.data[start_idx + idx] = *bit;
        }
        Ok(())
    }
}

impl Memory {
    pub fn new_with_binary(dram_base: u64, binary: Vec<u8>, cap: usize) -> Self {
        let mut data = vec![0; cap];
        data.splice(..binary.len(), binary.iter().cloned());
        Self {
            data: data,
            dram_base: dram_base,
        }
    }
}
