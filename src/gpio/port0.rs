use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    gpio_pin_pfs, 
    gpio_pin_analog, gpio_pin_input, gpio_pin_irq, gpio_pin_output
};

pub struct Port0 {
    _regs: pac::Port0
}

pub struct Ports {
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
    pub fn new(regs: pac::Port0) -> Self {
        Self { _regs: regs }
    }
    pub fn split(self) -> port0::Ports {
        Ports {
            p000: P000::default(),
            p001: P001::default(),
            p002: P002::default(),
            p003: P003::default(),
            p004: P004::default(),
            p005: P005::default(),
            p006: P006::default(),
            p007: P007::default(),
            p008: P008::default(),
            p009: P009::default(),
            p010: P010::default(),
            p014: P014::default(),
            p015: P015::default(),
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