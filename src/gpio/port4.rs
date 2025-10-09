use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    gpio_pin_pfs,
    gpio_pin_alternate, gpio_pin_drive, gpio_pin_input, gpio_pin_irq_edge, gpio_pin_output
};

pub struct Port4 {
    _regs: pac::Port1
}

pub struct Ports {
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
    pub fn new(regs: pac::Port1) -> Self {
        Self { _regs: regs }
    }
    pub fn split(self) -> port4::Ports {
        Ports {
            p400: P400::default(),
            p401: P401::default(),
            p402: P402::default(),
            p403: P403::default(),
            p404: P404::default(),
            p405: P405::default(),
            p406: P406::default(),
            p407: P407::default(),
            p408: P408::default(),
            p409: P409::default(),
            p410: P410::default(),
            p411: P411::default(),
            p412: P412::default(),
            p413: P413::default(),
            p414: P414::default(),
            p415: P415::default(),
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