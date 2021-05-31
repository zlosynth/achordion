#[allow(unused_imports)]
use micromath::F32Ext;

use daisy::hal;
use daisy_bsp as daisy;

use hal::adc::{self, Adc, Disabled, Enabled};
use hal::hal::digital::v2::InputPin;
use hal::pac::ADC1;
use hal::prelude::*;

type PinButton = hal::gpio::gpiob::PB4<hal::gpio::Input<hal::gpio::PullUp>>; // PIN 9
type PinPot1 = hal::gpio::gpioa::PA4<hal::gpio::Analog>; // PIN 23
type PinPot2 = hal::gpio::gpioa::PA1<hal::gpio::Analog>; // PIN 24
type PinPot3 = hal::gpio::gpioa::PA5<hal::gpio::Analog>; // PIN 22
type PinPot4 = hal::gpio::gpioc::PC4<hal::gpio::Analog>; // PIN 21

// type PinCV1 = hal::gpio::gpioc::PC1<hal::gpio::Analog>;
// type PinCV2 = hal::gpio::gpioa::PA6<hal::gpio::Analog>;
// type PinCV3 = hal::gpio::gpioc::PC0<hal::gpio::Analog>;
// type PinCV4 = hal::gpio::gpioa::PA3<hal::gpio::Analog>;
// type PinCV5 = hal::gpio::gpiob::PB1<hal::gpio::Analog>;
// type PinCV6 = hal::gpio::gpioa::PA7<hal::gpio::Analog>;
// type PinLed1 = hal::gpio::gpiob::PB15<hal::gpio::Analog>;
// type PinLed2 = hal::gpio::gpiob::PB14<hal::gpio::Analog>;
// type PinLed3 = hal::gpio::gpiod::PD11<hal::gpio::Analog>;
// type PinLed4 = hal::gpio::gpioa::PA0<hal::gpio::Analog>;
// type PinLed5 = hal::gpio::gpioc::PC9<hal::gpio::Analog>;
// type PinLed6 = hal::gpio::gpioc::PC8<hal::gpio::Analog>;
// type PinLed7 = hal::gpio::gpiod::PD2<hal::gpio::Analog>;
// type PinLed8 = hal::gpio::gpioc::PC12<hal::gpio::Analog>;
// type PinProbe = hal::gpio::gpiob::PB5<hal::gpio::Analog>;

pub struct Interface {
    adc1: Adc<ADC1, Enabled>,

    button: PinButton,

    pot1: PinPot1,
    pot2: PinPot2,
    pot3: PinPot3,
    pot4: PinPot4,

    button_clicked: bool,

    note_pot_buffer: ControlBuffer<2>,
    wavetable_pot_buffer: ControlBuffer<2>,
    wavetable_bank_pot_buffer: ControlBuffer<2>,
    chord_pot_buffer: ControlBuffer<2>,
    detune_pot_buffer: ControlBuffer<2>,
}

impl Interface {
    pub fn new(
        mut adc1: Adc<ADC1, Disabled>,
        button: PinButton,
        pot1: PinPot1,
        pot2: PinPot2,
        pot3: PinPot3,
        pot4: PinPot4,
    ) -> Self {
        adc1.set_resolution(adc::Resolution::SIXTEENBIT);
        adc1.set_sample_time(adc::AdcSampleTime::T_64);
        let adc1 = adc1.enable();

        Self {
            adc1,

            button,

            pot1,
            pot2,
            pot3,
            pot4,

            button_clicked: false,

            note_pot_buffer: ControlBuffer::new(),
            wavetable_pot_buffer: ControlBuffer::new(),
            wavetable_bank_pot_buffer: ControlBuffer::new(),
            chord_pot_buffer: ControlBuffer::new(),
            detune_pot_buffer: ControlBuffer::new(),
        }
    }

    pub fn note(&self) -> f32 {
        transpose_adc(self.note_pot_buffer.read(), self.adc1.max_sample()) * 4.0 + 1.0
    }

    pub fn wavetable(&self) -> f32 {
        transpose_adc(self.wavetable_pot_buffer.read(), self.adc1.max_sample())
    }

    pub fn wavetable_bank(&self) -> f32 {
        transpose_adc(
            self.wavetable_bank_pot_buffer.read(),
            self.adc1.max_sample(),
        )
    }

    pub fn chord(&self) -> f32 {
        transpose_adc(self.chord_pot_buffer.read(), self.adc1.max_sample())
    }

    pub fn detune(&self) -> f32 {
        transpose_adc(self.detune_pot_buffer.read(), self.adc1.max_sample())
    }

    pub fn sample(&mut self) {
        self.button_clicked = self.button.is_high().unwrap();

        let pot1_sample = self.adc1.read(&mut self.pot1).unwrap();
        self.note_pot_buffer.write(pot1_sample);
        let pot2_sample = self.adc1.read(&mut self.pot2).unwrap();
        self.wavetable_pot_buffer.write(pot2_sample);
        let pot3_sample = self.adc1.read(&mut self.pot3).unwrap();
        self.chord_pot_buffer.write(pot3_sample);
        let pot4_sample = self.adc1.read(&mut self.pot4).unwrap();
        self.detune_pot_buffer.write(pot4_sample);
    }
}

fn transpose_adc(sample: f32, max_sample: u32) -> f32 {
    (max_sample as f32 - sample) / max_sample as f32
}

struct ControlBuffer<const N: usize> {
    buffer: [f32; N],
    pointer: usize,
}

impl<const N: usize> ControlBuffer<N> {
    pub fn new() -> Self {
        Self {
            buffer: [0.0; N],
            pointer: 0,
        }
    }

    pub fn write(&mut self, value: u32) {
        self.buffer[self.pointer] = value as f32;
        self.pointer = (self.pointer + 1) % N;
    }

    pub fn read(&self) -> f32 {
        let sum: f32 = self.buffer.iter().sum();
        sum / N as f32
    }

    // TODO: Delta from the oldest to the newest, to detect movement
    // TODO: Detect probe
}
