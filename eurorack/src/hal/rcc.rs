use super::pac::RCC;

pub trait RccConstrain {
    fn constrain(self) -> Rcc;
}

impl RccConstrain for RCC {
    fn constrain(self) -> Rcc {
        Rcc {}
    }
}

pub struct Rcc {}
