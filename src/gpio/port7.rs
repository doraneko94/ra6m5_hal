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
pub const fn p70pfs() -> &'static pac::common::ClusterRegisterArray<
    pac::common::Reg<pac::pfs::P70Pfs_SPEC, pac::common::RW>, 8, 0x4,
>
{
    unsafe {
        let ptr = (PFS_BASE as *mut u8).add(0x1c0usize);
        &*(ptr as *const _)
    }
}

macro_rules! gpio_pin_pfs_7 {
    ($id:tt) => {
        paste! {
            impl<Mode> [<P70 $id>]<Mode> {
                pub fn set_pfs(
                    &self, 
                    podr: Option<Podr>, pdr: Option<Pdr>, pcr: Option<Pcr>, 
                    ncodr: Option<Ncodr>, dscr: Option<Drive>, eofr: Option<Edge>, 
                    isel: Option<Isel>, asel: Option<Asel>, pmr: Option<Pmr>, psel: Option<Peripheral>
                ) {
                    with_pfs(|_| unsafe {
                        let mut w = p70pfs().get($id).read();
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
                        p70pfs().get($id).write(w);
                    })
                }
            }
        }
    };
}

pub struct Port7 {
    _regs: pac::Port0
}

pub struct Pins {
    pub p700: P700<Input<Floating>>,
    pub p701: P701<Input<Floating>>,
    pub p702: P702<Input<Floating>>,
    pub p703: P703<Input<Floating>>,
    pub p704: P704<Input<Floating>>,
    pub p705: P705<Input<Floating>>,
    pub p706: P706<Input<Floating>>,
    pub p707: P707<Input<Floating>>,
    pub p708: P708<Input<Floating>>,
    pub p709: P709<Input<Floating>>,
    pub p710: P710<Input<Floating>>,
    pub p711: P711<Input<Floating>>,
    pub p712: P712<Input<Floating>>,
    pub p713: P713<Input<Floating>>,
}

impl Port7 {
    pub fn take(regs: pac::Port0) -> Result<Self, AlreadyTaken> {
        PORT_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .map_err(|_| AlreadyTaken)?;
        Ok(Self {
            _regs: regs
        })
    }
    pub fn split(self) -> port7::Pins {
        Pins {
            p700: P700 { _mode: PhantomData, _token: PinToken::<00> },
            p701: P701 { _mode: PhantomData, _token: PinToken::<01> },
            p702: P702 { _mode: PhantomData, _token: PinToken::<02> },
            p703: P703 { _mode: PhantomData, _token: PinToken::<03> },
            p704: P704 { _mode: PhantomData, _token: PinToken::<04> },
            p705: P705 { _mode: PhantomData, _token: PinToken::<05> },
            p706: P706 { _mode: PhantomData, _token: PinToken::<06> },
            p707: P707 { _mode: PhantomData, _token: PinToken::<07> },
            p708: P708 { _mode: PhantomData, _token: PinToken::<08> },
            p709: P709 { _mode: PhantomData, _token: PinToken::<09> },
            p710: P710 { _mode: PhantomData, _token: PinToken::<10> },
            p711: P711 { _mode: PhantomData, _token: PinToken::<11> },
            p712: P712 { _mode: PhantomData, _token: PinToken::<12> },
            p713: P713 { _mode: PhantomData, _token: PinToken::<13> },
        }
    }
}

gpio_pin_pfs_7!     (       0);
gpio_pin_input!     (7, 0, 00);
// no_irq
gpio_pin_output!    (7,    00);
gpio_pin_drive!     (7,    00);
gpio_pin_alternate! (7,    00);

gpio_pin_pfs_7!     (       1);
gpio_pin_input!     (7, 0, 01);
// no_irq
gpio_pin_output!    (7,    01);
gpio_pin_drive!     (7,    01);
gpio_pin_alternate! (7,    01);

gpio_pin_pfs_7!     (       2);
gpio_pin_input!     (7, 0, 02);
// no_irq
gpio_pin_output!    (7,    02);
gpio_pin_drive!     (7,    02);
gpio_pin_alternate! (7,    02);

gpio_pin_pfs_7!     (       3);
gpio_pin_input!     (7, 0, 03);
// no_irq
gpio_pin_output!    (7,    03);
gpio_pin_drive!     (7,    03);
gpio_pin_alternate! (7,    03);

gpio_pin_pfs_7!     (       4);
gpio_pin_input!     (7, 0, 04);
// no_irq
gpio_pin_output!    (7,    04);
gpio_pin_drive!     (7,    04);
gpio_pin_alternate! (7,    04);

gpio_pin_pfs_7!     (       5);
gpio_pin_input!     (7, 0, 05);
// no_irq
gpio_pin_output!    (7,    05);
gpio_pin_drive!     (7,    05);
gpio_pin_alternate! (7,    05);

gpio_pin_pfs_7!     (       6);
gpio_pin_input!     (7, 0, 06);
gpio_pin_irq!       (7,    06);
gpio_pin_output!    (7,    06);
gpio_pin_drive!     (7,    06);
gpio_pin_alternate! (7,    06);

gpio_pin_pfs_7!     (       7);
gpio_pin_input!     (7, 0, 07);
gpio_pin_irq!       (7,    07);
gpio_pin_output!    (7,    07);
gpio_pin_drive!     (7,    07);
gpio_pin_alternate! (7,    07);

gpio_pin_pfs!       (7,    08);
gpio_pin_input!     (7, 0, 08);
gpio_pin_irq!       (7,    08);
gpio_pin_output!    (7,    08);
gpio_pin_drive!     (7,    08);
gpio_pin_alternate! (7,    08);

gpio_pin_pfs!       (7,    09);
gpio_pin_input!     (7, 0, 09);
gpio_pin_irq!       (7,    09);
gpio_pin_output!    (7,    09);
gpio_pin_drive!     (7,    09);
gpio_pin_alternate! (7,    09);

gpio_pin_pfs!       (7,    10);
gpio_pin_input!     (7, 0, 10);
// no_irq
gpio_pin_output!    (7,    10);
gpio_pin_drive!     (7,    10);
gpio_pin_alternate! (7,    10);

gpio_pin_pfs!       (7,    11);
gpio_pin_input!     (7, 0, 11);
// no_irq
gpio_pin_output!    (7,    11);
gpio_pin_drive!     (7,    11);
gpio_pin_alternate! (7,    11);

gpio_pin_pfs!       (7,    12);
gpio_pin_input!     (7, 0, 12);
// no_irq
gpio_pin_output!    (7,    12);
gpio_pin_drive!     (7,    12);
gpio_pin_alternate! (7,    12);

gpio_pin_pfs!       (7,    13);
gpio_pin_input!     (7, 0, 13);
// no_irq
gpio_pin_output!    (7,    13);
gpio_pin_drive!     (7,    13);
gpio_pin_alternate! (7,    13);