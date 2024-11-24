#[repr(u8)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Note {
    CMinus1 = 0,
    DbMinus1 = 1,
    DMinus1 = 2,
    EbMinus1 = 3,
    EMinus1 = 4,
    FMinus1 = 5,
    GbMinus1 = 6,
    GMinus1 = 7,
    AbMinus1 = 8,
    AMinus1 = 9,
    BbMinus1 = 10,
    BMinus1 = 11,
    C0 = 12,
    Db0 = 13,
    D0 = 14,
    Eb0 = 15,
    E0 = 16,
    F0 = 17,
    Gb0 = 18,
    G0 = 19,
    Ab0 = 20,
    A0 = 21,
    Bb0 = 22,
    B0 = 23,
    C1 = 24,
    Db1 = 25,
    D1 = 26,
    Eb1 = 27,
    E1 = 28,
    F1 = 29,
    Gb1 = 30,
    G1 = 31,
    Ab1 = 32,
    A1 = 33,
    Bb1 = 34,
    B1 = 35,
    C2 = 36,
    Db2 = 37,
    D2 = 38,
    Eb2 = 39,
    E2 = 40,
    F2 = 41,
    Gb2 = 42,
    G2 = 43,
    Ab2 = 44,
    A2 = 45,
    Bb2 = 46,
    B2 = 47,
    C3 = 48,
    Db3 = 49,
    D3 = 50,
    Eb3 = 51,
    E3 = 52,
    F3 = 53,
    Gb3 = 54,
    G3 = 55,
    Ab3 = 56,
    A3 = 57,
    Bb3 = 58,
    B3 = 59,
    C4 = 60,
    Db4 = 61,
    D4 = 62,
    Eb4 = 63,
    E4 = 64,
    F4 = 65,
    Gb4 = 66,
    G4 = 67,
    Ab4 = 68,
    A4 = 69,
    Bb4 = 70,
    B4 = 71,
    C5 = 72,
    Db5 = 73,
    D5 = 74,
    Eb5 = 75,
    E5 = 76,
    F5 = 77,
    Gb5 = 78,
    G5 = 79,
    Ab5 = 80,
    A5 = 81,
    Bb5 = 82,
    B5 = 83,
    C6 = 84,
    Db6 = 85,
    D6 = 86,
    Eb6 = 87,
    E6 = 88,
    F6 = 89,
    Gb6 = 90,
    G6 = 91,
    Ab6 = 92,
    A6 = 93,
    Bb6 = 94,
    B6 = 95,
    C7 = 96,
    Db7 = 97,
    D7 = 98,
    Eb7 = 99,
    E7 = 100,
    F7 = 101,
    Gb7 = 102,
    G7 = 103,
    Ab7 = 104,
    A7 = 105,
    Bb7 = 106,
    B7 = 107,
    C8 = 108,
    Db8 = 109,
    D8 = 110,
    Eb8 = 111,
    E8 = 112,
    F8 = 113,
    Gb8 = 114,
    G8 = 115,
    Ab8 = 116,
    A8 = 117,
    Bb8 = 118,
    B8 = 119,
    C9 = 120,
    Db9 = 121,
    D9 = 122,
    Eb9 = 123,
    E9 = 124,
    F9 = 125,
    Gb9 = 126,
    G9 = 127,
}

// NOTE: Generated with Python:
// ```python
// for i in range(128):
//     print("{},".format(440 * 2.0 ** ((i - 12 * 6 + 3) / 12)))
// ```
#[allow(clippy::excessive_precision)]
const FREQUENCIES: [f32; 128] = [
    8.175798915643707,
    8.661957218027252,
    9.177023997418988,
    9.722718241315029,
    10.300861153527183,
    10.913382232281373,
    11.562325709738575,
    12.249857374429663,
    12.978271799373287,
    13.75,
    14.567617547440307,
    15.433853164253883,
    16.351597831287414,
    17.323914436054505,
    18.354047994837977,
    19.445436482630058,
    20.601722307054366,
    21.826764464562746,
    23.12465141947715,
    24.499714748859326,
    25.956543598746574,
    27.5,
    29.13523509488062,
    30.86770632850775,
    32.70319566257483,
    34.64782887210901,
    36.70809598967594,
    38.890872965260115,
    41.20344461410875,
    43.653528929125486,
    46.2493028389543,
    48.999429497718666,
    51.91308719749314,
    55.0,
    58.27047018976124,
    61.7354126570155,
    65.40639132514966,
    69.29565774421802,
    73.41619197935188,
    77.78174593052023,
    82.4068892282175,
    87.30705785825097,
    92.4986056779086,
    97.99885899543733,
    103.82617439498628,
    110.0,
    116.54094037952248,
    123.47082531403103,
    130.8127826502993,
    138.59131548843604,
    146.8323839587038,
    155.56349186104046,
    164.81377845643496,
    174.61411571650194,
    184.9972113558172,
    195.99771799087463,
    207.65234878997256,
    220.0,
    233.08188075904496,
    246.94165062806206,
    261.6255653005986,
    277.1826309768721,
    293.6647679174076,
    311.1269837220809,
    329.6275569128699,
    349.2282314330039,
    369.9944227116344,
    391.99543598174927,
    415.3046975799451,
    440.0,
    466.1637615180899,
    493.8833012561241,
    523.2511306011972,
    554.3652619537442,
    587.3295358348151,
    622.2539674441618,
    659.2551138257398,
    698.4564628660078,
    739.9888454232688,
    783.9908719634985,
    830.6093951598903,
    880.0,
    932.3275230361799,
    987.7666025122483,
    1046.5022612023945,
    1108.7305239074883,
    1174.6590716696303,
    1244.5079348883237,
    1318.5102276514797,
    1396.9129257320155,
    1479.9776908465376,
    1567.981743926997,
    1661.2187903197805,
    1760.0,
    1864.6550460723597,
    1975.533205024496,
    2093.004522404789,
    2217.4610478149766,
    2349.31814333926,
    2489.0158697766474,
    2637.02045530296,
    2793.825851464031,
    2959.955381693075,
    3135.9634878539946,
    3322.437580639561,
    3520.0,
    3729.3100921447194,
    3951.066410048992,
    4186.009044809578,
    4434.922095629953,
    4698.63628667852,
    4978.031739553295,
    5274.04091060592,
    5587.651702928062,
    5919.91076338615,
    6271.926975707989,
    6644.875161279122,
    7040.0,
    7458.620184289437,
    7902.132820097988,
    8372.018089619156,
    8869.844191259906,
    9397.272573357044,
    9956.06347910659,
    10548.081821211836,
    11175.303405856126,
    11839.8215267723,
    12543.853951415975,
];

#[allow(non_upper_case_globals)]
impl Note {
    pub const CSharpMinus1: Note = Note::DbMinus1;
    pub const DSharpMinus1: Note = Note::EbMinus1;
    pub const FSharpMinus1: Note = Note::GbMinus1;
    pub const GSharpMinus1: Note = Note::AbMinus1;
    pub const ASharpMinus1: Note = Note::BbMinus1;
    pub const CSharp0: Note = Note::Db0;
    pub const DSharp0: Note = Note::Eb0;
    pub const FSharp0: Note = Note::Gb0;
    pub const GSharp0: Note = Note::Ab0;
    pub const ASharp0: Note = Note::Bb0;
    pub const CSharp1: Note = Note::Db1;
    pub const DSharp1: Note = Note::Eb1;
    pub const FSharp1: Note = Note::Gb1;
    pub const GSharp1: Note = Note::Ab1;
    pub const ASharp1: Note = Note::Bb1;
    pub const CSharp2: Note = Note::Db2;
    pub const DSharp2: Note = Note::Eb2;
    pub const FSharp2: Note = Note::Gb2;
    pub const GSharp2: Note = Note::Ab2;
    pub const ASharp2: Note = Note::Bb2;
    pub const CSharp3: Note = Note::Db3;
    pub const DSharp3: Note = Note::Eb3;
    pub const FSharp3: Note = Note::Gb3;
    pub const GSharp3: Note = Note::Ab3;
    pub const ASharp3: Note = Note::Bb3;
    pub const CSharp4: Note = Note::Db4;
    pub const DSharp4: Note = Note::Eb4;
    pub const FSharp4: Note = Note::Gb4;
    pub const GSharp4: Note = Note::Ab4;
    pub const ASharp4: Note = Note::Bb4;
    pub const CSharp5: Note = Note::Db5;
    pub const DSharp5: Note = Note::Eb5;
    pub const FSharp5: Note = Note::Gb5;
    pub const GSharp5: Note = Note::Ab5;
    pub const ASharp5: Note = Note::Bb5;
    pub const CSharp6: Note = Note::Db6;
    pub const DSharp6: Note = Note::Eb6;
    pub const FSharp6: Note = Note::Gb6;
    pub const GSharp6: Note = Note::Ab6;
    pub const ASharp6: Note = Note::Bb6;
    pub const CSharp7: Note = Note::Db7;
    pub const DSharp7: Note = Note::Eb7;
    pub const FSharp7: Note = Note::Gb7;
    pub const GSharp7: Note = Note::Ab7;
    pub const ASharp7: Note = Note::Bb7;
    pub const CSharp8: Note = Note::Db8;
    pub const DSharp8: Note = Note::Eb8;
    pub const FSharp8: Note = Note::Gb8;
    pub const GSharp8: Note = Note::Ab8;
    pub const ASharp8: Note = Note::Bb8;
    pub const CSharp9: Note = Note::Db9;
    pub const DSharp9: Note = Note::Eb9;
    pub const FSharp9: Note = Note::Gb9;

    pub const HIGHEST_NOTE: Note = Note::G9;

    #[inline(always)]
    pub fn from_u8(note: u8) -> Note {
        Self::try_from_i16(note as i16).unwrap()
    }

    #[inline(always)]
    pub fn try_from_u8(note: u8) -> Option<Note> {
        Self::try_from_i16(note as i16)
    }

    #[inline(always)]
    pub fn try_from_i16(note: i16) -> Option<Note> {
        if note >= 0 && note <= Note::HIGHEST_NOTE as i16 {
            Some(unsafe { core::mem::transmute(note as u8) })
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn to_freq_f32(self) -> f32 {
        FREQUENCIES[self as usize]
    }

    #[inline(always)]
    pub fn to_voct(self) -> f32 {
        (self as usize) as f32 / 12.0
    }

    #[inline(always)]
    pub fn to_midi_id(self) -> u8 {
        self as u8
    }
}

impl From<u8> for Note {
    fn from(byte: u8) -> Self {
        Note::from_u8(byte)
    }
}

impl From<Note> for u8 {
    fn from(note: Note) -> u8 {
        note.to_midi_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_note_to_frequency() {
        assert_relative_eq!(Note::A4.to_freq_f32(), 440.0);
    }

    #[test]
    fn convert_to_midi_id() {
        assert_eq!(Note::C3.to_midi_id(), 48);
    }
}
