#[allow(unused_imports)]
use micromath::F32Ext;

use daisy::hal;
use daisy_bsp as daisy;

use hal::adc::{Adc, Enabled};
use hal::pac::{ADC1, ADC2};

use achordion_lib::store::Parameters;

use crate::system::Button;
use crate::system::Probe;
use crate::system::{Cv1, Cv2, Cv3, Cv4, Cv5, Cv6};
use crate::system::{Pot1, Pot2, Pot3, Pot4};

// V/OCT CV spans from 0.0 to 10.0 V.
const VOCT_CV_RANGE: f32 = 10.0;

pub struct ControlsConfig {
    pub adc1: Adc<ADC1, Enabled>,
    pub adc2: Adc<ADC2, Enabled>,
    pub alt_button: Button,
    pub pot_note: Pot1,
    pub pot_wavetable: Pot2,
    pub pot_chord: Pot3,
    pub pot_detune: Pot4,
    pub cv_voct: Cv1,
    pub cv_scale_tonic: Cv2,
    pub cv_scale_mode: Cv3,
    pub cv_chord: Cv4,
    pub cv_detune: Cv5,
    pub cv_wavetable: Cv6,
    pub cv_probe: Probe,
}

pub struct Controls {
    adc1: Adc<ADC1, Enabled>,
    adc2: Adc<ADC2, Enabled>,
    button: Button,
    pot1: Pot1,
    pot2: Pot2,
    pot3: Pot3,
    pot4: Pot4,
    cv1: Cv1,
    cv2: Cv2,
    cv3: Cv3,
    cv4: Cv4,
    cv5: Cv5,
    cv6: Cv6,
    probe: Probe,

    parameters: Parameters,

    last_note_pot_reading: f32,
    last_wavetable_pot_reading: f32,
    last_scale_root_pot_reading: f32,
    last_chord_pot_reading: f32,
    last_scale_mode_pot_reading: f32,

    calibration_state: CalibrationState,
    calibration_ratio: f32,
    calibration_offset: f32,
}

enum CalibrationState {
    Inactive,
    Entering,
    CalibratingLow,
    CalibratingHigh(f32),
}

impl Controls {
    pub fn new(config: ControlsConfig, parameters: Parameters) -> Self {
        let mut controls = Self {
            adc1: config.adc1,
            adc2: config.adc2,
            button: config.alt_button,
            pot1: config.pot_note,
            pot2: config.pot_wavetable,
            pot3: config.pot_chord,
            pot4: config.pot_detune,
            cv1: config.cv_voct,
            cv2: config.cv_scale_tonic,
            cv3: config.cv_scale_mode,
            cv4: config.cv_chord,
            cv5: config.cv_detune,
            cv6: config.cv_wavetable,
            probe: config.cv_probe,

            parameters,

            // These values are used to cache the last read value while the pot
            // is in its alternative mode (depending on the button state).
            last_note_pot_reading: 0.0,
            last_wavetable_pot_reading: 0.0,
            last_scale_root_pot_reading: 0.0,
            last_chord_pot_reading: 0.0,
            last_scale_mode_pot_reading: 0.0,

            calibration_state: CalibrationState::Inactive,
            calibration_ratio: 1.0,
            calibration_offset: 0.0,
        };

        // Initial probe tick, so the signal has enough time to propagate to all
        // the detectors.
        controls.probe.tick();

        controls
    }

    pub fn parameters(&self) -> Parameters {
        self.parameters
    }

    pub fn note(&self) -> f32 {
        self.parameters.note
    }

    pub fn wavetable(&self) -> f32 {
        self.parameters.wavetable
    }

    pub fn chord(&self) -> f32 {
        self.parameters.chord
    }

    pub fn detune(&self) -> f32 {
        self.parameters.detune
    }

    pub fn wavetable_bank(&self) -> f32 {
        self.parameters.bank
    }

    pub fn scale_root(&self) -> f32 {
        self.parameters.scale_root
    }

    pub fn scale_mode(&self) -> f32 {
        self.parameters.scale_mode
    }

    pub fn active(&self) -> bool {
        self.button.active()
            || self.pot1.active()
            || self.pot2.active()
            || self.pot3.active()
            || self.pot4.active()
    }

    pub fn wavetable_bank_pot_active(&self) -> bool {
        self.button.active() && self.pot2.active()
    }

    pub fn wavetable_pot_active(&self) -> bool {
        !self.button.active() && self.pot2.active()
    }

    pub fn note_pot_active(&self) -> bool {
        !self.button.active() && self.pot1.active()
    }

    pub fn scale_root_pot_active(&self) -> bool {
        self.button.active() && self.pot1.active()
    }

    pub fn scale_mode_pot_active(&self) -> bool {
        self.button.active() && self.pot3.active()
    }

    pub fn chord_pot_active(&self) -> bool {
        !self.button.active() && self.pot3.active()
    }

    pub fn detune_pot_active(&self) -> bool {
        self.pot4.active()
    }

    pub fn update(&mut self) {
        self.sample();
        self.reconcile();
    }

    fn sample(&mut self) {
        self.pot1.start_sampling(&mut self.adc2);
        self.pot2.start_sampling(&mut self.adc1);
        self.pot1.finish_sampling(&mut self.adc2);
        self.pot2.finish_sampling(&mut self.adc1);

        self.pot3.start_sampling(&mut self.adc1);
        self.pot4.start_sampling(&mut self.adc2);
        self.pot3.finish_sampling(&mut self.adc1);
        self.pot4.finish_sampling(&mut self.adc2);

        self.cv1.start_sampling(&mut self.adc1);
        self.cv2.start_sampling(&mut self.adc2);
        self.cv1.finish_sampling(&mut self.adc1);
        self.cv2.finish_sampling(&mut self.adc2);

        self.cv3.start_sampling(&mut self.adc1);
        self.cv4.start_sampling(&mut self.adc2);
        self.cv3.finish_sampling(&mut self.adc1);
        self.cv4.finish_sampling(&mut self.adc2);

        self.cv5.start_sampling(&mut self.adc1);
        self.cv6.start_sampling(&mut self.adc2);
        self.cv5.finish_sampling(&mut self.adc1);
        self.cv6.finish_sampling(&mut self.adc2);

        self.button.sample();

        // This has to be set last as it takes a while for the probe to get to
        // the detector. The interval between samples is enough for that.
        self.probe.tick();
    }

    fn reconcile(&mut self) {
        self.reconcile_note();
        self.reconcile_wavetable();
        self.reconcile_wavetable_bank();
        self.reconcile_chord();
        self.reconcile_detune();
        self.reconcile_scale_root();
        self.reconcile_scale_mode();
        self.reconcile_calibration();
    }

    fn reconcile_note(&mut self) {
        if !self.button.active() {
            self.last_note_pot_reading = self.pot1.value();
        };
        let pot = self.last_note_pot_reading;

        self.parameters.note = if self.cv1.connected() {
            // Keep the multiplier below 4, so assure that the result won't get
            // into the 5th octave when set on the edge.
            let octave_offset = (pot * 3.95).trunc() - 2.0;
            let note = self.sample_to_voct(self.cv1.value());
            note + octave_offset
        } else {
            pot * 4.0 + 3.0
        };
    }

    fn reconcile_wavetable(&mut self) {
        if !self.button.active() {
            self.last_wavetable_pot_reading = self.pot2.value();
        }
        let pot = self.last_wavetable_pot_reading;

        self.parameters.wavetable = if self.cv6.connected() {
            // CV is centered around zero, suited for LFO.
            let wavetable = self.cv6.value() * 2.0 - 1.0;
            let offset = pot;
            (wavetable + offset).min(0.9999).max(0.0)
        } else {
            pot
        };
    }

    fn reconcile_wavetable_bank(&mut self) {
        if self.wavetable_bank_pot_active() {
            self.parameters.bank = self.pot2.value();
        };
    }

    fn reconcile_chord(&mut self) {
        if !self.button.active() {
            self.last_chord_pot_reading = self.pot3.value();
        };
        let pot = self.last_chord_pot_reading;

        self.parameters.chord = if self.cv4.connected() {
            // CV is centered around zero, suited for LFO.
            let chord = self.cv4.value() * 2.0 - 1.0;
            let offset = pot;
            (chord + offset).min(0.9999).max(0.0)
        } else {
            pot
        };
    }

    fn reconcile_detune(&mut self) {
        self.parameters.detune = if self.cv5.connected() {
            // CV is centered around zero, suited for LFO.
            let detune = self.cv5.value() * 2.0 - 1.0;
            let offset = self.pot4.value();
            (detune + offset).min(0.9999).max(0.0)
        } else {
            self.pot4.value()
        };
    }

    fn reconcile_scale_root(&mut self) {
        if self.scale_root_pot_active() {
            self.last_scale_root_pot_reading = self.pot1.value();
        }
        let pot = self.last_scale_root_pot_reading;

        let cv = if self.cv2.connected() {
            self.sample_to_voct(self.cv2.value())
        } else {
            0.0
        };

        self.parameters.scale_root = cv + pot;
    }

    fn reconcile_scale_mode(&mut self) {
        if self.scale_mode_pot_active() {
            self.last_scale_mode_pot_reading = self.pot3.value();
        }
        let pot = self.last_scale_mode_pot_reading;

        let cv = if self.cv3.connected() {
            self.cv3.value() * 2.0 - 1.0
        } else {
            0.0
        };

        self.parameters.scale_mode = cv + pot;
    }

    fn reconcile_calibration(&mut self) {
        if self.cv1.was_unplugged() {
            self.calibration_state = CalibrationState::Inactive;
        } else {
            match self.calibration_state {
                CalibrationState::Inactive => {
                    if self.button.active() && self.cv1.was_plugged() {
                        self.calibration_state = CalibrationState::Entering;
                    }
                }
                CalibrationState::Entering => {
                    if !self.button.active() {
                        self.calibration_state = CalibrationState::CalibratingLow;
                    }
                }
                CalibrationState::CalibratingLow => {
                    if self.button.clicked() {
                        let c_a = self.cv1.value() * VOCT_CV_RANGE;
                        self.calibration_state = CalibrationState::CalibratingHigh(c_a);
                    }
                }
                CalibrationState::CalibratingHigh(c_a) => {
                    if self.button.clicked() {
                        let c_b = self.cv1.value() * VOCT_CV_RANGE;
                        self.calibrate(c_a, c_b);
                        self.calibration_state = CalibrationState::Inactive;
                    }
                }
            }
        }
    }

    fn calibrate(&mut self, c_a: f32, c_b: f32) {
        if let Ok((calibration_ratio, calibration_offset)) = calculate_calibration(c_a, c_b) {
            self.calibration_ratio = calibration_ratio;
            self.calibration_offset = calibration_offset;
        }
    }

    fn sample_to_voct(&self, transposed_sample: f32) -> f32 {
        let voct = transposed_sample * VOCT_CV_RANGE;
        voct * self.calibration_ratio + self.calibration_offset
    }
}

fn calculate_calibration(c_a: f32, c_b: f32) -> Result<(f32, f32), ()> {
    let (c_a, c_b) = if c_a < c_b { (c_a, c_b) } else { (c_b, c_a) };

    if c_b - c_a < 0.5 {
        return Err(());
    }

    let calibration_ratio = 1.0 / (c_b - c_a);

    let calibration_offset = if (c_a * calibration_ratio).fract() > 0.5 {
        1.0 - (c_a * calibration_ratio).fract()
    } else {
        -1.0 * (c_a * calibration_ratio).fract()
    };

    Ok((calibration_ratio, calibration_offset))
}
