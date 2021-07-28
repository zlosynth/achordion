# Changelog

All notable changes to this project will be documented in this file. See
[VERSIONING.md](VERSIONING.md) for more information about versioning and
backwards compatibility.

## Unreleased

## 0.7.0

* Visualize changes in settings on the display, both from pots and CV.
* Allow to offset Note CV to negative to compensate for high voltage input.
* Fix equilibrium of wavetables with depth of 16 bits.

## 0.6.0

* Added tooling supporting generation of wavetables from raw audio.
* Lowered LPF on wavetables to make the lower end thicker.
* Use floats for audio buffers instead of u16.
* Remove pulse wavetable bank.
* Add a bank compiled of FM, distorted and sampled wavetables.
* Add a bank compiled of soft wavetables.
* Add a bank of vocal wavetables.

## 0.5.0

* Allow control of wavetable bank through Wavetable pot while holding the
  button.
* Allow control of scale root through Note pot while holding the button. This
  value will be combined with the Tonic CV in case it is connected.
* Allow control of scale mode through Chord pot while holding the button. This
  value will be combined with the Mode CV in case it is connected.
* Remove support for 4 degrees of chords. It draws too much resources.

## 0.4.0

* Allowed control of all parameters through CV:
  - Root note, adhering to V/OCT. When connected, Note potentiometer controls
    octave offset.
  - Wavetable position. When connected, Wavetable potentiometer controls the
    offset.
  - Chord. When connected, Chord potentiometer controls the offset.
  - Detune. When connected, Detune potentiometer controls the offset.
  - Scale mode.
  - Scale root note, adhering to V/OCT.
* Fixed the bug where root and chord went to both channels.
* Introduced fade in when the module is powered on.
* Fixed an issue where full detune setting would disable detune.

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

