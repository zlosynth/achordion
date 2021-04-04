#![no_std]
#![allow(clippy::new_without_default)]

#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod midi;
pub mod oscillator;
pub mod wavetable;
