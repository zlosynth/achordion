#[allow(unused_imports)]
use micromath::F32Ext;

use super::wavetable::Wavetable;

// With 44800 hz, it takes 150 cycles to fade in, 3 ms. This deals with most of
// the pops between chords with sine wavetable, while not feeling delayed.
const STEPS: f32 = 150.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Amplitude {
    Stable(f32),
    Traveling {
        step: f32,
        current: f32,
        target: f32,
    },
}
use Amplitude::*;

pub struct Oscillator<'a> {
    pub frequency: f32,
    pub phase: f32,
    pub wavetable_bank: &'a [Wavetable<'a>],
    previous_wavetable: Option<f32>,
    wavetable: f32,
    amplitude: Amplitude,
    sample_rate: f32,
}

impl<'a> Oscillator<'a> {
    pub fn new(wavetable_bank: &'a [Wavetable], sample_rate: u32) -> Self {
        assert!(!wavetable_bank.is_empty());
        Self {
            frequency: 20.0,
            phase: 0.0,
            sample_rate: sample_rate as f32,
            previous_wavetable: None,
            wavetable: 0.0,
            amplitude: Stable(0.0),
            wavetable_bank,
        }
    }

    pub fn set_wavetable(&mut self, wavetable: f32) {
        if self.previous_wavetable.is_none() {
            self.previous_wavetable = Some(wavetable);
        }
        self.wavetable = wavetable;
    }

    pub fn wavetable(&self) -> f32 {
        self.wavetable
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        match self.amplitude {
            Stable(current) => {
                // There is no processing involved, so the float should be identical
                #[allow(clippy::float_cmp)]
                if amplitude != current {
                    self.amplitude = Traveling {
                        step: (amplitude - current) / STEPS,
                        current,
                        target: amplitude,
                    }
                }
            }
            Traveling {
                current, target, ..
            } => {
                // There is no processing involved, so the float should be identical
                #[allow(clippy::float_cmp)]
                if amplitude != target {
                    self.amplitude = Traveling {
                        step: (amplitude - current) / STEPS,
                        current,
                        target: amplitude,
                    }
                }
            }
        }
    }

    pub fn populate_add(&mut self, buffer: &mut [f32]) {
        macro_rules! lookup_wavetable {
            ( $wavetable:expr ) => {{
                let scaled_wavetable = $wavetable * (self.wavetable_bank.len() - 1) as f32;
                let wavetable_a_index = scaled_wavetable as usize;
                let wavetable_b_index = if wavetable_a_index == self.wavetable_bank.len() - 1 {
                    wavetable_a_index
                } else {
                    wavetable_a_index + 1
                };

                let band_wavetable_a = self.wavetable_bank[wavetable_a_index].band(self.frequency);
                let band_wavetable_b = self.wavetable_bank[wavetable_b_index].band(self.frequency);
                let xfade = scaled_wavetable - wavetable_a_index as f32;

                (band_wavetable_a, band_wavetable_b, xfade)
            }};
        }

        let (previous_band_wavetable_a, previous_band_wavetable_b, previous_xfade) = {
            let wavetable = if let Some(previous_wavetable) = self.previous_wavetable {
                previous_wavetable
            } else {
                self.wavetable
            };

            lookup_wavetable!(wavetable)
        };

        let (current_band_wavetable_a, current_band_wavetable_b, current_xfade) =
            { lookup_wavetable!(self.wavetable) };

        self.previous_wavetable = Some(self.wavetable);

        let interval_in_samples = self.frequency / self.sample_rate;
        let buffer_len = buffer.len() as f32;

        for (i, x) in buffer.iter_mut().enumerate() {
            let preparation = current_band_wavetable_a.prepare(self.phase);

            let previous_value = {
                let value_a = previous_band_wavetable_a.read(&preparation);
                let value_b = previous_band_wavetable_b.read(&preparation);
                value_a * (1.0 - previous_xfade) + value_b * previous_xfade
            };

            let current_value = {
                let value_a = current_band_wavetable_a.read(&preparation);
                let value_b = current_band_wavetable_b.read(&preparation);
                value_a * (1.0 - current_xfade) + value_b * current_xfade
            };

            let mix = i as f32 / buffer_len;

            *x += (previous_value * (1.0 - mix) + current_value * mix) * self.amplitude();

            self.phase += interval_in_samples;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
        }
    }

    fn amplitude(&mut self) -> f32 {
        match self.amplitude {
            Stable(current) => current,
            Traveling {
                step,
                current,
                target,
            } => {
                let mut new_value = current + step;
                self.amplitude =
                    if (step < 0.0 && new_value < target) || (step > 0.0 && new_value > target) {
                        new_value = target;
                        Stable(new_value)
                    } else {
                        Traveling {
                            step,
                            current: new_value,
                            target,
                        }
                    };
                new_value
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TENTH: f32 = 2.0 / 10.0;
    const WAVEFORM: [f32; 11] = [
        -1.0 + 0.0 * TENTH,
        -1.0 + 1.0 * TENTH,
        -1.0 + 2.0 * TENTH,
        -1.0 + 3.0 * TENTH,
        -1.0 + 4.0 * TENTH,
        -1.0 + 5.0 * TENTH,
        -1.0 + 6.0 * TENTH,
        -1.0 + 7.0 * TENTH,
        -1.0 + 8.0 * TENTH,
        -1.0 + 9.0 * TENTH,
        -1.0 + 10.0 * TENTH,
    ];

    const FACTORS: [&[f32]; 1] = [&WAVEFORM];
    const SAMPLE_RATE: u32 = 22;

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
        oscillator.amplitude = Stable(1.0);
        let step = 1.0 / 10.0;

        let mut buffer = [0.0; 22];
        oscillator.frequency = 1.0;
        oscillator.populate_add(&mut buffer);
        for i in 0..11 {
            assert_relative_eq!(
                buffer[i],
                -1.0 + i as f32 * step * oscillator.frequency,
                epsilon = 0.001
            );
        }

        let mut buffer = [0.0; 22];
        oscillator.frequency = 2.0;
        oscillator.populate_add(&mut buffer);
        for i in 0..11 {
            assert_relative_eq!(
                buffer[i],
                -1.0 + i as f32 * step * oscillator.frequency,
                epsilon = 0.001
            );
        }
        for i in 0..11 {
            assert_relative_eq!(
                buffer[i + 11],
                -1.0 + i as f32 * step * oscillator.frequency,
                epsilon = 0.001
            );
        }
    }

    #[test]
    fn interpolation() {
        let mut oscillator = Oscillator::new(&WAVETABLE_BANK[..], SAMPLE_RATE);
        oscillator.amplitude = Stable(1.0);
        let mut buffer = [0.0; 11];
        let step = 1.0 / 10.0;

        oscillator.frequency = 0.5;
        oscillator.populate_add(&mut buffer);

        for i in 0..11 {
            assert_relative_eq!(
                buffer[i],
                -1.0 + i as f32 * step * oscillator.frequency,
                epsilon = 0.001
            );
        }
    }

    #[test]
    fn fade_out_after_150_samples() {
        let mut oscillator = Oscillator::new(&WAVETABLE_BANK[..], SAMPLE_RATE);
        oscillator.amplitude = Stable(1.0);
        oscillator.frequency = 1.0;
        let step = 1.0 / 10.0;

        let mut buffer = [0.0; 22];
        oscillator.populate_add(&mut buffer);
        for i in 0..11 {
            assert_relative_eq!(
                buffer[i],
                -1.0 + i as f32 * step * oscillator.frequency,
                epsilon = 0.001
            );
        }

        oscillator.set_amplitude(0.0);
        let mut buffer = [0.0; 150];
        oscillator.populate_add(&mut buffer);
        for i in 0..50 {
            if i % 10 == 0 {
                continue;
            }
            let unsilenced = -1.0 + i as f32 * step * oscillator.frequency;
            assert!(buffer[i].abs() > 0.0);
            assert!(buffer[i].abs() < unsilenced.abs());
        }

        let mut buffer = [0.0; 12];
        oscillator.populate_add(&mut buffer);
        for i in 0..12 {
            assert_relative_eq!(buffer[i], 0.0, epsilon = 0.001);
        }
    }
}
