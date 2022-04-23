#[allow(unused_imports)]
use micromath::F32Ext;

use core::f32::consts::PI;

// Amplified by gain. Linear amplification up to max_linear, then starts soft
// clipping.
pub struct Overdrive {
    gain: f32,
    max_linear: f32,
}

impl Overdrive {
    pub fn new(gain: f32, max_linear: f32) -> Self {
        Self { gain, max_linear }
    }

    pub fn process(&self, value: f32) -> f32 {
        let gain = self.gain;
        let max_linear = self.max_linear;
        let non_linear = 1.0 - self.max_linear;

        if value < -max_linear / gain {
            (2.0 * non_linear / PI)
                * f32::atan(((gain * PI) / (2.0 * non_linear)) * (value + max_linear / gain))
                - max_linear
        } else if value > max_linear / gain {
            (2.0 * non_linear / PI)
                * f32::atan(((gain * PI) / (2.0 * non_linear)) * (value - max_linear / gain))
                + max_linear
        } else {
            value * gain
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let _overdrive = Overdrive::new(2.0, 0.8);
    }

    #[test]
    fn zero_stays_intact() {
        let overdrive = Overdrive::new(2.0, 0.8);
        assert_relative_eq!(overdrive.process(0.0), 0.0);
    }

    #[test]
    fn very_high_value_approaches_one() {
        let overdrive = Overdrive::new(2.0, 0.8);
        assert_relative_eq!(overdrive.process(1000.0), 1.0, epsilon = 0.01);
    }

    #[test]
    fn result_never_clips() {
        let overdrive = Overdrive::new(2.0, 0.8);
        assert!(overdrive.process(1.0) > -1.0);
        assert!(overdrive.process(1.0) < 1.0);
    }

    #[test]
    fn low_value_is_amplified_linearly() {
        let overdrive = Overdrive::new(2.0, 0.8);
        assert_relative_eq!(overdrive.process(-0.2), -0.4);
        assert_relative_eq!(overdrive.process(-0.1), -0.2);
        assert_relative_eq!(overdrive.process(0.1), 0.2);
        assert_relative_eq!(overdrive.process(0.2), 0.4);
    }
}
