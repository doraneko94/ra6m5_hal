use cortex_m::peripheral::{SYST, syst::SystClkSource};
use embedded_hal::delay::DelayNs;

pub struct Delay {
    syst: SYST,
    frequency: u32,
}

impl Delay {
    #[inline]
    pub fn with_iclk(syst: SYST, cpu_hz: u32) -> Self {
        Self::with_source(syst, SystClkSource::Core, cpu_hz)
    }
    #[inline]
    pub fn with_systicclk(syst: SYST) -> Self {
        Self::with_source(syst, SystClkSource::External, 32_768)
    }
    #[inline]
    fn with_source(mut syst: SYST, source: SystClkSource, frequency: u32) -> Self {
        syst.set_clock_source(source);
        syst.disable_counter();
        syst.disable_interrupt();
        Self { syst, frequency }
    }

    /// Releases the system timer (SysTick) resource.
    #[inline]
    pub fn free(self) -> SYST {
        self.syst
    }
}

impl DelayNs for Delay {
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        let ticks = ((u128::from(ns) * u128::from(self.frequency) + 999_999_999) / 1_000_000_000) as u64;

        let full_cycles = ticks >> 24;
        if full_cycles > 0 {
            self.syst.set_reload(0x00FF_FFFF);
            self.syst.clear_current();
            self.syst.enable_counter();

            for _ in 0..full_cycles {
                while !self.syst.has_wrapped() {}
            }
        }

        let ticks = (ticks & 0x00FF_FFFF) as u32;
        if ticks > 1 {
            self.syst.set_reload(ticks - 1);
            self.syst.clear_current();
            self.syst.enable_counter();

            while !self.syst.has_wrapped() {}
        }

        self.syst.disable_counter();
    }

    #[inline]
    fn delay_us(&mut self, mut us: u32) {
        while us > 4294967 {
            self.delay_ns(4294967000u32);
            us -= 4294967;
        }
        self.delay_ns(us.saturating_mul(1000));
    }

    #[inline]
    fn delay_ms(&mut self, mut ms: u32) {
        while ms > 4294967 {
            self.delay_us(4294967000u32);
            ms -= 4294967;
        }
        self.delay_us(ms.saturating_mul(1000));
    }
}