use crate::pac;

#[inline]
pub fn current_fclk_mhz() -> u8 {
    unsafe { pac::FLAD.fckmhz().read().fckmhz().get() }
}

pub fn set_fclk_mhz(mhz: u8) -> u8 {
    unsafe {
        pac::FLAD.fckmhz().modify(|r| r.fckmhz().set(mhz));
    }
    current_fclk_mhz()
} 