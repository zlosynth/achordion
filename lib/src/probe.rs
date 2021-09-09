pub const PROBE_SEQUENCE: [bool; 32] = [
    true, true, false, true, false, false, true, false, true, true, true, false, true, false,
    false, false, false, false, false, true, true, false, true, false, true, true, true, false,
    false, false, false, true,
];

pub struct ProbeGenerator<'a> {
    sequence: &'a [bool],
    position: usize,
}

impl<'a> ProbeGenerator<'a> {
    pub fn new(sequence: &'a [bool]) -> Self {
        Self {
            sequence,
            position: 0,
        }
    }

    pub fn read(&mut self) -> bool {
        let value = self.sequence[self.position];
        self.position = (self.position + 1) % self.sequence.len();
        value
    }
}

pub struct ProbeDetector<'a, const N: usize> {
    sequence: &'a [bool],
    position: usize,
    queue: [bool; N],
    detected_cache: bool,
}

impl<'a, const N: usize> ProbeDetector<'a, N> {
    pub fn new(sequence: &'a [bool]) -> Self {
        assert!(sequence.len() == N);
        Self {
            sequence,
            position: 0,
            queue: [false; N],
            detected_cache: false,
        }
    }

    pub fn write(&mut self, value: bool) {
        self.queue[self.position] = value;
        self.position = (self.position + 1) % N;
    }

    pub fn detected(&mut self) -> bool {
        if self.position == 0 {
            let unmatched: u32 = self
                .queue
                .iter()
                .zip(self.sequence)
                .map(|(q, s)| if q == s { 0 } else { 1 })
                .sum();
            self.detected_cache = unmatched <= 2;
        }
        self.detected_cache
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_sequence() {
        let sequence = [true, false, false];
        let mut generator = ProbeGenerator::new(&sequence);

        assert_eq!(generator.read(), true);
        assert_eq!(generator.read(), false);
        assert_eq!(generator.read(), false);
        assert_eq!(generator.read(), true);
    }

    #[test]
    fn fully_matching_sequence() {
        const SEQUENCE: [bool; 5] = [true, true, false, false, false];
        const SEQUENCE_LEN: usize = SEQUENCE.len();
        let mut detector = ProbeDetector::<SEQUENCE_LEN>::new(&SEQUENCE);

        detector.write(SEQUENCE[0]);
        detector.write(SEQUENCE[1]);
        detector.write(SEQUENCE[2]);
        detector.write(SEQUENCE[3]);
        detector.write(SEQUENCE[4]);

        assert!(detector.detected());
    }

    #[test]
    fn partially_matching_sequence() {
        const SEQUENCE: [bool; 5] = [true, true, false, false, false];
        const SEQUENCE_LEN: usize = SEQUENCE.len();
        let mut detector = ProbeDetector::<SEQUENCE_LEN>::new(&SEQUENCE);

        detector.write(SEQUENCE[0]);
        detector.write(!SEQUENCE[1]);
        detector.write(SEQUENCE[2]);
        detector.write(SEQUENCE[3]);
        detector.write(SEQUENCE[4]);

        assert!(detector.detected());
    }

    #[test]
    fn unmatching_sequence() {
        const SEQUENCE: [bool; 7] = [true, false, true, false, true, false, false];
        const SEQUENCE_LEN: usize = SEQUENCE.len();
        let mut detector = ProbeDetector::<SEQUENCE_LEN>::new(&SEQUENCE);

        detector.write(!SEQUENCE[0]);
        detector.write(!SEQUENCE[1]);
        detector.write(!SEQUENCE[2]);
        detector.write(!SEQUENCE[3]);
        detector.write(SEQUENCE[4]);
        detector.write(SEQUENCE[5]);
        detector.write(SEQUENCE[6]);

        assert!(!detector.detected());
    }
}
