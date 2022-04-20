pub mod audio;
pub mod button;
pub mod cv;
pub mod flash;
pub mod led;
pub mod pot;
pub mod probe;

mod control_buffer;
mod debounce_buffer;
mod hal;

use hal::adc::{Adc, AdcSampleTime, Enabled, Resolution};
use hal::delay::DelayFromCountDownTimer;
use hal::gpio;
use hal::pac::Peripherals as DevicePeripherals;
use hal::pac::DWT;
use hal::pac::{ADC1, ADC2};
use hal::prelude::*;
use rtic::Peripherals as CorePeripherals;

use audio::Audio;
use button::Button as ButtonWrapper;
use cv::Cv;
use flash::Flash;
use led::Led;
use pot::Pot;
use probe::Probe as ProbeWrapper;

pub type Button = ButtonWrapper<gpio::gpiog::PG11<gpio::Input>>; // PIN 8
pub type Pot1 = Pot<ADC2, gpio::gpioc::PC1<gpio::Analog>>; // PIN 20
pub type Pot2 = Pot<ADC1, gpio::gpioc::PC4<gpio::Analog>>; // PIN 21
pub type Pot3 = Pot<ADC1, gpio::gpioa::PA4<gpio::Analog>>; // PIN 23
pub type Pot4 = Pot<ADC2, gpio::gpioa::PA5<gpio::Analog>>; // PIN 22
pub type Cv1 = Cv<ADC1, gpio::gpiob::PB1<gpio::Analog>>; // PIN 17
pub type Cv2 = Cv<ADC2, gpio::gpioa::PA3<gpio::Analog>>; // PIN 16
pub type Cv3 = Cv<ADC1, gpio::gpioa::PA6<gpio::Analog>>; // PIN 19
pub type Cv4 = Cv<ADC2, gpio::gpioa::PA7<gpio::Analog>>; // PIN 18
pub type Cv5 = Cv<ADC1, gpio::gpioc::PC0<gpio::Analog>>; // PIN 15
pub type Probe = ProbeWrapper<gpio::gpiob::PB5<gpio::Output<gpio::PushPull>>>; // PIN 10
pub type Led1 = Led<gpio::gpiog::PG10<gpio::Output<gpio::PushPull>>>; // PIN 7
pub type Led2 = Led<gpio::gpioc::PC12<gpio::Output<gpio::PushPull>>>; // PIN 6
pub type Led3 = Led<gpio::gpiod::PD2<gpio::Output<gpio::PushPull>>>; // PIN 5
pub type Led4 = Led<gpio::gpioc::PC8<gpio::Output<gpio::PushPull>>>; // PIN 4
pub type Led5 = Led<gpio::gpioc::PC9<gpio::Output<gpio::PushPull>>>; // PIN 3
pub type Led6 = Led<gpio::gpioc::PC10<gpio::Output<gpio::PushPull>>>; // PIN 2
pub type Led7 = Led<gpio::gpioc::PC11<gpio::Output<gpio::PushPull>>>; // PIN 1
pub type Led8 = Led<gpio::gpiob::PB12<gpio::Output<gpio::PushPull>>>; // PIN 0

pub struct System {
    pub adc1: Adc<ADC1, Enabled>,
    pub adc2: Adc<ADC2, Enabled>,
    pub cvs: Cvs,
    pub pots: Pots,
    pub button: Button,
    pub leds: Leds,
    pub flash: Flash,
    pub audio: Audio,
}

pub struct Cvs {
    pub cv1: Cv1,
    pub cv2: Cv2,
    pub cv3: Cv3,
    pub cv4: Cv4,
    pub cv5: Cv5,
    pub cv_probe: Probe,
}

pub struct Pots {
    pub pot1: Pot1,
    pub pot2: Pot2,
    pub pot3: Pot3,
    pub pot4: Pot4,
}

pub struct Leds {
    pub led1: Led1,
    pub led2: Led2,
    pub led3: Led3,
    pub led4: Led4,
    pub led5: Led5,
    pub led6: Led6,
    pub led7: Led7,
    pub led8: Led8,
}

impl System {
    pub fn init(mut cp: CorePeripherals, dp: DevicePeripherals) -> Self {
        enable_cache(&mut cp);
        initialize_timers(&mut cp);

        let board = daisy::Board::take().unwrap();

        // TODO: Needed?
        let ccdr = {
            let rcc = dp.RCC.constrain().pll2_p_ck(4.MHz());
            board.freeze_clocks(dp.PWR.constrain(), rcc, &dp.SYSCFG)
        };
        // let ccdr = daisy::board_freeze_clocks!(board, dp);
        let pins = daisy::board_split_gpios!(board, ccdr, dp);

        let pots = Pots {
            pot1: Pot1::new(pins.GPIO.PIN_20),
            pot2: Pot2::new(pins.GPIO.PIN_21),
            pot3: Pot3::new(pins.GPIO.PIN_23),
            pot4: Pot4::new(pins.GPIO.PIN_22),
        };

        let cvs = Cvs {
            cv1: Cv1::new(pins.GPIO.PIN_17, (0.0, 10.0)),
            cv2: Cv2::new(pins.GPIO.PIN_16, (0.0, 10.0)),
            cv3: Cv3::new(pins.GPIO.PIN_19, (-5.0, 5.0)),
            cv4: Cv4::new(pins.GPIO.PIN_18, (-5.0, 5.0)),
            cv5: Cv5::new(pins.GPIO.PIN_15, (0.0, 10.0)),
            cv_probe: Probe::new(pins.GPIO.PIN_10.into_push_pull_output()),
        };

        let button = Button::new(pins.GPIO.PIN_8.into_pull_up_input());

        let leds = Leds {
            led1: Led::new(pins.GPIO.PIN_7.into_push_pull_output()),
            led2: Led::new(pins.GPIO.PIN_6.into_push_pull_output()),
            led3: Led::new(pins.GPIO.PIN_5.into_push_pull_output()),
            led4: Led::new(pins.GPIO.PIN_4.into_push_pull_output()),
            led5: Led::new(pins.GPIO.PIN_3.into_push_pull_output()),
            led6: Led::new(pins.GPIO.PIN_2.into_push_pull_output()),
            led7: Led::new(pins.GPIO.PIN_1.into_push_pull_output()),
            led8: Led::new(pins.GPIO.PIN_0.into_push_pull_output()),
        };

        let (adc1, adc2) = {
            let mut delay = DelayFromCountDownTimer::new(dp.TIM2.timer(
                100.Hz(),
                ccdr.peripheral.TIM2,
                &ccdr.clocks,
            ));
            let (mut adc1, mut adc2) = hal::adc::adc12(
                dp.ADC1,
                dp.ADC2,
                &mut delay,
                ccdr.peripheral.ADC12,
                &ccdr.clocks,
            );
            adc1.set_resolution(Resolution::SIXTEENBIT);
            adc1.set_sample_time(AdcSampleTime::T_16);
            adc2.set_resolution(Resolution::SIXTEENBIT);
            adc2.set_sample_time(AdcSampleTime::T_16);
            (adc1.enable(), adc2.enable())
        };

        let flash = daisy::board_split_flash!(ccdr, dp, pins);
        let audio = Audio::init(daisy::board_split_audio!(ccdr, pins));

        System {
            adc1,
            adc2,
            cvs,
            pots,
            button,
            leds,
            flash,
            audio,
        }
    }
}

/// AN5212: Improve application performance when fetching instruction and
/// data, from both internal andexternal memories.
fn enable_cache(cp: &mut CorePeripherals) {
    cp.SCB.enable_icache();
}

/// Initialize (enable) the monotonic timer (CYCCNT)
fn initialize_timers(cp: &mut CorePeripherals) {
    cp.DCB.enable_trace();
    DWT::unlock();
    cp.DWT.enable_cycle_counter();
}
