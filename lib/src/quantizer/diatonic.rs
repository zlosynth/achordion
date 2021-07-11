#[allow(unused_imports)]
use micromath::F32Ext;

use crate::note::Note;
use crate::scales::diatonic::{Mode, SEMITONES};

pub type Degree = u8;

// TODO: Return which degree it is
pub fn quantize(mode: Mode, root: Note, mut voct: f32) -> (Note, Degree) {
    if voct > to_voct(Note::HIGHEST_NOTE) {
        voct = to_voct(Note::HIGHEST_NOTE);
    }

    let root_frac = to_voct(root).fract();
    let voct_trunc = voct.trunc();
    let voct_frac = voct.fract();

    let close_root = if root_frac <= voct_frac {
        voct_trunc + root_frac
    } else {
        voct_trunc - 1.0 + root_frac
    };

    let difference = voct - close_root;

    let semitones = SEMITONES[mode as usize];
    if difference < above_mean(semitones[0], semitones[1]) / 12.0 {
        (to_midi_note(close_root), 1)
    } else if difference < above_mean(semitones[1], semitones[2]) / 12.0 {
        (to_midi_note(close_root + semitones[1] as f32 / 12.0), 2)
    } else if difference < above_mean(semitones[2], semitones[3]) / 12.0 {
        (to_midi_note(close_root + semitones[2] as f32 / 12.0), 3)
    } else if difference < above_mean(semitones[3], semitones[4]) / 12.0 {
        (to_midi_note(close_root + semitones[3] as f32 / 12.0), 4)
    } else if difference < above_mean(semitones[4], semitones[5]) / 12.0 {
        (to_midi_note(close_root + semitones[4] as f32 / 12.0), 5)
    } else if difference < above_mean(semitones[5], semitones[6]) / 12.0 {
        (to_midi_note(close_root + semitones[5] as f32 / 12.0), 6)
    } else if difference < above_mean(semitones[6], 12) / 12.0 {
        (to_midi_note(close_root + semitones[6] as f32 / 12.0), 7)
    } else {
        (to_midi_note(close_root + 1.0), 1)
    }
}

fn above_mean(a: i8, b: i8) -> f32 {
    debug_assert!(b > a);
    a as f32 + (b as f32 - a as f32) * 0.55
}

fn to_voct(note: Note) -> f32 {
    note.to_midi_id() as f32 / 12.0
}

fn to_midi_note(voct: f32) -> Note {
    Note::from_u8((voct * 12.0) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scales::diatonic::Mode::*;

    #[test]
    fn convert_midi_note_to_voct() {
        assert_relative_eq!(to_voct(Note::A4), 5.0 + 9.0 / 12.0);
    }

    #[test]
    fn convert_voct_to_midi_note() {
        assert_eq!(to_midi_note(2.0 + 2.0 / 12.0), Note::D1);
    }

    #[test]
    fn quantize_on_spot_voct_to_note_with_root_below() {
        let voct = 2.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::C1, 1));

        let voct = 2.0 + 2.0 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::D1, 2));

        let voct = 2.0 + 4.0 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::E1, 3));

        let voct = 2.0 + 5.0 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::F1, 4));

        let voct = 2.0 + 7.0 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::G1, 5));

        let voct = 2.0 + 9.0 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::A1, 6));

        let voct = 2.0 + 11.0 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::B1, 7));

        let voct = 3.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::C2, 1));
    }

    #[test]
    fn quantize_on_spot_with_note_below_the_lowest_root() {
        let voct = 1.0 / 12.0;
        assert_eq!(quantize(Ionian, Note::A0, voct), (Note::CSharpMinus1, 3));
    }

    #[test]
    fn quantize_on_spot_voct_to_note_with_root_above() {
        let voct = 2.0;
        assert_eq!(quantize(Ionian, Note::C3, voct), (Note::C1, 1));

        let voct = 2.0 + 2.0 / 12.0;
        assert_eq!(quantize(Ionian, Note::C3, voct), (Note::D1, 2));

        let voct = 2.0 + 4.0 / 12.0;
        assert_eq!(quantize(Ionian, Note::C3, voct), (Note::E1, 3));

        let voct = 2.0 + 5.0 / 12.0;
        assert_eq!(quantize(Ionian, Note::C3, voct), (Note::F1, 4));

        let voct = 2.0 + 7.0 / 12.0;
        assert_eq!(quantize(Ionian, Note::C3, voct), (Note::G1, 5));

        let voct = 2.0 + 9.0 / 12.0;
        assert_eq!(quantize(Ionian, Note::C3, voct), (Note::A1, 6));

        let voct = 2.0 + 11.0 / 12.0;
        assert_eq!(quantize(Ionian, Note::C3, voct), (Note::B1, 7));

        let voct = 3.0;
        assert_eq!(quantize(Ionian, Note::C3, voct), (Note::C2, 1));
    }

    #[test]
    fn quantize_note_over_limit_and_stay_in_scale() {
        let voct = 100.0;
        assert_eq!(quantize(Ionian, Note::B0, voct), (Note::FSharp9, 5));
    }

    #[test]
    fn quantize_distant_voct_to_note() {
        let voct = 2.0 + 0.4 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::C1, 1));

        let voct = 2.0 + 1.3 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::D1, 2));

        let voct = 2.0 + 3.3 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::E1, 3));

        let voct = 2.0 + 4.7 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::F1, 4));

        let voct = 2.0 + 7.9 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::G1, 5));

        let voct = 2.0 + 9.2 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::A1, 6));

        let voct = 2.0 + 11.4 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::B1, 7));

        let voct = 3.0 - 0.4 / 12.0;
        assert_eq!(quantize(Ionian, Note::C0, voct), (Note::C2, 1));
    }
}
