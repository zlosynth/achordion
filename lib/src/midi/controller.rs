use super::instrument::Instrument;
use super::parser::Parser;

pub use super::instrument::State;

pub struct Controller {
    instrument: Instrument,
    parser: Parser,
    previous_state: Option<State>,
}

impl Controller {
    pub fn new() -> Self {
        Self::new_from(Instrument::new(), Parser::new())
    }

    fn new_from(instrument: Instrument, parser: Parser) -> Self {
        Self {
            instrument,
            parser,
            previous_state: None,
        }
    }

    pub fn reconcile_byte(&mut self, byte: u8) -> Option<State> {
        let message = self.parser.parse_byte(byte)?;
        let state = self.instrument.reconcile(message);
        match self.previous_state {
            Some(previous_state) => {
                if state != previous_state {
                    Some(state)
                } else {
                    None
                }
            }
            None => Some(state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::note::Note;
    use super::*;

    #[test]
    fn parse_note_on() {
        let mut controller = Controller::new();

        assert_eq!(controller.reconcile_byte(0x92), None);
        assert_eq!(controller.reconcile_byte(0x45), None);
        assert_eq!(
            controller.reconcile_byte(0x7f).unwrap(),
            State {
                frequency: Note::A4.to_freq_f32(),
            }
        );
    }

    #[test]
    fn send_unimplemented_cc_after_note_on() {
        let mut controller = Controller::new();

        // Note on
        controller.reconcile_byte(0x92);
        controller.reconcile_byte(0x45);
        controller.reconcile_byte(0x7f);

        // CC
        assert_eq!(controller.reconcile_byte(0xb2), None);
        assert_eq!(controller.reconcile_byte(0x3c), None);
        assert_eq!(controller.reconcile_byte(0x18), None);

        // CC running state
        assert_eq!(controller.reconcile_byte(0x43), None);
        assert_eq!(controller.reconcile_byte(0x01), None);
    }
}
