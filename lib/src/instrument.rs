use crate::note::Note;
use crate::oscillator::Oscillator;
use crate::quantizer;
use crate::scales;
use crate::wavetable::Wavetable;

pub struct Instrument<'a> {
    voct: f32,
    mode: scales::diatonic::Mode,
    root: Note,
    oscillator: Oscillator<'a>,
}

impl<'a> Instrument<'a> {
    pub fn new(wavetables: &'a [&'a Wavetable], sample_rate: u32) -> Self {
        let oscillator = Oscillator::new(wavetables, sample_rate);
        Self {
            oscillator,
            voct: 0.0,
            root: Note::C1,
            mode: scales::diatonic::Ionian,
        }
    }

    pub fn set_mode(&mut self, mode: f32) {
        self.mode = if mode < 1.0 / 7.0 {
            scales::diatonic::Ionian
        } else if mode < 2.0 / 7.0 {
            scales::diatonic::Dorian
        } else if mode < 3.0 / 7.0 {
            scales::diatonic::Phrygian
        } else if mode < 4.0 / 7.0 {
            scales::diatonic::Lydian
        } else if mode < 5.0 / 7.0 {
            scales::diatonic::Mixolydian
        } else if mode < 6.0 / 7.0 {
            scales::diatonic::Aeolian
        } else {
            scales::diatonic::Locrian
        };
        self.apply_settings();
    }

    pub fn set_root(&mut self, root: f32) {
        self.root = quantizer::chromatic::quantize(root);
        self.apply_settings();
    }

    pub fn set_voct(&mut self, voct: f32) {
        self.voct = voct;
        self.apply_settings();
    }

    pub fn set_wavetable(&mut self, wavetable: f32) {
        self.oscillator.wavetable = wavetable;
    }

    pub fn populate(&mut self, buffer: &mut [u16]) {
        self.oscillator.populate(buffer);
    }

    fn apply_settings(&mut self) {
        let note = quantizer::diatonic::quantize(self.mode, self.root, self.voct);
        self.oscillator.frequency = note.to_freq_f32();
    }
}
