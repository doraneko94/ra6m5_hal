use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};
use critical_section::Mutex;

use crate::{pac, InitError};

pub(crate) static OFS1_CELL: Mutex<RefCell<Option<pac::Rtc>>> = Mutex::new(RefCell::new(None));
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Ofs1;

impl Ofs1 {
    pub fn init(ofs1: pac::Ofs1) -> Result<Ofs1, InitError> {
        if INITIALIZED.swap(true, Ordering::AcqRel) {
            return Err(InitError::AlreadyInit);
        }
        critical_section::with(|cs| {
            *OFS1_CELL.borrow(cs).borrow_mut() = Some(ofs1);
        });
        Ok(Ofs1)
    }
}