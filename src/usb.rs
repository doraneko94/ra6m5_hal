use core::cell::RefCell;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, Ordering};
use critical_section::Mutex;

use paste::paste;

use pac::RegisterValue;

use crate::pac;
use crate::{InitError, RegisterError, impl_with_cs, impl_with_cs_with};
use crate::mstp::{Mstp, MSTP_CELL};

pub mod descriptor;
pub mod interrupt;

pub(crate) static USBFS_CELL: Mutex<RefCell<Option<pac::Usbfs>>> = Mutex::new(RefCell::new(None));
pub(crate) static USBHS_CELL: Mutex<RefCell<Option<pac::Usbhs>>> = Mutex::new(RefCell::new(None));
static INITIALIZED_FS: AtomicBool = AtomicBool::new(false);
static INITIALIZED_HS: AtomicBool = AtomicBool::new(false);

// pub(crate) static LAST_SETUP: Mutex<RefCell<Option<UsbSetupPacket>>> = Mutex::new(RefCell::new(None));

pub struct UsbDevice<S> {
    _s: PhantomData<S>,
}

type UsbFs = UsbDevice<pac::Usbfs>;
type UsbHs = UsbDevice<pac::Usbhs>;

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
}

impl<S> UsbDevice<S> {
    pub fn poll(&mut self) {}
}