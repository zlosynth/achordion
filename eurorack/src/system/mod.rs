pub mod audio;
pub mod button;
pub mod cv;
pub mod flash;
pub mod led;
pub mod led_user;
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
use led_user::LedUser;
use pot::Pot;
use probe::Probe as ProbeWrapper;

pub type Button = ButtonWrapper<gpio::gpiob::PB4<gpio::Input>>; // PIN D1
pub type Pot1 = Pot<ADC2, gpio::gpioa::PA7<gpio::Analog>>; // PIN C2
pub type Pot2 = Pot<ADC1, gpio::gpioa::PA6<gpio::Analog>>; // PIN C4
pub type Pot3 = Pot<ADC2, gpio::gpiob::PB1<gpio::Analog>>; // PIN C8
pub type Pot4 = Pot<ADC1, gpio::gpioa::PA1<gpio::Analog>>; // PIN A2
pub type Cv1 = Cv<ADC1, gpio::gpioa::PA3<gpio::Analog>>; // PIN C5
pub type Cv2 = Cv<ADC2, gpio::gpioc::PC1<gpio::Analog>>; // PIN C6
pub type Cv3 = Cv<ADC1, gpio::gpioc::PC0<gpio::Analog>>; // PIN C7
pub type Cv4 = Cv<ADC2, gpio::gpioc::PC4<gpio::Analog>>; // PIN C9
pub type Cv5 = Cv<ADC1, gpio::gpioa::PA2<gpio::Analog>>; // PIN C3
pub type Probe = ProbeWrapper<gpio::gpioc::PC11<gpio::Output<gpio::PushPull>>>; // PIN D2
pub type Led1 = Led<gpio::gpiod::PD3<gpio::Output<gpio::PushPull>>>; // PIN D10
pub type Led2 = Led<gpio::gpioc::PC3<gpio::Output<gpio::PushPull>>>; // PIN D9
pub type Led3 = Led<gpio::gpioc::PC2<gpio::Output<gpio::PushPull>>>; // PIN D8
pub type Led4 = Led<gpio::gpiod::PD2<gpio::Output<gpio::PushPull>>>; // PIN D7
pub type Led5 = Led<gpio::gpioc::PC12<gpio::Output<gpio::PushPull>>>; // PIN D6
pub type Led6 = Led<gpio::gpioc::PC8<gpio::Output<gpio::PushPull>>>; // PIN D5
pub type Led7 = Led<gpio::gpioc::PC9<gpio::Output<gpio::PushPull>>>; // PIN D4
pub type Led8 = Led<gpio::gpioc::PC10<gpio::Output<gpio::PushPull>>>; // PIN D3

pub struct System {
    pub adc1: Adc<ADC1, Enabled>,
    pub adc2: Adc<ADC2, Enabled>,
    pub cvs: Cvs,
    pub pots: Pots,
    pub button: Button,
    pub leds: Leds,
    pub flash: Flash,
    pub audio: Audio,
    pub led_user: LedUser,
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
        let ccdr = daisy::board_freeze_clocks!(board, dp);
        let pins = daisy::board_split_gpios!(board, ccdr, dp);

        let pots = Pots {
            pot1: Pot1::new(pins.GPIO.PIN_C2, (0.5, 0.0)),
            pot2: Pot2::new(pins.GPIO.PIN_C4, (0.5, 0.0)),
            pot3: Pot3::new(pins.GPIO.PIN_C8, (0.5, 0.0)),
            pot4: Pot4::new(pins.GPIO.PIN_A2, (0.0, 1.0)),
        };

        let cvs = Cvs {
            cv1: Cv1::new(pins.GPIO.PIN_C5, (-5.0, 5.0)),
            cv2: Cv2::new(pins.GPIO.PIN_C6, (-5.0, 5.0)),
            cv3: Cv3::new(pins.GPIO.PIN_C7, (-5.0, 5.0)),
            cv4: Cv4::new(pins.GPIO.PIN_C9, (-5.0, 5.0)),
            cv5: Cv5::new(pins.GPIO.PIN_C3, (-5.0, 5.0)),
            cv_probe: Probe::new(pins.GPIO.PIN_D2.into_push_pull_output()),
        };

        let button = Button::new(pins.GPIO.PIN_D1.into_pull_up_input());

        let leds = Leds {
            led1: Led::new(pins.GPIO.PIN_D10.into_push_pull_output()),
            led2: Led::new(pins.GPIO.PIN_D9.into_push_pull_output()),
            led3: Led::new(pins.GPIO.PIN_D8.into_push_pull_output()),
            led4: Led::new(pins.GPIO.PIN_D7.into_push_pull_output()),
            led5: Led::new(pins.GPIO.PIN_D6.into_push_pull_output()),
            led6: Led::new(pins.GPIO.PIN_D5.into_push_pull_output()),
            led7: Led::new(pins.GPIO.PIN_D4.into_push_pull_output()),
            led8: Led::new(pins.GPIO.PIN_D3.into_push_pull_output()),
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
        let led_user = daisy::board_split_leds!(pins).USER;

        System {
            adc1,
            adc2,
            cvs,
            pots,
            button,
            leds,
            flash,
            audio,
            led_user,
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
