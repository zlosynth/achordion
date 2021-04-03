use stm32f3xx_hal::pac::{rcc, RCC};
use stm32f3xx_hal::rcc::APB1;

pub trait Apb1 {
    fn enr(&mut self) -> &rcc::APB1ENR;
}

impl Apb1 for APB1 {
    fn enr(&mut self) -> &rcc::APB1ENR {
        unsafe { &(*RCC::ptr()).apb1enr }
    }
}
