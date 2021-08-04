#[allow(unused_imports)]
use micromath::F32Ext;

use daisy::hal;
use daisy_bsp as daisy;

use hal::adc::{Adc, Enabled};
use hal::pac::ADC1;

use achordion_lib::display::State as DisplayState;
use achordion_lib::store::Parameters;

use crate::system::Button;
use crate::system::Probe;
use crate::system::{Cv1, Cv2, Cv3, Cv4, Cv5, Cv6};
use crate::system::{Led1, Led2, Led3, Led4, Led5, Led6, Led7, Led8};
use crate::system::{Pot1, Pot2, Pot3, Pot4};

pub struct Interface {
    adc: Adc<ADC1, Enabled>,

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

    led1: Led1,
    led2: Led2,
    led3: Led3,
    led4: Led4,
    led5: Led5,
    led6: Led6,
    led7: Led7,
    led8: Led8,

    parameters: Parameters,

    last_note_pot_reading: f32,
    last_wavetable_pot_reading: f32,
    last_scale_root_pot_reading: f32,
    last_chord_pot_reading: f32,
    last_scale_mode_pot_reading: f32,
}

impl Interface {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        adc: Adc<ADC1, Enabled>,
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
        led1: Led1,
        led2: Led2,
        led3: Led3,
        led4: Led4,
        led5: Led5,
        led6: Led6,
        led7: Led7,
        led8: Led8,
        parameters: Parameters,
    ) -> Self {
        Self {
            adc,
            button,

            pot1,
            pot2,
            pot3,
            pot4,

            cv1,
            cv2,
            cv3,
            cv4,
            cv5,
            cv6,

            probe,

            led1,
            led2,
            led3,
            led4,
            led5,
            led6,
            led7,
            led8,

            parameters,

            // These values are used to cache the last read value while the pot
            // is in its alternative mode (depending on the button state).
            last_note_pot_reading: 0.0,
            last_wavetable_pot_reading: 0.0,
            last_scale_root_pot_reading: 0.0,
            last_chord_pot_reading: 0.0,
            last_scale_mode_pot_reading: 0.0,
        }
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
        self.pot1.sample(&mut self.adc);
        self.pot2.sample(&mut self.adc);
        self.pot3.sample(&mut self.adc);
        self.pot4.sample(&mut self.adc);

        self.cv1.sample(&mut self.adc);
        self.cv2.sample(&mut self.adc);
        self.cv3.sample(&mut self.adc);
        self.cv4.sample(&mut self.adc);
        self.cv5.sample(&mut self.adc);
        self.cv6.sample(&mut self.adc);

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

        // XXX: Temporary for testing
        self.reconcile_amplitude();
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
            let note = sample_to_voct(self.cv1.value());
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
            sample_to_voct(self.cv2.value())
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

        // XXX: CV control is disable, so it can be used as VCA for testing
        let cv = 0.0;
        // let cv = if self.cv3.connected() {
        //     self.cv3.value() * 2.0 - 1.0
        // } else {
        //     0.0
        // };

        self.parameters.scale_mode = cv + pot;
    }

    // XXX: Temporary for testing
    fn reconcile_amplitude(&mut self) {
        self.parameters.amplitude = if self.cv3.connected() {
            self.cv3.value()
        } else {
            1.0
        };
    }

    // XXX: Temporary for testing
    pub fn amplitude(&self) -> f32 {
        self.parameters.amplitude
    }

    pub fn set_display(&mut self, display_state: DisplayState) {
        self.led4.set(display_state.led1);
        self.led8.set(display_state.led2);
        self.led3.set(display_state.led3);
        self.led7.set(display_state.led4);
        self.led2.set(display_state.led5);
        self.led6.set(display_state.led6);
        self.led1.set(display_state.led7);
        self.led5.set(display_state.led_sharp);
    }
}

fn sample_to_voct(transposed_sample: f32) -> f32 {
    // V/OCT CV spans from -1.5 to 5.5 V.
    transposed_sample * 7.0 + 0.5
}
