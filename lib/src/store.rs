use core::convert::TryInto;
use core::mem;

use crc::{Crc, CRC_16_IBM_SDLC};

const CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);

#[derive(Clone, Copy, PartialEq)]
pub struct Parameters {
    pub note: f32,
    pub solo: f32,
    pub wavetable: f32,
    pub bank: f32,
    pub chord: f32,
    pub detune: f32,
    pub scale_root: f32,
    pub scale_mode: f32,
    pub amplitude: f32,
    pub cv1_calibration_ratio: f32,
    pub cv1_calibration_offset: f32,
    pub cv2_calibration_ratio: f32,
    pub cv2_calibration_offset: f32,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            note: 0.0,
            solo: 0.0,
            wavetable: 0.0,
            bank: 0.0,
            chord: 0.0,
            detune: 0.0,
            scale_root: 0.0,
            scale_mode: 0.0,
            amplitude: 0.0,
            cv1_calibration_ratio: 1.0,
            cv1_calibration_offset: 0.0,
            cv2_calibration_ratio: 1.0,
            cv2_calibration_offset: 0.0,
        }
    }
}

impl Parameters {
    const SIZE: usize = mem::size_of::<Self>();

    fn from_bytes(bytes: [u8; Self::SIZE]) -> Parameters {
        macro_rules! f32_from_bytes {
            ( $attribute:ident ) => {{
                let start = offset_of!(Self => $attribute).get_byte_offset();
                let stop = start + mem::size_of::<f32>();
                f32::from_be_bytes(bytes[start..stop].try_into().unwrap())
            }}
        }

        Parameters {
            note: f32_from_bytes!(note),
            solo: f32_from_bytes!(solo),
            wavetable: f32_from_bytes!(wavetable),
            bank: f32_from_bytes!(bank),
            chord: f32_from_bytes!(chord),
            detune: f32_from_bytes!(detune),
            scale_root: f32_from_bytes!(scale_root),
            scale_mode: f32_from_bytes!(scale_mode),
            amplitude: f32_from_bytes!(amplitude),
            cv1_calibration_ratio: f32_from_bytes!(cv1_calibration_ratio),
            cv1_calibration_offset: f32_from_bytes!(cv1_calibration_offset),
            cv2_calibration_ratio: f32_from_bytes!(cv2_calibration_ratio),
            cv2_calibration_offset: f32_from_bytes!(cv2_calibration_offset),
        }
    }

    fn to_bytes(self) -> [u8; Self::SIZE] {
        let mut bytes = [0; Self::SIZE];

        macro_rules! f32_to_bytes {
            ( $attribute:ident ) => {{
                let start = offset_of!(Self => $attribute).get_byte_offset();
                let stop = start + mem::size_of::<f32>();
                bytes[start..stop].copy_from_slice(&self.$attribute.to_be_bytes());
            }}
        }

        f32_to_bytes!(note);
        f32_to_bytes!(solo);
        f32_to_bytes!(wavetable);
        f32_to_bytes!(bank);
        f32_to_bytes!(chord);
        f32_to_bytes!(detune);
        f32_to_bytes!(scale_root);
        f32_to_bytes!(scale_mode);
        f32_to_bytes!(amplitude);
        f32_to_bytes!(cv1_calibration_ratio);
        f32_to_bytes!(cv1_calibration_offset);
        f32_to_bytes!(cv2_calibration_ratio);
        f32_to_bytes!(cv2_calibration_offset);

        bytes
    }
}

#[derive(Clone, Copy)]
pub struct Store {
    version: u16,
    parameters_raw: [u8; Parameters::SIZE],
    crc: u16,
}

lazy_static! {
    static ref PARAMETERS_RAW_START: usize = offset_of!(Store => parameters_raw).get_byte_offset();
    static ref PARAMETERS_RAW_STOP: usize =
        *PARAMETERS_RAW_START + mem::size_of::<[u8; Parameters::SIZE]>();
}

pub struct InvalidData;

impl Store {
    pub const SIZE: usize = mem::size_of::<Self>();

    pub fn new(parameters: Parameters, version: u16) -> Self {
        let parameters_raw = parameters.to_bytes();
        let crc = CRC.checksum(&parameters_raw);
        Self {
            version,
            parameters_raw,
            crc,
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

        let store = Store {
            version: u16_from_bytes!(version),
            parameters_raw: bytes[*PARAMETERS_RAW_START..*PARAMETERS_RAW_STOP]
                .try_into()
                .unwrap(),
            crc: u16_from_bytes!(crc),
        };

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

        u16_to_bytes!(version);
        u16_to_bytes!(crc);

        let parameters_start = offset_of!(Self => parameters_raw).get_byte_offset();
        let parameters_stop = parameters_start + mem::size_of::<[u8; Parameters::SIZE]>();
        bytes[parameters_start..parameters_stop].copy_from_slice(&self.parameters_raw);

        bytes
    }

    pub fn parameters(&self) -> Parameters {
        Parameters::from_bytes(self.parameters_raw)
    }

    pub fn version(&self) -> u16 {
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
            amplitude: 0.8,
            cv1_calibration_ratio: 0.9,
            cv1_calibration_offset: 0.91,
            cv2_calibration_ratio: 0.92,
            cv2_calibration_offset: 0.93,
            solo: 0.94,
        };
        let bytes = parameters.to_bytes();
        assert!(Parameters::from_bytes(bytes) == parameters);
    }
}
