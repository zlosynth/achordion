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

const DETUNES: [[DetuneConfig; 3]; 4] = [
    [
        DetuneConfig::Disabled,
        DetuneConfig::Disabled,
        DetuneConfig::Disabled,
    ],
    [
        DetuneConfig::BothSides(1.0, 1.01),
        DetuneConfig::BothSides(1.0, 1.01),
        DetuneConfig::BothSides(1.0, 1.01),
    ],
    [
        DetuneConfig::SingleSide(0.5, 0.5 + 0.01),
        DetuneConfig::Disabled,
        DetuneConfig::Disabled,
    ],
    [
        DetuneConfig::SingleSide(0.5, 0.5 + 0.01),
        DetuneConfig::SingleSide(0.5, 0.5 + 0.01),
        DetuneConfig::SingleSide(0.5, 0.5 + 0.01),
    ],
];

const DEGREES_IN_INSTRUMENT: usize = 3;

pub struct Instrument<'a> {
    scale_root: Note,
    scale_mode: scales::diatonic::Mode,
    chord_root: f32,
    chord_degrees: [i8; 3],
    degrees: [Degree<'a>; DEGREES_IN_INSTRUMENT],
}

impl<'a> Instrument<'a> {
    pub fn new(wavetables: &'a [&'a Wavetable], sample_rate: u32) -> Self {
        Self {
            scale_root: Note::C1,
            scale_mode: scales::diatonic::Ionian,
            chord_root: 0.0,
            chord_degrees: CHORDS[0],
            degrees: [
                Degree::new(wavetables, sample_rate),
                Degree::new(wavetables, sample_rate),
                Degree::new(wavetables, sample_rate),
            ],
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
        self.degrees
            .iter_mut()
            .for_each(|d| d.set_wavetable(wavetable));
    }

    pub fn set_detune(&mut self, detune: f32) {
        let index = ((detune * DETUNES.len() as f32) as usize).min(DETUNES.len() - 1);

        let section = 1.0 / DETUNES.len() as f32;
        let phase = (detune % section) / section;

        for (i, degree) in self.degrees.iter_mut().enumerate() {
            degree.set_detune(DETUNES[index][i], phase)
        }
    }

    pub fn populate(&mut self, buffer_root: &mut [u16], buffer_chord: &mut [u16]) {
        zero_slice(buffer_root);
        zero_slice(buffer_chord);

        self.degrees[0].populate_add(buffer_root, 1.0);
        self.degrees[1..]
            .iter_mut()
            .for_each(|d| d.populate_add(buffer_chord, 1.0 / (DEGREES_IN_INSTRUMENT as f32 - 1.0)))
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

        for (i, degree) in self.degrees.iter_mut().enumerate() {
            degree.set_frequency(chord_notes[i].unwrap().to_freq_f32());
        }
    }
}

const OSCILLATORS_IN_DEGREE: usize = 2;

struct Degree<'a> {
    frequency: f32,
    detune_config: DetuneConfig,
    detune_phase: f32,
    detune_amplitude: f32,
    oscillators: [Oscillator<'a>; OSCILLATORS_IN_DEGREE],
}

impl<'a> Degree<'a> {
    pub fn new(wavetables: &'a [&'a Wavetable], sample_rate: u32) -> Self {
        Self {
            frequency: 0.0,
            detune_config: DetuneConfig::Disabled,
            detune_phase: 0.0,
            detune_amplitude: 0.0,
            oscillators: [
                Oscillator::new(wavetables, sample_rate),
                Oscillator::new(wavetables, sample_rate),
            ],
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.apply_settings();
    }

    pub fn set_detune(&mut self, detune_config: DetuneConfig, detune_phase: f32) {
        self.detune_config = detune_config;
        self.detune_phase = detune_phase;
        self.detune_amplitude = match detune_config {
            DetuneConfig::Disabled => 0.0,
            _ => {
                if detune_phase < 0.1 {
                    detune_phase / 0.1
                } else if detune_phase > 1.0 - 0.1 {
                    (1.0 - detune_phase) / 0.1
                } else {
                    1.0
                }
            }
        };
        self.apply_settings();
    }

    fn apply_settings(&mut self) {
        match self.detune_config {
            DetuneConfig::Disabled => {
                self.oscillators[0].frequency = self.frequency;
            }
            DetuneConfig::SingleSide(min, max) => {
                self.oscillators[0].frequency = self.frequency;

                for (i, oscillator) in self.oscillators[1..].iter_mut().enumerate() {
                    let detune_delta = max - min;
                    let stage = (i + 1) as f32;
                    let detune = (min + detune_delta * self.detune_phase) * stage;
                    oscillator.frequency = self.frequency * detune;
                }
            }
            DetuneConfig::BothSides(min, max) => {
                let start = if OSCILLATORS_IN_DEGREE % 2 == 0 { 0 } else { 1 };

                if start > 0 {
                    self.oscillators[0].frequency = self.frequency;
                }

                for (i, pair) in self.oscillators[start..].chunks_exact_mut(2).enumerate() {
                    let detune_delta = max - min;
                    let stage = (i + 1) as f32;
                    let detune = (min + detune_delta * self.detune_phase) * stage;
                    pair[0].frequency = self.frequency * (1.0 / detune);
                    pair[1].frequency = self.frequency * detune;
                }
            }
        }
    }

    pub fn set_wavetable(&mut self, wavetable: f32) {
        self.oscillators
            .iter_mut()
            .for_each(|o| o.wavetable = wavetable);
    }

    pub fn populate_add(&mut self, buffer: &mut [u16], amplitude: f32) {
        self.oscillators[0].populate_add(buffer, amplitude / OSCILLATORS_IN_DEGREE as f32 / 3.0);
        let amplitude = self.detune_amplitude / OSCILLATORS_IN_DEGREE as f32 / 3.0;
        self.oscillators[1..]
            .iter_mut()
            .for_each(|o| o.populate_add(buffer, amplitude));
    }
}

#[derive(Clone, Copy, PartialEq)]
enum DetuneConfig {
    Disabled,
    SingleSide(f32, f32),
    BothSides(f32, f32),
}

fn zero_slice(slice: &mut [u16]) {
    unsafe {
        let p = slice.as_mut_ptr();
        ptr::write_bytes(p, 0, slice.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_slice_contents_with_zeros() {
        let mut slice = [1, 2, 3];
        zero_slice(&mut slice);
        assert_eq!(slice, [0, 0, 0]);
    }
}
