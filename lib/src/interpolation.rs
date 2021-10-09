pub fn linear(data: &[f32], position: f32) -> f32 {
    let index = position as usize;
    let remainder = position - index as f32;

    let value = data[index];
    let delta_to_next = if index == (data.len() - 1) {
        data[0] - value
    } else {
        data[index + 1] - value
    };

    value + delta_to_next * remainder
}
