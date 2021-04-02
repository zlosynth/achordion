use super::pac::{syscfg, SYSCFG};
use super::rcc::APB2;

pub trait SysCfgConstrain {
    fn constrain(self, apb2: &mut APB2) -> SysCfg;
}

impl SysCfgConstrain for SYSCFG {
    fn constrain(self, apb2: &mut APB2) -> SysCfg {
        unsafe {
            apb2.enr().modify(|_, w| w.syscfgen().enabled());
        }

        SysCfg(self)
    }
}

pub struct SysCfg(SYSCFG);

impl SysCfg {
    pub unsafe fn exticr1(&self) -> &syscfg::EXTICR1 {
        &(*SYSCFG::ptr()).exticr1
    }
}
