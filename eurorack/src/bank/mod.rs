macro_rules! factors_ref {
    ( $factors:ident ) => {
        [
            &$factors.as_ref().unwrap().factor1,
            &$factors.as_ref().unwrap().factor2,
            &$factors.as_ref().unwrap().factor4,
            &$factors.as_ref().unwrap().factor8,
            &$factors.as_ref().unwrap().factor16,
            &$factors.as_ref().unwrap().factor32,
            &$factors.as_ref().unwrap().factor64,
            &$factors.as_ref().unwrap().factor128,
            &$factors.as_ref().unwrap().factor256,
            &$factors.as_ref().unwrap().factor512,
            &$factors.as_ref().unwrap().factor1024,
        ]
    };
}

mod harsh;
mod perfect;
mod soft;
mod vocal;

use achordion_lib::wavetable::Wavetable;

use crate::display::{Display, DisplayState};

type FactorsRef = [&'static [u16]; 11];

pub static mut WAVETABLE_BANKS: Option<[&'static [Wavetable<'static>]; 4]> = None;

pub struct Progress<'a> {
    display: &'a mut Display,
    phase: usize,
    total: usize,
}

impl<'a> Progress<'a> {
    pub fn new(display: &'a mut Display, total: usize) -> Self {
        display.set(DisplayState {
            led1: false,
            led2: false,
            led3: false,
            led4: false,
            led5: false,
            led6: false,
            led7: false,
            led_sharp: false,
        });
        Self {
            display,
            phase: 0,
            total,
        }
    }

    pub fn tick(&mut self) {
        self.display.set(DisplayState {
            led1: self.phase as f32 / self.total as f32 > 0.2,
            led2: self.phase as f32 / self.total as f32 > 0.2,
            led3: self.phase as f32 / self.total as f32 > 0.45,
            led4: self.phase as f32 / self.total as f32 > 0.45,
            led5: self.phase as f32 / self.total as f32 > 0.7,
            led6: self.phase as f32 / self.total as f32 > 0.7,
            led7: self.phase as f32 / self.total as f32 > 0.95,
            led_sharp: self.phase as f32 / self.total as f32 > 0.95,
        });
        self.phase += 1;
    }
}

pub fn setup(display: &mut Display) {
    let total = perfect::len() + harsh::len() + soft::len() + vocal::len();
    let mut progress = Progress::new(display, total);

    perfect::setup(&mut progress);
    harsh::setup(&mut progress);
    soft::setup(&mut progress);
    vocal::setup(&mut progress);

    unsafe {
        WAVETABLE_BANKS = Some([
            &perfect::BANK.as_ref().unwrap()[..],
            &harsh::BANK.as_ref().unwrap()[..],
            &soft::BANK.as_ref().unwrap()[..],
            &vocal::BANK.as_ref().unwrap()[..],
        ]);
    }
}
