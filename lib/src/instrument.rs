use core::cmp::PartialEq;
use core::ops::Deref;
use core::ptr;

#[allow(unused_imports)]
use micromath::F32Ext;

use crate::chords;
use crate::detune::DetuneConfig;
use crate::note::Note;
use crate::oscillator::Oscillator;
use crate::quantizer;
use crate::scales;
use crate::taper;
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
        DetuneConfig::SingleSide(0.5, 0.5 + 0.02),
        DetuneConfig::Disabled,
        DetuneConfig::Disabled,
    ],
    [
        DetuneConfig::SingleSide(0.5, 0.5 + 0.02),
        DetuneConfig::SingleSide(0.5, 0.5 + 0.02),
        DetuneConfig::SingleSide(0.5, 0.5 + 0.02),
    ],
    [
        DetuneConfig::BothSides(1.0, 1.01),
        DetuneConfig::BothSides(1.0, 1.01),
        DetuneConfig::BothSides(1.0, 1.01),
    ],
];

pub struct Instrument<'a> {
    scale_root: DiscreteParameter<Note>,
    scale_mode: DiscreteParameter<scales::diatonic::Mode>,
    chord_root_raw: f32,
    chord_root_degree: u8,
    chord_root_note: DiscreteParameter<Note>,
    chord_degrees_index: DiscreteParameter<usize>,
    selected_detune_index: DiscreteParameter<usize>,
    amplitude: f32,
    degrees: [Degree<'a>; DEGREES],
}

impl<'a> Instrument<'a> {
    pub fn new(wavetable_banks: &'a [&'a [Wavetable]], sample_rate: u32) -> Self {
        Self {
            scale_root: DiscreteParameter::new(Note::C1, 0.01),
            scale_mode: DiscreteParameter::new(scales::diatonic::Ionian, 0.001),
            chord_root_raw: 0.0,
            chord_root_note: DiscreteParameter::new(Note::C1, 0.01),
            chord_root_degree: 1,
            chord_degrees_index: DiscreteParameter::new(0, 0.001),
            selected_detune_index: DiscreteParameter::new(0, 0.001),
            amplitude: 1.0,
            degrees: [
                Degree::new(wavetable_banks, sample_rate),
                Degree::new(wavetable_banks, sample_rate),
                Degree::new(wavetable_banks, sample_rate),
            ],
        }
    }

    pub fn set_scale_mode(&mut self, scale_mode: f32) -> Option<scales::diatonic::Mode> {
        let original = self.scale_mode();

        let scale_mode = self.scale_mode.offset_raw(scale_mode);
        self.scale_mode.set(if scale_mode < 1.0 / 7.0 {
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
        });
        self.apply_settings();

        if original != self.scale_mode() {
            Some(self.scale_mode())
        } else {
            None
        }
    }

    pub fn scale_mode(&self) -> scales::diatonic::Mode {
        *self.scale_mode
    }

    pub fn set_scale_root(&mut self, scale_root: f32) -> Option<Note> {
        let original = self.scale_root();

        self.scale_root.set(quantizer::chromatic::quantize(
            self.scale_root.offset_raw(scale_root),
        ));
        self.apply_settings();

        if original != self.scale_root() {
            Some(self.scale_root())
        } else {
            None
        }
    }

    pub fn scale_root(&self) -> Note {
        *self.scale_root
    }

    pub fn set_chord_root(&mut self, chord_root: f32) -> Option<u8> {
        let original = self.chord_root_degree;

        self.chord_root_raw = chord_root;
        self.apply_settings();

        let updated = self.chord_root_degree;
        if original != updated {
            Some(updated)
        } else {
            None
        }
    }

    pub fn chord_root_degree(&self) -> u8 {
        self.chord_root_degree
    }

    pub fn set_chord_degrees(&mut self, chord_degrees: f32) -> Option<[i8; DEGREES]> {
        let original = *self.chord_degrees_index;

        self.chord_degrees_index.set(
            ((self.chord_degrees_index.offset_raw(chord_degrees) * CHORDS.len() as f32) as usize)
                .min(CHORDS.len() - 1),
        );
        self.apply_settings();

        if original != *self.chord_degrees_index {
            Some(self.chord_degrees())
        } else {
            None
        }
    }

    pub fn chord_degrees(&self) -> [i8; DEGREES] {
        CHORDS[*self.chord_degrees_index]
    }

    pub fn set_wavetable_bank(&mut self, wavetable_bank: f32) -> Option<usize> {
        let update = self.degrees[0].set_wavetable_bank(wavetable_bank);
        self.degrees[1..].iter_mut().for_each(|d| {
            d.set_wavetable_bank(wavetable_bank);
        });
        update
    }

    pub fn wavetable_bank(&self) -> usize {
        self.degrees[0].wavetable_bank()
    }

    pub fn set_wavetable(&mut self, wavetable: f32) -> Option<f32> {
        let original = self.wavetable();

        self.degrees
            .iter_mut()
            .for_each(|d| d.set_wavetable(wavetable));

        let updated = self.wavetable();

        if (original - updated).abs() > 0.01 {
            Some(updated)
        } else {
            None
        }
    }

    pub fn wavetable(&self) -> f32 {
        self.degrees[0].wavetable()
    }

    pub fn set_detune(&mut self, detune: f32) -> Option<(usize, f32)> {
        let original = self.detune();

        let detune = self.selected_detune_index.offset_raw(detune);
        let index = ((detune * DETUNES.len() as f32) as usize).min(DETUNES.len() - 1);
        self.selected_detune_index.set(index);

        // Slightly over 1, so it never hits the maximum and wraps back
        let section = 1.001 / DETUNES.len() as f32;
        let phase = taper::log((detune % section) / section);

        for (i, degree) in self.degrees.iter_mut().enumerate() {
            degree.set_detune(DETUNES[index][i], phase)
        }

        let updated = self.detune();
        if original.0 != updated.0 || (original.1 - updated.1).abs() > 0.01 {
            Some(updated)
        } else {
            None
        }
    }

    pub fn detune(&self) -> (usize, f32) {
        let index = *self.selected_detune_index;
        let phase = self.degrees[0].detune_phase;

        (index, phase)
    }

    pub fn amplitude(&self) -> f32 {
        self.amplitude
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    pub fn populate(&mut self, buffer_root: &mut [f32], buffer_chord: &mut [f32]) {
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

        let amplitude = self.amplitude;

        self.degrees[0].populate_add(
            buffer_root,
            amplitude * self.degrees[0].amplitude() / perceived_amplitude,
        );

        self.degrees[1..].iter_mut().for_each(|d| {
            d.populate_add(
                buffer_chord,
                amplitude * d.amplitude() / perceived_amplitude,
            )
        });
    }

    fn apply_settings(&mut self) {
        let (chord_root_note, chord_root_degree) = quantizer::diatonic::quantize(
            self.scale_mode(),
            self.scale_root(),
            self.chord_root_note.offset_raw(self.chord_root_raw),
        );

        self.chord_root_note.set(chord_root_note);
        self.chord_root_degree = chord_root_degree;

        let chord_notes = chords::diatonic::build(
            self.scale_root(),
            self.scale_mode(),
            chord_root_note,
            self.chord_degrees(),
        );

        for (i, degree) in self.degrees.iter_mut().enumerate() {
            if let Some(note) = chord_notes[i] {
                degree.set_frequency(note.to_freq_f32());
                degree.enable();
            } else {
                degree.disable();
            }
        }
    }
}

const OSCILLATORS_IN_DEGREE: usize = 2;

struct Degree<'a> {
    frequency: f32,
    detune_config: DetuneConfig,
    detune_phase: f32,
    wavetable_banks: &'a [&'a [Wavetable<'a>]],
    selected_wavetable_bank: DiscreteParameter<usize>,
    oscillators: [Oscillator<'a>; OSCILLATORS_IN_DEGREE],
    enabled: bool,
}

impl<'a> Degree<'a> {
    pub fn new(wavetable_banks: &'a [&'a [Wavetable]], sample_rate: u32) -> Self {
        assert!(!wavetable_banks.is_empty());
        Self {
            frequency: 0.0,
            detune_config: DetuneConfig::Disabled,
            detune_phase: 0.0,
            wavetable_banks,
            selected_wavetable_bank: DiscreteParameter::new(0, 0.001),
            oscillators: [
                Oscillator::new(wavetable_banks[0], sample_rate),
                Oscillator::new(wavetable_banks[0], sample_rate),
            ],
            enabled: true,
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

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
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

    pub fn set_wavetable_bank(&mut self, wavetable_bank: f32) -> Option<usize> {
        let original = self.wavetable_bank();

        self.selected_wavetable_bank.set(
            ((self.selected_wavetable_bank.offset_raw(wavetable_bank)
                * self.wavetable_banks.len() as f32) as usize)
                .min(self.wavetable_banks.len() - 1),
        );

        let wavetable_bank = self.wavetable_banks[self.wavetable_bank()];
        self.oscillators
            .iter_mut()
            .for_each(|o| o.wavetable_bank = wavetable_bank);

        if original != self.wavetable_bank() {
            Some(self.wavetable_bank())
        } else {
            None
        }
    }

    pub fn wavetable_bank(&self) -> usize {
        *self.selected_wavetable_bank
    }

    pub fn set_wavetable(&mut self, wavetable: f32) {
        self.oscillators
            .iter_mut()
            .for_each(|o| o.wavetable = wavetable);
    }

    pub fn wavetable(&self) -> f32 {
        self.oscillators[0].wavetable
    }

    pub fn populate_add(&mut self, buffer: &mut [f32], amplitude: f32) {
        if !self.enabled {
            return;
        }

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

fn zero_slice(slice: &mut [f32]) {
    unsafe {
        let p = slice.as_mut_ptr();
        ptr::write_bytes(p, 0, slice.len());
    }
}

#[derive(Clone, Copy)]
struct DiscreteParameter<T: PartialOrd + Copy> {
    offset: f32,
    next_offset: f32,
    value: T,
}

impl<T: Copy + PartialOrd> DiscreteParameter<T> {
    pub fn new(value: T, offset: f32) -> Self {
        Self {
            value,
            offset,
            next_offset: 0.0,
        }
    }

    pub fn offset_raw(&self, value: f32) -> f32 {
        value + self.next_offset
    }

    pub fn set(&mut self, value: T) {
        if value < self.value && (self.next_offset + self.offset).abs() > 0.0001 {
            self.next_offset = -1.0 * self.offset;
        } else if value > self.value && (self.next_offset - self.offset).abs() > 0.0001 {
            self.next_offset = self.offset;
        };
        self.value = value;
    }
}

impl<T: Copy + PartialOrd> Deref for DiscreteParameter<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Copy + PartialOrd> PartialEq for DiscreteParameter<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::waveform;

    const SAMPLE_RATE: u32 = 44_100;

    lazy_static! {
        static ref BANK_A: [Wavetable<'static>; 1] = [Wavetable::new(
            &waveform::perfect::PERFECT_2_FACTORS,
            SAMPLE_RATE
        )];
        static ref WAVETABLE_BANKS: [&'static [Wavetable<'static>]; 2] = [&BANK_A[..], &BANK_A[..]];
    }

    #[test]
    fn replace_slice_contents_with_zeros() {
        let mut slice = [1.0, 2.0, 3.0];
        zero_slice(&mut slice);
        assert_eq!(slice, [0.0, 0.0, 0.0]);
    }

    fn create_valid_instrument() -> Instrument<'static> {
        let mut instrument = Instrument::new(&WAVETABLE_BANKS[..], SAMPLE_RATE);
        instrument.set_scale_mode(0.0);
        instrument.set_scale_root(2.0);
        instrument.set_chord_root(2.5);
        instrument.set_chord_degrees(0.8);
        instrument.set_wavetable(0.1);
        instrument.set_detune(1.0);
        instrument
    }

    fn assert_populate(instrument: &mut Instrument) {
        let mut root_buffer = [-10.0; 64];
        let mut chord_buffer = [-10.0; 64];
        instrument.populate(&mut root_buffer, &mut chord_buffer);

        assert!(root_buffer[0].abs() <= 1.0);
        assert!(chord_buffer[0].abs() <= 1.0);
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

    fn assert_centered_around_zero(data: &[f32]) {
        let min = data.iter().fold(f32::MAX, |a, b| a.min(*b));
        let max = data.iter().fold(f32::MIN, |a, b| a.max(*b));
        let center = (min + max) / 2.0;
        let delta = center.abs() / 1.0;
        assert!(
            delta < 0.05,
            "Delta {} % is bigger than allowed",
            delta * 100.0
        );
    }

    #[test]
    fn output_centered_around_zero_simple() {
        let mut instrument = create_valid_instrument();
        instrument.set_chord_root(2.5);
        instrument.set_chord_degrees(0.0);

        let mut root_buffer = [0.0; 1024];
        let mut chord_buffer = [0.0; 1024];
        instrument.populate(&mut root_buffer, &mut chord_buffer);

        assert_centered_around_zero(&root_buffer);
        assert_centered_around_zero(&chord_buffer);
    }

    #[test]
    fn output_centered_around_zero_with_detune() {
        let mut instrument = create_valid_instrument();
        instrument.set_chord_root(2.5);
        instrument.set_chord_degrees(0.0);
        instrument.set_detune(1.0);

        let mut root_buffer = [0.0; 1024];
        let mut chord_buffer = [0.0; 1024];
        instrument.populate(&mut root_buffer, &mut chord_buffer);

        assert_centered_around_zero(&root_buffer);
        assert_centered_around_zero(&chord_buffer);
    }

    #[test]
    fn output_centered_around_zero_with_chord() {
        let mut instrument = create_valid_instrument();
        instrument.set_chord_root(2.5);
        instrument.set_chord_degrees(1.0);

        let mut root_buffer = [0.0; 1024];
        let mut chord_buffer = [0.0; 1024];
        instrument.populate(&mut root_buffer, &mut chord_buffer);

        assert_centered_around_zero(&root_buffer);
        assert_centered_around_zero(&chord_buffer);
    }

    #[test]
    fn change_chord_degrees() {
        let mut instrument = create_valid_instrument();
        instrument.set_chord_degrees(0.0);

        let new_degrees = instrument.set_chord_degrees(0.4);
        assert!(new_degrees.is_some());

        let new_degrees = instrument.set_chord_degrees(0.4);
        assert!(new_degrees.is_none());
    }

    #[test]
    fn get_chord_degrees() {
        let mut instrument = create_valid_instrument();

        instrument.set_chord_degrees(0.0);
        let old_degrees = instrument.chord_degrees();

        instrument.set_chord_degrees(0.5);
        let new_degrees = instrument.chord_degrees();

        assert!(old_degrees != new_degrees);
    }

    #[test]
    fn change_scale_root() {
        let mut instrument = create_valid_instrument();
        instrument.set_scale_root(1.0);

        let new_root = instrument.set_scale_root(1.0 + 2.0 / 12.0);
        assert!(new_root.is_some());

        let new_root = instrument.set_scale_root(1.0 + 2.0 / 12.0);
        assert!(new_root.is_none());
    }

    #[test]
    fn get_scale_root() {
        let mut instrument = create_valid_instrument();

        instrument.set_scale_root(1.0);
        let old_root = instrument.scale_root();

        instrument.set_scale_root(1.0 + 5.0 / 12.0);
        let new_root = instrument.scale_root();

        assert!(old_root != new_root);
    }

    #[test]
    fn change_scale_mode() {
        let mut instrument = create_valid_instrument();
        instrument.set_scale_mode(0.0);

        let new_mode = instrument.set_scale_mode(0.5);
        assert!(new_mode.is_some());

        let new_mode = instrument.set_scale_mode(0.5);
        assert!(new_mode.is_none());
    }

    #[test]
    fn get_scale_mode() {
        let mut instrument = create_valid_instrument();

        instrument.set_scale_mode(0.0);
        let old_mode = instrument.scale_mode();

        instrument.set_scale_mode(0.5);
        let new_mode = instrument.scale_mode();

        assert!(old_mode != new_mode);
    }

    #[test]
    fn change_chord_root() {
        let mut instrument = create_valid_instrument();
        instrument.set_chord_root(1.0);

        let new_root = instrument.set_chord_root(1.0 + 2.0 / 12.0);
        assert!(new_root.is_some());

        let new_root = instrument.set_chord_root(1.0 + 2.0 / 12.0);
        assert!(new_root.is_none());
    }

    #[test]
    fn get_chord_root_degree() {
        let mut instrument = create_valid_instrument();

        instrument.set_chord_root(1.0);
        let old_degree = instrument.chord_root_degree();

        instrument.set_chord_root(1.0 + 2.0 / 12.0);
        let new_degree = instrument.chord_root_degree();

        assert!(old_degree != new_degree);
    }

    #[test]
    fn change_wavetable_bank() {
        let mut instrument = create_valid_instrument();
        instrument.set_wavetable_bank(0.0);

        let new_bank = instrument.set_wavetable_bank(0.9);
        assert!(new_bank.is_some());

        let new_bank = instrument.set_wavetable_bank(0.9);
        assert!(new_bank.is_none());
    }

    #[test]
    fn get_wavetable_bank() {
        let mut instrument = create_valid_instrument();

        instrument.set_wavetable_bank(0.0);
        let old_bank = instrument.wavetable_bank();

        instrument.set_wavetable_bank(0.9);
        let new_bank = instrument.wavetable_bank();

        assert!(old_bank != new_bank);
    }

    #[test]
    fn change_wavetable() {
        let mut instrument = create_valid_instrument();
        instrument.set_wavetable(0.0);

        let new_wavetable = instrument.set_wavetable(0.9);
        assert!(new_wavetable.is_some());

        let new_wavetable = instrument.set_wavetable(0.9);
        assert!(new_wavetable.is_none());
    }

    #[test]
    fn get_wavetable() {
        let mut instrument = create_valid_instrument();

        instrument.set_wavetable(0.0);
        let old_wavetable = instrument.wavetable();

        instrument.set_wavetable(0.9);
        let new_wavetable = instrument.wavetable();

        assert!(old_wavetable != new_wavetable);
    }

    #[test]
    fn change_detune() {
        let mut instrument = create_valid_instrument();
        instrument.set_detune(0.0);

        let new_detune = instrument.set_detune(0.9);
        assert!(new_detune.is_some());

        let new_detune = instrument.set_detune(0.9);
        assert!(new_detune.is_none());
    }

    #[test]
    fn get_detune_index() {
        let mut instrument = create_valid_instrument();

        instrument.set_detune(0.0);
        let (old_detune_index, _) = instrument.detune();

        instrument.set_detune(0.9);
        let (new_detune_index, _) = instrument.detune();

        assert!(old_detune_index != new_detune_index);
    }

    #[test]
    fn get_detune_phase() {
        let mut instrument = create_valid_instrument();

        instrument.set_detune(0.0);
        let (_, old_detune_phase) = instrument.detune();

        instrument.set_detune(0.9);
        let (_, new_detune_phase) = instrument.detune();

        assert!(old_detune_phase != new_detune_phase);
    }
}
