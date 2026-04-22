#![allow(clippy::match_same_arms)]

use core::cell::RefCell;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, Ordering};

use critical_section::Mutex;

use usb_device::{Result as UsbResult, UsbDirection, UsbError};
use usb_device::bus::{PollResult, UsbBus};
use usb_device::endpoint::{EndpointAddress, EndpointType};

use super::UsbHs;
use crate::mstp::Mstp;
use crate::usb::USBHS_CELL;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EpAlloc {
    addr: EndpointAddress,
    ep_type: EndpointType,
    max_packet_size: u16,
    interval: u8,
    pipe: u8,
    used: bool,
    stalled: bool,
}

impl EpAlloc {
    fn empty() -> Self {
        Self {
            addr: EndpointAddress::from_parts(0, UsbDirection::Out),
            ep_type: EndpointType::Bulk,
            max_packet_size: 0,
            interval: 0,
            pipe: 0,
            used: false,
            stalled: false,
        }
    }
}

#[derive(Debug)]
struct BusState {
    enabled: bool,
    suspended: bool,
    ep0_allocated: bool,
    eps: [EpAlloc; 4],
}

impl BusState {
    fn new() -> Self {
        Self {
            enabled: false,
            suspended: false,
            ep0_allocated: false,
            eps: [
                EpAlloc::empty(),
                EpAlloc::empty(),
                EpAlloc::empty(),
                EpAlloc::empty(),
            ]
        }
    }

    fn find_alloc(&self, ep_addr: EndpointAddress) -> Option<&EpAlloc> {
        self.eps.iter().find(|e| e.used && e.addr == ep_addr)
    }

    fn find_alloc_mut(&mut self, ep_addr: EndpointAddress) -> Option<&mut EpAlloc> {
        self.eps.iter_mut().find(|e| e.used && e.addr == ep_addr)
    }

    fn alloc_control(&mut self, dir: UsbDirection, max_packet_size: u16) -> UsbResult<EndpointAddress> {
        if self.ep0_allocated {
            return Err(UsbError::InvalidEndpoint);
        }
        if max_packet_size > 64 {
            return Err(UsbError::Unsupported);
        }

        let addr = EndpointAddress::from_parts(0, dir);
        self.eps[0] = EpAlloc {
            addr, 
            ep_type: EndpointType::Control, 
            max_packet_size, 
            interval: 0, 
            pipe: 0, 
            used: true, 
            stalled: false, 
        };
        self.ep0_allocated = true;
        Ok(addr)
    }

    fn alloc_fixed(
        &mut self,
        dir: UsbDirection,
        ep_type: EndpointType,
        requested: Option<EndpointAddress>,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<EndpointAddress> {
        let candidate = match (dir, ep_type) {
            (UsbDirection::In, EndpointType::Interrupt) => {
                EndpointAddress::from_parts(1, UsbDirection::In)
            }
            (UsbDirection::Out, EndpointType::Bulk) => {
                EndpointAddress::from_parts(2, UsbDirection::Out)
            }
            (UsbDirection::In, EndpointType::Bulk) => {
                EndpointAddress::from_parts(3, UsbDirection::In)
            }
            _ => return Err(UsbError::Unsupported),
        };

        if let Some(req) = requested {
            if req != candidate {
                return Err(UsbError::InvalidEndpoint);
            }
        }

        let slot = match candidate.index() {
            1 => 1,
            2 => 2,
            3 => 3,
            _ => return Err(UsbError::InvalidEndpoint),
        };

        if self.eps[slot].used {
            return Err(UsbError::InvalidEndpoint);
        }

        let pipe = match candidate.index() {
            1 => 1,
            2 => 2,
            3 => 3,
            _ => unreachable!()
        };

        self.eps[slot] = EpAlloc { 
            addr: candidate, 
            ep_type, 
            max_packet_size, 
            interval, 
            pipe, 
            used: true, 
            stalled: true
        };

        Ok(candidate)
    }
}

pub struct UsbHsBus {
    xtal_hz: u32,
    iclk_hz: u32,
    state: Mutex<RefCell<BusState>>,
    started: AtomicBool,
}

impl UsbHsBus {
    pub fn new(xtal_hz: u32, iclk_hz: u32) -> Self {
        Self {
            xtal_hz,
            iclk_hz,
            state: Mutex::new(RefCell::new(BusState::new())),
            started: AtomicBool::new(false),
        }
    }

    fn with_usb<R>(&self, f: impl FnOnce(&mut UsbHs) -> R) -> R {
        critical_section::with(|cs| {
            let mut usb = USBHS_CELL.borrow(cs).borrow_mut();
            f(usb.as_mut().unwrap())
        })
    }
    /*fn with_usb<R>(&self, f: impl FnOnce(&mut pac::Usbhs) -> R) -> R {
        critical_section::with(|cs| {
            let mut bor_usbhs = USBHS_CELL.borrow(cs).borrow_mut();
            let usbhs = bor_usbhs.as_mut().expect("USBHS is not initialized");

            f(usbhs)
        })
    }*/

    fn with_state<R>(&self, f: impl FnOnce(&mut BusState) -> R) -> R {
        critical_section::with(|cs| {
            let mut st = self.state.borrow(cs).borrow_mut();
            f(&mut st)
        })
    }

    fn ep_max_packet_size(&self, ep_addr: EndpointAddress) -> Option<u16> {
        critical_section::with(|cs| {
            self.state
                .borrow(cs)
                .borrow()
                .find_alloc(ep_addr)
                .map(|e| e.max_packet_size)
        })
    }

    fn ep_pipe(&self, ep_addr: EndpointAddress) -> Option<u8> {
        critical_section::with(|cs| {
            self.state
                .borrow(cs)
                .borrow()
                .find_alloc(ep_addr)
                .map(|e| e.pipe)
        })
    }

    fn init_pipes_after_reset(&self) {
        self.with_usb(|usb| {
            usb.bus_ep0_reset(64);
            usb.bus_pipe_config_interrupt_in(1, 1, 64, 16); // EP1 IN interrupt
            usb.bus_pipe_config_bulk_out(2, 2, 512);
            usb.bus_pipe_config_bulk_in(3, 3, 512);
        });
    }
}

impl UsbBus for UsbHsBus {
    const QUIRK_SET_ADDRESS_BEFORE_STATUS: bool = false;

    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<EndpointAddress> {
        self.with_state(|st| {
            if st.enabled {
                return Err(UsbError::InvalidState);
            }

            if ep_type == EndpointType::Control {
                let addr = ep_addr.unwrap_or_else(|| EndpointAddress::from_parts(0, ep_dir));
                if addr.index() != 0 {
                    return Err(UsbError::InvalidEndpoint);
                }
                return st.alloc_control(ep_dir, max_packet_size);
            }

            st.alloc_fixed(ep_dir, ep_type, ep_addr, max_packet_size, interval)
        })
    }

    fn enable(&mut self) {
        if self.started.swap(true, Ordering::AcqRel) {
            return;
        }

        self.with_usb(|usb| {
            usb._with_cs_mstp(|_usb, mstp| {
                let _ = usb.start(mstp, 0, 0);

            });
            //let mut mstp = Mstp { _s: PhantomData };
            //let _ = usb.start(&mut mstp, self.xtal_hz, self.iclk_hz);
            //let _ = usb.bus_enable_interrupts();
            //let _ = usb.connect(&mut mstp);
        });

        self.with_state(|st| {
            st.enabled = true;
            st.suspended = false;
        });
    }

    fn reset(&self) {
        self.with_usb(|usb| {
            usb.bus_clear_bus_events();
        });

        self.with_state(|st| {
            st.suspended = false;
            for ep in &mut st.eps {
                if ep.used {
                    ep.stalled = false;
                }
            }
        });

        self.init_pipes_after_reset();
    }

    fn set_device_address(&self, addr: u8) {
        self.with_usb(|usb| {
            usb.bus_set_address(addr);
        });
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        let Some(max_packet_size) = self.ep_max_packet_size(ep_addr) else {
            return Err(UsbError::InvalidEndpoint);
        };

        if buf.len() > max_packet_size as usize {
            return Err(UsbError::BufferOverflow);
        }

        if ep_addr.index() == 0 {
            self.with_usb(|usb| usb.bus_ep0_write_in(buf))?;
            return Ok(buf.len());
        }

        let Some(pipe) = self.ep_pipe(ep_addr) else {
            return Err(UsbError::InvalidEndpoint);
        };

        self.with_usb(|usb| usb.bus_pipe_write_in(pipe, buf))?;
        Ok(buf.len())
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        let Some(max_packet_size) = self.ep_max_packet_size(ep_addr) else {
            return Err(UsbError::InvalidEndpoint);
        };

        if buf.len() < max_packet_size as usize {
            return Err(UsbError::BufferOverflow);
        }

        if ep_addr.index() == 0 {
            return self.with_usb(|usb| usb.bus_ep0_read_out(buf));
        }

        let Some(pipe) = self.ep_pipe(ep_addr) else {
            return Err(UsbError::InvalidEndpoint);
        };

        self.with_usb(|usb| usb.bus_pipe_read_out(pipe, buf))
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        self.with_state(|st| {
            if let Some(ep) = st.find_alloc_mut(ep_addr) {
                ep.stalled = stalled;
            }
        });

        self.with_usb(|usb| {
            if ep_addr.index() == 0 {
                usb.bus_ep0_set_stall(stalled);
            } else if let Some(pipe) = self.ep_pipe(ep_addr) {
                usb.bus_pipe_set_stall(pipe, ep_addr.is_in(), stalled);
            }
        });
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        critical_section::with(|cs| {
            self.state
                .borrow(cs)
                .borrow()
                .find_alloc(ep_addr)
                .map(|e| e.stalled)
                .unwrap_or(false)
        })
    }

    fn suspend(&self) {
        self.with_usb(|usb| usb.bus_suspend());
        self.with_state(|st| st.suspended = true);
    }

    fn resume(&self) {
        self.with_usb(|usb| usb.bus_resume());
        self.with_state(|st| st.suspended = false);
    }

    fn poll(&self) -> PollResult {
        let ev = self.with_usb(|usb| usb.bus_poll());

        if ev.reset {
            return PollResult::Reset;
        }

        if ev.resume {
            return PollResult::Resume;
        }

        if ev.suspend {
            return PollResult::Suspend;
        }

        if ev.ep_out != 0 || ev.ep_in_complete != 0 || ev.ep_setup != 0 {
            return PollResult::Data {
                ep_out: ev.ep_out,
                ep_in_complete: ev.ep_in_complete,
                ep_setup: ev.ep_setup,
            };
        }

        PollResult::None
    }

    fn force_reset(&self) -> UsbResult<()> {
        self.with_usb(|usb| {
            //let mut mstp = Mstp { _s: PhantomData };
            //usb.disconnect(&mut mstp).map_err(|_| UsbError::InvalidState)?;
            //usb.bus_delay_ms(20);
            //usb.connect(&mut mstp).map_err(|_| UsbError::InvalidState)?;
            Ok(())
        })
    }
}