use bit_field::BitField;

use crate::register::satp::Mode;

#[derive(Clone, Debug)]
pub struct PageTableEnty(pub u64);

#[allow(dead_code)]
impl PageTableEnty {
    // If non-leaf PTE. ppn points to the base address of the next level page table
    pub fn ppn(&self, mode: &Mode) -> u64 {
        match mode {
            Mode::Sv32 => self.0.get_bits(10..),
            Mode::Sv39 => self.0.get_bits(10..54),
            _ => unimplemented!(),
        }
    }

    pub fn ppns(&self, mode: &Mode) -> Vec<u64> {
        match mode {
            Mode::Sv32 => vec![self.0.get_bits(10..20), self.0.get_bits(20..32)],
            Mode::Sv39 => vec![
                self.0.get_bits(10..19),
                self.0.get_bits(19..28),
                self.0.get_bits(28..54),
            ],
            _ => unimplemented!(),
        }
    }

    /// V 位决定了该页表项的其余部分是否有效（V = 1 时有效）。若 V = 0，则任何遍历
    /// 到此页表项的虚址转换操作都会导致页错误。
    #[inline]
    pub fn v(&self) -> bool {
        self.0.get_bit(0)
    }

    /// 表示此页是否可以读取
    #[inline]
    pub fn r(&self) -> bool {
        self.0.get_bit(1)
    }

    /// 表示此页是否可以写入
    #[inline]
    pub fn w(&self) -> bool {
        self.0.get_bit(2)
    }

    /// 表示此页是否可以执行
    #[inline]
    pub fn x(&self) -> bool {
        self.0.get_bit(3)
    }

    /// U 位表示该页是否是用户页面。若 U = 0，则 U 模式不能访问此页面，
    /// 但 S 模式可以。若 U = 1，则 U 模式下能访问这个页面，而 S 模式不能。
    #[inline]
    pub fn u(&self) -> bool {
        self.0.get_bit(4)
    }

    /// G 位表示这个映射是否对所有虚址空间有效，硬件可以用这个信息来提高地址转
    /// 换的性能。这一位通常只用于属于操作系统的页面。
    #[inline]
    pub fn g(&self) -> bool {
        self.0.get_bit(5)
    }

    /// A 位表示自从上次 A 位被清除以来，该页面是否被访问过。
    #[inline]
    pub fn a(&self) -> bool {
        self.0.get_bit(6)
    }

    /// D 位表示自从上次清除 D 位以来页面是否被弄脏（例如被写入）。
    #[inline]
    pub fn d(&self) -> bool {
        self.0.get_bit(7)
    }
}

#[derive(Clone, Debug)]
pub struct VirtualAddress(pub u64);

impl VirtualAddress {
    pub fn virtual_page_offsets(&self, mode: &Mode) -> Vec<u64> {
        match mode {
            Mode::Sv32 => vec![self.0.get_bits(12..21) << 2, self.0.get_bits(22..31) << 2],
            Mode::Sv39 => vec![
                self.0.get_bits(12..21) << 3,
                self.0.get_bits(21..30) << 3,
                self.0.get_bits(30..39) << 3,
            ],
            _ => unimplemented!(),
        }
    }

    pub fn offset(&self) -> u64 {
        self.0.get_bits(0..12)
    }
}
