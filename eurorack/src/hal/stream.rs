use stm32f4xx_hal::dma::traits::Stream;
use stm32f4xx_hal::dma::Stream5;
use stm32f4xx_hal::pac::DMA1;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum WordSize {
    Byte = 0,
    HalfWord,
    Word,
}

pub trait StreamExt {
    fn set_circular(&mut self, circular: bool);
    fn set_word_size(&mut self, size: WordSize);
}

impl StreamExt for Stream5<DMA1> {
    fn set_circular(&mut self, circular: bool) {
        let cr = unsafe { &(*DMA1::ptr()).st[5].cr };
        cr.modify(|_, w| w.circ().bit(circular));
    }

    fn set_word_size(&mut self, size: WordSize) {
        unsafe {
            self.set_memory_size(size as u8);
            self.set_peripheral_size(size as u8);
        }
    }
}
