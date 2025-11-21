#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

defmt::timestamp!("{=u64}", 0);

#[entry]
fn main() -> ! {
    loop {
        defmt::info!("boot: hello from defmt");
        cortex_m::asm::delay(2_000_000);
    }
}