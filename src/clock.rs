#![allow(dead_code)]

use crate::pac;

pub const EK_RA6M5_XTAL_HZ: u32 = 24_000_000;

pub const MAIN_HZ_MIN: u32 =      8_000_000;
pub const MAIN_HZ_MAX: u32 =     24_000_000;
pub const PLL_HZ_MIN: u32 =     120_000_000;
pub const PLL_HZ_MAX: u32 =     240_000_000;

pub const MOCO_TYP_HZ: u32 =    8_000_000;
pub const LOCO_TYP_HZ: u32 =       32_768;

pub const HIGH_SPEED_ICLK_MAX: u32 =          200_000_000;
pub const HIGH_SPEED_PCLKA_MAX: u32 =         100_000_000;
pub const HIGH_SPEED_PCLKB_MAX: u32 =          50_000_000;
pub const HIGH_SPEED_PCLKC_MAX: u32 =          50_000_000;
pub const HIGH_SPEED_PCLKD_MAX: u32 =         100_000_000;
pub const HIGH_SPEED_FCLK_MAX: u32 =           50_000_000;
pub const HIGH_SPEED_BCLK_MAX: u32 =          100_000_000;
pub const HIGH_SPEED_EBCLK_MAX: u32 =          50_000_000;
pub const HIGH_SPEED_PCLKC_ADC12_MIN: u32 =     1_000_000;

pub const LOW_SPEED_MAX: u32 =   1_000_000;

pub const SUBOSC_SPEED_MAX: u32 =               36_100;
pub const SUBOSC_SPEED_ICLK_FCLK_MIN: u32 =     29_400;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    HighSpeed, LowSpeed, SuboscSpeed
}

/// System Clock Source
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Source {
    Hoco { hoco_hz: HocoHz, fll: Option<HocoFll> },
    Moco,
    Loco,
    Mosc { ext_main_hz: u32 },
    Sosc,
    Pll  { ext_main_hz: u32, plidiv: PliDiv, pllmul: u8 },
}
impl Source {
    #[inline] pub fn bits(self) -> u8 {
        match self {
            Self::Hoco { .. }=>0b000, Self::Moco=>0b001, Self::Loco=>0b010,
            Self::Mosc { .. }=>0b011, Self::Sosc=>0b100, Self::Pll { .. }=>0b101
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClockSecurity { Secure, NonSecure }
impl ClockSecurity {
    #[inline] pub fn bits(self) -> u8 { match self { Self::Secure=>0, Self::NonSecure=>1 } }
}

/// SCKDIVCR
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Div2Pow { Div1, Div2, Div4, Div8, Div16, Div32, Div64 }
impl Div2Pow {
    #[inline] pub fn bits(self) -> u8 {
        match self {
            Self::Div1=>0b000, Self::Div2=>0b001, Self::Div4=>0b010, Self::Div8=>0b011,
            Self::Div16=>0b100, Self::Div32=>0b101, Self::Div64=>0b110,
        }
    }
    #[inline] pub fn val(self) -> u32 { 1u32 << self.bits() }
}

/// PLL
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PliDiv { Div1, Div2, Div3 }
impl PliDiv {
    #[inline] fn to_bits(self) -> u8 { match self { Self::Div1=>0, Self::Div2=>1, Self::Div3=>2 } }
    #[inline] fn val(self) -> u32 { match self { Self::Div1=>1, Self::Div2=>2, Self::Div3=>3 } }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PllMul {
    Mul10_0, Mul10_5, Mul11_0, Mul11_5, Mul12_0, Mul12_5, Mul13_0, Mul13_5, 
    Mul14_0, Mul14_5, Mul15_0, Mul15_5, Mul16_0, Mul16_5, Mul17_0, Mul17_5, 
    Mul18_0, Mul18_5, Mul19_0, Mul19_5, Mul20_0, Mul20_5, Mul21_0, Mul21_5, 
    Mul22_0, Mul22_5, Mul23_0, Mul23_5, Mul24_0, Mul24_5, Mul25_0, Mul25_5, 
    Mul26_0, Mul26_5, Mul27_0, Mul27_5, Mul28_0, Mul28_5, Mul29_0, Mul29_5, Mul30_0
}
impl PllMul {
    #[inline] fn to_bits(self) -> u8 {
        match self {
            Self::Mul10_0=>0x13, Self::Mul10_5=>0x14, Self::Mul11_0=>0x15, Self::Mul11_5=>0x16, Self::Mul12_0=>0x17, Self::Mul12_5=>0x18, Self::Mul13_0=>0x19, Self::Mul13_5=>0x1a, 
            Self::Mul14_0=>0x1b, Self::Mul14_5=>0x1c, Self::Mul15_0=>0x1d, Self::Mul15_5=>0x1e, Self::Mul16_0=>0x1f, Self::Mul16_5=>0x20, Self::Mul17_0=>0x21, Self::Mul17_5=>0x22, 
            Self::Mul18_0=>0x23, Self::Mul18_5=>0x24, Self::Mul19_0=>0x25, Self::Mul19_5=>0x26, Self::Mul20_0=>0x27, Self::Mul20_5=>0x28, Self::Mul21_0=>0x29, Self::Mul21_5=>0x2a, 
            Self::Mul22_0=>0x2b, Self::Mul22_5=>0x2c, Self::Mul23_0=>0x2d, Self::Mul23_5=>0x2e, Self::Mul24_0=>0x2f, Self::Mul24_5=>0x30, Self::Mul25_0=>0x31, Self::Mul25_5=>0x32, 
            Self::Mul26_0=>0x33, Self::Mul26_5=>0x34, Self::Mul27_0=>0x35, Self::Mul27_5=>0x36, Self::Mul28_0=>0x37, Self::Mul28_5=>0x38, Self::Mul29_0=>0x39, Self::Mul29_5=>0x3a, Self::Mul30_0=>0x3b
    } }
    #[inline] fn val(self) -> f32 {
        match self {
            Self::Mul10_0=>10.0, Self::Mul10_5=>10.5, Self::Mul11_0=>11.0, Self::Mul11_5=>11.5, Self::Mul12_0=>12.0, Self::Mul12_5=>12.5, Self::Mul13_0=>13.0, Self::Mul13_5=>13.5, 
            Self::Mul14_0=>14.0, Self::Mul14_5=>14.5, Self::Mul15_0=>15.0, Self::Mul15_5=>15.5, Self::Mul16_0=>16.0, Self::Mul16_5=>16.5, Self::Mul17_0=>17.0, Self::Mul17_5=>17.5, 
            Self::Mul18_0=>18.0, Self::Mul18_5=>18.5, Self::Mul19_0=>19.0, Self::Mul19_5=>19.5, Self::Mul20_0=>20.0, Self::Mul20_5=>20.5, Self::Mul21_0=>21.0, Self::Mul21_5=>21.5, 
            Self::Mul22_0=>22.0, Self::Mul22_5=>22.5, Self::Mul23_0=>23.0, Self::Mul23_5=>23.5, Self::Mul24_0=>24.0, Self::Mul24_5=>24.5, Self::Mul25_0=>25.0, Self::Mul25_5=>25.5, 
            Self::Mul26_0=>26.0, Self::Mul26_5=>26.5, Self::Mul27_0=>27.0, Self::Mul27_5=>27.5, Self::Mul28_0=>28.0, Self::Mul28_5=>28.5, Self::Mul29_0=>29.0, Self::Mul29_5=>29.5, Self::Mul30_0=>30.0
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BClkDiv { BClk, BClkDiv2 }
impl BClkDiv {
    #[inline] pub fn bits(self) -> u8 { match self { Self::BClk=>0, Self::BClkDiv2=>1 } }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ControlMode { Active, Stop }
impl ControlMode {
    #[inline] pub fn bits(self) -> u8 { match self { Self::Active=>0, Self::Stop=>1 } }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FllMode { Disabled, Enabled }
impl FllMode {
    #[inline] pub fn bits(self) -> u8 { match self { Self::Disabled=>0, Self::Enabled=>1 } }
}

/// HOCO
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HocoHz { F16MHz, F18MHz, F20MHz }
impl HocoHz {
    #[inline] pub fn hz(self) -> u32 {
        match self { Self::F16MHz=>16_000_000, Self::F18MHz=>18_000_000, Self::F20MHz=>20_000_000 }
    }
    #[inline] fn bits(self) -> u8 {
        match self { Self::F16MHz=>0b00, Self::F18MHz=>0b01, Self::F20MHz=>0b10 }
    }
    #[inline] fn fll_cntl(self) -> u16 {
        match self { Self::F16MHz=>0x1e9, Self::F18MHz=>0x226, Self::F20MHz=>0x263 }
    }
}

/// FLL
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HocoFll {
    /// FLL enable
    pub enable: bool,
    /// Details
    pub mult: FllCntlSel,
}

/// Settings for FLLCR2 -> fllcntl
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FllCntlSel {
    Pac,
    Raw(u32)
}

#[derive(Copy, Clone, Debug)]
pub struct SysDiv {
    pub ick: Div2Pow,   // src_hz / X
    pub pcka: Div2Pow,  // ick / X
    pub pckb: Div2Pow,  // ick / X
    pub pckc: Div2Pow,  // ick / X
    pub pckd: Div2Pow,  // ick / X
    pub fck: Div2Pow,   // ick / X
    pub bck: Div2Pow,   // bck / X
   
    /// EBCLK
    pub ebck: Div2Pow,
}
impl SysDiv {
    fn highest_freq_200mhz() -> Self {
        Self {
            ick: Div2Pow::Div1,
            pcka: Div2Pow::Div2,
            pckb: Div2Pow::Div4,
            pckc: Div2Pow::Div4,
            pckd: Div2Pow::Div2,
            fck: Div2Pow::Div4,
            bck: Div2Pow::Div2,
            ebck: Div2Pow::Div2,
        }
    }
}
impl Default for SysDiv {
    fn default() -> Self {
        Self {
            ick: Div2Pow::Div1,
            pcka: Div2Pow::Div2,    // 100MHz
            pckb: Div2Pow::Div4,    //  25MHz
            pckc: Div2Pow::Div4,    //  50MHz
            pckd: Div2Pow::Div2,    //  25MHz
            fck: Div2Pow::Div4,     //  50MHz
            bck: Div2Pow::Div2,     // 100MHz
            ebck: Div2Pow::Div2,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum EbclkDiv { Div1, Div2, Div4, Div8, Div16 }
impl EbclkDiv {
    #[inline] pub fn ratio(self) -> u32 {
        match self { Self::Div1=>1, EbclkDiv::Div2=>2, EbclkDiv::Div4=>4, EbclkDiv::Div8=>8, EbclkDiv::Div16=>16 }
    }
    #[inline] pub fn raw(self) -> u32 {
        match self { Self::Div1=>0, EbclkDiv::Div2=>1, EbclkDiv::Div4=>2, EbclkDiv::Div8=>3, EbclkDiv::Div16=>4 }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct EbckoCfg { pub enable: bool, pub div: EbclkDiv }

#[derive(Copy, Clone, Debug, Default)]
pub struct Limits {
    pub bclk_hs_max_hz: Option<u32>,
    pub ebclk_hs_max_hz: Option<u32>,
}

#[derive(Copy, Clone, Debug)]
pub struct  Config {
    pub mode: Mode,
    pub source: Source,
    pub divs: SysDiv,
    pub limits: Limits,
    pub ebcko: Option<EbckoCfg>,
    pub use_adc: bool,
}
/*impl Config {
    fn ek_ra6m5_highest_freq() -> Self {
        Self {
            mode: Mode::HighSpeed,
            source: Source::PllFormMain { ext_main_hz: EK_RA6M5_XTAL_HZ, plidiv: PliDiv::Div3, pllmul: 25 },
            divs: SysDiv::highest_freq_200mhz(),
            limits: Limits::default(),
            ebcko: None,
            use_adc: true
        }
    }
}*/

#[derive(Debug)]
pub struct Realized {
    pub mode: Mode,
    pub source: Source,
    pub src_hz: u32,
    pub f_pll_hz: Option<u32>,
    pub iclk_hz: u32,
    pub fclk_hz: u32,
    pub bclk_hz: u32,
    pub ebclk_hz: u32,
    pub pcka_hz: u32,
    pub pckb_hz: u32,
    pub pckc_hz: u32,
    pub pckd_hz: u32,
    pub cksel_bits: u8,
    pub ick_bits:  u8,
    pub fck_bits:  u8,
    pub bck_bits:  u8,
    pub pcka_bits: u8,
    pub pckb_bits: u8,
    pub pckc_bits: u8,
    pub pckd_bits: u8,
    pub ebck_bits: u8,
    pub plidiv_bits: Option<u8>,
    pub pllmul_bits: Option<u8>,
}

#[derive(Debug)]
pub enum Error {
    MainOutOfRange,
    IllegalCombo(&'static str),
    PllOutOfRange(u32),
    ClockTooHigh(&'static str, u32),
    ClockTooLow(&'static str, u32),
}

/*pub fn plan(cfg: Config) -> Result<Realized, Error> {
    let (src_hz, cksel_bits, plidiv_bits, pllmul_bits, f_pll_hz) = match cfg.source {
        Source::PllFormMain { ext_main_hz, plidiv, pllmul } => {
            if (ext_main_hz < MAIN_HZ_MIN) || (ext_main_hz > MAIN_HZ_MAX) { return Err(Error::MainOutOfRange); }
            if cfg.mode == Mode::SuboscSpeed { return Err(Error::IllegalCombo("Subosc requires LOCO/Subclk"));
            }
            let fpll = (ext_main_hz / plidiv.val()) * (pllmul as u32);
            if (fpll < PLL_HZ_MIN) || (fpll > PLL_HZ_MAX) { return Err(Error::PllOutOfRange(fpll)); }
            (fpll, 0b101, Some(plidiv.to_bits()), Some(pllmul_to_bits(pllmul)), Some(fpll))
        }
        Source::Main { ext_main_hz } => {
            if (ext_main_hz < MAIN_HZ_MIN) || (ext_main_hz > MAIN_HZ_MAX) { return Err(Error::MainOutOfRange); }
            if cfg.mode == Mode::SuboscSpeed { return Err(Error::IllegalCombo("Subosc requires LOCO/Subclk")); }
            (ext_main_hz, 0b011, None, None, None)
        }
        Source::Hoco { hoco_hz, .. } => {
            if cfg.mode == Mode::SuboscSpeed { return Err(Error::IllegalCombo("Subosc requires LOCO/Subclk")); }
            (hoco_hz.hz(), 0b000, None, None, None)
        }
        Source::Moco => {
            if cfg.mode == Mode::SuboscSpeed { return Err(Error::IllegalCombo("Subosc requires LOCO/Subclk")); }
            (MOCO_TYP_HZ, 0b001, None, None, None)
        }
        Source::Loco => { (LOCO_TYP_HZ, 0b010, None, None, None) }
    };

    let ick_bits = cfg.divs.ick.bits();
    let fck_bits = cfg.divs.fck.bits();
    let bck_bits = cfg.divs.bck.bits();
    let pcka_bits = cfg.divs.pcka.bits();
    let pckb_bits = cfg.divs.pckb.bits();
    let pckc_bits = cfg.divs.pckc.bits();
    let pckd_bits = cfg.divs.pckd.bits();
    let ebck_bits = cfg.divs.ebck.bits();

    let iclk = src_hz / cfg.divs.ick.val();
    let fclk = iclk / cfg.divs.fck.val();
    let bclk = iclk / cfg.divs.bck.val();
    let pclka = iclk / cfg.divs.pcka.val();
    let pclkb = iclk / cfg.divs.pckb.val();
    let pclkc = iclk / cfg.divs.pckc.val();
    let pclkd = iclk / cfg.divs.pckd.val();
    let ebclk = bclk / cfg.divs.ebck.val();

    match cfg.mode {
        Mode::HighSpeed => {
            if iclk > HIGH_SPEED_ICLK_MAX { return Err(Error::ClockTooHigh("ICLK", iclk)); }
            if fclk > HIGH_SPEED_FCLK_MAX { return Err(Error::ClockTooHigh("FCLK", fclk)); }
            if (bclk > HIGH_SPEED_BCLK_MAX) || (bclk > iclk) {
                return Err(Error::ClockTooHigh("BCLK", bclk));
            }
            if pclka > HIGH_SPEED_PCLKA_MAX { return Err(Error::ClockTooHigh("PCLKA", pclka)); }
            if pclkb > HIGH_SPEED_PCLKB_MAX { return Err(Error::ClockTooHigh("PCLKB", pclkb)); }
            if pclkc > HIGH_SPEED_PCLKC_MAX { return Err(Error::ClockTooHigh("PCLKC", pclkc)); }
            if (cfg.use_adc) && (pclkc < HIGH_SPEED_PCLKC_ADC12_MIN) {
                return Err(Error::ClockTooLow("PCLKC", pclkc));
            }
            if pclkd > HIGH_SPEED_PCLKD_MAX { return Err(Error::ClockTooHigh("PCLKD", pclkd)); }
            if let Some(max) = cfg.limits.bclk_hs_max_hz {
                if bclk > max { return Err(Error::ClockTooHigh("BCLK", bclk)); }
            }
            if let Some(max) = cfg.limits.ebclk_hs_max_hz {
                if ebclk > max { return Err(Error::ClockTooHigh("EBCLK", bclk)); }
            }
        }
        Mode::LowSpeed => {
            for (n, hz) in [
                ("ICLK", iclk), ("FCLK", fclk), ("BCLK", bclk), 
                ("PCLKA", pclka), ("PCLKB", pclkb), ("PCLKC", pclkc), ("PCLKD", pclkd), ("EBCLK", ebclk) 
            ] { if hz > LOW_SPEED_MAX { return Err(Error::ClockTooHigh(n, hz)); } }
        }
        Mode::SuboscSpeed => {
            for (n, hz) in [
                ("ICLK", iclk), ("FCLK", fclk) 
            ] { if hz < SUBOSC_SPEED_ICLK_FCLK_MIN { return Err(Error::ClockTooLow(n, hz)); } }
            for (n, hz) in [
                ("ICLK", iclk), ("FCLK", fclk), ("BCLK", bclk), 
                ("PCLKA", pclka), ("PCLKB", pclkb), ("PCLKC", pclkc), ("PCLKD", pclkd), ("EBCLK", ebclk) 
            ] { if hz > SUBOSC_SPEED_MAX { return Err(Error::ClockTooHigh(n, hz)); } }
            if cfg.source != Source::Loco { return Err(Error::IllegalCombo("Subosc mode must use LOCO as source")); }
        }
    }

    Ok(Realized {
        mode: cfg.mode, 
        source: cfg.source,
        src_hz, 
        f_pll_hz, 
        iclk_hz: iclk, 
        fclk_hz: fclk, 
        bclk_hz: bclk, 
        ebclk_hz: ebclk, 
        pcka_hz: pclka, pckb_hz: pclkb, pckc_hz: pclkc, pckd_hz: pclkd, 
        cksel_bits, 
        ick_bits, 
        fck_bits, 
        bck_bits, 
        pcka_bits, pckb_bits, pckc_bits, pckd_bits, 
        ebck_bits, 
        plidiv_bits, 
        pllmul_bits
    })
}*/

/*pub fn apply(realized: Realized) -> Result<(), Error> {
    unsafe {
        prc0(true);

        match realized.source {
            Source::PllFormMain { .. } | Source::Main { .. } => {

            }
        }

        //let memwait = cfg.memwait.unwrap_or(if r.iclk_hz > 120_000_000 { 1 } else { 0 });
        
    }

    Ok(())
}*/

/// PRCR (PRKEY=0xA5, PRC0=1/0)
unsafe fn prc0(enable: bool) {
    unsafe {
        pac::SYSC.prcr().write(
        pac::sysc::Prcr::default()
            .prkey().set(0xA5)
            .prc0().set(if enable { pac::sysc::prcr::Prc0::_1 } else { pac::sysc::prcr::Prc0::_0 })
        );
    }
}

unsafe fn require_main_on() -> Result<(), Error> {
    Ok(())
}

#[inline]
fn pllmul_to_bits(mul: u8) -> u8 {
    mul.saturating_sub(1)
}