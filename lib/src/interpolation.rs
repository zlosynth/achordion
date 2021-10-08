pub fn linear(data: &[u16], position: f32) -> f32 {
    let index = position as usize;
    let remainder = position - index as f32;

    let value = data[index] as f32;
    let delta_to_next = if index == (data.len() - 1) {
        data[0] as f32 - value
    } else {
        data[index + 1] as f32 - value
    };

    value + delta_to_next * remainder
}
