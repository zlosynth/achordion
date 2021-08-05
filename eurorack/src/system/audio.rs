// TODO: This should introduce an abstraction to deal with producing and consuming of audio
use daisy_bsp::audio::{Block, Interface, Sai1Pins, BLOCK_LENGTH};

use crate::system::hal;
use hal::gpio;

static mut BUFFER: [(f32, f32); BLOCK_LENGTH] = [(0.0, 0.0); BLOCK_LENGTH];

pub struct AudioPins {
    pub pdn: gpio::gpiob::PB11<gpio::Output<gpio::PushPull>>,
    pub mclk_a: gpio::gpioe::PE2<gpio::Alternate<gpio::AF6>>,
    pub sck_a: gpio::gpioe::PE5<gpio::Alternate<gpio::AF6>>,
    pub fs_a: gpio::gpioe::PE4<gpio::Alternate<gpio::AF6>>,
    pub sd_a: gpio::gpioe::PE6<gpio::Alternate<gpio::AF6>>,
    pub sd_b: gpio::gpioe::PE3<gpio::Alternate<gpio::AF6>>,
}

impl From<AudioPins> for Sai1Pins {
    fn from(audio_pins: AudioPins) -> Self {
        (
            audio_pins.pdn,
            audio_pins.mclk_a,
            audio_pins.sck_a,
            audio_pins.fs_a,
            audio_pins.sd_a,
            audio_pins.sd_b,
        )
    }
}

pub struct Audio<'a> {
    interface: Option<Interface<'a>>,
}

impl Audio<'_> {
    pub fn init(
        pins: AudioPins,
        clocks: &hal::rcc::CoreClocks,
        sai: hal::rcc::rec::Sai1,
        dma: hal::rcc::rec::Dma1,
    ) -> Self {
        Self {
            interface: Some(Interface::init(clocks, sai, pins.into(), dma).unwrap()),
        }
    }

    pub fn spawn(&mut self) {
        self.interface = Some(self.interface.take().unwrap().spawn(callback).unwrap());
    }

    pub fn update_buffer(&mut self, mut callback: impl FnMut(&mut [(f32, f32); BLOCK_LENGTH])) {
        let buffer: &'static mut [(f32, f32); BLOCK_LENGTH] = unsafe { &mut BUFFER };
        callback(buffer);
        self.interface
            .as_mut()
            .unwrap()
            .handle_interrupt_dma1_str1()
            .unwrap();
    }
}

fn callback(_fs: f32, block: &mut Block) {
    let buffer: &'static mut [(f32, f32); BLOCK_LENGTH] = unsafe { &mut BUFFER };
    for (source, target) in buffer.iter().zip(block.iter_mut()) {
        *target = *source;
    }
}

// TODO: Keep the buffer here, accept a callback filling in stuff
