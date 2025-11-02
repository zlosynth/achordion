use core::marker::PhantomData;

#[allow(unused_imports)]
use micromath::F32Ext;

use nb::block;

use crate::system::hal::adc::{Adc, Enabled};
use crate::system::hal::hal::adc::Channel;
use crate::system::hal::pac::{ADC1, ADC2};

use super::control_buffer::ControlBuffer;

const BASE_TOLERATION: f32 = 0.005;

pub struct Pot<A, P> {
    pin: P,
    input_range: (f32, f32),
    toleration: f32,
    // XXX: ControlBuffer sizes must be power of 2.
    position_filter: ControlBuffer<8>,
    movement_detector: ControlBuffer<124>,
    _adc: PhantomData<A>,
}

macro_rules! pot {
    ($adc:ident) => {
        impl<P: Channel<$adc, ID = u8>> Pot<$adc, P> {
            pub fn new(pin: P, input_range: (f32, f32)) -> Self {
                let toleration = BASE_TOLERATION * (input_range.0 - input_range.1).abs();
                Self {
                    pin,
                    input_range,
                    toleration,
                    position_filter: ControlBuffer::new(),
                    movement_detector: ControlBuffer::new(),
                    _adc: PhantomData,
                }
            }

            // NOTE: Prevent inlining to avoid duplicating ADC HAL code for each pot.
            // This method is called 8 times (4 pots × 2 ADCs), and inlining would
            // duplicate ~2KB of register configuration code per call site (~4KB total).
            #[inline(never)]
            pub fn start_sampling(&mut self, adc: &mut Adc<$adc, Enabled>) {
                adc.start_conversion(&mut self.pin);
            }

            // NOTE: Prevent inlining to avoid duplicating ADC HAL code for each pot.
            // This method is called 8 times (4 pots × 2 ADCs), and inlining would
            // duplicate ~2KB of register configuration code per call site (~4KB total).
            #[inline(never)]
            pub fn finish_sampling(&mut self, adc: &mut Adc<$adc, Enabled>) {
                let sample: u32 = block!(adc.read_sample()).unwrap();
                let transposed_sample = self.transpose_adc(sample, adc.slope());
                self.position_filter.write(transposed_sample);
                self.movement_detector.write(transposed_sample);
            }

            pub fn value(&self) -> f32 {
                self.position_filter.read()
            }

            pub fn active(&self) -> bool {
                self.movement_detector.traveled() > self.toleration
            }

            fn transpose_adc(&self, sample: u32, slope: u32) -> f32 {
                let sample = sample as f32;
                let slope = slope as f32;

                let position_in_full_range = sample / slope;

                let position_in_restricted_range = if self.input_range.0 < self.input_range.1 {
                    let offset_position = position_in_full_range - self.input_range.0;
                    let scaled_up_position =
                        offset_position / (self.input_range.1 - self.input_range.0);
                    scaled_up_position
                } else {
                    let offset_position = position_in_full_range - self.input_range.1;
                    let delta = self.input_range.0 - self.input_range.1;
                    let inverted_position = delta - offset_position;
                    let scaled_up_position = inverted_position / delta;
                    scaled_up_position
                };

                position_in_restricted_range
            }
        }
    };
}

pot!(ADC1);
pot!(ADC2);
