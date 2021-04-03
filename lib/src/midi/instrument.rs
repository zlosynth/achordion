use heapless::consts::*;
use heapless::Vec;

use super::message::Message;
use super::note::Note;

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
            Message::NoteOn(_, note, _) => {
                self.notes.remove(note);
                self.notes.push(note);
                State {
                    frequency: note.to_freq_f32(),
                }
            }
            Message::NoteOff(_, note) => {
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

#[cfg(test)]
mod tests {
    use super::super::channel::Channel::*;
    use super::super::velocity::Velocity;
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

        let state = instrument.reconcile(Message::NoteOn(Channel1, Note::A4, Velocity::MAX));

        assert_relative_eq!(state.frequency, 440.0);
    }

    #[test]
    fn reconcile_note_off_message() {
        let mut instrument = Instrument::new();

        instrument.reconcile(Message::NoteOn(Channel1, Note::A4, Velocity::MAX));
        let state = instrument.reconcile(Message::NoteOff(Channel1, Note::A4));

        assert_relative_eq!(state.frequency, 0.0);
    }

    #[test]
    fn reconcile_multiple_note_on_messages() {
        let mut instrument = Instrument::new();

        let state = instrument.reconcile(Message::NoteOn(Channel1, Note::A3, Velocity::MAX));
        assert_relative_eq!(state.frequency, Note::A3.to_freq_f32());

        let state = instrument.reconcile(Message::NoteOn(Channel1, Note::A4, Velocity::MAX));
        assert_relative_eq!(state.frequency, Note::A4.to_freq_f32());

        let state = instrument.reconcile(Message::NoteOff(Channel1, Note::A4));
        assert_relative_eq!(state.frequency, Note::A3.to_freq_f32());

        let state = instrument.reconcile(Message::NoteOff(Channel1, Note::A3));
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
            instrument.reconcile(Message::NoteOn(Channel1, *note, Velocity::MAX));
        }

        for i in 0..MAX_NOTES - 1 {
            let i = notes.len() - 1 - i;
            let state = instrument.reconcile(Message::NoteOff(Channel1, notes[i]));
            assert_relative_eq!(state.frequency, notes[i - 1].to_freq_f32());
        }

        let state = instrument.reconcile(Message::NoteOff(Channel1, notes[1]));
        assert_relative_eq!(state.frequency, 0.0);
    }
}
