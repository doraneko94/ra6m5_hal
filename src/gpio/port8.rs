use core::{convert::Infallible, sync::atomic::{AtomicBool, Ordering}};
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    AlreadyTaken, gpio_pin_pfs, 
    gpio_pin_analog, gpio_pin_alternate, gpio_pin_drive, gpio_pin_input, gpio_pin_irq, gpio_pin_output
};

static PORT_TAKEN: AtomicBool = AtomicBool::new(false);
struct PinToken<const N: u8>;

pub struct Port8 {
    _regs: pac::Port0
}

pub struct Pins {
    pub p800: P800<Input<Floating>>,
    pub p801: P801<Input<Floating>>,
    pub p802: P802<Input<Floating>>,
    pub p803: P803<Input<Floating>>,
    pub p804: P804<Input<Floating>>,
    pub p805: P805<Input<Floating>>,
    pub p806: P806<Input<Floating>>,
}

impl Port8 {
    pub fn take(regs: pac::Port0) -> Result<Self, AlreadyTaken> {
        PORT_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .map_err(|_| AlreadyTaken)?;
        Ok(Self {
            _regs: regs
        })
    }
    pub fn split(self) -> port8::Pins {
        Pins {
            p800: P800 { _mode: PhantomData, _token: PinToken::<00> },
            p801: P801 { _mode: PhantomData, _token: PinToken::<01> },
            p802: P802 { _mode: PhantomData, _token: PinToken::<02> },
            p803: P803 { _mode: PhantomData, _token: PinToken::<03> },
            p804: P804 { _mode: PhantomData, _token: PinToken::<04> },
            p805: P805 { _mode: PhantomData, _token: PinToken::<05> },
            p806: P806 { _mode: PhantomData, _token: PinToken::<06> },
        }
    }
}

gpio_pin_pfs!       (8,    00);
gpio_pin_input!     (8, 0, 00);
gpio_pin_analog!    (8,    00);
// no_irq
gpio_pin_output!    (8,    00);
gpio_pin_drive!     (8,    00);
gpio_pin_alternate! (8,    00);

gpio_pin_pfs!       (8,    01);
gpio_pin_input!     (8, 0, 01);
gpio_pin_analog!    (8,    01);
// no_irq
gpio_pin_output!    (8,    01);
gpio_pin_drive!     (8,    01);
gpio_pin_alternate! (8,    01);

gpio_pin_pfs!       (8,    02);
gpio_pin_input!     (8, 0, 02);
gpio_pin_analog!    (8,    02);
gpio_pin_irq!       (8,    02);
gpio_pin_output!    (8,    02);
gpio_pin_drive!     (8,    02);
// no_alternate

gpio_pin_pfs!       (8,    03);
gpio_pin_input!     (8, 0, 03);
gpio_pin_analog!    (8,    03);
gpio_pin_irq!       (8,    03);
gpio_pin_output!    (8,    03);
gpio_pin_drive!     (8,    03);
// no_alternate

gpio_pin_pfs!       (8,    04);
gpio_pin_input!     (8, 0, 04);
// no_analog
gpio_pin_irq!       (8,    04);
gpio_pin_output!    (8,    04);
gpio_pin_drive!     (8,    04);
// no_alternate

gpio_pin_pfs!       (8,    05);
gpio_pin_input!     (8, 0, 05);
// no_analog
// no_irq
gpio_pin_output!    (8,    05);
gpio_pin_drive!     (8,    05);
gpio_pin_alternate! (8,    05);

gpio_pin_pfs!       (8,    06);
gpio_pin_input!     (8, 0, 06);
// no_analog
gpio_pin_irq!       (8,    06);
gpio_pin_output!    (8,    06);
gpio_pin_drive!     (8,    06);
// no_alternate