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

pub struct Port2 {
    _regs: pac::Port1
}

pub struct Pins {
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
    pub fn take(regs: pac::Port1) -> Result<Self, AlreadyTaken> {
        PORT_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .map_err(|_| AlreadyTaken)?;
        Ok(Self {
            _regs: regs
        })
    }
    pub fn split(self) -> port2::Pins {
        Pins {
            p200: P200 { _mode: PhantomData, _token: PinToken::<00> },
            p201: P201 { _mode: PhantomData, _token: PinToken::<01> },
            p202: P202 { _mode: PhantomData, _token: PinToken::<02> },
            p203: P203 { _mode: PhantomData, _token: PinToken::<03> },
            p204: P204 { _mode: PhantomData, _token: PinToken::<04> },
            p205: P205 { _mode: PhantomData, _token: PinToken::<05> },
            p206: P206 { _mode: PhantomData, _token: PinToken::<06> },
            p207: P207 { _mode: PhantomData, _token: PinToken::<07> },
            p208: P208 { _mode: PhantomData, _token: PinToken::<08> },
            p209: P209 { _mode: PhantomData, _token: PinToken::<09> },
            p210: P210 { _mode: PhantomData, _token: PinToken::<10> },
            p211: P211 { _mode: PhantomData, _token: PinToken::<11> },
            p212: P212 { _mode: PhantomData, _token: PinToken::<12> },
            p213: P213 { _mode: PhantomData, _token: PinToken::<13> },
            p214: P214 { _mode: PhantomData, _token: PinToken::<14> },
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