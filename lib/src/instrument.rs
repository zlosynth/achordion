use core::ptr;

use crate::chords;
use crate::note::Note;
use crate::oscillator::Oscillator;
use crate::quantizer;
use crate::scales;
use crate::wavetable::Wavetable;

const CHORDS: [[i8; 3]; 9] = [
    [1, 3, 5],
    [1, 2, 5],
    [1, 4, 5],
    [1, 5, 7],
    [1, 3, 7],
    [1, 4, 7],
    [1, 2, 7],
    [1, 5, 9],
    [1, 2, 9],
];

pub struct Instrument<'a> {
    scale_root: Note,
    scale_mode: scales::diatonic::Mode,
    chord_root: f32,
    chord_degrees: [i8; 3],
    oscillator_a: Oscillator<'a>,
    oscillator_b: Oscillator<'a>,
    oscillator_c: Oscillator<'a>,
}

impl<'a> Instrument<'a> {
    pub fn new(wavetables: &'a [&'a Wavetable], sample_rate: u32) -> Self {
        let oscillator_a = Oscillator::new(wavetables, sample_rate);
        let oscillator_b = Oscillator::new(wavetables, sample_rate);
        let oscillator_c = Oscillator::new(wavetables, sample_rate);
        Self {
            scale_root: Note::C1,
            scale_mode: scales::diatonic::Ionian,
            chord_root: 0.0,
            chord_degrees: CHORDS[0],
            oscillator_a,
            oscillator_b,
            oscillator_c,
        }
    }

    pub fn set_scale_mode(&mut self, scale_mode: f32) {
        self.scale_mode = if scale_mode < 1.0 / 7.0 {
            scales::diatonic::Ionian
        } else if scale_mode < 2.0 / 7.0 {
            scales::diatonic::Dorian
        } else if scale_mode < 3.0 / 7.0 {
            scales::diatonic::Phrygian
        } else if scale_mode < 4.0 / 7.0 {
            scales::diatonic::Lydian
        } else if scale_mode < 5.0 / 7.0 {
            scales::diatonic::Mixolydian
        } else if scale_mode < 6.0 / 7.0 {
            scales::diatonic::Aeolian
        } else {
            scales::diatonic::Locrian
        };
        self.apply_settings();
    }

    pub fn set_scale_root(&mut self, scale_root: f32) {
        self.scale_root = quantizer::chromatic::quantize(scale_root);
        self.apply_settings();
    }

    pub fn set_chord_root(&mut self, chord_root: f32) {
        self.chord_root = chord_root;
        self.apply_settings();
    }

    pub fn set_chord_degrees(&mut self, chord_degrees: f32) {
        for i in 1..=CHORDS.len() {
            if chord_degrees < i as f32 / CHORDS.len() as f32 {
                self.chord_degrees = CHORDS[i - 1];
                break;
            }
        }
        self.apply_settings();
    }

    pub fn set_wavetable(&mut self, wavetable: f32) {
        self.oscillator_a.wavetable = wavetable;
        self.oscillator_b.wavetable = wavetable;
        self.oscillator_c.wavetable = wavetable;
    }

    pub fn populate(&mut self, buffer: &mut [u16]) {
        zero_slice(buffer);
        self.oscillator_a.populate_add(buffer, 1.0 / 3.0);
        self.oscillator_b.populate_add(buffer, 1.0 / 3.0);
        self.oscillator_c.populate_add(buffer, 1.0 / 3.0);
    }

    fn apply_settings(&mut self) {
        let chord_root_note =
            quantizer::diatonic::quantize(self.scale_mode, self.scale_root, self.chord_root);
        let chord_notes = chords::diatonic::build(
            self.scale_mode,
            self.scale_root,
            chord_root_note,
            self.chord_degrees,
        );
        self.oscillator_a.frequency = chord_notes[0].unwrap().to_freq_f32();
        self.oscillator_b.frequency = chord_notes[1].unwrap().to_freq_f32();
        self.oscillator_c.frequency = chord_notes[2].unwrap().to_freq_f32();
    }
}

fn zero_slice(slice: &mut [u16]) {
    unsafe {
        let p = slice.as_mut_ptr();
        ptr::write_bytes(p, 0, slice.len());
    }
}
