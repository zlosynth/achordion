use stm32f3xx_hal::pac::TIM2;
use stm32f3xx_hal::timer::Timer;

pub trait TimerExt {
    fn reset_on_overflow(&mut self);
}

impl TimerExt for Timer<TIM2> {
    fn reset_on_overflow(&mut self) {
        let cr2 = unsafe { &(*TIM2::ptr()).cr2 };
        cr2.write(|w| w.mms().update());
    }
}
