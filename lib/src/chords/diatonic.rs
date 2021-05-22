use crate::note::Note;
use crate::scales::diatonic::{Mode, SEMITONES};

pub fn build(
    mode: Mode,
    scale_root: Note,
    chord_root: Note,
    degrees: [i8; 3],
) -> [Option<Note>; 3] {
    let scale_root_id = (scale_root.to_midi_id() % 12) as i16;
    let mut chord_root_id = (chord_root.to_midi_id() % 12) as i16;

    if scale_root_id > chord_root_id {
        chord_root_id += 12;
    }

    let semitones = SEMITONES[mode as usize];

    let progression_start = semitones
        .iter()
        .enumerate()
        .find(|(_, x)| **x as i16 == chord_root_id - scale_root_id)
        .map(|(i, _)| i)
        .unwrap_or(0);

    let mut notes = [None; 3];

    for (i, degree) in degrees.iter().enumerate() {
        assert!(
            (0..12).contains(degree),
            "Only basic intervals are implemented"
        );

        let target = progression_start + *degree as usize - 1;

        let x = if target > 6 {
            semitones[(progression_start + *degree as usize - 1) % 7] - semitones[progression_start]
                + 12
        } else {
            semitones[(progression_start + *degree as usize - 1)] - semitones[progression_start]
        };

        notes[i] = Note::try_from_u8(chord_root.to_midi_id() + x as u8);
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
}
