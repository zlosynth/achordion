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
    pub unsafe fn pr1(&self) -> &exti::PR1 {
        &(*EXTI::ptr()).pr1
    }

    pub unsafe fn imr1(&self) -> &exti::IMR1 {
        &(*EXTI::ptr()).imr1
    }

    pub unsafe fn rtsr1(&self) -> &exti::RTSR1 {
        &(*EXTI::ptr()).rtsr1
    }
}
