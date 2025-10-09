use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    gpio_pin_pfs,
    gpio_pin_alternate, gpio_pin_drive, gpio_pin_input, gpio_pin_irq_edge, gpio_pin_output
};

pub struct Port3 {
    _regs: pac::Port1
}

pub struct Ports {
    pub p300: P300<Input<Floating>>,
    pub p301: P301<Input<Floating>>,
    pub p302: P302<Input<Floating>>,
    pub p303: P303<Input<Floating>>,
    pub p304: P304<Input<Floating>>,
    pub p305: P305<Input<Floating>>,
    pub p306: P306<Input<Floating>>,
    pub p307: P307<Input<Floating>>,
    pub p308: P308<Input<Floating>>,
    pub p309: P309<Input<Floating>>,
    pub p310: P310<Input<Floating>>,
    pub p311: P311<Input<Floating>>,
    pub p312: P312<Input<Floating>>,
    pub p313: P313<Input<Floating>>,
    pub p314: P314<Input<Floating>>,
    pub p315: P315<Input<Floating>>,
}

impl Port3 {
    pub fn new(regs: pac::Port1) -> Self {
        Self { _regs: regs }
    }
    pub fn split(self) -> port3::Ports {
        Ports {
            p300: P300::default(),
            p301: P301::default(),
            p302: P302::default(),
            p303: P303::default(),
            p304: P304::default(),
            p305: P305::default(),
            p306: P306::default(),
            p307: P307::default(),
            p308: P308::default(),
            p309: P309::default(),
            p310: P310::default(),
            p311: P311::default(),
            p312: P312::default(),
            p313: P313::default(),
            p314: P314::default(),
            p315: P315::default(),
        }
    }
}

gpio_pin_pfs!       (3,    00);
gpio_pin_input!     (3, 1, 00);
// no_irq
gpio_pin_output!    (3,    00);
gpio_pin_drive!     (3,    00);
gpio_pin_alternate! (3,    00);

gpio_pin_pfs!       (3,    01);
gpio_pin_input!     (3, 1, 01);
gpio_pin_irq_edge!  (3,    01);
gpio_pin_output!    (3,    01);
gpio_pin_drive!     (3,    01);
gpio_pin_alternate! (3,    01);

gpio_pin_pfs!       (3,    02);
gpio_pin_input!     (3, 1, 02);
gpio_pin_irq_edge!  (3,    02);
gpio_pin_output!    (3,    02);
gpio_pin_drive!     (3,    02);
gpio_pin_alternate! (3,    02);

gpio_pin_pfs!       (3,    03);
gpio_pin_input!     (3, 1, 03);
// no_irq
gpio_pin_output!    (3,    03);
gpio_pin_drive!     (3,    03);
gpio_pin_alternate! (3,    03);

gpio_pin_pfs!       (3,    04);
gpio_pin_input!     (3, 1, 04);
gpio_pin_irq_edge!  (3,    04);
gpio_pin_output!    (3,    04);
gpio_pin_drive!     (3,    04);
gpio_pin_alternate! (3,    04);

gpio_pin_pfs!       (3,    05);
gpio_pin_input!     (3, 1, 05);
gpio_pin_irq_edge!  (3,    05);
gpio_pin_output!    (3,    05);
gpio_pin_drive!     (3,    05);
gpio_pin_alternate! (3,    05);

gpio_pin_pfs!       (3,    06);
gpio_pin_input!     (3, 1, 06);
// no_irq
gpio_pin_output!    (3,    06);
gpio_pin_drive!     (3,    06);
gpio_pin_alternate! (3,    06);

gpio_pin_pfs!       (3,    07);
gpio_pin_input!     (3, 1, 07);
// no_irq
gpio_pin_output!    (3,    07);
gpio_pin_drive!     (3,    07);
gpio_pin_alternate! (3,    07);

gpio_pin_pfs!       (3,    08);
gpio_pin_input!     (3, 1, 08);
// no_irq
gpio_pin_output!    (3,    08);
gpio_pin_drive!     (3,    08);
gpio_pin_alternate! (3,    08);

gpio_pin_pfs!       (3,    09);
gpio_pin_input!     (3, 1, 09);
// no_irq
gpio_pin_output!    (3,    09);
gpio_pin_drive!     (3,    09);
gpio_pin_alternate! (3,    09);

gpio_pin_pfs!       (3,    10);
gpio_pin_input!     (3, 1, 10);
gpio_pin_irq_edge!  (3,    10);
gpio_pin_output!    (3,    10);
gpio_pin_drive!     (3,    10);
gpio_pin_alternate! (3,    10);

gpio_pin_pfs!       (3,    11);
gpio_pin_input!     (3, 1, 11);
// no_irq
gpio_pin_output!    (3,    11);
gpio_pin_drive!     (3,    11);
gpio_pin_alternate! (3,    11);

gpio_pin_pfs!       (3,    12);
gpio_pin_input!     (3, 1, 12);
// no_irq
gpio_pin_output!    (3,    12);
gpio_pin_drive!     (3,    12);
gpio_pin_alternate! (3,    12);

gpio_pin_pfs!       (3,    13);
gpio_pin_input!     (3, 1, 13);
// no_irq
gpio_pin_output!    (3,    13);
gpio_pin_drive!     (3,    13);
gpio_pin_alternate! (3,    13);

gpio_pin_pfs!       (3,    14);
gpio_pin_input!     (3, 1, 14);
// no_irq
gpio_pin_output!    (3,    14);
gpio_pin_drive!     (3,    14);
gpio_pin_alternate! (3,    14);

gpio_pin_pfs!       (3,    15);
gpio_pin_input!     (3, 1, 15);
// no_irq
gpio_pin_output!    (3,    15);
gpio_pin_drive!     (3,    15);
gpio_pin_alternate! (3,    15);