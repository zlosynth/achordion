pub const PROBE_SEQUENCE: [bool; 32] = [
    true, true, false, true, false, false, true, false, true, true, true, false, true, false,
    false, false, false, false, false, true, true, false, true, false, true, true, true, false,
    false, false, false, true,
];

// TODO: Pass the sequence to this
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

// TODO: Replace 32 by len of the sequence
pub struct ProbeDetector<'a> {
    sequence: &'a [bool],
    sequence_sum: i32,
    position: usize,
    queue: [bool; 32],
}

impl<'a> ProbeDetector<'a> {
    pub fn new(sequence: &'a [bool]) -> Self {
        Self {
            sequence,
            sequence_sum: sequence.iter().map(|b| if *b { 1 } else { 0 }).sum(),
            position: 0,
            queue: [false; 32],
        }
    }

    pub fn write(&mut self, value: bool) {
        self.queue[self.position] = value;
        self.position = (self.position + 1) % 32;
    }

    pub fn connected(&self) -> bool {
        let sum: i32 = self.queue.iter().map(|b| if *b { 1 } else { 0 }).sum();
        if sum < self.sequence_sum - 2 || sum > self.sequence_sum + 2 {
            return false;
        }

        for start in 0..self.sequence.len() {
            let mut unmatching = 0;

            for i in 0..self.sequence.len() {
                if self.sequence[(start + i) % self.sequence.len()]
                    != self.queue[(self.position + i + 1) % 32]
                {
                    unmatching += 1;

                    if unmatching > 2 {
                        break;
                    }
                }
            }

            if unmatching <= 2 {
                return true;
            }
        }

        false
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
        let sequence = [true, true, false, false, false];
        let mut detector = ProbeDetector::new(&sequence);

        detector.write(sequence[0]);
        detector.write(sequence[1]);
        detector.write(sequence[2]);
        detector.write(sequence[3]);
        detector.write(sequence[4]);

        assert!(detector.connected());
    }

    #[test]
    fn fully_matching_sequence_offset() {
        let sequence = [true, true, false, false, false];
        let mut detector = ProbeDetector::new(&sequence);

        detector.write(sequence[3]);
        detector.write(sequence[4]);
        detector.write(sequence[0]);
        detector.write(sequence[1]);
        detector.write(sequence[2]);

        assert!(detector.connected());
    }

    #[test]
    fn partially_matching_sequence() {
        let sequence = [true, true, false, false, false];
        let mut detector = ProbeDetector::new(&sequence);

        detector.write(sequence[0]);
        detector.write(!sequence[1]);
        detector.write(sequence[2]);
        detector.write(sequence[3]);
        detector.write(sequence[4]);

        assert!(detector.connected());
    }

    #[test]
    fn partially_matching_sequence_offset() {
        let sequence = [true, true, false, false, false];
        let mut detector = ProbeDetector::new(&sequence);

        detector.write(sequence[2]);
        detector.write(sequence[3]);
        detector.write(sequence[4]);
        detector.write(sequence[0]);
        detector.write(!sequence[1]);

        assert!(detector.connected());
    }

    #[test]
    fn unmatching_sequence_with_matching_sum() {
        let sequence = [true, false, true, false, true, false, false];
        let mut detector = ProbeDetector::new(&sequence);

        detector.write(!sequence[0]);
        detector.write(!sequence[1]);
        detector.write(!sequence[2]);
        detector.write(!sequence[3]);
        detector.write(sequence[4]);
        detector.write(sequence[5]);
        detector.write(sequence[6]);

        assert!(!detector.connected());
    }

    #[test]
    fn unmatching_sequence_with_matching_sum_offset() {
        let sequence = [true, false, true, false, true, false, false];
        let mut detector = ProbeDetector::new(&sequence);

        detector.write(!sequence[2]);
        detector.write(!sequence[3]);
        detector.write(sequence[4]);
        detector.write(sequence[5]);
        detector.write(sequence[6]);
        detector.write(!sequence[0]);
        detector.write(!sequence[1]);

        assert!(!detector.connected());
    }

    #[test]
    fn unmatching_sequence_with_sum_below() {
        let sequence = [true, true, true, true];
        let mut detector = ProbeDetector::new(&sequence);

        detector.write(false);
        detector.write(false);
        detector.write(false);
        detector.write(false);

        assert!(!detector.connected());
    }

    #[test]
    fn unmatching_sequence_with_sum_above() {
        let sequence = [false, false, false, false];
        let mut detector = ProbeDetector::new(&sequence);

        detector.write(true);
        detector.write(true);
        detector.write(true);
        detector.write(true);

        assert!(!detector.connected());
    }
}
