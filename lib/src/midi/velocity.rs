#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Velocity(u8);

impl Velocity {
    pub const MAX: Velocity = Velocity(127);

    #[inline(always)]
    pub fn from_u8(velocity: u8) -> Velocity {
        assert!(velocity <= Velocity::MAX.0);
        unsafe { core::mem::transmute(velocity) }
    }
}

impl From<u8> for Velocity {
    fn from(byte: u8) -> Self {
        Velocity::from_u8(byte)
    }
}

impl From<Velocity> for u8 {
    fn from(velocity: Velocity) -> Self {
        velocity.0
    }
}
