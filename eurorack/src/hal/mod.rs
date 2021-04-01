pub mod dac;
pub mod gpio;
pub mod rcc;
pub mod timer;

pub use stm32f3::stm32f303 as pac;

pub mod prelude {
    pub use super::dac::DacConstrain;
    pub use super::gpio::GpioSplit;
    pub use super::rcc::RccConstrain;
    pub use super::timer::Timer2Constrain;
}
