use crate::note::Note;
use crate::oscillator::Oscillator;
use crate::quantizer;
use crate::scales;
use crate::wavetable::Wavetable;

pub struct Instrument<'a> {
    oscillator: Oscillator<'a>,
}

impl<'a> Instrument<'a> {
    pub fn new(wavetables: &'a [&'a Wavetable], sample_rate: u32) -> Self {
        let oscillator = Oscillator::new(wavetables, sample_rate);
        Self { oscillator }
    }

    pub fn set_voct(&mut self, voct: f32) {
        let note = quantizer::diatonic::quantize(scales::diatonic::Ionian, Note::C1, voct);
        self.oscillator.frequency = note.to_freq_f32();
    }

    pub fn populate(&mut self, buffer: &mut [u16]) {
        self.oscillator.populate(buffer);
    }
}
