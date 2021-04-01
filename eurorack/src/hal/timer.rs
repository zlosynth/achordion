use core::marker::PhantomData;

use super::pac::{tim2, TIM2};
use super::rcc::APB1;

const SYSTEM_CLOCK: u32 = 8_000_000;

pub struct Uninitialized;
pub struct Periodic;

pub trait Timer2Constrain {
    fn constrain(self, apb1: &mut APB1) -> Timer2<Uninitialized>;
}

impl Timer2Constrain for TIM2 {
    fn constrain(self, apb1: &mut APB1) -> Timer2<Uninitialized> {
        unsafe {
            apb1.enr().modify(|_, w| w.tim2en().set_bit());
        }

        Timer2 { _mode: PhantomData }
    }
}

pub struct Timer2<MODE> {
    _mode: PhantomData<MODE>,
}

impl<MODE> Timer2<MODE> {
    pub fn into_periodic(mut self, frequency: u32) -> Timer2<Periodic> {
        let arr = SYSTEM_CLOCK / frequency; // value to use for auto reload register (arr)

        unsafe {
            self.cr2().write(|w| w.mms().update()); // update when counter reaches arr value, on overflow
            self.arr().write(|w| w.arr().bits(arr)); // set timer period
        }

        Timer2 { _mode: PhantomData }
    }

    pub unsafe fn cr1(&mut self) -> &tim2::CR1 {
        &(*TIM2::ptr()).cr1
    }

    pub unsafe fn cr2(&mut self) -> &tim2::CR2 {
        &(*TIM2::ptr()).cr2
    }

    pub unsafe fn arr(&mut self) -> &tim2::ARR {
        &(*TIM2::ptr()).arr
    }
}

impl Timer2<Periodic> {
    pub fn enable(&mut self) {
        unsafe {
            self.cr1().modify(|_, w| w.cen().enabled());
        }
    }
}
