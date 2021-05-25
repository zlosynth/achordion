#[allow(unused_imports)]
use micromath::F32Ext;

const LOG: [f32; 21] = [
    0.0,
    0.019996643,
    0.040958643,
    0.06298393,
    0.08618611,
    0.11069828,
    0.13667715,
    0.16430944,
    0.19382,
    0.225483,
    0.2596373,
    0.29670864,
    0.3372422,
    0.38195187,
    0.43179822,
    0.48811662,
    0.55284196,
    0.6289321,
    0.72124636,
    0.838632,
    1.0,
];

pub fn log(position: f32) -> f32 {
    debug_assert!((0.0..=1.0).contains(&position));

    let array_position = position * (LOG.len() - 1) as f32;
    let index_a = array_position as usize;
    let index_b = (array_position as usize + 1).min(LOG.len() - 1);
    let remainder = array_position.fract();

    let value = LOG[index_a];
    let delta_to_next = LOG[index_b] - LOG[index_a];

    value + delta_to_next * remainder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn log_taper_below_zero() {
        let _ = log(-1.0);
    }

    #[test]
    #[should_panic]
    fn log_taper_above_one() {
        let _ = log(2.0);
    }

    #[test]
    fn log_taper_within_limits() {
        assert_relative_eq!(log(0.0), 0.0);
        assert_relative_eq!(log(0.025), 0.0099983215);
        assert_relative_eq!(log(0.05), 0.019996643);
        assert_relative_eq!(log(0.5), 0.2596373);
        assert_relative_eq!(log(1.0), 1.0);
    }
}
