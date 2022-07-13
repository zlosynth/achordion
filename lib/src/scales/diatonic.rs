use core::cmp::Ordering;

use crate::note::Note;

#[repr(u8)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Mode {
    Ionian = 0,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
    HarmonicMinor,
}

pub use Mode::*;

pub const SEMITONES: [[i8; 7]; 8] = [
    [0, 2, 4, 5, 7, 9, 11],
    [0, 2, 3, 5, 7, 9, 10],
    [0, 1, 3, 5, 7, 8, 10],
    [0, 2, 4, 6, 7, 9, 11],
    [0, 2, 4, 5, 7, 9, 10],
    [0, 2, 3, 5, 7, 8, 10],
    [0, 1, 3, 5, 6, 8, 10],
    [0, 2, 3, 5, 7, 8, 11],
];

// Scale arithmetics. Find a note from the given scale, defined by its `root`
// and `mode`, that forms and `interval` from given `note`.
pub fn lookup_degree(root: Note, mode: Mode, note: Note, interval: i32) -> Option<Note> {
    let semitones = SEMITONES[mode as usize];

    let distance = match interval.cmp(&0) {
        Ordering::Greater => interval - 1,
        Ordering::Less => interval + 1,
        Ordering::Equal => return None,
    };

    let note_index = find_index_in_scale(root, mode, note);

    let octave_distance = if distance < 0 {
        // Start an octave lower, suboctave distance is always adding to right
        ((distance + 1) / 7 - 1) * 12
    } else {
        (distance / 7) * 12
    };

    let suboctave_distance = {
        let delta = (semitones[(note_index as i32 + distance).rem_euclid(7) as usize]
            - semitones[note_index as usize]) as i32;
        if delta < 0 {
            // Treat the scale as circular
            delta + 12
        } else {
            delta
        }
    };

    let total_semitone_distance = octave_distance + suboctave_distance;

    Note::try_from_i16(note.to_midi_id() as i16 + total_semitone_distance as i16)
}

fn find_index_in_scale(root: Note, mode: Mode, note: Note) -> usize {
    let semitones = SEMITONES[mode as usize];

    let scale_root_to_note_distance = {
        let scale_id = (root.to_midi_id() % 12) as i32;
        let note_id = (note.to_midi_id() % 12) as i32;
        (scale_id - note_id).abs()
    };

    semitones
        .iter()
        .enumerate()
        .find(|(_, x)| **x as i32 == scale_root_to_note_distance)
        .map(|(i, _)| i)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_root_identical_degree() {
        let note = lookup_degree(Note::C1, Ionian, Note::C3, 1);
        assert_eq!(note.unwrap(), Note::C3);
    }

    #[test]
    fn lookup_root_identical_degree_octave_above() {
        let note = lookup_degree(Note::C1, Ionian, Note::C3, 8);
        assert_eq!(note.unwrap(), Note::C4);
    }

    #[test]
    fn lookup_root_identical_degree_octave_below() {
        let note = lookup_degree(Note::C1, Ionian, Note::C3, -8);
        assert_eq!(note.unwrap(), Note::C2);
    }

    #[test]
    fn lookup_degree_near_above_root() {
        let note = lookup_degree(Note::C1, Ionian, Note::C3, 2);
        assert_eq!(note.unwrap(), Note::D3);

        let note = lookup_degree(Note::C1, Ionian, Note::C3, 5);
        assert_eq!(note.unwrap(), Note::G3);
    }

    #[test]
    fn lookup_degree_near_above_root_with_root_above() {
        let note = lookup_degree(Note::C5, Ionian, Note::C3, 2);
        assert_eq!(note.unwrap(), Note::D3);

        let note = lookup_degree(Note::C5, Ionian, Note::C3, 5);
        assert_eq!(note.unwrap(), Note::G3);
    }

    #[test]
    fn lookup_degree_far_above_root() {
        let note = lookup_degree(Note::C1, Ionian, Note::C3, 7 + 2);
        assert_eq!(note.unwrap(), Note::D4);

        let note = lookup_degree(Note::C1, Ionian, Note::C3, 2 * 7 + 5);
        assert_eq!(note.unwrap(), Note::G5);
    }

    #[test]
    fn lookup_degree_near_below_root() {
        let note = lookup_degree(Note::C1, Ionian, Note::C3, -2);
        assert_eq!(note.unwrap(), Note::B2);

        let note = lookup_degree(Note::C1, Ionian, Note::C3, -4);
        assert_eq!(note.unwrap(), Note::G2);
    }

    #[test]
    fn lookup_degree_far_below_root() {
        let note = lookup_degree(Note::C1, Ionian, Note::C3, -7 - 2);
        assert_eq!(note.unwrap(), Note::B1);

        let note = lookup_degree(Note::C1, Ionian, Note::C3, 2 * -7 - 4);
        assert_eq!(note.unwrap(), Note::G0);
    }

    #[test]
    fn lookup_non_root_identical_degree() {
        let note = lookup_degree(Note::C1, Ionian, Note::D3, 1);
        assert_eq!(note.unwrap(), Note::D3);
    }

    #[test]
    fn lookup_non_root_identical_degree_octave_above() {
        let note = lookup_degree(Note::C1, Ionian, Note::D3, 8);
        assert_eq!(note.unwrap(), Note::D4);
    }

    #[test]
    fn lookup_non_root_identical_degree_octave_below() {
        let note = lookup_degree(Note::C1, Ionian, Note::D3, -8);
        assert_eq!(note.unwrap(), Note::D2);
    }

    #[test]
    fn lookup_degree_near_above_non_root() {
        let note = lookup_degree(Note::C1, Ionian, Note::D3, 2);
        assert_eq!(note.unwrap(), Note::E3);

        let note = lookup_degree(Note::C1, Ionian, Note::D3, 7);
        assert_eq!(note.unwrap(), Note::C4);
    }

    #[test]
    fn lookup_degree_far_above_non_root() {
        let note = lookup_degree(Note::C1, Ionian, Note::D3, 7 + 2);
        assert_eq!(note.unwrap(), Note::E4);

        let note = lookup_degree(Note::C1, Ionian, Note::D3, 7 + 7);
        assert_eq!(note.unwrap(), Note::C5);
    }

    #[test]
    fn lookup_degree_near_below_non_root() {
        let note = lookup_degree(Note::C3, Ionian, Note::D3, -2);
        assert_eq!(note.unwrap(), Note::C3);

        let note = lookup_degree(Note::C3, Ionian, Note::D3, -7);
        assert_eq!(note.unwrap(), Note::E2);
    }

    #[test]
    fn lookup_degree_far_below_non_root() {
        let note = lookup_degree(Note::C3, Ionian, Note::D3, -7 - 2);
        assert_eq!(note.unwrap(), Note::C2);

        let note = lookup_degree(Note::C3, Ionian, Note::D3, -7 - 7);
        assert_eq!(note.unwrap(), Note::E1);
    }

    #[test]
    fn find_index_of_arbitrary_note_in_scale_in_octave() {
        let index = find_index_in_scale(Note::C3, Ionian, Note::F3);
        assert_eq!(index, 3);
    }

    #[test]
    fn find_index_of_arbitrary_note_in_scale_higher_octave() {
        let index = find_index_in_scale(Note::C3, Ionian, Note::A4);
        assert_eq!(index, 5);
    }

    #[test]
    fn find_index_of_arbitrary_note_in_scale_lower_octave() {
        let index = find_index_in_scale(Note::C3, Ionian, Note::A2);
        assert_eq!(index, 5);
    }

    #[test]
    fn find_index_of_unison_in_scale_in_octave() {
        let index = find_index_in_scale(Note::C3, Ionian, Note::C3);
        assert_eq!(index, 0);
    }

    #[test]
    fn find_index_of_unison_in_scale_higher_octave() {
        let index = find_index_in_scale(Note::C3, Ionian, Note::C4);
        assert_eq!(index, 0);
    }

    #[test]
    fn find_index_of_unison_in_scale_lower_octave() {
        let index = find_index_in_scale(Note::C3, Ionian, Note::C2);
        assert_eq!(index, 0);
    }

    #[test]
    fn find_index_of_out_of_scale_note() {
        let index = find_index_in_scale(Note::C3, Ionian, Note::FSharp3);
        assert_eq!(index, 0);
    }
}
