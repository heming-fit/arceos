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
