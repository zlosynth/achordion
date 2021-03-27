// #![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    asm::nop();

    loop {}
}
