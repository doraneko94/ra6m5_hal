use core::{convert::Infallible, sync::atomic::{AtomicBool, Ordering}};
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    AlreadyTaken, gpio_pin_pfs, 
    gpio_pin_alternate, gpio_pin_drive, gpio_pin_input, gpio_pin_irq_edge, gpio_pin_output
};

static PORT_TAKEN: AtomicBool = AtomicBool::new(false);
struct PinToken<const N: u8>;

pub struct Port4 {
    _regs: pac::Port1
}

pub struct Pins {
    pub p400: P400<Input<Floating>>,
    pub p401: P401<Input<Floating>>,
    pub p402: P402<Input<Floating>>,
    pub p403: P403<Input<Floating>>,
    pub p404: P404<Input<Floating>>,
    pub p405: P405<Input<Floating>>,
    pub p406: P406<Input<Floating>>,
    pub p407: P407<Input<Floating>>,
    pub p408: P408<Input<Floating>>,
    pub p409: P409<Input<Floating>>,
    pub p410: P410<Input<Floating>>,
    pub p411: P411<Input<Floating>>,
    pub p412: P412<Input<Floating>>,
    pub p413: P413<Input<Floating>>,
    pub p414: P414<Input<Floating>>,
    pub p415: P415<Input<Floating>>,
}

impl Port4 {
    pub fn take(regs: pac::Port1) -> Result<Self, AlreadyTaken> {
        PORT_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .map_err(|_| AlreadyTaken)?;
        Ok(Self {
            _regs: regs
        })
    }
    pub fn split(self) -> port4::Pins {
        Pins {
            p400: P400 { _mode: PhantomData, _token: PinToken::<00> },
            p401: P401 { _mode: PhantomData, _token: PinToken::<01> },
            p402: P402 { _mode: PhantomData, _token: PinToken::<02> },
            p403: P403 { _mode: PhantomData, _token: PinToken::<03> },
            p404: P404 { _mode: PhantomData, _token: PinToken::<04> },
            p405: P405 { _mode: PhantomData, _token: PinToken::<05> },
            p406: P406 { _mode: PhantomData, _token: PinToken::<06> },
            p407: P407 { _mode: PhantomData, _token: PinToken::<07> },
            p408: P408 { _mode: PhantomData, _token: PinToken::<08> },
            p409: P409 { _mode: PhantomData, _token: PinToken::<09> },
            p410: P410 { _mode: PhantomData, _token: PinToken::<10> },
            p411: P411 { _mode: PhantomData, _token: PinToken::<11> },
            p412: P412 { _mode: PhantomData, _token: PinToken::<12> },
            p413: P413 { _mode: PhantomData, _token: PinToken::<13> },
            p414: P414 { _mode: PhantomData, _token: PinToken::<14> },
            p415: P415 { _mode: PhantomData, _token: PinToken::<15> },
        }
    }
}

gpio_pin_pfs!       (4,    00);
gpio_pin_input!     (4, 1, 00);
gpio_pin_irq_edge!  (4,    00);
gpio_pin_output!    (4,    00);
gpio_pin_drive!     (4,    00);
gpio_pin_alternate! (4,    00);

gpio_pin_pfs!       (4,    01);
gpio_pin_input!     (4, 1, 01);
gpio_pin_irq_edge!  (4,    01);
gpio_pin_output!    (4,    01);
gpio_pin_drive!     (4,    01);
gpio_pin_alternate! (4,    01);

gpio_pin_pfs!       (4,    02);
gpio_pin_input!     (4, 1, 02);
gpio_pin_irq_edge!  (4,    02);
gpio_pin_output!    (4,    02);
gpio_pin_drive!     (4,    02);
gpio_pin_alternate! (4,    02);

gpio_pin_pfs!       (4,    03);
gpio_pin_input!     (4, 1, 03);
gpio_pin_irq_edge!  (4,    03);
gpio_pin_output!    (4,    03);
gpio_pin_drive!     (4,    03);
gpio_pin_alternate! (4,    03);

gpio_pin_pfs!       (4,    04);
gpio_pin_input!     (4, 1, 04);
gpio_pin_irq_edge!  (4,    04);
gpio_pin_output!    (4,    04);
gpio_pin_drive!     (4,    04);
gpio_pin_alternate! (4,    04);

gpio_pin_pfs!       (4,    05);
gpio_pin_input!     (4, 1, 05);
// no_irq
gpio_pin_output!    (4,    05);
gpio_pin_drive!     (4,    05);
gpio_pin_alternate! (4,    05);

gpio_pin_pfs!       (4,    06);
gpio_pin_input!     (4, 1, 06);
// no_irq
gpio_pin_output!    (4,    06);
gpio_pin_drive!     (4,    06);
gpio_pin_alternate! (4,    06);

gpio_pin_pfs!       (4,    07);
gpio_pin_input!     (4, 1, 07);
// no_irq
gpio_pin_output!    (4,    07);
gpio_pin_drive!     (4,    07);
gpio_pin_alternate! (4,    07);

gpio_pin_pfs!       (4,    08);
gpio_pin_input!     (4, 1, 08);
gpio_pin_irq_edge!  (4,    08);
gpio_pin_output!    (4,    08);
gpio_pin_drive!     (4,    08);
gpio_pin_alternate! (4,    08);

gpio_pin_pfs!       (4,    09);
gpio_pin_input!     (4, 1, 09);
gpio_pin_irq_edge!  (4,    09);
gpio_pin_output!    (4,    09);
gpio_pin_drive!     (4,    09);
gpio_pin_alternate! (4,    09);

gpio_pin_pfs!       (4,    10);
gpio_pin_input!     (4, 1, 10);
gpio_pin_irq_edge!  (4,    10);
gpio_pin_output!    (4,    10);
gpio_pin_drive!     (4,    10);
gpio_pin_alternate! (4,    10);

gpio_pin_pfs!       (4,    11);
gpio_pin_input!     (4, 1, 11);
gpio_pin_irq_edge!  (4,    11);
gpio_pin_output!    (4,    11);
gpio_pin_drive!     (4,    11);
gpio_pin_alternate! (4,    11);

gpio_pin_pfs!       (4,    12);
gpio_pin_input!     (4, 1, 12);
// no_irq
gpio_pin_output!    (4,    12);
gpio_pin_drive!     (4,    12);
gpio_pin_alternate! (4,    12);

gpio_pin_pfs!       (4,    13);
gpio_pin_input!     (4, 1, 13);
// no_irq
gpio_pin_output!    (4,    13);
gpio_pin_drive!     (4,    13);
gpio_pin_alternate! (4,    13);

gpio_pin_pfs!       (4,    14);
gpio_pin_input!     (4, 1, 14);
gpio_pin_irq_edge!  (4,    14);
gpio_pin_output!    (4,    14);
gpio_pin_drive!     (4,    14);
gpio_pin_alternate! (4,    14);

gpio_pin_pfs!       (4,    15);
gpio_pin_input!     (4, 1, 15);
gpio_pin_irq_edge!  (4,    15);
gpio_pin_output!    (4,    15);
gpio_pin_drive!     (4,    15);
gpio_pin_alternate! (4,    15);