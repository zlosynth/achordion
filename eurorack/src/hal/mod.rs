pub mod rcc;

pub use stm32f3::stm32f303 as pac;

pub mod prelude {
    pub use super::rcc::{Rcc, RccConstrain};
}
