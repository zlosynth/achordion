#[derive(Clone, Copy, Default, PartialEq)]
pub struct Config {
    pub overdrive: bool,
    pub reordered_modes: bool,
    pub modal_playing: bool,
}

impl From<u8> for Config {
    fn from(other: u8) -> Self {
        Self {
            overdrive: other & 1 != 0,
            reordered_modes: other & (1 << 1) != 0,
            modal_playing: other & (1 << 2) != 0,
        }
    }
}

impl From<Config> for u8 {
    fn from(other: Config) -> Self {
        let mut value = 0;
        if other.overdrive {
            value |= 1;
        }
        if other.reordered_modes {
            value |= 1 << 1;
        }
        if other.modal_playing {
            value |= 1 << 2;
        }
        value
    }
}

impl From<Config> for [bool; 8] {
    fn from(other: Config) -> Self {
        let mut value = [false; 8];
        let raw: u8 = other.into();

        value[0] = raw & 1 != 0;
        value[1] = raw & (1 << 1) != 0;
        value[2] = raw & (1 << 2) != 0;
        value[3] = raw & (1 << 3) != 0;
        value[4] = raw & (1 << 4) != 0;
        value[5] = raw & (1 << 5) != 0;
        value[6] = raw & (1 << 6) != 0;
        value[7] = raw & (1 << 7) != 0;

        value
    }
}
