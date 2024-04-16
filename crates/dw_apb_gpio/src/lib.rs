//! Definitions for snps,dw-apb-uart serial driver.
//! Uart snps,dw-apb-uart driver in Rust for BST A1000b FADA board.
#![no_std]

use tock_registers::{
    interfaces::ReadWriteable,
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

register_structs! {
    GPIORegs {
        /// Get or Put Register.
        (0x00 => rbr: ReadWrite<u32>),
        (0x04 => ier: ReadWrite<u32>),
        (0x08 => fcr: ReadWrite<u32>),
        (0x0c => lcr: ReadWrite<u32>),
        (0x10 => mcr: ReadWrite<u32>),
        (0x14 => lsr: ReadOnly<u32>),
        (0x18 => msr: ReadOnly<u32>),
        (0x1c => scr: ReadWrite<u32>),
        (0x20 => lpdll: ReadWrite<u32>),
        (0x24 => _reserved0),
        /// Uart Status Register.
        (0x7c => usr: ReadOnly<u32>),
        (0x80 => _reserved1),
        (0xc0 => dlf: ReadWrite<u32>),
        (0xc4 => @END),
    }
}

register_structs! {
    LOCKRegs {
        (0x0000 => aon_pmm_reg_0: ReadWrite<u32, aon_0::Register>),
        (0x0004 => _reserved0),
        (0x0008 => _reserved1),
        (0x000C => _reserved2),
        (0x0010 => aon_pmm_reg_4: ReadWrite<u32, >),
        (0x0014 => @END),
    }
}

register_bitfields! [
    u32,
    aon_0 [
        GPIO49 OFFSET(25) NUMBITS(1) [
            A_URAT3_TXD = 0,
            GPIO = 1
        ],
        GPIO50 OFFSET(26) NUMBITS(1) [
            A_URAT3_RXD = 0,
            GPIO = 1
        ],
    ],
    aon_4 [
        GPIO51 OFFSET(20) NUMBITS(2) [
            GPIO = 0b11,
            I2C_SM2_SUS_INn = 0b10,
            SPI3_MOSI = 0b01,
            A_URAT3_CTS = 0b00
        ]
    ]
];

/// dw-apb-uart serial driver: DW8250
pub struct GPIO {
    base_vaddr: usize,
}

impl GPIO {
    /// New a GPIO
    pub const fn new(base_vaddr: usize) -> Self {
        Self { base_vaddr }
    }

    const fn unlock_regs(&self, lock_base_address: u32) -> &LOCKRegs {
        unsafe { &*(lock_base_address as *const _) }
    }

    /// 根据引脚号创建使用对象，例如使用portb的49就
    /// ```
    /// let GPIO0 = GPIO::new(0x33001000);//之后需要讨论一下如何创建这个
    /// let pin = GPIO0.pin(49);
    /// ```
    pub fn pin(&mut self, index: usize) -> Pin {
        let lock_base_address = match index < 111 {
            true => 0x0038000,
            false => 0x33001000,
        };
        match index {
            49 => {
                self.unlock_regs(lock_base_address)
                    .aon_pmm_reg_0
                    .modify(aon_0::GPIO49::GPIO);
            }
            _ => panic!("引脚功能未实现"),
        };
        Pin {
            base_vaddr: self.base_vaddr,
            index,
            lock_base_address,
        }
    }

    /// gpio initialize
    pub fn init(&mut self) {
        todo!("根据初始化流程编写寄存器")
    }
}

/// 引脚具体动作的实现，创建它应当通过GPIO::pin()
pub struct Pin {
    base_vaddr: usize,
    index: usize,
    lock_base_address: u32,
}

impl Pin {
    const fn regs(&self) -> &GPIORegs {
        unsafe { &*(self.base_vaddr as *const _) }
    }

    /// 设置电平，true为高
    pub fn set_level(&self, value: bool) {
        todo!("根据手册编写设置流程")
    }

    /// 设置输入输出模式，true为输出
    pub fn set_io(&self, value: bool) {}
}
