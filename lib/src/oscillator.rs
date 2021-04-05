#[allow(unused_imports)]
use micromath::F32Ext;

pub struct Oscillator<'a> {
    pub frequency: f32,
    phase: f32,
    sample_rate: f32,
    wavetable_factors: &'a [&'a [u16]],
}

impl<'a> Oscillator<'a> {
    pub fn new(wavetable_factors: &'a [&[u16]], sample_rate: u32) -> Self {
        Self {
            frequency: 0.0,
            phase: 0.0,
            sample_rate: sample_rate as f32,
            wavetable_factors,
        }
    }

    pub fn populate(&mut self, buffer: &mut [u16]) {
        let interval_in_samples = self.frequency / self.sample_rate;

        let factor = {
            let mut factor = 0;
            let mut block = self.sample_rate as f32 / 4.0;
            while self.frequency < block && factor < self.wavetable_factors.len() - 1 {
                block /= 2.0;
                factor += 1;
            }
            factor
        };
        let wavetable = self.wavetable_factors[factor];

        for x in buffer.iter_mut() {
            let position = self.phase * wavetable.len() as f32;
            *x = linear_interpolation(wavetable, position);
            self.phase += interval_in_samples;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
        }
    }
}

// TODO: Bench and optimize
pub fn linear_interpolation(data: &[u16], position: f32) -> u16 {
    let index = position as usize;
    let remainder = position.fract();

    let value = data[index];
    let delta_to_next = if index == (data.len() - 1) {
        data[0] as i32 - data[index] as i32
    } else {
        data[index + 1] as i32 - data[index] as i32
    };

    (value as f32 + delta_to_next as f32 * remainder) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    const WAVETABLE: [u16; 8] = [8, 10, 12, 14, 0, 2, 4, 6];
    const FACTORS: [&[u16]; 1] = [&WAVETABLE];
    const SAMPLE_RATE: u32 = 8;

    #[test]
    fn initialize() {
        let _oscillator = Oscillator::new(&FACTORS, SAMPLE_RATE);
    }

    #[test]
    fn populate() {
        let mut oscillator = Oscillator::new(&FACTORS, SAMPLE_RATE);
        let mut buffer = [0; 8];

        oscillator.frequency = 1.0;
        oscillator.populate(&mut buffer);
        assert_eq!(buffer, WAVETABLE);

        oscillator.frequency = 2.0;
        oscillator.populate(&mut buffer);
        assert_eq!(buffer, [8, 12, 0, 4, 8, 12, 0, 4]);
    }

    #[test]
    fn interpolation() {
        let mut oscillator = Oscillator::new(&FACTORS, SAMPLE_RATE);
        let mut buffer = [0; 8];

        oscillator.frequency = 0.5;
        oscillator.populate(&mut buffer);
        assert_eq!(buffer, [8, 9, 10, 11, 12, 13, 14, 7]);
    }
}
