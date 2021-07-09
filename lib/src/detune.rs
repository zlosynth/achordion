#[derive(Clone, Copy, PartialEq)]
pub enum DetuneConfig {
    Disabled,
    SingleSide(f32, f32),
    BothSides(f32, f32),
}
