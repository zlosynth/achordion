use core::marker::PhantomData;

use crate::system::hal::adc::{Adc, Enabled};
use crate::system::hal::hal::adc::Channel;
use crate::system::hal::pac::{ADC1, ADC2};
use crate::system::hal::prelude::*;

use achordion_lib::probe::{ProbeDetector, PROBE_SEQUENCE};

pub struct Cv<A, P> {
    pin: P,
    probe_detector: ProbeDetector<'static>,
    value: f32,
    input_range: (f32, f32),
    _adc: PhantomData<A>,
}

macro_rules! cv {
    ($adc:ident) => {
        impl<P: Channel<$adc, ID = u8>> Cv<$adc, P> {
            pub fn new(pin: P, input_range: (f32, f32)) -> Self {
                Self {
                    pin,
                    probe_detector: ProbeDetector::new(&PROBE_SEQUENCE),
                    value: 0.0,
                    input_range,
                    _adc: PhantomData,
                }
            }

            pub fn sample(&mut self, adc: &mut Adc<$adc, Enabled>) {
                let sample: u32 = adc.read(&mut self.pin).unwrap();
                self.value = transpose_adc(sample as f32, adc.max_sample());
                self.probe_detector
                    .write(is_high(sample, adc.max_sample(), self.input_range));
            }

            pub fn connected(&self) -> bool {
                !self.probe_detector.detected()
            }

            pub fn value(&self) -> f32 {
                self.value
            }
        }
    };
}

cv!(ADC1);
cv!(ADC2);

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
