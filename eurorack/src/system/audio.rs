use daisy::audio::{self, Block, Interface};

// pub const SAMPLE_RATE: u32 = audio::FS.to_Hz();
// NOTE: The SAMPLE_RATE needs to be adjusted. Probably because the clock on
// STM32 is unable to exactly match the speed.
pub const SAMPLE_RATE: u32 = 47_810;
pub const BLOCK_LENGTH: usize = audio::BLOCK_LENGTH;

static mut BUFFER: [(f32, f32); BLOCK_LENGTH] = [(0.0, 0.0); BLOCK_LENGTH];

pub struct Audio {
    interface: Option<Interface>,
}

impl Audio {
    pub fn init(interface: daisy::audio::Interface) -> Self {
        Self {
            interface: Some(interface),
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
