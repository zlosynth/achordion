#[allow(unused_imports)]
use micromath::F32Ext;

use crate::midi::note::Note;

pub fn quantize(root: Note, mut voct: f32) -> Note {
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

    if difference <= 1.1 / 12.0 {
        to_midi_note(close_root)
    } else if difference <= 3.1 / 12.0 {
        to_midi_note(close_root + 2.0 / 12.0)
    } else if difference <= 4.5 / 12.0 {
        to_midi_note(close_root + 4.0 / 12.0)
    } else if difference <= 6.1 / 12.0 {
        to_midi_note(close_root + 5.0 / 12.0)
    } else if difference <= 8.1 / 12.0 {
        to_midi_note(close_root + 7.0 / 12.0)
    } else if difference <= 10.1 / 12.0 {
        to_midi_note(close_root + 9.0 / 12.0)
    } else if difference <= 11.5 / 12.0 {
        to_midi_note(close_root + 11.0 / 12.0)
    } else {
        to_midi_note(close_root + 1.0)
    }
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
        assert_eq!(quantize(Note::C0, voct), Note::C1);

        let voct = 2.0 + 2.0 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::D1);

        let voct = 2.0 + 4.0 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::E1);

        let voct = 2.0 + 5.0 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::F1);

        let voct = 2.0 + 7.0 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::G1);

        let voct = 2.0 + 9.0 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::A1);

        let voct = 2.0 + 11.0 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::B1);

        let voct = 3.0;
        assert_eq!(quantize(Note::C0, voct), Note::C2);
    }

    #[test]
    fn quantize_on_spot_with_note_below_the_lowest_root() {
        let voct = 1.0 / 12.0;
        assert_eq!(quantize(Note::A0, voct), Note::CSharpMinus1);
    }

    #[test]
    fn quantize_on_spot_voct_to_note_with_root_above() {
        let voct = 2.0;
        assert_eq!(quantize(Note::C3, voct), Note::C1);

        let voct = 2.0 + 2.0 / 12.0;
        assert_eq!(quantize(Note::C3, voct), Note::D1);

        let voct = 2.0 + 4.0 / 12.0;
        assert_eq!(quantize(Note::C3, voct), Note::E1);

        let voct = 2.0 + 5.0 / 12.0;
        assert_eq!(quantize(Note::C3, voct), Note::F1);

        let voct = 2.0 + 7.0 / 12.0;
        assert_eq!(quantize(Note::C3, voct), Note::G1);

        let voct = 2.0 + 9.0 / 12.0;
        assert_eq!(quantize(Note::C3, voct), Note::A1);

        let voct = 2.0 + 11.0 / 12.0;
        assert_eq!(quantize(Note::C3, voct), Note::B1);

        let voct = 3.0;
        assert_eq!(quantize(Note::C3, voct), Note::C2);
    }

    #[test]
    fn quantize_note_over_limit_and_stay_in_scale() {
        let voct = 100.0;
        assert_eq!(quantize(Note::B0, voct), Note::Gb9);
    }

    #[test]
    fn quantize_distant_voct_to_note() {
        let voct = 2.0 + 0.4 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::C1);

        let voct = 2.0 + 1.3 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::D1);

        let voct = 2.0 + 3.2 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::E1);

        let voct = 2.0 + 4.6 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::F1);

        let voct = 2.0 + 7.9 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::G1);

        let voct = 2.0 + 9.2 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::A1);

        let voct = 2.0 + 11.4 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::B1);

        let voct = 3.0 - 0.4 / 12.0;
        assert_eq!(quantize(Note::C0, voct), Note::C2);
    }
}
