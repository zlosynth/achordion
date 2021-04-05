// Kudos to https://github.com/mendelt/embedded-midi

use super::channel::Channel;
use super::message::Message;
use super::note::Note;

enum ParserState {
    Idle,
    NoteOnRecieved(Channel),
    NoteOnNoteRecieved(Channel, Note),
    NoteOffRecieved(Channel),
    NoteOffNoteRecieved(Channel, Note),
}

pub struct Parser {
    state: ParserState,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            state: ParserState::Idle,
        }
    }

    pub fn parse_byte(&mut self, byte: u8) -> Option<Message> {
        if is_status_byte(byte) {
            let (message, channel) = split_message_and_channel(byte);

            match message {
                0x80 => {
                    self.state = ParserState::NoteOffRecieved(channel);
                    None
                }
                0x90 => {
                    self.state = ParserState::NoteOnRecieved(channel);
                    None
                }
                _ => {
                    self.state = ParserState::Idle;
                    None
                }
            }
        } else {
            match self.state {
                ParserState::NoteOffRecieved(channel) => {
                    self.state = ParserState::NoteOffNoteRecieved(channel, byte.into());
                    None
                }
                ParserState::NoteOffNoteRecieved(channel, note) => {
                    self.state = ParserState::NoteOffRecieved(channel);
                    Some(Message::NoteOff(channel, note))
                }
                ParserState::NoteOnRecieved(channel) => {
                    self.state = ParserState::NoteOnNoteRecieved(channel, byte.into());
                    None
                }
                ParserState::NoteOnNoteRecieved(channel, note) => {
                    self.state = ParserState::NoteOnRecieved(channel);
                    Some(Message::NoteOn(channel, note, byte.into()))
                }
                _ => None,
            }
        }
    }
}

fn is_status_byte(byte: u8) -> bool {
    byte & 0x80 == 0x80
}

fn split_message_and_channel(byte: u8) -> (u8, Channel) {
    (byte & 0xf0u8, (byte & 0x0fu8).into())
}

#[cfg(test)]
mod tests {
    use super::super::channel::Channel::*;
    use super::super::message::Message;
    use super::super::note::Note;
    use super::*;

    #[test]
    fn parse_status_byte() {
        assert!(is_status_byte(0x80u8));
        assert!(is_status_byte(0x94u8));
        assert!(!is_status_byte(0x00u8));
        assert!(!is_status_byte(0x78u8));
    }

    #[test]
    fn should_split_message_and_channel() {
        let (message, channel) = split_message_and_channel(0x91u8);
        assert_eq!(message, 0x90u8);
        assert_eq!(channel, Channel2);
    }

    #[test]
    fn parse_note_on() {
        let mut parser = Parser::new();
        assert!(parser.parse_byte(0x92).is_none());
        assert!(parser.parse_byte(0x45).is_none());
        assert_eq!(
            parser.parse_byte(0x7f).unwrap(),
            Message::NoteOn(Channel3, Note::A4, 127.into())
        );
    }

    #[test]
    fn split_stream_of_note_on() {
        let mut parser = Parser::new();
        assert!(parser.parse_byte(0x92).is_none());
        assert!(parser.parse_byte(0x45).is_none());
        assert_eq!(
            parser.parse_byte(0x7f).unwrap(),
            Message::NoteOn(Channel3, Note::A4, 127.into())
        );
        assert!(parser.parse_byte(0x47).is_none());
        assert_eq!(
            parser.parse_byte(0x7f).unwrap(),
            Message::NoteOn(Channel3, Note::B4, 127.into())
        );
    }

    #[test]
    fn parse_note_off() {
        let mut parser = Parser::new();
        assert!(parser.parse_byte(0x82).is_none());
        assert!(parser.parse_byte(0x45).is_none());
        assert_eq!(
            parser.parse_byte(0x00).unwrap(),
            Message::NoteOff(Channel3, Note::A4)
        );
    }

    #[test]
    fn incomplete_message() {
        let mut parser = Parser::new();
        assert!(parser.parse_byte(0x92).is_none()); // note on
        assert!(parser.parse_byte(0x82).is_none()); // note off
        assert!(parser.parse_byte(0x45).is_none());
        assert_eq!(
            parser.parse_byte(0x00).unwrap(),
            Message::NoteOff(Channel3, Note::A4)
        );
    }
}
