use core::ptr;

use crate::chords;
use crate::note::Note;
use crate::oscillator::Oscillator;
use crate::quantizer;
use crate::scales;
use crate::wavetable::Wavetable;

const DEGREES: usize = 3;

const CHORDS: [[i8; DEGREES]; 22] = [
    [1, 0, 0],
    [1, 2, 0],
    [1, 3, 0],
    [1, 4, 0],
    [1, 5, 0],
    [1, 6, 0],
    [1, 7, 0],
    [1, 3, 5],
    [1, 2, 5],
    [1, 4, 5],
    [1, 5, 7],
    [1, 3, 7],
    [1, 4, 7],
    [1, 2, 7],
    [1, 5, 9],
    [1, 2, 9],
    [1, 7 + 5, 7 + 3],
    [1, 7 + 7, 7 + 3],
    [1, 7 + 9, 7 + 3],
    [1, 3, 5],
    [-4, 1, 3],
    [-6, 1, 5],
];

const DETUNES: [[DetuneConfig; DEGREES]; 4] = [
    [
        DetuneConfig::Disabled,
        DetuneConfig::Disabled,
        DetuneConfig::Disabled,
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
    [
        DetuneConfig::BothSides(1.0, 1.01),
        DetuneConfig::BothSides(1.0, 1.01),
        DetuneConfig::BothSides(1.0, 1.01),
    ],
];

pub struct Instrument<'a> {
    scale_root: Note,
    scale_mode: scales::diatonic::Mode,
    chord_root: f32,
    chord_degrees: [i8; DEGREES],
    degrees: [Degree<'a>; DEGREES],
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

        // Amplitude of N mixed voices is not N times higher than the one of a
        // single one. Express perceived amplitude by increasing lower values.
        // This should make changes between different numbers of oscillators
        // less noticable.
        let perceived_amplitude = {
            let max_amplitude = (DEGREES * OSCILLATORS_IN_DEGREE) as f32;
            let total_amplitude = self.degrees.iter().fold(0.0, |a, d| a + d.amplitude());
            (total_amplitude + max_amplitude) / 2.0
        };

        self.degrees[0].populate_add(
            buffer_root,
            self.degrees[0].amplitude() / perceived_amplitude,
        );

        self.degrees[1..]
            .iter_mut()
            .for_each(|d| d.populate_add(buffer_chord, d.amplitude() / perceived_amplitude));
    }

    fn apply_settings(&mut self) {
        let chord_root_note =
            quantizer::diatonic::quantize(self.scale_mode, self.scale_root, self.chord_root);

        let chord_notes = chords::diatonic::build(
            self.scale_root,
            self.scale_mode,
            chord_root_note,
            self.chord_degrees,
        );

        for (i, degree) in self.degrees.iter_mut().enumerate() {
            if let Some(note) = chord_notes[i] {
                degree.set_frequency(note.to_freq_f32());
            } else {
                degree.set_frequency(0.0);
            }
        }
    }
}

const OSCILLATORS_IN_DEGREE: usize = 2;

struct Degree<'a> {
    frequency: f32,
    detune_config: DetuneConfig,
    detune_phase: f32,
    oscillators: [Oscillator<'a>; OSCILLATORS_IN_DEGREE],
}

impl<'a> Degree<'a> {
    pub fn new(wavetables: &'a [&'a Wavetable], sample_rate: u32) -> Self {
        Self {
            frequency: 0.0,
            detune_config: DetuneConfig::Disabled,
            detune_phase: 0.0,
            oscillators: [
                Oscillator::new(wavetables, sample_rate),
                Oscillator::new(wavetables, sample_rate),
            ],
        }
    }

    pub fn amplitude(&self) -> f32 {
        match self.detune_config {
            DetuneConfig::Disabled => 1.0,
            _ => OSCILLATORS_IN_DEGREE as f32,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.apply_settings();
    }

    pub fn set_detune(&mut self, detune_config: DetuneConfig, detune_phase: f32) {
        self.detune_config = detune_config;
        self.detune_phase = detune_phase;
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
        match self.detune_config {
            DetuneConfig::Disabled => {
                self.oscillators[0].populate_add(buffer, amplitude);
            }
            _ => {
                self.oscillators
                    .iter_mut()
                    .for_each(|o| o.populate_add(buffer, amplitude / OSCILLATORS_IN_DEGREE as f32));
            }
        }
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
    use crate::waveform;

    const SAMPLE_RATE: u32 = 44_100;

    lazy_static! {
        static ref WAVETABLE: Wavetable<'static> =
            Wavetable::new(&waveform::saw::SAW_FACTORS, SAMPLE_RATE);
        static ref WAVETABLES: [&'static Wavetable<'static>; 1] = [&WAVETABLE];
    }

    #[test]
    fn replace_slice_contents_with_zeros() {
        let mut slice = [1, 2, 3];
        zero_slice(&mut slice);
        assert_eq!(slice, [0, 0, 0]);
    }

    fn create_valid_instrument() -> Instrument<'static> {
        let mut instrument = Instrument::new(&WAVETABLES[..], SAMPLE_RATE);
        instrument.set_scale_mode(0.0);
        instrument.set_scale_root(2.0);
        instrument.set_chord_root(2.5);
        instrument.set_chord_degrees(0.8);
        instrument.set_wavetable(0.1);
        instrument.set_detune(1.0);
        instrument
    }

    fn assert_populate(instrument: &mut Instrument) {
        let mut root_buffer = [0; 64];
        let mut chord_buffer = [0; 64];
        instrument.populate(&mut root_buffer, &mut chord_buffer);

        assert!(root_buffer[0] > 0);
        assert!(chord_buffer[0] > 0);
    }

    #[test]
    fn populate_when_all_is_in_range() {
        let mut instrument = create_valid_instrument();
        assert_populate(&mut instrument);
    }

    #[test]
    fn recover_after_scale_mode_was_set_above_range() {
        let mut instrument = create_valid_instrument();
        instrument.set_scale_mode(100.0);
        assert_populate(&mut instrument);
    }

    #[test]
    fn recover_after_scale_mode_was_set_below_range() {
        let mut instrument = create_valid_instrument();
        instrument.set_scale_mode(-100.0);
        assert_populate(&mut instrument);
    }

    #[test]
    fn recover_after_scale_root_was_set_above_range() {
        let mut instrument = create_valid_instrument();
        instrument.set_scale_root(100.0);
        assert_populate(&mut instrument);
    }

    #[test]
    fn recover_after_scale_root_was_set_below_range() {
        let mut instrument = create_valid_instrument();
        instrument.set_scale_root(-100.0);
        assert_populate(&mut instrument);
    }

    #[test]
    fn recover_after_chord_root_was_set_above_range() {
        let mut instrument = create_valid_instrument();
        instrument.set_chord_root(100.0);
        assert_populate(&mut instrument);
    }

    #[test]
    fn recover_after_chord_root_was_set_below_range() {
        let mut instrument = create_valid_instrument();
        instrument.set_chord_root(-100.0);
        assert_populate(&mut instrument);
    }

    #[test]
    fn recover_after_chord_degrees_were_set_above_range() {
        let mut instrument = create_valid_instrument();
        instrument.set_chord_degrees(10.0);
        assert_populate(&mut instrument);
    }

    #[test]
    fn recover_after_chord_degrees_were_set_below_range() {
        let mut instrument = create_valid_instrument();
        instrument.set_chord_degrees(-10.0);
        assert_populate(&mut instrument);
    }

    #[test]
    fn recover_after_wavetable_was_set_above_range() {
        let mut instrument = create_valid_instrument();
        instrument.set_wavetable(10.0);
        assert_populate(&mut instrument);
    }

    #[test]
    fn recover_after_wavetable_was_set_below_range() {
        let mut instrument = create_valid_instrument();
        instrument.set_wavetable(-10.0);
        assert_populate(&mut instrument);
    }

    #[test]
    fn recover_after_detune_was_set_above_range() {
        let mut instrument = create_valid_instrument();
        instrument.set_detune(10.0);
        assert_populate(&mut instrument);
    }

    #[test]
    fn recover_after_detune_was_set_below_range() {
        let mut instrument = create_valid_instrument();
        instrument.set_detune(-10.0);
        assert_populate(&mut instrument);
    }
}
