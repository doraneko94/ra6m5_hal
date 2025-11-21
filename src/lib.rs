#![no_std]

pub mod dbg;
pub mod delay;
pub mod fcache;
pub mod flad;
pub mod gpio;
pub mod icu;
pub mod mstp;
pub mod register_protection;
pub mod rtc;
pub mod sysc;
pub mod usb;

pub use ra6m5_pac as pac;

#[derive(Debug)]
pub struct AlreadyTaken;
#[derive(Debug)]
pub enum InitError { AlreadyInit }
#[derive(Debug)]
pub enum RegisterError {
    NotReadyToWrite(&'static str),
    InvalidValue(&'static str),
    ProhibitedOperation(&'static str)
}

#[macro_export]
macro_rules! impl_with_cs {
    ($name:tt, $peri:tt) => {
        paste! {
            impl $name {
                fn _with_cs<R>(&mut self, f: impl FnOnce(&mut pac::$peri) -> R) -> R {
                    critical_section::with(|cs| {
                        let mut bor = [<$peri:upper _CELL>].borrow(cs).borrow_mut();
                        let [<$peri:lower>] = bor.as_mut().expect(concat!(stringify!([<$peri:upper>]), " is not initialized"));

                        f([<$peri:lower>])
                    })
                }
            }
        }
    };
}
#[macro_export]
macro_rules! impl_with_cs_with {
    ($name:tt, $peri:tt, $with:tt) => {
        paste! {
            impl $name {
                fn [<_with_cs_ $with:lower>]<R>(&mut self, f: impl FnOnce(&mut pac::$peri, &mut pac::$with) -> R) -> R {
                    critical_section::with(|cs| {
                        let mut bor = [<$peri:upper _CELL>].borrow(cs).borrow_mut();
                        let [<$peri:lower>] = bor.as_mut().expect(concat!(stringify!([<$peri:upper>]), " is not initialized"));
                        let mut [<bor_ $with:lower>] = [<$with:upper _CELL>].borrow(cs).borrow_mut();
                        let [<$with:lower>] = [<bor_ $with:lower>].as_mut().expect(concat!(stringify!([<$with:upper>]), " is not initialized"));

                        f([<$peri:lower>], [<$with:lower>])
                    })
                }
            }
        }
    };
}

/*pub struct Hal {
    pub pac: pac::Peripherals,
}

impl Hal {
    pub fn take() -> Option<Self> {
        pac::Peripherals::take().map(|p| Self { pac: p })
    }
}*/

