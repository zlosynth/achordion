use super::pac::dma1::ch::cr;
use super::rcc::AHB;

pub enum Event {
    HalfTransfer,
    TransferComplete,
    TransferError,
    Any,
}

pub enum Direction {
    FromMemory,
}

impl From<Direction> for cr::DIR_A {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::FromMemory => cr::DIR_A::FROMMEMORY,
        }
    }
}

pub enum Increment {
    Enable,
    Disable,
}

impl From<Increment> for cr::PINC_A {
    fn from(inc: Increment) -> Self {
        match inc {
            Increment::Enable => cr::PINC_A::ENABLED,
            Increment::Disable => cr::PINC_A::DISABLED,
        }
    }
}

pub enum Priority {
    High,
}

impl From<Priority> for cr::PL_A {
    fn from(prio: Priority) -> Self {
        match prio {
            Priority::High => cr::PL_A::HIGH,
        }
    }
}

pub trait Dma2Split {
    type Parts;

    fn split(self, ahb: &mut AHB) -> Self::Parts;
}

pub mod _2 {
    use core::mem;

    use stm32f3::stm32f303::interrupt;

    use super::super::pac::{
        dma1::{self, ch::cr},
        DMA2,
    };
    use super::super::rcc::AHB;
    use super::{Direction, Dma2Split, Event, Increment, Priority};

    impl Dma2Split for DMA2 {
        type Parts = Parts;

        fn split(self, ahb: &mut AHB) -> Parts {
            ahb.enr().modify(|_, w| w.dma2en().set_bit());

            Parts {
                ch3: CHANNEL3 { _0: () },
            }
        }
    }

    pub struct Parts {
        pub ch3: CHANNEL3,
    }

    pub struct CHANNEL3 {
        _0: (),
    }

    impl CHANNEL3 {
        pub fn set_direction(&mut self, direction: Direction) {
            match direction {
                Direction::FromMemory => {
                    self.cr()
                        .modify(|_, w| w.dir().from_memory().mem2mem().disabled());
                }
            }
        }

        pub unsafe fn set_peripheral_address(&mut self, address: u32, increment: Increment) {
            if self.is_enabled() {
                panic!();
            }

            self.par().write(|w| w.pa().bits(address));
            self.cr().modify(|_, w| w.pinc().variant(increment.into()));
        }

        pub unsafe fn set_memory_address(&mut self, address: u32, increment: Increment) {
            if self.is_enabled() {
                panic!();
            }

            self.mar().write(|w| w.ma().bits(address));
            self.cr().modify(|_, w| w.minc().variant(increment.into()));
        }

        pub fn set_transfer_length(&mut self, len: u16) {
            if self.is_enabled() {
                panic!();
            }

            self.ndtr().write(|w| w.ndt().bits(len));
        }

        pub fn set_word_size<W>(&mut self) {
            use cr::PSIZE_A::*;

            let psize = match mem::size_of::<W>() {
                1 => BITS8,
                2 => BITS16,
                4 => BITS32,
                s => panic!("unsupported word size: {:?}", s),
            };

            self.cr().modify(|_, w| {
                w.psize().variant(psize);
                w.msize().variant(psize)
            });
        }

        pub fn set_priority_level(&mut self, priority: Priority) {
            let priority_level = priority.into();
            self.cr().modify(|_, w| w.pl().variant(priority_level));
        }

        pub fn set_circular(&mut self, circular: bool) {
            self.cr().modify(|_, w| w.circ().bit(circular));
        }

        pub fn listen(&mut self, event: Event) {
            use Event::*;

            match event {
                HalfTransfer => self.cr().modify(|_, w| w.htie().enabled()),
                TransferComplete => self.cr().modify(|_, w| w.tcie().enabled()),
                TransferError => self.cr().modify(|_, w| w.teie().enabled()),
                Any => self.cr().modify(|_, w| {
                    w.htie().enabled();
                    w.tcie().enabled();
                    w.teie().enabled()
                }),
            }
        }

        pub fn event(&self) -> Option<Event> {
            let isr = self.isr().read();
            if isr.htif3().is_half() {
                Some(Event::HalfTransfer)
            } else if isr.tcif3().is_complete() {
                Some(Event::TransferComplete)
            } else if isr.teif3().is_error() {
                Some(Event::TransferError)
            } else {
                None
            }
        }

        pub fn clear_events(&mut self) {
            unsafe {
                self.ifcr().write(|w| {
                    w.chtif3().clear();
                    w.ctcif3().clear();
                    w.cteif3().clear()
                });
            }
        }

        pub fn unmask_interrupt(&mut self) {
            unsafe {
                rtic::export::NVIC::unmask(interrupt::DMA2_CH3);
            }
        }

        pub fn enable(&mut self) {
            self.cr().modify(|_, w| w.en().enabled());
        }

        fn is_enabled(&mut self) -> bool {
            self.cr().read().en().is_enabled()
        }

        fn cr(&mut self) -> &stm32f3::Reg<u32, dma1::ch::_CR> {
            unsafe { &(*DMA2::ptr()).ch3.cr }
        }

        fn par(&mut self) -> &stm32f3::Reg<u32, dma1::ch::_PAR> {
            unsafe { &(*DMA2::ptr()).ch3.par }
        }

        fn mar(&mut self) -> &stm32f3::Reg<u32, dma1::ch::_MAR> {
            unsafe { &(*DMA2::ptr()).ch3.mar }
        }

        fn ndtr(&mut self) -> &stm32f3::Reg<u32, dma1::ch::_NDTR> {
            unsafe { &(*DMA2::ptr()).ch3.ndtr }
        }

        fn isr(&self) -> &dma1::ISR {
            unsafe { &(*DMA2::ptr()).isr }
        }

        unsafe fn ifcr(&mut self) -> &dma1::IFCR {
            &(*DMA2::ptr()).ifcr
        }
    }
}
