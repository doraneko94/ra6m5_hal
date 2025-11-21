use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};
use critical_section::Mutex;

use crate::{pac, InitError};

pub(crate) static MSTP_CELL: Mutex<RefCell<Option<pac::Mstp>>> = Mutex::new(RefCell::new(None));
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Mstp {
    _id: ()
}

impl Mstp {
    pub fn init(mstp: pac::Mstp) -> Result<Mstp, InitError> {
        if INITIALIZED.swap(true, Ordering::AcqRel) {
            return Err(InitError::AlreadyInit);
        }
        critical_section::with(|cs| {
            *MSTP_CELL.borrow(cs).borrow_mut() = Some(mstp);
        });
        Ok(Mstp { _id: () })
    }
}