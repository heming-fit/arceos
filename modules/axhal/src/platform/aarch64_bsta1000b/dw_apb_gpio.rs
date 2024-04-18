//! 寄存器的GPIO实现

use crate::mem::phys_to_virt;
use dw_apb_gpio::{Pin, GPIO};
use memory_addr::PhysAddr;
use spinlock::SpinNoIrq;

const GPIO0_BASE: PhysAddr = PhysAddr::from(axconfig::GPIO0_PADDR);
const LOCK_BASE: PhysAddr = PhysAddr::from(axconfig::LOCK_PADDR);

pub static GPIO0: SpinNoIrq<GPIO> = SpinNoIrq::new(GPIO::new(
    phys_to_virt(GPIO0_BASE).as_usize(),
    phys_to_virt(LOCK_BASE).as_usize(),
));

/// 得到pin
pub fn pin(index: usize) -> Pin {
    GPIO0.lock().pin(index)
}
