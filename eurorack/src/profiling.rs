/// These are temporary macros that will be used for profiling of the code
/// through oscilloscope. They are not using HAL, but instead they access the
/// hardware directly to save troubles passing resources.
///
/// A: Daisy Patch SM pin A3, PA0
/// B: Daisy Patch SM pin A8, PB14
/// C: Daisy Patch SM pin A9, PB15
#[macro_export]
macro_rules! profile {
    (a, on) => {{
        use daisy::hal::pac::{GPIOA, RCC};
        let rcc = unsafe { &*RCC::ptr() };
        let gpiob = unsafe { &*GPIOA::ptr() };
        rcc.ahb4enr.modify(|_, w| w.gpioben().set_bit());
        gpiob.moder.modify(|_, w| w.moder0().output());
        gpiob.odr.modify(|_, w| w.odr0().set_bit());
    }};
    (a, off) => {{
        use daisy::hal::pac::{GPIOA, RCC};
        let rcc = unsafe { &*RCC::ptr() };
        let gpiob = unsafe { &*GPIOA::ptr() };
        rcc.ahb4enr.modify(|_, w| w.gpioben().set_bit());
        gpiob.moder.modify(|_, w| w.moder0().output());
        gpiob.odr.modify(|_, w| w.odr0().clear_bit());
    }};
    (b, on) => {{
        use daisy::hal::pac::{GPIOB, RCC};
        let rcc = unsafe { &*RCC::ptr() };
        let gpioc = unsafe { &*GPIOB::ptr() };
        rcc.ahb4enr.modify(|_, w| w.gpiocen().set_bit());
        gpioc.moder.modify(|_, w| w.moder14().output());
        gpioc.odr.modify(|_, w| w.odr14().set_bit());
    }};
    (b, off) => {{
        use daisy::hal::pac::{GPIOB, RCC};
        let rcc = unsafe { &*RCC::ptr() };
        let gpioc = unsafe { &*GPIOB::ptr() };
        rcc.ahb4enr.modify(|_, w| w.gpiocen().set_bit());
        gpioc.moder.modify(|_, w| w.moder14().output());
        gpioc.odr.modify(|_, w| w.odr14().clear_bit());
    }};
    (c, on) => {{
        use daisy::hal::pac::{GPIOB, RCC};
        let rcc = unsafe { &*RCC::ptr() };
        let gpioc = unsafe { &*GPIOB::ptr() };
        rcc.ahb4enr.modify(|_, w| w.gpiocen().set_bit());
        gpioc.moder.modify(|_, w| w.moder15().output());
        gpioc.odr.modify(|_, w| w.odr15().set_bit());
    }};
    (c, off) => {{
        use daisy::hal::pac::{GPIOB, RCC};
        let rcc = unsafe { &*RCC::ptr() };
        let gpioc = unsafe { &*GPIOB::ptr() };
        rcc.ahb4enr.modify(|_, w| w.gpiocen().set_bit());
        gpioc.moder.modify(|_, w| w.moder15().output());
        gpioc.odr.modify(|_, w| w.odr15().clear_bit());
    }};
}
