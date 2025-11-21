use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};
use critical_section::Mutex;

use crate::{pac, InitError, RegisterError};

pub(crate) static FCACHE_CELL: Mutex<RefCell<Option<pac::Fcache>>> = Mutex::new(RefCell::new(None));
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Fcache;

impl Fcache {
    pub fn init(fcache: pac::Fcache) -> Result<Fcache, InitError> {
        if INITIALIZED.swap(true, Ordering::AcqRel) {
            return Err(InitError::AlreadyInit);
        }
        critical_section::with(|cs| {
            *FCACHE_CELL.borrow(cs).borrow_mut() = Some(fcache);
        });
        Ok(Fcache)
    }
    fn _with_cs<R>(&mut self, f: impl FnOnce(&mut pac::Fcache) -> R) -> R {
        critical_section::with(|cs| {
            let mut bor_fcache = FCACHE_CELL.borrow(cs).borrow_mut();
            let fcache = bor_fcache.as_mut().expect("FCACHE is not initialized");

            f(fcache)
        })
    }
    pub fn get_flash_wait(&mut self) -> Option<FlashWait> {
        self._with_cs(|fcache| unsafe {
            FlashWait::from_u8(fcache.flwt().read().flwt().get().0)
        })
    }
    pub fn set_flash_wait(&mut self, wait: FlashWait) -> Result<(), RegisterError> {
        self._with_cs(|fcache| unsafe {
            fcache.flwt().write(fcache.flwt().read().flwt().set(
                match wait {
                    FlashWait::Wait0 => pac::fcache::flwt::Flwt::_000,
                    FlashWait::Wait1 => pac::fcache::flwt::Flwt::_001,
                    FlashWait::Wait2 => pac::fcache::flwt::Flwt::_010,
                    FlashWait::Wait3 => pac::fcache::flwt::Flwt::_011,
                }
            ));
        });
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FlashWait { Wait0=0b000, Wait1=0b001, Wait2=0b010, Wait3=0b011 }
impl FlashWait {
    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> { match value {
        0b000=>Some(Self::Wait0), 0b001=>Some(Self::Wait1), 0b010=>Some(Self::Wait2), 
        0b011=>Some(Self::Wait3), _=>None
    } }
    pub fn from_iclk(iclk: u32) -> Option<Self> {
        if iclk <= 50_000_000 { Some(Self::Wait0) }
        else if iclk <= 100_000_000 { Some(Self::Wait1) }
        else if iclk <= 150_000_000 { Some(Self::Wait2) }
        else if iclk <= 200_000_000 { Some(Self::Wait3) }
        else { None }
    }
}
