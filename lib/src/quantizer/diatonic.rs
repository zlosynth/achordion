#[allow(unused_imports)]
use micromath::F32Ext;

use crate::note::Note;
use crate::scales;
use crate::scales::diatonic::{Mode, SEMITONES};

pub type Degree = u8;

#[cfg(feature = "even_quantization")]
pub fn quantize_voct(mode: Mode, root: Note, voct: f32) -> (Note, Degree) {
    quantize_voct_center(mode, root, voct)
}

#[cfg(not(feature = "even_quantization"))]
pub fn quantize_voct(mode: Mode, root: Note, voct: f32) -> (Note, Degree) {
    quantize_voct_white_keys(mode, root, voct)
}

fn quantize_voct_white_keys(mode: Mode, root: Note, mut voct: f32) -> (Note, Degree) {
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

#[allow(dead_code)]
fn quantize_voct_center(mode: Mode, root: Note, voct: f32) -> (Note, Degree) {
    // XXX: This is making the method simpler by sacrificing a part of the
    // lowest octave.
    let lowest_tonic = lowest_note(root);
    if voct < lowest_tonic.to_voct() {
        return (lowest_tonic, 0);
    }

    let closest_tonic = find_closest_tonic(lowest_tonic, voct);
    if closest_tonic.is_none() {
        return (lowest_tonic, 0);
    }
    let closest_tonic = closest_tonic.unwrap();

    let surrounding_notes = find_surrounding_notes_ascending(mode, voct, closest_tonic);
    if surrounding_notes.is_none() {
        return (lowest_tonic, 0);
    }
    let ((below_note, below_degree), (above_note, above_degree)) = surrounding_notes.unwrap();

    let center_voct = (below_note.to_voct() + above_note.to_voct()) / 2.0;
    if voct < center_voct {
        (below_note, below_degree)
    } else {
        (above_note, above_degree)
    }
}

fn lowest_note(note: Note) -> Note {
    match note {
        Note::A0
        | Note::A1
        | Note::A2
        | Note::A3
        | Note::A4
        | Note::A5
        | Note::A6
        | Note::A7
        | Note::A8
        | Note::AMinus1 => Note::AMinus1,
        Note::Ab0
        | Note::Ab1
        | Note::Ab2
        | Note::Ab3
        | Note::Ab4
        | Note::Ab5
        | Note::Ab6
        | Note::Ab7
        | Note::Ab8
        | Note::AbMinus1 => Note::AbMinus1,
        Note::B0
        | Note::B1
        | Note::B2
        | Note::B3
        | Note::B4
        | Note::B5
        | Note::B6
        | Note::B7
        | Note::B8
        | Note::BMinus1 => Note::BMinus1,
        Note::Bb0
        | Note::Bb1
        | Note::Bb2
        | Note::Bb3
        | Note::Bb4
        | Note::Bb5
        | Note::Bb6
        | Note::Bb7
        | Note::Bb8
        | Note::BbMinus1 => Note::BbMinus1,
        Note::C0
        | Note::C1
        | Note::C2
        | Note::C3
        | Note::C4
        | Note::C5
        | Note::C6
        | Note::C7
        | Note::C8
        | Note::C9
        | Note::CMinus1 => Note::CMinus1,
        Note::D0
        | Note::D1
        | Note::D2
        | Note::D3
        | Note::D4
        | Note::D5
        | Note::D6
        | Note::D7
        | Note::D8
        | Note::D9
        | Note::DMinus1 => Note::DMinus1,
        Note::Db0
        | Note::Db1
        | Note::Db2
        | Note::Db3
        | Note::Db4
        | Note::Db5
        | Note::Db6
        | Note::Db7
        | Note::Db8
        | Note::Db9
        | Note::DbMinus1 => Note::DbMinus1,
        Note::E0
        | Note::E1
        | Note::E2
        | Note::E3
        | Note::E4
        | Note::E5
        | Note::E6
        | Note::E7
        | Note::E8
        | Note::E9
        | Note::EMinus1 => Note::EMinus1,
        Note::Eb0
        | Note::Eb1
        | Note::Eb2
        | Note::Eb3
        | Note::Eb4
        | Note::Eb5
        | Note::Eb6
        | Note::Eb7
        | Note::Eb8
        | Note::Eb9
        | Note::EbMinus1 => Note::EbMinus1,
        Note::F0
        | Note::F1
        | Note::F2
        | Note::F3
        | Note::F4
        | Note::F5
        | Note::F6
        | Note::F7
        | Note::F8
        | Note::F9
        | Note::FMinus1 => Note::FMinus1,
        Note::G0
        | Note::G1
        | Note::G2
        | Note::G3
        | Note::G4
        | Note::G5
        | Note::G6
        | Note::G7
        | Note::G8
        | Note::G9
        | Note::GMinus1 => Note::GMinus1,
        Note::Gb0
        | Note::Gb1
        | Note::Gb2
        | Note::Gb3
        | Note::Gb4
        | Note::Gb5
        | Note::Gb6
        | Note::Gb7
        | Note::Gb8
        | Note::Gb9
        | Note::GbMinus1 => Note::GbMinus1,
    }
}

fn find_closest_tonic(lowest_tonic: Note, voct: f32) -> Option<Note> {
    let distance_in_full_octaves = (voct - lowest_tonic.to_voct()) as u8;
    Note::try_from_u8(lowest_tonic.to_midi_id() + 12 * distance_in_full_octaves.min(11))
}

fn find_surrounding_notes_ascending(
    mode: Mode,
    voct: f32,
    closest_tonic: Note,
) -> Option<((Note, Degree), (Note, Degree))> {
    let mut below_note = closest_tonic;
    let mut below_degree = 1;
    let mut above_note = None;
    let mut above_degree = None;

    let mut distance = 0;
    let semitones = SEMITONES[mode as usize];
    // TODO: Move this to a function
    let mut steps = [0; 7];
    for i in 0..steps.len() {
        steps[i] = semitones.get(i + 1).unwrap_or(&12) - semitones[i];
    }
    for (i, step) in steps.iter().enumerate() {
        distance += step;
        let note = Note::try_from_u8(closest_tonic.to_midi_id() + distance as u8)?;
        let index = i as u8;
        let degree = if index == 6 { 1 } else { index + 2 };
        if note.to_voct() > voct {
            above_note = Some(note);
            above_degree = Some(degree);
            break;
        }
        below_note = note;
        below_degree = degree;
    }

    Some(((below_note, below_degree), (above_note?, above_degree?)))
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

// Python for min-maxing:
//
// ```ignore
// >>> print(
//     12 - T / 2 + O,
//     T / 2 + O,
//     T / 2 + T + O,
//     T / 2 + T + ST + O,
//     T / 2 + T + ST + T + O,
//     T / 2 + T + ST + 2 * T + O,
//     T / 2 + T + ST + 3 * T + O,
//     T / 2 + T + ST + 3 * T + ST + O,
// )
// ```
pub fn voct_to_white_key(voct: f32) -> usize {
    const T: f32 = 12.0 / 7.0 + 0.05;
    const ST: f32 = (12.0 - 5.0 * T) / 2.0;
    const O: f32 = 0.55;

    let voct_trunc = voct.trunc();
    let white_octaves = voct_trunc as usize * 7;

    let voct_fract = voct.fract();
    let white_steps = if voct_fract < (T / 2.0 + O) / 12.0 {
        0
    } else if voct_fract < (T / 2.0 + T + O) / 12.0 {
        1
    } else if voct_fract < (T / 2.0 + T + ST + O) / 12.0 {
        2
    } else if voct_fract < (T / 2.0 + T + ST + T + O) / 12.0 {
        3
    } else if voct_fract < (T / 2.0 + T + ST + 2.0 * T + O) / 12.0 {
        4
    } else if voct_fract < (T / 2.0 + T + ST + 3.0 * T + O) / 12.0 {
        5
    } else if voct_fract < (T / 2.0 + T + ST + 3.0 * T + ST + O) / 12.0 {
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
        assert_eq!(voct_to_white_key(9.9 / 12.0), 5);
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
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::C1, 1));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::C1, 1)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::C1, 1)
        );

        let voct = 2.0 + 2.0 / 12.0;
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::D1, 2));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::D1, 2)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::D1, 2)
        );

        let voct = 2.0 + 4.0 / 12.0;
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::E1, 3));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::E1, 3)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::E1, 3)
        );

        let voct = 2.0 + 5.0 / 12.0;
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::F1, 4));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::F1, 4)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::F1, 4)
        );

        let voct = 2.0 + 7.0 / 12.0;
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::G1, 5));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::G1, 5)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::G1, 5)
        );

        let voct = 2.0 + 9.0 / 12.0;
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::A1, 6));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::A1, 6)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::A1, 6)
        );

        let voct = 2.0 + 11.0 / 12.0;
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::B1, 7));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::B1, 7)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::B1, 7)
        );

        let voct = 3.0;
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::C2, 1));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::C2, 1)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::C2, 1)
        );
    }

    #[test]
    fn quantize_voct_center_in_c_major_with_root_below() {
        quantize_voct_center_in_c_major_with_root(Note::C0);
    }

    #[test]
    fn quantize_voct_center_in_c_major_with_root_above() {
        quantize_voct_center_in_c_major_with_root(Note::C4);
    }

    fn quantize_voct_center_in_c_major_with_root(root: Note) {
        let voct = 2.0;
        assert_eq!(quantize_voct_center(Ionian, root, voct), (Note::C1, 1));
        assert_eq!(
            quantize_voct_center(Ionian, root, voct - 0.4 / 12.0),
            (Note::C1, 1)
        );
        assert_eq!(
            quantize_voct_center(Ionian, root, voct + 0.9 / 12.0),
            (Note::C1, 1)
        );

        let voct = 2.0 + 2.0 / 12.0;
        assert_eq!(quantize_voct_center(Ionian, root, voct), (Note::D1, 2));
        assert_eq!(
            quantize_voct_center(Ionian, root, voct - 0.9 / 12.0),
            (Note::D1, 2)
        );
        assert_eq!(
            quantize_voct_center(Ionian, root, voct + 0.9 / 12.0),
            (Note::D1, 2)
        );

        let voct = 2.0 + 4.0 / 12.0;
        assert_eq!(quantize_voct_center(Ionian, root, voct), (Note::E1, 3));
        assert_eq!(
            quantize_voct_center(Ionian, root, voct - 0.9 / 12.0),
            (Note::E1, 3)
        );
        assert_eq!(
            quantize_voct_center(Ionian, root, voct + 0.4 / 12.0),
            (Note::E1, 3)
        );

        let voct = 2.0 + 5.0 / 12.0;
        assert_eq!(quantize_voct_center(Ionian, root, voct), (Note::F1, 4));
        assert_eq!(
            quantize_voct_center(Ionian, root, voct - 0.4 / 12.0),
            (Note::F1, 4)
        );
        assert_eq!(
            quantize_voct_center(Ionian, root, voct + 0.9 / 12.0),
            (Note::F1, 4)
        );

        let voct = 2.0 + 7.0 / 12.0;
        assert_eq!(quantize_voct_center(Ionian, root, voct), (Note::G1, 5));
        assert_eq!(
            quantize_voct_center(Ionian, root, voct - 0.9 / 12.0),
            (Note::G1, 5)
        );
        assert_eq!(
            quantize_voct_center(Ionian, root, voct + 0.9 / 12.0),
            (Note::G1, 5)
        );

        let voct = 2.0 + 9.0 / 12.0;
        assert_eq!(quantize_voct_center(Ionian, root, voct), (Note::A1, 6));
        assert_eq!(
            quantize_voct_center(Ionian, root, voct - 0.9 / 12.0),
            (Note::A1, 6)
        );
        assert_eq!(
            quantize_voct_center(Ionian, root, voct + 0.9 / 12.0),
            (Note::A1, 6)
        );

        let voct = 2.0 + 11.0 / 12.0;
        assert_eq!(quantize_voct_center(Ionian, root, voct), (Note::B1, 7));
        assert_eq!(
            quantize_voct_center(Ionian, root, voct - 0.9 / 12.0),
            (Note::B1, 7)
        );
        assert_eq!(
            quantize_voct_center(Ionian, root, voct + 0.4 / 12.0),
            (Note::B1, 7)
        );

        let voct = 3.0;
        assert_eq!(quantize_voct_center(Ionian, root, voct), (Note::C2, 1));
        assert_eq!(
            quantize_voct_center(Ionian, root, voct - 0.4 / 12.0),
            (Note::C2, 1)
        );
        assert_eq!(
            quantize_voct_center(Ionian, root, voct + 0.9 / 12.0),
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
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct),
            (Note::CSharp1, 5)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::CSharp1, 5)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::CSharp1, 5)
        );

        let voct = 2.0 + 2.0 / 12.0;
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct),
            (Note::DSharp1, 6)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::DSharp1, 6)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::DSharp1, 6)
        );

        let voct = 2.0 + 4.0 / 12.0;
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::F1, 7));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::F1, 7)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::F1, 7)
        );

        let voct = 2.0 + 5.0 / 12.0;
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct),
            (Note::FSharp1, 1)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::FSharp1, 1)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::FSharp1, 1)
        );

        let voct = 2.0 + 7.0 / 12.0;
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct),
            (Note::GSharp1, 2)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::GSharp1, 2)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::GSharp1, 2)
        );

        let voct = 2.0 + 9.0 / 12.0;
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct),
            (Note::ASharp1, 3)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::ASharp1, 3)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::ASharp1, 3)
        );

        let voct = 2.0 + 11.0 / 12.0;
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::B1, 4));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::B1, 4)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
            (Note::B1, 4)
        );

        let voct = 3.0;
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct),
            (Note::CSharp2, 5)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.2 / 12.0),
            (Note::CSharp2, 5)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.2 / 12.0),
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
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::C1, 1));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.05 / 12.0),
            (Note::C1, 1)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.05 / 12.0),
            (Note::C1, 1)
        );

        let voct = 2.0 + 3.0 / 12.0;
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::D1, 2));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.05 / 12.0),
            (Note::D1, 2)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.05 / 12.0),
            (Note::D1, 2)
        );

        let voct = 2.0 + 6.0 / 12.0;
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::F1, 4));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.05 / 12.0),
            (Note::F1, 4)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.05 / 12.0),
            (Note::F1, 4)
        );

        let voct = 2.0 + 8.0 / 12.0;
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::G1, 5));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.05 / 12.0),
            (Note::G1, 5)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.05 / 12.0),
            (Note::G1, 5)
        );

        let voct = 2.0 + 10.0 / 12.0;
        assert_eq!(quantize_voct_white_keys(Ionian, root, voct), (Note::A1, 6));
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.05 / 12.0),
            (Note::A1, 6)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.05 / 12.0),
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
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct),
            (Note::CSharp1, 5)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.05 / 12.0),
            (Note::CSharp1, 5)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.05 / 12.0),
            (Note::CSharp1, 5)
        );

        let voct = 2.0 + 3.0 / 12.0;
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct),
            (Note::DSharp1, 6)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.05 / 12.0),
            (Note::DSharp1, 6)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.05 / 12.0),
            (Note::DSharp1, 6)
        );

        let voct = 2.0 + 6.0 / 12.0;
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct),
            (Note::FSharp1, 1)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.05 / 12.0),
            (Note::FSharp1, 1)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.05 / 12.0),
            (Note::FSharp1, 1)
        );

        let voct = 2.0 + 8.0 / 12.0;
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct),
            (Note::GSharp1, 2)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.05 / 12.0),
            (Note::GSharp1, 2)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.05 / 12.0),
            (Note::GSharp1, 2)
        );

        let voct = 2.0 + 10.0 / 12.0;
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct),
            (Note::ASharp1, 3)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct - 0.05 / 12.0),
            (Note::ASharp1, 3)
        );
        assert_eq!(
            quantize_voct_white_keys(Ionian, root, voct + 0.05 / 12.0),
            (Note::ASharp1, 3)
        );
    }

    #[test]
    fn quantize_voct_with_note_below_the_lowest_root() {
        let voct = 1.0 / 12.0;
        assert_eq!(
            quantize_voct_white_keys(Ionian, Note::A0, voct),
            (Note::CSharpMinus1, 3)
        );
    }

    #[test]
    fn quantize_voct_note_over_limit_and_stay_in_scale() {
        let voct = 100.0;
        assert_eq!(
            quantize_voct_white_keys(Ionian, Note::B0, voct),
            (Note::FSharp9, 5)
        );
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
