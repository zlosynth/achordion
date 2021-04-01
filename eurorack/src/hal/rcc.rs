use super::pac::{rcc, RCC};

pub trait RccConstrain {
    fn constrain(self) -> Rcc;
}

impl RccConstrain for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            ahb: AHB { _0: () },
            apb1: APB1 { _0: () },
        }
    }
}

pub struct Rcc {
    /// AMBA High-performance Bus (AHB) registers
    pub ahb: AHB,
    /// Advanced Peripheral Bus 1 (APB1) registers
    pub apb1: APB1,
}

/// AMBA High-performance Bus (AHB) registers
pub struct AHB {
    _0: (),
}

impl AHB {
    pub unsafe fn enr(&mut self) -> &rcc::AHBENR {
        &(*RCC::ptr()).ahbenr
    }

    pub unsafe fn rstr(&mut self) -> &rcc::AHBRSTR {
        &(*RCC::ptr()).ahbrstr
    }
}

/// Advanced Peripheral Bus 1 (APB1) registers
pub struct APB1 {
    _0: (),
}

impl APB1 {
    pub unsafe fn enr(&mut self) -> &rcc::APB1ENR {
        &(*RCC::ptr()).apb1enr
    }
}
