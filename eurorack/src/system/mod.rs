// TODO: This is mean to abstract any hardware lib that may be used
// TODO: And also hides the board itself
// TODO: Define abstraction for system initialization
// TODO: Define abstraction for peripherals

mod hal;
pub mod peripherals;

use rtic::Peripherals as CorePeripherals;

use daisy::audio;
use daisy::flash::Flash;
use daisy::pac;
use daisy::pac::Peripherals as DevicePeripherals;
use daisy_bsp as daisy;
use hal::adc::Adc;
use hal::delay::DelayFromCountDownTimer;
use hal::pac::DWT;
use hal::prelude::*;

pub struct System<'a> {
    pub adc: hal::adc::Adc<pac::ADC1, hal::adc::Disabled>,
    pub audio: audio::Interface<'a>,
    pub flash: Flash,
    pub pins: Pins,
}

// XXX: Temporary until this module returns final abstractions of peripherals
#[allow(non_snake_case)]
pub struct Pins {
    pub SEED_PIN_9: hal::gpio::gpiob::PB4<hal::gpio::Alternate<hal::gpio::AF0>>,
    pub SEED_PIN_23: hal::gpio::gpioa::PA4<hal::gpio::Analog>,
    pub SEED_PIN_24: hal::gpio::gpioa::PA1<hal::gpio::Analog>,
    pub SEED_PIN_22: hal::gpio::gpioa::PA5<hal::gpio::Analog>,
    pub SEED_PIN_21: hal::gpio::gpioc::PC4<hal::gpio::Analog>,
    pub SEED_PIN_20: hal::gpio::gpioc::PC1<hal::gpio::Analog>,
    pub SEED_PIN_19: hal::gpio::gpioa::PA6<hal::gpio::Analog>,
    pub SEED_PIN_15: hal::gpio::gpioc::PC0<hal::gpio::Analog>,
    pub SEED_PIN_16: hal::gpio::gpioa::PA3<hal::gpio::Analog>,
    pub SEED_PIN_17: hal::gpio::gpiob::PB1<hal::gpio::Analog>,
    pub SEED_PIN_18: hal::gpio::gpioa::PA7<hal::gpio::Analog>,
    pub SEED_PIN_10: hal::gpio::gpiob::PB5<hal::gpio::Analog>,
    pub SEED_PIN_30: hal::gpio::gpiob::PB15<hal::gpio::Analog>,
    pub SEED_PIN_29: hal::gpio::gpiob::PB14<hal::gpio::Analog>,
    pub SEED_PIN_26: hal::gpio::gpiod::PD11<hal::gpio::Analog>,
    pub SEED_PIN_25: hal::gpio::gpioa::PA0<hal::gpio::Analog>,
    pub SEED_PIN_3: hal::gpio::gpioc::PC9<hal::gpio::Analog>,
    pub SEED_PIN_4: hal::gpio::gpioc::PC8<hal::gpio::Analog>,
    pub SEED_PIN_5: hal::gpio::gpiod::PD2<hal::gpio::Analog>,
    pub SEED_PIN_6: hal::gpio::gpioc::PC12<hal::gpio::Analog>,
}

impl System<'_> {
    pub fn init(mut cp: CorePeripherals, dp: DevicePeripherals) -> Self {
        enable_cache(&mut cp);
        initialize_timers(&mut cp);

        let board = daisy::Board::take().unwrap();

        let rcc = dp.RCC.constrain().pll2_p_ck(4.mhz());
        let ccdr = board.freeze_clocks(dp.PWR.constrain(), rcc, &dp.SYSCFG);

        let pins = board.split_gpios(
            dp.GPIOA.split(ccdr.peripheral.GPIOA),
            dp.GPIOB.split(ccdr.peripheral.GPIOB),
            dp.GPIOC.split(ccdr.peripheral.GPIOC),
            dp.GPIOD.split(ccdr.peripheral.GPIOD),
            dp.GPIOE.split(ccdr.peripheral.GPIOE),
            dp.GPIOF.split(ccdr.peripheral.GPIOF),
            dp.GPIOG.split(ccdr.peripheral.GPIOG),
        );

        let flash =
            daisy::flash::Flash::new(&ccdr.clocks, dp.QUADSPI, ccdr.peripheral.QSPI, pins.FMC);

        let mut delay = DelayFromCountDownTimer::new(dp.TIM2.timer(
            10.ms(),
            ccdr.peripheral.TIM2,
            &ccdr.clocks,
        ));
        let adc = Adc::adc1(dp.ADC1, &mut delay, ccdr.peripheral.ADC12, &ccdr.clocks);

        let ak_pins = (
            pins.AK4556.PDN.into_push_pull_output(),
            pins.AK4556.MCLK_A.into_alternate_af6(),
            pins.AK4556.SCK_A.into_alternate_af6(),
            pins.AK4556.FS_A.into_alternate_af6(),
            pins.AK4556.SD_A.into_alternate_af6(),
            pins.AK4556.SD_B.into_alternate_af6(),
        );

        let sai1_prec = ccdr
            .peripheral
            .SAI1
            .kernel_clk_mux(hal::rcc::rec::Sai1ClkSel::PLL3_P);

        let audio_interface =
            audio::Interface::init(&ccdr.clocks, sai1_prec, ak_pins, ccdr.peripheral.DMA1).unwrap();

        let pins = Pins {
            SEED_PIN_9: pins.SEED_PIN_9,
            SEED_PIN_23: pins.SEED_PIN_23,
            SEED_PIN_24: pins.SEED_PIN_24,
            SEED_PIN_22: pins.SEED_PIN_22,
            SEED_PIN_21: pins.SEED_PIN_21,
            SEED_PIN_20: pins.SEED_PIN_20,
            SEED_PIN_19: pins.SEED_PIN_19,
            SEED_PIN_15: pins.SEED_PIN_15,
            SEED_PIN_16: pins.SEED_PIN_16,
            SEED_PIN_17: pins.SEED_PIN_17,
            SEED_PIN_18: pins.SEED_PIN_18,
            SEED_PIN_10: pins.SEED_PIN_10,
            SEED_PIN_30: pins.SEED_PIN_30,
            SEED_PIN_29: pins.SEED_PIN_29,
            SEED_PIN_26: pins.SEED_PIN_26,
            SEED_PIN_25: pins.SEED_PIN_25,
            SEED_PIN_3: pins.SEED_PIN_3,
            SEED_PIN_4: pins.SEED_PIN_4,
            SEED_PIN_5: pins.SEED_PIN_5,
            SEED_PIN_6: pins.SEED_PIN_6,
        };

        Self {
            adc,
            audio: audio_interface,
            flash,
            pins, // TODO: Create a special struct with remaining pins
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
