use core::convert::TryFrom;

use usbd_midi::data::byte::u7::U7;
use usbd_midi::data::midi::channel::Channel as USBDChannel;
use usbd_midi::data::midi::message::control_function::ControlFunction as USBDControlFunction;
use usbd_midi::data::midi::message::message::Message as USBDMessage;
use usbd_midi::data::midi::notes::Note as USBDNote;

use super::channel::Channel;
use super::control::{ControlFunction, ControlValue};
use super::message::Message;
use super::note::Note;
use super::velocity::Velocity;

impl From<U7> for Velocity {
    fn from(value: U7) -> Self {
        Self::from(u8::from(value))
    }
}

impl From<USBDChannel> for Channel {
    fn from(usbd_channel: USBDChannel) -> Self {
        Self::from(usbd_channel as u8)
    }
}

impl From<USBDNote> for Note {
    fn from(usbd_note: USBDNote) -> Self {
        Self::from(usbd_note as u8)
    }
}

impl From<USBDControlFunction> for ControlFunction {
    fn from(usbd_function: USBDControlFunction) -> Self {
        Self::from(unsafe { core::mem::transmute::<_, u8>(usbd_function) })
    }
}

impl From<U7> for ControlValue {
    fn from(usbd_value: U7) -> Self {
        Self::from(u8::from(usbd_value))
    }
}

impl TryFrom<USBDMessage> for Message {
    type Error = &'static str;

    fn try_from(usbd_message: USBDMessage) -> Result<Self, Self::Error> {
        match usbd_message {
            USBDMessage::NoteOn(channel, note, velocity) => Ok(Message::NoteOn(
                channel.into(),
                note.into(),
                velocity.into(),
            )),
            USBDMessage::NoteOff(channel, note, _) => {
                Ok(Message::NoteOff(channel.into(), note.into()))
            }
            USBDMessage::ControlChange(channel, function, value) => Ok(Message::ControlChange(
                channel.into(),
                function.into(),
                value.into(),
            )),
            _ => Err("conversion not available"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::TryInto;

    #[test]
    fn convert_usbd_velocity_to_internal() {
        let note: Velocity = U7::try_from(20).ok().unwrap().into();
        assert_eq!(note, Velocity::from_u8(20));
    }

    #[test]
    fn convert_usbd_channel_to_internal() {
        let note: Channel = USBDChannel::Channel1.into();
        assert_eq!(note, Channel::Channel1);
    }

    #[test]
    fn convert_usbd_note_to_internal() {
        let note: Note = USBDNote::A4.into();
        assert_eq!(note, Note::A4);
    }

    #[test]
    fn convert_usbd_control_function_to_internal() {
        let function: ControlFunction = USBDControlFunction::MOD_WHEEL_1.into();
        assert_eq!(function, ControlFunction::CC1);
    }

    #[test]
    fn convert_usbd_note_on_message_to_internal() {
        let message: Message = USBDMessage::NoteOn(
            USBDChannel::Channel1,
            USBDNote::A4,
            U7::try_from(20).ok().unwrap(),
        )
        .try_into()
        .unwrap();
        assert_eq!(
            message,
            Message::NoteOn(Channel::Channel1, Note::A4, Velocity::from_u8(20))
        );
    }

    #[test]
    fn convert_usbd_note_off_message_to_internal() {
        let message: Message = USBDMessage::NoteOff(
            USBDChannel::Channel1,
            USBDNote::A4,
            U7::try_from(0).ok().unwrap(),
        )
        .try_into()
        .unwrap();
        assert_eq!(message, Message::NoteOff(Channel::Channel1, Note::A4));
    }

    #[test]
    fn convert_usbd_control_change_message_to_internal() {
        let message: Message = USBDMessage::ControlChange(
            USBDChannel::Channel1,
            USBDControlFunction::MOD_WHEEL_1,
            U7::try_from(100).ok().unwrap(),
        )
        .try_into()
        .unwrap();
        assert_eq!(
            message,
            Message::ControlChange(
                Channel::Channel1,
                ControlFunction::CC1,
                ControlValue::from_u8(100)
            )
        );
    }
}