#[allow(unused_imports)]
use micromath::F32Ext;

use daisy::hal;
use daisy_bsp as daisy;

use hal::adc::{self, Adc, Disabled, Enabled};
use hal::hal::adc::Channel;
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
type PinCv2 = hal::gpio::gpioa::PA6<hal::gpio::Analog>; // PIN 19
type PinCv3 = hal::gpio::gpioc::PC0<hal::gpio::Analog>; // PIN 15
type PinCv4 = hal::gpio::gpioa::PA3<hal::gpio::Analog>; // PIN 16
type PinCv5 = hal::gpio::gpiob::PB1<hal::gpio::Analog>; // PIN 17
type PinCv6 = hal::gpio::gpioa::PA7<hal::gpio::Analog>; // PIN 18
type PinProbe = hal::gpio::gpiob::PB5<hal::gpio::Output<hal::gpio::PushPull>>; // PIN 10

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

    #[allow(dead_code)]
    button: Button<PinButton>,

    pot1: Pot<PinPot1>,
    pot2: Pot<PinPot2>,
    pot3: Pot<PinPot3>,
    pot4: Pot<PinPot4>,

    cv1: Cv<PinCv1>,
    cv2: Cv<PinCv2>,
    cv3: Cv<PinCv3>,
    cv4: Cv<PinCv4>,
    cv5: Cv<PinCv5>,
    cv6: Cv<PinCv6>,

    probe: PinProbe,
    probe_generator: ProbeGenerator<'static>,

    parameters: Parameters,
}

#[derive(Default)]
struct Parameters {
    pub note: f32,
    pub wavetable: f32,
    pub bank: f32,
    pub chord: f32,
    pub detune: f32,
    pub scale_root: f32,
    pub scale_mode: f32,
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
        cv2: PinCv2,
        cv3: PinCv3,
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

            button: Button::new(button),

            pot1: Pot::new(pot1),
            pot2: Pot::new(pot2),
            pot3: Pot::new(pot3),
            pot4: Pot::new(pot4),

            cv1: Cv::new(cv1),
            cv2: Cv::new(cv2),
            cv3: Cv::new(cv3),
            cv4: Cv::new(cv4),
            cv5: Cv::new(cv5),
            cv6: Cv::new(cv6),

            probe,
            probe_generator: ProbeGenerator::new(&PROBE_SEQUENCE),

            parameters: Parameters::default(),
        }
    }

    pub fn note(&self) -> f32 {
        self.parameters.note
    }

    pub fn wavetable(&self) -> f32 {
        self.parameters.wavetable
    }

    pub fn chord(&self) -> f32 {
        self.parameters.chord
    }

    pub fn detune(&self) -> f32 {
        self.parameters.detune
    }

    pub fn root(&self) -> f32 {
        self.parameters.scale_root
    }

    pub fn mode(&self) -> f32 {
        self.parameters.scale_mode
    }

    pub fn wavetable_bank(&self) -> f32 {
        self.parameters.bank
    }

    pub fn foo(&self) -> bool {
        self.button.active()
    }

    pub fn update(&mut self) {
        self.sample();
        self.reconcile();
    }

    fn sample(&mut self) {
        self.pot1.sample(&mut self.adc1);
        self.pot2.sample(&mut self.adc1);
        self.pot3.sample(&mut self.adc1);
        self.pot4.sample(&mut self.adc1);

        self.cv1.sample(&mut self.adc1);
        self.cv2.sample(&mut self.adc1);
        self.cv3.sample(&mut self.adc1);
        self.cv4.sample(&mut self.adc1);
        self.cv5.sample(&mut self.adc1);
        self.cv6.sample(&mut self.adc1);

        if self.probe_generator.read() {
            self.probe.set_high().unwrap();
        } else {
            self.probe.set_low().unwrap();
        }
    }

    fn reconcile(&mut self) {
        self.reconcile_note();
        self.reconcile_wavetable();
        self.reconcile_chord();
        self.reconcile_detune();
        self.reconcile_scale_root();
        self.reconcile_scale_mode();
    }

    fn reconcile_note(&mut self) {
        self.parameters.note = if self.cv1.connected() {
            // Keep the multiplier below 4, so assure that the result won't get
            // into the 5th octave when set on the edge.
            let octave_offset = (self.pot1.value() * 3.95).trunc();
            let note = sample_to_voct(self.cv1.value());
            note + octave_offset
        } else {
            self.pot1.value() * 4.0 + 3.0
        };
    }

    fn reconcile_wavetable(&mut self) {
        self.parameters.wavetable = if self.cv6.connected() {
            // CV is centered around zero, suited for LFO.
            let wavetable = self.cv6.value() * 2.0 - 1.0;
            let offset = self.pot2.value();
            (wavetable + offset).min(0.9999).max(0.0)
        } else {
            self.pot2.value()
        };
    }

    fn reconcile_chord(&mut self) {
        self.parameters.chord = if self.cv4.connected() {
            // CV is centered around zero, suited for LFO.
            let chord = self.cv4.value() * 2.0 - 1.0;
            let offset = self.pot3.value();
            (chord + offset).min(0.9999).max(0.0)
        } else {
            self.pot3.value()
        };
    }

    fn reconcile_detune(&mut self) {
        self.parameters.detune = if self.cv5.connected() {
            // CV is centered around zero, suited for LFO.
            let detune = self.cv5.value() * 2.0 - 1.0;
            let offset = self.pot4.value();
            (detune + offset).min(0.9999).max(0.0)
        } else {
            self.pot4.value()
        };
    }

    fn reconcile_scale_root(&mut self) {
        self.parameters.scale_root = sample_to_voct(self.cv2.value());
    }

    fn reconcile_scale_mode(&mut self) {
        self.parameters.scale_mode = self.cv3.value();
    }
}

fn sample_to_voct(transposed_sample: f32) -> f32 {
    // V/OCT CV spans from -1.5 to 5.5 V.
    transposed_sample * 7.0 + 0.5
}

fn transpose_adc(sample: f32, max_sample: u32) -> f32 {
    (max_sample as f32 - sample) / max_sample as f32
}

fn is_high(sample: u32, max_sample: u32) -> bool {
    transpose_adc(sample as f32, max_sample) > 0.5
}

#[allow(dead_code)]
struct Button<P> {
    pin: P,
}

impl<P: InputPin> Button<P> {
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    #[allow(dead_code)]
    pub fn active(&self) -> bool {
        self.pin.is_low().ok().unwrap()
    }
}

struct Pot<P> {
    pin: P,
    buffer: ControlBuffer<8>,
}

impl<P: Channel<ADC1, ID = u8>> Pot<P> {
    pub fn new(pin: P) -> Self {
        Self {
            pin,
            buffer: ControlBuffer::new(),
        }
    }

    pub fn sample(&mut self, adc: &mut Adc<ADC1, Enabled>) {
        let sample: u32 = adc.read(&mut self.pin).unwrap();
        self.buffer
            .write(transpose_adc(sample as f32, adc.max_sample()));
    }

    pub fn value(&self) -> f32 {
        self.buffer.read()
    }
}

struct Cv<P> {
    pin: P,
    probe_detector: ProbeDetector<'static>,
    buffer: ControlBuffer<8>,
}

impl<P: Channel<ADC1, ID = u8>> Cv<P> {
    pub fn new(pin: P) -> Self {
        Self {
            pin,
            probe_detector: ProbeDetector::new(&PROBE_SEQUENCE),
            buffer: ControlBuffer::new(),
        }
    }

    pub fn sample(&mut self, adc: &mut Adc<ADC1, Enabled>) {
        let sample: u32 = adc.read(&mut self.pin).unwrap();
        self.buffer
            .write(transpose_adc(sample as f32, adc.max_sample()));
        self.probe_detector.write(is_high(sample, adc.max_sample()));
    }

    pub fn connected(&self) -> bool {
        !self.probe_detector.detected()
    }

    pub fn value(&self) -> f32 {
        self.buffer.read()
    }
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

    pub fn write(&mut self, value: f32) {
        self.buffer[self.pointer] = value;
        self.pointer = (self.pointer + 1) % N;
    }

    pub fn read(&self) -> f32 {
        let sum: f32 = self.buffer.iter().sum();
        sum / N as f32
    }

    // TODO: Delta from the oldest to the newest, to detect movement
}
