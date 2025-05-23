#[allow(unused_imports)]
use micromath::F32Ext;

use core::convert::TryInto;
use core::mem;

use crc::{Crc, CRC_16_USB};

use crate::config::Config;

const CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_USB);

#[derive(Clone, Copy, PartialEq)]
pub struct Parameters {
    pub note: f32,
    pub solo: f32,
    pub solo_quantization: bool,
    pub solo_enabled: bool,
    pub wavetable: f32,
    pub bank: f32,
    pub chord: f32,
    pub chord_quantization: bool,
    pub style: f32,
    pub detune: f32,
    pub scale_root: f32,
    pub scale_mode: f32,
    pub last_scale_mode_pot_reading: f32,
    pub amplitude: f32,
    pub cv1_calibration_ratio: f32,
    pub cv1_calibration_offset: f32,
    pub cv2_calibration_ratio: f32,
    pub cv2_calibration_offset: f32,
    pub cv5_calibration_ratio: f32,
    pub cv5_calibration_offset: f32,
    pub config: Config,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            note: 0.0,
            solo: 0.0,
            solo_quantization: true,
            solo_enabled: true,
            wavetable: 0.0,
            bank: 0.0,
            chord: 0.0,
            chord_quantization: false,
            style: 0.0,
            detune: 0.0,
            scale_root: 0.0,
            scale_mode: 0.0,
            last_scale_mode_pot_reading: 0.0,
            amplitude: 0.0,
            cv1_calibration_ratio: 1.0,
            cv1_calibration_offset: 0.0,
            cv2_calibration_ratio: 1.0,
            cv2_calibration_offset: 0.0,
            cv5_calibration_ratio: 1.0,
            cv5_calibration_offset: 0.0,
            config: Config::default(),
        }
    }
}

impl Parameters {
    const SIZE: usize = mem::size_of::<Self>();

    fn from_bytes(bytes: [u8; Self::SIZE]) -> Self {
        unsafe { mem::transmute(bytes) }
    }

    fn to_bytes(self) -> [u8; Self::SIZE] {
        unsafe { mem::transmute(self) }
    }

    // For purposes of checking whether the new value is different enough from
    // the previousy stored one
    pub fn close_to(&self, other: &Self) -> bool {
        f32_close(self.note, other.note)
            && f32_close(self.solo, other.solo)
            && f32_close(self.wavetable, other.wavetable)
            && f32_close(self.bank, other.bank)
            && f32_close(self.chord, other.chord)
            && f32_close(self.style, other.style)
            && f32_close(self.detune, other.detune)
            && f32_close(self.scale_root, other.scale_root)
            && f32_close(self.scale_mode, other.scale_mode)
            && f32_close(
                self.last_scale_mode_pot_reading,
                other.last_scale_mode_pot_reading,
            )
            && f32_close(self.amplitude, other.amplitude)
            && self.cv1_calibration_ratio == other.cv1_calibration_ratio
            && self.cv1_calibration_offset == other.cv1_calibration_offset
            && self.cv2_calibration_ratio == other.cv2_calibration_ratio
            && self.cv2_calibration_offset == other.cv2_calibration_offset
            && self.cv5_calibration_ratio == other.cv5_calibration_ratio
            && self.cv5_calibration_offset == other.cv5_calibration_offset
            && self.solo_quantization == other.solo_quantization
            && self.solo_enabled == other.solo_enabled
            && self.chord_quantization == other.chord_quantization
            && self.config == other.config
    }
}

fn f32_close(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.01
}

#[derive(Clone, Copy)]
pub struct Store {
    version: u32,
    token: u16,
    parameters_raw: [u8; Parameters::SIZE],
    crc: u16,
}

lazy_static! {
    static ref PARAMETERS_RAW_START: usize = offset_of!(Store => parameters_raw).get_byte_offset();
    static ref PARAMETERS_RAW_STOP: usize =
        *PARAMETERS_RAW_START + mem::size_of::<[u8; Parameters::SIZE]>();
}

// This constant is used to invalidate data when needed
const TOKEN: u16 = 102;

pub struct InvalidData;

impl Store {
    pub const SIZE: usize = mem::size_of::<Self>();

    pub fn new(parameters: Parameters, version: u32) -> Self {
        let parameters_raw = parameters.to_bytes();
        let crc = CRC.checksum(&parameters_raw);
        Self {
            version,
            parameters_raw,
            crc,
            token: TOKEN,
        }
    }

    pub fn from_bytes(bytes: [u8; Self::SIZE]) -> Result<Self, InvalidData> {
        macro_rules! u16_from_bytes {
            ( $attribute:ident ) => {{
                let start = offset_of!(Self => $attribute).get_byte_offset();
                let stop = start + mem::size_of::<u16>();
                u16::from_be_bytes(bytes[start..stop].try_into().unwrap())
            }}
        }

        macro_rules! u32_from_bytes {
            ( $attribute:ident ) => {{
                let start = offset_of!(Self => $attribute).get_byte_offset();
                let stop = start + mem::size_of::<u32>();
                u32::from_be_bytes(bytes[start..stop].try_into().unwrap())
            }}
        }

        let store = Store {
            version: u32_from_bytes!(version),
            parameters_raw: bytes[*PARAMETERS_RAW_START..*PARAMETERS_RAW_STOP]
                .try_into()
                .unwrap(),
            crc: u16_from_bytes!(crc),
            token: u16_from_bytes!(token),
        };

        if store.token != TOKEN {
            return Err(InvalidData);
        }

        let crc = CRC.checksum(&store.parameters_raw);
        if crc == store.crc {
            Ok(store)
        } else {
            Err(InvalidData)
        }
    }

    pub fn to_bytes(self) -> [u8; Self::SIZE] {
        let mut bytes = [0; Self::SIZE];

        macro_rules! u16_to_bytes {
            ( $attribute:ident ) => {{
                let start = offset_of!(Self => $attribute).get_byte_offset();
                let stop = start + mem::size_of::<u16>();
                bytes[start..stop].copy_from_slice(&self.$attribute.to_be_bytes());
            }}
        }

        macro_rules! u32_to_bytes {
            ( $attribute:ident ) => {{
                let start = offset_of!(Self => $attribute).get_byte_offset();
                let stop = start + mem::size_of::<u32>();
                bytes[start..stop].copy_from_slice(&self.$attribute.to_be_bytes());
            }}
        }

        u32_to_bytes!(version);
        u16_to_bytes!(crc);
        u16_to_bytes!(token);

        let parameters_start = offset_of!(Self => parameters_raw).get_byte_offset();
        let parameters_stop = parameters_start + mem::size_of::<[u8; Parameters::SIZE]>();
        bytes[parameters_start..parameters_stop].copy_from_slice(&self.parameters_raw);

        bytes
    }

    pub fn parameters(&self) -> Parameters {
        Parameters::from_bytes(self.parameters_raw)
    }

    pub fn version(&self) -> u32 {
        self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_store() {
        let _store = Store::new(Parameters::default(), 0);
    }

    #[test]
    fn get_parameters_from_store() {
        let parameters = Parameters::default();
        let store = Store::new(parameters, 0);
        assert!(store.parameters() == parameters);
    }

    #[test]
    fn get_version_from_store() {
        let store = Store::new(Parameters::default(), 10);
        assert_eq!(store.version(), 10);
    }

    #[test]
    fn initialize_store_from_bytes() {
        let store_a = Store::new(Parameters::default(), 0);
        let bytes = store_a.to_bytes();
        let store_b = Store::from_bytes(bytes).ok().unwrap();
        assert!(store_a.parameters() == store_b.parameters());
    }

    #[test]
    fn detect_invalid_crc_while_initializing_from_bytes() {
        let store = Store::new(Parameters::default(), 0);
        let mut bytes = store.to_bytes();
        bytes[5] = 0x13;
        assert!(Store::from_bytes(bytes).is_err());
    }

    #[test]
    fn dump_store_as_bytes() {
        let mut parameters_a = Parameters::default();
        parameters_a.note = 0.0;
        let store_a = Store::new(parameters_a, 0);
        let bytes_a = store_a.to_bytes();

        let mut parameters_b = Parameters::default();
        parameters_b.note = 1.0;
        let store_b = Store::new(parameters_b, 0);
        let bytes_b = store_b.to_bytes();

        assert!(bytes_a != bytes_b);
    }

    #[test]
    fn convert_parameters_to_and_from_bytes() {
        let parameters = Parameters {
            note: 0.1,
            wavetable: 0.2,
            bank: 0.3,
            chord: 0.4,
            detune: 0.5,
            scale_root: 0.6,
            scale_mode: 0.7,
            last_scale_mode_pot_reading: 0.75,
            amplitude: 0.8,
            cv1_calibration_ratio: 0.9,
            cv1_calibration_offset: 0.91,
            cv2_calibration_ratio: 0.92,
            cv2_calibration_offset: 0.93,
            solo: 0.94,
            style: 0.95,
            solo_quantization: true,
            solo_enabled: true,
            cv5_calibration_ratio: 0.96,
            cv5_calibration_offset: 0.97,
            chord_quantization: true,
            config: Config::from(1),
        };
        let bytes = parameters.to_bytes();
        assert!(Parameters::from_bytes(bytes) == parameters);
    }

    #[test]
    fn parameters_fit_into_one_page() {
        let page_size = 256;
        let parameters_size = mem::size_of::<Parameters>();
        assert!(parameters_size < page_size);
    }
}
