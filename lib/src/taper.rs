#[allow(unused_imports)]
use micromath::F32Ext;

const LOG: [f32; 22] = [
    0.0,
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
    if position < 0.0 {
        return 0.0;
    } else if position > 1.0 {
        return 1.0;
    }

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
    fn log_taper_below_zero() {
        assert_relative_eq!(log(-1.0), 0.0);
    }

    #[test]
    fn log_taper_above_one() {
        assert_relative_eq!(log(2.0), 1.0);
    }

    #[test]
    fn log_taper_within_limits() {
        assert_relative_eq!(log(0.0), 0.0);
        assert_relative_eq!(log(0.025), 0.0);
        assert_relative_eq!(log(0.05), 0.0009998336);
        assert_relative_eq!(log(0.1), 0.022092845);
        assert_relative_eq!(log(0.3), 0.11849195);
        assert_relative_eq!(log(0.5), 0.24256015);
        assert_relative_eq!(log(0.7), 0.4168443);
        assert_relative_eq!(log(0.9), 0.7120149);
        assert_relative_eq!(log(1.0), 1.0);
    }
}
