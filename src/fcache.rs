//#![allow(dead_code)]

use crate::pac;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FlashWait { Wait0, Wait1, Wait2, Wait3 }
impl FlashWait {
    #[inline]
    pub fn raw(self) -> u8 {
        match self { Self::Wait0=>0b000, Self::Wait1=>0b001, Self::Wait2=>0b010, Self::Wait3=>0b011 }
    }
    #[inline]
    pub fn from_raw(v: u8) -> Result<Self, Error> {
        match v {
            0 => Ok(Self::Wait0),
            1 => Ok(Self::Wait1),
            2 => Ok(Self::Wait2),
            3 => Ok(Self::Wait3),
            _ => Err(Error::InvalidValue("FLWT out of range (0..=3)"))
        }
    }
    pub fn from_iclk(iclk: u32) -> Option<Self> {
        if iclk <= 50_000_000 { Some(Self::Wait0) }
        else if iclk <= 100_000_000 { Some(Self::Wait1) }
        else if iclk <= 150_000_000 { Some(Self::Wait2) }
        else if iclk <= 200_000_000 { Some(Self::Wait3) }
        else { None }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidValue(&'static str),
}

#[inline]
pub fn current_flash_wait() -> Result<FlashWait, Error> {
    let raw = unsafe { pac::FCACHE.flwt().read().flwt().get().0 };
    FlashWait::from_raw(raw)
}

pub fn set_flash_wait(wait: FlashWait) -> Result<FlashWait, Error> {
    unsafe {
        pac::FCACHE.flwt().modify(|r| r.flwt().set(wait.raw().into()));
    }
    Ok(current_flash_wait()?)
}