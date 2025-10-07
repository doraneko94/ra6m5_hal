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

use crate::pac;

use core::marker::PhantomData;

pub struct Input<F>(PhantomData<F>);
pub struct Output<D>(PhantomData<D>);
pub struct Analog;
pub struct Alternate<const AF: u8>;
pub struct EventInput<E>(PhantomData<E>);

pub struct Floating;
pub struct PullUp;
pub struct PushPull;
pub struct OpenDrain;
pub struct Rising;
pub struct Falling;
pub struct Both;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Edge { DontCare, Rising, Falling, Both }

#[inline(always)]
fn with_pfs<F: FnOnce()>(f: F) {
    unsafe {
        let r = pac::PFS.pwpr().read();
        let w_b0wi_0 = r.b0wi().set(pac::pfs::pwpr::B0Wi::_0);
        pac::PFS.pwpr().write(w_b0wi_0);

        let w_pfswe_1 = r.pfswe().set(pac::pfs::pwpr::Pfswe::_1);
        pac::PFS.pwpr().write(w_pfswe_1);

        f();

        let w_pfswe_0 = r.pfswe().set(pac::pfs::pwpr::Pfswe::_0);
        pac::PFS.pwpr().write(w_pfswe_0);
        let w_b0wi_1 = r.b0wi().set(pac::pfs::pwpr::B0Wi::_1);
        pac::PFS.pwpr().write(w_b0wi_1);
    }
}

#[macro_export]
macro_rules! gpio_pin_input {
    ($port:tt, $group:tt, $id:tt) => {
        paste! {
            pub struct [<P $port $id>]<Mode = Input<Floating>>(PhantomData<Mode>);
            impl<Mode> [<P $port $id>]<Mode> {
                #[inline(always)]
                fn set_latch_high() {
                    unsafe {
                        let w = pac::[<port $port>]::Posr::new(0).[<posr $id>]().set(pac::[<port $port>]::posr::[<Posr $id>]::_1);
                        pac::[<PORT $port>].posr().write(w);
                    }
                }
                #[inline(always)]
                fn set_latch_low() {
                    unsafe {
                        let w = pac::[<port $port>]::Porr::new(0).[<porr $id>]().set(pac::[<port $port>]::porr::[<Porr $id>]::_1);
                        pac::[<PORT $port>].porr().write(w);
                    }
                }
            }

            impl Default for [<P $port $id>]<Input<Floating>> {
                fn default() -> Self {
                    Self(PhantomData)
                }
            }

            impl [<P $port $id>]<Input<Floating>> {
                pub fn into_floating_input(self) -> [<P $port $id>]<Input<Floating>> {
                    with_pfs(|| unsafe {
                        let r = pac::PFS.[<p $port $id pfs>]().read();
                        let w = r.pmr().set(pac::pfs::[<p $group pfs>]::Pmr::_0)
                            .pdr().set(pac::pfs::[<p $group pfs>]::Pdr::_0)
                            .pcr().set(pac::pfs::[<p $group pfs>]::Pcr::_0)
                            .asel().set(pac::pfs::[<p $group pfs>]::Asel::_0)
                            .isel().set(pac::pfs::[<p $group pfs>]::Isel::_0)
                            .ncodr().set(pac::pfs::[<p $group pfs>]::Ncodr::_0);
                        pac::PFS.[<p $port $id pfs>]().write(w);
                    });
                    Self(PhantomData)
                }

                pub fn into_pullup_input(self) -> [<P $port $id>]<Input<PullUp>> {
                    with_pfs(|| unsafe {
                        let r = pac::PFS.[<p $port $id pfs>]().read();
                        let w = r.pmr().set(pac::pfs::[<p $group pfs>]::Pmr::_0)
                            .pdr().set(pac::pfs::[<p $group pfs>]::Pdr::_0)
                            .pcr().set(pac::pfs::[<p $group pfs>]::Pcr::_1)
                            .asel().set(pac::pfs::[<p $group pfs>]::Asel::_0)
                            .isel().set(pac::pfs::[<p $group pfs>]::Isel::_0)
                            .ncodr().set(pac::pfs::[<p $group pfs>]::Ncodr::_0);
                        pac::PFS.[<p $port $id pfs>]().write(w);
                    });
                    [<P $port $id>](PhantomData)
                }
            }

            impl<D> ErrorType for [<P $port $id>]<D> {
                type Error = Infallible;
            }

            impl<D> InputPin for [<P $port $id>]<Input<D>> {
                #[inline]
                fn is_high(&mut self) -> Result<bool, Self::Error> {
                    Ok(self.is_high_level())
                }
                #[inline]
                fn is_low(&mut self) -> Result<bool, Self::Error> {
                    Ok(!self.is_high_level())
                }
            }

            impl<MODE> [<P $port $id>]<MODE> {
                #[inline]
                fn is_high_level(&self) -> bool {
                    unsafe {
                        // Read PCNTR.PIDRnn -> _0:Low / _1:High
                        let r = pac::[<PORT $port>].pidr().read();
                        let v = r.[<pidr $id>]().get();
                        // enum -> bool
                        matches!(v, pac::[<port $port>]::pidr::[<Pidr $id>]::_1)
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! gpio_pin_analog {
    ($port:tt, $group:tt, $id:tt) => {
        paste! {
            impl [<P $port $id>]<Input<Floating>> {
                pub fn into_analog_input(self) -> [<P $port $id>]<Input<Floating>> {
                    with_pfs(|| unsafe {
                        let r = pac::PFS.[<p $port $id pfs>]().read();
                        let w = r.asel().set(pac::pfs::[<p $group pfs>]::Asel::_1);
                        pac::PFS.[<p $port $id pfs>]().write(w);
                    });
                    Self(PhantomData)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! gpio_pin_irq {
    ($port:tt, $group:tt, $id:tt) => {
        paste! {
            impl [<P $port $id>]<Input<Floating>> {
                pub fn into_irq_input(self) -> [<P $port $id>]<Input<Floating>> {
                    with_pfs(|| unsafe {
                        let r = pac::PFS.[<p $port $id pfs>]().read();
                        let w = r.pmr().set(pac::pfs::[<p $group pfs>]::Pmr::_0)
                            .pdr().set(pac::pfs::[<p $group pfs>]::Pdr::_0)
                            .asel().set(pac::pfs::[<p $group pfs>]::Asel::_0)
                            .isel().set(pac::pfs::[<p $group pfs>]::Isel::_1);
                        pac::PFS.[<p $port $id pfs>]().write(w);
                    });
                    Self(PhantomData)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! gpio_pin_irq_edge {
    ($port:tt, $group:tt, $id:tt) => {
        paste! {
            impl [<P $port $id>]<Input<Floating>> {
                pub fn into_irq_input_with_event_edge(self, event: Edge) -> [<P $port $id>]<Input<Floating>> {
                    with_pfs(|| unsafe {
                        let r = pac::PFS.[<p $port $id pfs>]().read();
                        let eofr = match event {
                            Edge::DontCare => pac::pfs::[<p $group pfs>]::Eofr::_00,
                            Edge::Rising => pac::pfs::[<p $group pfs>]::Eofr::_01,
                            Edge::Falling => pac::pfs::[<p $group pfs>]::Eofr::_10,
                            Edge::Both => pac::pfs::[<p $group pfs>]::Eofr::_11,
                        };
                        let w = r.pmr().set(pac::pfs::[<p $group pfs>]::Pmr::_0)
                            .pdr().set(pac::pfs::[<p $group pfs>]::Pdr::_0)
                            .asel().set(pac::pfs::[<p $group pfs>]::Asel::_0)
                            .isel().set(pac::pfs::[<p $group pfs>]::Isel::_1)
                            .eofr().set(eofr);
                        pac::PFS.[<p $port $id pfs>]().write(w);
                    });
                    Self(PhantomData)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! gpio_pin_output {
    ($port:tt, $group:tt, $id:tt) => {
        paste! {
            impl<Mode> [<P $port $id>]<Mode> {
                pub fn into_push_pull_output(self, set_high: bool) -> [<P $port $id>]<Output<PushPull>> {
                    if set_high { Self::set_latch_high(); } else { Self::set_latch_low(); }
                    with_pfs(|| unsafe {
                        let r = pac::PFS.[<p $port $id pfs>]().read();
                        let w = r.pmr().set(pac::pfs::[<p $group pfs>]::Pmr::_0)
                            .asel().set(pac::pfs::[<p $group pfs>]::Asel::_0)
                            .isel().set(pac::pfs::[<p $group pfs>]::Isel::_0)
                            .ncodr().set(pac::pfs::[<p $group pfs>]::Ncodr::_0)
                            .pdr().set(pac::pfs::[<p $group pfs>]::Pdr::_1);
                        pac::PFS.[<p $port $id pfs>]().write(w);
                    });
                    [<P $port $id>](PhantomData)
                }

                pub fn into_open_drain_output_hi(self) -> [<P $port $id>]<Output<OpenDrain>> {
                    Self::set_latch_high();
                    with_pfs(|| unsafe {
                        let r = pac::PFS.[<p $port $id pfs>]().read();
                        let w = r.pmr().set(pac::pfs::[<p $group pfs>]::Pmr::_0)
                            .asel().set(pac::pfs::[<p $group pfs>]::Asel::_0)
                            .isel().set(pac::pfs::[<p $group pfs>]::Isel::_0)
                            .ncodr().set(pac::pfs::[<p $group pfs>]::Ncodr::_0)
                            .pdr().set(pac::pfs::[<p $group pfs>]::Pdr::_1);
                        pac::PFS.[<p $port $id pfs>]().write(w);
                    });
                    [<P $port $id>](PhantomData)
                }
            }

            impl<D> OutputPin for [<P $port $id>]<Output<D>> {
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

            impl<D> StatefulOutputPin for [<P $port $id>]<Output<D>> {
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