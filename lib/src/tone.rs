pub trait ToFrequency {
    fn to_frequency(&self) -> f32;
}

impl ToFrequency for f32 {
    fn to_frequency(&self) -> f32 {
        *self
    }
}
