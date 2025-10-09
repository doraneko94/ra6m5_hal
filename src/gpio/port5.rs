use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    gpio_pin_pfs,
    gpio_pin_alternate, gpio_pin_analog, gpio_pin_drive, gpio_pin_input, gpio_pin_irq, gpio_pin_output
};

pub struct Port5 {
    _regs: pac::Port0
}

pub struct Ports {
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
    pub fn new(regs: pac::Port0) -> Self {
        Self { _regs: regs }
    }
    pub fn split(self) -> port5::Ports {
        Ports {
            p500: P500::default(),
            p501: P501::default(),
            p502: P502::default(),
            p503: P503::default(),
            p504: P504::default(),
            p505: P505::default(),
            p506: P506::default(),
            p507: P507::default(),
            p508: P508::default(),
            p511: P511::default(),
            p512: P512::default(),
            p513: P513::default(),
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