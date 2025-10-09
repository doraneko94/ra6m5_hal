#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

use ra6m5_pac as pac;
use ra6m5_hal::{gpio, delay};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let p0 = gpio::port0::Port0::new(dp.PORT0);
    let ports = p0.split();

    let mut led = ports.p006.into_push_pull_output(false);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut d = delay::Delay::with_iclk(cp.SYST, 2_4000_000);

    loop {
        let _ = led.set_high();
        d.delay_ms(200);
        let _ = led.set_low();
        d.delay_ms(200);
    }
}