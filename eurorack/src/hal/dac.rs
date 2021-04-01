use super::pac::{dac1, DAC1};
use super::rcc::APB1;

pub trait DacConstrain {
    fn constrain(self, apb1: &mut APB1) -> Dac;
}

impl DacConstrain for DAC1 {
    fn constrain(self, apb1: &mut APB1) -> Dac {
        unsafe {
            apb1.enr().modify(|_, w| w.dac1en().set_bit());
        }

        Dac { _0: () }
    }
}

pub struct Dac {
    _0: (),
}

impl Dac {
    pub unsafe fn cr(&mut self) -> &dac1::CR {
        &(*DAC1::ptr()).cr
    }
}
