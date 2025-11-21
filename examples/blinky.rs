#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception};
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayNs;
//use panic_halt as _;

use defmt_rtt as _;
use panic_probe as _;

defmt::timestamp!("{=u64}", 0);

#[exception]
unsafe fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    // 可能な限り早く最低限の情報を出す
    let scb = unsafe { &*cortex_m::peripheral::SCB::PTR };
    defmt::error!(
        "HardFault: HFSR={=u32:x} CFSR={=u32:x} BFAR={=u32:x} MMFAR={=u32:x}",
        scb.hfsr.read(), scb.cfsr.read(), scb.bfar.read(), scb.mmfar.read()
    );
    defmt::error!("PC/LR: {=u32:x} / {=u32:x}", ef.pc(), ef.lr());

    // bkpt 連打はツールが「Exception 終了」に見えるので、WFI で停止ループに
    loop { cortex_m::asm::wfi(); }
}

use ra6m5_pac as pac;
use ra6m5_hal::{delay, fcache, gpio, mstp, sysc};
use ra6m5_hal::sysc::clock::{ClocksConfig, ClocksDiv, MoscConfig, MoscSource, PliDiv, PllConfig, PllMul, EK_RA6M5_XTAL_HZ};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let p0 = gpio::port0::Port0::take(dp.PORT0).unwrap();
    let ports = p0.split();
    let mut led = ports.p006.into_push_pull_output(false);

    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut clocks = sysc::Sysc::init(pac::SYSC).unwrap();
    
    let mut mstp = mstp::Mstp::init(pac::MSTP).unwrap();
    let mut fcache = fcache::Fcache::init(pac::FCACHE).unwrap();

    if clocks.mosc.is_running() { clocks.mosc.stop().unwrap(); }
    let mosc_config = MoscConfig::new(EK_RA6M5_XTAL_HZ, MoscSource::Oscillator).unwrap();
    clocks.mosc.set_config(&mosc_config).unwrap();
    clocks.mosc.run().unwrap();
    while !clocks.mosc.is_stable() {}

    if clocks.pll.is_running() { clocks.pll.stop().unwrap(); }
    let pll_config = PllConfig::new(&mut clocks.mosc, PliDiv::Div3, PllMul::Mul25_0).unwrap();
    clocks.pll.set_config(&pll_config).unwrap();
    clocks.pll.run().unwrap();
    while !clocks.pll.is_stable() {}

    let config = ClocksConfig::new(
        &mut clocks.pll, ClocksDiv::Div1, ClocksDiv::Div2, ClocksDiv::Div4, ClocksDiv::Div4, ClocksDiv::Div2, 
        ClocksDiv::Div4, ClocksDiv::Div2, true, false, false
    ).unwrap();

    clocks.set_config(&config, &mut mstp, &mut fcache, &mut cp.DCB, &mut cp.DWT).unwrap();

    let mut d = delay::Delay::new(cp.SYST, &clocks);
    defmt::info!("{=u32}", clocks.get_freqs().ick);
    
    loop {
        let _ = led.set_high();
        d.delay_ms(500);
        let _ = led.set_low();
        d.delay_ms(500);
    }
}