//! Definitions for snps,dw-apb-uart serial driver.
//! Uart snps,dw-apb-uart driver in Rust for BST A1000b FADA board.
#![no_std]

use tock_registers::{
    interfaces::ReadWriteable, register_bitfields, register_structs, registers::ReadWrite,
};

register_structs! {
    GPIORegs {
        (0x00 => _reserved0),//GPIO_SWPORTA_DR
        (0x04 => _reserved1),//GPIO_SWPORTA_DDR
        (0x08 => _reserved2),//GPIO_SWPORTA_CTL
        (0x0c => gpio_swportb_dr: ReadWrite<u32, dr::Register>),
        (0x10 => gpio_swportb_ddr: ReadWrite<u32, ddr::Register>),
        (0x14 => gpio_swportb_ctl: ReadWrite<u32, ctl::Register>),
        (0x18 => @END),
    }
}

register_structs! {
    LOCKRegs {
        (0x0000 => aon_pmm_reg_0: ReadWrite<u32, aon_0::Register>),
        (0x0004 => _reserved0),
        (0x0008 => _reserved1),
        (0x000C => _reserved2),
        (0x0010 => aon_pmm_reg_4: ReadWrite<u32, aon_4::Register>),
        (0x0014 => @END),
    }
}

// 寄存器具体到位的功能，目前计划用多少写多少
register_bitfields! [
    u32,
    /// 设置引脚复用
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
    /// 设置引脚复用
    aon_4 [
        GPIO51 OFFSET(20) NUMBITS(2) [
            GPIO = 0b11,
            I2C_SM2_SUS_INn = 0b10,
            SPI3_MOSI = 0b01,
            A_URAT3_CTS = 0b00
        ],
        GPIO52 OFFSET(22) NUMBITS(2) [
            GPIO = 0b11,
            I2C_SM2_ALERT_INn = 0b10,
            SPI3_MISO = 0b01,
            A_URAT3_RTS = 0b00
        ]
    ],
    /// 设置硬件软件模式
    ctl [
        GPIO49 OFFSET(17) NUMBITS(1) [
            SOFT = 0,
            HARD = 1
        ],
        GPIO50 OFFSET(18) NUMBITS(1) [
            SOFT = 0,
            HARD = 1
        ],
        GPIO51 OFFSET(19) NUMBITS(1) [
            SOFT = 0,
            HARD = 1
        ],
        GPIO52 OFFSET(20) NUMBITS(1) [
            SOFT = 0,
            HARD = 1
        ]
    ],
    /// 设置高低电平
    dr [
        GPIO49 OFFSET(17) NUMBITS(1) [
            LOW = 0,
            HIGH = 1
        ],
        GPIO50 OFFSET(18) NUMBITS(1) [
            LOW = 0,
            HIGH = 1
        ],
        GPIO51 OFFSET(19) NUMBITS(1) [
            LOW = 0,
            HIGH = 1
        ],
        GPIO52 OFFSET(20) NUMBITS(1) [
            LOW = 0,
            HIGH = 1
        ]
    ],
    /// 设置输入输出模式
    ddr [
        GPIO49 OFFSET(17) NUMBITS(1) [
            INPUT = 0,
            OUTPUT = 1
        ],
        GPIO50 OFFSET(18) NUMBITS(1) [
            INPUT = 0,
            OUTPUT = 1
        ],
        GPIO51 OFFSET(19) NUMBITS(1) [
            INPUT = 0,
            OUTPUT = 1
        ],
        GPIO52 OFFSET(20) NUMBITS(1) [
            INPUT = 0,
            OUTPUT = 1
        ]
    ]
];

/// dw-apb-uart serial driver: DW8250
pub struct GPIO {
    base_vaddr: usize,
    lock_base_address: usize,
}

impl GPIO {
    /// New a GPIO
    pub const fn new(base_vaddr: usize, lock_base_address: usize) -> Self {
        Self {
            base_vaddr,
            lock_base_address,
        }
    }

    const fn unlock_regs(&self, lock_base_address: usize) -> &LOCKRegs {
        unsafe { &*(lock_base_address as *const _) }
    }

    /// 根据引脚号创建使用对象，例如使用portb的49就
    /// ```
    /// let GPIO0 = GPIO::new(0x33001000);//之后需要讨论一下如何创建这个
    /// let pin = GPIO0.pin(49);
    /// ```
    pub fn pin(&mut self, index: usize) -> Pin {
        let lock_base_address = self.lock_base_address;
        // 似乎只能写成这样，宏无法作引脚引索到寄存器引索的映射
        match index {
            49 => {
                self.unlock_regs(lock_base_address)
                    .aon_pmm_reg_0
                    .modify(aon_0::GPIO49::GPIO);
            }
            50 => {
                self.unlock_regs(lock_base_address)
                    .aon_pmm_reg_0
                    .modify(aon_0::GPIO50::GPIO);
            }
            51 => {
                self.unlock_regs(lock_base_address)
                    .aon_pmm_reg_4
                    .modify(aon_4::GPIO51::GPIO);
            }
            52 => {
                self.unlock_regs(lock_base_address)
                    .aon_pmm_reg_4
                    .modify(aon_4::GPIO52::GPIO);
            }
            _ => panic!("引脚{index}功能未实现"),
        };
        Pin {
            base_vaddr: self.base_vaddr,
            index,
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
}

impl From<u8> for Pin {
    fn from(_value: u8) -> Self {
        panic!("错误的平台调用导致以dummy方式执行")
    }
}

impl Pin {
    const fn regs(&self) -> &GPIORegs {
        unsafe { &*(self.base_vaddr as *const _) }
    }

    /// 设置电平，true为高
    pub fn set_level(&self, value: bool) {
        match self.index {
            49 => {
                self.regs().gpio_swportb_dr.modify(match value {
                    true => dr::GPIO49::HIGH,
                    false => dr::GPIO49::LOW,
                });
            }
            50 => {
                self.regs().gpio_swportb_dr.modify(match value {
                    true => dr::GPIO50::HIGH,
                    false => dr::GPIO50::LOW,
                });
            }
            51 => {
                self.regs().gpio_swportb_dr.modify(match value {
                    true => dr::GPIO51::HIGH,
                    false => dr::GPIO51::LOW,
                });
            }
            52 => {
                self.regs().gpio_swportb_dr.modify(match value {
                    true => dr::GPIO52::HIGH,
                    false => dr::GPIO52::LOW,
                });
            }
            index => panic!("引脚{index}功能未实现"),
        }
    }

    /// 设置输入输出模式，true为输出
    pub fn set_io(&self, value: bool) {
        match self.index {
            49 => {
                self.regs().gpio_swportb_ddr.modify(match value {
                    true => ddr::GPIO49::OUTPUT,
                    false => ddr::GPIO49::INPUT,
                });
                self.regs().gpio_swportb_ctl.modify(ctl::GPIO49::SOFT);
            }
            50 => {
                self.regs().gpio_swportb_ddr.modify(match value {
                    true => ddr::GPIO50::OUTPUT,
                    false => ddr::GPIO50::INPUT,
                });
                self.regs().gpio_swportb_ctl.modify(ctl::GPIO50::SOFT);
            }
            51 => {
                self.regs().gpio_swportb_ddr.modify(match value {
                    true => ddr::GPIO51::OUTPUT,
                    false => ddr::GPIO51::INPUT,
                });
                self.regs().gpio_swportb_ctl.modify(ctl::GPIO51::SOFT);
            }
            52 => {
                self.regs().gpio_swportb_ddr.modify(match value {
                    true => ddr::GPIO52::OUTPUT,
                    false => ddr::GPIO52::INPUT,
                });
                self.regs().gpio_swportb_ctl.modify(ctl::GPIO52::SOFT);
            }
            index => panic!("引脚{index}功能未实现"),
        }
    }
}
