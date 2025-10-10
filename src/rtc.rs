use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};
use critical_section::Mutex;

use crate::{pac, InitError};

pub(crate) static RTC_CELL: Mutex<RefCell<Option<pac::Rtc>>> = Mutex::new(RefCell::new(None));
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Rtc;

impl Rtc {
    pub fn init(rtc: pac::Rtc) -> Result<Rtc, InitError> {
        if INITIALIZED.swap(true, Ordering::AcqRel) {
            return Err(InitError::AlreadyInit);
        }
        critical_section::with(|cs| {
            *RTC_CELL.borrow(cs).borrow_mut() = Some(rtc);
        });
        Ok(Rtc)
    }
}

