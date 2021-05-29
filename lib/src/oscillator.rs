#[allow(unused_imports)]
use micromath::F32Ext;

use super::wavetable::Wavetable;

pub struct Oscillator<'a> {
    pub frequency: f32,
    pub wavetable: f32,
    pub phase: f32,
    pub wavetable_bank: &'a [Wavetable<'a>],
    sample_rate: f32,
}

impl<'a> Oscillator<'a> {
    pub fn new(wavetable_bank: &'a [Wavetable], sample_rate: u32) -> Self {
        assert!(!wavetable_bank.is_empty());
        Self {
            frequency: 0.0,
            phase: 0.0,
            sample_rate: sample_rate as f32,
            wavetable: 0.0,
            wavetable_bank,
        }
    }

    pub fn populate_add(&mut self, buffer: &mut [u16], amplitude: f32) {
        let scaled_wavetable = self.wavetable * (self.wavetable_bank.len() - 1) as f32;
        let wavetable_a_index = scaled_wavetable as usize;
        let wavetable_b_index = if wavetable_a_index == self.wavetable_bank.len() - 1 {
            wavetable_a_index
        } else {
            wavetable_a_index + 1
        };

        let xfade = scaled_wavetable - wavetable_a_index as f32;

        let band_wavetable_a = self.wavetable_bank[wavetable_a_index].band(self.frequency);
        let band_wavetable_b = self.wavetable_bank[wavetable_b_index].band(self.frequency);

        let interval_in_samples = self.frequency / self.sample_rate;

        for x in buffer.iter_mut() {
            let value_a = band_wavetable_a.read(self.phase);
            let value_b = band_wavetable_b.read(self.phase);

            let value = value_a as f32 * (1.0 - xfade) + value_b as f32 * xfade;
            *x += (value * amplitude) as u16;

            self.phase += interval_in_samples;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
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
        static ref WAVETABLE_BANK: [Wavetable<'static>; 1] =
            [Wavetable::new(&FACTORS, SAMPLE_RATE)];
    }

    #[test]
    fn initialize() {
        let _oscillator = Oscillator::new(&WAVETABLE_BANK[..], SAMPLE_RATE);
    }

    #[test]
    fn populate() {
        let mut oscillator = Oscillator::new(&WAVETABLE_BANK[..], SAMPLE_RATE);

        let mut buffer = [0; 8];
        oscillator.frequency = 1.0;
        oscillator.populate_add(&mut buffer, 1.0);
        assert_eq!(buffer, WAVEFORM);

        let mut buffer = [0; 8];
        oscillator.frequency = 2.0;
        oscillator.populate_add(&mut buffer, 1.0);
        assert_eq!(buffer, [8, 12, 0, 4, 8, 12, 0, 4]);
    }

    #[test]
    fn interpolation() {
        let mut oscillator = Oscillator::new(&WAVETABLE_BANK[..], SAMPLE_RATE);
        let mut buffer = [0; 8];

        oscillator.frequency = 0.5;
        oscillator.populate_add(&mut buffer, 1.0);
        assert_eq!(buffer, [8, 9, 10, 11, 12, 13, 14, 7]);
    }
}
