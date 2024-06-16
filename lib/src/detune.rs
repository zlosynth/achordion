#[derive(Clone, Copy, PartialEq)]
pub enum DetuneConfig {
    Disabled,
    SingleSide(f32, f32, usize),
    BothSides(f32, f32, usize),
    SingleVoice(f32, f32),
}
