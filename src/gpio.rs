//! gpio.rs - The smallest GPIO HAL for RA6M5
//! 
//! Premise:
//!  - PCNTR1: pdr/podr (16-bit each)
//!  - PCNTR2: pidr
//!  - PCNTR3: posr/porr (write-1-to-set/clear)
//! 
//! Features:
//!  - Own Port0 and split it into all pins (P0.0..P0.15) with `split()`.
//!  - Type the pin mode with typestate (Input/Output).
//!  - Update High/Low safely and quickly with POSR/PORR (single-bit write-1).
//!  - Read PODR/PIDR to check the status.
//! 
//! How to Use:
//!   let dp = pac::Peripherals::take().unwrap();
//!   let p0 = Port0::new(dp.Port0).split();
//!   let mut led = p0.p0_6.into_output(false);
//!   loop { led.set_high().ok(); delay(...); led.set_low().ok(); delay(...); }

pub mod port0;
pub mod port1;
pub mod port2;
pub mod port3;
pub mod port4;
pub mod port5;
pub mod port6;
pub mod port7;
pub mod port8;
pub mod port9;
pub mod porta;
pub mod portb;

use crate::pac;

use core::marker::PhantomData;

pub const PFS_BASE: u32 = 0x40080800;

pub struct Input<F>(PhantomData<F>);
pub struct Output<D>(PhantomData<D>);
pub struct Analog;
pub struct EventInput<E>(PhantomData<E>);
pub struct Alternate;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Podr { Low = 0, High = 1 }
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Pdr { Input = 0, Output = 1 }
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Pcr { Enable = 0, Disable = 1 }
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Ncodr { PushPull = 0, OpenDrain = 1 }
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Isel { Enable = 0, Disable = 1 }
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Asel { Enable = 0, Disable = 1 }
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Pmr { General = 0, Peripheral = 1 }

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Drive { Low = 0b00, Middle = 0b01, HighHigh = 0b10, High = 0b11 }
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Edge { DontCare = 0b00, Rising = 0b01, Falling = 0b10, Both = 0b11 }

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Peripheral {
    GPIO = 0b00000,
    AGT = 0b00001,
    GPT_0 = 0b00010, 
    GPT_1 = 0b00011,
    SCI_0 = 0b00100, 
    SCI_1 = 0b00101, 
    SPI = 0b00110, 
    IIC = 0b00111, 
    CLKOUT_RTC = 0b01001, 
    CAC_ADC12 = 0b01010, 
    BUS = 0b01011,
    CTSU = 0b01100,
    CANFD = 0b10000, 
    QSPI = 0b10001, 
    SSIE = 0b10010, 
    USBFS = 0b10011, 
    USBHS = 0b10100,
    SDHI = 0b10101, 
    ETHERC_MII = 0b10110, 
    ETHERC_RMII = 0b10111, 
    Trace_Debug = 0b11010, 
    OSPI = 0b11100, 
    CEC = 0b11101, 
    DontCare = 0b11111,
}

pub struct Floating;
pub struct PullUp;
pub struct PushPull;
pub struct OpenDrain;
pub struct Rising;
pub struct Falling;
pub struct Both;

pub const DSCR_SHIFT: u32 = 10;
pub const DSCR_WIDTH: u32 = 2;
pub const DSCR_MASK : u32 = ((1u32 << DSCR_WIDTH) - 1) << DSCR_SHIFT;

pub const EOFR_SHIFT: u32 = 12;
pub const EOFR_WIDTH: u32 = 2;
pub const EOFR_MASK : u32 = ((1u32 << EOFR_WIDTH) - 1) << EOFR_SHIFT;



#[inline(always)]
fn with_pfs<F: FnOnce(pac::Pfs)>(f: F) {
    unsafe {
        let pfs = pac::PFS;
        let r = pfs.pwpr().read();
        let w_b0wi_0 = r.b0wi().set(pac::pfs::pwpr::B0Wi::_0);
        pfs.pwpr().write(w_b0wi_0);

        let w_pfswe_1 = r.pfswe().set(pac::pfs::pwpr::Pfswe::_1);
        pfs.pwpr().write(w_pfswe_1);

        f(pfs);

        let w_pfswe_0 = r.pfswe().set(pac::pfs::pwpr::Pfswe::_0);
        pfs.pwpr().write(w_pfswe_0);
        let w_b0wi_1 = r.b0wi().set(pac::pfs::pwpr::B0Wi::_1);
        pfs.pwpr().write(w_b0wi_1);
    }
}

#[macro_export]
macro_rules! gpio_pin_pfs {
    ($port:tt, $id:tt) => {
        paste! {
            impl<Mode> [<P $port:upper $id>]<Mode> {
                pub fn set_pfs(
                    &self, 
                    podr: Option<Podr>, pdr: Option<Pdr>, pcr: Option<Pcr>, 
                    ncodr: Option<Ncodr>, dscr: Option<Drive>, eofr: Option<Edge>, 
                    isel: Option<Isel>, asel: Option<Asel>, pmr: Option<Pmr>, psel: Option<Peripheral>
                ) {
                    with_pfs(|pfs| unsafe {
                        let mut w = pfs.[<p $port $id pfs>]().read();
                        if let Some(value) = podr { w = w.podr().set((value as u8).into()); }
                        if let Some(value) = pdr { w = w.pdr().set((value as u8).into()); }
                        if let Some(value) = pcr { w = w.pcr().set((value as u8).into()); }
                        if let Some(value) = ncodr { w = w.ncodr().set((value as u8).into()); }
                        if let Some(value) = dscr {
                            let bits = w.get_raw();
                            w = w.set_raw((bits & !DSCR_MASK) | (((value as u32) << DSCR_SHIFT) & DSCR_MASK));
                        }
                        if let Some(value) = eofr {
                            let bits = w.get_raw();
                            w = w.set_raw((bits & !EOFR_MASK) | (((value as u32) << EOFR_SHIFT) & EOFR_MASK));
                        }
                        if let Some(value) = isel { w = w.isel().set((value as u8).into()); }
                        if let Some(value) = asel { w = w.asel().set((value as u8).into()); }
                        if let Some(value) = pmr { w = w.pmr().set((value as u8).into()); }
                        if let Some(value) = psel { w = w.psel().set((value as u8).into()); }
                        pfs.[<p $port $id pfs>]().write(w);
                    })
                }
            }
        }
    };
}

#[macro_export]
macro_rules! gpio_pin_input {
    ($port:tt, $port_group:tt, $id:tt) => {
        paste! {
            pub struct [<P $port:upper $id>]<Mode = Input<Floating>> {
                _mode: PhantomData<Mode>,
                _token: PinToken<$id>
            }
            impl<Mode> [<P $port:upper $id>]<Mode> {
                #[inline(always)]
                fn set_latch_high() {
                    unsafe {
                        let w = pac::[<port $port_group>]::Posr::new(0).[<posr $id>]().set(pac::[<port $port_group>]::posr::[<Posr $id>]::_1);
                        pac::[<PORT $port:upper>].posr().write(w);
                    }
                }
                #[inline(always)]
                fn set_latch_low() {
                    unsafe {
                        let w = pac::[<port $port_group>]::Porr::new(0).[<porr $id>]().set(pac::[<port $port_group>]::porr::[<Porr $id>]::_1);
                        pac::[<PORT $port:upper>].porr().write(w);
                    }
                }
            }

            impl<Mode> [<P $port:upper $id>]<Mode> {
                pub fn into_unused_input(self) -> [<P $port:upper $id>]<Input<Floating>> {
                    self.set_pfs(
                        Some(Podr::Low), Some(Pdr::Input), Some(Pcr::Disable),
                        None, None, None, 
                        Some(Isel::Disable), Some(Asel::Disable), Some(Pmr::General), Some(Peripheral::GPIO)
                    );
                    [<P $port:upper $id>] { _mode: PhantomData, _token: self._token }
                }
            }
            
            impl<Mode> [<P $port:upper $id>]<Mode> {
                pub fn into_floating_input(self) -> [<P $port:upper $id>]<Input<Floating>> {
                    self.set_pfs(
                        None, Some(Pdr::Input), Some(Pcr::Disable), 
                        None, None, None, 
                        Some(Isel::Disable), Some(Asel::Disable), Some(Pmr::General), Some(Peripheral::GPIO)
                    );
                    [<P $port:upper $id>] { _mode: PhantomData, _token: self._token }
                }

                pub fn into_pullup_input(self) -> [<P $port:upper $id>]<Input<PullUp>> {
                    self.set_pfs(
                        None, Some(Pdr::Input), Some(Pcr::Enable), 
                        None, None, None, 
                        Some(Isel::Disable), Some(Asel::Disable), Some(Pmr::General), Some(Peripheral::GPIO)
                    );
                    [<P $port:upper $id>] { _mode: PhantomData, _token: self._token }
                }
            }

            impl<D> ErrorType for [<P $port:upper $id>]<D> {
                type Error = Infallible;
            }

            impl<D> InputPin for [<P $port:upper $id>]<Input<D>> {
                #[inline]
                fn is_high(&mut self) -> Result<bool, Self::Error> {
                    Ok(self.is_high_level())
                }
                #[inline]
                fn is_low(&mut self) -> Result<bool, Self::Error> {
                    Ok(!self.is_high_level())
                }
            }

            impl<MODE> [<P $port:upper $id>]<MODE> {
                #[inline]
                fn is_high_level(&self) -> bool {
                    unsafe {
                        // Read PCNTR.PIDRnn -> _0:Low / _1:High
                        let r = pac::[<PORT $port:upper>].pidr().read();
                        let v = r.[<pidr $id>]().get();
                        // enum -> bool
                        matches!(v, pac::[<port $port_group>]::pidr::[<Pidr $id>]::_1)
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! gpio_pin_analog {
    ($port:tt, $id:tt) => {
        paste! {
            impl<Mode> [<P $port:upper $id>]<Mode> {
                pub fn into_analog(self) -> [<P $port $id>]<Analog> {
                    self.set_pfs(
                        None, Some(Pdr::Input), Some(Pcr::Disable), 
                        None, None, None, 
                        Some(Isel::Disable), Some(Asel::Enable), Some(Pmr::General), Some(Peripheral::GPIO)
                    );
                    [<P $port:upper $id>] { _mode: PhantomData, _token: self._token }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! gpio_pin_irq {
    ($port:tt, $id:tt) => {
        paste! {
            impl<Mode> [<P $port:upper $id>]<Mode> {
                pub fn into_irq_input(self, pullup: Pcr) -> [<P $port:upper $id>]<Input<Floating>> {
                    self.set_pfs(
                        None, Some(Pdr::Input), Some(pullup), 
                        None, None, None, 
                        Some(Isel::Enable), Some(Asel::Disable), Some(Pmr::General), Some(Peripheral::GPIO));
                    [<P $port:upper $id>] { _mode: PhantomData, _token: self._token }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! gpio_pin_irq_edge {
    ($port:tt, $id:tt) => {
        paste! {
            impl<Mode> [<P $port:upper $id>]<Mode> {
                pub fn into_irq_input_with_event_edge(self, event: Edge, pullup: Pcr) -> [<P $port $id>]<Input<Floating>> {
                    self.set_pfs(
                        None, Some(Pdr::Input), Some(pullup), 
                        None, None, Some(event), 
                        Some(Isel::Enable), Some(Asel::Disable), Some(Pmr::General), Some(Peripheral::GPIO));
                    [<P $port:upper $id>] { _mode: PhantomData, _token: self._token }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! gpio_pin_output {
    ($port:tt, $id:tt) => {
        paste! {
            impl<Mode> [<P $port:upper $id>]<Mode> {
                pub fn into_push_pull_output(self, set_high: bool) -> [<P $port:upper $id>]<Output<PushPull>> {
                    if set_high { Self::set_latch_high(); } else { Self::set_latch_low(); }
                    self.set_pfs(
                        None, Some(Pdr::Output), Some(Pcr::Disable), 
                        Some(Ncodr::PushPull), None, None, 
                        Some(Isel::Disable), Some(Asel::Disable), Some(Pmr::General), Some(Peripheral::GPIO)
                    );
                    [<P $port:upper $id>] { _mode: PhantomData, _token: self._token }
                }

                pub fn into_open_drain_output_hi(self) -> [<P $port:upper $id>]<Output<OpenDrain>> {
                    Self::set_latch_high();
                    self.set_pfs(
                        None, Some(Pdr::Output), Some(Pcr::Disable), 
                        Some(Ncodr::OpenDrain), None, None, 
                        Some(Isel::Disable), Some(Asel::Disable), Some(Pmr::General), Some(Peripheral::GPIO)
                    );
                    [<P $port:upper $id>] { _mode: PhantomData, _token: self._token }
                }

                pub fn into_unused_output(self) -> [<P $port:upper $id>]<Output<PushPull>> {
                    self.set_pfs(
                        Some(Podr::Low), Some(Pdr::Output), Some(Pcr::Disable),
                        None, None, None, 
                        Some(Isel::Disable), Some(Asel::Disable), Some(Pmr::General), Some(Peripheral::GPIO)
                    );
                    [<P $port:upper $id>] { _mode: PhantomData, _token: self._token }
                }
            }

            impl<D> OutputPin for [<P $port:upper $id>]<Output<D>> {
                #[inline(always)]
                fn set_high(&mut self) -> Result<(), Self::Error> {
                    Self::set_latch_high();
                    Ok(())
                }
                #[inline(always)]
                fn set_low(&mut self) -> Result<(), Self::Error> {
                    Self::set_latch_low();
                    Ok(())
                }
            }

            impl<D> StatefulOutputPin for [<P $port:upper $id>]<Output<D>> {
                #[inline(always)]
                fn is_set_high(&mut self) -> Result<bool, Self::Error> {
                    Ok(self.is_high_level())
                }
                #[inline(always)]
                fn is_set_low(&mut self) -> Result<bool, Self::Error> {
                    Ok(!self.is_high_level())
                }
                #[inline(always)]
                fn toggle(&mut self) -> Result<(), Self::Error> {
                    if self.is_high_level() {
                        self.set_low()
                    } else {
                        self.set_high()
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! gpio_pin_drive {
    ($port:tt, $id:tt) => {
        paste! {
            impl<Mode> [<P $port:upper $id>]<Mode> {
                /// In some pins, HighHigh is not available.
                pub fn set_drive_ability(self, ability: Drive) -> [<P $port:upper $id>]<Mode> {
                    self.set_pfs(
                        None, None, None,
                        None, Some(ability), None,
                        None, None, None, None
                    );
                    [<P $port:upper $id>] { _mode: PhantomData, _token: self._token }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! gpio_pin_alternate {
    ($port:tt, $id:tt) => {
        paste! {
            impl<Mode> [<P $port:upper $id>]<Mode> {
                pub fn into_alternate(self, peripheral: Peripheral, drive_ability: Option<Drive>) -> [<P $port:upper $id>]<Alternate> {
                    let (pdr, ncodr, asel, pmr) = match peripheral {
                        Peripheral::GPIO => (
                            None, None, Some(Asel::Disable), Some(Pmr::General)
                        ),
                        Peripheral::IIC | Peripheral::CEC => (
                            None, Some(Ncodr::OpenDrain), Some(Asel::Disable), Some(Pmr::General)
                        ),
                        Peripheral::CAC_ADC12 | Peripheral::CTSU => (
                            None, Some(Ncodr::PushPull), Some(Asel::Enable), Some(Pmr::Peripheral)
                        ),
                        Peripheral::DontCare => (
                            Some(Pdr::Input), None, Some(Asel::Disable), Some(Pmr::General)
                        ),
                        _ => (
                            None, Some(Ncodr::PushPull), Some(Asel::Disable), Some(Pmr::Peripheral)
                        )
                    };
                    self.set_pfs(
                        None, pdr, None,
                        ncodr, drive_ability, None,
                        None, asel, pmr, Some(peripheral)
                    );
                    [<P $port:upper $id>] { _mode: PhantomData, _token: self._token }
                }
            }
        }
    };
}