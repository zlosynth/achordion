#[derive(Clone, Copy, Default, PartialEq)]
pub struct Config {
    config: u8,
}

impl Config {
    pub fn overdrive(&self) -> bool {
        self.config & 1 != 0
    }

    pub fn modes_ordered_by_brightness(&self) -> bool {
        self.config & (1 << 1) != 0
    }

    pub fn mode_controlled_by_detune_cv(&self) -> bool {
        self.config & (1 << 2) != 0
    }

    pub fn tonic_controlled_by_solo_cv(&self) -> bool {
        self.config & (1 << 3) != 0
    }
}

impl From<u8> for Config {
    fn from(other: u8) -> Self {
        Self {
            config: other & 0b1111,
        }
    }
}

impl From<Config> for u8 {
    fn from(other: Config) -> Self {
        other.config
    }
}

impl From<Config> for [bool; 8] {
    fn from(other: Config) -> Self {
        let mut value = [false; 8];

        value[0] = other.overdrive();
        value[1] = other.modes_ordered_by_brightness();
        value[2] = other.mode_controlled_by_detune_cv();
        value[3] = other.tonic_controlled_by_solo_cv();

        value
    }
}
