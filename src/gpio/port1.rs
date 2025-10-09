use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    gpio_pin_pfs,
    gpio_pin_alternate, gpio_pin_drive, gpio_pin_input, gpio_pin_irq_edge, gpio_pin_output
};

pub struct Port1 {
    _regs: pac::Port1
}

pub struct Ports {
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
    pub fn new(regs: pac::Port1) -> Self {
        Self { _regs: regs }
    }
    pub fn split(self) -> port1::Ports {
        Ports {
            p100: P100::default(),
            p101: P101::default(),
            p102: P102::default(),
            p103: P103::default(),
            p104: P104::default(),
            p105: P105::default(),
            p106: P106::default(),
            p107: P107::default(),
            p108: P108::default(),
            p109: P109::default(),
            p110: P110::default(),
            p111: P111::default(),
            p112: P112::default(),
            p113: P113::default(),
            p114: P114::default(),
            p115: P115::default(),
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