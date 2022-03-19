# Changelog

All notable changes to this project will be documented in this file. See
[VERSIONING.md](VERSIONING.md) for more information about versioning and
backwards compatibility.

## Unreleased

* Replace Daisy Seed with Daisy Patch Submodule, to assure CE requirements.
* Switch to 0805 footprints for easier hand-soldering.
* Change from 3 board to 2 board design to become skiff friendly.
* Remove debug pins due to lack of I/O.
* Prevent flash memory from getting worn out by utilizing its full range and
  increasing the backup interval.

## 1.3.1

* Fix english, change "unisono" to "unison".
* Disable line voice when its V/OCT input gets below zero.
* Fix a regression where the module would not fade in on boot.
* Document current requirements.

## 1.3.0

* Introduce documentation of the build process.
* Remove amplitude balancing on small chords. This causes weak signal output but
  guarantees stable loudness on continuous tones.
* Adjust solo and chord V/OCT tracking to react uniformly to input voltage.
* Avoid pops on chord changes by increasing the fade-in/out time of individual
  oscillators to 3 ms.

## 1.2.0

* Add mounting holes for distancing standoffs.
* Label boards and their sides to allow clear identification while building.
* Make it possible to disable quantization on the side line.
* Divide each octave V/OCT input to 7 almost equal sections.
* Document the depth of the module.
* Fully utilize the display while indicating the chord mode.
* Introduce V/OCT control of chords.
* Change CV range for chords to 0-10V.
* Improve the amplitude balance between low and high number of voices.
* Fix a bug where uninitialized voices would cause bit noise.
* Send solo output to SOLO' jack, as written in the documentation.
* Decrease the brightness of LEDs.

## 1.1.0

* The display now priorititizes what is being changed through pots over CV.
  Furthermore, one CV overtakes another only after the original source is idle.
* Fix calibration display. The lower octave is now signalized by LEDs on the
  left, higher on the right.
* Increase display sensitivity to CV, so it shows even small changes of input.
* If multiple CV inputs compete, the one that was controled through the pot most
  recently is picked.
* Prevent solo CV from bouncing between notes.
* Stabilize the display, so it does not blink while crossing large differences
  in V/OCT.
* Change the loading animation to go one LED at the time.
* Fix scale tonic control, so the pot does not wrap around when set to top.
* Resolve a bug where saved parameters sometimes failed to get restored.

## 1.0.0

* Convert PCB to two stacked layers for easier soldering.
* Change the panel layout.
* Expose debugging GPIO and UART pins.
* Wire compatibility version into the PCB.
* Document the module through a user manual.

## 0.16.0

* Allow using 3 voices in each chord degree.
* Use 2 sub-octave oscillators in the chord detune mode.
* Prevent phase lock by detuning solo if it collides with the chord.
* Prevent pop on change of detune by fading in/out the oscillators.
* Smoothly traverse wavetable changes between CV reconciles.
* Store bandlimited wavetables as f32 sequences to save on processing.
* Compensate amplitude of lower numbers of enabled oscillators.

## 0.15.0

* Speed up boot time of the module.
* Display progress of the boot on module's display.
* Exponentially fade in on boot.
* Increase the Note pot range to 6 octaves, extending in both directions.
* Make eurorack module louder.
* Replace vowel bank with sine multiple and sum sequences.
* Optimize booting code for size, to free up more space for wavetables.
* Make it possible to select between two bandlimiting algorithms.

## 0.14.0

* Introduce "style" setting to switch between different approaches to select
  harmonies. This also introduces alternative to chords where a simple interval
  +- 2 octaves is selected.
* Add a new play style which opens chords note by note. This could be used with
  ADSR connected to chord input to play more human-like.
* Add a new play mode which contains inversions of chords around the root.
* Increase the maximum number of chord degrees to 5 and populate the default
  chord style with some nineth chords.

## 0.13.0

* Add 4th voice to the instrument and allow its individual control through CV.
* List more accurate parts for the V/OCT CV input.
* Instead of pre-calculating bandlimited wavetables and storing them in flash,
  filter them during runtime initialization and store them in SRAM. This allows
  to fit more banks into eurorack and also to higher optimization.
* Reintroduce vowel wavetable bank into eurorack module.

## 0.12.0

* Align all quantization with white keys. All scales can be now played through
  white keys only, with black keys always quantizing to the previous white.
* Introduce CV calibration, allowing the V/OCT CV input to be calibrated
  precisely on any connected device.
* Allow reset of all stored parameters on boot by holding the button while
  powering on.
* Prevent jumps back and forth on borders between two discrete values of
  parameters.
* Scroll through notes via the pot lineary, instead of unequal jumps between
  tones and semitones.
* Increase detune phase on the display lineary.

## 0.11.0

* Remove CV buffering for faster response.
* Fixed a bug where settings backup to EEPROM was hogging the MCU and causing CV
  latency up to 100 ms. Now the CV latency should be around 2 ms.
* Increased control loop frequency to provide 0.5 ms latency.
* Simlified connected jack detection probe to reduce MCU usage.

## 0.10.0

* Update PCB with user labels for printing and easier soldering.
* Replace tantalum DC decoupling capacitors with aluminium.
* Use 10uF for DC decoupling capacitors.
* Scale modulation CV input range from -5 to +5 V.
* Scale V/OCT CV input range from 0 to +10 V.
* Add a script generating gerber and drill files.
* Add a script printing PDF with the schematic.
* Add a script printing PDF with assembly helper.
* Remove amplitude control.

## 0.9.0

* Remove component labels from the silkscreen.
* Add graphics to front and back of the PCB.
* Widen tracks on the PCB.
* Replace decoupling capacitor on DAC with LPF.
* Introduce ferrite beads to filter the power input.
* Major refactoring of the eurorack codebase, introducing new hardware
  abstractions.

## 0.8.1

* Fix dependency set on a daisy_bsp fork.

## 0.8.0

* Persist dialed settings between restarts.

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

