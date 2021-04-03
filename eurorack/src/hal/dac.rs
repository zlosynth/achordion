use stm32f3xx_hal::gpio::gpioa::{MODER, PA4, PA5, PUPDR};
use stm32f3xx_hal::gpio::Analog;
use stm32f3xx_hal::pac::{dac1, DAC1};
use stm32f3xx_hal::rcc::APB1;

use super::rcc::Apb1 as _;

pub trait DacExt {
    fn constrain(
        self,
        _pa4: PA4<Analog>,
        _pa5: PA5<Analog>,
        apb1: &mut APB1,
        _moder: &mut MODER,
        _pupdr: &mut PUPDR,
    ) -> DualModeDac;
}

impl DacExt for DAC1 {
    fn constrain(
        self,
        _pa4: PA4<Analog>,
        _pa5: PA5<Analog>,
        apb1: &mut APB1,
        _moder: &mut MODER,
        _pupdr: &mut PUPDR,
    ) -> DualModeDac {
        apb1.enr().modify(|_, w| w.dac1en().set_bit());

        DualModeDac { _0: () }
    }
}

pub struct DualModeDac {
    _0: (),
}

impl DualModeDac {
    pub fn disable_buffer(&mut self) {
        self.cr()
            .modify(|_, w| w.boff1().disabled().boff2().disabled());
    }

    pub fn set_trigger_tim2(&mut self) {
        self.cr().modify(|_, w| {
            w.ten1()
                .enabled()
                .ten2()
                .enabled()
                .tsel1()
                .tim2_trgo()
                .tsel2()
                .tim2_trgo()
        });
    }

    pub fn enable_dma(&mut self) {
        // in dual mode, it is enough to enable one dma channel, the other
        // will be enabled implicitly
        self.cr().modify(|_, w| w.dmaen1().enabled());
    }

    pub fn enable(&mut self) {
        self.cr().modify(|_, w| w.en1().enabled().en2().enabled());
    }

    fn cr(&mut self) -> &dac1::CR {
        unsafe { &(*DAC1::ptr()).cr }
    }
}
