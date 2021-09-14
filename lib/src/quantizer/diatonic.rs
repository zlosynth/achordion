#[allow(unused_imports)]
use micromath::F32Ext;

use crate::note::Note;
use crate::scales;
use crate::scales::diatonic::Mode;

pub type Degree = u8;

pub fn quantize_voct(mode: Mode, root: Note, mut voct: f32) -> (Note, Degree) {
    if voct > to_voct(Note::G9) {
        // One below the highest to allow it to quantize up
        voct = to_voct(Note::Gb9);
    }

    let root_white = voct_to_white_key(to_voct(root));
    let voct_white = voct_to_white_key(voct);

    let white_diff = voct_white as i32 - root_white as i32;
    let interval = if white_diff > 0 {
        white_diff + 1
    } else {
        white_diff - 1
    };

    let note = scales::diatonic::lookup_degree(root, mode, root, interval).unwrap();

    (note, white_diff.rem_euclid(7) as u8 + 1)
}

pub fn quantize_linear(mode: Mode, root: Note, mut value: f32) -> (Note, Degree) {
    if value > 10.0 {
        value = 10.0
    }

    let root_white = voct_to_white_key(to_voct(root));
    let value_white = linear_to_white_key(value);

    let white_diff = value_white as i32 - root_white as i32;
    let interval = if white_diff > 0 {
        white_diff + 1
    } else {
        white_diff - 1
    };

    let note = scales::diatonic::lookup_degree(root, mode, root, interval).unwrap();

    (note, white_diff.rem_euclid(7) as u8 + 1)
}

fn to_voct(note: Note) -> f32 {
    note.to_midi_id() as f32 / 12.0
}

fn voct_to_white_key(voct: f32) -> usize {
    let voct_trunc = voct.trunc();
    let white_octaves = voct_trunc as usize * 7;

    let voct_fract = voct.fract();
    let white_steps = if voct_fract < 1.5 / 12.0 {
        0
    } else if voct_fract < 3.5 / 12.0 {
        1
    } else if voct_fract < 4.5 / 12.0 {
        2
    } else if voct_fract < 6.5 / 12.0 {
        3
    } else if voct_fract < 8.5 / 12.0 {
        4
    } else if voct_fract < 10.5 / 12.0 {
        5
    } else if voct_fract < 11.5 / 12.0 {
        6
    } else {
        7
    };

    white_octaves + white_steps
}

fn linear_to_white_key(value: f32) -> usize {
    (7.0 * value) as usize
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
    fn convert_voct_to_white_key() {
        assert_eq!(voct_to_white_key(0.0), 0);
        assert_eq!(voct_to_white_key(1.0 / 12.0), 0);
        assert_eq!(voct_to_white_key(2.0 / 12.0), 1);
        assert_eq!(voct_to_white_key(3.0 / 12.0), 1);
        assert_eq!(voct_to_white_key(4.0 / 12.0), 2);
        assert_eq!(voct_to_white_key(5.0 / 12.0), 3);
        assert_eq!(voct_to_white_key(6.0 / 12.0), 3);
        assert_eq!(voct_to_white_key(7.0 / 12.0), 4);
        assert_eq!(voct_to_white_key(8.0 / 12.0), 4);
        assert_eq!(voct_to_white_key(9.0 / 12.0), 5);
        assert_eq!(voct_to_white_key(10.0 / 12.0), 5);
        assert_eq!(voct_to_white_key(11.0 / 12.0), 6);
        assert_eq!(voct_to_white_key(11.8 / 12.0), 7);
        assert_eq!(voct_to_white_key(1.0), 7);
    }

    #[test]
    fn convert_linear_to_white_key() {
        assert_eq!(linear_to_white_key(0.5 / 7.0), 0);
        assert_eq!(linear_to_white_key(1.5 / 7.0), 1);
        assert_eq!(linear_to_white_key(2.5 / 7.0), 2);
        assert_eq!(linear_to_white_key(3.5 / 7.0), 3);
        assert_eq!(linear_to_white_key(4.5 / 7.0), 4);
        assert_eq!(linear_to_white_key(5.5 / 7.0), 5);
        assert_eq!(linear_to_white_key(6.5 / 7.0), 6);
        assert_eq!(linear_to_white_key(1.0 + 0.5 / 7.0), 7);
    }

    #[test]
    fn quantize_voct_white_keys_in_c_major_with_root_below() {
        quantize_voct_white_keys_in_c_major_with_root(Note::C0);
    }

    #[test]
    fn quantize_voct_white_keys_in_c_major_with_root_above() {
        quantize_voct_white_keys_in_c_major_with_root(Note::C4);
    }

    fn quantize_voct_white_keys_in_c_major_with_root(root: Note) {
        let voct = 2.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::C1, 1));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::C1, 1)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::C1, 1)
        );

        let voct = 2.0 + 2.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::D1, 2));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::D1, 2)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::D1, 2)
        );

        let voct = 2.0 + 4.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::E1, 3));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::E1, 3)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::E1, 3)
        );

        let voct = 2.0 + 5.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::F1, 4));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::F1, 4)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::F1, 4)
        );

        let voct = 2.0 + 7.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::G1, 5));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::G1, 5)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::G1, 5)
        );

        let voct = 2.0 + 9.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::A1, 6));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::A1, 6)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::A1, 6)
        );

        let voct = 2.0 + 11.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::B1, 7));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::B1, 7)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::B1, 7)
        );

        let voct = 3.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::C2, 1));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::C2, 1)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::C2, 1)
        );
    }

    #[test]
    fn quantize_voct_white_keys_in_f_sharp_major_with_root_below() {
        quantize_voct_white_keys_in_f_sharp_major_with_root(Note::FSharp0);
    }

    #[test]
    fn quantize_voct_white_keys_in_f_sharp_major_with_root_above() {
        quantize_voct_white_keys_in_f_sharp_major_with_root(Note::FSharp3);
    }

    fn quantize_voct_white_keys_in_f_sharp_major_with_root(root: Note) {
        let voct = 2.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::CSharp1, 5));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::CSharp1, 5)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::CSharp1, 5)
        );

        let voct = 2.0 + 2.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::DSharp1, 6));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::DSharp1, 6)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::DSharp1, 6)
        );

        let voct = 2.0 + 4.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::F1, 7));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::F1, 7)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::F1, 7)
        );

        let voct = 2.0 + 5.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::FSharp1, 1));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::FSharp1, 1)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::FSharp1, 1)
        );

        let voct = 2.0 + 7.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::GSharp1, 2));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::GSharp1, 2)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::GSharp1, 2)
        );

        let voct = 2.0 + 9.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::ASharp1, 3));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::ASharp1, 3)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::ASharp1, 3)
        );

        let voct = 2.0 + 11.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::B1, 4));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::B1, 4)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::B1, 4)
        );

        let voct = 3.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::CSharp2, 5));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::CSharp2, 5)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::CSharp2, 5)
        );
    }

    #[test]
    fn quantize_voct_black_keys_in_c_major_with_root_below() {
        quantize_voct_black_keys_in_c_major_with_root(Note::C0);
    }

    #[test]
    fn quantize_voct_black_keys_in_c_major_with_root_above() {
        quantize_voct_black_keys_in_c_major_with_root(Note::C4);
    }

    fn quantize_voct_black_keys_in_c_major_with_root(root: Note) {
        let voct = 2.0 + 1.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::C1, 1));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::C1, 1)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::C1, 1)
        );

        let voct = 2.0 + 3.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::D1, 2));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::D1, 2)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::D1, 2)
        );

        let voct = 2.0 + 6.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::F1, 4));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::F1, 4)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::F1, 4)
        );

        let voct = 2.0 + 8.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::G1, 5));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::G1, 5)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::G1, 5)
        );

        let voct = 2.0 + 10.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::A1, 6));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::A1, 6)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::A1, 6)
        );
    }

    #[test]
    fn quantize_voct_black_keys_in_f_sharp_major_with_root_below() {
        quantize_voct_black_keys_in_f_sharp_major_with_root(Note::FSharp0);
    }

    #[test]
    fn quantize_voct_black_keys_in_f_sharp_major_with_root_above() {
        quantize_voct_black_keys_in_f_sharp_major_with_root(Note::FSharp3);
    }

    fn quantize_voct_black_keys_in_f_sharp_major_with_root(root: Note) {
        let voct = 2.0 + 1.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::CSharp1, 5));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::CSharp1, 5)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::CSharp1, 5)
        );

        let voct = 2.0 + 3.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::DSharp1, 6));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::DSharp1, 6)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::DSharp1, 6)
        );

        let voct = 2.0 + 6.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::FSharp1, 1));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::FSharp1, 1)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::FSharp1, 1)
        );

        let voct = 2.0 + 8.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::GSharp1, 2));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::GSharp1, 2)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::GSharp1, 2)
        );

        let voct = 2.0 + 10.0 / 12.0;
        assert_eq!(quantize_voct(Ionian, root, voct), (Note::ASharp1, 3));
        assert_eq!(
            quantize_voct(Ionian, root, voct - 0.4 / 12.0),
            (Note::ASharp1, 3)
        );
        assert_eq!(
            quantize_voct(Ionian, root, voct + 0.4 / 12.0),
            (Note::ASharp1, 3)
        );
    }

    #[test]
    fn quantize_voct_with_note_below_the_lowest_root() {
        let voct = 1.0 / 12.0;
        assert_eq!(
            quantize_voct(Ionian, Note::A0, voct),
            (Note::CSharpMinus1, 3)
        );
    }

    #[test]
    fn quantize_voct_note_over_limit_and_stay_in_scale() {
        let voct = 100.0;
        assert_eq!(quantize_voct(Ionian, Note::B0, voct), (Note::FSharp9, 5));
    }

    #[test]
    fn quantize_linear_with_note_below_the_lowest_root() {
        let value = 0.5 / 7.0;
        assert_eq!(
            quantize_linear(Ionian, Note::A0, value),
            (Note::CSharpMinus1, 3)
        );
    }

    #[test]
    fn quantize_linear_note_over_limit_and_stay_in_scale() {
        let value = 100.0;
        assert_eq!(quantize_linear(Ionian, Note::B0, value), (Note::CSharp9, 2));
    }

    #[test]
    fn quantize_linear_c_major_with_root_below() {
        quantize_linear_c_major_with_root(Note::C0);
    }

    #[test]
    fn quantize_linear_c_major_with_root_above() {
        quantize_linear_c_major_with_root(Note::C4);
    }

    fn quantize_linear_c_major_with_root(root: Note) {
        let value = 2.0 + 0.5 / 7.0;
        assert_eq!(quantize_linear(Ionian, root, value), (Note::C1, 1));

        let value = 2.0 + 1.5 / 7.0;
        assert_eq!(quantize_linear(Ionian, root, value), (Note::D1, 2));

        let value = 2.0 + 2.5 / 7.0;
        assert_eq!(quantize_linear(Ionian, root, value), (Note::E1, 3));

        let value = 2.0 + 3.5 / 7.0;
        assert_eq!(quantize_linear(Ionian, root, value), (Note::F1, 4));

        let value = 2.0 + 4.5 / 7.0;
        assert_eq!(quantize_linear(Ionian, root, value), (Note::G1, 5));

        let value = 2.0 + 5.5 / 7.0;
        assert_eq!(quantize_linear(Ionian, root, value), (Note::A1, 6));

        let value = 2.0 + 6.5 / 7.0;
        assert_eq!(quantize_linear(Ionian, root, value), (Note::B1, 7));

        let value = 3.0 + 0.5 / 7.0;
        assert_eq!(quantize_linear(Ionian, root, value), (Note::C2, 1));
    }
}
