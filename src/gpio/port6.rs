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

pub struct Port6 {
    _regs: pac::Port0
}

pub struct Pins {
    pub p600: P600<Input<Floating>>,
    pub p601: P601<Input<Floating>>,
    pub p602: P602<Input<Floating>>,
    pub p603: P603<Input<Floating>>,
    pub p604: P604<Input<Floating>>,
    pub p605: P605<Input<Floating>>,
    pub p606: P606<Input<Floating>>,
    pub p607: P607<Input<Floating>>,
    pub p608: P608<Input<Floating>>,
    pub p609: P609<Input<Floating>>,
    pub p610: P610<Input<Floating>>,
    pub p611: P611<Input<Floating>>,
    pub p612: P612<Input<Floating>>,
    pub p613: P613<Input<Floating>>,
    pub p614: P614<Input<Floating>>,
    pub p615: P615<Input<Floating>>,
}

impl Port6 {
    pub fn take(regs: pac::Port0) -> Result<Self, AlreadyTaken> {
        PORT_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .map_err(|_| AlreadyTaken)?;
        Ok(Self {
            _regs: regs
        })
    }
    pub fn split(self) -> port6::Pins {
        Pins {
            p600: P600 { _mode: PhantomData, _token: PinToken::<00> },
            p601: P601 { _mode: PhantomData, _token: PinToken::<01> },
            p602: P602 { _mode: PhantomData, _token: PinToken::<02> },
            p603: P603 { _mode: PhantomData, _token: PinToken::<03> },
            p604: P604 { _mode: PhantomData, _token: PinToken::<04> },
            p605: P605 { _mode: PhantomData, _token: PinToken::<05> },
            p606: P606 { _mode: PhantomData, _token: PinToken::<06> },
            p607: P607 { _mode: PhantomData, _token: PinToken::<07> },
            p608: P608 { _mode: PhantomData, _token: PinToken::<08> },
            p609: P609 { _mode: PhantomData, _token: PinToken::<09> },
            p610: P610 { _mode: PhantomData, _token: PinToken::<10> },
            p611: P611 { _mode: PhantomData, _token: PinToken::<11> },
            p612: P612 { _mode: PhantomData, _token: PinToken::<12> },
            p613: P613 { _mode: PhantomData, _token: PinToken::<13> },
            p614: P614 { _mode: PhantomData, _token: PinToken::<14> },
            p615: P615 { _mode: PhantomData, _token: PinToken::<15> },
        }
    }
}

gpio_pin_pfs!       (6,    00);
gpio_pin_input!     (6, 0, 00);
// no_irq
gpio_pin_output!    (6,    00);
gpio_pin_drive!     (6,    00);
gpio_pin_alternate! (6,    00);

gpio_pin_pfs!       (6,    01);
gpio_pin_input!     (6, 0, 01);
// no_irq
gpio_pin_output!    (6,    01);
gpio_pin_drive!     (6,    01);
gpio_pin_alternate! (6,    01);

gpio_pin_pfs!       (6,    02);
gpio_pin_input!     (6, 0, 02);
// no_irq
gpio_pin_output!    (6,    02);
gpio_pin_drive!     (6,    02);
gpio_pin_alternate! (6,    02);

gpio_pin_pfs!       (6,    03);
gpio_pin_input!     (6, 0, 03);
// no_irq
gpio_pin_output!    (6,    03);
gpio_pin_drive!     (6,    03);
gpio_pin_alternate! (6,    03);

gpio_pin_pfs!       (6,    04);
gpio_pin_input!     (6, 0, 04);
// no_irq
gpio_pin_output!    (6,    04);
gpio_pin_drive!     (6,    04);
gpio_pin_alternate! (6,    04);

gpio_pin_pfs!       (6,    05);
gpio_pin_input!     (6, 0, 05);
// no_irq
gpio_pin_output!    (6,    05);
gpio_pin_drive!     (6,    05);
gpio_pin_alternate! (6,    05);

gpio_pin_pfs!       (6,    06);
gpio_pin_input!     (6, 0, 06);
// no_irq
gpio_pin_output!    (6,    06);
gpio_pin_drive!     (6,    06);
gpio_pin_alternate! (6,    06);

gpio_pin_pfs!       (6,    07);
gpio_pin_input!     (6, 0, 07);
// no_irq
gpio_pin_output!    (6,    07);
gpio_pin_drive!     (6,    07);
gpio_pin_alternate! (6,    07);

gpio_pin_pfs!       (6,    08);
gpio_pin_input!     (6, 0, 08);
// no_irq
gpio_pin_output!    (6,    08);
gpio_pin_drive!     (6,    08);
gpio_pin_alternate! (6,    08);

gpio_pin_pfs!       (6,    09);
gpio_pin_input!     (6, 0, 09);
// no_irq
gpio_pin_output!    (6,    09);
gpio_pin_drive!     (6,    09);
gpio_pin_alternate! (6,    09);

gpio_pin_pfs!       (6,    10);
gpio_pin_input!     (6, 0, 10);
// no_irq
gpio_pin_output!    (6,    10);
gpio_pin_drive!     (6,    10);
gpio_pin_alternate! (6,    10);

gpio_pin_pfs!       (6,    11);
gpio_pin_input!     (6, 0, 11);
// no_irq
gpio_pin_output!    (6,    11);
gpio_pin_drive!     (6,    11);
gpio_pin_alternate! (6,    11);

gpio_pin_pfs!       (6,    12);
gpio_pin_input!     (6, 0, 12);
// no_irq
gpio_pin_output!    (6,    12);
gpio_pin_drive!     (6,    12);
gpio_pin_alternate! (6,    12);

gpio_pin_pfs!       (6,    13);
gpio_pin_input!     (6, 0, 13);
// no_irq
gpio_pin_output!    (6,    13);
gpio_pin_drive!     (6,    13);
gpio_pin_alternate! (6,    13);

gpio_pin_pfs!       (6,    14);
gpio_pin_input!     (6, 0, 14);
// no_irq
gpio_pin_output!    (6,    14);
gpio_pin_drive!     (6,    14);
gpio_pin_alternate! (6,    14);

gpio_pin_pfs!       (6,    15);
gpio_pin_input!     (6, 0, 15);
gpio_pin_irq!       (6,    15);
gpio_pin_output!    (6,    15);
gpio_pin_drive!     (6,    15);
gpio_pin_alternate! (6,    15);