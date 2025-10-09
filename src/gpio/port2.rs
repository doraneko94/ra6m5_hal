use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    gpio_pin_pfs,
    gpio_pin_alternate, gpio_pin_drive, gpio_pin_input, gpio_pin_irq_edge, gpio_pin_output
};

pub struct Port2 {
    _regs: pac::Port1
}

pub struct Ports {
    pub p200: P200<Input<Floating>>,
    pub p201: P201<Input<Floating>>,
    pub p202: P202<Input<Floating>>,
    pub p203: P203<Input<Floating>>,
    pub p204: P204<Input<Floating>>,
    pub p205: P205<Input<Floating>>,
    pub p206: P206<Input<Floating>>,
    pub p207: P207<Input<Floating>>,
    pub p208: P208<Input<Floating>>,
    pub p209: P209<Input<Floating>>,
    pub p210: P210<Input<Floating>>,
    pub p211: P211<Input<Floating>>,
    pub p212: P212<Input<Floating>>,
    pub p213: P213<Input<Floating>>,
    pub p214: P214<Input<Floating>>,
}

impl Port2 {
    pub fn new(regs: pac::Port1) -> Self {
        Self { _regs: regs }
    }
    pub fn split(self) -> port2::Ports {
        Ports {
            p200: P200::default(),
            p201: P201::default(),
            p202: P202::default(),
            p203: P203::default(),
            p204: P204::default(),
            p205: P205::default(),
            p206: P206::default(),
            p207: P207::default(),
            p208: P208::default(),
            p209: P209::default(),
            p210: P210::default(),
            p211: P211::default(),
            p212: P212::default(),
            p213: P213::default(),
            p214: P214::default(),
        }
    }
}

gpio_pin_pfs!       (2,    00);
gpio_pin_input!     (2, 1, 00);
// no_irq
// no_output
// no_drive
// no_alternative

gpio_pin_pfs!       (2,    01);
gpio_pin_input!     (2, 1, 01);
// no_irq
// no_output
// no_drive
// no_alternative

gpio_pin_pfs!       (2,    02);
gpio_pin_input!     (2, 1, 02);
gpio_pin_irq_edge!  (2,    02);
gpio_pin_output!    (2,    02);
gpio_pin_drive!     (2,    02);
gpio_pin_alternate! (2,    02);

gpio_pin_pfs!       (2,    03);
gpio_pin_input!     (2, 1, 03);
gpio_pin_irq_edge!  (2,    03);
gpio_pin_output!    (2,    03);
gpio_pin_drive!     (2,    03);
gpio_pin_alternate! (2,    03);

gpio_pin_pfs!       (2,    04);
gpio_pin_input!     (2, 1, 04);
// no_irq
gpio_pin_output!    (2,    04);
gpio_pin_drive!     (2,    04);
gpio_pin_alternate! (2,    04);

gpio_pin_pfs!       (2,    05);
gpio_pin_input!     (2, 1, 05);
gpio_pin_irq_edge!  (2,    05);
gpio_pin_output!    (2,    05);
gpio_pin_drive!     (2,    05);
gpio_pin_alternate! (2,    05);

gpio_pin_pfs!       (2,    06);
gpio_pin_input!     (2, 1, 06);
gpio_pin_irq_edge!  (2,    06);
gpio_pin_output!    (2,    06);
gpio_pin_drive!     (2,    06);
gpio_pin_alternate! (2,    06);

gpio_pin_pfs!       (2,    07);
gpio_pin_input!     (2, 1, 07);
// no_irq
gpio_pin_output!    (2,    07);
gpio_pin_drive!     (2,    07);
gpio_pin_alternate! (2,    07);

gpio_pin_pfs!       (2,    08);
gpio_pin_input!     (2, 1, 08);
// no_irq
gpio_pin_output!    (2,    08);
gpio_pin_drive!     (2,    08);
gpio_pin_alternate! (2,    08);

gpio_pin_pfs!       (2,    09);
gpio_pin_input!     (2, 1, 09);
// no_irq
gpio_pin_output!    (2,    09);
gpio_pin_drive!     (2,    09);
gpio_pin_alternate! (2,    09);

gpio_pin_pfs!       (2,    10);
gpio_pin_input!     (2, 1, 10);
// no_irq
gpio_pin_output!    (2,    10);
gpio_pin_drive!     (2,    10);
gpio_pin_alternate! (2,    10);

gpio_pin_pfs!       (2,    11);
gpio_pin_input!     (2, 1, 11);
// no_irq
gpio_pin_output!    (2,    11);
gpio_pin_drive!     (2,    11);
gpio_pin_alternate! (2,    11);

gpio_pin_pfs!       (2,    12);
gpio_pin_input!     (2, 1, 12);
// no_irq
gpio_pin_output!    (2,    12);
gpio_pin_drive!     (2,    12);
gpio_pin_alternate! (2,    12);

gpio_pin_pfs!       (2,    13);
gpio_pin_input!     (2, 1, 13);
// no_irq
gpio_pin_output!    (2,    13);
gpio_pin_drive!     (2,    13);
gpio_pin_alternate! (2,    13);

gpio_pin_pfs!       (2,    14);
gpio_pin_input!     (2, 1, 14);
// no_irq
gpio_pin_output!    (2,    14);
gpio_pin_drive!     (2,    14);
gpio_pin_alternate! (2,    14);