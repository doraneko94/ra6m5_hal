use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};

use critical_section::Mutex;

use crate::{pac, InitError};

pub mod iel;

static ICU_CELL: Mutex<RefCell<Option<pac::Icu>>> = Mutex::new(RefCell::new(None));
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Icu;

impl Icu {
    pub fn init(icu: pac::Icu) -> Result<iel::Iel, InitError> {
        if INITIALIZED.swap(true, Ordering::AcqRel) {
            return Err(InitError::AlreadyInit);
        }
        critical_section::with(|cs| {
            *ICU_CELL.borrow(cs).borrow_mut() = Some(icu);
        });
        Ok(iel::Iel {
            _id: (),
            iel0: iel::Iel0::new(),
            iel1: iel::Iel1::new(),
            iel2: iel::Iel2::new(),
            iel3: iel::Iel3::new(),
            iel4: iel::Iel4::new(),
            iel5: iel::Iel5::new(),
            iel6: iel::Iel6::new(),
            iel7: iel::Iel7::new(),
            iel8: iel::Iel8::new(),
            iel9: iel::Iel9::new(),
            iel10: iel::Iel10::new(),
            iel11: iel::Iel11::new(),
            iel12: iel::Iel12::new(),
            iel13: iel::Iel13::new(),
            iel14: iel::Iel14::new(),
            iel15: iel::Iel15::new(),
            iel16: iel::Iel16::new(),
            iel17: iel::Iel17::new(),
            iel18: iel::Iel18::new(),
            iel19: iel::Iel19::new(),
            iel20: iel::Iel20::new(),
            iel21: iel::Iel21::new(),
            iel22: iel::Iel22::new(),
            iel23: iel::Iel23::new(),
            iel24: iel::Iel24::new(),
            iel25: iel::Iel25::new(),
            iel26: iel::Iel26::new(),
            iel27: iel::Iel27::new(),
            iel28: iel::Iel28::new(),
            iel29: iel::Iel29::new(),
            iel30: iel::Iel30::new(),
            iel31: iel::Iel31::new(),
            iel32: iel::Iel32::new(),
            iel33: iel::Iel33::new(),
            iel34: iel::Iel34::new(),
            iel35: iel::Iel35::new(),
            iel36: iel::Iel36::new(),
            iel37: iel::Iel37::new(),
            iel38: iel::Iel38::new(),
            iel39: iel::Iel39::new(),
            iel40: iel::Iel40::new(),
            iel41: iel::Iel41::new(),
            iel42: iel::Iel42::new(),
            iel43: iel::Iel43::new(),
            iel44: iel::Iel44::new(),
            iel45: iel::Iel45::new(),
            iel46: iel::Iel46::new(),
            iel47: iel::Iel47::new(),
            iel48: iel::Iel48::new(),
            iel49: iel::Iel49::new(),
            iel50: iel::Iel50::new(),
            iel51: iel::Iel51::new(),
            iel52: iel::Iel52::new(),
            iel53: iel::Iel53::new(),
            iel54: iel::Iel54::new(),
            iel55: iel::Iel55::new(),
            iel56: iel::Iel56::new(),
            iel57: iel::Iel57::new(),
            iel58: iel::Iel58::new(),
            iel59: iel::Iel59::new(),
            iel60: iel::Iel60::new(),
            iel61: iel::Iel61::new(),
            iel62: iel::Iel62::new(),
            iel63: iel::Iel63::new(),
            iel64: iel::Iel64::new(),
            iel65: iel::Iel65::new(),
            iel66: iel::Iel66::new(),
            iel67: iel::Iel67::new(),
            iel68: iel::Iel68::new(),
            iel69: iel::Iel69::new(),
            iel70: iel::Iel70::new(),
            iel71: iel::Iel71::new(),
            iel72: iel::Iel72::new(),
            iel73: iel::Iel73::new(),
            iel74: iel::Iel74::new(),
            iel75: iel::Iel75::new(),
            iel76: iel::Iel76::new(),
            iel77: iel::Iel77::new(),
            iel78: iel::Iel78::new(),
            iel79: iel::Iel79::new(),
            iel80: iel::Iel80::new(),
            iel81: iel::Iel81::new(),
            iel82: iel::Iel82::new(),
            iel83: iel::Iel83::new(),
            iel84: iel::Iel84::new(),
            iel85: iel::Iel85::new(),
            iel86: iel::Iel86::new(),
            iel87: iel::Iel87::new(),
            iel88: iel::Iel88::new(),
            iel89: iel::Iel89::new(),
            iel90: iel::Iel90::new(),
            iel91: iel::Iel91::new(),
            iel92: iel::Iel92::new(),
            iel93: iel::Iel93::new(),
            iel94: iel::Iel94::new(),
            iel95: iel::Iel95::new(),
        })
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IcuEvent {
    Prohibited = 0x000,
    USBFS0_D0FIFO = 0x06b, USBFS0_D1FIFO = 0x06c, USBFS0_USBI = 0x06d, USBFS0_USBR = 0x06e, 
}
impl IcuEvent {
    pub fn is_dtc_allowed(&self) -> bool {
        match self {
            Self::USBFS0_D0FIFO | Self::USBFS0_D1FIFO => true,
            _ => false
        }
    }
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0x000 => Some(Self::Prohibited),
            0x06b => Some(Self::USBFS0_D0FIFO), 0x06c => Some(Self::USBFS0_D1FIFO), 
            0x06d => Some(Self::USBFS0_USBI), 0x06e => Some(Self::USBFS0_USBR), 
            _ => None
        }
    }
}
