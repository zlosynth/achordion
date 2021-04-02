use super::rcc::AHB;

pub struct Analog;
pub struct Input;
pub struct Uninitialized;

pub trait GpioSplit {
    type Parts;

    fn split(self, ahb: &mut AHB) -> Self::Parts;
}

pub enum Edge {
    Rising,
}

pub mod a {
    use core::marker::PhantomData;

    use super::super::exti::Exti;
    use super::super::pac::interrupt;
    use super::super::pac::{gpioa, GPIOA};
    use super::super::rcc::AHB;
    use super::super::syscfg::SysCfg;
    use super::{Analog, Edge, GpioSplit, Input, Uninitialized};

    /// Opaque MODER register
    pub struct MODER {
        _0: (),
    }

    impl MODER {
        pub unsafe fn moder(&mut self) -> &gpioa::MODER {
            &(*GPIOA::ptr()).moder
        }
    }

    /// Opaque PUPDR register
    pub struct PUPDR {
        _0: (),
    }

    impl PUPDR {
        pub unsafe fn pupdr(&mut self) -> &gpioa::PUPDR {
            &(*GPIOA::ptr()).pupdr
        }
    }

    pub struct PA0<MODE> {
        _mode: PhantomData<MODE>,
    }

    impl PA0<Uninitialized> {
        pub fn into_pull_down(self, moder: &mut MODER, pupdr: &mut PUPDR) -> PA0<Input> {
            unsafe {
                moder.moder().modify(|_, w| w.moder0().input());
                pupdr.pupdr().modify(|_, w| w.pupdr0().pull_down());
            }
            PA0 { _mode: PhantomData }
        }
    }

    impl PA0<Input> {
        pub fn interrupt_exti0(&mut self, syscfg: &mut SysCfg) {
            unsafe {
                syscfg.exticr1().modify(|_, w| w.exti0().pa0());
            }
        }

        pub fn clear_exti0(&mut self, exti: &mut Exti) {
            unsafe {
                exti.pr1().modify(|_, w| w.pr0().clear());
            }
        }

        pub fn unmask_exti0(&mut self, exti: &mut Exti) {
            unsafe {
                exti.imr1().modify(|_, w| w.mr0().unmasked());
                rtic::export::NVIC::unmask(interrupt::DMA2_CH3);
            }
        }

        pub fn trigger_on_edge(&mut self, exti: &mut Exti, edge: Edge) {
            use super::Edge::*;

            match edge {
                Rising => unsafe {
                    exti.rtsr1().modify(|_, w| w.tr0().enabled());
                },
            }
        }

        pub fn is_high(&mut self) -> bool {
            unsafe { self.idr().read().idr0().is_high() }
        }

        pub unsafe fn idr(&self) -> &gpioa::IDR {
            &(*GPIOA::ptr()).idr
        }
    }

    pub struct PA4<MODE> {
        _mode: PhantomData<MODE>,
    }

    impl PA4<Uninitialized> {
        /// Configures the pin to operate as analog, with disabled schmitt trigger.
        /// This mode is suitable when the pin is connected to the DAC or ADC.
        pub fn into_analog(self, moder: &mut MODER, pupdr: &mut PUPDR) -> PA4<Analog> {
            unsafe {
                moder.moder().modify(|_, w| w.moder4().analog());
                pupdr.pupdr().modify(|_, w| w.pupdr4().floating());
            }
            PA4 { _mode: PhantomData }
        }
    }

    pub struct PA5<MODE> {
        _mode: PhantomData<MODE>,
    }

    impl PA5<Uninitialized> {
        /// Configures the pin to operate as analog, with disabled schmitt trigger.
        /// This mode is suitable when the pin is connected to the DAC or ADC.
        pub fn into_analog(self, moder: &mut MODER, pupdr: &mut PUPDR) -> PA5<Analog> {
            unsafe {
                moder.moder().modify(|_, w| w.moder5().analog());
                pupdr.pupdr().modify(|_, w| w.pupdr5().floating());
            }
            PA5 { _mode: PhantomData }
        }
    }

    pub struct Parts {
        /// Opaque MODER register
        pub moder: MODER,
        /// Opaque PUPDR register
        pub pupdr: PUPDR,
        /// Pin
        pub pa0: PA0<Uninitialized>,
        /// Pin
        pub pa4: PA4<Uninitialized>,
        /// Pin
        pub pa5: PA5<Uninitialized>,
    }

    impl GpioSplit for GPIOA {
        type Parts = Parts;

        fn split(self, ahb: &mut AHB) -> Parts {
            unsafe {
                ahb.enr().modify(|_, w| w.iopaen().set_bit());
                ahb.rstr().modify(|_, w| w.ioparst().set_bit());
                ahb.rstr().modify(|_, w| w.ioparst().clear_bit());
            }

            Parts {
                moder: MODER { _0: () },
                pupdr: PUPDR { _0: () },
                pa0: PA0 { _mode: PhantomData },
                pa4: PA4 { _mode: PhantomData },
                pa5: PA5 { _mode: PhantomData },
            }
        }
    }
}
