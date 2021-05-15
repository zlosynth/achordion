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

const DETUNES: [[f32; 3]; 8] = [
    [0.0, 0.0, 0.0],
    [0.1, 0.1, 0.1],
    [0.5, 0.0, 0.0],
    [0.5, 0.0, 0.0],
    [0.5, 0.5, 0.5],
    [4.0 / 3.0, 4.0 / 3.0, 4.0 / 3.0],
    [4.0 / 3.0, 4.0 / 3.0, 4.0 / 3.0],
    [5.0 / 2.0, 5.0 / 2.0, 5.0 / 2.0],
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
        self.degrees.iter_mut().for_each(|d| d.set_detune(detune));
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
    detune_base: f32,
    detune_intensity: f32,
    detune_amplitude: f32,
    oscillators: [Oscillator<'a>; OSCILLATORS_IN_DEGREE],
}

impl<'a> Degree<'a> {
    pub fn new(wavetables: &'a [&'a Wavetable], sample_rate: u32) -> Self {
        Self {
            frequency: 0.0,
            detune_base: 1.0,
            detune_intensity: 0.0,
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

    pub fn set_detune(&mut self, detune: f32) {
        const TURN_ON_TRESHOLD: f32 = 0.02;

        self.detune_intensity = detune;

        if detune > TURN_ON_TRESHOLD {
            self.detune_amplitude = 1.0;
        } else {
            self.detune_amplitude = detune / TURN_ON_TRESHOLD;
        };

        self.apply_settings();
    }

    fn apply_settings(&mut self) {
        let start = if OSCILLATORS_IN_DEGREE % 2 == 0 { 0 } else { 1 };
        let distance = (OSCILLATORS_IN_DEGREE - start) / 2;

        self.oscillators[0].frequency = self.frequency;

        for (i, pair) in self.oscillators[start..].chunks_exact_mut(2).enumerate() {
            // TODO: Detune base
            let detune = 0.01 * self.detune_intensity * ((i + 1) / distance) as f32;
            pair[0].frequency = self.frequency * (1.0 - detune) * (1.0 / self.detune_base) * (i + 1) as f32;
            pair[1].frequency =
                self.frequency * (1.0 + detune) * self.detune_base * (i + 1) as f32;
        }
    }

    pub fn set_wavetable(&mut self, wavetable: f32) {
        self.oscillators
            .iter_mut()
            .for_each(|o| o.wavetable = wavetable);
    }

    pub fn populate_add(&mut self, buffer: &mut [u16], amplitude: f32) {
        self.oscillators[0].populate_add(buffer, amplitude / OSCILLATORS_IN_DEGREE as f32);

        let amplitude = amplitude * self.detune_amplitude / OSCILLATORS_IN_DEGREE as f32;
        self.oscillators[1..]
            .iter_mut()
            .for_each(|o| o.populate_add(buffer, amplitude));
    }
}

fn zero_slice(slice: &mut [u16]) {
    unsafe {
        let p = slice.as_mut_ptr();
        ptr::write_bytes(p, 0, slice.len());
    }
}

fn detune_interpolation<const N: usize>(detunes: &[[f32; N]], position: f32) -> [f32; N] {
    let index = position as usize;
    let next_index = usize::min(index + 1, DETUNES.len() - 1);
    let remainder = position - index as f32;

    let mut detune = [0.0; N];

    for i in 0..N {
        let value = detunes[index][i];
        let delta_to_next = detunes[next_index][i] - detunes[index][i];
        detune[i] = value + delta_to_next * remainder;
    }

    detune
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

    #[test]
    fn detune_interpolation_in_the_middle() {
        let detunes = [
            [0.0, 0.0],
            [1.0, 2.0],
        ];

        let detune = detune_interpolation(&detunes, 0.5);

        assert_relative_eq!(detune[0], 0.5);
        assert_relative_eq!(detune[1], 1.0);
    }
}
