use crate::oscillator::Oscillator;

pub struct Instrument<'a> {
    oscillator: Oscillator<'a>,
}

impl<'a> Instrument<'a> {
    pub fn new(wavetables: &'a [&'a Wavetable], sample_rate: u32) -> Self {
        let oscillator = Oscillator::new(wavetables, sample_rate);
        Self {
            oscillator
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.oscillator.frequency = frequency;
    }

    pub fn populate(&mut self, buffer: &mut [u16]) {
        self.oscillator.populate(buffer);
    }
}
