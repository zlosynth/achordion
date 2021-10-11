#[allow(unused_imports)]
use micromath::F32Ext;

use super::table::{MULTIPLE, START, WEIGHTING};

pub fn lookup(frequency: f32) -> f32 {
    let index = (MULTIPLE * f32::log10(frequency - START)) as usize;
    WEIGHTING[index.min(WEIGHTING.len() - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    const OFFSET: f32 = 38.2;

    fn db_to_amplitude(db: f32) -> f32 {
        f32::powf(10.0, (db - OFFSET) / 100.0)
    }

    #[test]
    fn lookup_below_range() {
        let weight = lookup(-1.0);
        assert_relative_eq!(weight, db_to_amplitude(38.2), max_relative = 0.01);
    }

    #[test]
    fn lookup_above_range() {
        let weight = lookup(30000.0);
        assert_relative_eq!(weight, db_to_amplitude(-11.1), max_relative = 0.01);
    }

    #[test]
    fn lookup_lowest() {
        let weight = lookup(0.0);
        assert_relative_eq!(weight, db_to_amplitude(38.2), max_relative = 0.01);
    }

    #[test]
    fn lookup_highest() {
        let weight = lookup(20000.0);
        assert_relative_eq!(weight, db_to_amplitude(-11.1), max_relative = 0.01);
    }

    #[test]
    fn lookup_in_range() {
        let weight = lookup(13.0);
        assert_relative_eq!(weight, db_to_amplitude(32.2), max_relative = 0.03);

        let weight = lookup(125.0);
        assert_relative_eq!(weight, db_to_amplitude(4.2), max_relative = 0.01);

        let weight = lookup(1250.0);
        assert_relative_eq!(weight, db_to_amplitude(-0.2), max_relative = 0.01);

        let weight = lookup(8000.0);
        assert_relative_eq!(weight, db_to_amplitude(-2.9), max_relative = 0.01);

        let weight = lookup(16000.0);
        assert_relative_eq!(weight, db_to_amplitude(-8.4), max_relative = 0.01);
    }
}
