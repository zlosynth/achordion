use stm32f4xx_hal::gpio::gpiod::PD4;
use stm32f4xx_hal::gpio::{Output, PushPull};
use stm32f4xx_hal::hal::blocking::i2c::Write;
use stm32f4xx_hal::prelude::*;

/// Interface to the I2C control port of a Cirrus Logic CS43L22 DAC
pub struct Cs43L22<I> {
    /// I2C interface
    i2c: I,
    /// Address of DAC in 7 bit, shifted left
    address: u8,
}

impl<I> Cs43L22<I>
where
    I: Write,
{
    pub fn new(
        i2c: I,
        address: u8,
        reset_pin: PD4<Output<PushPull>>,
    ) -> Result<Self, <I as Write>::Error> {
        let mut dac = Cs43L22 { i2c, address };
        dac.reset(reset_pin);
        dac.setup()?;
        Ok(dac)
    }

    /// Does basic configuration as specified in the datasheet
    fn setup(&mut self) -> Result<(), <I as Write>::Error> {
        // Settings from section 4.11 of the datasheet
        self.write(Register::Magic00, 0x99)?;
        self.write(Register::Magic47, 0x80)?;
        self.write(Register::Magic32, 0x80)?;
        self.write(Register::Magic32, 0x00)?;
        self.write(Register::Magic00, 0x00)?;

        // Clocking control from the table in section 4.6 of the datasheet:
        // Auto mode: disabled
        // Speed mode: 01 (single-speed)
        // 8 kHz, 16 kHz, or 32 kHz sample rate: no
        // 27 MHz video clock: no
        // Internal MCLK/LRCLCK ratio: 00
        // MCLK divide by 2: no
        #[allow(clippy::unusual_byte_groupings)]
        self.write(Register::ClockingCtl, 0b0_01_0_0_00_0)?;

        // Interface control:
        // Slave mode
        // SCLK not inverted
        // DSP mode disabled
        // Interface format I2S
        // Word length 16 bits
        #[allow(clippy::unusual_byte_groupings)]
        self.write(Register::InterfaceCtl1, 0b0_0_0_0_01_11)
    }

    fn reset(&mut self, mut reset_pin: PD4<Output<PushPull>>) {
        // Keep DAC reset low for at least one millisecond
        cortex_m::asm::delay(168_000_000 / 1_000);

        // Release the DAC from reset
        reset_pin.set_high().unwrap();

        // Wait at least 550 ns before starting I2C communication
        cortex_m::asm::delay(168_000_000 / 1_000);
    }

    pub fn set_volume_a(&mut self, volume: i8) -> Result<(), <I as Write>::Error> {
        self.write(Register::HeadphoneAVol, volume as u8)
    }

    pub fn set_volume_b(&mut self, volume: i8) -> Result<(), <I as Write>::Error> {
        self.write(Register::HeadphoneBVol, volume as u8)
    }

    pub fn enable(&mut self) -> Result<(), <I as Write>::Error> {
        self.write(Register::PowerCtl1, 0b1001_1110)
    }

    fn write(&mut self, register: Register, value: u8) -> Result<(), <I as Write>::Error> {
        // Set auto-increment bit
        let map = (register as u8) | 0x80;
        self.i2c.write(self.address, &[map, value])
    }
}

/// CS43L22 registers
#[allow(dead_code)]
enum Register {
    /// This is used in the specified startup sequence, but its actual content is not documented.
    Magic00 = 0x00,
    Id = 0x01,
    PowerCtl1 = 0x02,
    PowerCtl2 = 0x04,
    ClockingCtl = 0x05,
    InterfaceCtl1 = 0x06,
    InterfaceCtl2 = 0x07,
    PassthroughASelect = 0x08,
    PassthroughBSelect = 0x09,
    AnalogZcSr = 0x0a,
    PassthroughGangCtl = 0x0c,
    PlaybackCtl1 = 0x0d,
    MiscCtl = 0x0e,
    PlaybackCtl2 = 0x0f,
    PassthroughAVol = 0x14,
    PassthroughBVol = 0x15,
    PcmAVol = 0x1a,
    PcmBVol = 0x1b,
    BeepFreqOnTime = 0x1c,
    BeepVolOffTime = 0x1d,
    BeepToneCfg = 0x1e,
    ToneCtl = 0x1f,
    MasterAVol = 0x20,
    MasterBVol = 0x21,
    HeadphoneAVol = 0x22,
    HeadphoneBVol = 0x23,
    SpeakerAVol = 0x24,
    SpeakerBVol = 0x25,
    ChannelMixer = 0x26,
    LimitCtl1 = 0x27,
    LimitClt2 = 0x28,
    LimitAttack = 0x29,
    Status = 0x2e,
    BatteryComp = 0x2f,
    VpBatteryLevel = 0x30,
    SpeakerStatus = 0x31,
    /// This is used in the specified startup sequence, but its actual content is not documented.
    Magic32 = 0x32,
    ChargePumpFreq = 0x34,
    /// This is used in the specified startup sequence, but its actual content is not documented.
    Magic47 = 0x47,
}
