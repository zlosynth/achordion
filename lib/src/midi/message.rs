use super::channel::Channel;
use super::note::Note;

pub enum Message {
    NoteOn(Channel, Note),
    NoteOff(Channel, Note),
}
