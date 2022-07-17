#[allow(unused_imports)]
use micromath::F32Ext;

const EQULIBRIUM: [f32; 1] = [0.0];

pub struct Wavetable<'a> {
    niquist: f32,
    factors: &'a [&'a [f32]],
}

impl<'a> Wavetable<'a> {
    #[inline(always)]
    pub fn new(factors: &'a [&'a [f32]], sample_rate: u32) -> Self {
        Wavetable {
            niquist: sample_rate as f32 / 2.0,
            factors,
        }
    }

    pub fn band(&self, frequency: f32) -> BandWavetable {
        let (wavetable_a, wavetable_b, mix): (&[f32], &[f32], f32) = {
            let (factor, mix) = {
                let (factor, mix) = calculate_factor_and_mix(frequency, self.niquist);
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
    lower: &'a [f32],
    lower_len: f32,
    higher: &'a [f32],
    higher_len: f32,
    mix: f32,
    mix_remainder: f32,
}

pub struct Preparation {
    pub lower: SubPreparation,
    pub higher: SubPreparation,
}

pub struct SubPreparation {
    pub index: usize,
    pub next_index: usize,
    pub remainder: f32,
}

impl<'a> BandWavetable<'a> {
    fn new(lower: &'a [f32], higher: &'a [f32], mix: f32) -> Self {
        Self {
            lower,
            lower_len: lower.len() as f32,
            higher,
            higher_len: higher.len() as f32,
            mix,
            mix_remainder: 1.0 - mix,
        }
    }

    pub fn prepare(&self, phase: f32) -> Preparation {
        Preparation {
            lower: self.sub_prepare(self.lower.len(), self.lower_len, phase),
            higher: self.sub_prepare(self.higher.len(), self.higher_len, phase),
        }
    }

    fn sub_prepare(&self, len: usize, len_f32: f32, phase: f32) -> SubPreparation {
        let position = phase * len_f32;
        let index = position as usize;
        let next_index = if index == (len - 1) { 0 } else { index + 1 };
        let remainder = position - index as f32;
        SubPreparation {
            index,
            next_index,
            remainder,
        }
    }

    pub fn read(&self, preparation: &Preparation) -> f32 {
        let a = {
            let value = self.lower[preparation.lower.index];
            let delta_to_next = self.lower[preparation.lower.next_index] - value;
            value + delta_to_next * preparation.lower.remainder
        };
        let b = {
            let value = self.higher[preparation.higher.index];
            let delta_to_next = self.higher[preparation.higher.next_index] - value;
            value + delta_to_next * preparation.higher.remainder
        };

        linear_xfade(a, b, self.mix, self.mix_remainder)
    }
}

fn linear_xfade(a: f32, b: f32, mix: f32, mix_remainder: f32) -> f32 {
    debug_assert!((0.0..=1.0).contains(&mix));
    debug_assert!((0.0..=1.0).contains(&mix_remainder));
    debug_assert!((1.0 - mix - mix_remainder).abs() < 0.001);
    a * mix_remainder + b * mix
}

#[cfg(test)]
mod tests {
    use super::*;

    const WAVEFORM: [f32; 8] = [8.0, 10.0, 12.0, 14.0, 0.0, 2.0, 4.0, 6.0];
    const FACTORS: [&[f32]; 1] = [&WAVEFORM];
    const SAMPLE_RATE: u32 = 8;

    #[test]
    fn init_wavetable() {
        let _wavetable = Wavetable::new(&FACTORS, SAMPLE_RATE);
    }

    #[test]
    fn read_value() {
        let wavetable = Wavetable::new(&FACTORS, SAMPLE_RATE);

        let band_wavetable = wavetable.band(1.0);
        let first = band_wavetable.read(&band_wavetable.prepare(0.0));
        let second = band_wavetable.read(&band_wavetable.prepare(0.1));
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
        assert_eq!(linear_xfade(8.0, 4.0, 0.5, 0.5), 6.0);
    }

    #[test]
    fn linear_xfade_uneven() {
        assert_eq!(linear_xfade(10.0, 20.0, 0.2, 0.8), 12.0);
    }

    #[test]
    fn linear_xfade_left_side() {
        assert_eq!(linear_xfade(8.0, 4.0, 0.0, 1.0), 8.0);
    }

    #[test]
    fn linear_xfade_right_side() {
        assert_eq!(linear_xfade(8.0, 4.0, 1.0, 0.0), 4.0);
    }

    #[test]
    #[should_panic]
    fn linear_xfade_panics_on_x_below_zero() {
        linear_xfade(8.0, 4.0, -1.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn linear_xfade_panics_on_x_above_one() {
        linear_xfade(8.0, 4.0, 2.0, 0.0);
    }
}
