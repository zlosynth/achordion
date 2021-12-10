use core::marker::PhantomData;

use nb::block;

use crate::system::hal::adc::{Adc, Enabled};
use crate::system::hal::hal::adc::Channel;
use crate::system::hal::pac::{ADC1, ADC2};

use super::control_buffer::ControlBuffer;

pub struct Pot<A, P> {
    pin: P,
    position_filter: ControlBuffer<8>,
    movement_detector: ControlBuffer<124>,
    _adc: PhantomData<A>,
}

macro_rules! pot {
    ($adc:ident) => {
        impl<P: Channel<$adc, ID = u8>> Pot<$adc, P> {
            pub fn new(pin: P) -> Self {
                Self {
                    pin,
                    position_filter: ControlBuffer::new(),
                    movement_detector: ControlBuffer::new(),
                    _adc: PhantomData,
                }
            }

            pub fn start_sampling(&mut self, adc: &mut Adc<$adc, Enabled>) {
                adc.start_conversion(&mut self.pin);
            }

            pub fn finish_sampling(&mut self, adc: &mut Adc<$adc, Enabled>) {
                let sample: u32 = block!(adc.read_sample()).unwrap();
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
pot!(ADC2);

fn transpose_adc(sample: f32, max_sample: u32) -> f32 {
    sample / max_sample as f32
}
