#[allow(unused_imports)]
use micromath::F32Ext;

use super::wavetable::Wavetable;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Enabling(f32),
    Enabled,
    Disabling(f32),
    Disabled,
}
use State::*;

pub struct Oscillator<'a> {
    pub frequency: f32,
    pub wavetable: f32,
    pub phase: f32,
    pub wavetable_bank: &'a [Wavetable<'a>],
    state: State,
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
            state: Enabled,
            wavetable_bank,
        }
    }

    pub fn enable(&mut self) {
        match self.state {
            Disabled => self.state = Enabling(0.0),
            Disabling(value) => self.state = Enabling(value),
            _ => (),
        }
    }

    pub fn disable(&mut self) {
        match self.state {
            Enabled => self.state = Disabling(1.0),
            Enabling(value) => self.state = Disabling(value),
            _ => (),
        }
    }

    pub fn populate_add(&mut self, buffer: &mut [f32], amplitude: f32) {
        if self.state == Disabled {
            return;
        }

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

        macro_rules! populate_buffer {
            ( $self:ident, $fader:ident ) => {
                for x in buffer.iter_mut() {
                    let value_a = band_wavetable_a.read(self.phase);
                    let value_b = band_wavetable_b.read(self.phase);

                    let value = value_a * (1.0 - xfade) + value_b * xfade;

                    *x += value * amplitude * $self.$fader();

                    self.phase += interval_in_samples;
                    if self.phase >= 1.0 {
                        self.phase -= 1.0;
                    }
                }
            };
        }

        match self.state {
            Enabled => {
                populate_buffer!(self, no_fade);
            }
            Enabling(_) | Disabling(_) => {
                populate_buffer!(self, step_fade);
            }
            _ => unreachable!(),
        }
    }

    fn no_fade(&mut self) -> f32 {
        1.0
    }

    fn step_fade(&mut self) -> f32 {
        // With 44800 hz, it takes 50 cycles to fade in, 1 ms
        const STEP: f32 = 0.02;

        match self.state {
            Enabled => 1.0,
            Disabled => 0.0,
            Enabling(value) => {
                let mut new_value = value + STEP;
                if new_value > 1.0 {
                    new_value = 1.0;
                    self.state = Enabled;
                } else {
                    self.state = Enabling(new_value);
                }
                new_value
            }
            Disabling(value) => {
                let mut new_value = value - STEP;
                if new_value < 0.0 {
                    new_value = 0.0;
                    self.state = Disabled;
                } else {
                    self.state = Disabling(new_value);
                }
                new_value
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TENTH: u16 = u16::MAX / 10;
    const WAVEFORM: [u16; 11] = [
        0 * TENTH,
        1 * TENTH,
        2 * TENTH,
        3 * TENTH,
        4 * TENTH,
        5 * TENTH,
        6 * TENTH,
        7 * TENTH,
        8 * TENTH,
        9 * TENTH,
        10 * TENTH,
    ];

    const FACTORS: [&[u16]; 1] = [&WAVEFORM];
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
        let step = 1.0 / 10.0;

        let mut buffer = [0.0; 22];
        oscillator.frequency = 1.0;
        oscillator.populate_add(&mut buffer, 1.0);
        for i in 0..11 {
            assert_relative_eq!(
                buffer[i],
                -1.0 + i as f32 * step * oscillator.frequency,
                epsilon = 0.001
            );
        }

        let mut buffer = [0.0; 22];
        oscillator.frequency = 2.0;
        oscillator.populate_add(&mut buffer, 1.0);
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
        let mut buffer = [0.0; 11];
        let step = 1.0 / 10.0;

        oscillator.frequency = 0.5;
        oscillator.populate_add(&mut buffer, 1.0);

        for i in 0..11 {
            assert_relative_eq!(
                buffer[i],
                -1.0 + i as f32 * step * oscillator.frequency,
                epsilon = 0.001
            );
        }
    }

    #[test]
    fn fade_out_after_50_samples() {
        let mut oscillator = Oscillator::new(&WAVETABLE_BANK[..], SAMPLE_RATE);
        oscillator.frequency = 1.0;
        let step = 1.0 / 10.0;

        let mut buffer = [0.0; 22];
        oscillator.populate_add(&mut buffer, 1.0);
        for i in 0..11 {
            assert_relative_eq!(
                buffer[i],
                -1.0 + i as f32 * step * oscillator.frequency,
                epsilon = 0.001
            );
        }

        oscillator.disable();
        let mut buffer = [0.0; 50];
        oscillator.populate_add(&mut buffer, 1.0);
        for i in 0..50 {
            if i % 10 == 0 {
                continue;
            }
            let unsilenced = -1.0 + i as f32 * step * oscillator.frequency;
            assert!(buffer[i].abs() > 0.0);
            assert!(buffer[i].abs() < unsilenced.abs());
        }

        let mut buffer = [0.0; 12];
        oscillator.populate_add(&mut buffer, 1.0);
        for i in 0..12 {
            assert_relative_eq!(buffer[i], 0.0, epsilon = 0.001);
        }
    }
}
