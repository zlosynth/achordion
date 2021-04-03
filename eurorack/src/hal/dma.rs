use stm32f3xx_hal::dma::dma2::C3;
use stm32f3xx_hal::pac::DMA2;

pub trait Dma2Ch3Ext {
    fn set_circular(&mut self, circular: bool);
}

impl Dma2Ch3Ext for C3 {
    fn set_circular(&mut self, circular: bool) {
        let cr = unsafe { &(*DMA2::ptr()).ch3.cr };
        cr.modify(|_, w| w.circ().bit(circular));
    }
}
