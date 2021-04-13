#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ControlFunction {
    CC0 = 0,
    CC1 = 1,
    CC2 = 2,
    CC3 = 3,
    CC4 = 4,
    CC5 = 5,
    CC6 = 6,
    CC7 = 7,
    CC8 = 8,
    CC9 = 9,
    CC10 = 10,
    CC11 = 11,
    CC12 = 12,
    CC13 = 13,
    CC14 = 14,
    CC15 = 15,
    CC16 = 16,
    CC17 = 17,
    CC18 = 18,
    CC19 = 19,
    CC20 = 20,
    CC21 = 21,
    CC22 = 22,
    CC23 = 23,
    CC24 = 24,
    CC25 = 25,
    CC26 = 26,
    CC27 = 27,
    CC28 = 28,
    CC29 = 29,
    CC30 = 30,
    CC31 = 31,
    CC32 = 32,
    CC33 = 33,
    CC34 = 34,
    CC35 = 35,
    CC36 = 36,
    CC37 = 37,
    CC38 = 38,
    CC39 = 39,
    CC40 = 40,
    CC41 = 41,
    CC42 = 42,
    CC43 = 43,
    CC44 = 44,
    CC45 = 45,
    CC46 = 46,
    CC47 = 47,
    CC48 = 48,
    CC49 = 49,
    CC50 = 50,
    CC51 = 51,
    CC52 = 52,
    CC53 = 53,
    CC54 = 54,
    CC55 = 55,
    CC56 = 56,
    CC57 = 57,
    CC58 = 58,
    CC59 = 59,
    CC60 = 60,
    CC61 = 61,
    CC62 = 62,
    CC63 = 63,
    CC64 = 64,
    CC65 = 65,
    CC66 = 66,
    CC67 = 67,
    CC68 = 68,
    CC69 = 69,
    CC70 = 70,
    CC71 = 71,
    CC72 = 72,
    CC73 = 73,
    CC74 = 74,
    CC75 = 75,
    CC76 = 76,
    CC77 = 77,
    CC78 = 78,
    CC79 = 79,
    CC80 = 80,
    CC81 = 81,
    CC82 = 82,
    CC83 = 83,
    CC84 = 84,
    CC85 = 85,
    CC86 = 86,
    CC87 = 87,
    CC88 = 88,
    CC89 = 89,
    CC90 = 90,
    CC91 = 91,
    CC92 = 92,
    CC93 = 93,
    CC94 = 94,
    CC95 = 95,
    CC96 = 96,
    CC97 = 97,
    CC98 = 98,
    CC99 = 99,
    CC100 = 100,
    CC101 = 101,
    CC102 = 102,
    CC103 = 103,
    CC104 = 104,
    CC105 = 105,
    CC106 = 106,
    CC107 = 107,
    CC108 = 108,
    CC109 = 109,
    CC110 = 110,
    CC111 = 111,
    CC112 = 112,
    CC113 = 113,
    CC114 = 114,
    CC115 = 115,
    CC116 = 116,
    CC117 = 117,
    CC118 = 118,
    CC119 = 119,
    CC120 = 120,
    CC121 = 121,
    CC122 = 122,
    CC123 = 123,
    CC124 = 124,
    CC125 = 125,
    CC126 = 126,
    CC127 = 127,
}

#[allow(non_upper_case_globals)]
impl ControlFunction {
    pub const HIGHEST_CONTROL_FUNCTION: ControlFunction = ControlFunction::CC127;

    #[inline(always)]
    pub fn from_u8(function: u8) -> ControlFunction {
        assert!(function <= ControlFunction::HIGHEST_CONTROL_FUNCTION as u8);
        unsafe { core::mem::transmute(function) }
    }
}

impl From<u8> for ControlFunction {
    fn from(byte: u8) -> Self {
        ControlFunction::from_u8(byte)
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ControlValue(u8);

impl ControlValue {
    pub const MAX: ControlValue = ControlValue(127);

    #[inline(always)]
    pub fn from_u8(velocity: u8) -> ControlValue {
        assert!(velocity <= ControlValue::MAX.0);
        unsafe { core::mem::transmute(velocity) }
    }
}

impl From<u8> for ControlValue {
    fn from(byte: u8) -> Self {
        ControlValue::from_u8(byte)
    }
}

impl From<ControlValue> for u8 {
    fn from(velocity: ControlValue) -> Self {
        velocity.0
    }
}
