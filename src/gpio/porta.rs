use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, StatefulOutputPin, OutputPin};
use ra6m5_pac::RegisterValue;
use paste::paste;

use super::*;
use crate::{
    gpio_pin_pfs,
    gpio_pin_alternate, gpio_pin_drive, gpio_pin_input, gpio_pin_irq, gpio_pin_output
};

#[inline(always)]
pub const fn pa0pfs() -> &'static pac::common::ClusterRegisterArray<
    pac::common::Reg<pac::pfs::P70Pfs_SPEC, pac::common::RW>, 2, 0x4,
>
{
    unsafe {
        let ptr = (PFS_BASE as *mut u8).add(0x280usize);
        &*(ptr as *const _)
    }
}

macro_rules! gpio_pin_pfs_a {
    ($id:tt) => {
        paste! {
            impl<Mode> [<PA0 $id>]<Mode> {
                pub fn set_pfs(
                    self, 
                    podr: Option<Podr>, pdr: Option<Pdr>, pcr: Option<Pcr>, 
                    ncodr: Option<Ncodr>, dscr: Option<Drive>, eofr: Option<Edge>, 
                    isel: Option<Isel>, asel: Option<Asel>, pmr: Option<Pmr>, psel: Option<Peripheral>
                ) {
                    with_pfs(|| unsafe {
                        let mut w = pa0pfs().get($id).read();
                        if let Some(value) = podr { w = w.podr().set((value as u8).into()); }
                        if let Some(value) = pdr { w = w.pdr().set((value as u8).into()); }
                        if let Some(value) = pcr { w = w.pcr().set((value as u8).into()); }
                        if let Some(value) = ncodr { w = w.ncodr().set((value as u8).into()); }
                        if let Some(value) = dscr {
                            let bits = w.get_raw();
                            w = w.set_raw((bits & !DSCR_MASK) | (((value as u32) << DSCR_SHIFT) & DSCR_MASK));
                        }
                        if let Some(value) = eofr {
                            let bits = w.get_raw();
                            w = w.set_raw((bits & !EOFR_MASK) | (((value as u32) << EOFR_SHIFT) & EOFR_MASK));
                        }
                        if let Some(value) = isel { w = w.isel().set((value as u8).into()); }
                        if let Some(value) = asel { w = w.asel().set((value as u8).into()); }
                        if let Some(value) = pmr { w = w.pmr().set((value as u8).into()); }
                        if let Some(value) = psel { w = w.psel().set((value as u8).into()); }
                        pa0pfs().get($id).write(w);
                    })
                }
            }
        }
    };
}

pub struct PortA {
    _regs: pac::Porta
}

pub struct Ports {
    pub pa00: PA00<Input<Floating>>,
    pub pa01: PA01<Input<Floating>>,
    pub pa08: PA08<Input<Floating>>,
    pub pa09: PA09<Input<Floating>>,
    pub pa10: PA10<Input<Floating>>,
}

impl PortA {
    pub fn new(regs: pac::Porta) -> Self {
        Self { _regs: regs }
    }
    pub fn split(self) -> porta::Ports {
        Ports {
            pa00: PA00::default(),
            pa01: PA01::default(),
            pa08: PA08::default(),
            pa09: PA09::default(),
            pa10: PA10::default(),
        }
    }
}

gpio_pin_pfs_a!     (       0);
gpio_pin_input!     (a, a, 00);
// no_irq
gpio_pin_output!    (a,    00);
gpio_pin_drive!     (a,    00);
gpio_pin_alternate! (a,    00);

gpio_pin_pfs_a!     (       1);
gpio_pin_input!     (a, a, 01);
// no_irq
gpio_pin_output!    (a,    01);
gpio_pin_drive!     (a,    01);
gpio_pin_alternate! (a,    01);

gpio_pin_pfs!       (a,    08);
gpio_pin_input!     (a, a, 08);
gpio_pin_irq!       (a,    08);
gpio_pin_output!    (a,    08);
gpio_pin_drive!     (a,    08);
gpio_pin_alternate! (a,    08);

gpio_pin_pfs!       (a,    09);
gpio_pin_input!     (a, a, 09);
gpio_pin_irq!       (a,    09);
gpio_pin_output!    (a,    09);
gpio_pin_drive!     (a,    09);
gpio_pin_alternate! (a,    09);

gpio_pin_pfs!       (a,    10);
gpio_pin_input!     (a, a, 10);
gpio_pin_irq!       (a,    10);
gpio_pin_output!    (a,    10);
gpio_pin_drive!     (a,    10);
// no_alternate