use core::{convert::Infallible, sync::atomic::{AtomicBool, Ordering}};
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    AlreadyTaken, gpio_pin_pfs, 
    gpio_pin_alternate, gpio_pin_drive, gpio_pin_input, gpio_pin_irq, gpio_pin_output
};

static PORT_TAKEN: AtomicBool = AtomicBool::new(false);
struct PinToken<const N: u8>;

pub struct PortA {
    _regs: pac::Porta
}

pub struct Pins {
    pub pa00: PA00<Input<Floating>>,
    pub pa01: PA01<Input<Floating>>,
    pub pa08: PA08<Input<Floating>>,
    pub pa09: PA09<Input<Floating>>,
    pub pa10: PA10<Input<Floating>>,
}

impl PortA {
    pub fn take(regs: pac::Porta) -> Result<Self, AlreadyTaken> {
        PORT_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .map_err(|_| AlreadyTaken)?;
        Ok(Self {
            _regs: regs
        })
    }
    pub fn split(self) -> porta::Pins {
        Pins {
            pa00: PA00 { _mode: PhantomData, _token: PinToken::<00> },
            pa01: PA01 { _mode: PhantomData, _token: PinToken::<01> },
            pa08: PA08 { _mode: PhantomData, _token: PinToken::<08> },
            pa09: PA09 { _mode: PhantomData, _token: PinToken::<09> },
            pa10: PA10 { _mode: PhantomData, _token: PinToken::<10> },
        }
    }
}

gpio_pin_pfs!       (a,    00);
gpio_pin_input!     (a, a, 00);
// no_irq
gpio_pin_output!    (a,    00);
gpio_pin_drive!     (a,    00);
gpio_pin_alternate! (a,    00);

gpio_pin_pfs!       (a,    01);
gpio_pin_input!     (a, a, 01);
// no_irq
gpio_pin_output!    (a,    01);
gpio_pin_drive!     (a,    01);
gpio_pin_alternate! (a,    01);

gpio_pin_pfs!       (a,    08);
gpio_pin_input!     (a, a, 08);
gpio_pin_irq!       (a,    08);
gpio_pin_output!    (a,    08);
gpio_pin_drive!     (a,    08);
gpio_pin_alternate! (a,    08);

gpio_pin_pfs!       (a,    09);
gpio_pin_input!     (a, a, 09);
gpio_pin_irq!       (a,    09);
gpio_pin_output!    (a,    09);
gpio_pin_drive!     (a,    09);
gpio_pin_alternate! (a,    09);

gpio_pin_pfs!       (a,    10);
gpio_pin_input!     (a, a, 10);
gpio_pin_irq!       (a,    10);
gpio_pin_output!    (a,    10);
gpio_pin_drive!     (a,    10);
// no_alternate