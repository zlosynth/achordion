#[allow(unused_imports)]
use micromath::F32Ext;

use daisy::hal;

use hal::adc::{Adc, Enabled};
use hal::pac::{ADC1, ADC2};

use achordion_lib::config::Config;
use achordion_lib::store::Parameters;

use crate::system::Button;
use crate::system::Probe;
use crate::system::{Cv1, Cv2, Cv3, Cv4, Cv5};
use crate::system::{Pot1, Pot2, Pot3, Pot4};

// V/OCT CV spans from -5.0 to 5.0 V.
const VOCT_CV_RANGE: f32 = 10.0;

pub struct ControlsConfig {
    pub adc1: Adc<ADC1, Enabled>,
    pub adc2: Adc<ADC2, Enabled>,
    pub alt_button: Button,
    pub pot_wavetable: Pot1,
    pub pot_note: Pot2,
    pub pot_detune: Pot3,
    pub pot_chord: Pot4,
    pub cv_voct: Cv1,
    pub cv_solo: Cv2,
    pub cv_wavetable: Cv3,
    pub cv_detune: Cv4,
    pub cv_chord: Cv5,
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
    probe: Probe,

    parameters: Parameters,

    last_note_pot_reading: f32,
    last_wavetable_pot_reading: f32,
    last_scale_root_pot_reading: f32,
    last_chord_pot_reading: f32,
    last_detune_pot_reading: f32,

    scale_mode_cv: Option<f32>,

    note_source: NoteSource,

    calibration_target: CalibrationTarget,
    calibration_state: CalibrationState,

    configuration_state: ConfigurationState,
}

#[derive(Clone, Copy)]
enum CalibrationTarget {
    Cv1,
    Cv2,
    Cv5,
    None,
}

#[derive(Clone, Copy)]
enum CalibrationState {
    Inactive,
    Entering,
    CalibratingLow,
    CalibratingHigh(f32),
    Succeeded,
    Failed,
}

#[derive(Clone, Copy)]
enum ConfigurationState {
    Inactive,
    Active,
}

#[derive(Clone, Copy)]
enum Submenu {
    None,
    Calibration,
    Configuration,
}

impl Submenu {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_calibration(&self) -> bool {
        matches!(self, Self::Calibration)
    }

    pub fn is_configuration(&self) -> bool {
        matches!(self, Self::Configuration)
    }
}

#[derive(Clone, Copy)]
enum NoteSource {
    Cv,
    Pot,
}

impl Controls {
    pub fn new(config: ControlsConfig, parameters: Parameters) -> Self {
        let mut controls = Self {
            adc1: config.adc1,
            adc2: config.adc2,
            button: config.alt_button,
            pot1: config.pot_wavetable,
            pot2: config.pot_note,
            pot3: config.pot_detune,
            pot4: config.pot_chord,
            cv1: config.cv_voct,
            cv2: config.cv_solo,
            cv3: config.cv_wavetable,
            cv4: config.cv_detune,
            cv5: config.cv_chord,
            probe: config.cv_probe,

            parameters,

            // These values are used to cache the last read value while the pot
            // is in its alternative mode (depending on the button state).
            last_note_pot_reading: 0.0,
            last_wavetable_pot_reading: 0.0,
            last_scale_root_pot_reading: 0.0,
            last_chord_pot_reading: 0.0,
            last_detune_pot_reading: 0.0,

            scale_mode_cv: None,

            note_source: NoteSource::Pot,

            calibration_target: CalibrationTarget::None,
            calibration_state: CalibrationState::Inactive,

            configuration_state: ConfigurationState::Inactive,
        };

        // Initial probe tick, so the signal has enough time to propagate to all
        // the detectors.
        controls.probe.tick();

        controls
    }

    pub fn parameters(&self) -> Parameters {
        self.parameters
    }

    pub fn note(&self) -> Option<f32> {
        if self.parameters.note < 0.5 / 12.0 {
            None
        } else {
            Some(self.parameters.note)
        }
    }

    pub fn solo(&self) -> Option<f32> {
        if self.parameters.solo_enabled {
            Some(self.parameters.solo)
        } else {
            None
        }
    }

    pub fn note_from_pot(&self) -> bool {
        matches!(self.note_source, NoteSource::Pot)
    }

    pub fn wavetable(&self) -> f32 {
        self.parameters.wavetable
    }

    pub fn chord(&self) -> f32 {
        self.parameters.chord
    }

    pub fn style(&self) -> f32 {
        self.parameters.style
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
        if let Some(scale_mode_cv) = self.scale_mode_cv {
            scale_mode_cv
        } else {
            self.parameters.scale_mode
        }
    }

    pub fn active(&self) -> bool {
        self.button.active() || self.pots_active()
    }

    fn pots_active(&self) -> bool {
        self.pot1.active() || self.pot2.active() || self.pot3.active() || self.pot4.active()
    }

    pub fn wavetable_bank_pot_active(&self) -> bool {
        self.button.active() && self.pot1.active()
    }

    pub fn wavetable_pot_active(&self) -> bool {
        !self.button.active() && self.pot1.active()
    }

    pub fn note_pot_active(&self) -> bool {
        !self.button.active() && self.pot2.active()
    }

    pub fn scale_root_pot_active(&self) -> bool {
        self.button.active() && self.pot2.active()
    }

    pub fn chord_pot_active(&self) -> bool {
        !self.button.active() && self.pot4.active()
    }

    pub fn style_pot_active(&self) -> bool {
        self.button.active() && self.pot4.active()
    }

    pub fn detune_pot_active(&self) -> bool {
        !self.button.active() && self.pot3.active()
    }

    pub fn scale_mode_pot_active(&self) -> bool {
        self.button.active() && self.pot3.active()
    }

    pub fn calibration_waiting_low(&self) -> bool {
        matches!(self.calibration_state, CalibrationState::Entering)
            || matches!(self.calibration_state, CalibrationState::CalibratingLow)
    }

    pub fn calibration_waiting_high(&self) -> bool {
        matches!(self.calibration_state, CalibrationState::CalibratingHigh(_))
    }

    pub fn calibration_succeeded(&self) -> bool {
        matches!(self.calibration_state, CalibrationState::Succeeded)
    }

    pub fn calibration_failed(&self) -> bool {
        matches!(self.calibration_state, CalibrationState::Failed)
    }

    pub fn config_open(&self) -> bool {
        self.active_submenu().is_configuration()
    }

    pub fn config(&self) -> Config {
        self.parameters.config
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

        self.pot3.start_sampling(&mut self.adc2);
        self.pot4.start_sampling(&mut self.adc1);
        self.pot3.finish_sampling(&mut self.adc2);
        self.pot4.finish_sampling(&mut self.adc1);

        self.cv1.start_sampling(&mut self.adc1);
        self.cv2.start_sampling(&mut self.adc2);
        self.cv1.finish_sampling(&mut self.adc1);
        self.cv2.finish_sampling(&mut self.adc2);

        self.cv3.start_sampling(&mut self.adc1);
        self.cv4.start_sampling(&mut self.adc2);
        self.cv3.finish_sampling(&mut self.adc1);
        self.cv4.finish_sampling(&mut self.adc2);

        self.cv5.start_sampling(&mut self.adc1);
        self.cv5.finish_sampling(&mut self.adc1);

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
        self.reconcile_style();
        self.reconcile_detune();
        self.reconcile_solo();
        self.reconcile_scale_root();
        self.reconcile_scale_mode();
        self.reconcile_solo_quantization();
        self.reconcile_chord_quantization();

        if self.active_submenu().is_none() || self.active_submenu().is_calibration() {
            self.reconcile_calibration();
        }

        if self.active_submenu().is_none() || self.active_submenu().is_configuration() {
            self.reconcile_configuration();
        }
    }

    fn reconcile_note(&mut self) {
        if !self.button.active() {
            self.last_note_pot_reading = self.pot2.value();
        };
        let pot = self.last_note_pot_reading;

        self.parameters.note = if self.cv1.connected() {
            // Keep the multiplier below 5, so assure that the result won't get
            // into the 6th octave when set on the edge.
            let octave_offset = (pot * 4.95).trunc() - 5.0;
            let note = self.cv1_sample_to_voct(self.cv1.value());
            self.note_source = NoteSource::Cv;
            if note < 0.5 / 12.0 {
                0.0
            } else {
                note + octave_offset
            }
        } else {
            self.note_source = NoteSource::Pot;
            pot * 6.0 + 2.0 + 0.7 / 7.0
        };
    }

    fn reconcile_wavetable(&mut self) {
        if !self.button.active() {
            self.last_wavetable_pot_reading = self.pot1.value();
        }
        let pot = self.last_wavetable_pot_reading;

        self.parameters.wavetable = if self.cv3.connected() {
            // CV is centered around zero, suited for LFO.
            let wavetable = self.cv3.value() * 2.0 - 1.0;
            let offset = pot;
            (wavetable + offset).min(0.9999).max(0.0)
        } else {
            pot
        };
    }

    fn reconcile_wavetable_bank(&mut self) {
        if self.wavetable_bank_pot_active() {
            self.parameters.bank = self.pot1.value();
        };
    }

    fn reconcile_chord(&mut self) {
        if !self.button.active() {
            self.last_chord_pot_reading = self.pot4.value();
        };
        let pot = self.last_chord_pot_reading;

        self.parameters.chord = if self.cv5.connected() {
            if self.parameters.chord_quantization {
                // Keep the multiplier below 7, so assure that the result won't get
                // into the 8th octave when set on the edge.
                let offset = (pot * 6.95).trunc() - 7.0;
                self.cv5_sample_to_voct(self.cv5.value()) + offset
            } else {
                // CV is centered around zero, suited for LFO.
                let chord = self.cv5.value() * 2.0 - 1.0;
                let offset = pot;
                (chord + offset).min(0.9999).max(0.0)
            }
        } else {
            pot
        };
    }

    fn reconcile_chord_quantization(&mut self) {
        if !self.cv5.connected() {
            self.parameters.chord_quantization = false;
        }
    }

    pub fn chord_quantization(&self) -> bool {
        self.parameters.chord_quantization
    }

    fn reconcile_style(&mut self) {
        if self.style_pot_active() {
            self.parameters.style = self.pot4.value();
        };
    }

    fn reconcile_detune(&mut self) {
        if !self.button.active() {
            self.last_detune_pot_reading = self.pot3.value();
        };
        let pot = self.last_detune_pot_reading;

        self.parameters.detune = if self.cv4.connected() && !self.mode_controlled_by_detune_cv() {
            // CV is centered around zero, suited for LFO.
            let detune = self.cv4.value() * 2.0 - 1.0;
            let offset = pot;
            (detune + offset).min(0.9999).max(0.0)
        } else {
            pot
        };
    }

    fn reconcile_solo(&mut self) {
        if self.cv2.connected() && !self.tonic_controlled_by_solo_cv() {
            let note = self.cv2_sample_to_voct(self.cv2.value());
            let offset = -4.0;
            self.parameters.solo = note + offset;
            self.parameters.solo_enabled = true;
        } else {
            self.parameters.solo = 0.0;
            self.parameters.solo_enabled = false;
        }
    }

    fn reconcile_scale_root(&mut self) {
        self.parameters.scale_root = if self.cv2.connected() && self.tonic_controlled_by_solo_cv() {
            self.cv2_sample_to_voct(self.cv2.value())
        } else {
            if self.scale_root_pot_active() {
                self.last_scale_root_pot_reading = self.pot2.value();
            }
            const HALF_SEMITONE: f32 = 0.5 / 12.0;
            self.last_scale_root_pot_reading * 0.98 - HALF_SEMITONE
        };
    }

    fn reconcile_scale_mode(&mut self) {
        if self.scale_mode_pot_active() {
            self.parameters.scale_mode = self.pot3.value();
        }
        self.scale_mode_cv = if self.cv4.connected() && self.mode_controlled_by_detune_cv() {
            if self.parameters.scale_mode < 0.5 {
                Some(self.cv4.value())
            } else {
                Some(self.cv4.value() * 2.0 - 1.0)
            }
        } else {
            None
        };
    }

    fn reconcile_configuration(&mut self) {
        if self.pots_active() {
            self.button.long_click_reset();
        }

        if self.button.long_clicked() {
            self.configuration_state = ConfigurationState::Active;
        } else if matches!(self.configuration_state, ConfigurationState::Active)
            && self.button.clicked()
        {
            self.configuration_state = ConfigurationState::Inactive;
        }

        if matches!(self.configuration_state, ConfigurationState::Active) && self.pot1.active() {
            const OPTIONS: i32 = 4;
            let scale = f32::powi(2.0, OPTIONS);
            let config = (self.pot1.value() * scale - 0.01) as u8;
            self.parameters.config = Config::from(config);
        }
    }

    fn reconcile_calibration(&mut self) {
        if matches!(self.calibration_target, CalibrationTarget::Cv1) && self.cv1.was_unplugged() {
            self.calibration_target = CalibrationTarget::None;
            self.calibration_state = CalibrationState::Inactive;
        }

        if matches!(self.calibration_target, CalibrationTarget::Cv2) && self.cv2.was_unplugged() {
            self.calibration_target = CalibrationTarget::None;
            self.calibration_state = CalibrationState::Inactive;
        }

        if matches!(self.calibration_target, CalibrationTarget::Cv5) && self.cv5.was_unplugged() {
            self.calibration_target = CalibrationTarget::None;
            self.calibration_state = CalibrationState::Inactive;
        }

        match self.calibration_state {
            CalibrationState::Inactive => {
                if self.button.active() {
                    if self.cv1.was_plugged() {
                        self.calibration_target = CalibrationTarget::Cv1;
                        self.calibration_state = CalibrationState::Entering;
                    } else if self.cv2.was_plugged() {
                        self.calibration_target = CalibrationTarget::Cv2;
                        self.calibration_state = CalibrationState::Entering;
                    } else if self.cv5.was_plugged() {
                        self.calibration_target = CalibrationTarget::Cv5;
                        self.calibration_state = CalibrationState::Entering;
                    }
                }
            }
            CalibrationState::Entering => {
                if !self.button.active() {
                    self.calibration_state = CalibrationState::CalibratingLow;
                }
            }
            CalibrationState::CalibratingLow => {
                if self.button.clicked() {
                    let c_a = match self.calibration_target {
                        CalibrationTarget::Cv1 => self.cv1.value() * VOCT_CV_RANGE,
                        CalibrationTarget::Cv2 => self.cv2.value() * VOCT_CV_RANGE,
                        CalibrationTarget::Cv5 => self.cv5.value() * VOCT_CV_RANGE,
                        _ => unreachable!(),
                    };
                    self.calibration_state = CalibrationState::CalibratingHigh(c_a);
                }
            }
            CalibrationState::CalibratingHigh(c_a) => {
                if self.button.clicked() {
                    let c_b = match self.calibration_target {
                        CalibrationTarget::Cv1 => self.cv1.value() * VOCT_CV_RANGE,
                        CalibrationTarget::Cv2 => self.cv2.value() * VOCT_CV_RANGE,
                        CalibrationTarget::Cv5 => self.cv5.value() * VOCT_CV_RANGE,
                        _ => unreachable!(),
                    };
                    self.calibration_state =
                        if self.calibrate(self.calibration_target, c_a, c_b).is_ok() {
                            CalibrationState::Succeeded
                        } else {
                            CalibrationState::Failed
                        };
                }
            }
            CalibrationState::Succeeded => {
                self.calibration_state = CalibrationState::Inactive;
                if let CalibrationTarget::Cv5 = self.calibration_target {
                    self.parameters.chord_quantization = true;
                }
            }
            CalibrationState::Failed => {
                self.calibration_state = CalibrationState::Inactive;
            }
        }
    }

    fn calibrate(&mut self, target: CalibrationTarget, c_a: f32, c_b: f32) -> Result<(), ()> {
        assert!(!matches!(target, CalibrationTarget::None));

        if let Ok((calibration_ratio, calibration_offset)) = calculate_calibration(c_a, c_b) {
            match target {
                CalibrationTarget::Cv1 => {
                    self.parameters.cv1_calibration_ratio = calibration_ratio;
                    self.parameters.cv1_calibration_offset = calibration_offset;
                }
                CalibrationTarget::Cv2 => {
                    self.parameters.cv2_calibration_ratio = calibration_ratio;
                    self.parameters.cv2_calibration_offset = calibration_offset;
                }
                CalibrationTarget::Cv5 => {
                    self.parameters.cv5_calibration_ratio = calibration_ratio;
                    self.parameters.cv5_calibration_offset = calibration_offset;
                }
                _ => unreachable!(),
            }
            return Ok(());
        }

        Err(())
    }

    fn active_submenu(&self) -> Submenu {
        if !matches!(self.calibration_state, CalibrationState::Inactive) {
            Submenu::Calibration
        } else if !matches!(self.configuration_state, ConfigurationState::Inactive) {
            Submenu::Configuration
        } else {
            Submenu::None
        }
    }

    fn reconcile_solo_quantization(&mut self) {
        if self.button.active() && self.cv4.was_plugged() {
            self.parameters.solo_quantization = !self.parameters.solo_quantization;
        }
    }

    pub fn solo_quantization(&self) -> bool {
        self.parameters.solo_quantization
    }

    pub fn overdrive(&self) -> bool {
        self.parameters.config.overdrive()
    }

    pub fn modes_ordered_by_brightness(&self) -> bool {
        self.parameters.config.modes_ordered_by_brightness()
    }

    pub fn mode_controlled_by_detune_cv(&self) -> bool {
        self.parameters.config.mode_controlled_by_detune_cv()
    }

    pub fn tonic_controlled_by_solo_cv(&self) -> bool {
        self.parameters.config.tonic_controlled_by_solo_cv()
    }

    fn cv1_sample_to_voct(&self, transposed_sample: f32) -> f32 {
        let voct = transposed_sample * VOCT_CV_RANGE;
        voct * self.parameters.cv1_calibration_ratio + self.parameters.cv1_calibration_offset
    }

    fn cv2_sample_to_voct(&self, transposed_sample: f32) -> f32 {
        let voct = transposed_sample * VOCT_CV_RANGE;
        voct * self.parameters.cv2_calibration_ratio + self.parameters.cv2_calibration_offset
    }

    fn cv5_sample_to_voct(&self, transposed_sample: f32) -> f32 {
        let voct = transposed_sample * VOCT_CV_RANGE;
        voct * self.parameters.cv5_calibration_ratio + self.parameters.cv5_calibration_offset
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
