use core::{convert::Infallible, sync::atomic::{AtomicBool, Ordering}};
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    AlreadyTaken, gpio_pin_pfs, 
    gpio_pin_alternate, gpio_pin_drive, gpio_pin_input, gpio_pin_output
};

static PORT_TAKEN: AtomicBool = AtomicBool::new(false);
struct PinToken<const N: u8>;

pub struct PortB {
    _regs: pac::Portb
}

pub struct Pins {
    pub pb00: PB00<Input<Floating>>,
    pub pb01: PB01<Input<Floating>>,
}

impl PortB {
    pub fn take(regs: pac::Portb) -> Result<Self, AlreadyTaken> {
        PORT_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .map_err(|_| AlreadyTaken)?;
        Ok(Self {
            _regs: regs
        })
    }
    pub fn split(self) -> portb::Pins {
        Pins {
            pb00: PB00 { _mode: PhantomData, _token: PinToken::<00> },
            pb01: PB01 { _mode: PhantomData, _token: PinToken::<01> },
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