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

pub struct Port3 {
    _regs: pac::Port1
}

pub struct Pins {
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
    pub fn take(regs: pac::Port1) -> Result<Self, AlreadyTaken> {
        PORT_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .map_err(|_| AlreadyTaken)?;
        Ok(Self {
            _regs: regs
        })
    }
    pub fn split(self) -> port3::Pins {
        Pins {
            p300: P300 { _mode: PhantomData, _token: PinToken::<00> },
            p301: P301 { _mode: PhantomData, _token: PinToken::<01> },
            p302: P302 { _mode: PhantomData, _token: PinToken::<02> },
            p303: P303 { _mode: PhantomData, _token: PinToken::<03> },
            p304: P304 { _mode: PhantomData, _token: PinToken::<04> },
            p305: P305 { _mode: PhantomData, _token: PinToken::<05> },
            p306: P306 { _mode: PhantomData, _token: PinToken::<06> },
            p307: P307 { _mode: PhantomData, _token: PinToken::<07> },
            p308: P308 { _mode: PhantomData, _token: PinToken::<08> },
            p309: P309 { _mode: PhantomData, _token: PinToken::<09> },
            p310: P310 { _mode: PhantomData, _token: PinToken::<10> },
            p311: P311 { _mode: PhantomData, _token: PinToken::<11> },
            p312: P312 { _mode: PhantomData, _token: PinToken::<12> },
            p313: P313 { _mode: PhantomData, _token: PinToken::<13> },
            p314: P314 { _mode: PhantomData, _token: PinToken::<14> },
            p315: P315 { _mode: PhantomData, _token: PinToken::<15> },
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