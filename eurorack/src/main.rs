//!
//! # I2S example for STM32F411
//!
//! This application demonstrates I2S communication with the DAC on an STM32F411E-DISCO board.
//! Unlike `i2s-audio-out`, this example uses DMA instead of writing one sample at a time.
//!
//! # Hardware required
//!
//! * STM32F411E-DISCO evaluation board
//! * Headphones or speakers with a headphone plug
//!
//! # Procedure
//!
//! 1. Connect the headphones or speakers to the headphone jack on the evaluation board
//!    (warning: the DAC may produce a powerful signal that becomes a very loud sound.
//!    Set the speaker volume to minimum, or do not put on the headphones.)
//! 2. Load this compiled application on the microcontroller and run it
//!
//! Expected behavior: the speakers/headphones emit a continuous 750 Hz tone
//!
//! # Pins and addresses
//!
//! * PD4 -> DAC ~RESET (pulled low)
//!
//! * PB9 -> SDA (pulled high)
//! * PB6 -> SCL (pulled high)
//!
//! * PC7 -> MCLK
//! * PC10 -> SCK (bit clock)
//! * PC12 -> SD
//! * PA4 -> WS
//!
//! DAC I2C address 0x94
//!

#![no_std]
#![no_main]

mod cs43l22;

use core::cell::RefCell;

use panic_halt as _;

use rtic::app;

use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

use stm32_i2s_v12x::format::{Data16Frame16, FrameFormat};
use stm32_i2s_v12x::{MasterClock, MasterConfig, Polarity, TransmitMode};

use stm32f4xx_hal::delay::Delay;
use stm32f4xx_hal::dma::config::{DmaConfig, Priority};
use stm32f4xx_hal::dma::MemoryToPeripheral;
use stm32f4xx_hal::dma::traits::{Stream, PeriAddress,Direction};
use stm32f4xx_hal::dma::{Channel0, Stream5, StreamsTuple, Transfer, DmaDirection};
use stm32f4xx_hal::gpio::gpioa::PA4;
use stm32f4xx_hal::gpio::gpioc::{PC10, PC12, PC7};
use stm32f4xx_hal::gpio::Alternate;
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::i2s::I2s;
use stm32f4xx_hal::pac::{interrupt, Interrupt};
use stm32f4xx_hal::pac::{CorePeripherals, Peripherals};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32::{DMA1, SPI3};

use cs43l22::{Cs43L22, Register};

const BUFFER_SIZE: usize = 64;

/// A sine wave spanning 64 samples
///
/// With a sample rate of 48 kHz, this produces a 750 Hz tone.
const SINE_750: [i16; BUFFER_SIZE] = [
    // 0, 3211, 6392, 9511, 12539, 15446, 18204, 20787, 23169, 0, 27244, 28897, 30272, 0,
    // 32137, 32609, 32767, 32609, 32137, 31356, 30272, 28897, 0, 25329, 23169, 20787, 18204,
    // 15446, 12539, 9511, 6392, 3211, 0, -3211, -6392, 0, -12539, -15446, -18204, -20787, -23169,
    // -25329, -27244, -28897, -30272, -31356, -32137, 0, -32767, -32609, -32137, -31356, -30272,
    // -28897, -27244, -25329, -23169, -20787, -18204, 0, -12539, -9511, -6392, -3211,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
    -28000,
];

// Make a copy of SINE_750 with three differences:
// 1. It's a way to get a &'static mut
// 2. Its type is u16 instead of i16
// 3. Each sample is repeated (for the left and right channels)
static mut BUFFER: [u16; BUFFER_SIZE * 2] = [0; BUFFER_SIZE * 2];

/// DMA transfer handoff from init() to interrupt handler
static G_TRANSFER: Mutex<RefCell<Option<Stream5<DMA1>>>> = Mutex::new(RefCell::new(None));

#[app(device = stm32f4xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    #[init]
    fn init(mut cx: init::Context) {
        let cp = cx.core;
        let dp = cx.device;

        let rcc = dp.RCC.constrain();
        // The 86 MHz frequency can be divided to get a sample rate very close to 48 kHz.
        let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(168.mhz()).i2s_clk(86.mhz()).freeze();

        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();
        let gpioc = dp.GPIOC.split();
        let gpiod = dp.GPIOD.split();

        let mut delay = Delay::new(cp.SYST, clocks);

        let i2c = I2c::i2c1(
            dp.I2C1,
            (
                gpiob.pb6.into_alternate_af4_open_drain(),
                gpiob.pb9.into_alternate_af4_open_drain(),
            ),
            100.khz(),
            clocks,
        );
        // Shift the address to deal with different ways of representing I2C addresses
        let mut dac = Cs43L22::new(i2c, 0x94 >> 1);

        let mut dac_reset = gpiod.pd4.into_push_pull_output();

        // I2S pins: (WS, CK, MCLK, SD) for I2S3
        let i2s_pins = (
            gpioa.pa4.into_alternate_af6(),
            gpioc.pc10.into_alternate_af6(),
            gpioc.pc7.into_alternate_af6(),
            gpioc.pc12.into_alternate_af6(),
        );
        let hal_i2s = I2s::i2s3(dp.SPI3, i2s_pins, clocks);
        let i2s_clock = hal_i2s.input_clock();

        // Audio timing configuration:
        // Sample rate 48 kHz
        // 16 bits per sample -> SCK rate 1.536 MHz
        // MCK frequency = 256 * sample rate -> MCK rate 12.228 MHz (also equal to 8 * SCK rate)
        let sample_rate = 48000;

        let i2s = stm32_i2s_v12x::I2s::new(hal_i2s);
        let mut i2s = i2s.configure_master_transmit(MasterConfig::with_sample_rate(
            i2s_clock.0,
            sample_rate,
            Data16Frame16,
            FrameFormat::PhilipsI2s,
            Polarity::IdleHigh,
            MasterClock::Enable,
        ));
        i2s.set_dma_enabled(true);

        // Keep DAC reset low for at least one millisecond
        delay.delay_ms(1u8);
        // Release the DAC from reset
        dac_reset.set_high().unwrap();
        // Wait at least 550 ns before starting I2C communication
        delay.delay_us(1u8);

        dac.basic_setup().unwrap();
        // Clocking control from the table in section 4.6 of the datasheet:
        // Auto mode: disabled
        // Speed mode: 01 (single-speed)
        // 8 kHz, 16 kHz, or 32 kHz sample rate: no
        // 27 MHz video clock: no
        // Internal MCLK/LRCLCK ratio: 00
        // MCLK divide by 2: no
        dac.write(Register::ClockingCtl, 0b0_01_0_0_00_0).unwrap();
        // Interface control:
        // Slave mode
        // SCLK not inverted
        // DSP mode disabled
        // Interface format I2S
        // Word length 16 bits
        dac.write(Register::InterfaceCtl1, 0b0_0_0_0_01_11).unwrap();

        // Reduce the headphone volume to make the demo less annoying
        let headphone_volume = -10i8 as u8;
        dac.write(Register::HeadphoneAVol, headphone_volume)
            .unwrap();
        dac.write(Register::HeadphoneBVol, headphone_volume)
            .unwrap();

        // Power up DAC
        dac.write(Register::PowerCtl1, 0b1001_1110).unwrap();

        unsafe {
            // Copy samples from flash into the buffer that DMA will use
            let mut dest_iter = BUFFER.iter_mut();
            for sample in SINE_750.iter() {
                // Duplicate sample for the left and right channels
                let left = dest_iter.next().unwrap();
                let right = dest_iter.next().unwrap();
                *left = *sample as u16;
                *right = *sample as u16;
            }
        }

        // Set up DMA: DMA 1 stream 5 channel 0 memory -> peripheral
        let dma1_streams = StreamsTuple::new(dp.DMA1);

        let ma = unsafe { BUFFER.as_ptr() } as usize as u32; // source: memory address
        let pa = i2s.address(); // destination: Dual DAC 12-bit right-aligned data holding register (DHR12RD)
        let ndt = unsafe {BUFFER.len() as u16}; // number of items to transfer

        let mut stream = dma1_streams.5;
        stream.set_channel(Channel0);
        stream.set_direction(MemoryToPeripheral);
        unsafe {stream.set_memory_address(ma);}
        stream.set_memory_increment(true);
        unsafe {stream.set_memory_size(1);}
        unsafe {stream.set_peripheral_address(pa);}
        stream.set_peripheral_increment(false);
        unsafe {stream.set_peripheral_size(1);}
        stream.set_number_of_transfers(ndt);
        let cr = unsafe { &(*DMA1::ptr()).st[5].cr };
        cr.modify(|_, w| w.circ().bit(true));
        stream.set_priority(Priority::VeryHigh);
        stream.set_interrupts_enable(true, true, true, true);
        unsafe {stream.enable();}
        i2s.enable();

        // Hand off transfer to interrupt handler
        cortex_m::interrupt::free(|cs| *G_TRANSFER.borrow(cs).borrow_mut() = Some(stream));
        // Enable interrupt
        unsafe {
            cortex_m::peripheral::NVIC::unmask(Interrupt::DMA1_STREAM5);
        }
    }

    /// This interrupt handler runs when DMA 1 finishes a transfer to the I2S peripheral
    #[task(binds = DMA1_STREAM5)]
    fn dsp_request(cx: dsp_request::Context) {
        // TODO: Use circular DMA instead of transfer API

        static mut TRANSFER: Option<Stream5<DMA1>> = None;

        let transfer = TRANSFER.get_or_insert_with(|| {
            cortex_m::interrupt::free(|cs| G_TRANSFER.borrow(cs).replace(None).unwrap())
        });

        transfer.clear_interrupts();
    }
};


// // use stm32f3xx_hal::dma::dma2::C3;
// // use stm32f3xx_hal::dma::Channel;
// // use stm32f3xx_hal::dma::{self, Direction, Increment, Priority};
// // use stm32f3xx_hal::gpio::{Edge, Gpioa, Input, Pin};
// // use stm32f3xx_hal::pac::{Interrupt, NVIC, USART1};
// // use stm32f3xx_hal::prelude::*;
// // use stm32f3xx_hal::serial::{self, Serial};
// // use stm32f3xx_hal::timer::Timer;
// // use typenum::UTerm;

// use achordion_lib::midi::controller::Controller as MidiController;
// use achordion_lib::oscillator::Oscillator;
// use achordion_lib::waveform;
// use achordion_lib::wavetable::Wavetable;

// // use crate::hal::prelude::*;
// use crate::cs43l22::{Cs43L22, Register};

// // Audio timing configuration:
// // Sample rate 48 kHz
// // 16 bits per sample -> SCK rate 1.536 MHz
// // MCK frequency = 256 * sample rate -> MCK rate 12.228 MHz (also equal to 8 * SCK rate)
// const SAMPLE_RATE: u32 = 48_000;

// const DMA_LENGTH: usize = 64;
// static mut DMA_BUFFER: [u32; DMA_LENGTH] = [0; DMA_LENGTH];

// lazy_static! {
//     static ref WAVETABLE: Wavetable<'static> =
//         Wavetable::new(&waveform::saw::SAW_FACTORS, SAMPLE_RATE);
// }

// #[app(device = stm32f4xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
// const APP: () = {
//     struct Resources {
//         // dsp_dma: C3,
//         // midi_rx: serial::Rx<USART1>,
//         // midi_controller: MidiController,
//         // oscillator: Oscillator<'static>,
//         button: PA0<Input<PullDown>>,
//         status_led: PD12<Output<PushPull>>,
//     }

//     #[init]
//     fn init(mut cx: init::Context) -> init::LateResources {
//         let mut syscfg = cx.device.SYSCFG.constrain();
//         let gpioa = cx.device.GPIOA.split();
//         let gpiob = cx.device.GPIOB.split();
//         let gpioc = cx.device.GPIOC.split();
//         let gpiod = cx.device.GPIOD.split();
//         let rcc = cx.device.RCC.constrain();

//         // The 86 MHz frequency can be divided to get a sample rate very close to 48 kHz.
//         let clocks = rcc.cfgr.use_hse(8.mhz()).i2s_clk(86.mhz()).freeze();

//         let mut delay = Delay::new(cx.core.SYST, clocks);

//         let status_led = gpiod.pd12.into_push_pull_output();

//         let button = {
//             let mut button = gpioa.pa0.into_pull_down_input();
//             button.make_interrupt_source(&mut syscfg);
//             button.enable_interrupt(&mut cx.device.EXTI);
//             button.trigger_on_edge(&mut cx.device.EXTI, Edge::RISING);

//             pac::NVIC::unpend(pac::Interrupt::EXTI0);
//             unsafe {
//                 pac::NVIC::unmask(pac::Interrupt::EXTI0);
//             };

//             button
//         };

//         let mut dac = {
//             let i2c = I2c::i2c1(
//                 cx.device.I2C1,
//                 (
//                     gpiob.pb6.into_alternate_af4_open_drain(),
//                     gpiob.pb9.into_alternate_af4_open_drain(),
//                 ),
//                 100.khz(),
//                 clocks,
//             );

//             // Shift the address to deal with different ways of representing I2C addresses
//             let dac = Cs43L22::new(i2c, 0x94 >> 1);

//             dac
//         };

//         let mut dac_reset = gpiod.pd4.into_push_pull_output();

//         // I2S pins: (WS, CK, MCLK, SD) for I2S3
//         let i2s_pins = (
//             gpioa.pa4.into_alternate_af6(),
//             gpioc.pc10.into_alternate_af6(),
//             gpioc.pc7.into_alternate_af6(),
//             gpioc.pc12.into_alternate_af6(),
//         );
//         let hal_i2s = I2s::i2s3(cx.device.SPI3, i2s_pins, clocks);
//         let i2s_clock = hal_i2s.input_clock();

//         let i2s = stm32_i2s_v12x::I2s::new(hal_i2s);
//         let mut i2s = i2s.configure_master_transmit(MasterConfig::with_sample_rate(
//             i2s_clock.0,
//             SAMPLE_RATE,
//             Data16Frame16,
//             FrameFormat::PhilipsI2s,
//             Polarity::IdleHigh,
//             MasterClock::Enable,
//         ));
//         i2s.set_dma_enabled(true);

//         // Keep DAC reset low for at least one millisecond
//         delay.delay_ms(1u8);
//         // Release the DAC from reset
//         dac_reset.set_high().unwrap();
//         // Wait at least 550 ns before starting I2C communication
//         delay.delay_us(1u8);

//         // dac.basic_setup().unwrap();
//         // // Clocking control from the table in section 4.6 of the datasheet:
//         // // Auto mode: disabled
//         // // Speed mode: 01 (single-speed)
//         // // 8 kHz, 16 kHz, or 32 kHz sample rate: no
//         // // 27 MHz video clock: no
//         // // Internal MCLK/LRCLCK ratio: 00
//         // // MCLK divide by 2: no
//         // dac.write(Register::ClockingCtl, 0b0_01_0_0_00_0).unwrap();
//         // // Interface control:
//         // // Slave mode
//         // // SCLK not inverted
//         // // DSP mode disabled
//         // // Interface format I2S
//         // // Word length 16 bits
//         // dac.write(Register::InterfaceCtl1, 0b0_0_0_0_01_11).unwrap();

//         // let mut rcc = cx.device.RCC.constrain();
//         // let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb);
//         // let mut gpioc = cx.device.GPIOC.split(&mut rcc.ahb);
//         // let mut exti = cx.device.EXTI;
//         // let mut syscfg = cx.device.SYSCFG.constrain(&mut rcc.apb2);
//         // let mut flash = cx.device.FLASH.constrain();
//         // let mut dma2 = cx.device.DMA2.split(&mut rcc.ahb);

//         // let clocks = rcc
//         //     .cfgr
//         //     .use_hse(8.MHz())
//         //     .sysclk(72.MHz())
//         //     .freeze(&mut flash.acr);

//         // // Configure DSP
//         // let dsp_dma = {
//         //     // Start periodic timer on TIM2
//         //     {
//         //         let mut tim2 = Timer::tim2(cx.device.TIM2, SAMPLE_RATE.Hz(), clocks, &mut rcc.apb1);
//         //         tim2.reset_on_overflow();
//         //     }

//         //     // Configure DAC, triggered by TIM2 and requesting data from DMA
//         //     {
//         //         let mut dac = cx.device.DAC1.constrain(
//         //             gpioa.pa4.into_analog(&mut gpioa.moder, &mut gpioa.pupdr),
//         //             gpioa.pa5.into_analog(&mut gpioa.moder, &mut gpioa.pupdr),
//         //             &mut rcc.apb1,
//         //             &mut gpioa.moder,
//         //             &mut gpioa.pupdr,
//         //         );
//         //         // disable buffer for better SNR
//         //         dac.disable_buffer();
//         //         dac.set_trigger_tim2();
//         //         dac.enable_dma();
//         //         dac.enable();
//         //     }

//         //     // Configure DMA for transfer between buffer and DAC
//         //     let dsp_dma = {
//         //         let ma = unsafe { DMA_BUFFER.as_ptr() } as usize as u32; // source: memory address
//         //         let pa = 0x40007420; // destination: Dual DAC 12-bit right-aligned data holding register (DHR12RD)
//         //         let ndt = DMA_LENGTH as u16; // number of items to transfer

//         //         dma2.ch3.set_direction(Direction::FromMemory);
//         //         unsafe {
//         //             dma2.ch3.set_memory_address(ma, Increment::Enable);
//         //             dma2.ch3.set_peripheral_address(pa, Increment::Disable);
//         //         }
//         //         dma2.ch3.set_transfer_length(ndt);
//         //         dma2.ch3.set_word_size::<u32>();
//         //         dma2.ch3.set_circular(true);
//         //         dma2.ch3.set_priority_level(Priority::High);
//         //         dma2.ch3.listen(dma::Event::Any);

//         //         unsafe { NVIC::unmask(Interrupt::DMA2_CH3) };

//         //         dma2.ch3.enable();

//         //         dma2.ch3
//         //     };

//         //     dsp_dma
//         // };

//         // // Configure PA0 (blue button) to trigger an interrupt when clicked
//         // let button = {
//         //     let mut button = gpioa
//         //         .pa0
//         //         .into_pull_down_input(&mut gpioa.moder, &mut gpioa.pupdr);
//         //     button.make_interrupt_source(&mut syscfg);
//         //     button.trigger_on_edge(&mut exti, Edge::Rising);
//         //     button.enable_interrupt(&mut exti);

//         //     let interrupt_num = button.nvic();
//         //     unsafe { NVIC::unmask(interrupt_num) };

//         //     button
//         // };

//         // // Configure USART for MIDI
//         // let midi_rx = {
//         //     let pins = (
//         //         gpioc
//         //             .pc4
//         //             .into_af7_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrl),
//         //         gpioc
//         //             .pc5
//         //             .into_af7_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrl),
//         //     );

//         //     let mut serial =
//         //         Serial::usart1(cx.device.USART1, pins, 9600.Bd(), clocks, &mut rcc.apb2);
//         //     serial.listen(serial::Event::Rxne);

//         //     unsafe { NVIC::unmask(Interrupt::USART1_EXTI25) };

//         //     let (_tx, rx) = serial.split();
//         //     rx
//         // };

//         init::LateResources {
//         //     button,
//         //     dsp_dma,
//         //     midi_rx,
//         //     midi_controller: MidiController::new(),
//         //     oscillator: Oscillator::new(&WAVETABLE, SAMPLE_RATE),
//             button,
//             status_led,
//         }
//     }

//     // #[task(priority = 2, binds = DMA2_CH3, resources = [dsp_dma, oscillator])]
//     // fn dsp_request(cx: dsp_request::Context) {
//     //     use dma::Event::*;

//     //     let event = {
//     //         let event = if cx.resources.dsp_dma.event_occurred(HalfTransfer) {
//     //             Some(HalfTransfer)
//     //         } else if cx.resources.dsp_dma.event_occurred(TransferComplete) {
//     //             Some(TransferComplete)
//     //         } else {
//     //             None
//     //         };
//     //         cx.resources.dsp_dma.clear_event(Any);
//     //         event
//     //     };

//     //     if let Some(event) = event {
//     //         match event {
//     //             HalfTransfer => audio_callback(
//     //                 unsafe { &mut DMA_BUFFER },
//     //                 DMA_LENGTH / 2,
//     //                 0,
//     //                 cx.resources.oscillator,
//     //             ),
//     //             TransferComplete => audio_callback(
//     //                 unsafe { &mut DMA_BUFFER },
//     //                 DMA_LENGTH / 2,
//     //                 1,
//     //                 cx.resources.oscillator,
//     //             ),
//     //             _ => (),
//     //         }
//     //     }
//     // }

//     // #[task(binds = USART1_EXTI25, resources = [oscillator, midi_rx, midi_controller])]
//     // fn midi_rx(mut cx: midi_rx::Context) {
//     //     while let Ok(x) = cx.resources.midi_rx.read() {
//     //         if let Some(state) = cx.resources.midi_controller.reconcile_byte(x) {
//     //             cx.resources.oscillator.lock(|oscillator| {
//     //                 oscillator.frequency = state.frequency;
//     //             });
//     //         }
//     //     }
//     // }

//     // #[task(binds = EXTI0, resources = [button, oscillator])]
//     #[task(binds = EXTI0, resources = [button, status_led])]
//     fn button_click(mut cx: button_click::Context) {
//         cx.resources.button.clear_interrupt_pending_bit();
//         cx.resources.status_led.toggle();

//         // cx.resources.oscillator.lock(|oscillator| {
//         //     if oscillator.frequency == 0.0 {
//         //         oscillator.frequency = 20.0;
//         //     } else {
//         //         oscillator.frequency *= 1.5;
//         //     }
//         // });
//     }
// };

// fn audio_callback(
//     buffer: &mut [u32; DMA_LENGTH],
//     length: usize,
//     offset: usize,
//     oscillator: &mut Oscillator,
// ) {
//     let mut buffer_osc = [0; DMA_LENGTH / 2];
//     oscillator.populate(&mut buffer_osc);

//     for (i, x) in buffer[offset * length..offset * length + length]
//         .iter_mut()
//         .enumerate()
//     {
//         let value = buffer_osc[i] as u32;
//         *x = (value << 16) + value;
//     }
// }
