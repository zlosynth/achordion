pub mod audio;
pub mod button;
pub mod cv;
pub mod flash;
pub mod led;
pub mod pot;
pub mod probe;

mod control_buffer;
mod hal;

use hal::adc::{Adc, AdcSampleTime, Enabled, Resolution};
use hal::delay::DelayFromCountDownTimer;
use hal::gpio;
use hal::pac::Peripherals as DevicePeripherals;
use hal::pac::DWT;
use hal::pac::{ADC1, ADC2};
use hal::prelude::*;
use rtic::Peripherals as CorePeripherals;

use audio::{Audio, AudioPins};
use button::Button as ButtonWrapper;
use cv::Cv;
use flash::Flash;
use led::Led;
use pot::Pot;
use probe::Probe as ProbeWrapper;

pub type Button = ButtonWrapper<gpio::gpiob::PB4<gpio::Input<gpio::PullUp>>>; // PIN 9
pub type Pot1 = Pot<gpio::gpioa::PA4<gpio::Analog>>; // PIN 23
pub type Pot2 = Pot<gpio::gpioa::PA1<gpio::Analog>>; // PIN 24
pub type Pot3 = Pot<gpio::gpioa::PA5<gpio::Analog>>; // PIN 22
pub type Pot4 = Pot<gpio::gpioc::PC4<gpio::Analog>>; // PIN 21
pub type Cv1 = Cv<ADC2, gpio::gpioc::PC1<gpio::Analog>>; // PIN 20
pub type Cv2 = Cv<ADC2, gpio::gpioa::PA6<gpio::Analog>>; // PIN 19
pub type Cv3 = Cv<ADC2, gpio::gpioc::PC0<gpio::Analog>>; // PIN 15
pub type Cv4 = Cv<ADC2, gpio::gpioa::PA3<gpio::Analog>>; // PIN 16
pub type Cv5 = Cv<ADC2, gpio::gpiob::PB1<gpio::Analog>>; // PIN 17
pub type Cv6 = Cv<ADC1, gpio::gpioa::PA7<gpio::Analog>>; // PIN 18
pub type Probe = ProbeWrapper<gpio::gpiob::PB5<gpio::Output<gpio::PushPull>>>; // PIN 10
pub type Led1 = Led<gpio::gpiob::PB15<gpio::Output<gpio::PushPull>>>; // PIN 30
pub type Led2 = Led<gpio::gpiob::PB14<gpio::Output<gpio::PushPull>>>; // PIN 29
pub type Led3 = Led<gpio::gpiod::PD11<gpio::Output<gpio::PushPull>>>; // PIN 26
pub type Led4 = Led<gpio::gpioa::PA0<gpio::Output<gpio::PushPull>>>; // PIN 25
pub type Led5 = Led<gpio::gpioc::PC9<gpio::Output<gpio::PushPull>>>; // PIN 3
pub type Led6 = Led<gpio::gpioc::PC8<gpio::Output<gpio::PushPull>>>; // PIN 4
pub type Led7 = Led<gpio::gpiod::PD2<gpio::Output<gpio::PushPull>>>; // PIN 5
pub type Led8 = Led<gpio::gpioc::PC12<gpio::Output<gpio::PushPull>>>; // PIN 6

pub struct System<'a> {
    pub adc1: Adc<ADC1, Enabled>,
    pub adc2: Adc<ADC2, Enabled>,
    pub cvs: Cvs,
    pub pots: Pots,
    pub button: Button,
    pub leds: Leds,
    pub flash: Flash,
    pub audio: Audio<'a>,
}

pub struct Cvs {
    pub cv1: Cv1,
    pub cv2: Cv2,
    pub cv3: Cv3,
    pub cv4: Cv4,
    pub cv5: Cv5,
    pub cv6: Cv6,
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

impl System<'_> {
    pub fn init(mut cp: CorePeripherals, dp: DevicePeripherals) -> Self {
        enable_cache(&mut cp);
        initialize_timers(&mut cp);

        let board = daisy_bsp::Board::take().unwrap();

        let ccdr = {
            let rcc = dp.RCC.constrain().pll2_p_ck(4.mhz());
            board.freeze_clocks(dp.PWR.constrain(), rcc, &dp.SYSCFG)
        };

        let pins = board.split_gpios(
            dp.GPIOA.split(ccdr.peripheral.GPIOA),
            dp.GPIOB.split(ccdr.peripheral.GPIOB),
            dp.GPIOC.split(ccdr.peripheral.GPIOC),
            dp.GPIOD.split(ccdr.peripheral.GPIOD),
            dp.GPIOE.split(ccdr.peripheral.GPIOE),
            dp.GPIOF.split(ccdr.peripheral.GPIOF),
            dp.GPIOG.split(ccdr.peripheral.GPIOG),
        );

        let pots = Pots {
            pot1: Pot::new(pins.SEED_PIN_23),
            pot2: Pot::new(pins.SEED_PIN_24),
            pot3: Pot::new(pins.SEED_PIN_22),
            pot4: Pot::new(pins.SEED_PIN_21),
        };

        let cvs = Cvs {
            // TODO: Replace by type aliases?
            cv1: Cv::<ADC2, _>::new(pins.SEED_PIN_20, (0.0, 10.0)),
            cv2: Cv::<ADC2, _>::new(pins.SEED_PIN_19, (0.0, 10.0)),
            cv3: Cv::<ADC2, _>::new(pins.SEED_PIN_15, (-5.0, 5.0)),
            cv4: Cv::<ADC2, _>::new(pins.SEED_PIN_16, (-5.0, 5.0)),
            cv5: Cv::<ADC2, _>::new(pins.SEED_PIN_17, (-5.0, 5.0)),
            cv6: Cv::<ADC1, _>::new(pins.SEED_PIN_18, (-5.0, 5.0)),
            cv_probe: Probe::new(pins.SEED_PIN_10.into_push_pull_output()),
        };

        let button = Button::new(pins.SEED_PIN_9.into_pull_up_input());

        let leds = Leds {
            led1: Led::new(pins.SEED_PIN_30.into_push_pull_output()),
            led2: Led::new(pins.SEED_PIN_29.into_push_pull_output()),
            led3: Led::new(pins.SEED_PIN_26.into_push_pull_output()),
            led4: Led::new(pins.SEED_PIN_25.into_push_pull_output()),
            led5: Led::new(pins.SEED_PIN_3.into_push_pull_output()),
            led6: Led::new(pins.SEED_PIN_4.into_push_pull_output()),
            led7: Led::new(pins.SEED_PIN_5.into_push_pull_output()),
            led8: Led::new(pins.SEED_PIN_6.into_push_pull_output()),
        };

        let (adc1, adc2) = {
            let mut delay = DelayFromCountDownTimer::new(dp.TIM2.timer(
                10.ms(),
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
            adc1.set_sample_time(AdcSampleTime::T_64);
            adc2.set_resolution(Resolution::SIXTEENBIT);
            adc2.set_sample_time(AdcSampleTime::T_64);
            (adc1.enable(), adc2.enable())
        };

        let flash = Flash::new(&ccdr.clocks, dp.QUADSPI, ccdr.peripheral.QSPI, pins.FMC);

        let audio = {
            let audio_pins = AudioPins {
                pdn: pins.AK4556.PDN.into_push_pull_output(),
                mclk_a: pins.AK4556.MCLK_A.into_alternate_af6(),
                sck_a: pins.AK4556.SCK_A.into_alternate_af6(),
                fs_a: pins.AK4556.FS_A.into_alternate_af6(),
                sd_a: pins.AK4556.SD_A.into_alternate_af6(),
                sd_b: pins.AK4556.SD_B.into_alternate_af6(),
            };

            let sai = ccdr
                .peripheral
                .SAI1
                .kernel_clk_mux(hal::rcc::rec::Sai1ClkSel::PLL3_P);

            Audio::init(audio_pins, &ccdr.clocks, sai, ccdr.peripheral.DMA1)
        };

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
