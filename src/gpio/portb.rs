use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    gpio_pin_pfs,
    gpio_pin_alternate, gpio_pin_drive, gpio_pin_input, gpio_pin_output
};

pub struct PortB {
    _regs: pac::Portb
}

pub struct Ports {
    pub pb00: PB00<Input<Floating>>,
    pub pb01: PB01<Input<Floating>>,
}

impl PortB {
    pub fn new(regs: pac::Portb) -> Self {
        Self { _regs: regs }
    }
    pub fn split(self) -> portb::Ports {
        Ports {
            pb00: PB00::default(),
            pb01: PB01::default(),
        }
    }
}

gpio_pin_pfs!       (b,    00);
gpio_pin_input!     (b, b, 00);
gpio_pin_output!    (b,    00);
gpio_pin_drive!     (b,    00);
gpio_pin_alternate! (b,    00);

gpio_pin_pfs!       (b,    01);
gpio_pin_input!     (b, b, 01);
gpio_pin_output!    (b,    01);
gpio_pin_drive!     (b,    01);
gpio_pin_alternate! (b,    01);