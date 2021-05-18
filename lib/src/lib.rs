#![no_std]
#![allow(clippy::new_without_default)]

#[cfg(test)]
#[macro_use]
extern crate approx;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub mod chords;
pub mod instrument;
pub mod note;
pub mod oscillator;
pub mod quantizer;
pub mod scales;
pub mod waveform;
pub mod wavetable;
