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

pub struct Port1 {
    _regs: pac::Port1
}

pub struct Pins {
    pub p100: P100<Input<Floating>>,
    pub p101: P101<Input<Floating>>,
    pub p102: P102<Input<Floating>>,
    pub p103: P103<Input<Floating>>,
    pub p104: P104<Input<Floating>>,
    pub p105: P105<Input<Floating>>,
    pub p106: P106<Input<Floating>>,
    pub p107: P107<Input<Floating>>,
    pub p108: P108<Input<Floating>>,
    pub p109: P109<Input<Floating>>,
    pub p110: P110<Input<Floating>>,
    pub p111: P111<Input<Floating>>,
    pub p112: P112<Input<Floating>>,
    pub p113: P113<Input<Floating>>,
    pub p114: P114<Input<Floating>>,
    pub p115: P115<Input<Floating>>,
}

impl Port1 {
    pub fn take(regs: pac::Port1) -> Result<Self, AlreadyTaken> {
        PORT_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .map_err(|_| AlreadyTaken)?;
        Ok(Self {
            _regs: regs
        })
    }
    pub fn split(self) -> port1::Pins {
        Pins {
            p100: P100 { _mode: PhantomData, _token: PinToken::<00> },
            p101: P101 { _mode: PhantomData, _token: PinToken::<01> },
            p102: P102 { _mode: PhantomData, _token: PinToken::<02> },
            p103: P103 { _mode: PhantomData, _token: PinToken::<03> },
            p104: P104 { _mode: PhantomData, _token: PinToken::<04> },
            p105: P105 { _mode: PhantomData, _token: PinToken::<05> },
            p106: P106 { _mode: PhantomData, _token: PinToken::<06> },
            p107: P107 { _mode: PhantomData, _token: PinToken::<07> },
            p108: P108 { _mode: PhantomData, _token: PinToken::<08> },
            p109: P109 { _mode: PhantomData, _token: PinToken::<09> },
            p110: P110 { _mode: PhantomData, _token: PinToken::<10> },
            p111: P111 { _mode: PhantomData, _token: PinToken::<11> },
            p112: P112 { _mode: PhantomData, _token: PinToken::<12> },
            p113: P113 { _mode: PhantomData, _token: PinToken::<13> },
            p114: P114 { _mode: PhantomData, _token: PinToken::<14> },
            p115: P115 { _mode: PhantomData, _token: PinToken::<15> },
        }
    }
}

gpio_pin_pfs!       (1,    00);
gpio_pin_input!     (1, 1, 00);
gpio_pin_irq_edge!  (1,    00);
gpio_pin_output!    (1,    00);
gpio_pin_drive!     (1,    00);
gpio_pin_alternate! (1,    00);

gpio_pin_pfs!       (1,    01);
gpio_pin_input!     (1, 1, 01);
gpio_pin_irq_edge!  (1,    01);
gpio_pin_output!    (1,    01);
gpio_pin_drive!     (1,    01);
gpio_pin_alternate! (1,    01);

gpio_pin_pfs!       (1,    02);
gpio_pin_input!     (1, 1, 02);
// no_irq
gpio_pin_output!    (1,    02);
gpio_pin_drive!     (1,    02);
gpio_pin_alternate! (1,    02);

gpio_pin_pfs!       (1,    03);
gpio_pin_input!     (1, 1, 03);
// no_irq
gpio_pin_output!    (1,    03);
gpio_pin_drive!     (1,    03);
gpio_pin_alternate! (1,    03);

gpio_pin_pfs!       (1,    04);
gpio_pin_input!     (1, 1, 04);
gpio_pin_irq_edge!  (1,    04);
gpio_pin_output!    (1,    04);
gpio_pin_drive!     (1,    04);
gpio_pin_alternate! (1,    04);

gpio_pin_pfs!       (1,    05);
gpio_pin_input!     (1, 1, 05);
gpio_pin_irq_edge!  (1,    05);
gpio_pin_output!    (1,    05);
gpio_pin_drive!     (1,    05);
gpio_pin_alternate! (1,    05);

gpio_pin_pfs!       (1,    06);
gpio_pin_input!     (1, 1, 06);
// no_irq
gpio_pin_output!    (1,    06);
gpio_pin_drive!     (1,    06);
gpio_pin_alternate! (1,    06);

gpio_pin_pfs!       (1,    07);
gpio_pin_input!     (1, 1, 07);
// no_irq
gpio_pin_output!    (1,    07);
gpio_pin_drive!     (1,    07);
gpio_pin_alternate! (1,    07);

gpio_pin_pfs!       (1,    08);
gpio_pin_input!     (1, 1, 08);
// no_irq
gpio_pin_output!    (1,    08);
gpio_pin_drive!     (1,    08);
gpio_pin_alternate! (1,    08);

gpio_pin_pfs!       (1,    09);
gpio_pin_input!     (1, 1, 09);
// no_irq
gpio_pin_output!    (1,    09);
gpio_pin_drive!     (1,    09);
gpio_pin_alternate! (1,    09);

gpio_pin_pfs!       (1,    10);
gpio_pin_input!     (1, 1, 10);
gpio_pin_irq_edge!  (1,    10);
gpio_pin_output!    (1,    10);
gpio_pin_drive!     (1,    10);
gpio_pin_alternate! (1,    10);

gpio_pin_pfs!       (1,    11);
gpio_pin_input!     (1, 1, 11);
gpio_pin_irq_edge!  (1,    11);
gpio_pin_output!    (1,    11);
gpio_pin_drive!     (1,    11);
gpio_pin_alternate! (1,    11);

gpio_pin_pfs!       (1,    12);
gpio_pin_input!     (1, 1, 12);
// no_irq
gpio_pin_output!    (1,    12);
gpio_pin_drive!     (1,    12);
gpio_pin_alternate! (1,    12);

gpio_pin_pfs!       (1,    13);
gpio_pin_input!     (1, 1, 13);
// no_irq
gpio_pin_output!    (1,    13);
gpio_pin_drive!     (1,    13);
gpio_pin_alternate! (1,    13);

gpio_pin_pfs!       (1,    14);
gpio_pin_input!     (1, 1, 14);
// no_irq
gpio_pin_output!    (1,    14);
gpio_pin_drive!     (1,    14);
gpio_pin_alternate! (1,    14);

gpio_pin_pfs!       (1,    15);
gpio_pin_input!     (1, 1, 15);
// no_irq
gpio_pin_output!    (1,    15);
gpio_pin_drive!     (1,    15);
gpio_pin_alternate! (1,    15);