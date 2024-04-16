//! Definitions for snps,dw-apb-uart serial driver.
//! Uart snps,dw-apb-uart driver in Rust for BST A1000b FADA board.
#![no_std]

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_structs,
    registers::{ReadOnly, ReadWrite},
};

register_structs! {
    GPIORegs {
        todo!("根据手册填写功能寄存器")
    }
}

/// dw-apb-uart serial driver: DW8250
pub struct GPIO {
    base_vaddr: usize,
}

impl GPIO {
    /// New a GPIO
    pub const fn new(base_vaddr: usize) -> Self {
        Self { base_vaddr }
    }

    const fn regs(&self) -> &GPIORegs {
        unsafe { &*(self.base_vaddr as *const _) }
    }

    /// gpio initialize
    pub fn init(&mut self) {
        todo!("根据初始化流程编写寄存器")
    }
}

pub struct IO {
    gpio: GPIO,
}

impl IO {
    pub init {
        todo!("根据手册编写初始化流程")
    }

    /// 根据引脚号创建使用对象，例如使用portb的49就
    /// ```
    /// let pin = IO::new('b', 49);
    /// ```
    pub fn new(part: char, index: usize) -> Self{}

    /// 设置电平，true为高
    pub fn set_level(&self, value: bool) {
        todo!("根据手册编写设置流程")
    }

    /// 设置输入输出模式，true为输出
    pub fn set_io(&self, value: bool) {}
}
