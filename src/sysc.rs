use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};
use critical_section::Mutex;

use crate::{pac, InitError};

pub mod clock;
pub mod mstp;

static SYSC_CELL: Mutex<RefCell<Option<pac::Sysc>>> = Mutex::new(RefCell::new(None));
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Sysc;

impl Sysc {
    pub fn init(sysc: pac::Sysc) -> Result<(clock::Clocks, mstp::Mstp), InitError> {
        if INITIALIZED.swap(true, Ordering::AcqRel) {
            return Err(InitError::AlreadyInit);
        }
        critical_section::with(|cs| {
            *SYSC_CELL.borrow(cs).borrow_mut() = Some(sysc);
        });
        Ok((clock::Clocks, mstp::Mstp))
    }
}