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

#[inline(always)]
pub const fn p90pfs() -> &'static pac::common::ClusterRegisterArray<
    pac::common::Reg<pac::pfs::P70Pfs_SPEC, pac::common::RW>, 2, 0x4,
>
{
    unsafe {
        let ptr = (PFS_BASE as *mut u8).add(0x240usize);
        &*(ptr as *const _)
    }
}

macro_rules! gpio_pin_pfs_9 {
    ($id:tt) => {
        paste! {
            impl<Mode> [<P90 $id>]<Mode> {
                pub fn set_pfs(
                    &self, 
                    podr: Option<Podr>, pdr: Option<Pdr>, pcr: Option<Pcr>, 
                    ncodr: Option<Ncodr>, dscr: Option<Drive>, eofr: Option<Edge>, 
                    isel: Option<Isel>, asel: Option<Asel>, pmr: Option<Pmr>, psel: Option<Peripheral>
                ) {
                    with_pfs(|_| unsafe {
                        let mut w = p90pfs().get($id).read();
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
                        p90pfs().get($id).write(w);
                    })
                }
            }
        }
    };
}

pub struct Port9 {
    _regs: pac::Port0
}

pub struct Pins {
    pub p900: P900<Input<Floating>>,
    pub p901: P901<Input<Floating>>,
    pub p905: P905<Input<Floating>>,
    pub p906: P906<Input<Floating>>,
    pub p907: P907<Input<Floating>>,
    pub p908: P908<Input<Floating>>,
}

impl Port9 {
    pub fn take(regs: pac::Port0) -> Result<Self, AlreadyTaken> {
        PORT_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .map_err(|_| AlreadyTaken)?;
        Ok(Self {
            _regs: regs
        })
    }
    pub fn split(self) -> port9::Pins {
        Pins {
            p900: P900 { _mode: PhantomData, _token: PinToken::<00> },
            p901: P901 { _mode: PhantomData, _token: PinToken::<01> },
            p905: P905 { _mode: PhantomData, _token: PinToken::<05> },
            p906: P906 { _mode: PhantomData, _token: PinToken::<06> },
            p907: P907 { _mode: PhantomData, _token: PinToken::<07> },
            p908: P908 { _mode: PhantomData, _token: PinToken::<08> },
        }
    }
}

gpio_pin_pfs_9!     (       0);
gpio_pin_input!     (9, 0, 00);
// no_irq
gpio_pin_output!    (9,    00);
gpio_pin_drive!     (9,    00);
gpio_pin_alternate! (9,    00);

gpio_pin_pfs_9!     (       1);
gpio_pin_input!     (9, 0, 01);
// no_irq
gpio_pin_output!    (9,    01);
gpio_pin_drive!     (9,    01);
gpio_pin_alternate! (9,    01);

gpio_pin_pfs!       (9,    05);
gpio_pin_input!     (9, 0, 05);
gpio_pin_irq!       (9,    05);
gpio_pin_output!    (9,    05);
gpio_pin_drive!     (9,    05);
gpio_pin_alternate! (9,    05);

gpio_pin_pfs!       (9,    06);
gpio_pin_input!     (9, 0, 06);
gpio_pin_irq!       (9,    06);
gpio_pin_output!    (9,    06);
gpio_pin_drive!     (9,    06);
gpio_pin_alternate! (9,    06);

gpio_pin_pfs!       (9,    07);
gpio_pin_input!     (9, 0, 07);
gpio_pin_irq!       (9,    07);
gpio_pin_output!    (9,    07);
gpio_pin_drive!     (9,    07);
gpio_pin_alternate! (9,    07);

gpio_pin_pfs!       (9,    08);
gpio_pin_input!     (9, 0, 08);
gpio_pin_irq!       (9,    08);
gpio_pin_output!    (9,    08);
gpio_pin_drive!     (9,    08);
gpio_pin_alternate! (9,    08);