use core::{convert::Infallible, sync::atomic::{AtomicBool, Ordering}};
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    AlreadyTaken, gpio_pin_pfs, 
    gpio_pin_alternate, gpio_pin_analog, gpio_pin_drive, gpio_pin_input, gpio_pin_irq, gpio_pin_output
};

static PORT_TAKEN: AtomicBool = AtomicBool::new(false);
struct PinToken<const N: u8>;

pub struct Port5 {
    _regs: pac::Port0
}

pub struct Pins {
    pub p500: P500<Input<Floating>>,
    pub p501: P501<Input<Floating>>,
    pub p502: P502<Input<Floating>>,
    pub p503: P503<Input<Floating>>,
    pub p504: P504<Input<Floating>>,
    pub p505: P505<Input<Floating>>,
    pub p506: P506<Input<Floating>>,
    pub p507: P507<Input<Floating>>,
    pub p508: P508<Input<Floating>>,
    pub p511: P511<Input<Floating>>,
    pub p512: P512<Input<Floating>>,
    pub p513: P513<Input<Floating>>,
}

impl Port5 {
    pub fn take(regs: pac::Port0) -> Result<Self, AlreadyTaken> {
        PORT_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .map_err(|_| AlreadyTaken)?;
        Ok(Self {
            _regs: regs
        })
    }
    pub fn split(self) -> port5::Pins {
        Pins {
            p500: P500 { _mode: PhantomData, _token: PinToken::<00> },
            p501: P501 { _mode: PhantomData, _token: PinToken::<01> },
            p502: P502 { _mode: PhantomData, _token: PinToken::<02> },
            p503: P503 { _mode: PhantomData, _token: PinToken::<03> },
            p504: P504 { _mode: PhantomData, _token: PinToken::<04> },
            p505: P505 { _mode: PhantomData, _token: PinToken::<05> },
            p506: P506 { _mode: PhantomData, _token: PinToken::<06> },
            p507: P507 { _mode: PhantomData, _token: PinToken::<07> },
            p508: P508 { _mode: PhantomData, _token: PinToken::<08> },
            p511: P511 { _mode: PhantomData, _token: PinToken::<11> },
            p512: P512 { _mode: PhantomData, _token: PinToken::<12> },
            p513: P513 { _mode: PhantomData, _token: PinToken::<13> },
        }
    }
}

gpio_pin_pfs!       (5,    00);
gpio_pin_input!     (5, 0, 00);
gpio_pin_analog!    (5,    00);
// no_irq
gpio_pin_output!    (5,    00);
gpio_pin_drive!     (5,    00);
gpio_pin_alternate! (5,    00);

gpio_pin_pfs!       (5,    01);
gpio_pin_input!     (5, 0,    01);
gpio_pin_analog!    (5,    01);
gpio_pin_irq!       (5,    01);
gpio_pin_output!    (5,    01);
gpio_pin_drive!     (5,    01);
gpio_pin_alternate! (5,    01);

gpio_pin_pfs!       (5,    02);
gpio_pin_input!     (5, 0, 02);
gpio_pin_analog!    (5,    02);
gpio_pin_irq!       (5,    02);
gpio_pin_output!    (5,    02);
gpio_pin_drive!     (5,    02);
gpio_pin_alternate! (5,    02);

gpio_pin_pfs!       (5,    03);
gpio_pin_input!     (5, 0, 03);
gpio_pin_analog!    (5,    03);
// no_irq
gpio_pin_output!    (5,    03);
gpio_pin_drive!     (5,    03);
gpio_pin_alternate! (5,    03);

gpio_pin_pfs!       (5,    04);
gpio_pin_input!     (5, 0, 04);
gpio_pin_analog!    (5,    04);
// no_irq
gpio_pin_output!    (5,    04);
gpio_pin_drive!     (5,    04);
gpio_pin_alternate! (5,    04);

gpio_pin_pfs!       (5,    05);
gpio_pin_input!     (5, 0, 05);
gpio_pin_analog!    (5,    05);
gpio_pin_irq!       (5,    05);
gpio_pin_output!    (5,    05);
gpio_pin_drive!     (5,    05);
gpio_pin_alternate! (5,    05);

gpio_pin_pfs!       (5,    06);
gpio_pin_input!     (5, 0, 06);
gpio_pin_analog!    (5,    06);
gpio_pin_irq!       (5,    06);
gpio_pin_output!    (5,    06);
gpio_pin_drive!     (5,    06);
gpio_pin_alternate! (5,    06);

gpio_pin_pfs!       (5,    07);
gpio_pin_input!     (5, 0, 07);
gpio_pin_analog!    (5,    07);
// no_irq
gpio_pin_output!    (5,    07);
gpio_pin_drive!     (5,    07);
gpio_pin_alternate! (5,    07);

gpio_pin_pfs!       (5,    08);
gpio_pin_input!     (5, 0, 08);
gpio_pin_analog!    (5,    08);
// no_irq
gpio_pin_output!    (5,    08);
gpio_pin_drive!     (5,    08);
gpio_pin_alternate! (5,    08);

gpio_pin_pfs!       (5,    11);
gpio_pin_input!     (5, 0, 11);
// no_analog
gpio_pin_irq!       (5,    11);
gpio_pin_output!    (5,    11);
gpio_pin_drive!     (5,    11);
gpio_pin_alternate! (5,    11);

gpio_pin_pfs!       (5,    12);
gpio_pin_input!     (5, 0, 12);
// no_analog
gpio_pin_irq!       (5,    12);
gpio_pin_output!    (5,    12);
gpio_pin_drive!     (5,    12);
gpio_pin_alternate! (5,    12);

gpio_pin_pfs!       (5,    13);
gpio_pin_input!     (5, 0, 13);
// no_analog
// no_irq
gpio_pin_output!    (5,    13);
gpio_pin_drive!     (5,    13);
gpio_pin_alternate! (5,    13);