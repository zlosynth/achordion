use super::pac::{exti, EXTI};

pub trait ExtiConstrain {
    fn constrain(self) -> Exti;
}

impl ExtiConstrain for EXTI {
    fn constrain(self) -> Exti {
        Exti(self)
    }
}

pub struct Exti(EXTI);

impl Exti {
    pub fn pr1(&self) -> &exti::PR1 {
        unsafe { &(*EXTI::ptr()).pr1 }
    }

    pub fn imr1(&self) -> &exti::IMR1 {
        unsafe { &(*EXTI::ptr()).imr1 }
    }

    pub fn rtsr1(&self) -> &exti::RTSR1 {
        unsafe { &(*EXTI::ptr()).rtsr1 }
    }
}
