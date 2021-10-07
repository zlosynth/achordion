#[allow(unused_imports)]
use micromath::F32Ext;

const EQULIBRIUM: [u16; 1] = [2 << 14];

const TWO_POW_15: f32 = 32768.0;

pub struct Wavetable<'a> {
    sample_rate: u32,
    factors: &'a [&'a [u16]],
}

impl<'a> Wavetable<'a> {
    pub fn new(factors: &'a [&'a [u16]], sample_rate: u32) -> Self {
        Wavetable {
            sample_rate,
            factors,
        }
    }

    pub fn band(&self, frequency: f32) -> BandWavetable {
        let (wavetable_a, wavetable_b, mix): (&[u16], &[u16], f32) = {
            let niquist = self.sample_rate as f32 / 2.0;

            let (factor, mix) = {
                let (factor, mix) = calculate_factor_and_mix(frequency, niquist);
                if factor > self.factors.len() - 1 {
                    (self.factors.len() - 1, 0.0)
                } else {
                    (factor, mix)
                }
            };

            let wavetable_a = self.factors[factor];
            let wavetable_b = if factor == 0 {
                &EQULIBRIUM
            } else {
                self.factors[factor - 1]
            };

            (wavetable_a, wavetable_b, mix)
        };

        BandWavetable::new(wavetable_a, wavetable_b, mix)
    }
}

fn calculate_factor_and_mix(frequency: f32, niquist: f32) -> (usize, f32) {
    let mut factor = 0;
    let mut block = niquist / 2.0;
    while frequency < block {
        block /= 2.0;
        factor += 1;
    }
    let mix = (frequency - block) / block;
    (factor, mix)
}

pub struct BandWavetable<'a> {
    lower: &'a [u16],
    higher: &'a [u16],
    mix: f32,
}

impl<'a> BandWavetable<'a> {
    fn new(lower: &'a [u16], higher: &'a [u16], mix: f32) -> Self {
        Self { lower, higher, mix }
    }

    pub fn read(&self, phase: f32) -> f32 {
        let a = {
            let position = phase * self.lower.len() as f32;
            linear_interpolation(self.lower, position)
        };
        let b = {
            let position = phase * self.higher.len() as f32;
            linear_interpolation(self.higher, position)
        };

        linear_xfade(a, b, self.mix)
    }
}

fn linear_interpolation(data: &[u16], position: f32) -> f32 {
    let index = position as usize;
    let remainder = position - index as f32;

    let value = data[index];
    let delta_to_next = if index == (data.len() - 1) {
        data[0] as i32 - value as i32
    } else {
        data[index + 1] as i32 - value as i32
    };

    (value as f32 + delta_to_next as f32 * remainder) / TWO_POW_15 - 1.0
}

fn linear_xfade(a: f32, b: f32, mix: f32) -> f32 {
    debug_assert!((0.0..=1.0).contains(&mix));
    a * (1.0 - mix) + b * mix
}

#[cfg(test)]
mod tests {
    use super::*;

    const WAVEFORM: [u16; 8] = [8, 10, 12, 14, 0, 2, 4, 6];
    const FACTORS: [&[u16]; 1] = [&WAVEFORM];
    const SAMPLE_RATE: u32 = 8;

    #[test]
    fn init_wavetable() {
        let _wavetable = Wavetable::new(&FACTORS, SAMPLE_RATE);
    }

    #[test]
    fn read_value() {
        let wavetable = Wavetable::new(&FACTORS, SAMPLE_RATE);

        let band_wavetable = wavetable.band(1.0);
        let first = band_wavetable.read(0.0);
        let second = band_wavetable.read(0.1);
        assert!(second > first);
    }

    #[test]
    fn factor_and_mix_top_middle_of_top() {
        let (factor, mix) = calculate_factor_and_mix(7.0, 8.0);
        assert_eq!(factor, 0);
        assert_relative_eq!(mix, 0.75);
    }

    #[test]
    fn factor_and_mix_middle_of_second_factor() {
        let (factor, mix) = calculate_factor_and_mix(3.0, 8.0);
        assert_eq!(factor, 1);
        assert_relative_eq!(mix, 0.5);
    }

    #[test]
    fn linear_xfade_even() {
        assert_eq!(linear_xfade(8.0, 4.0, 0.5), 6.0);
    }

    #[test]
    fn linear_xfade_uneven() {
        assert_eq!(linear_xfade(10.0, 20.0, 0.2), 12.0);
    }

    #[test]
    fn linear_xfade_left_side() {
        assert_eq!(linear_xfade(8.0, 4.0, 0.0), 8.0);
    }

    #[test]
    fn linear_xfade_right_side() {
        assert_eq!(linear_xfade(8.0, 4.0, 1.0), 4.0);
    }

    #[test]
    #[should_panic]
    fn linear_xfade_panics_on_x_below_zero() {
        linear_xfade(8.0, 4.0, -1.0);
    }

    #[test]
    #[should_panic]
    fn linear_xfade_panics_on_x_above_one() {
        linear_xfade(8.0, 4.0, 2.0);
    }
}
