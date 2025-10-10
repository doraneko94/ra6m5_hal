use core::{convert::Infallible, sync::atomic::{AtomicBool, Ordering}};
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    AlreadyTaken, gpio_pin_pfs, 
    gpio_pin_analog, gpio_pin_input, gpio_pin_irq, gpio_pin_output
};

static PORT_TAKEN: AtomicBool = AtomicBool::new(false);
struct PinToken<const N: u8>;

pub struct Port0 {
    _regs: pac::Port0
}

pub struct Pins {
    pub p000: P000<Input<Floating>>,
    pub p001: P001<Input<Floating>>,
    pub p002: P002<Input<Floating>>,
    pub p003: P003<Input<Floating>>,
    pub p004: P004<Input<Floating>>,
    pub p005: P005<Input<Floating>>,
    pub p006: P006<Input<Floating>>,
    pub p007: P007<Input<Floating>>,
    pub p008: P008<Input<Floating>>,
    pub p009: P009<Input<Floating>>,
    pub p010: P010<Input<Floating>>,
    pub p014: P014<Input<Floating>>,
    pub p015: P015<Input<Floating>>,
}

impl Port0 {
    pub fn take(regs: pac::Port0) -> Result<Self, AlreadyTaken> {
        PORT_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .map_err(|_| AlreadyTaken)?;
        Ok(Self {
            _regs: regs
        })
    }
    pub fn split(self) -> port0::Pins {
        Pins {
            p000: P000 { _mode: PhantomData, _token: PinToken::<00> },
            p001: P001 { _mode: PhantomData, _token: PinToken::<01> },
            p002: P002 { _mode: PhantomData, _token: PinToken::<02> },
            p003: P003 { _mode: PhantomData, _token: PinToken::<03> },
            p004: P004 { _mode: PhantomData, _token: PinToken::<04> },
            p005: P005 { _mode: PhantomData, _token: PinToken::<05> },
            p006: P006 { _mode: PhantomData, _token: PinToken::<06> },
            p007: P007 { _mode: PhantomData, _token: PinToken::<07> },
            p008: P008 { _mode: PhantomData, _token: PinToken::<08> },
            p009: P009 { _mode: PhantomData, _token: PinToken::<09> },
            p010: P010 { _mode: PhantomData, _token: PinToken::<10> },
            p014: P014 { _mode: PhantomData, _token: PinToken::<14> },
            p015: P015 { _mode: PhantomData, _token: PinToken::<15> },
        }
    }
}

gpio_pin_pfs!   (0,    00);
gpio_pin_input! (0, 0, 00);
gpio_pin_analog!(0,    00);
gpio_pin_irq!   (0,    00);
gpio_pin_output!(0,    00);

gpio_pin_pfs!   (0,    01);
gpio_pin_input! (0, 0, 01);
gpio_pin_analog!(0,    01);
gpio_pin_irq!   (0,    01);
gpio_pin_output!(0,    01);

gpio_pin_pfs!   (0,    02);
gpio_pin_input! (0, 0, 02);
gpio_pin_analog!(0,    02);
gpio_pin_irq!   (0,    02);
gpio_pin_output!(0,    02);

gpio_pin_pfs!   (0,    03);
gpio_pin_input! (0, 0, 03);
gpio_pin_analog!(0,    03);
// no_irq
gpio_pin_output!(0,    03);

gpio_pin_pfs!   (0,    04);
gpio_pin_input! (0, 0, 04);
gpio_pin_analog!(0,    04);
gpio_pin_irq!   (0,    04);
gpio_pin_output!(0,    04);

gpio_pin_pfs!   (0,    05);
gpio_pin_input! (0, 0, 05);
gpio_pin_analog!(0,    05);
gpio_pin_irq!   (0,    05);
gpio_pin_output!(0,    05);

gpio_pin_pfs!   (0,    06);
gpio_pin_input! (0, 0, 06);
gpio_pin_analog!(0,    06);
gpio_pin_irq!   (0,    06);
gpio_pin_output!(0,    06);

gpio_pin_pfs!   (0,    07);
gpio_pin_input! (0, 0, 07);
gpio_pin_analog!(0,    07);
// no_irq
gpio_pin_output!(0,    07);

gpio_pin_pfs!   (0,    08);
gpio_pin_input! (0, 0, 08);
gpio_pin_analog!(0,    08);
gpio_pin_irq!   (0,    08);
gpio_pin_output!(0,    08);

gpio_pin_pfs!   (0,    09);
gpio_pin_input! (0, 0, 09);
gpio_pin_analog!(0,    09);
gpio_pin_irq!   (0,    09);
gpio_pin_output!(0,    09);

gpio_pin_pfs!   (0,    10);
gpio_pin_input! (0, 0, 10);
gpio_pin_analog!(0,    10);
gpio_pin_irq!   (0,    10);
gpio_pin_output!(0,    10);

gpio_pin_pfs!   (0,    14);
gpio_pin_input! (0, 0, 14);
gpio_pin_analog!(0,    14);
// no_irq
gpio_pin_output!(0,    14);

gpio_pin_pfs!   (0,    15);
gpio_pin_input! (0, 0, 15);
gpio_pin_analog!(0,    15);
gpio_pin_irq!   (0,    15);
gpio_pin_output!(0,    15);