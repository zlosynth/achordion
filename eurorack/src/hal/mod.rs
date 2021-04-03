pub mod dac;
pub mod dma;
pub mod rcc;
pub mod timer;

pub mod prelude {
    pub use super::dac::DacExt;
    pub use super::dma::Dma2Ch3Ext;
    pub use super::rcc::Apb1;
    pub use super::timer::TimerExt;
}
