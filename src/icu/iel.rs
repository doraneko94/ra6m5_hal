use crate::pac;
use crate::{RegisterError, impl_with_cs};

use super::{ICU_CELL, IcuEvent};

use pac::RegisterValue;

use paste::paste;


pub struct Iel {
    pub(crate)_id: (),
    pub iel0: Iel0,
    pub iel1: Iel1,
    pub iel2: Iel2,
    pub iel3: Iel3,
    pub iel4: Iel4,
    pub iel5: Iel5,
    pub iel6: Iel6,
    pub iel7: Iel7,
    pub iel8: Iel8,
    pub iel9: Iel9,
    pub iel10: Iel10,
    pub iel11: Iel11,
    pub iel12: Iel12,
    pub iel13: Iel13,
    pub iel14: Iel14,
    pub iel15: Iel15,
    pub iel16: Iel16,
    pub iel17: Iel17,
    pub iel18: Iel18,
    pub iel19: Iel19,
    pub iel20: Iel20,
    pub iel21: Iel21,
    pub iel22: Iel22,
    pub iel23: Iel23,
    pub iel24: Iel24,
    pub iel25: Iel25,
    pub iel26: Iel26,
    pub iel27: Iel27,
    pub iel28: Iel28,
    pub iel29: Iel29,
    pub iel30: Iel30,
    pub iel31: Iel31,
    pub iel32: Iel32,
    pub iel33: Iel33,
    pub iel34: Iel34,
    pub iel35: Iel35,
    pub iel36: Iel36,
    pub iel37: Iel37,
    pub iel38: Iel38,
    pub iel39: Iel39,
    pub iel40: Iel40,
    pub iel41: Iel41,
    pub iel42: Iel42,
    pub iel43: Iel43,
    pub iel44: Iel44,
    pub iel45: Iel45,
    pub iel46: Iel46,
    pub iel47: Iel47,
    pub iel48: Iel48,
    pub iel49: Iel49,
    pub iel50: Iel50,
    pub iel51: Iel51,
    pub iel52: Iel52,
    pub iel53: Iel53,
    pub iel54: Iel54,
    pub iel55: Iel55,
    pub iel56: Iel56,
    pub iel57: Iel57,
    pub iel58: Iel58,
    pub iel59: Iel59,
    pub iel60: Iel60,
    pub iel61: Iel61,
    pub iel62: Iel62,
    pub iel63: Iel63,
    pub iel64: Iel64,
    pub iel65: Iel65,
    pub iel66: Iel66,
    pub iel67: Iel67,
    pub iel68: Iel68,
    pub iel69: Iel69,
    pub iel70: Iel70,
    pub iel71: Iel71,
    pub iel72: Iel72,
    pub iel73: Iel73,
    pub iel74: Iel74,
    pub iel75: Iel75,
    pub iel76: Iel76,
    pub iel77: Iel77,
    pub iel78: Iel78,
    pub iel79: Iel79,
    pub iel80: Iel80,
    pub iel81: Iel81,
    pub iel82: Iel82,
    pub iel83: Iel83,
    pub iel84: Iel84,
    pub iel85: Iel85,
    pub iel86: Iel86,
    pub iel87: Iel87,
    pub iel88: Iel88,
    pub iel89: Iel89,
    pub iel90: Iel90,
    pub iel91: Iel91,
    pub iel92: Iel92,
    pub iel93: Iel93,
    pub iel94: Iel94,
    pub iel95: Iel95,
}

pub struct Iel0 {
    _id: u8,
}

impl_with_cs!(Iel0, Icu);

impl Iel0 {
    pub(crate) fn new() -> Self {
        Self { _id: 0 }
    }
    pub fn is_fired(&mut self) -> bool {
        self._with_cs(|icu| unsafe {
            ((icu.ielsr0().read().get_raw() >> 16) & 1) == 1
        })
    }
    pub fn reset_fire(&mut self) -> Result<(), RegisterError> {
        if self.is_dtce_enabled() {
            return Err(RegisterError::NotReadyToWrite("[IEL0] DTC is working."));
        }
        self._with_cs(|icu| unsafe {
            let r = icu.ielsr0().read().get_raw();
            icu.ielsr0().write_raw(r | (1 << 16));
        });
        Ok(())
    }
    pub fn on_interrupt() -> Result<(), RegisterError> {
        critical_section::with(|cs| unsafe {
            let mut bor = ICU_CELL.borrow(cs).borrow_mut();
            let icu = bor.as_mut().expect("Icu is not initialized");
            let r = icu.ielsr0().read().get_raw();
            if (r >> 24 & 1) == 0 {
                icu.ielsr0().write_raw(r & !(1 << 16));
            }
        });
        Ok(())
    }
    pub fn is_dtce_enabled(&mut self) -> bool {
        self._with_cs(|icu| unsafe {
            ((icu.ielsr0().read().get_raw() >> 24) & 1) == 1
        })
    }
    pub fn enable_dtce(&mut self) -> Result<(), RegisterError> {
        let event = self.get_event();
        match event {
            Some(e) => if e.is_dtc_allowed() {
                return Err(RegisterError::ProhibitedOperation("[IEL0] DTC is not allowed for this event."));
            }
            None => {
                return Err(RegisterError::InvalidValue("[IEL0] An invalid event is given."));
            }
        }
        self._with_cs(|icu| unsafe {
            let r = icu.ielsr0().read().get_raw();
            icu.ielsr0().write_raw(r | (1 << 24));
        });
        Ok(())
    }
    pub fn disable_dtce(&mut self) -> Result<(), RegisterError> {
        self._with_cs(|icu| unsafe {
            let r = icu.ielsr0().read().get_raw();
            let mask = !(1u32 << 24);
            icu.ielsr0().write_raw(r & mask);
        });
        Ok(())
    }
    pub fn get_event(&mut self) -> Option<IcuEvent> {
        self._with_cs(|icu| unsafe {
            IcuEvent::from_u32(icu.ielsr0().read().get_raw() & 0x1ff)
        })
    }
    pub fn set_event(&mut self, event: IcuEvent) -> Result<(), RegisterError> {
        if self.is_fired() {
            return Err(RegisterError::NotReadyToWrite("[IEL0] Event is fired."));
        }
        self._with_cs(|icu| unsafe {
            let r = icu.ielsr0().read().get_raw();
            icu.ielsr0().write_raw(((r >> 8) << 8) | (event as u32));
        });
        Ok(())
    }
}

macro_rules! ieln {
    ($id:tt) => {
        paste! {
            pub struct [<Iel $id>] {
                _id: u8,
            }

            impl_with_cs!([<Iel $id>], Icu);

            impl [<Iel $id>] {
                pub(crate) fn new() -> Self {
                    Self { _id: $id }
                }
                pub fn is_fired(&mut self) -> bool {
                    self._with_cs(|icu| unsafe {
                        ((icu.[<ielsr $id>]().read().get_raw() >> 16) & 1) == 1
                    })
                }
                pub fn reset_fire(&mut self) -> Result<(), RegisterError> {
                    if self.is_dtce_enabled() {
                        return Err(RegisterError::NotReadyToWrite(concat!("[IEL", stringify!($id), "] DTC is working.")));
                    }
                    self._with_cs(|icu| unsafe {
                        let r = icu.[<ielsr $id>]().read().get_raw();
                        icu.[<ielsr $id>]().write_raw(r & !(1u32 << 16));
                    });
                    Ok(())
                }
                pub fn on_interrupt() -> Result<(), RegisterError> {
                    critical_section::with(|cs| unsafe {
                        let mut bor = ICU_CELL.borrow(cs).borrow_mut();
                        let icu = bor.as_mut().expect("Icu is not initialized");
                        let r = icu.[<ielsr $id>]().read().get_raw();
                        if (r >> 24 & 1) == 0 {
                            icu.[<ielsr $id>]().write_raw(r & !(1 << 16));
                        }
                    });
                    Ok(())
                }
                pub fn is_dtce_enabled(&mut self) -> bool {
                    self._with_cs(|icu| unsafe {
                        ((icu.[<ielsr $id>]().read().get_raw() >> 24) & 1) == 1
                    })
                }
                pub fn enable_dtce(&mut self) -> Result<(), RegisterError> {
                    let event = self.get_event();
                    match event {
                        Some(e) => if e.is_dtc_allowed() {
                            return Err(RegisterError::ProhibitedOperation(concat!("[IEL", stringify!($id), "] DTC is not allowed for this event.")));
                        }
                        None => {
                            return Err(RegisterError::InvalidValue(concat!("[IEL", stringify!($id), "] An invalid event is given.")));
                        }
                    }
                    self._with_cs(|icu| unsafe {
                        let r = icu.[<ielsr $id>]().read().get_raw();
                        icu.[<ielsr $id>]().write_raw(r | (1 << 24));
                    });
                    Ok(())
                }
                pub fn disable_dtce(&mut self) -> Result<(), RegisterError> {
                    self._with_cs(|icu| unsafe {
                        let r = icu.[<ielsr $id>]().read().get_raw();
                        let mask = !(1u32 << 24);
                        icu.[<ielsr $id>]().write_raw(r & mask);
                    });
                    Ok(())
                }
                pub fn get_event(&mut self) -> Option<IcuEvent> {
                    self._with_cs(|icu| unsafe {
                        IcuEvent::from_u32(icu.[<ielsr $id>]().read().get_raw() & 0x1ff)
                    })
                }
                pub fn set_event(&mut self, event: IcuEvent) -> Result<(), RegisterError> {
                    if self.is_fired() {
                        return Err(RegisterError::NotReadyToWrite(concat!("[IEL", stringify!($id), "] Event is fired.")));
                    }
                    self._with_cs(|icu| unsafe {
                        let r = icu.[<ielsr $id>]().read().get_raw();
                        icu.[<ielsr $id>]().write_raw(((r >> 8) << 8) | (event as u32));
                    });
                    Ok(())
                }
            }
        }
    };
}

ieln!(1);
ieln!(2);
ieln!(3);
ieln!(4);
ieln!(5);
ieln!(6);
ieln!(7);
ieln!(8);
ieln!(9);
ieln!(10);
ieln!(11);
ieln!(12);
ieln!(13);
ieln!(14);
ieln!(15);
ieln!(16);
ieln!(17);
ieln!(18);
ieln!(19);
ieln!(20);
ieln!(21);
ieln!(22);
ieln!(23);
ieln!(24);
ieln!(25);
ieln!(26);
ieln!(27);
ieln!(28);
ieln!(29);
ieln!(30);
ieln!(31);
ieln!(32);
ieln!(33);
ieln!(34);
ieln!(35);
ieln!(36);
ieln!(37);
ieln!(38);
ieln!(39);
ieln!(40);
ieln!(41);
ieln!(42);
ieln!(43);
ieln!(44);
ieln!(45);
ieln!(46);
ieln!(47);
ieln!(48);
ieln!(49);
ieln!(50);
ieln!(51);
ieln!(52);
ieln!(53);
ieln!(54);
ieln!(55);
ieln!(56);
ieln!(57);
ieln!(58);
ieln!(59);
ieln!(60);
ieln!(61);
ieln!(62);
ieln!(63);
ieln!(64);
ieln!(65);
ieln!(66);
ieln!(67);
ieln!(68);
ieln!(69);
ieln!(70);
ieln!(71);
ieln!(72);
ieln!(73);
ieln!(74);
ieln!(75);
ieln!(76);
ieln!(77);
ieln!(78);
ieln!(79);
ieln!(80);
ieln!(81);
ieln!(82);
ieln!(83);
ieln!(84);
ieln!(85);
ieln!(86);
ieln!(87);
ieln!(88);
ieln!(89);
ieln!(90);
ieln!(91);
ieln!(92);
ieln!(93);
ieln!(94);
ieln!(95);