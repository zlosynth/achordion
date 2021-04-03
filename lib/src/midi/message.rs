use super::channel::Channel;
use super::note::Note;
use super::velocity::Velocity;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Message {
    NoteOn(Channel, Note, Velocity),
    NoteOff(Channel, Note),
}
