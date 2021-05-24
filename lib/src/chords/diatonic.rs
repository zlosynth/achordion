use crate::note::Note;
use crate::scales;
use crate::scales::diatonic::Mode;

pub fn build<const D: usize>(
    scale_root: Note,
    scale_mode: Mode,
    chord_root: Note,
    degrees: [i8; D],
) -> [Option<Note>; D] {
    let mut notes = [None; D];

    for (i, degree) in degrees.iter().enumerate() {
        if *degree == 0 {
            notes[i] = None;
            continue;
        }

        notes[i] =
            scales::diatonic::lookup_degree(scale_root, scale_mode, chord_root, *degree as i32);
    }

    notes
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::scales::diatonic::Mode::*;

    #[test]
    fn build_major_triad_on_the_first_degree() {
        let notes = build(Note::C3, Ionian, Note::C4, [1, 3, 5]);

        assert_eq!(notes[0], Some(Note::C4));
        assert_eq!(notes[1], Some(Note::E4));
        assert_eq!(notes[2], Some(Note::G4));
    }

    #[test]
    fn build_minor_triad_on_the_second_degree() {
        let notes = build(Note::C3, Ionian, Note::D4, [1, 3, 5]);

        assert_eq!(notes[0], Some(Note::D4));
        assert_eq!(notes[1], Some(Note::F4));
        assert_eq!(notes[2], Some(Note::A4));
    }

    #[test]
    fn build_chord_that_overflows_note_range() {
        let notes = build(Note::C9, Ionian, Note::G9, [1, 3, 5]);

        assert_eq!(notes[0], Some(Note::G9));
        assert_eq!(notes[1], None);
        assert_eq!(notes[2], None);
    }

    #[test]
    fn build_chord_with_disabled_degree() {
        let notes = build(Note::C3, Ionian, Note::C4, [1, 0, 5]);

        assert_eq!(notes[0], Some(Note::C4));
        assert_eq!(notes[1], None);
        assert_eq!(notes[2], Some(Note::G4));
    }

    #[test]
    fn build_chord_over_multiple_octaves() {
        let notes = build(Note::C3, Ionian, Note::C4, [1, 7 + 3, 2 * 7 + 5]);

        assert_eq!(notes[0], Some(Note::C4));
        assert_eq!(notes[1], Some(Note::E5));
        assert_eq!(notes[2], Some(Note::G6));
    }

    #[test]
    fn build_chord_with_negative_degrees() {
        let notes = build(Note::C3, Ionian, Note::C4, [1, -1, -2]);

        assert_eq!(notes[0], Some(Note::C4));
        assert_eq!(notes[1], Some(Note::C4));
        assert_eq!(notes[2], Some(Note::B3));

        let notes = build(Note::C3, Ionian, Note::C4, [-7, -8, -9]);

        assert_eq!(notes[0], Some(Note::D3));
        assert_eq!(notes[1], Some(Note::C3));
        assert_eq!(notes[2], Some(Note::B2));
    }
}
