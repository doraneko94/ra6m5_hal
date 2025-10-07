#![no_std]

pub mod clock;
pub mod delay;
pub mod fcache;
pub mod flad;
pub mod gpio;
pub mod register_protection;

pub use ra6m5_pac as pac;

pub struct Hal {
    pub pac: pac::Peripherals,
}

impl Hal {
    pub fn take() -> Option<Self> {
        pac::Peripherals::take().map(|p| Self { pac: p })
    }
}