use super::note::Note;

pub enum Message {
    NoteOn(Note),
    NoteOff(Note),
}
