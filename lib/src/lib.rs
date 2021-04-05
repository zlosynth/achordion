#![no_std]
#![allow(clippy::new_without_default)]

#[cfg(test)]
#[macro_use]
extern crate approx;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub mod midi;
pub mod oscillator;
pub mod waveform;
pub mod wavetable;
