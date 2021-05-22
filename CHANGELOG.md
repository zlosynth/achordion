# Changelog

All notable changes to this project will be documented in this file. The project
adheres to [Semantic Versioning](https://semver.org/), where the versioned
interface is the one between the hardware and software.

## Unreleased

* Finally starting using Semantic Versioning.
* Compensate perceived amplitudes, making sure that combination of less
  oscillators sound as loud as many.

## 7123cbb, revision 3

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

