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
}

impl<'a> Progress<'a> {
    pub fn new(display: &'a mut Display) -> Self {
        Self { display, phase: 0 }
    }

    pub fn tick(&mut self) {
        self.display.set(DisplayState {
            led1: self.phase == 0,
            led2: self.phase == 7,
            led3: self.phase == 1,
            led4: self.phase == 6,
            led5: self.phase == 2,
            led6: self.phase == 5,
            led7: self.phase == 3,
            led_sharp: self.phase == 4,
        });
        self.phase = (self.phase + 1) % 8;
    }
}

pub fn setup(display: &mut Display) {
    let mut progress = Progress::new(display);
    progress.tick();

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
