#[repr(u8)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Channel {
    Channel1 = 0,
    Channel2 = 1,
    Channel3 = 2,
    Channel4 = 3,
    Channel5 = 4,
    Channel6 = 5,
    Channel7 = 6,
    Channel8 = 7,
    Channel9 = 8,
    Channel10 = 9,
    Channel11 = 10,
    Channel12 = 11,
    Channel13 = 12,
    Channel14 = 13,
    Channel15 = 14,
    Channel16 = 15,
}

pub use Channel::*;

#[allow(non_upper_case_globals)]
impl Channel {
    pub const HIGHEST_CHANNEL: Channel = Channel16;

    #[inline(always)]
    pub fn from_u8(channel: u8) -> Channel {
        assert!(channel <= Channel::HIGHEST_CHANNEL as u8);
        unsafe { core::mem::transmute(channel) }
    }
}

impl From<u8> for Channel {
    fn from(byte: u8) -> Self {
        Channel::from_u8(byte)
    }
}
