#[allow(unused_imports)]
use micromath::F32Ext;

use daisy::hal;
use daisy_bsp as daisy;

use hal::adc::{self, Adc, Disabled, Enabled};
use hal::hal::digital::v2::{InputPin, OutputPin};
use hal::pac::ADC1;
use hal::prelude::*;

use achordion_lib::probe::{ProbeDetector, ProbeGenerator, PROBE_SEQUENCE};

type PinButton = hal::gpio::gpiob::PB4<hal::gpio::Input<hal::gpio::PullUp>>; // PIN 9
type PinPot1 = hal::gpio::gpioa::PA4<hal::gpio::Analog>; // PIN 23
type PinPot2 = hal::gpio::gpioa::PA1<hal::gpio::Analog>; // PIN 24
type PinPot3 = hal::gpio::gpioa::PA5<hal::gpio::Analog>; // PIN 22
type PinPot4 = hal::gpio::gpioc::PC4<hal::gpio::Analog>; // PIN 21
type PinCv1 = hal::gpio::gpioc::PC1<hal::gpio::Analog>; // PIN 20
type PinCv4 = hal::gpio::gpioa::PA3<hal::gpio::Analog>; // PIN 16
type PinCv5 = hal::gpio::gpiob::PB1<hal::gpio::Analog>; // PIN 17
type PinCv6 = hal::gpio::gpioa::PA7<hal::gpio::Analog>; // PIN 18
type PinProbe = hal::gpio::gpiob::PB5<hal::gpio::Output<hal::gpio::PushPull>>; // PIN 10

// type PinCV2 = hal::gpio::gpioa::PA6<hal::gpio::Analog>;
// type PinCV3 = hal::gpio::gpioc::PC0<hal::gpio::Analog>;
// type PinLed1 = hal::gpio::gpiob::PB15<hal::gpio::Analog>;
// type PinLed2 = hal::gpio::gpiob::PB14<hal::gpio::Analog>;
// type PinLed3 = hal::gpio::gpiod::PD11<hal::gpio::Analog>;
// type PinLed4 = hal::gpio::gpioa::PA0<hal::gpio::Analog>;
// type PinLed5 = hal::gpio::gpioc::PC9<hal::gpio::Analog>;
// type PinLed6 = hal::gpio::gpioc::PC8<hal::gpio::Analog>;
// type PinLed7 = hal::gpio::gpiod::PD2<hal::gpio::Analog>;
// type PinLed8 = hal::gpio::gpioc::PC12<hal::gpio::Analog>;

pub struct Interface {
    adc1: Adc<ADC1, Enabled>,

    button: PinButton,

    pot1: PinPot1,
    pot2: PinPot2,
    pot3: PinPot3,
    pot4: PinPot4,

    cv1: PinCv1,
    cv1_probe_detector: ProbeDetector<'static>,
    cv4: PinCv4,
    cv4_probe_detector: ProbeDetector<'static>,
    cv5: PinCv5,
    cv5_probe_detector: ProbeDetector<'static>,
    cv6: PinCv6,
    cv6_probe_detector: ProbeDetector<'static>,

    probe: PinProbe,
    probe_generator: ProbeGenerator<'static>,

    button_clicked: bool,

    note_pot_buffer: ControlBuffer<8>,
    wavetable_pot_buffer: ControlBuffer<8>,
    wavetable_bank_pot_buffer: ControlBuffer<8>,
    chord_pot_buffer: ControlBuffer<8>,
    detune_pot_buffer: ControlBuffer<8>,

    voct_cv_buffer: ControlBuffer<8>,
    wavetable_cv_buffer: ControlBuffer<8>,
    chord_cv_buffer: ControlBuffer<8>,
    detune_cv_buffer: ControlBuffer<8>,
}

impl Interface {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        mut adc1: Adc<ADC1, Disabled>,
        button: PinButton,
        pot1: PinPot1,
        pot2: PinPot2,
        pot3: PinPot3,
        pot4: PinPot4,
        cv1: PinCv1,
        cv4: PinCv4,
        cv5: PinCv5,
        cv6: PinCv6,
        probe: PinProbe,
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

            cv1,
            cv1_probe_detector: ProbeDetector::new(&PROBE_SEQUENCE),
            cv4,
            cv4_probe_detector: ProbeDetector::new(&PROBE_SEQUENCE),
            cv5,
            cv5_probe_detector: ProbeDetector::new(&PROBE_SEQUENCE),
            cv6,
            cv6_probe_detector: ProbeDetector::new(&PROBE_SEQUENCE),

            probe,
            probe_generator: ProbeGenerator::new(&PROBE_SEQUENCE),

            button_clicked: false,

            note_pot_buffer: ControlBuffer::new(),
            wavetable_pot_buffer: ControlBuffer::new(),
            wavetable_bank_pot_buffer: ControlBuffer::new(),
            chord_pot_buffer: ControlBuffer::new(),
            detune_pot_buffer: ControlBuffer::new(),

            voct_cv_buffer: ControlBuffer::new(),
            wavetable_cv_buffer: ControlBuffer::new(),
            chord_cv_buffer: ControlBuffer::new(),
            detune_cv_buffer: ControlBuffer::new(),
        }
    }

    pub fn note(&self) -> f32 {
        if self.cv1_probe_detector.detected() {
            transpose_adc(self.note_pot_buffer.read(), self.adc1.max_sample()) * 4.0
        } else {
            // Keep the multiplier below 4, so assure that the result won't get
            // into the 5th octave when set on the edge.
            let octave =
                (transpose_adc(self.note_pot_buffer.read(), self.adc1.max_sample()) * 3.95).trunc();
            transpose_adc(self.voct_cv_buffer.read(), self.adc1.max_sample()) * 4.0 + octave
        }
    }

    pub fn wavetable(&self) -> f32 {
        if self.cv6_probe_detector.detected() {
            transpose_adc(self.wavetable_pot_buffer.read(), self.adc1.max_sample())
        } else {
            // CV is centered around zero, suited for LFO.
            let cv =
                transpose_adc(self.wavetable_cv_buffer.read(), self.adc1.max_sample()) * 2.0 - 1.0;
            let pot = transpose_adc(self.wavetable_pot_buffer.read(), self.adc1.max_sample());
            (cv + pot).min(0.9999).max(0.0)
        }
    }

    pub fn wavetable_bank(&self) -> f32 {
        transpose_adc(
            self.wavetable_bank_pot_buffer.read(),
            self.adc1.max_sample(),
        )
    }

    pub fn chord(&self) -> f32 {
        if self.cv4_probe_detector.detected() {
            transpose_adc(self.chord_pot_buffer.read(), self.adc1.max_sample())
        } else {
            // CV is centered around zero, suited for LFO.
            let cv = transpose_adc(self.chord_cv_buffer.read(), self.adc1.max_sample()) * 2.0 - 1.0;
            let pot = transpose_adc(self.chord_pot_buffer.read(), self.adc1.max_sample());
            (cv + pot).min(0.9999).max(0.0)
        }
    }

    pub fn detune(&self) -> f32 {
        if self.cv5_probe_detector.detected() {
            transpose_adc(self.detune_pot_buffer.read(), self.adc1.max_sample())
        } else {
            // CV is centered around zero, suited for LFO.
            let cv =
                transpose_adc(self.detune_cv_buffer.read(), self.adc1.max_sample()) * 2.0 - 1.0;
            let pot = transpose_adc(self.detune_pot_buffer.read(), self.adc1.max_sample());
            (cv + pot).min(0.9999).max(0.0)
        }
    }

    pub fn foo(&self) -> bool {
        !self.cv6_probe_detector.detected()
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

        let cv1_sample: u32 = self.adc1.read(&mut self.cv1).unwrap();
        self.voct_cv_buffer.write(cv1_sample);
        self.cv1_probe_detector
            .write(is_high(cv1_sample, self.adc1.max_sample()));

        let cv4_sample: u32 = self.adc1.read(&mut self.cv4).unwrap();
        self.chord_cv_buffer.write(cv4_sample);
        self.cv4_probe_detector
            .write(is_high(cv4_sample, self.adc1.max_sample()));

        let cv5_sample: u32 = self.adc1.read(&mut self.cv5).unwrap();
        self.detune_cv_buffer.write(cv5_sample);
        self.cv5_probe_detector
            .write(is_high(cv5_sample, self.adc1.max_sample()));

        let cv6_sample: u32 = self.adc1.read(&mut self.cv6).unwrap();
        self.wavetable_cv_buffer.write(cv6_sample);
        self.cv6_probe_detector
            .write(is_high(cv6_sample, self.adc1.max_sample()));

        if self.probe_generator.read() {
            self.probe.set_high().unwrap();
        } else {
            self.probe.set_low().unwrap();
        }
    }
}

fn transpose_adc(sample: f32, max_sample: u32) -> f32 {
    (max_sample as f32 - sample) / max_sample as f32
}

fn is_high(sample: u32, max_sample: u32) -> bool {
    transpose_adc(sample as f32, max_sample) > 0.5
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
}