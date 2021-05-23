use crate::note::Note;
use crate::scales::diatonic::{Mode, SEMITONES};

pub fn build(
    mode: Mode,
    scale_root: Note,
    chord_root: Note,
    degrees: [i8; 3],
) -> [Option<Note>; 3] {
    let scale_semitones = SEMITONES[mode as usize];

    let scale_index_of_chord = {
        let scale_root_to_chord_distance = {
            let scale_root_id = (scale_root.to_midi_id() % 12) as i16;
            let chord_root_id = (chord_root.to_midi_id() % 12) as i16;
            (scale_root_id - chord_root_id).abs()
        };

        scale_semitones
            .iter()
            .enumerate()
            .find(|(_, x)| **x as i16 == scale_root_to_chord_distance)
            .map(|(i, _)| i)
            .unwrap_or(0)
    };

    let mut notes = [None; 3];

    for (i, degree) in degrees.iter().enumerate() {
        assert!(*degree >= 0, "Only positive intervals are implemented");

        if *degree == 0 {
            notes[i] = None;
            continue;
        }

        let semitones_distance = {
            let scale_index_of_degree = scale_index_of_chord + *degree as usize - 1;
            let semitones_for_octaves = (scale_index_of_degree as i8 / 7) * 12;
            let semitones_for_interval =
                scale_semitones[scale_index_of_degree % 7] - scale_semitones[scale_index_of_chord];
            semitones_for_interval + semitones_for_octaves
        };

        notes[i] = Note::try_from_u8(chord_root.to_midi_id() + semitones_distance as u8);
    }

    notes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scales::diatonic::Mode::*;

    #[test]
    fn build_major_triad_on_the_first_degree() {
        let notes = build(Ionian, Note::C3, Note::C4, [1, 3, 5]);

        assert_eq!(notes[0], Some(Note::C4));
        assert_eq!(notes[1], Some(Note::E4));
        assert_eq!(notes[2], Some(Note::G4));
    }

    #[test]
    fn build_minor_triad_on_the_second_degree() {
        let notes = build(Ionian, Note::C3, Note::D4, [1, 3, 5]);

        assert_eq!(notes[0], Some(Note::D4));
        assert_eq!(notes[1], Some(Note::F4));
        assert_eq!(notes[2], Some(Note::A4));
    }

    #[test]
    fn build_chord_that_overflows_note_range() {
        let notes = build(Ionian, Note::C9, Note::G9, [1, 3, 5]);

        assert_eq!(notes[0], Some(Note::G9));
        assert_eq!(notes[1], None);
        assert_eq!(notes[2], None);
    }

    #[test]
    fn build_chord_with_disabled_degree() {
        let notes = build(Ionian, Note::C3, Note::C4, [1, 0, 5]);

        assert_eq!(notes[0], Some(Note::C4));
        assert_eq!(notes[1], None);
        assert_eq!(notes[2], Some(Note::G4));
    }

    #[test]
    fn build_chord_over_multiple_octaves() {
        let notes = build(Ionian, Note::C3, Note::C4, [1, 10, 19]);

        assert_eq!(notes[0], Some(Note::C4));
        assert_eq!(notes[1], Some(Note::E5));
        assert_eq!(notes[2], Some(Note::G6));
    }
}
