use crate::midi::note::Note;

pub fn quantize(voct: f32) -> Note {
    let offset_voct = voct + (1.0 / 24.0);
    let position = offset_voct / (1.0 / 12.0);
    let id = position as u8;

    if id > Note::HIGHEST_NOTE.to_midi_id() {
        Note::HIGHEST_NOTE
    } else {
        Note::from(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantize_on_spot_voct_to_note() {
        let voct = 0.0;
        assert_eq!(quantize(voct), Note::CMinus1);

        let voct = 3.0;
        assert_eq!(quantize(voct), Note::C2);

        let voct = 3.0 + 1.0 / 12.0;
        assert_eq!(quantize(voct), Note::CSharp2);

        let voct = 4.0;
        assert_eq!(quantize(voct), Note::C3);
    }

    #[test]
    fn quantize_note_over_limit() {
        let voct = 100.0;
        assert_eq!(quantize(voct), Note::HIGHEST_NOTE);
    }

    #[test]
    fn quantize_voct_above_to_note() {
        let voct = 3.0 + 0.4 / 12.0;
        assert_eq!(quantize(voct), Note::C2);

        let voct = 3.0 + 1.4 / 12.0;
        assert_eq!(quantize(voct), Note::CSharp2);
    }

    #[test]
    fn quantize_voct_below_to_note() {
        let voct = 3.0 - 0.4 / 12.0;
        assert_eq!(quantize(voct), Note::C2);

        let voct = 3.0 - 1.4 / 12.0;
        assert_eq!(quantize(voct), Note::B1);
    }
}
