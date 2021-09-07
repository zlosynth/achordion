use crate::system::hal::adc::{Adc, Enabled};
use crate::system::hal::hal::adc::Channel;
use crate::system::hal::pac::ADC1;
use crate::system::hal::prelude::*;

use super::control_buffer::ControlBuffer;

pub struct Pot<P> {
    pin: P,
    position_filter: ControlBuffer<8>,
    movement_detector: ControlBuffer<124>,
}

macro_rules! pot {
    ($adc:ident) => {
        impl<P: Channel<$adc, ID = u8>> Pot<P> {
            pub fn new(pin: P) -> Self {
                Self {
                    pin,
                    position_filter: ControlBuffer::new(),
                    movement_detector: ControlBuffer::new(),
                }
            }

            pub fn sample(&mut self, adc: &mut Adc<$adc, Enabled>) {
                let sample: u32 = adc.read(&mut self.pin).unwrap();
                let transposed_sample = transpose_adc(sample as f32, adc.max_sample());
                self.position_filter.write(transposed_sample);
                self.movement_detector.write(transposed_sample);
            }

            pub fn value(&self) -> f32 {
                self.position_filter.read()
            }

            pub fn active(&self) -> bool {
                self.movement_detector.traveled() > 0.005
            }
        }
    };
}

pot!(ADC1);

fn transpose_adc(sample: f32, max_sample: u32) -> f32 {
    (max_sample as f32 - sample) / max_sample as f32
}
