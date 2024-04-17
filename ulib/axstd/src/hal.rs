use arceos_api;
pub use dw_apb_gpio;

fn pin(index: usize) -> dw_apb_gpio::Pin {
    arceos_api::hal::pin(index)
}
