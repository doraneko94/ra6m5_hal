use crate::pac;

pub unsafe fn clock_write_enable(enable: bool) {
    unsafe {
        pac::SYSC.prcr().write(
            pac::SYSC.prcr().read()
                .prkey().set(0xA5)
                .prc0().set(if enable { pac::sysc::prcr::Prc0::_1 } else { pac::sysc::prcr::Prc0::_0 })
        )
    }
}
pub unsafe fn clock_write_is_enabled() -> bool {
    unsafe {
        pac::SYSC.prcr().read().prc0().get() == pac::sysc::prcr::Prc0::_1
    }
}

pub unsafe fn battery_write_enable(enable: bool) {
    unsafe {
        pac::SYSC.prcr().write(
            pac::SYSC.prcr().read()
                .prkey().set(0xA5)
                .prc1().set(if enable { pac::sysc::prcr::Prc1::_1 } else { pac::sysc::prcr::Prc1::_0 })
        )
    }
}
pub unsafe fn battery_write_is_enabled() -> bool {
    unsafe {
        pac::SYSC.prcr().read().prc1().get() == pac::sysc::prcr::Prc1::_1
    }
}

pub unsafe fn lvd_write_enable(enable: bool) {
    unsafe {
        pac::SYSC.prcr().write(
            pac::SYSC.prcr().read()
                .prkey().set(0xA5)
                .prc3().set(if enable { pac::sysc::prcr::Prc3::_1 } else { pac::sysc::prcr::Prc3::_0 })
        )
    }
}
pub unsafe fn lvd_write_is_enabled() -> bool {
    unsafe {
        pac::SYSC.prcr().read().prc3().get() == pac::sysc::prcr::Prc3::_1
    }
}

pub unsafe fn security_write_enable(enable: bool) {
    unsafe {
        pac::SYSC.prcr().write(
            pac::SYSC.prcr().read()
                .prkey().set(0xA5)
                .prc4().set(if enable { pac::sysc::prcr::Prc4::_1 } else { pac::sysc::prcr::Prc4::_0 })
        )
    }
}
pub unsafe fn security_write_is_enabled() -> bool {
    unsafe {
        pac::SYSC.prcr().read().prc4().get() == pac::sysc::prcr::Prc4::_1
    }
}