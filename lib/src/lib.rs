#![no_std]
#![allow(clippy::new_without_default)]

#[cfg(test)]
#[macro_use]
extern crate approx;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub mod instrument;
pub mod probe;
pub mod waveform;
pub mod wavetable;

mod chords;
mod detune;
mod note;
mod oscillator;
mod quantizer;
mod scales;
mod taper;
