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

#[allow(clippy::excessive_precision)]
const FREQUENCIES: [f32; 128] = [
    8.175798650097969,
    8.661956936691345,
    9.177023699353972,
    9.722717925526148,
    10.30086081896052,
    10.913381877820335,
    11.56232533420019,
    12.249856976560608,
    12.9782713778457,
    13.749999553407081,
    14.567617074291595,
    15.433852662970274,
    16.351597300195937,
    17.323913873382683,
    18.354047398707948,
    19.445435851052295,
    20.601721637921035,
    21.826763755640677,
    23.12465066840038,
    24.49971395312121,
    25.95654275569141,
    27.499999106814162,
    29.135234148583184,
    30.86770532594056,
    32.703194600391875,
    34.647827746765365,
    36.708094797415896,
    38.89087170210459,
    41.20344327584207,
    43.653527511281354,
    46.24930133680076,
    48.99942790624242,
    51.91308551138282,
    54.999998213628324,
    58.27046829716637,
    61.73541065188112,
    65.40638920078375,
    69.29565549353073,
    73.41618959483179,
    77.78174340420918,
    82.40688655168414,
    87.30705502256271,
    92.49860267360152,
    97.99885581248483,
    103.82617102276564,
    109.99999642725665,
    116.54093659433273,
    123.47082130376224,
    130.8127784015675,
    138.59131098706146,
    146.83237918966358,
    155.56348680841836,
    164.81377310336828,
    174.61411004512541,
    184.99720534720305,
    195.99771162496967,
    207.65234204553127,
    219.9999928545133,
    233.08187318866547,
    246.94164260752447,
    261.625556803135,
    277.1826219741231,
    293.664758379327,
    311.1269736168367,
    329.6275462067368,
    349.22822009025066,
    369.9944106944061,
    391.99542324993956,
    415.30468409106226,
    439.9999857090266,
    466.1637463773312,
    493.88328521504866,
    523.25111360627,
    554.3652439482462,
    587.329516758654,
    622.2539472336734,
    659.2550924134736,
    698.4564401805013,
    739.9888213888122,
    783.9908464998791,
    830.6093681821245,
    879.9999714180532,
    932.3274927546624,
    987.7665704300973,
    1046.50222721254,
    1108.7304878964924,
    1174.659033517308,
    1244.507894467347,
    1318.5101848269471,
    1396.9128803610026,
    1479.9776427776244,
    1567.9816929997583,
    1661.218736364249,
    1759.9999428361064,
    1864.654985509325,
    1975.5331408601946,
    2093.00445442508,
    2217.4609757929848,
    2349.318067034616,
    2489.015788934694,
    2637.0203696538942,
    2793.8257607220053,
    2959.955285555249,
    3135.9633859995165,
    3322.437472728498,
    3519.9998856722127,
    3729.30997101865,
    3951.0662817203893,
    4186.00890885016,
    4434.9219515859695,
    4698.636134069232,
    4978.031577869388,
    5274.0407393077885,
    5587.6515214440105,
    5919.910571110498,
    6271.926771999033,
    6644.874945456996,
    7039.999771344425,
    7458.6199420373,
    7902.1325634407785,
    8372.01781770032,
    8869.843903171939,
    9397.272268138464,
    9956.063155738775,
    10548.081478615577,
    11175.303042888021,
    11839.821142220995,
    12543.853543998066,
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
        assert!(note <= Note::HIGHEST_NOTE as u8);
        unsafe { core::mem::transmute(note) }
    }

    #[inline(always)]
    pub fn to_freq_f32(self) -> f32 {
        FREQUENCIES[self as usize]
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
