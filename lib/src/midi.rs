use core::convert::TryFrom;

use heapless::consts::*;
use heapless::Vec;
#[allow(unused_imports)]
use micromath::F32Ext;
use usbd_midi::data::midi::message::Message as UsbdMessage;

const MAX_NOTES: usize = 8;

pub struct Instrument {
    notes: NoteBuffer,
}

impl Instrument {
    pub fn new() -> Self {
        Self {
            notes: NoteBuffer::new(),
        }
    }

    pub fn reconcile(&mut self, message: Message) -> State {
        match message {
            Message::NoteOn(note) => {
                self.notes.remove(note);
                self.notes.push(note);
                State {
                    frequency: note.to_freq_f32(),
                }
            }
            Message::NoteOff(note) => {
                self.notes.remove(note);
                match self.notes.last() {
                    Some(note) => State {
                        frequency: note.to_freq_f32(),
                    },
                    None => State { frequency: 0.0 },
                }
            }
        }
    }
}

struct NoteBuffer {
    buffer: Vec<Note, U8>,
}

impl NoteBuffer {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    fn push(&mut self, note: Note) {
        if self.buffer.len() == MAX_NOTES {
            self.buffer = self.buffer.iter().skip(1).copied().collect();
        }
        self.buffer.push(note).unwrap();
    }

    fn remove(&mut self, note: Note) {
        self.buffer = self
            .buffer
            .iter()
            .filter(|n| **n != note)
            .copied()
            .collect();
    }

    fn last(&self) -> Option<Note> {
        if self.buffer.len() == 0 {
            None
        } else {
            Some(self.buffer[self.buffer.len() - 1])
        }
    }
}

pub struct State {
    pub frequency: f32,
}

pub enum Message {
    NoteOn(Note),
    NoteOff(Note),
}

impl TryFrom<UsbdMessage> for Message {
    type Error = &'static str;

    fn try_from(message: UsbdMessage) -> Result<Self, Self::Error> {
        match message {
            UsbdMessage::NoteOn(_, note, _) => Ok(Message::NoteOn(Note::from_u8(note.into()))),
            UsbdMessage::NoteOff(_, note, _) => Ok(Message::NoteOff(Note::from_u8(note.into()))),
            _ => Err("Conversion not implemented"),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Note {
    CMinus1 = 0,
    DbMinus1 = 1,
    DMinus1 = 2,
    EbMinus1 = 3,
    EMinus1 = 4,
    FMinus1 = 5,
    GbMinus1 = 6,
    GMinus1 = 7,
    AbMinus1 = 8,
    AMinus1 = 9,
    BbMinus1 = 10,
    BMinus1 = 11,
    C0 = 12,
    Db0 = 13,
    D0 = 14,
    Eb0 = 15,
    E0 = 16,
    F0 = 17,
    Gb0 = 18,
    G0 = 19,
    Ab0 = 20,
    A0 = 21,
    Bb0 = 22,
    B0 = 23,
    C1 = 24,
    Db1 = 25,
    D1 = 26,
    Eb1 = 27,
    E1 = 28,
    F1 = 29,
    Gb1 = 30,
    G1 = 31,
    Ab1 = 32,
    A1 = 33,
    Bb1 = 34,
    B1 = 35,
    C2 = 36,
    Db2 = 37,
    D2 = 38,
    Eb2 = 39,
    E2 = 40,
    F2 = 41,
    Gb2 = 42,
    G2 = 43,
    Ab2 = 44,
    A2 = 45,
    Bb2 = 46,
    B2 = 47,
    C3 = 48,
    Db3 = 49,
    D3 = 50,
    Eb3 = 51,
    E3 = 52,
    F3 = 53,
    Gb3 = 54,
    G3 = 55,
    Ab3 = 56,
    A3 = 57,
    Bb3 = 58,
    B3 = 59,
    C4 = 60,
    Db4 = 61,
    D4 = 62,
    Eb4 = 63,
    E4 = 64,
    F4 = 65,
    Gb4 = 66,
    G4 = 67,
    Ab4 = 68,
    A4 = 69,
    Bb4 = 70,
    B4 = 71,
    C5 = 72,
    Db5 = 73,
    D5 = 74,
    Eb5 = 75,
    E5 = 76,
    F5 = 77,
    Gb5 = 78,
    G5 = 79,
    Ab5 = 80,
    A5 = 81,
    Bb5 = 82,
    B5 = 83,
    C6 = 84,
    Db6 = 85,
    D6 = 86,
    Eb6 = 87,
    E6 = 88,
    F6 = 89,
    Gb6 = 90,
    G6 = 91,
    Ab6 = 92,
    A6 = 93,
    Bb6 = 94,
    B6 = 95,
    C7 = 96,
    Db7 = 97,
    D7 = 98,
    Eb7 = 99,
    E7 = 100,
    F7 = 101,
    Gb7 = 102,
    G7 = 103,
    Ab7 = 104,
    A7 = 105,
    Bb7 = 106,
    B7 = 107,
    C8 = 108,
    Db8 = 109,
    D8 = 110,
    Eb8 = 111,
    E8 = 112,
    F8 = 113,
    Gb8 = 114,
    G8 = 115,
    Ab8 = 116,
    A8 = 117,
    Bb8 = 118,
    B8 = 119,
    C9 = 120,
    Db9 = 121,
    D9 = 122,
    Eb9 = 123,
    E9 = 124,
    F9 = 125,
    Gb9 = 126,
    G9 = 127,
}

#[allow(non_upper_case_globals)]
impl Note {
    pub const CSharpMinus1: Note = Note::DbMinus1;
    pub const DSharpMinus1: Note = Note::EbMinus1;
    pub const FSharpMinus1: Note = Note::GbMinus1;
    pub const GSharpMinus1: Note = Note::AbMinus1;
    pub const ASharpMinus1: Note = Note::BbMinus1;
    pub const CSharp0: Note = Note::Db0;
    pub const DSharp0: Note = Note::Eb0;
    pub const FSharp0: Note = Note::Gb0;
    pub const GSharp0: Note = Note::Ab0;
    pub const ASharp0: Note = Note::Bb0;
    pub const CSharp1: Note = Note::Db1;
    pub const DSharp1: Note = Note::Eb1;
    pub const FSharp1: Note = Note::Gb1;
    pub const GSharp1: Note = Note::Ab1;
    pub const ASharp1: Note = Note::Bb1;
    pub const CSharp2: Note = Note::Db2;
    pub const DSharp2: Note = Note::Eb2;
    pub const FSharp2: Note = Note::Gb2;
    pub const GSharp2: Note = Note::Ab2;
    pub const ASharp2: Note = Note::Bb2;
    pub const CSharp3: Note = Note::Db3;
    pub const DSharp3: Note = Note::Eb3;
    pub const FSharp3: Note = Note::Gb3;
    pub const GSharp3: Note = Note::Ab3;
    pub const ASharp3: Note = Note::Bb3;
    pub const CSharp4: Note = Note::Db4;
    pub const DSharp4: Note = Note::Eb4;
    pub const FSharp4: Note = Note::Gb4;
    pub const GSharp4: Note = Note::Ab4;
    pub const ASharp4: Note = Note::Bb4;
    pub const CSharp5: Note = Note::Db5;
    pub const DSharp5: Note = Note::Eb5;
    pub const FSharp5: Note = Note::Gb5;
    pub const GSharp5: Note = Note::Ab5;
    pub const ASharp5: Note = Note::Bb5;
    pub const CSharp6: Note = Note::Db6;
    pub const DSharp6: Note = Note::Eb6;
    pub const FSharp6: Note = Note::Gb6;
    pub const GSharp6: Note = Note::Ab6;
    pub const ASharp6: Note = Note::Bb6;
    pub const CSharp7: Note = Note::Db7;
    pub const DSharp7: Note = Note::Eb7;
    pub const FSharp7: Note = Note::Gb7;
    pub const GSharp7: Note = Note::Ab7;
    pub const ASharp7: Note = Note::Bb7;
    pub const CSharp8: Note = Note::Db8;
    pub const DSharp8: Note = Note::Eb8;
    pub const FSharp8: Note = Note::Gb8;
    pub const GSharp8: Note = Note::Ab8;
    pub const ASharp8: Note = Note::Bb8;
    pub const CSharp9: Note = Note::Db9;
    pub const DSharp9: Note = Note::Eb9;
    pub const FSharp9: Note = Note::Gb9;

    pub const HIGHEST_NOTE: Note = Note::G9;

    #[inline(always)]
    pub fn from_u8(note: u8) -> Note {
        assert!(note <= Note::HIGHEST_NOTE as u8);
        unsafe { core::mem::transmute(note) }
    }

    // TODO: Precalculate
    #[inline(always)]
    pub fn to_freq_f32(self) -> f32 {
        let exp = (f32::from(self as u8) + 36.376_316) / 12.0;
        2f32.powf(exp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_buffer() {
        let _buffer = NoteBuffer::new();
    }

    #[test]
    fn push_to_buffer() {
        let mut buffer = NoteBuffer::new();
        buffer.push(Note::A4);
        assert_eq!(buffer.last(), Some(Note::A4));
    }

    #[test]
    fn push_to_full_buffer() {
        let mut buffer = NoteBuffer::new();

        buffer.push(Note::A4);
        buffer.push(Note::B4);
        buffer.push(Note::C4);
        buffer.push(Note::D4);
        buffer.push(Note::E4);
        buffer.push(Note::F4);
        buffer.push(Note::G4);
        buffer.push(Note::A5);
        // This one is over limit and should cause the first one to be dropped
        buffer.push(Note::B5);

        assert_eq!(buffer.last(), Some(Note::B5));

        buffer.remove(Note::B5);
        assert_eq!(buffer.last(), Some(Note::A5));

        buffer.remove(Note::A5);
        assert_eq!(buffer.last(), Some(Note::G4));

        buffer.remove(Note::G4);
        assert_eq!(buffer.last(), Some(Note::F4));

        buffer.remove(Note::F4);
        assert_eq!(buffer.last(), Some(Note::E4));

        buffer.remove(Note::E4);
        assert_eq!(buffer.last(), Some(Note::D4));

        buffer.remove(Note::D4);
        assert_eq!(buffer.last(), Some(Note::C4));

        buffer.remove(Note::C4);
        assert_eq!(buffer.last(), Some(Note::B4));

        buffer.remove(Note::B4);
        assert_eq!(buffer.last(), None);

        // A4 was dropped implicitly
    }

    #[test]
    fn remove_from_buffer() {
        let mut buffer = NoteBuffer::new();
        buffer.push(Note::A4);

        buffer.remove(Note::A4);
        assert_eq!(buffer.last(), None);
    }

    #[test]
    fn remove_from_empty_buffer() {
        let mut buffer = NoteBuffer::new();
        buffer.remove(Note::A4);
    }

    #[test]
    fn last_empty_buffer_item() {
        let buffer = NoteBuffer::new();
        assert_eq!(buffer.last(), None);
    }

    #[test]
    fn initialize_instrument() {
        let _instrument = Instrument::new();
    }

    #[test]
    fn reconcile_note_on_message() {
        let mut instrument = Instrument::new();

        let state = instrument.reconcile(Message::NoteOn(Note::A4));

        assert_relative_eq!(state.frequency, 440.0);
    }

    #[test]
    fn reconcile_note_off_message() {
        let mut instrument = Instrument::new();

        instrument.reconcile(Message::NoteOn(Note::A4));
        let state = instrument.reconcile(Message::NoteOff(Note::A4));

        assert_relative_eq!(state.frequency, 0.0);
    }

    #[test]
    fn reconcile_multiple_note_on_messages() {
        let mut instrument = Instrument::new();

        let state = instrument.reconcile(Message::NoteOn(Note::A3));
        assert_relative_eq!(state.frequency, Note::A3.to_freq_f32());

        let state = instrument.reconcile(Message::NoteOn(Note::A4));
        assert_relative_eq!(state.frequency, Note::A4.to_freq_f32());

        let state = instrument.reconcile(Message::NoteOff(Note::A4));
        assert_relative_eq!(state.frequency, Note::A3.to_freq_f32());

        let state = instrument.reconcile(Message::NoteOff(Note::A3));
        assert_relative_eq!(state.frequency, 0.0);
    }

    #[test]
    fn reconcile_over_limit_note_on_messages() {
        let mut instrument = Instrument::new();

        let notes = [
            Note::A1,
            Note::B1,
            Note::C1,
            Note::D1,
            Note::E1,
            Note::F1,
            Note::G1,
            Note::A2,
            // This one is over limit and should cause the first one to be dropped
            Note::B2,
        ];

        for note in notes.iter() {
            instrument.reconcile(Message::NoteOn(*note));
        }

        for i in 0..MAX_NOTES - 1 {
            let i = notes.len() - 1 - i;
            let state = instrument.reconcile(Message::NoteOff(notes[i]));
            assert_relative_eq!(state.frequency, notes[i - 1].to_freq_f32());
        }

        let state = instrument.reconcile(Message::NoteOff(notes[1]));
        assert_relative_eq!(state.frequency, 0.0);
    }

    #[test]
    fn from_usbd_note_on() {
        use core::convert::TryFrom;
        use usbd_midi::data::byte::u7::U7 as UsbdU7;
        use usbd_midi::data::midi::channel::Channel as UsbdChannel;
        use usbd_midi::data::midi::notes::Note as UsbdNote;

        let message = Message::try_from(UsbdMessage::NoteOn(
            UsbdChannel::Channel1,
            UsbdNote::A4,
            UsbdU7::try_from(127).ok().unwrap(),
        ))
        .unwrap();

        match message {
            Message::NoteOn(note) => {
                assert_eq!(note, Note::A4);
            }
            _ => panic!("Invalid variant"),
        }
    }

    #[test]
    fn from_usbd_note_off() {
        use core::convert::TryFrom;
        use usbd_midi::data::byte::u7::U7 as UsbdU7;
        use usbd_midi::data::midi::channel::Channel as UsbdChannel;
        use usbd_midi::data::midi::notes::Note as UsbdNote;

        let message = Message::try_from(UsbdMessage::NoteOff(
            UsbdChannel::Channel1,
            UsbdNote::A4,
            UsbdU7::try_from(0).ok().unwrap(),
        ))
        .unwrap();

        match message {
            Message::NoteOff(note) => {
                assert_eq!(note, Note::A4);
            }
            _ => panic!("Invalid variant"),
        }
    }
}
