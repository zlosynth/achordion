#[allow(unused_imports)]
use micromath::F32Ext;

use super::wavetable::Wavetable;

pub struct Oscillator<'a> {
    pub frequency: f32,
    phase: f32,
    phase_2: f32,
    phase_3: f32,
    phase_4: f32,
    phase_5: f32,
    sample_rate: f32,
    wavetable: &'a Wavetable<'a>,
}

impl<'a> Oscillator<'a> {
    pub fn new(wavetable: &'a Wavetable, sample_rate: u32) -> Self {
        Self {
            frequency: 0.0,
            phase: 0.0,
            phase_2: 0.0,
            phase_3: 0.0,
            phase_4: 0.0,
            phase_5: 0.0,
            sample_rate: sample_rate as f32,
            wavetable,
        }
    }

    pub fn populate(&mut self, buffer: &mut [u16]) {
        // TODO: In second stage optimization, frequency should be treated as integer too
        let interval_in_samples = self.frequency / self.sample_rate;
        let interval_in_samples_2 = (self.frequency * 1.5) / self.sample_rate;
        let interval_in_samples_3 = (self.frequency * 1.5 * 1.5) / self.sample_rate;
        let interval_in_samples_4 = (self.frequency * 0.5) / self.sample_rate;
        let interval_in_samples_5 = (self.frequency * 0.25) / self.sample_rate;
        let band_wavetable = self.wavetable.band(self.frequency);
        for x in buffer.iter_mut() {
            *x = band_wavetable.read(self.phase);
            *x += band_wavetable.read(self.phase_2);
            *x += band_wavetable.read(self.phase_3);
            *x += band_wavetable.read(self.phase_4);
            *x += band_wavetable.read(self.phase_5);
            *x /= 5;
            self.phase += interval_in_samples;
            self.phase_2 += interval_in_samples_2;
            self.phase_3 += interval_in_samples_3;
            self.phase_4 += interval_in_samples_4;
            self.phase_4 += interval_in_samples_5;
            // TODO: Could be dropped with u32 to encode this, will overflow back
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
            if self.phase_2 >= 1.0 {
                self.phase_2 -= 1.0;
            }
            if self.phase_3 >= 1.0 {
                self.phase_3 -= 1.0;
            }
            if self.phase_4 >= 1.0 {
                self.phase_4 -= 1.0;
            }
            if self.phase_5 >= 1.0 {
                self.phase_5 -= 1.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WAVEFORM: [u16; 8] = [8, 10, 12, 14, 0, 2, 4, 6];
    const FACTORS: [&[u16]; 1] = [&WAVEFORM];
    const SAMPLE_RATE: u32 = 8;

    lazy_static! {
        static ref WAVETABLE: Wavetable<'static> = Wavetable::new(&FACTORS, SAMPLE_RATE);
    }

    #[test]
    fn initialize() {
        let _oscillator = Oscillator::new(&WAVETABLE, SAMPLE_RATE);
    }

    #[test]
    fn populate() {
        let mut oscillator = Oscillator::new(&WAVETABLE, SAMPLE_RATE);
        let mut buffer = [0; 8];

        oscillator.frequency = 1.0;
        oscillator.populate(&mut buffer);
        assert_eq!(buffer, WAVEFORM);

        oscillator.frequency = 2.0;
        oscillator.populate(&mut buffer);
        assert_eq!(buffer, [8, 12, 0, 4, 8, 12, 0, 4]);
    }

    #[test]
    fn interpolation() {
        let mut oscillator = Oscillator::new(&WAVETABLE, SAMPLE_RATE);
        let mut buffer = [0; 8];

        oscillator.frequency = 0.5;
        oscillator.populate(&mut buffer);
        assert_eq!(buffer, [8, 9, 10, 11, 12, 13, 14, 7]);
    }
}
