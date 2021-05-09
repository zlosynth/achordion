use crate::oscillator::Oscillator;
use crate::quantizer;
use crate::scales;
use crate::tone::ToFrequency;
use crate::wavetable::Wavetable;

pub struct Instrument<'a> {
    oscillator: Oscillator<'a>,
}

impl<'a> Instrument<'a> {
    pub fn new(wavetables: &'a [&'a Wavetable], sample_rate: u32) -> Self {
        let oscillator = Oscillator::new(wavetables, sample_rate);
        Self { oscillator }
    }

    pub fn set_frequency<T>(&mut self, tone: T)
    where
        T: ToFrequency,
    {
        self.oscillator.frequency = tone.to_frequency();
    }

    pub fn populate(&mut self, buffer: &mut [u16]) {
        self.oscillator.populate(buffer);
    }
}
