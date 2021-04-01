use super::rcc::AHB;

pub trait GpioSplit {
    type Parts;

    fn split(self, ahb: &mut AHB) -> Self::Parts;
}

pub mod a {
    use super::super::pac::{gpioa, GPIOA};
    use super::super::rcc::AHB;
    use super::GpioSplit;

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

    pub struct PA4 {
        _0: (),
    }

    impl PA4 {
        /// Configures the pin to operate as analog, with disabled schmitt trigger.
        /// This mode is suitable when the pin is connected to the DAC or ADC.
        pub fn into_analog(self, moder: &mut MODER, pupdr: &mut PUPDR) -> PA4 {
            unsafe {
                moder.moder().modify(|_, w| w.moder4().analog());
                pupdr.pupdr().modify(|_, w| w.pupdr4().floating());
            }
            self
        }
    }

    pub struct PA5 {
        _0: (),
    }

    impl PA5 {
        /// Configures the pin to operate as analog, with disabled schmitt trigger.
        /// This mode is suitable when the pin is connected to the DAC or ADC.
        pub fn into_analog(self, moder: &mut MODER, pupdr: &mut PUPDR) -> PA5 {
            unsafe {
                moder.moder().modify(|_, w| w.moder5().analog());
                pupdr.pupdr().modify(|_, w| w.pupdr5().floating());
            }
            self
        }
    }

    pub struct Parts {
        /// Opaque MODER register
        pub moder: MODER,
        /// Opaque PUPDR register
        pub pupdr: PUPDR,
        /// Pin
        pub pa4: PA4,
        /// Pin
        pub pa5: PA5,
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
                pa4: PA4 { _0: () },
                pa5: PA5 { _0: () },
            }
        }
    }
}
