#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_probe as _;
use defmt_rtt as _;

use cortex_m_rt::entry;

use ra6m5_hal as hal;
use hal::pac;
use hal::sysc;
use hal::mstp;
use hal::fcache;
use hal::delay;
use hal::usb::UsbPeripheral;

use hal::sysc::clock::{
    ClocksConfig,
    ClocksDiv,
    EK_RA6M5_XTAL_HZ,
    MoscConfig,
    MoscSource,
    PliDiv,
    PllConfig,
    PllMul,
};

use core::sync::atomic::{AtomicU32, Ordering};
static COUNT: AtomicU32 = AtomicU32::new(0);
defmt::timestamp!("{=u32}", COUNT.fetch_add(1, Ordering::Relaxed));

#[entry]
fn main() -> ! {
    defmt::info!("usb_hs_step0: start");

    let dp = pac::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();

    let mut clocks = sysc::Sysc::init(dp.SYSC).unwrap();
    let mut mstp = mstp::Mstp::init(dp.MSTP).unwrap();
    let mut fcache = fcache::Fcache::init(dp.FCACHE).unwrap();

    if clocks.mosc.is_running() {
        clocks.mosc.stop().unwrap();
    }
    let mosc_config = MoscConfig::new(EK_RA6M5_XTAL_HZ, MoscSource::Oscillator).unwrap();
    clocks.mosc.set_config(&mosc_config).unwrap();
    clocks.mosc.run().unwrap();
    while !clocks.mosc.is_stable() {}

    if clocks.pll.is_running() {
        clocks.pll.stop().unwrap();
    }
    let pll_config = PllConfig::new(&mut clocks.mosc, PliDiv::Div3, PllMul::Mul25_0).unwrap();
    clocks.pll.set_config(&pll_config).unwrap();
    clocks.pll.run().unwrap();
    while !clocks.pll.is_stable() {}

    let clocks_config = ClocksConfig::new(
        &mut clocks.pll,
        ClocksDiv::Div1, // ICLK  = 200 / 1 = 200 MHz
        ClocksDiv::Div2, // PCLKA = 200 / 2 = 100 MHz
        ClocksDiv::Div4, // PCLKB = 200 / 4 =  50 MHz
        ClocksDiv::Div4, // PCLKC = 200 / 4 =  50 MHz
        ClocksDiv::Div2, // PCLKD = 200 / 2 = 100 MHz
        ClocksDiv::Div4, // FCLK  = 200 / 4 =  50 MHz
        ClocksDiv::Div2, // BCLK  = 200 / 2 = 100 MHz
        true,
        false,
        false,
    ).unwrap();

    clocks.set_config(
        &clocks_config,
        &mut mstp,
        &mut fcache,
        &mut cp.DCB,
        &mut cp.DWT,
    ).unwrap();

    let mut d = delay::Delay::new(cp.SYST, &clocks);

    defmt::info!("mosc stable = {}", clocks.mosc.is_stable());
    defmt::info!("pll stable = {}", clocks.pll.is_stable());

    let mut usb = UsbPeripheral::<pac::Usbhs>::init(dp.USBHS).unwrap();

    defmt::info!("usbhs enabled(before) = {}", usb.is_enabled(&mut mstp));

    match usb.start(&mut mstp, EK_RA6M5_XTAL_HZ, clocks.get_freqs().ick) {
        Ok(()) => {
            defmt::info!("usbhs enable ok");
            defmt::info!("usbhs pll lock(after enable) = {}", usb.is_pll_locked());
            defmt::info!("usbhs enabled(after) = {}", usb.is_enabled(&mut mstp));

            match usb.connect(&mut mstp) {
                Ok(()) => {
                    defmt::info!("usbhs connect ok");
                    defmt::info!("usbhs connected = {}", usb.is_connected(&mut mstp));
                }
                Err(e) => {
                    defmt::info!("usbhs connect error = {}", defmt::Debug2Format(&e));
                }
            }
        }
        Err(e) => {
            defmt::error!("usbhs enable err: {}", defmt::Debug2Format(&e));
        }
    }

    loop {
        defmt::info!("usb_hs_step0: alive");
        d.delay_ms(1000);
    }
}