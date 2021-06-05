# Changelog

All notable changes to this project will be documented in this file. See
[VERSIONING.md](VERSIONING.md) for more information about versioning and
backwards compatibility.

## Unreleased

* Added an internal probe detecting whether there is anything connected to CV
  inputs.
* Introduced variable octave offset. When V/OCT CV is connected, control the
  offset via the note pot.

## 0.3.0

* Detune sections now respond logarithmically, this allows for finer control in
  lower values.
* Added pulse waveforms of variable width.
* Support multiple wavetable banks.
* Label button and probe pins on the PCB.
* Switch to Daisy Seed, STM32H7.

## 0.2.0

* Finally starting using Semantic Versioning.
* Compensate perceived amplitudes, making sure that combination of less
  oscillators sound as loud as many.
* Added support for silenced chord degrees, allowing for subset of voices to
  sound and build smaller chords/intervals.
* Added support for chord degrees spanning multiple upwards octaves.
* Added support for inverted chords.
* Added support for seventh chords.
* Defined versioning and compatibility policies.

## 7123cbb, revision 2-3

* Increased size of pads on the PCB for easier hand soldering.

## 27707a5, revision 2

* Converted the board into 2-layer design.
* Replacing 7-segment display with LEDs.
* Added CV probing to the PCB, allowing to detect connected jacks.
* Mixing both output channels when only one jack is plugged in.
* Added labels to PCB silkscreen.
* Spaced out jack sockets on the PCB.
* Adjusted all the CV ranges.
* Introduced Pure Data External to allow for easier testing.
* Quantizing note to diatonic scale.
* Wavetable control.
* Chord control.
* Removed control over MIDI.
* Added detune control and suboscillators.
* Improved reading of potentiometer values with buffering.

## cb3808f, revision 1

* Introduced panel sketch.
* PCB designed based around Daisy Seed.
* 4 basic waveforms.
* Quantizing notes to selected scale.
* Chord, scale mode, and tone controlled via MIDI over USB.

