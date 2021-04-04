pub struct Oscillator<'a> {
    pub frequency: f32,
    phase: f32,
    sample_rate: f32,
    wavetable: &'a [u16],
}

impl<'a> Oscillator<'a> {
    pub fn new(wavetable: &'a [u16], sample_rate: u32) -> Self {
        Self {
            frequency: 0.0,
            phase: 0.0,
            sample_rate: sample_rate as f32,
            wavetable,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WAVETABLE: [u16; 8] = [4, 5, 6, 7, 0, 1, 2, 3];
    const SAMPLE_RATE: u32 = 44100;

    #[test]
    fn initialize() {
        let _oscillator = Oscillator::new(&WAVETABLE, SAMPLE_RATE);
    }
}
