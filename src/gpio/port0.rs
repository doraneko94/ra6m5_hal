use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{gpio_pin_analog, gpio_pin_input, gpio_pin_irq, gpio_pin_output};

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

gpio_pin_input!(0, 00, 00);
gpio_pin_analog!(0, 00, 00);
gpio_pin_irq!(0, 00, 00);
gpio_pin_output!(0, 00, 00);

gpio_pin_input!(0, 00, 01);
gpio_pin_analog!(0, 00, 01);
gpio_pin_irq!(0, 00, 01);
gpio_pin_output!(0, 00, 01);

gpio_pin_input!(0, 00, 02);
gpio_pin_analog!(0, 00, 02);
gpio_pin_irq!(0, 00, 02);
gpio_pin_output!(0, 00, 02);

gpio_pin_input!(0, 00, 03);
gpio_pin_analog!(0, 00, 03);
// no_irq
gpio_pin_output!(0, 00, 03);

gpio_pin_input!(0, 00, 04);
gpio_pin_analog!(0, 00, 04);
gpio_pin_irq!(0, 00, 04);
gpio_pin_output!(0, 00, 04);

gpio_pin_input!(0, 00, 05);
gpio_pin_analog!(0, 00, 05);
gpio_pin_irq!(0, 00, 05);
gpio_pin_output!(0, 00, 05);

gpio_pin_input!(0, 00, 06);
gpio_pin_analog!(0, 00, 06);
gpio_pin_irq!(0, 00, 06);
gpio_pin_output!(0, 00, 06);

gpio_pin_input!(0, 00, 07);
gpio_pin_analog!(0, 00, 07);
// no_irq
gpio_pin_output!(0, 00, 07);

gpio_pin_input!(0, 008, 08);
gpio_pin_analog!(0, 008, 08);
gpio_pin_irq!(0, 008, 08);
gpio_pin_output!(0, 008, 08);

gpio_pin_input!(0, 009, 09);
gpio_pin_analog!(0, 009, 09);
gpio_pin_irq!(0, 009, 09);
gpio_pin_output!(0, 009, 09);

gpio_pin_input!(0, 010, 10);
gpio_pin_analog!(0, 010, 10);
gpio_pin_irq!(0, 010, 10);
gpio_pin_output!(0, 010, 10);

gpio_pin_input!(0, 0, 14);
gpio_pin_analog!(0, 0, 14);
// no_irq
gpio_pin_output!(0, 0, 14);

gpio_pin_input!(0, 0, 15);
gpio_pin_analog!(0, 0, 15);
gpio_pin_irq!(0, 0, 15);
gpio_pin_output!(0, 0, 15);