use crate::system::hal::adc::{Adc, Enabled};
use crate::system::hal::hal::adc::Channel;
use crate::system::hal::pac::ADC1;
use crate::system::hal::prelude::*;

use achordion_lib::probe::{ProbeDetector, PROBE_SEQUENCE};

use super::control_buffer::ControlBuffer;

pub struct Cv<P> {
    pin: P,
    probe_detector: ProbeDetector<'static>,
    buffer: ControlBuffer<8>,
    input_range: (f32, f32),
}

impl<P: Channel<ADC1, ID = u8>> Cv<P> {
    pub fn new(pin: P, input_range: (f32, f32)) -> Self {
        Self {
            pin,
            probe_detector: ProbeDetector::new(&PROBE_SEQUENCE),
            buffer: ControlBuffer::new(),
            input_range,
        }
    }

    pub fn sample(&mut self, adc: &mut Adc<ADC1, Enabled>) {
        let sample: u32 = adc.read(&mut self.pin).unwrap();
        self.buffer
            .write(transpose_adc(sample as f32, adc.max_sample()));
        self.probe_detector
            .write(is_high(sample, adc.max_sample(), self.input_range));
    }

    pub fn connected(&self) -> bool {
        !self.probe_detector.detected()
    }

    pub fn value(&self) -> f32 {
        self.buffer.read()
    }
}

fn transpose_adc(sample: f32, max_sample: u32) -> f32 {
    (max_sample as f32 - sample) / max_sample as f32
}

fn is_high(sample: u32, max_sample: u32, input_range: (f32, f32)) -> bool {
    // Calculate what does center of probe voltage translates to the range of
    // the input.
    let max_probe = 3.3;
    let center = (input_range.0 - max_probe / 2.0) / (input_range.0 - input_range.1);
    transpose_adc(sample as f32, max_sample) > center
}
