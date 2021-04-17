use crate::midi::note::Note;

const SEMITONES: [i8; 7] = [0, 2, 4, 5, 7, 9, 11];

pub fn build(scale_root: Note, chord_root: Note, degrees: [i8; 3]) -> [Option<Note>; 3] {
    let scale_root_id = (scale_root.to_midi_id() % 12) as i16;
    let mut chord_root_id = (chord_root.to_midi_id() % 12) as i16;

    if scale_root_id > chord_root_id {
        chord_root_id += 12;
    }

    let progression_start = match chord_root_id - scale_root_id {
        0 => 0,
        2 => 1,
        4 => 2,
        5 => 3,
        7 => 4,
        9 => 5,
        11 => 6,
        _ => 0,
    };

    let mut notes = [None; 3];

    for (i, degree) in degrees.iter().enumerate() {
        assert!(
            (0..12).contains(degree),
            "Only basic intervals are implemented"
        );

        let target = progression_start + *degree as usize - 1;

        let x = if target > 6 {
            SEMITONES[(progression_start + *degree as usize - 1) % 7] - SEMITONES[progression_start]
                + 12
        } else {
            SEMITONES[(progression_start + *degree as usize - 1)] - SEMITONES[progression_start]
        };

        notes[i] = Some(Note::from_u8(chord_root.to_midi_id() + x as u8));
    }

    notes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_major_triad_on_the_first_degree() {
        let notes = build(Note::C3, Note::C4, [1, 3, 5]);

        assert_eq!(notes[0], Some(Note::C4));
        assert_eq!(notes[1], Some(Note::E4));
        assert_eq!(notes[2], Some(Note::G4));
    }

    #[test]
    fn build_minor_triad_on_the_second_degree() {
        let notes = build(Note::C3, Note::D4, [1, 3, 5]);

        assert_eq!(notes[0], Some(Note::D4));
        assert_eq!(notes[1], Some(Note::F4));
        assert_eq!(notes[2], Some(Note::A4));
    }
}
