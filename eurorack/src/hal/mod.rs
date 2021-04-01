pub mod dac;
pub mod gpio;
pub mod rcc;

pub use stm32f3::stm32f303 as pac;

pub mod prelude {
    pub use super::dac::DacConstrain;
    pub use super::gpio::GpioSplit;
    pub use super::rcc::RccConstrain;
}
