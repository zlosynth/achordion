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

type FactorsRef = [&'static [u16]; 11];

pub static mut WAVETABLE_BANKS: Option<[&'static [Wavetable<'static>]; 4]> = None;

pub fn setup() {
    perfect::setup();
    harsh::setup();
    soft::setup();
    vocal::setup();

    unsafe {
        WAVETABLE_BANKS = Some([
            &perfect::BANK.as_ref().unwrap()[..],
            &harsh::BANK.as_ref().unwrap()[..],
            &soft::BANK.as_ref().unwrap()[..],
            &vocal::BANK.as_ref().unwrap()[..],
        ]);
    }
}
