use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};
use critical_section::Mutex;

use crate::{pac, InitError};

pub(crate) static DBG_CELL: Mutex<RefCell<Option<pac::Dbg>>> = Mutex::new(RefCell::new(None));
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Dbg;

impl Dbg {
    pub fn init(dbg: pac::Dbg) -> Result<Dbg, InitError> {
        if INITIALIZED.swap(true, Ordering::AcqRel) {
            return Err(InitError::AlreadyInit);
        }
        critical_section::with(|cs| {
            *DBG_CELL.borrow(cs).borrow_mut() = Some(dbg);
        });
        Ok(Dbg)
    }
}