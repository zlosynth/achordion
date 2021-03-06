#![no_std]
#![allow(clippy::new_without_default)]

#[macro_use]
extern crate field_offset;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod display;
pub mod instrument;
pub mod probe;
pub mod store;
pub mod waveform;
pub mod wavetable;

mod chords;
mod detune;
mod note;
mod oscillator;
mod quantizer;
mod scales;
mod taper;
