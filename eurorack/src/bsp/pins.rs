use stm32h7xx_hal as hal;

pub type PinButton = hal::gpio::gpiob::PB4<hal::gpio::Alternate<hal::gpio::AF0>>;
pub type PinCV1 = hal::gpio::gpioc::PC1<hal::gpio::Analog>;
pub type PinCV2 = hal::gpio::gpioa::PA6<hal::gpio::Analog>;
pub type PinCV3 = hal::gpio::gpioc::PC0<hal::gpio::Analog>;
pub type PinCV4 = hal::gpio::gpioa::PA3<hal::gpio::Analog>;
pub type PinCV5 = hal::gpio::gpiob::PB1<hal::gpio::Analog>;
pub type PinCV6 = hal::gpio::gpioa::PA7<hal::gpio::Analog>;
pub type PinLed1 = hal::gpio::gpiob::PB15<hal::gpio::Analog>;
pub type PinLed2 = hal::gpio::gpiob::PB14<hal::gpio::Analog>;
pub type PinLed3 = hal::gpio::gpiod::PD11<hal::gpio::Analog>;
pub type PinLed4 = hal::gpio::gpioa::PA0<hal::gpio::Analog>;
pub type PinLed5 = hal::gpio::gpioc::PC9<hal::gpio::Analog>;
pub type PinLed6 = hal::gpio::gpioc::PC8<hal::gpio::Analog>;
pub type PinLed7 = hal::gpio::gpiod::PD2<hal::gpio::Analog>;
pub type PinLed8 = hal::gpio::gpioc::PC12<hal::gpio::Analog>;
pub type PinPot1 = hal::gpio::gpioa::PA4<hal::gpio::Analog>;
pub type PinPot2 = hal::gpio::gpioa::PA1<hal::gpio::Analog>;
pub type PinPot3 = hal::gpio::gpioa::PA5<hal::gpio::Analog>;
pub type PinPot4 = hal::gpio::gpioc::PC4<hal::gpio::Analog>;
pub type PinProbe = hal::gpio::gpiob::PB5<hal::gpio::Analog>;

pub type LedUser = hal::gpio::gpioc::PC7<hal::gpio::Analog>;

#[allow(non_snake_case)]
pub struct Pins {
    pub PIN_BUTTON: PinButton,
    pub PIN_CV1: PinCV1,
    pub PIN_CV2: PinCV2,
    pub PIN_CV3: PinCV3,
    pub PIN_CV4: PinCV4,
    pub PIN_CV5: PinCV5,
    pub PIN_CV6: PinCV6,
    pub PIN_LED1: PinLed1,
    pub PIN_LED2: PinLed2,
    pub PIN_LED3: PinLed3,
    pub PIN_LED4: PinLed4,
    pub PIN_LED5: PinLed5,
    pub PIN_LED6: PinLed6,
    pub PIN_LED7: PinLed7,
    pub PIN_LED8: PinLed8,
    pub PIN_POT1: PinPot1,
    pub PIN_POT2: PinPot2,
    pub PIN_POT3: PinPot3,
    pub PIN_POT4: PinPot4,
    pub PIN_PROBE: PinProbe,

    pub LED_USER: LedUser,
}
