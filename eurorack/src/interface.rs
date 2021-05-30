#[allow(unused_imports)]
use micromath::F32Ext;

use hal::adc::{self, Adc, Disabled, Enabled};
use hal::pac::ADC1;
use hal::prelude::*;
use stm32h7xx_hal as hal;

use crate::bsp::pins::*;

pub struct Interface {
    adc1: Adc<ADC1, Enabled>,

    pot1: PinPot1,
    pot2: PinPot2,
    pot3: PinPot3,
    pot4: PinPot4,

    note_pot_buffer: ControlBuffer<2>,
    wavetable_pot_buffer: ControlBuffer<2>,
    wavetable_bank_pot_buffer: ControlBuffer<2>,
    chord_pot_buffer: ControlBuffer<2>,
    detune_pot_buffer: ControlBuffer<2>,
}

impl Interface {
    pub fn new(
        mut adc1: Adc<ADC1, Disabled>,
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

            pot1,
            pot2,
            pot3,
            pot4,

            note_pot_buffer: ControlBuffer::new(),
            wavetable_pot_buffer: ControlBuffer::new(),
            wavetable_bank_pot_buffer: ControlBuffer::new(),
            chord_pot_buffer: ControlBuffer::new(),
            detune_pot_buffer: ControlBuffer::new(),
        }
    }

    // TODO: Helper converting this
    pub fn note(&self) -> f32 {
        ((self.adc1.max_sample() as f32- self.note_pot_buffer.read()) / self.adc1.max_sample() as f32) * 4.0 + 1.0
    }

    pub fn wavetable(&self) -> f32 {
        (self.adc1.max_sample() as f32- self.wavetable_pot_buffer.read()) / self.adc1.max_sample() as f32
    }

    pub fn wavetable_bank(&self) -> f32 {
        (self.adc1.max_sample() as f32- self.wavetable_bank_pot_buffer.read()) / self.adc1.max_sample() as f32
    }

    pub fn chord(&self) -> f32 {
        (self.adc1.max_sample() as f32- self.chord_pot_buffer.read()) / self.adc1.max_sample() as f32
    }

    pub fn detune(&self) -> f32 {
        (self.adc1.max_sample() as f32 -self.detune_pot_buffer.read()) / self.adc1.max_sample() as f32
    }

    pub fn sample(&mut self) {
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
