use core::cell::RefCell;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, Ordering};
use critical_section::Mutex;
use usb_device::{Result as UsbResult, UsbError};

use paste::paste;

use pac::RegisterValue;

use crate::pac;
use crate::{InitError, RegisterError, impl_with_cs, impl_with_cs_with};
use crate::sysc::clock::busy_wait_ns_with_dwt;
use crate::mstp::{Mstp, MSTP_CELL};

//pub mod bus;
pub mod descriptor;
pub mod interrupt;

pub(crate) static USBFS_CELL: Mutex<RefCell<Option<pac::Usbfs>>> = Mutex::new(RefCell::new(None));
pub(crate) static USBHS_CELL: Mutex<RefCell<Option<pac::Usbhs>>> = Mutex::new(RefCell::new(None));
static INITIALIZED_FS: AtomicBool = AtomicBool::new(false);
static INITIALIZED_HS: AtomicBool = AtomicBool::new(false);

// pub(crate) static LAST_SETUP: Mutex<RefCell<Option<UsbSetupPacket>>> = Mutex::new(RefCell::new(None));

pub struct UsbPeripheral<S> {
    _s: PhantomData<S>,
}

pub type UsbFs = UsbPeripheral<pac::Usbfs>;
pub type UsbHs = UsbPeripheral<pac::Usbhs>;

impl_with_cs!(UsbFs, Usbfs);
impl_with_cs_with!(UsbFs, Usbfs, Mstp);
impl_with_cs!(UsbHs, Usbhs);
impl_with_cs_with!(UsbHs, Usbhs, Mstp);

impl UsbFs {
    pub fn init(usbfs: pac::Usbfs) -> Result<Self, InitError> {
        if INITIALIZED_FS.swap(true, Ordering::AcqRel) {
            return Err(InitError::AlreadyInit);
        }
        critical_section::with(|cs| {
            *USBFS_CELL.borrow(cs).borrow_mut() = Some(usbfs);
        });
        Ok(Self { _s: PhantomData })
    }
    /*pub(crate) fn _store_last_setup(pkt: UsbSetupPacket) {
        critical_section::with(|cs| {
            *LAST_SETUP.borrow(cs).borrow_mut() = Some(pkt);
        })
    }
    pub fn take_last_setup() -> Option<UsbSetupPacket> {
        critical_section::with(|cs| {
            LAST_SETUP.borrow(cs).borrow_mut().take()
        })
    }*/
    /*pub fn _store_last_setup(&mut self, pkt: UsbSetupPacket) {
        self.last_setup = Some(pkt);
    }
    pub fn take_last_setup(&self) -> Option<UsbSetupPacket> {
        self.last_setup
    }*/
    pub fn _is_clock_enabled(&mut self) -> bool {
        self._with_cs(|usbfs| unsafe {
            usbfs.syscfg().read().scke().get() == pac::usbfs::syscfg::Scke::_1
        })
    }
    pub fn _set_scke(&mut self, enable: bool) -> Result<(), RegisterError> {
        self._with_cs(|usbfs| unsafe {
            let r = usbfs.syscfg().read();
            usbfs.syscfg().write(r.scke().set(
                if enable { pac::usbfs::syscfg::Scke::_1 } 
                else { pac::usbfs::syscfg::Scke::_0 }))
        });
        Ok(())
    }
    pub fn _enable_clock(&mut self) -> Result<(), RegisterError> { self._set_scke(true) }
    pub fn _disable_clock(&mut self) -> Result<(), RegisterError> { self._set_scke(false) }
    pub fn is_enabled(&mut self, _mstp: &mut Mstp) -> bool {
        self._with_cs_mstp(|_usbfs, mstp| unsafe {
            mstp.mstpcrb().read().mstpb11().get() == pac::mstp::mstpcrb::Mstpb11::_0
        }) 
        && self._is_clock_enabled()
        && self._with_cs(|usbfs| unsafe {
            let r = usbfs.syscfg().read();
            r.usbe().get() == pac::usbfs::syscfg::Usbe::_1
            && r.dcfm().get() == pac::usbfs::syscfg::Dcfm::_0
        })
    }
    pub fn enable(&mut self, mstp: &mut Mstp) -> Result<(), RegisterError> {
        if self.is_enabled(mstp) {
            return Err(RegisterError::NotReadyToWrite("USBFS device is already enabled."));
        }

        self._with_cs_mstp(|_usbfs, mstp| unsafe {
            let r = mstp.mstpcrb().read();
            mstp.mstpcrb().write(r.mstpb11().set(pac::mstp::mstpcrb::Mstpb11::_0));            
        });

        self._enable_clock().unwrap();

        self._with_cs(|usbfs| unsafe {
            let r = usbfs.syscfg().read();
            usbfs.syscfg().write(r
                .usbe().set(pac::usbfs::syscfg::Usbe::_1)
                .dcfm().set(pac::usbfs::syscfg::Dcfm::_0)
            )
        });

        Ok(())
    }
    pub fn disable(&mut self, mstp: &mut Mstp) -> Result<(), RegisterError> {
        if !self.is_enabled(mstp) {
            return Err(RegisterError::NotReadyToWrite("USBFS device is not enabled."));
        }
        if !self.is_connected(mstp) {
            return Err(RegisterError::NotReadyToWrite("USBFS device is connected."));
        }
        self._with_cs(|usbfs| unsafe {
            let r = usbfs.syscfg().read();
            usbfs.syscfg().write(r.usbe().set(pac::usbfs::syscfg::Usbe::_0))
        });

        self._disable_clock().unwrap();

        self._with_cs_mstp(|_usbfs, mstp| unsafe {
            let r = mstp.mstpcrb().read();
            mstp.mstpcrb().write(r.mstpb11().set(pac::mstp::mstpcrb::Mstpb11::_1));            
        });

        Ok(())
    }
    pub fn is_connected(&mut self, mstp: &mut Mstp) -> bool {
        self.is_enabled(mstp) && 
        self._with_cs(|usbfs| unsafe {
            usbfs.syscfg().read().dprpu().get() == pac::usbfs::syscfg::Dprpu::_1
        })
    }
    pub fn _set_dprpu(&mut self, mstp: &mut Mstp, connect: bool) -> Result<(), RegisterError> {
        if !self.is_enabled(mstp) {
            return Err(RegisterError::NotReadyToWrite("USBFS device is not initialized."));
        }
        self._with_cs(|usbfs| unsafe {
            let r = usbfs.syscfg().read();
            usbfs.syscfg().write(r.dprpu().set(
                if connect { pac::usbfs::syscfg::Dprpu::_1 }
                else { pac::usbfs::syscfg::Dprpu::_0 }
            ))
        });
        Ok(())
    }
    pub fn connect(&mut self, mstp: &mut Mstp) -> Result<(), RegisterError> { self._set_dprpu(mstp, true) }
    pub fn disconnect(&mut self, mstp: &mut Mstp) -> Result<(), RegisterError> { self._set_dprpu(mstp, false) }
    pub fn is_usbi_interrupt_enabled(&mut self) -> bool {
        self._with_cs(|usbfs| unsafe {
            let r0 = usbfs.intenb0().read();
            let r1 = usbfs.intenb1().read();

            r0.dvse().get() == pac::usbfs::intenb0::Dvse::_1 &&
            r0.vbse().get() == pac::usbfs::intenb0::Vbse::_1 &&
            r1.attche().get() == pac::usbfs::intenb1::Attche::_1 &&
            r1.dtche().get() == pac::usbfs::intenb1::Dtche::_1 &&
            r1.bchge().get() == pac::usbfs::intenb1::Bchge::_1
        })
    }
    pub fn enable_usbi_interrupt(&mut self) -> Result<(), RegisterError> {
        self._with_cs(|usbfs| unsafe {
            let r0 = usbfs.intenb0().read();
            usbfs.intenb0().write(r0
                .dvse().set(pac::usbfs::intenb0::Dvse::_1)
                .vbse().set(pac::usbfs::intenb0::Vbse::_1)
                .ctre().set(pac::usbfs::intenb0::Ctre::_1)
            );
            let r1 = usbfs.intenb1().read();
            usbfs.intenb1().write(r1
                .attche().set(pac::usbfs::intenb1::Attche::_1)
                .dtche().set(pac::usbfs::intenb1::Dtche::_1)
                .bchge().set(pac::usbfs::intenb1::Bchge::_1)
            );
        });
        Ok(())
    }
    pub fn disable_usbi_interrupt(&mut self) -> Result<(), RegisterError> {
        self._with_cs(|usbfs| unsafe {
            let r0 = usbfs.intenb0().read();
            usbfs.intenb0().write(r0
                .dvse().set(pac::usbfs::intenb0::Dvse::_0)
                .vbse().set(pac::usbfs::intenb0::Vbse::_0)
                .ctre().set(pac::usbfs::intenb0::Ctre::_0)
            );
            let r1 = usbfs.intenb1().read();
            usbfs.intenb1().write(r1
                .attche().set(pac::usbfs::intenb1::Attche::_0)
                .dtche().set(pac::usbfs::intenb1::Dtche::_0)
                .bchge().set(pac::usbfs::intenb1::Bchge::_0)
            );
        });
        Ok(())
    }
    pub fn send_ep0_in(&mut self, data: &[u8]) -> Result<(), RegisterError> {
        self._with_cs(|usbfs| unsafe {
            // PID == NAK ?
            let r = usbfs.dcpctr().read();
            if r.pid().get() != pac::usbfs::dcpctr::Pid::_00 {
                if r.pbusy().get() == pac::usbfs::dcpctr::Pbusy::_0 {
                    usbfs.dcpctr().write(r.pid().set(pac::usbfs::dcpctr::Pid::_00));
                } else {
                    return Err(RegisterError::NotReadyToWrite("PID != NAK & Pipe is busy."));
                }
            }

            // MAX packet size
            let r = usbfs.dcpmaxp().read();
            usbfs.dcpmaxp().write(r.mxps().set(64));

            // DCPCFG - Control Pipe
            let r = usbfs.dcpcfg().read();
            usbfs.dcpcfg().write(r.dir().set(pac::usbfs::dcpcfg::Dir::_0));

            // CFIFO - DCP (IN)
            let r = usbfs.cfifosel().read();
            usbfs.cfifosel().write(r
                .curpipe().set(pac::usbfs::cfifosel::Curpipe::_0_X_0)
                .isel().set(pac::usbfs::cfifosel::Isel::_1)
                .mbw().set(pac::usbfs::cfifosel::Mbw::_1)
            );
            loop {
                let r = usbfs.cfifosel().read();
                if (r.curpipe().get() == pac::usbfs::cfifosel::Curpipe::_0_X_0) 
                && (r.isel().get() == pac::usbfs::cfifosel::Isel::_1) { break; }
            }

            // FIFO clear
            let r = usbfs.cfifoctr().read();
            usbfs.cfifoctr().write(r.bclr().set(pac::usbfs::cfifoctr::Bclr::_1));

            let mut i = 0;
            while i < data.len() {
                while usbfs.cfifoctr().read().frdy().get() == pac::usbfs::cfifoctr::Frdy::_0 {}

                if i + 1 < data.len() {
                    let v = (data[i] as u16) | ((data[i + 1] as u16) << 8);
                    usbfs.cfifo().write_raw(v);
                    i += 2;
                } else {
                    usbfs.cfifol().write_raw(data[i]);
                    i += 1;
                }
            }

            let r = usbfs.cfifoctr().read();
            usbfs.cfifoctr().write(r.bval().set(pac::usbfs::cfifoctr::Bval::_1));

            // Send
            let r = usbfs.dcpctr().read();
            usbfs.dcpctr().write(r
                .pid().set(pac::usbfs::dcpctr::Pid::_01)
                .sqclr().set(pac::usbfs::dcpctr::Sqclr::_1)
            );

            Ok(())
        })
    }
}

impl UsbHs {
    pub fn init(usbhs: pac::Usbhs) -> Result<Self, InitError> {
        if INITIALIZED_HS.swap(true, Ordering::AcqRel) {
            return Err(InitError::AlreadyInit);
        }
        critical_section::with(|cs| {
            *USBHS_CELL.borrow(cs).borrow_mut() = Some(usbhs);
        });
        Ok(Self { _s: PhantomData })
    }
    pub fn is_pll_locked(&mut self) -> bool {
        self._with_cs(|usbhs| unsafe {
            usbhs.pllsta().read().plllock().get() == pac::usbhs::pllsta::Plllock::_1
        })
    }
    pub fn is_enabled(&mut self, _mstp: &mut Mstp) -> bool {
        self._with_cs_mstp(|_usbhs, mstp| unsafe {
            mstp.mstpcrb().read().mstpb12().get() == pac::mstp::mstpcrb::Mstpb12::_0
        })
        && self._with_cs(|usbhs| unsafe {
            let r = usbhs.syscfg().read();
            r.usbe().get() == pac::usbhs::syscfg::Usbe::_1
                && r.hse().get() == pac::usbhs::syscfg::Hse::_1
        })
    }
    pub fn is_connected(&mut self, mstp: &mut Mstp) -> bool {
        self.is_enabled(mstp)
            && self._with_cs(|usbhs| unsafe {
                usbhs.syscfg().read().dprpu().get() == pac::usbhs::syscfg::Dprpu::_1
            })
    }
    pub fn config(
        &mut self, 
        mstp: &mut Mstp, 
        xtal_hz: u32,
        iclk_hz: u32
    ) -> Result<(), RegisterError> {
        if self.is_connected(mstp) {
            return Err(RegisterError::NotReadyToWrite(
                "USBHS device is already connected."
            ));
        }
        if self.is_enabled(mstp) {
            return Err(RegisterError::NotReadyToWrite(
                "USBHS device is already enabled."
            ));
        }

        self._with_cs_mstp(|_usbhs, mstp| unsafe {
            let r = mstp.mstpcrb().read();
            mstp.mstpcrb().write(
                r.mstpb12().set(pac::mstp::mstpcrb::Mstpb12::_0)
            );
        });

        self._with_cs(|usbhs| unsafe {
            let r = usbhs.syscfg().read();
            usbhs.syscfg().write(
                r.dcfm().set(pac::usbhs::syscfg::Dcfm::_0)
                    .hse().set(pac::usbhs::syscfg::Hse::_1)
            );
        });

        let clksel =match xtal_hz {
            12_000_000 => pac::usbhs::physet::Clksel::_00,
            20_000_000 => pac::usbhs::physet::Clksel::_10,
            24_000_000 => pac::usbhs::physet::Clksel::_11,
            _ => { return Err(RegisterError::InvalidValue("XTAL_HZ should be 12MHz, 20MHz or 24MHz")); }
        };

        self._with_cs(|usbhs| unsafe {
            let r = usbhs.physet().read();
            usbhs.physet().write(
                r.hseb().set(pac::usbhs::physet::Hseb::_0)
                    .clksel().set(clksel)
            );
        });

        busy_wait_ns_with_dwt(1_000, iclk_hz);

        self._with_cs(|usbhs| unsafe {
            let r = usbhs.physet().read();
            usbhs.physet().write(
                r.dirpd().set(pac::usbhs::physet::Dirpd::_0)
            );
        });

        busy_wait_ns_with_dwt(1_000_000, iclk_hz);

        self._with_cs(|usbhs| unsafe {
            let r = usbhs.physet().read();
            usbhs.physet().write(
                r.pllreset().set(pac::usbhs::physet::Pllreset::_0)
            );
        });

        self._with_cs(|usbhs| unsafe {
            let r = usbhs.lpsts().read();
            usbhs.lpsts().write(
                r.suspendm().set(pac::usbhs::lpsts::Suspendm::_1)
            );
        });

        for _ in 0..1_000_000 {
            if self.is_pll_locked() { break; }
        }
        if !self.is_pll_locked() {
            return Err(RegisterError::NotReadyToWrite("PLL lock is timed out."))
        }

        self._with_cs(|usbhs| unsafe {
            let r = usbhs.syscfg().read();
            usbhs.syscfg().write(
                r.dprpu().set(pac::usbhs::syscfg::Dprpu::_1)
                    .drpd().set(pac::usbhs::syscfg::Drpd::_0)
                    .usbe().set(pac::usbhs::syscfg::Usbe::_1)
            );
        });

        Ok(())
    }
    pub fn enable(&mut self, mstp: &mut Mstp) -> Result<(), RegisterError> {
        if self.is_connected(mstp) {
            return Err(RegisterError::NotReadyToWrite(
                "USBHS device is already connected."
            ));
        }
        if self.is_enabled(mstp) {
            return Err(RegisterError::NotReadyToWrite(
                "USBHS device is already enabled."
            ));
        }

        self._with_cs_mstp(|_usbhs, mstp| unsafe {
            let r = mstp.mstpcrb().read();
            mstp.mstpcrb().write(
                r.mstpb12().set(pac::mstp::mstpcrb::Mstpb12::_0)
            );
        });

        self._with_cs(|usbhs| unsafe {
            let r = usbhs.syscfg().read();
            usbhs.syscfg().write(
                r.usbe().set(pac::usbhs::syscfg::Usbe::_1)
            );
        });

        Ok(())
    }
    
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BusPoll {
    pub reset: bool,
    pub suspend: bool,
    pub resume: bool,
    pub ep_out: u16,
    pub ep_in_complete: u16,
    pub ep_setup: u16,
}

impl UsbHs {
    pub fn send_ep0_in(&mut self, data: &[u8]) -> Result<(), RegisterError> {
        self._with_cs(|usbhs| unsafe {
            // PID = NAK
            let r = usbhs.dcpctr().read();
            if r.pid().get() != pac::usbhs::dcpctr::Pid::_00 {
                if r.pbusy().get() == pac::usbhs::dcpctr::Pbusy::_0 {
                    usbhs.dcpctr().write(
                        r.pid().set(pac::usbhs::dcpctr::Pid::_00)
                    );
                } else {
                    return Err(RegisterError::NotReadyToWrite(
                        "EP0 busy"
                    ));
                }
            }

            // max packet
            let r = usbhs.dcpmaxp().read();
            usbhs.dcpmaxp().write(r.mxps().set(64));

            // IN direction
            let r = usbhs.dcpcfg().read();
            usbhs.dcpcfg().write(
                r.dir().set(pac::usbhs::dcpcfg::Dir::_0)
            );

            // FIFO select EP0 IN
            let r = usbhs.cfifosel().read();
            usbhs.cfifosel().write(
                r.curpipe().set(
                    pac::usbhs::cfifosel::Curpipe::_0_X_0
                )
                .isel().set(
                    pac::usbhs::cfifosel::Isel::_1
                )
                .mbw().set(
                    pac::usbhs::cfifosel::Mbw::_10
                )
            );

            while usbhs.cfifoctr().read().frdy().get()
                == pac::usbhs::cfifoctr::Frdy::_0 {}

            // clear FIFO
            let r = usbhs.cfifoctr().read();
            usbhs.cfifoctr().write(
                r.bclr().set(
                    pac::usbhs::cfifoctr::Bclr::_1
                )
            );

            // write payload
            let mut i = 0;
            while i < data.len() {
                while usbhs.cfifoctr().read().frdy().get()
                    == pac::usbhs::cfifoctr::Frdy::_0 {}

                if i + 1 < data.len() {
                    let v =
                        (data[i] as u32) |
                        ((data[i + 1] as u32) << 8);
                    usbhs.cfifo().write_raw(v);
                    i += 2;
                } else {
                    usbhs.cfifol().write_raw(data[i] as u16);
                    i += 1;
                }
            }

            // commit
            let r = usbhs.cfifoctr().read();
            usbhs.cfifoctr().write(
                r.bval().set(
                    pac::usbhs::cfifoctr::Bval::_1
                )
            );

            // BUF
            let r = usbhs.dcpctr().read();
            usbhs.dcpctr().write(
                r.pid().set(
                    pac::usbhs::dcpctr::Pid::_01
                )
                .sqclr().set(
                    pac::usbhs::dcpctr::Sqclr::_1
                )
            );

            Ok(())
        })
    }
    pub fn bus_enable_interrupts(&mut self) -> Result<(), RegisterError> {
        self._with_cs(|usbhs| unsafe {
            let r0 = usbhs.intenb0().read();
            usbhs.intenb0().write(
                r0
                    .brdye().set(pac::usbhs::intenb0::Brdye::_1)
                    .bempe().set(pac::usbhs::intenb0::Bempe::_1)
                    .ctre().set(pac::usbhs::intenb0::Ctre::_1)
                    .dvse().set(pac::usbhs::intenb0::Dvse::_1)
            );

            let r1 = usbhs.intenb1().read();
            usbhs.intenb1().write(
                r1
                    .signe().set(pac::usbhs::intenb1::Signe::_1)
                    .sacke().set(pac::usbhs::intenb1::Sacke::_1)
                    .attche().set(pac::usbhs::intenb1::Attche::_1)
                    .dtche().set(pac::usbhs::intenb1::Dtche::_1)
                    .bchge().set(pac::usbhs::intenb1::Bchge::_1)
            );
        });
        Ok(())
    }

    pub fn bus_clear_bus_events(&mut self) {
        self._with_cs(|usbhs| unsafe {
            let intsts0 = usbhs.intsts0().read();
            let intsts1 = usbhs.intsts1().read();

            if intsts0.brdy().get() == pac::usbhs::intsts0::Brdy::_1 {
                let brdysts = usbhs.brdysts().read().get_raw();
                let brdyenb = usbhs.brdyenb().read().get_raw();
                let pending = brdysts & brdyenb;

                if pending != 0 {
                    usbhs.brdysts().write_raw((!pending) as _);
                }
            }
            if intsts0.bemp().get() == pac::usbhs::intsts0::Bemp::_1 {
                let bempsts = usbhs.bempsts().read().get_raw();
                let bempenb = usbhs.bempenb().read().get_raw();
                let pending = bempsts & bempenb;

                if pending != 0 {
                    usbhs.brdysts().write_raw((!pending) as _);
                }
            }
            usbhs.intsts0().write(
                intsts0
                    .ctrt().set(pac::usbhs::intsts0::Ctrt::_0)
                    .dvst().set(pac::usbhs::intsts0::Dvst::_0)
            );

            usbhs.intsts1().write(
                intsts1
                    .sign().set(pac::usbhs::intsts1::Sign::_0)
                    .sack().set(pac::usbhs::intsts1::Sack::_0)
            );
        })
    }

    pub fn bus_poll(&mut self) -> BusPoll {
        self._with_cs(|usbhs| unsafe {
            let mut ev = BusPoll::default();

            let intsts0 = usbhs.intsts0().read();
            let intsts1 = usbhs.intsts1().read();

            if intsts0.dvst().get() == pac::usbhs::intsts0::Dvst::_1 {
                ev.reset = true;
            }
            if intsts0.ctrt().get() == pac::usbhs::intsts0::Ctrt::_1 {
                ev.ep_setup |= 1 << 0;
            }

            if intsts0.brdy().get() == pac::usbhs::intsts0::Brdy::_1 {
                let brdysts = usbhs.brdysts().read().get_raw();
                let brdyenb = usbhs.brdyenb().read().get_raw();
                let pending = brdysts & brdyenb;

                if (pending & (1 << 2)) != 0 {
                    ev.ep_out |= 1 << 2;
                }

                if pending != 0 {
                    usbhs.brdysts().write_raw((!pending) as _);
                }
            }

            if intsts0.bemp().get() == pac::usbhs::intsts0::Bemp::_1 {
                let bempsts = usbhs.bempsts().read().get_raw();
                let bempenb = usbhs.bempenb().read().get_raw();
                let pending = bempsts & bempenb;

                if (pending & (1 << 1)) != 0 {
                    ev.ep_in_complete |= 1 << 1;
                }
                if (pending & (1 << 3)) != 0 {
                    ev.ep_in_complete |= 1 << 3;
                }

                if pending != 0 {
                    usbhs.brdysts().write_raw((!pending) as _);
                }
            }

            if intsts1.sack().get() == pac::usbhs::intsts1::Sack::_1 {
                ev.suspend = true;
            }
            if intsts1.sign().get() == pac::usbhs::intsts1::Sign::_1 {
                ev.resume = true;
            }

            usbhs.intsts0().write(
                intsts0
                    .ctrt().set(pac::usbhs::intsts0::Ctrt::_0)
                    .dvst().set(pac::usbhs::intsts0::Dvst::_0)
            );
            usbhs.intsts1().write(
                intsts1
                    .sign().set(pac::usbhs::intsts1::Sign::_0)
                    .sack().set(pac::usbhs::intsts1::Sack::_0)
            );

            ev
        })
    }

    pub fn bus_set_address(&mut self, _addr: u8) {}

    pub fn bus_ep0_reset(&mut self, max_packet_size: u8) {
        self._with_cs(|usbhs| unsafe {
            let r = usbhs.dcpmaxp().read();
            usbhs.dcpmaxp().write(r.mxps().set(max_packet_size));

            let r = usbhs.dcpcfg().read();
            usbhs.dcpcfg().write(r.dir().set(pac::usbhs::dcpcfg::Dir::_0));

            let r = usbhs.dcpctr().read();
            usbhs.dcpctr().write(
                r
                    .pid().set(pac::usbhs::dcpctr::Pid::_00)
                    .sqclr().set(pac::usbhs::dcpctr::Sqclr::_1)
                    .sureqclr().set(pac::usbhs::dcpctr::Sureqclr::_1)
            );
        });
    }

    pub fn bus_ep0_write_in(&mut self, data: &[u8]) -> UsbResult<usize> {
        self.send_ep0_in(data).map_err(|_| UsbError::WouldBlock)?;
        Ok(data.len())
    }

    pub fn bus_ep0_read_out(&mut self, buf: &mut [u8]) -> UsbResult<usize> {
        self._with_cs(|usbhs| unsafe {
            let r = usbhs.cfifosel().read();
            usbhs.cfifosel().write(
                r
                    .curpipe().set(pac::usbhs::cfifosel::Curpipe::_0_X_0)
                    .isel().set(pac::usbhs::cfifosel::Isel::_0)
                    .mbw().set(pac::usbhs::cfifosel::Mbw::_10)
            );

            while usbhs.cfifoctr().read().frdy().get() == pac::usbhs::cfifoctr::Frdy::_0 {}

            let dtln = usbhs.cfifoctr().read().dtln().get() as usize;
            if dtln > buf.len() {
                return Err(UsbError::BufferOverflow);
            }

            let mut i = 0;
            while i + 1 < dtln {
                let w = usbhs.cfifo().read().get_raw();
                buf[i] = (w & 0xFF) as u8;
                buf[i + 1] = ((w >> 8) & 0xFF) as u8;
                i += 2;
            }
            if i < dtln {
                buf[i] = usbhs.cfifol().read().get_raw() as u8;
            }

            let r = usbhs.cfifoctr().read();
            usbhs.cfifoctr().write(r.bclr().set(pac::usbhs::cfifoctr::Bclr::_1));

            Ok(dtln)
        })
    }

    pub fn bus_ep0_set_stall(&mut self, stalled: bool) {
        self._with_cs(|usbhs| unsafe {
            let r = usbhs.dcpctr().read();
            usbhs.dcpctr().write(
                r.pid().set(
                    if stalled {
                        pac::usbhs::dcpctr::Pid::_10
                    } else {
                        pac::usbhs::dcpctr::Pid::_00
                    }
                )
            );
        });
    }

    /// PIPESEL.PIPESEL = pipe
    pub fn bus_select_pipe(&mut self, pipe: u8) {
        self._with_cs(|usbhs| unsafe {
            let r = usbhs.pipesel().read();
            usbhs.pipesel().write(
                r.pipesel().set(pipe.into())
            );

            // 選択反映待ち
            loop {
                if usbhs.pipesel().read().pipesel().get() == pipe.into() {
                    break;
                }
            }
        });
    }

    /// PIPECTR.PID = NAK
    pub fn bus_pipe_nak(&mut self, pipe: u8) {
        self._with_cs(|usbhs| unsafe {
            match pipe {
                1 => {
                    let r = usbhs.pipe1ctr().read();
                    usbhs.pipe1ctr().write(
                        r.pid().set(
                            pac::usbhs::pipectr::Pid::_00
                        )
                    );
                }
                2 => {
                    let r = usbhs.pipe2ctr().read();
                    usbhs.pipe2ctr().write(
                        r.pid().set(
                            pac::usbhs::pipectr::Pid::_00
                        )
                    );
                }
                3 => {
                    let r = usbhs.pipe3ctr().read();
                    usbhs.pipe3ctr().write(
                        r.pid().set(
                            pac::usbhs::pipectr::Pid::_00
                        )
                    );
                }
                _ => {}
            }
        });
    }

    /// PIPECFG / PIPEMAXP 設定
    pub fn bus_pipe_set_ep(
        &mut self,
        pipe: u8,
        ep_num: u8,
        in_dir: bool,
        typ: pac::EnumBitfieldStruct<u8, pac::usbhs::pipecfg::Type_SPEC>,
        mxps: u16,
    ) {
        self.bus_select_pipe(pipe);

        self._with_cs(|usbhs| unsafe {
            let r = usbhs.pipecfg().read();
            usbhs.pipecfg().write(
                r
                .epnum().set(ep_num)
                .dir().set(
                    if in_dir {
                        pac::usbhs::pipecfg::Dir::_1
                    } else {
                        pac::usbhs::pipecfg::Dir::_0
                    }
                )
                .r#type().set(typ)
            );

            // PIPEMAXP
            let r = usbhs.pipemaxp().read();
            usbhs.pipemaxp().write(
                r.mxps().set(mxps)
            );
        });
    }

    /// FIFO clear
    pub fn bus_pipe_clear_fifo(&mut self, pipe: u8) {
        self._with_cs(|usbhs| unsafe {
            // CFIFO select pipe
            let r = usbhs.cfifosel().read();
            usbhs.cfifosel().write(
                r.curpipe().set(pipe.into())
            );

            while usbhs.cfifoctr().read().frdy().get()
                == pac::usbhs::cfifoctr::Frdy::_0 {}

            let r = usbhs.cfifoctr().read();
            usbhs.cfifoctr().write(
                r.bclr().set(
                    pac::usbhs::cfifoctr::Bclr::_1
                )
            );
        });
    }

    /// OUT pipe ready (BUF)
    pub fn bus_pipe_prepare_out(&mut self, pipe: u8) {
        self._with_cs(|usbhs| unsafe {
            match pipe {
                1 => {
                    let r = usbhs.pipe1ctr().read();
                    usbhs.pipe1ctr().write(
                        r.pid().set(
                            pac::usbhs::pipectr::Pid::_01
                        )
                    );
                }
                2 => {
                    let r = usbhs.pipe2ctr().read();
                    usbhs.pipe2ctr().write(
                        r.pid().set(
                            pac::usbhs::pipectr::Pid::_01
                        )
                    );
                }
                3 => {
                    let r = usbhs.pipe3ctr().read();
                    usbhs.pipe3ctr().write(
                        r.pid().set(
                            pac::usbhs::pipectr::Pid::_01
                        )
                    );
                }
                _ => {}
            }
        });
    }

    /// Interrupt / Isochronous endpoint polling interval
    /// interval は USB descriptor の bInterval 相当
    pub fn bus_pipe_set_interval(
        &mut self,
        pipe: u8,
        interval: u8,
    ) {
        // 対象 pipe 選択
        self.bus_select_pipe(pipe);

        // PIPEPERI 設定
        self._with_cs(|usbhs| unsafe {
            let r = usbhs.pipeperi().read();
            usbhs.pipeperi().write(
                r.iitv().set(interval)
            );
        });
    }

    pub fn bus_pipe_config_interrupt_in(
        &mut self,
        pipe: u8,
        ep_num: u8,
        mxps: u16,
        interval: u8,
    ) {

        self.bus_select_pipe(pipe);
        self.bus_pipe_nak(pipe);
        self.bus_pipe_set_ep(pipe, ep_num, true, pac::usbhs::pipecfg::Type::_11, mxps);
        self.bus_pipe_set_interval(pipe, interval);
        self.bus_pipe_clear_fifo(pipe);
    }

    pub fn bus_pipe_config_bulk_out(
        &mut self,
        pipe: u8,
        ep_num: u8,
        mxps: u16,
    ) {
        self.bus_select_pipe(pipe);
        self.bus_pipe_nak(pipe);
        self.bus_pipe_set_ep(pipe, ep_num, false, pac::usbhs::pipecfg::Type::_01, mxps);
        self.bus_pipe_clear_fifo(pipe);
        self.bus_pipe_prepare_out(pipe);
    }

    pub fn bus_pipe_config_bulk_in(
        &mut self,
        pipe: u8,
        ep_num: u8,
        mxps: u16,
    ) {
        self.bus_select_pipe(pipe);
        self.bus_pipe_nak(pipe);
        self.bus_pipe_set_ep(pipe, ep_num, true, pac::usbhs::pipecfg::Type::_01, mxps);
        self.bus_pipe_clear_fifo(pipe);
    }

    pub fn bus_pipe_write_in(&mut self, pipe: u8, data: &[u8]) -> UsbResult<usize> {
        self.bus_select_pipe(pipe);
        if !self.bus_pipe_tx_ready(pipe) {
            return Err(UsbError::WouldBlock);
        }
        let mxps = self.bus_pipe_mxps(pipe) as usize;
        if data.len() > mxps {
            return Err(UsbError::BufferOverflow);
        }

        self.bus_fifo_select(pipe, true);
        self.bus_fifo_clear();
        self.bus_fifo_write(data);
        self.bus_fifo_commit_in(data.len() as u16);
        self.bus_pipe_buf(pipe);

        Ok(data.len())
    }

    pub fn bus_pipe_read_out(&mut self, pipe: u8, buf: &mut [u8]) -> UsbResult<usize> {
        self.bus_select_pipe(pipe);
        if !self.bus_pipe_rx_ready(pipe) {
            return Err(UsbError::WouldBlock);
        }

        self.bus_fifo_select(pipe, false);
        let len = self.bus_fifo_data_len() as usize;

        if len > buf.len() {
            return Err(UsbError::BufferOverflow);
        }

        self.bus_fifo_read(&mut buf[..len]);
        self.bus_fifo_clear();
        self.bus_pipe_prepare_out(pipe);

        Ok(len)
    }

    pub fn bus_pipe_set_stall(&mut self, pipe: u8, _in_dir: bool, stalled: bool) {
        if stalled {
            self.bus_pipe_stall(pipe);
        } else {
            self.bus_pipe_nak(pipe);
            self.bus_pipe_buf(pipe);
        }
    }

    /// PID=BUF
    pub fn bus_pipe_buf(&mut self, pipe: u8) {
        self._with_cs(|usbhs| unsafe {
            match pipe {
                1 => {
                    let r = usbhs.pipe1ctr().read();
                    usbhs.pipe1ctr().write(
                        r.pid().set(
                            pac::usbhs::pipectr::Pid::_01
                        )
                    );
                }
                2 => {
                    let r = usbhs.pipe2ctr().read();
                    usbhs.pipe2ctr().write(
                        r.pid().set(
                            pac::usbhs::pipectr::Pid::_01
                        )
                    );
                }
                3 => {
                    let r = usbhs.pipe3ctr().read();
                    usbhs.pipe3ctr().write(
                        r.pid().set(
                            pac::usbhs::pipectr::Pid::_01
                        )
                    );
                }
                _ => {}
            }
        });
    }

    /// PID=STALL
    pub fn bus_pipe_stall(&mut self, pipe: u8) {
        self._with_cs(|usbhs| unsafe {
            match pipe {
                1 => {
                    let r = usbhs.pipe1ctr().read();
                    usbhs.pipe1ctr().write(
                        r.pid().set(
                            pac::usbhs::pipectr::Pid::_10
                        )
                    );
                }
                2 => {
                    let r = usbhs.pipe2ctr().read();
                    usbhs.pipe2ctr().write(
                        r.pid().set(
                            pac::usbhs::pipectr::Pid::_10
                        )
                    );
                }
                3 => {
                    let r = usbhs.pipe3ctr().read();
                    usbhs.pipe3ctr().write(
                        r.pid().set(
                            pac::usbhs::pipectr::Pid::_10
                        )
                    );
                }
                _ => {}
            }
        });
    }

    /// FIFO選択
    pub fn bus_fifo_select(
        &mut self,
        pipe: u8,
        is_in: bool,
    ) {
        self._with_cs(|usbhs| unsafe {
            let r = usbhs.cfifosel().read();
            usbhs.cfifosel().write(
                r.curpipe().set(pipe.into())
                 .isel().set(
                    if is_in {
                        pac::usbhs::cfifosel::Isel::_1
                    } else {
                        pac::usbhs::cfifosel::Isel::_0
                    }
                 )
                 .mbw().set(
                    pac::usbhs::cfifosel::Mbw::_10
                 )
            );

            loop {
                let r = usbhs.cfifosel().read();
                if r.curpipe().get() == pipe.into() {
                    break;
                }
            }

            while usbhs.cfifoctr().read().frdy().get()
                == pac::usbhs::cfifoctr::Frdy::_0 {}
        });
    }

    /// FIFO clear
    pub fn bus_fifo_clear(&mut self) {
        self._with_cs(|usbhs| unsafe {
            let r = usbhs.cfifoctr().read();
            usbhs.cfifoctr().write(
                r.bclr().set(
                    pac::usbhs::cfifoctr::Bclr::_1
                )
            );
        });
    }

    /// IN転送開始可能
    pub fn bus_pipe_tx_ready(&mut self, pipe: u8) -> bool {
        self._with_cs(|usbhs| unsafe {
            let ready = usbhs.cfifoctr().read().frdy().get()
                == pac::usbhs::cfifoctr::Frdy::_1;

            let busy = match pipe {
                1 => usbhs.pipe1ctr().read().pbusy().get()
                    == pac::usbhs::pipectr::Pbusy::_1,
                2 => usbhs.pipe2ctr().read().pbusy().get()
                    == pac::usbhs::pipectr::Pbusy::_1,
                3 => usbhs.pipe3ctr().read().pbusy().get()
                    == pac::usbhs::pipectr::Pbusy::_1,
                _ => true,
            };

            ready && !busy
        })
    }

    /// OUT受信済み
    pub fn bus_pipe_rx_ready(&mut self, pipe: u8) -> bool {
        self._with_cs(|usbhs| unsafe {
            let sts = usbhs.brdysts().read().get_raw();
            (sts & (1 << pipe)) != 0
        })
    }

    /// 最大packet長取得
    pub fn bus_pipe_mxps(&mut self, pipe: u8) -> u16 {
        self.bus_select_pipe(pipe);
        self._with_cs(|usbhs| unsafe {
            usbhs.pipemaxp().read().mxps().get()
        })
    }

    /// FIFOへ書き込み
    pub fn bus_fifo_write(&mut self, data: &[u8]) {
        self._with_cs(|usbhs| unsafe {
            let mut i = 0;
            while i < data.len() {
                if i + 1 < data.len() {
                    let v =
                        (data[i] as u32) |
                        ((data[i + 1] as u32) << 8);
                    usbhs.cfifo().write_raw(v);
                    i += 2;
                } else {
                    usbhs.cfifol().write_raw(data[i].into());
                    i += 1;
                }
            }
        });
    }

    /// INコミット
    pub fn bus_fifo_commit_in(&mut self, _len: u16) {
        self._with_cs(|usbhs| unsafe {
            let r = usbhs.cfifoctr().read();
            usbhs.cfifoctr().write(
                r.bval().set(
                    pac::usbhs::cfifoctr::Bval::_1
                )
            );
        });
    }

    /// FIFO内データ長
    pub fn bus_fifo_data_len(&mut self) -> u16 {
        self._with_cs(|usbhs| unsafe {
            usbhs.cfifoctr().read().dtln().get()
        })
    }

    /// FIFO読み出し
    pub fn bus_fifo_read(&mut self, buf: &mut [u8]) {
        let len = self.bus_fifo_data_len() as usize;
        self._with_cs(|usbhs| unsafe {
            let n = len.min(buf.len());

            let mut i = 0;
            while i + 1 < n {
                let v = usbhs.cfifo().read().get_raw();
                buf[i] = (v & 0xFF) as u8;
                buf[i + 1] = ((v >> 8) & 0xFF) as u8;
                i += 2;
            }

            if i < n {
                buf[i] = usbhs.cfifol().read().get_raw() as u8;
            }
        });
    }

    pub fn bus_suspend(&mut self) {
        self._with_cs(|usbhs| unsafe {
            let r = usbhs.lpsts().read();
            usbhs.lpsts().write(r.suspendm().set(pac::usbhs::lpsts::Suspendm::_0));
        });
    }

    pub fn bus_resume(&mut self) {
        self._with_cs(|usbhs| unsafe {
            let r = usbhs.lpsts().read();
            usbhs.lpsts().write(r.suspendm().set(pac::usbhs::lpsts::Suspendm::_1));
        });
    }

    pub fn bus_delay_ms(&mut self, ms: u32) {
        for _ in 0..(ms * 1000) {
            cortex_m::asm::nop();
        }
    }
}

impl<S> UsbPeripheral<S> {
    pub fn poll(&mut self) {}
}