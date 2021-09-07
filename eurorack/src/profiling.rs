/// These are temporary macros that will be used for profiling of the code
/// through oscilloscope. They are not using HAL, but instead they access the
/// hardware directly to save troubles passing resources.
///
/// A: Daisy pin 1, PB12
/// B: Daisy pin 2, PC11
#[macro_export]
macro_rules! profile {
    (a, on) => {{
        use daisy_bsp::hal::pac::{GPIOB, RCC};
        let rcc = unsafe { &*RCC::ptr() };
        let gpiob = unsafe { &*GPIOB::ptr() };
        rcc.ahb4enr.modify(|_, w| w.gpioben().set_bit());
        gpiob.moder.modify(|_, w| w.moder12().output());
        gpiob.odr.modify(|_, w| w.odr12().set_bit());
    }};
    (a, off) => {{
        use daisy_bsp::hal::pac::{GPIOB, RCC};
        let rcc = unsafe { &*RCC::ptr() };
        let gpiob = unsafe { &*GPIOB::ptr() };
        rcc.ahb4enr.modify(|_, w| w.gpioben().set_bit());
        gpiob.moder.modify(|_, w| w.moder12().output());
        gpiob.odr.modify(|_, w| w.odr12().clear_bit());
    }};
    (b, on) => {{
        use daisy_bsp::hal::pac::{GPIOC, RCC};
        let rcc = unsafe { &*RCC::ptr() };
        let gpioc = unsafe { &*GPIOC::ptr() };
        rcc.ahb4enr.modify(|_, w| w.gpiocen().set_bit());
        gpioc.moder.modify(|_, w| w.moder11().output());
        gpioc.odr.modify(|_, w| w.odr11().set_bit());
    }};
    (b, off) => {{
        use daisy_bsp::hal::pac::{GPIOC, RCC};
        let rcc = unsafe { &*RCC::ptr() };
        let gpioc = unsafe { &*GPIOC::ptr() };
        rcc.ahb4enr.modify(|_, w| w.gpiocen().set_bit());
        gpioc.moder.modify(|_, w| w.moder11().output());
        gpioc.odr.modify(|_, w| w.odr11().clear_bit());
    }};
}
