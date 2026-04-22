/// 今回スキップ
/// ・CGFSAR: クロック関連レジスタのセキュア・非セキュアを切り替え
/// 
/// 全体
/// 書き込み前にPRCR.PRC0=1、書き込み後にPRCR.PRC0=0

//#![allow(dead_code)]

use crate::fcache::{Fcache, FlashWait};
use crate::pac;
use crate::RegisterError;
//use crate::ofs1::{Ofs1, OFS1_CELL};
use crate::dbg::{Dbg, DBG_CELL};
use crate::mstp::{Mstp, MSTP_CELL};
use crate::rtc::{Rtc, RTC_CELL};
use super::_is_prc0;
use super::{SYSC_CELL, _prc0};

use cortex_m::peripheral::{DCB, DWT};
use paste::paste;
use ra6m5_pac::RegisterValue;

pub const EK_RA6M5_XTAL_HZ: u32 = 24_000_000;

pub const MOSC_HZ_MIN: u32 =      8_000_000;
pub const MOSC_HZ_MAX: u32 =     24_000_000;
pub const PLL_HZ_MIN: u32 =     120_000_000;
pub const PLL_HZ_MAX: u32 =     240_000_000;

pub const MOCO_HZ: u32 =    8_000_000;
pub const LOCO_HZ: u32 =       32_768;
pub const SOSC_HZ: u32 =       32_768;

pub const SOSC_HZ_MAX: u32 =   36_100;
pub const SOSC_HZ_MIN: u32 =   29_400;

pub const ICLK_HZ_MAX: u32 =          200_000_000;
pub const PCLKA_HZ_MAX: u32 =         100_000_000;
pub const PCLKB_HZ_MAX: u32 =          50_000_000;
pub const PCLKC_HZ_MAX: u32 =          50_000_000;
pub const PCLKD_HZ_MAX: u32 =         100_000_000;
pub const FCLK_HZ_MAX: u32 =           50_000_000;
pub const BCLK_HZ_MAX: u32 =          100_000_000;
pub const EBCLK_HZ_MAX: u32 =          50_000_000;
pub const PCLKC_ADC12_HZ_MIN: u32 =     1_000_000;
pub const PCLKA_ETHERC_HZ_MAX: u32 =  100_000_000;
pub const PCLKA_ETHERC_HZ_MIN: u32 =   12_500_000;

pub const USBCLK_HZ: u32 =      48_000_000;
pub const USB60CLK_HZ: u32 =    60_000_000;

pub const OCTACLK_HZ_MAX: u32 =     200_000_000;
pub const CANFDCLK_HZ_MAX: u32 =     40_000_000;
pub const CECCLK_HZ_MAX: u32 =       20_000_000;
pub const TRCLK_HZ_MAX: u32 =       100_000_000;

#[inline(always)]
fn dwt_enable_once(dcb: &mut DCB, dwt: &mut DWT) {
    dcb.enable_trace();
    DWT::unlock();
    dwt.enable_cycle_counter();
}

#[inline(always)]
pub fn busy_wait_ns_with_dwt(ns: u32, iclk_hz: u32) {
    let start = DWT::cycle_count();
    let cycles = (((ns as u128) * (iclk_hz as u128) + 999_999_999u128) / 1_000_000_000u128) as u32;
    while DWT::cycle_count().wrapping_sub(start) < cycles {}
}

pub struct Clocks {
    freqs: ClocksFreq,
    pub pll: Pll,
    pub pll2: Pll2,
    pub mosc: Mosc,
    pub sosc: Sosc,
    pub hoco: Hoco,
    pub moco: Moco,
    pub loco: Loco,
    pub ebclk: Ebclk,

    pub clkout: ClockOut,
    pub usbclk: UsbClk,
    pub octaclk: OctaClk,
    pub canfdclk: CanfdClk,
    pub usb60clk: Usb60Clk,
    pub cecclk: CecClk,
    pub trclk: TrClk,

    use_adc12: bool,
    use_canfd: bool,
    use_etherc: bool,
}
pub struct ClocksConfig {
    source: ClocksSource,
    pckd: u32, pckc: u32, pckb: u32, pcka: u32,
    bck: u32, ick: u32, fck: u32,
    pckd_div: ClocksDiv, pckc_div: ClocksDiv, pckb_div: ClocksDiv, pcka_div: ClocksDiv, 
    bck_div: ClocksDiv, ick_div: ClocksDiv, fck_div: ClocksDiv, 
    use_adc12: bool, use_canfd: bool, use_etherc: bool,
}
impl Default for ClocksConfig {
    fn default() -> Self {
        Self {
            source: ClocksSource::Moco,
            pckd: 2_000_000, pckc: 2_000_000, pckb: 2_000_000, pcka: 2_000_000,
            bck: 2_000_000, ick: 2_000_000, fck: 2_000_000,
            pckd_div: ClocksDiv::Div4, pckc_div: ClocksDiv::Div4, pckb_div: ClocksDiv::Div4, pcka_div: ClocksDiv::Div4, 
            bck_div: ClocksDiv::Div4, ick_div: ClocksDiv::Div4, fck_div: ClocksDiv::Div4, 
            use_adc12: true, use_canfd: false, use_etherc: false,
        }
    }
}
impl ClocksConfig {
    pub fn new<T: ClocksSources>(
        source: &mut T,
        ick_div: ClocksDiv, pcka_div: ClocksDiv, pckb_div: ClocksDiv, pckc_div: ClocksDiv, pckd_div: ClocksDiv, 
        fck_div: ClocksDiv, bck_div: ClocksDiv, use_adc12: bool, use_canfd: bool, use_etherc: bool,
    ) -> Result<Self, RegisterError> {
        let base = source.hz()?;
        let pckd = base / pckd_div.value();
        let pckc = base / pckc_div.value();
        let pckb = base / pckb_div.value();
        let pcka = base / pcka_div.value();
        let bck = base / bck_div.value();
        let ick = base / ick_div.value();
        let fck = base / fck_div.value();

        if ick > ICLK_HZ_MAX { return  Err(RegisterError::InvalidValue("FOLLOW THIS RULE: ICLK <= 200MHz.")); }
        if pcka > PCLKA_HZ_MAX { return  Err(RegisterError::InvalidValue("FOLLOW THIS RULE: PCLKA <= 100MHz.")); }
        if pckb > PCLKB_HZ_MAX { return  Err(RegisterError::InvalidValue("FOLLOW THIS RULE: PCLKB <= 50MHz.")); }
        if pckc > PCLKC_HZ_MAX { return  Err(RegisterError::InvalidValue("FOLLOW THIS RULE: PCLKC <= 50MHz.")); }
        if pckd > PCLKD_HZ_MAX { return  Err(RegisterError::InvalidValue("FOLLOW THIS RULE: PCLKD <= 100MHz.")); }
        if fck > FCLK_HZ_MAX { return  Err(RegisterError::InvalidValue("FOLLOW THIS RULE: FCLK <= 50MHz.")); }
        if bck > BCLK_HZ_MAX { return  Err(RegisterError::InvalidValue("FOLLOW THIS RULE: ICLK <= 100MHz.")); }

        if (ick < pcka) || (pcka < pckb) { return Err(RegisterError::InvalidValue("FOLLOW THIS RULE: ICLK >= PCLKA >= PCLKB")); }
        if (pckd < pcka) || (pcka < pckb) { return Err(RegisterError::InvalidValue("FOLLOW THIS RULE: PCLKD >= PCLKA >= PCLKB")); }
        if ick < fck { return Err(RegisterError::InvalidValue("FOLLOW THIS RULE: ICLK >= FCLK")); }
        if ick < bck { return Err(RegisterError::InvalidValue("FOLLOW THIS RULE: ICLK >= BCLK")); }

        if use_adc12 {
            if pckc < PCLKC_ADC12_HZ_MIN { return  Err(RegisterError::InvalidValue("FOLLOW THIS RULE: PCLKC >= 1MHz.")); }
            if pcka >= pckc {
                if pcka_div.value() / pckc_div.value() > 8 {
                    return Err(RegisterError::InvalidValue("PCLKA:PCLKC must be 1:1, 2:1, 4:1, 8:1, 1:2 or 1:4"));
                }
            } else {
                if pckc_div.value() / pcka_div.value() > 4 {
                    return Err(RegisterError::InvalidValue("PCLKA:PCLKC must be 1:1, 2:1, 4:1, 8:1, 1:2 or 1:4"));
                }
            }
        }

        if use_canfd && pcka_div.value() / pckb_div.value() != 2 {
            return Err(RegisterError::InvalidValue("FOLLOW THIS RULE: PCLKA:PCLKB = 2:1"));
        }

        if use_etherc && ((pcka > PCLKA_ETHERC_HZ_MAX) || (pcka < PCLKA_ETHERC_HZ_MIN)) {
            return Err(RegisterError::InvalidValue("FOLLOW THIS RULE: 12.5MHz <= PCLKA <= 100MHz"));
        } 

        Ok(Self {
            source: source.as_source(),
            pckd, pckc, pckb, pcka, 
            bck, ick, fck, 
            pckd_div, pckc_div, pckb_div, pcka_div, 
            bck_div, ick_div, fck_div, 
            use_adc12, use_canfd, use_etherc
        })
    }
    pub fn source(&self) -> ClocksSource { self.source }
    pub fn pckd(&self) -> u32 { self.pckd }
    pub fn pckc(&self) -> u32 { self.pckc }
    pub fn pckb(&self) -> u32 { self.pckb }
    pub fn pcka(&self) -> u32 { self.pcka }
    pub fn bck(&self) -> u32 { self.bck }
    pub fn ick(&self) -> u32 { self.ick }
    pub fn fck(&self) -> u32 { self.fck }
    pub fn pckd_div(&self) -> ClocksDiv { self.pckd_div }
    pub fn pckc_div(&self) -> ClocksDiv { self.pckc_div }
    pub fn pckb_div(&self) -> ClocksDiv { self.pckb_div }
    pub fn pcka_div(&self) -> ClocksDiv { self.pcka_div }
    pub fn bck_div(&self) -> ClocksDiv { self.bck_div }
    pub fn ick_div(&self) -> ClocksDiv { self.ick_div }
    pub fn fck_div(&self) -> ClocksDiv { self.fck_div }
    pub fn use_adc12(&self) -> bool { self.use_adc12 }
    pub fn use_canfd(&self) -> bool { self.use_canfd }
    pub fn use_etherc(&self) -> bool { self.use_etherc }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClocksSource { Hoco=0b000, Moco=0b001, Loco=0b010, Mosc=0b011, Sosc=0b100, Pll=0b101 }
impl ClocksSource {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b000=>Some(Self::Hoco), 0b001=>Some(Self::Moco), 0b010=>Some(Self::Loco), 
            0b011=>Some(Self::Mosc), 0b100=>Some(Self::Sosc), 0b101=>Some(Self::Pll), _=>None
        }
    }
}
pub trait ClocksSources {
    fn hz(&mut self) -> Result<u32, RegisterError>;
    fn as_source(&self) -> ClocksSource;
}
impl ClocksSources for Hoco {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        if let Some(freq) = self.get_freq() { Ok(freq.hz()) }
        else { Err(RegisterError::InvalidValue("HOCO has unknown frequency.")) }
    }
    fn as_source(&self) -> ClocksSource { ClocksSource::Hoco }
}
impl ClocksSources for Moco {
    fn hz(&mut self) -> Result<u32, RegisterError> { Ok(MOCO_HZ) }
    fn as_source(&self) -> ClocksSource { ClocksSource::Moco }
}
impl ClocksSources for Loco {
    fn hz(&mut self) -> Result<u32, RegisterError> { Ok(LOCO_HZ) }
    fn as_source(&self) -> ClocksSource { ClocksSource::Loco }
}
impl ClocksSources for Mosc {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        if let Some(freq) = self.get_freq() { Ok(freq) }
        else { Err(RegisterError::InvalidValue("MOSC is not initialized.")) }
    }
    fn as_source(&self) -> ClocksSource { ClocksSource::Mosc }
}
impl ClocksSources for Sosc {
    fn hz(&mut self) -> Result<u32, RegisterError> { Ok(SOSC_HZ) }
    fn as_source(&self) -> ClocksSource { ClocksSource::Sosc }
}
impl ClocksSources for Pll {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        if let Some(freq) = self.get_freq() { Ok(freq) }
        else { Err(RegisterError::InvalidValue("Pll is not initialized.")) }
    }
    fn as_source(&self) -> ClocksSource { ClocksSource::Pll }
}
/// SCKDIVCR
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ClocksDiv {
    Div1=0b000, Div2=0b001, Div4=0b010, Div8=0b011, Div16=0b100, Div32=0b101, Div64=0b110
}
impl ClocksDiv {
    #[inline] pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b000=>Some(Self::Div1), 0b001=>Some(Self::Div2), 0b010=>Some(Self::Div4), 0b011=>Some(Self::Div8), 
            0b100=>Some(Self::Div16), 0b101=>Some(Self::Div32), 0b110=>Some(Self::Div64), _=>None
        }
    }
    #[inline] pub fn value(self) -> u32 { 1u32 << (self as u32) }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ClocksFreq {
    pub pckd: u32,
    pub pckc: u32,
    pub pckb: u32,
    pub pcka: u32,
    pub bck: u32,
    pub ick: u32,
    pub fck: u32
}
impl Default for ClocksFreq {
    fn default() -> Self {
        Self {
            pckd: 2_000_000,
            pckc: 2_000_000,
            pckb: 2_000_000,
            pcka: 2_000_000,
            bck: 2_000_000,
            ick: 2_000_000,
            fck: 2_000_000,
        }
    }
}
macro_rules! clock_impl_core {
    ($name:ident) => {
        impl $name {
            fn _with_cs<R>(&mut self, f: impl FnOnce(&mut pac::Sysc) -> R) -> R {
                critical_section::with(|cs| {
                    let mut bor = SYSC_CELL.borrow(cs).borrow_mut();
                    let sysc = bor.as_mut().expect("SYSC is not initialized");

                    f(sysc)
                })
            }
            fn _with_prcr<R>(&mut self, f: impl FnOnce(&mut pac::Sysc) -> R) -> R {
                self._with_cs(|sysc| unsafe {
                    _prc0(sysc, true);
                    let r = f(sysc);
                    _prc0(sysc, false);
                    r
                })
            }
        }
    };
}
macro_rules! clock_impl_with {
    ($name:ident, $with:tt) => {
        paste! {
            impl $name {
                fn [<_with_cs_ $with:lower>]<R>(&mut self, f: impl FnOnce(&mut pac::Sysc, &mut pac::$with) -> R) -> R {
                    critical_section::with(|cs| {
                        let mut bor_sysc = SYSC_CELL.borrow(cs).borrow_mut();
                        let sysc = bor_sysc.as_mut().expect("SYSC is not initialized");
                        let mut [<bor_ $with:lower>] = [<$with:upper _CELL>].borrow(cs).borrow_mut();
                        let [<$with:lower>] = [<bor_ $with:lower>].as_mut().expect(concat!(stringify!([<$with:upper>]), " is not initialized"));

                        f(sysc, [<$with:lower>])
                    })
                }
                fn [<_with_prcr_ $with:lower>]<R>(&mut self, f: impl FnOnce(&mut pac::Sysc, &mut pac::$with) -> R) -> R {
                    self.[<_with_cs_ $with:lower>](|sysc, [<$with:lower>]| unsafe {
                        _prc0(sysc, true);
                        let r = f(sysc, [<$with:lower>]);
                        _prc0(sysc, false);
                        r
                    })
                }
            }
        }
    }
}
clock_impl_core!(Clocks);
clock_impl_with!(Clocks, Mstp);
impl Clocks {
    pub(crate) fn init() -> Self {
        Self {
            freqs: ClocksFreq::default(),
            pll: Pll { freq: None },
            pll2: Pll2 { freq: None },
            mosc: Mosc { freq: None },
            sosc: Sosc { _id: () },
            hoco: Hoco { _id: () },
            moco: Moco { _id: () },
            loco: Loco { _id: () },
            ebclk: Ebclk { _id: () },

            clkout: ClockOut { _id: () },
            usbclk: UsbClk { _id: () },
            octaclk: OctaClk { _id: () },
            canfdclk: CanfdClk { _id: () },
            usb60clk: Usb60Clk { _id: () },
            cecclk: CecClk { _id: () },
            trclk: TrClk { _id: () },

            use_adc12: true,
            use_canfd: false,
            use_etherc: false,
        }
    }
    /*fn _with_cs_mstp_fcache<R>(&mut self, f: impl FnOnce(&mut pac::Sysc, &mut pac::Mstp, &mut pac::Fcache) -> R) -> R {
        critical_section::with(|cs| {
            let mut bor_sysc = SYSC_CELL.borrow(cs).borrow_mut();
            let sysc = bor_sysc.as_mut().expect("SYSC is not initialized");
            let mut bor_mstp = MSTP_CELL.borrow(cs).borrow_mut();
            let mstp = bor_mstp.as_mut().expect("MSTP is not initialized");
            let mut bor_fcache = FCACHE_CELL.borrow(cs).borrow_mut();
            let fcache = bor_fcache.as_mut().expect("FCACHE is not initialized");

            f(sysc, mstp, fcache)
        })
    }
    fn _with_prcr_mstp_fcache<R>(&mut self, f: impl FnOnce(&mut pac::Sysc, &mut pac::Mstp, &mut pac::Fcache) -> R) -> R {
        self._with_cs_mstp_fcache(|sysc, mstp, fcache| unsafe {
            _prc0(sysc, true);
            let r = f(sysc, mstp, fcache);
            _prc0(sysc, false);
            r
        })
    }*/
    fn _set_prc0(&mut self, enable: bool) -> Result<(), RegisterError> {
        self._with_cs(|sysc| unsafe { _prc0(sysc, enable); });
        Ok(())
    }
    pub fn enable_clock_write(&mut self) -> Result<(), RegisterError> { self._set_prc0(true) }
    pub fn disable_clock_write(&mut self) -> Result<(), RegisterError> { self._set_prc0(false) }
    pub fn is_clock_write_enabled(&mut self) -> bool {
        self._with_cs(|sysc| unsafe { _is_prc0(sysc) })
    }

    pub fn get_pckd_div(&mut self) -> Option<ClocksDiv> {
        self._with_cs(|sysc| unsafe { ClocksDiv::from_u8(sysc.sckdivcr().read().pckd().get().0) })
    }
    pub fn get_pckc_div(&mut self) -> Option<ClocksDiv> {
        self._with_cs(|sysc| unsafe { ClocksDiv::from_u8(sysc.sckdivcr().read().pckc().get().0) })
    }
    pub fn get_pckb_div(&mut self) -> Option<ClocksDiv> {
        self._with_cs(|sysc| unsafe { ClocksDiv::from_u8(sysc.sckdivcr().read().pckb().get().0) })
    }
    pub fn get_pcka_div(&mut self) -> Option<ClocksDiv> {
        self._with_cs(|sysc| unsafe { ClocksDiv::from_u8(sysc.sckdivcr().read().pcka().get().0) })
    }
    pub fn get_bck_div(&mut self) -> Option<ClocksDiv> {
        self._with_cs(|sysc| unsafe { ClocksDiv::from_u8(sysc.sckdivcr().read().bck().get().0) })
    }
    pub fn get_ick_div(&mut self) -> Option<ClocksDiv> {
        self._with_cs(|sysc| unsafe { ClocksDiv::from_u8(sysc.sckdivcr().read().ick().get().0) })
    }
    pub fn get_fck_div(&mut self) -> Option<ClocksDiv> {
        self._with_cs(|sysc| unsafe { ClocksDiv::from_u8(sysc.sckdivcr().read().fck().get().0) })
    }
    pub fn get_freqs(&self) -> ClocksFreq { self.freqs }
    pub fn get_source(&mut self) -> Option<ClocksSource> {
        self._with_cs(|sysc| unsafe {
            ClocksSource::from_u8(sysc.sckscr().read().cksel().get().0)
        })
    }
    pub fn is_adc12_available(&self) -> bool { self.use_adc12 }
    pub fn is_canfd_available(&self) -> bool { self.use_canfd }
    pub fn is_etherc_available(&self) -> bool { self.use_etherc }
    pub fn set_config(&mut self, config: &ClocksConfig, _mstp: &mut Mstp, fcache: &mut Fcache, dcb: &mut DCB, dwt: &mut DWT) -> Result<(), RegisterError> {
        let source_old = self.get_source();
        let source_new = config.source;
        let iclk_old = self.get_freqs().ick;
        let iclk_new = config.ick;
        let is_iclk_higher = iclk_new >= iclk_old;
        let flwt = FlashWait::from_iclk(iclk_new).unwrap();
        let iclk_div_new = config.ick_div;
        let iclk_tmp = match self.get_ick_div() {
            Some(iclk_div_old) => { iclk_new * iclk_div_new.value() / iclk_div_old.value() }
            None => { iclk_new * iclk_div_new.value() / ClocksDiv::Div64.value() }
        };
        if is_iclk_higher { fcache.set_flash_wait(flwt).unwrap(); }
        self._with_prcr_mstp(|sysc, mstp| unsafe {
            let is_pll = match (source_old, source_new) {
                (None, _) => true,
                (Some(ClocksSource::Pll), _) => true,
                (_, ClocksSource::Pll) => true,
                _ => false
            };
            if is_pll { 
                dwt_enable_once(dcb, dwt);

                let r = mstp.mstpcrb().read();
                let b12_flg = if r.mstpb12().get() == pac::mstp::mstpcrb::Mstpb12::_0 {
                    mstp.mstpcrb().write(r.mstpb12().set(pac::mstp::mstpcrb::Mstpb12::_1));
                    true
                } else { false };
                busy_wait_ns_with_dwt(250, iclk_old);

                let r = mstp.mstpcrc().read();
                let c31_flg = if r.mstpc31().get() == pac::mstp::mstpcrc::Mstpc31::_0 {
                    mstp.mstpcrc().write(r.mstpc31().set(pac::mstp::mstpcrc::Mstpc31::_1));
                    true
                } else { false };
                busy_wait_ns_with_dwt(250, iclk_old);

                match (source_old, source_new) {
                    (Some(ClocksSource::Pll), ClocksSource::Pll) => {}
                    (Some(ClocksSource::Pll), _) => {
                        busy_wait_ns_with_dwt(750, iclk_old);
                        sysc.sckscr().write(sysc.sckscr().read().cksel().set(match source_new {
                            ClocksSource::Hoco => pac::sysc::sckscr::Cksel::_000,
                            ClocksSource::Moco => pac::sysc::sckscr::Cksel::_001,
                            ClocksSource::Loco => pac::sysc::sckscr::Cksel::_010,
                            ClocksSource::Mosc => pac::sysc::sckscr::Cksel::_011,
                            ClocksSource::Sosc => pac::sysc::sckscr::Cksel::_100,
                            ClocksSource::Pll => pac::sysc::sckscr::Cksel::_101
                        }));
                    }
                    (_, _) => {
                        sysc.sckscr().write(sysc.sckscr().read().cksel().set(pac::sysc::sckscr::Cksel::_101));
                        busy_wait_ns_with_dwt(250, iclk_tmp);
                    }
                }

                busy_wait_ns_with_dwt(750, iclk_tmp);
                sysc.sckdivcr().write(sysc.sckdivcr().read()
                    .pckd().set(match config.pckd_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Pckd::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Pckd::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Pckd::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Pckd::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Pckd::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Pckd::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Pckd::_110,
                    }).pckc().set(match config.pckc_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Pckc::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Pckc::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Pckc::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Pckc::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Pckc::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Pckc::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Pckc::_110,
                    }).pckb().set(match config.pckb_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Pckb::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Pckb::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Pckb::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Pckb::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Pckb::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Pckb::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Pckb::_110,
                    }).pcka().set(match config.pcka_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Pcka::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Pcka::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Pcka::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Pcka::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Pcka::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Pcka::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Pcka::_110,
                    }).bck().set(match config.bck_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Bck::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Bck::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Bck::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Bck::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Bck::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Bck::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Bck::_110,
                    }).ick().set(match config.ick_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Ick::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Ick::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Ick::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Ick::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Ick::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Ick::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Ick::_110,
                    }).fck().set(match config.fck_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Fck::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Fck::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Fck::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Fck::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Fck::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Fck::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Fck::_110,
                    })
                );
                busy_wait_ns_with_dwt(250, iclk_new);

                if b12_flg {
                    mstp.mstpcrb().write(mstp.mstpcrb().read().mstpb12().set(pac::mstp::mstpcrb::Mstpb12::_0));
                    busy_wait_ns_with_dwt(250, iclk_new);
                }
                if c31_flg {
                    mstp.mstpcrc().write(mstp.mstpcrc().read().mstpc31().set(pac::mstp::mstpcrc::Mstpc31::_0));
                    busy_wait_ns_with_dwt(250, iclk_new);
                }
            } else {
                sysc.sckscr().write(sysc.sckscr().read().cksel().set(match source_new {
                    ClocksSource::Hoco => pac::sysc::sckscr::Cksel::_000,
                    ClocksSource::Moco => pac::sysc::sckscr::Cksel::_001,
                    ClocksSource::Loco => pac::sysc::sckscr::Cksel::_010,
                    ClocksSource::Mosc => pac::sysc::sckscr::Cksel::_011,
                    ClocksSource::Sosc => pac::sysc::sckscr::Cksel::_100,
                    ClocksSource::Pll => pac::sysc::sckscr::Cksel::_101
                }));
                sysc.sckdivcr().write(sysc.sckdivcr().read()
                    .pckd().set(match config.pckd_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Pckd::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Pckd::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Pckd::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Pckd::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Pckd::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Pckd::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Pckd::_110,
                    }).pckc().set(match config.pckc_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Pckc::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Pckc::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Pckc::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Pckc::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Pckc::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Pckc::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Pckc::_110,
                    }).pckb().set(match config.pckb_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Pckb::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Pckb::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Pckb::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Pckb::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Pckb::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Pckb::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Pckb::_110,
                    }).pcka().set(match config.pcka_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Pcka::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Pcka::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Pcka::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Pcka::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Pcka::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Pcka::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Pcka::_110,
                    }).bck().set(match config.bck_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Bck::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Bck::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Bck::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Bck::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Bck::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Bck::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Bck::_110,
                    }).ick().set(match config.ick_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Ick::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Ick::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Ick::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Ick::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Ick::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Ick::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Ick::_110,
                    }).fck().set(match config.fck_div {
                        ClocksDiv::Div1  => pac::sysc::sckdivcr::Fck::_000,
                        ClocksDiv::Div2  => pac::sysc::sckdivcr::Fck::_001,
                        ClocksDiv::Div4  => pac::sysc::sckdivcr::Fck::_010,
                        ClocksDiv::Div8  => pac::sysc::sckdivcr::Fck::_011,
                        ClocksDiv::Div16 => pac::sysc::sckdivcr::Fck::_100,
                        ClocksDiv::Div32 => pac::sysc::sckdivcr::Fck::_101,
                        ClocksDiv::Div64 => pac::sysc::sckdivcr::Fck::_110,
                    })
                );
            }
        });
        if !is_iclk_higher { fcache.set_flash_wait(flwt).unwrap(); }
        self.freqs = ClocksFreq {
            pckd: config.pckd, pckc: config.pckc, pckb: config.pckb, pcka: config.pcka, bck: config.bck, ick: config.ick, fck: config.fck
        };
        self.use_adc12 = config.use_adc12;
        self.use_canfd = config.use_canfd;
        self.use_etherc = config.use_etherc;
        Ok(())
    }

    pub fn is_oscillation_stop_detection_interrupt_enabled(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.ostdcr().read().ostdie().get() == pac::sysc::ostdcr::Ostdie::_1
        })
    }
    fn _set_ostdie(&mut self, enable: bool) -> Result<(), RegisterError> {
        if self.is_oscillation_stop_detection_interrupt_enabled() == enable {
            return Err(RegisterError::NotReadyToWrite(if enable { "OSTDI is already enabled."} else { "OSTDI is already disabled." }));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.ostdcr().write(sysc.ostdcr().read().ostdie().set(
                if enable { pac::sysc::ostdcr::Ostdie::_1 } else { pac::sysc::ostdcr::Ostdie::_0 }
            ))
        });
        Ok(())
    }
    pub fn enable_oscillation_stop_detection_interrupt(&mut self) -> Result<(), RegisterError> { self._set_ostdie(true) }
    pub fn disable_oscillation_stop_detection_interrupt(&mut self) -> Result<(), RegisterError> { self._set_ostdie(false) }
    pub fn is_oscillation_stop_detection_enabled(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.ostdcr().read().ostde().get() == pac::sysc::ostdcr::Ostde::_1
        })
    }
    pub fn enable_oscillation_stop_detection(&mut self) -> Result<(), RegisterError> {
        if self.is_oscillation_stop_detection_enabled() {
            return Err(RegisterError::NotReadyToWrite("Oscillation stop detection is already enabled."));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.ostdcr().write(sysc.ostdcr().read().ostde().set(pac::sysc::ostdcr::Ostde::_1));
        });
        Ok(())
    }
    pub fn disable_oscillation_stop_detection(&mut self) -> Result<(), RegisterError> {
        if !self.is_oscillation_stop_detection_enabled() {
            return Err(RegisterError::NotReadyToWrite("Oscillation stop detection is already disabled."));
        } else if self.is_oscillation_stop_detected() {
            return Err(RegisterError::NotReadyToWrite("Oscillation stop is detected."));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.ostdcr().write(sysc.ostdcr().read().ostde().set(pac::sysc::ostdcr::Ostde::_0));
        });
        Ok(())
    }
    pub fn is_oscillation_stop_detected(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.ostdsr().read().ostdf().get() == pac::sysc::ostdsr::Ostdf::_1
        })
    }
    pub fn reset_oscillation_stop_detection(&mut self) -> Result<(), RegisterError> {
        if self.is_oscillation_stop_detection_interrupt_enabled() {
            return Err(RegisterError::NotReadyToWrite("OSTDI is disabled."));
        } else if !self.is_oscillation_stop_detected() {
            return Err(RegisterError::NotReadyToWrite("Oscillation stop is not detected."));
        }
        if let Some(src) = self.get_source() {
            if src == ClocksSource::Mosc {
                return Err(RegisterError::NotReadyToWrite("MOSC is selected for system clock source."));
            } else if (src == ClocksSource::Pll) && (self.pll.get_source() == Some(PllSource::Mosc)) {
                return Err(RegisterError::NotReadyToWrite("PLL (source: MOSC) is selected for system clock source."));
            }
        }
        self._with_prcr(|sysc| unsafe {
            sysc.ostdsr().write(sysc.ostdsr().read().ostdf().set(pac::sysc::ostdsr::Ostdf::_0));
        });
        Ok(())
    }
}

pub struct Pll { freq: Option<u32> }
pub struct Pll2 { freq: Option<u32> }
pub trait PllSources {
    fn hz(&mut self) -> Option<u32>;
    fn as_source(&self) -> PllSource;
}
impl PllSources for Mosc {
    fn hz(&mut self) -> Option<u32> { self.freq }
    fn as_source(&self) -> PllSource { PllSource::Mosc }
}
impl PllSources for Hoco {
    fn hz(&mut self) -> Option<u32> {
        if let Some(hf) = self.get_freq() { Some(hf.hz()) } else { None }
    }
    fn as_source(&self) -> PllSource { PllSource::Hoco }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PliDiv { Div1=0b00, Div2=0b01, Div3=0b10 }
impl PliDiv {
    #[inline] fn value(self) -> u32 { match self { Self::Div1=>1, Self::Div2=>2, Self::Div3=>3 } }
    #[inline] fn from_u8(value: u8) -> Option<Self> {
        match value { 0b00=>Some(Self::Div1), 0b01=>Some(Self::Div2), 0b10=>Some(Self::Div3), _=>None }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PllMul {
    Mul10_0=0x13, Mul10_5=0x14, Mul11_0=0x15, Mul11_5=0x16, Mul12_0=0x17, Mul12_5=0x18, Mul13_0=0x19, Mul13_5=0x1a, 
    Mul14_0=0x1b, Mul14_5=0x1c, Mul15_0=0x1d, Mul15_5=0x1e, Mul16_0=0x1f, Mul16_5=0x20, Mul17_0=0x21, Mul17_5=0x22, 
    Mul18_0=0x23, Mul18_5=0x24, Mul19_0=0x25, Mul19_5=0x26, Mul20_0=0x27, Mul20_5=0x28, Mul21_0=0x29, Mul21_5=0x2a, 
    Mul22_0=0x2b, Mul22_5=0x2c, Mul23_0=0x2d, Mul23_5=0x2e, Mul24_0=0x2f, Mul24_5=0x30, Mul25_0=0x31, Mul25_5=0x32, 
    Mul26_0=0x33, Mul26_5=0x34, Mul27_0=0x35, Mul27_5=0x36, Mul28_0=0x37, Mul28_5=0x38, Mul29_0=0x39, Mul29_5=0x3a, Mul30_0=0x3b
}
impl PllMul {
    #[inline] fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x13=>Some(Self::Mul10_0), 0x14=>Some(Self::Mul10_5), 0x15=>Some(Self::Mul11_0), 0x16=>Some(Self::Mul11_5), 0x17=>Some(Self::Mul12_0), 0x18=>Some(Self::Mul12_5), 0x19=>Some(Self::Mul13_0), 0x1a=>Some(Self::Mul13_5),
            0x1b=>Some(Self::Mul14_0), 0x1c=>Some(Self::Mul14_5), 0x1d=>Some(Self::Mul15_0), 0x1e=>Some(Self::Mul15_5), 0x1f=>Some(Self::Mul16_0), 0x20=>Some(Self::Mul16_5), 0x21=>Some(Self::Mul17_0), 0x22=>Some(Self::Mul17_5),
            0x23=>Some(Self::Mul18_0), 0x24=>Some(Self::Mul18_5), 0x25=>Some(Self::Mul19_0), 0x26=>Some(Self::Mul19_5), 0x27=>Some(Self::Mul20_0), 0x28=>Some(Self::Mul20_5), 0x29=>Some(Self::Mul21_0), 0x2a=>Some(Self::Mul21_5),
            0x2b=>Some(Self::Mul22_0), 0x2c=>Some(Self::Mul22_5), 0x2d=>Some(Self::Mul23_0), 0x2e=>Some(Self::Mul23_5), 0x2f=>Some(Self::Mul24_0), 0x30=>Some(Self::Mul24_5), 0x31=>Some(Self::Mul25_0), 0x32=>Some(Self::Mul25_5),
            0x33=>Some(Self::Mul26_0), 0x34=>Some(Self::Mul26_5), 0x35=>Some(Self::Mul27_0), 0x36=>Some(Self::Mul27_5), 0x37=>Some(Self::Mul28_0), 0x38=>Some(Self::Mul28_5), 0x39=>Some(Self::Mul29_0), 0x3a=>Some(Self::Mul29_5), 
            0x3b=>Some(Self::Mul30_0), _=>None
        }
    }
    #[inline] fn value(self) -> f32 {
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
pub enum PllSource { Mosc=0, Hoco=1 }
impl PllSource {
    fn from_u8(value: u8) -> Option<Self> { match value { 0=>Some(Self::Mosc), 1=>Some(Self::Hoco), _=>None } }
}
pub struct PllConfig {
    freq: u32,
    div: PliDiv,
    source: PllSource,
    mul: PllMul,
}
impl PllConfig {
    pub fn new<T: PllSources>(source: &mut T, div: PliDiv, mul: PllMul) -> Result<Self, RegisterError> {
        if let Some(base) = source.hz() {
            let freq = (base as f32 * mul.value() / div.value() as f32) as u32;
            if (freq >= PLL_HZ_MIN) && (freq <= PLL_HZ_MAX) {
                Ok(Self { freq, div, source: source.as_source(), mul })
            } else {
                Err(RegisterError::InvalidValue("Frequency outside the range of PLL."))
            }
        } else {
            Err(RegisterError::NotReadyToWrite("The oscillator frequency is not initialized."))
        }
    }
}
macro_rules! pll_impl {
    ($name:tt, $short:tt, $names:tt, $shorts:tt) => {
        paste! {
            clock_impl_core!($name);
            impl $name {
                pub fn is_running(&mut self) -> bool {
                    self._with_cs(|sysc| unsafe {
                        sysc.[<$name:lower cr>]().read().[<$name:lower stp>]().get() == pac::sysc::[<$name:lower cr>]::[<$names tp>]::_0
                        && sysc.oscsf().read().[<$name:lower sf>]().get() == pac::sysc::oscsf::[<$names f>]::_1
                    })
                }
                pub fn is_stopped(&mut self) -> bool {
                    self._with_cs(|sysc| unsafe {
                        sysc.[<$name:lower cr>]().read().[<$name:lower stp>]().get() == pac::sysc::[<$name:lower cr>]::[<$names tp>]::_1
                        && sysc.oscsf().read().[<$name:lower sf>]().get() == pac::sysc::oscsf::[<$names f>]::_0
                    })
                }
                pub fn run(&mut self) -> Result<(), RegisterError> {
                    if !self.is_stopped() {
                        return Err(RegisterError::NotReadyToWrite(concat!(
                            stringify!([<$name:upper>]),
                            " is not stopped."
                        )));
                    }
                    let result = self._with_prcr(|sysc| unsafe {
                        let src = sysc.[<$name:lower ccr>]().read().[<$short:lower srcsel>]().get();
                        if (src == pac::sysc::[<$name:lower ccr>]::[<$shorts rcsel>]::_0) && (sysc.mosccr().read().mostp().get() == pac::sysc::mosccr::Mostp::_1) {
                            return Err(RegisterError::NotReadyToWrite("MOSC (source) is not running."));
                        }
                        if (src == pac::sysc::[<$name:lower ccr>]::[<$shorts rcsel>]::_1) && (sysc.hococr().read().hcstp().get() == pac::sysc::hococr::Hcstp::_1) {
                            return Err(RegisterError::NotReadyToWrite("HOCO (source) is not running."));
                        }
                        sysc.[<$name:lower cr>]().write(sysc.[<$name:lower cr>]().read().[<$name:lower stp>]().set(pac::sysc::[<$name:lower cr>]::[<$names tp>]::_0));
                        Ok(())
                    });
                    result
                }
                pub fn is_stable(&mut self) -> bool {
                    self._with_cs(|sysc| unsafe {
                        sysc.oscsf().read().[<$name:lower sf>]().get() == pac::sysc::oscsf::[<$names f>]::_1
                    })
                }
                pub fn get_freq(&self) -> Option<u32> { self.freq }
                pub fn get_plidiv(&mut self) -> Option<PliDiv> {
                    self._with_cs(|sysc| unsafe { PliDiv::from_u8(sysc.[<$name:lower ccr>]().read().[<$short:lower idiv>]().get().0) })
                }
                pub fn get_source(&mut self) -> Option<PllSource> {
                    self._with_cs(|sysc| unsafe { PllSource::from_u8(sysc.[<$name:lower ccr>]().read().[<$short:lower srcsel>]().get().0) })
                }
                pub fn get_pllmul(&mut self) -> Option<PllMul> {
                    self._with_cs(|sysc| unsafe { PllMul::from_u8(sysc.[<$name:lower ccr>]().read().[<$name:lower mul>]().get()) })
                }
                pub fn set_config(&mut self, cfg: &PllConfig) -> Result<(), RegisterError> {
                    if !self.is_stopped() {
                        return Err(RegisterError::NotReadyToWrite(concat!(
                            stringify!([<$name:upper>]),
                            " is not stopped."
                        )));
                    }
                    self._with_prcr(|sysc| unsafe {
                        sysc.[<$name:lower ccr>]().write(sysc.[<$name:lower ccr>]().read()
                            .[<$short:lower idiv>]().set((cfg.div as u8).into())
                            .[<$short:lower srcsel>]().set((cfg.source as u8).into())
                            .[<$name:lower mul>]().set((cfg.mul as u8).into())
                        );
                    });
                    self.freq = Some(cfg.freq);
                    Ok(())
                }
            }
        }
    };
}
pll_impl!(Pll, Pl, Plls, Pls);
pll_impl!(Pll2, Pl2, Pll2S, Pl2S);
impl Pll {
    pub fn stop(&mut self) -> Result<(), RegisterError> {
        if !self.is_running() {
            return Err(RegisterError::NotReadyToWrite("PLL is not running."));
        }
        let result = self._with_prcr(|sysc| unsafe {
            if sysc.sckscr().read().cksel().get() == pac::sysc::sckscr::Cksel::_101 {
                return Err(RegisterError::NotReadyToWrite("PLL is selected for system clock source."));
            }
            sysc.pllcr().write(sysc.pllcr().read().pllstp().set(pac::sysc::pllcr::Pllstp::_1));
            Ok(())
        });
        result
    }
}
impl Pll2 {
    pub fn stop(&mut self) -> Result<(), RegisterError> {
        if !self.is_running() {
            return Err(RegisterError::NotReadyToWrite("PLL2 is not running."));
        }
        let result = self._with_prcr(|sysc| unsafe {
            sysc.pllcr().write(sysc.pllcr().read().pllstp().set(pac::sysc::pllcr::Pllstp::_1));
            Ok(())
        });
        result
    }
}

pub struct Mosc { freq: Option<u32> }
pub struct MoscConfig {
    freq: u32,
    freq_range: MoscFreqRange,
    source: MoscSource,
}
impl MoscConfig {
    pub fn new(freq: u32, source: MoscSource) -> Result<Self, RegisterError> {
        if let Some(freq_range) = MoscFreqRange::from_freq(freq) {
            Ok(Self { freq, freq_range, source })
        } else {
            Err(RegisterError::InvalidValue("Frequency outside the range of MOSC."))
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MoscFreqRange {
    F20_24MHz=0b00, F16_20MHz=0b01, F8_16MHz=0b10, F8MHz=0b11,
}
impl MoscFreqRange {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b00=>Some(Self::F20_24MHz), 0b01=>Some(Self::F16_20MHz), 
            0b10=>Some(Self::F8_16MHz), 0b11=>Some(Self::F8MHz), _=>None 
        }
    }
    pub fn from_freq(freq: u32) -> Option<Self> {
        if (freq < MOSC_HZ_MIN) || (freq > MOSC_HZ_MAX) {
            None
        } else if freq == MOSC_HZ_MIN {
            Some(Self::F8MHz)
        } else if freq <= 16_000_000 {
            Some(Self::F8_16MHz)
        } else if freq <= 20_000_000 {
            Some(Self::F16_20MHz)
        } else {
            Some(Self::F20_24MHz)
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MoscSource { Oscillator, ExternalClock }
impl MoscSource {
    pub fn from_u8(value: u8) -> Option<Self> { match value { 0=>Some(Self::Oscillator), 1=>Some(Self::ExternalClock), _=>None } }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MoscStabilizationTime {
    Cycle3=0x0, Cycle35=0x1, Cycle67=0x2, Cycle131=0x3, Cycle259=0x4, Cycle547=0x5,
    Cycle1059=0x6, Cycle2147=0x7, Cycle4291=0x8, Cycle8163=0x9
}
impl MoscStabilizationTime {
    pub fn cycle(self) -> u32 {
        match self {
            Self::Cycle3=>3, Self::Cycle35=>35, Self::Cycle67=>67, Self::Cycle131=>131, Self::Cycle259=>259, Self::Cycle547=>547,
            Self::Cycle1059=>1059, Self::Cycle2147=>2147, Self::Cycle4291=>4291, Self::Cycle8163=>8163
        }
    }
    pub fn us(self) -> f32 { 1.0 / (0.032768 * 8.0) * self.cycle() as f32 }
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x0 => Some(Self::Cycle3),
            0x1 => Some(Self::Cycle35),
            0x2 => Some(Self::Cycle67),
            0x3 => Some(Self::Cycle131),
            0x4 => Some(Self::Cycle259),
            0x5 => Some(Self::Cycle547),
            0x6 => Some(Self::Cycle1059),
            0x7 => Some(Self::Cycle2147),
            0x8 => Some(Self::Cycle4291),
            0x9 => Some(Self::Cycle8163),
            _ => None,
        }
    }
}
clock_impl_core!(Mosc);
impl Mosc {
    pub fn is_running(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.mosccr().read().mostp().get() == pac::sysc::mosccr::Mostp::_0
            && sysc.oscsf().read().moscsf().get() == pac::sysc::oscsf::Moscsf::_1
        })
    }
    pub fn is_stopped(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.mosccr().read().mostp().get() == pac::sysc::mosccr::Mostp::_1
            && sysc.oscsf().read().moscsf().get() == pac::sysc::oscsf::Moscsf::_0
        })
    }
    pub fn run(&mut self) -> Result<(), RegisterError> {
        if !self.is_stopped() {
            return Err(RegisterError::NotReadyToWrite("MOSC is not stopped."));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.mosccr().write(sysc.mosccr().read().mostp().set(pac::sysc::mosccr::Mostp::_0));
        });
        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), RegisterError> {
        if !self.is_running() {
            return Err(RegisterError::NotReadyToWrite("MOSC is not running."));
        }
        let result = self._with_prcr(|sysc| unsafe {
            let sysclk_src = sysc.sckscr().read().cksel().get();
            if sysclk_src == pac::sysc::sckscr::Cksel::_011 {
                return Err(RegisterError::NotReadyToWrite("MOSC is selected for system clock source."));
            }
            if sysc.pllccr().read().plsrcsel().get() == pac::sysc::pllccr::Plsrcsel::_0 {
                if sysclk_src == pac::sysc::sckscr::Cksel::_101 {
                    return Err(RegisterError::NotReadyToWrite("PLL (source: MOSC) is selected for system clock source."));
                }
                if sysc.pllcr().read().pllstp().get() == pac::sysc::pllcr::Pllstp::_0 {
                    return Err(RegisterError::NotReadyToWrite("MOSC is selected for running PLL source."));
                }
            }
            if sysc.pll2ccr().read().pl2srcsel().get() == pac::sysc::pll2ccr::Pl2Srcsel::_0
                && sysc.pll2cr().read().pll2stp().get() == pac::sysc::pll2cr::Pll2Stp::_0 {
                    return Err(RegisterError::NotReadyToWrite("MOSC is selected for running PLL2 source."));
            }
            sysc.mosccr().write(sysc.mosccr().read().mostp().set(pac::sysc::mosccr::Mostp::_1));
            Ok(())
        });
        result
    }
    pub fn is_stable(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.oscsf().read().moscsf().get() == pac::sysc::oscsf::Moscsf::_1
        })
    }
    pub fn get_freq(&self) -> Option<u32> { self.freq }
    pub fn get_freq_range(&mut self) -> Option<MoscFreqRange> {
        self._with_cs(|sysc| unsafe { MoscFreqRange::from_u8(sysc.momcr().read().modrv().get().0) })
    }
    pub fn get_source(&mut self) -> Option<MoscSource> {
        self._with_cs(|sysc| unsafe { MoscSource::from_u8(sysc.momcr().read().mosel().get().0) })
    }
    pub fn set_config(&mut self, cfg: &MoscConfig) -> Result<(), RegisterError> {
        if !self.is_stopped() {
            return Err(RegisterError::NotReadyToWrite("MOSC is not stopped."));
        } else if MoscFreqRange::from_freq(cfg.freq) != Some(cfg.freq_range) {
            return Err(RegisterError::InvalidValue("Frequency value and range do not match."));
        }
        self._with_prcr(|sysc| unsafe {
            let r = sysc.momcr().read();
            let w = r.modrv().set(match cfg.freq_range {
                MoscFreqRange::F20_24MHz => pac::sysc::momcr::Modrv::_00,
                MoscFreqRange::F16_20MHz => pac::sysc::momcr::Modrv::_01,
                MoscFreqRange::F8_16MHz => pac::sysc::momcr::Modrv::_10,
                MoscFreqRange::F8MHz => pac::sysc::momcr::Modrv::_11,
            }).mosel().set(match cfg.source {
                MoscSource::Oscillator => pac::sysc::momcr::Mosel::_0,
                MoscSource::ExternalClock => pac::sysc::momcr::Mosel::_1,
            });
            sysc.momcr().write(w);
        });
        self.freq = Some(cfg.freq);
        Ok(())
    }
    pub fn get_stabilization_time(&mut self) -> Option<MoscStabilizationTime> {
        self._with_cs(|sysc| unsafe {
            MoscStabilizationTime::from_u8(sysc.moscwtcr().read().msts().get().0)
        })
    }
    pub fn set_stabilization_time(&mut self, cycle: MoscStabilizationTime) -> Result<(), RegisterError> {
        if !self.is_stopped() {
            return Err(RegisterError::NotReadyToWrite("MOSC is not stopped."));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.moscwtcr().write(
                sysc.moscwtcr().read().msts().set((cycle as u8).into())
            );
            Ok(())
        })
    }
}

pub struct Sosc { _id: () }
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SoscMode { Standard=0, Low=1 }
impl SoscMode {
    fn from_u8(value: u8) -> Option<Self> { match value { 0=>Some(Self::Standard), 1=>Some(Self::Low), _=>None } }
}
clock_impl_core!(Sosc);
impl Sosc {
    fn _is_sosccr_0(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.sosccr().read().sostp().get() == pac::sysc::sosccr::Sostp::_0
        })
    }
    pub fn is_running(&mut self) -> bool { self._is_sosccr_0() }
    pub fn is_stopped(&mut self) -> bool { !self._is_sosccr_0() }
    pub fn run(&mut self) -> Result<(), RegisterError> {
        if self.is_running() {
            return Err(RegisterError::NotReadyToWrite("SOSC is already running."));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.sosccr().write(sysc.sosccr().read().sostp().set(
                pac::sysc::sosccr::Sostp::_0
            ));
        });
        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), RegisterError> {
        if self.is_stopped() {
            return Err(RegisterError::NotReadyToWrite("SOSC is already stopped."));
        }
        self._with_prcr(|sysc| unsafe {
            if sysc.sckscr().read().cksel().get() == pac::sysc::sckscr::Cksel::_100 {
                return Err(RegisterError::NotReadyToWrite("SOSC is selected for system clock source."));
            }
            sysc.sosccr().write(sysc.sosccr().read().sostp().set(
                pac::sysc::sosccr::Sostp::_1
            ));
            Ok(())
        })
    }
    pub fn get_mode(&mut self) -> Option<SoscMode> {
        self._with_cs(|sysc| unsafe {
            SoscMode::from_u8(sysc.somcr().read().sodrv().get().0)
        })
    }
    pub fn set_mode(&mut self, mode: SoscMode) -> Result<(), RegisterError> {
        if !self.is_stopped() {
            return Err(RegisterError::NotReadyToWrite("SOSC is not stopped."));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.somcr().write(sysc.somcr().read().sodrv().set(
                if mode == SoscMode::Standard { pac::sysc::somcr::Sodrv::_0 } else { pac::sysc::somcr::Sodrv::_1 }));
        });
        Ok(())
    }
}

pub struct Hoco { _id: () }
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HocoFreq { F16MHz=0b00, F18MHz=0b01, F20MHz=0b10 }
impl HocoFreq {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b00=>Some(Self::F16MHz), 0b01=>Some(Self::F18MHz), 0b10=>Some(Self::F20MHz), _=>None
        }
    }
    pub fn hz(self) -> u32 {
        match self { Self::F16MHz=>16_000_000, Self::F18MHz=>18_000_000, Self::F20MHz=>20_000_000 }
    }
    fn fll_cntl(self) -> u16 {
        match self { Self::F16MHz=>0x1e9, Self::F18MHz=>0x226, Self::F20MHz=>0x263 }
    }
}
clock_impl_core!(Hoco);
clock_impl_with!(Hoco, Rtc);
impl Hoco {
    /*fn _with_cs_ofs1<R>(&mut self, f: impl FnOnce(&mut pac::Sysc, &mut pac::Ofs1) -> R) -> R {
        critical_section::with(|cs| {
            let mut bor_sysc = SYSC_CELL.borrow(cs).borrow_mut();
            let sysc = bor_sysc.as_mut().expect("SYSC not initialized");
            let mut bor_ofs1 = OFS1_CELL.borrow(cs).borrow_mut();
            let ofs1 = bor_ofs1.as_mut().expect("OFS1 not initialized");

            f(sysc, ofs1)
        })
    }
    fn _with_cs_ofs1_prcr<R>(&mut self, f: impl FnOnce(&mut pac::Sysc, &mut pac::Ofs1) -> R) -> R {
        self._with_cs_rtc(|sysc, ofs1| unsafe {
            _prc0(sysc, true);
            let r = f(sysc, ofs1);
            _prc0(sysc, false);
            r
        })
    }*/
    pub fn is_running(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.hococr().read().hcstp().get() == pac::sysc::hococr::Hcstp::_0
            && sysc.oscsf().read().hocosf().get() == pac::sysc::oscsf::Hocosf::_1
        })
    }
    pub fn is_stopped(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.hococr().read().hcstp().get() == pac::sysc::hococr::Hcstp::_1
            && sysc.oscsf().read().hocosf().get() == pac::sysc::oscsf::Hocosf::_0
        })
    }
    pub fn run(&mut self) -> Result<(), RegisterError> {
        if !self.is_stopped() {
            return Err(RegisterError::NotReadyToWrite("HOCO is not stopped."));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.hococr().write(sysc.hococr().read().hcstp().set(pac::sysc::hococr::Hcstp::_0));
        });
        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), RegisterError> {
        if !self.is_running() {
            return Err(RegisterError::NotReadyToWrite("HOCO is not running."));
        }
        let result = self._with_prcr(|sysc| unsafe {
            let sysclk_src = sysc.sckscr().read().cksel().get();
            if sysclk_src == pac::sysc::sckscr::Cksel::_000 {
                return Err(RegisterError::NotReadyToWrite("HOCO is selected for system clock source."));
            }
            if sysc.pllccr().read().plsrcsel().get() == pac::sysc::pllccr::Plsrcsel::_1 {
                if sysclk_src == pac::sysc::sckscr::Cksel::_101 {
                    return Err(RegisterError::NotReadyToWrite("PLL (source: HOCO) is selected for system clock source."));
                }
                if sysc.pllcr().read().pllstp().get() == pac::sysc::pllcr::Pllstp::_0 {
                    return Err(RegisterError::NotReadyToWrite("HOCO is selected for running PLL source."));
                }
            }
            if sysc.pll2ccr().read().pl2srcsel().get() == pac::sysc::pll2ccr::Pl2Srcsel::_1
                && sysc.pll2cr().read().pll2stp().get() == pac::sysc::pll2cr::Pll2Stp::_0 {
                    return Err(RegisterError::NotReadyToWrite("HOCO is selected for running PLL2 source."));
            }
            sysc.hococr().write(sysc.hococr().read().hcstp().set(pac::sysc::hococr::Hcstp::_1));
            Ok(())
        });
        result
    }
    pub fn is_stable(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.oscsf().read().hocosf().get() == pac::sysc::oscsf::Hocosf::_1
        })
    }
    pub fn get_freq(&mut self) -> Option<HocoFreq> {
        self._with_cs(|sysc| unsafe {
            HocoFreq::from_u8(sysc.hococr2().read().hcfrq0().get().0)
        })
    } 
    pub fn set_freq(&mut self, freq: HocoFreq) -> Result<(), RegisterError> {
        if !self.is_stopped() {
            return Err(RegisterError::NotReadyToWrite("HOCO is not stopped."));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.hococr2().write(sysc.hococr2().read().hcfrq0().set(
                match freq {
                    HocoFreq::F16MHz => pac::sysc::hococr2::Hcfrq0::_00,
                    HocoFreq::F18MHz => pac::sysc::hococr2::Hcfrq0::_01,
                    HocoFreq::F20MHz => pac::sysc::hococr2::Hcfrq0::_10,
                }
            ));
            sysc.fllcr2().write(sysc.fllcr2().read().fllcntl().set(freq.fll_cntl()));
        });
        Ok(())
    }
    pub fn is_fll_enabled(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.fllcr1().read().fllen().get() == pac::sysc::fllcr1::Fllen::_1
        })
    }
    fn _set_fllcr1(&mut self, enable: bool) -> Result<(), RegisterError> {
        if !self.is_stopped() {
            return Err(RegisterError::NotReadyToWrite("HOCO is not stopped."));
        } else if self.is_fll_enabled() == enable {
            return Err(RegisterError::NotReadyToWrite(if enable { "FLL is already enabled."} else { "FLL is already disabled." }));
        }
        self._with_prcr(|sysc| unsafe {
            if enable && sysc.sosccr().read().sostp().get() == pac::sysc::sosccr::Sostp::_1 {
                Err(RegisterError::NotReadyToWrite("SOSC is not running."))
            } else {
                sysc.fllcr1().write(sysc.fllcr1().read().fllen().set(
                    if enable { pac::sysc::fllcr1::Fllen::_1 } else { pac::sysc::fllcr1::Fllen::_0 }
                ));
                Ok(())
            }
        })
    }
    pub fn enable_fll(&mut self) -> Result<(), RegisterError> { self._set_fllcr1(true) }
    pub fn disable_fll(&mut self) -> Result<(), RegisterError> { self._set_fllcr1(false) }
    pub fn get_hoco_user_trimming(&mut self) -> i8 {
        self._with_cs(|sysc| { unsafe {
            ((sysc.hocoutcr().read().hocoutrm().get() as i16 - 128 - 0x80) as u8) as i8
        } })
    }
    pub fn set_hoco_user_trimming(&mut self, trimming: i8, _rtc: &mut Rtc) -> Result<(), RegisterError> {
        self._with_prcr_rtc(|sysc, rtc| unsafe {
            if rtc.rcr2().read().start().get() == pac::rtc::rcr2::Start::_1 {
                Err(RegisterError::NotReadyToWrite("RTC is running: RCR2.START=1"))
            } else {
                let w = sysc.hocoutcr().read().hocoutrm().set((trimming as i16 + 128 + 0x80) as u8);
                sysc.hocoutcr().write(w);
                Ok(())
            }
        })
    }
}

pub struct Moco { _id: () }
clock_impl_core!(Moco);
clock_impl_with!(Moco, Rtc);
impl Moco {
    fn _is_mococr_0(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.mococr().read().mcstp().get() == pac::sysc::mococr::Mcstp::_0
        })
    }
    pub fn is_running(&mut self) -> bool { self._is_mococr_0() }
    pub fn is_stopped(&mut self) -> bool { !self._is_mococr_0() }
    pub fn run(&mut self) -> Result<(), RegisterError> {
        if self.is_running() {
            return Err(RegisterError::NotReadyToWrite("MOCO is already running."));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.mococr().write(sysc.mococr().read().mcstp().set(
                pac::sysc::mococr::Mcstp::_0
            ));
        });
        Ok(())
    }
    /// before use, wait for tMOCOWT
    pub fn stop(&mut self) -> Result<(), RegisterError> {
        if self.is_stopped() {
            return Err(RegisterError::NotReadyToWrite("MOCO is already stopped."));
        }
        self._with_prcr(|sysc| unsafe {
            if sysc.sckscr().read().cksel().get() == pac::sysc::sckscr::Cksel::_001 {
                return Err(RegisterError::NotReadyToWrite("MOCO is selected for system clock source."));
            } else if sysc.ostdcr().read().ostde().get() == pac::sysc::ostdcr::Ostde::_1 {
                return Err(RegisterError::NotReadyToWrite("Oscillation stop detection is enabled."));
            }
            sysc.mococr().write(sysc.mococr().read().mcstp().set(
                pac::sysc::mococr::Mcstp::_1
            ));
            Ok(())
        })
    }
    pub fn get_moco_user_trimming(&mut self) -> i8 {
        self._with_cs(|sysc| { unsafe {
            ((sysc.mocoutcr().read().mocoutrm().get() as i16 - 128 - 0x80) as u8) as i8
        } })
    }
    pub fn set_moco_user_trimming(&mut self, trimming: i8, _rtc: &mut Rtc) -> Result<(), RegisterError> {
        self._with_prcr_rtc(|sysc, rtc| unsafe {
            if rtc.rcr2().read().start().get() == pac::rtc::rcr2::Start::_1 {
                Err(RegisterError::NotReadyToWrite("RTC is running: RCR2.START=1"))
            } else {
                let w = sysc.mocoutcr().read().mocoutrm().set((trimming as i16 + 128 + 0x80) as u8);
                sysc.mocoutcr().write(w);
                Ok(())
            }
        })
    }
}

pub struct Loco { _id: () }
clock_impl_core!(Loco);
clock_impl_with!(Loco, Rtc);
impl Loco {
    fn _is_lococr_0(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.lococr().read().lcstp().get() == pac::sysc::lococr::Lcstp::_0
        })
    }
    pub fn is_running(&mut self) -> bool { self._is_lococr_0() }
    pub fn is_stopped(&mut self) -> bool { !self._is_lococr_0() }
    pub fn run(&mut self) -> Result<(), RegisterError> {
        if self.is_running() {
            return Err(RegisterError::NotReadyToWrite("LOCO is already running."));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.lococr().write(sysc.lococr().read().lcstp().set(
                pac::sysc::lococr::Lcstp::_0
            ));
        });
        Ok(())
    }
    /// before use, wait for tLOCOWT
    pub fn stop(&mut self) -> Result<(), RegisterError> {
        if self.is_stopped() {
            return Err(RegisterError::NotReadyToWrite("LOCO is already stopped."));
        }
        self._with_prcr(|sysc| unsafe {
            if sysc.sckscr().read().cksel().get() == pac::sysc::sckscr::Cksel::_010 {
                return Err(RegisterError::NotReadyToWrite("LOCO is selected for system clock source."));
            }
            sysc.lococr().write(sysc.lococr().read().lcstp().set(
                pac::sysc::lococr::Lcstp::_1
            ));
            Ok(())
        })
    }
    pub fn get_loco_user_trimming(&mut self) -> i8 {
        self._with_cs(|sysc| { unsafe {
            ((sysc.locoutcr().read().locoutrm().get() as i16 - 128 - 0x80) as u8) as i8
        } })
    }
    pub fn set_loco_user_trimming(&mut self, trimming: i8, _rtc: &mut Rtc) -> Result<(), RegisterError> {
        self._with_prcr_rtc(|sysc, rtc| unsafe {
            if rtc.rcr2().read().start().get() == pac::rtc::rcr2::Start::_1 {
                Err(RegisterError::NotReadyToWrite("RTC is running: RCR2.START=1"))
            } else {
                let w = sysc.locoutcr().read().locoutrm().set((trimming as i16 + 128 + 0x80) as u8);
                sysc.locoutcr().write(w);
                Ok(())
            }
        })
    } 
}

pub struct Ebclk { _id: () }
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EbclkOutput { Bclk = 0, BclkDiv2 = 1 }
impl EbclkOutput {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value { 0 => Some(Self::Bclk), 1 => Some(Self::BclkDiv2), _ => None }
    }
}
clock_impl_core!(Ebclk);
impl Ebclk {
    pub fn is_enabled(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.ebckocr().read().ebckoen().get() == pac::sysc::ebckocr::Ebckoen::_1
        })
    }
    fn _set_ebckocr(&mut self, enable: bool) -> Result<(), RegisterError> {
        if self.is_enabled() == enable {
            return Err(RegisterError::NotReadyToWrite(if enable { "EBCLK is already enabled."} else { "EBCLK is already disabled." }));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.ebckocr().write(sysc.ebckocr().read().ebckoen().set(
                if enable { pac::sysc::ebckocr::Ebckoen::_1 } else { pac::sysc::ebckocr::Ebckoen::_0 }
            ));
        });
        Ok(())
    }
    pub fn enable(&mut self) -> Result<(), RegisterError> { self._set_ebckocr(true) }
    pub fn disable(&mut self) -> Result<(), RegisterError> { self._set_ebckocr(false) }
    pub fn get_external_bus_clock(&mut self) -> Option<EbclkOutput> {
        self._with_cs(|sysc| unsafe {
            EbclkOutput::from_u8(sysc.bckcr().read().bclkdiv().get().0)
        })
    }
    pub fn set_external_bus_clock(&mut self, div: EbclkOutput, bclk_freq: u32) -> Result<(), RegisterError> {
        if (div == EbclkOutput::Bclk) && (bclk_freq > EBCLK_HZ_MAX) {
            return Err(RegisterError::InvalidValue("Frequency of EBCLK become higher than 50MHz."));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.bckcr().write(pac::sysc::Bckcr::default().bclkdiv().set((div as u8).into())); 
        });
        Ok(())
    }
}

pub struct ClockOut { _id: () }
pub struct ClockOutConfig {
    source: ClockOutSource,
    freq: u32,
    div: ClockOutDiv,
}
impl ClockOutConfig {
    pub fn new<T: ClockOutSources>(source: &mut T, div: ClockOutDiv) -> Result<Self, RegisterError> {
        let base = source.hz()?;
        let freq = base / div.value();
        Ok(Self { source: source.as_source(), freq, div })
    }
    pub fn source(&self) -> ClockOutSource { self.source }
    pub fn freq(&self) -> u32 { self.freq }
    pub fn div(&self) -> ClockOutDiv { self.div }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClockOutSource {
    Hoco=0b000, Moco=0b001, Loco=0b010, Mosc=0b011, Sosc=0b100
}
impl ClockOutSource {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b000=>Some(Self::Hoco), 0b001=>Some(Self::Moco), 0b010=>Some(Self::Loco), 
            0b011=>Some(Self::Mosc), 0b100=>Some(Self::Sosc), _=>None
        }
    }
}
pub trait ClockOutSources {
    fn hz(&mut self) -> Result<u32, RegisterError>;
    fn as_source(&self) -> ClockOutSource;
}
impl ClockOutSources for Hoco {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        if let Some(freq) = self.get_freq() { Ok(freq.hz()) }
        else { Err(RegisterError::InvalidValue("HOCO has unknown frequency.")) }
    }
    fn as_source(&self) -> ClockOutSource { ClockOutSource::Hoco }
}
impl ClockOutSources for Moco {
    fn hz(&mut self) -> Result<u32, RegisterError> { Ok(MOCO_HZ) }
    fn as_source(&self) -> ClockOutSource { ClockOutSource::Moco }
}
impl ClockOutSources for Loco {
    fn hz(&mut self) -> Result<u32, RegisterError> { Ok(LOCO_HZ) }
    fn as_source(&self) -> ClockOutSource { ClockOutSource::Loco }
}
impl ClockOutSources for Mosc {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        if let Some(freq) = self.get_freq() { Ok(freq) }
        else { Err(RegisterError::InvalidValue("MOSC is not initialized.")) }
    }
    fn as_source(&self) -> ClockOutSource { ClockOutSource::Mosc }
}
impl ClockOutSources for Sosc {
    fn hz(&mut self) -> Result<u32, RegisterError> { Ok(SOSC_HZ) }
    fn as_source(&self) -> ClockOutSource { ClockOutSource::Sosc }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClockOutDiv {
    Div1=0b000, Div2=0b001, Div4=0b010, Div8=0b011, Div16=0b100, Div32=0b101, Div64=0b110, Div128=0b111
}
impl ClockOutDiv {
    #[inline] fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b000=>Some(Self::Div1), 0b001=>Some(Self::Div2), 0b010=>Some(Self::Div4), 0b011=>Some(Self::Div8), 
            0b100=>Some(Self::Div16), 0b101=>Some(Self::Div32), 0b110=>Some(Self::Div64), 0b111=>Some(Self::Div128), _=>None
        }
    }
    #[inline] pub fn value(self) -> u32 { 1u32 << (self as u32) }
}
clock_impl_core!(ClockOut);
impl ClockOut {
    pub fn is_enabled(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.ckocr().read().ckoen().get() == pac::sysc::ckocr::Ckoen::_1
        })
    }
    fn _set_ckoen(&mut self, enable: bool) -> Result<(), RegisterError> {
        if self.is_enabled() == enable {
            return Err(RegisterError::NotReadyToWrite(if enable { "Clock out is already enabled."} else { "Clock out is already disabled." }));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.ckocr().write(sysc.ckocr().read().ckoen().set(
                if enable { pac::sysc::ckocr::Ckoen::_1 } else { pac::sysc::ckocr::Ckoen::_0 }))
        });
        Ok(())
    }
    pub fn enable(&mut self) -> Result<(), RegisterError> { self._set_ckoen(true) }
    pub fn disable(&mut self) -> Result<(), RegisterError> { self._set_ckoen(false) }
    pub fn get_source(&mut self) -> Option<ClockOutSource> {
        self._with_cs(|sysc| unsafe {
            ClockOutSource::from_u8(sysc.ckocr().read().ckosel().get().0)
        })
    }
    pub fn get_div(&mut self) -> Option<ClockOutDiv> {
        self._with_cs(|sysc| unsafe {
            ClockOutDiv::from_u8(sysc.ckocr().read().ckodiv().get().0)
        })
    }
    pub fn set_config(&mut self, cfg: ClockOutConfig) -> Result<(), RegisterError> {
        if self.is_enabled() {
            return Err(RegisterError::NotReadyToWrite("MOSC is not disabled."));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.ckocr().write(sysc.ckocr().read().ckosel().set(match cfg.source {
                ClockOutSource::Hoco => pac::sysc::ckocr::Ckosel::_000,
                ClockOutSource::Moco => pac::sysc::ckocr::Ckosel::_001,
                ClockOutSource::Loco => pac::sysc::ckocr::Ckosel::_010,
                ClockOutSource::Mosc => pac::sysc::ckocr::Ckosel::_011,
                ClockOutSource::Sosc => pac::sysc::ckocr::Ckosel::_100,
            }).ckodiv().set(match cfg.div {
                ClockOutDiv::Div1 => pac::sysc::ckocr::Ckodiv::_000,
                ClockOutDiv::Div2 => pac::sysc::ckocr::Ckodiv::_001,
                ClockOutDiv::Div4 => pac::sysc::ckocr::Ckodiv::_010,
                ClockOutDiv::Div8 => pac::sysc::ckocr::Ckodiv::_011,
                ClockOutDiv::Div16 => pac::sysc::ckocr::Ckodiv::_100,
                ClockOutDiv::Div32 => pac::sysc::ckocr::Ckodiv::_101,
                ClockOutDiv::Div64 => pac::sysc::ckocr::Ckodiv::_110,
                ClockOutDiv::Div128 => pac::sysc::ckocr::Ckodiv::_111,
            }));
        });
        Ok(())
    }
}

pub struct UsbClk { _id: () }
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UsbClkDiv { Div4=0b010, Div3=0b101, Div5=0b110 }
impl UsbClkDiv {
    #[inline] fn from_u8(value: u8) -> Option<Self> {
        match value { 0b010=>Some(Self::Div4), 0b101=>Some(Self::Div3), 0b110=>Some(Self::Div5), _=>None }
    }
    #[inline] pub fn value(self) -> u32 {
        match self { Self::Div4=>4, Self::Div3=>3, Self::Div5=>5 }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UsbClkSource { Pll=0b101, Pll2=0b110 }
impl UsbClkSource {
    fn from_u8(value: u8) -> Option<Self> { match value { 0b101=>Some(Self::Pll), 0b110=>Some(Self::Pll2), _=>None } }
}
pub trait UsbClkSources {
    fn hz(&mut self) -> Result<u32, RegisterError>;
    fn as_source(&self) -> UsbClkSource;
}
impl UsbClkSources for Pll {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        match self.freq {
            Some(freq) => Ok(freq), 
            None => Err(RegisterError::InvalidValue("PLL is not initialized."))
        }
    }
    fn as_source(&self) -> UsbClkSource { UsbClkSource::Pll }
}
impl UsbClkSources for Pll2 {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        match self.freq {
            Some(freq) => Ok(freq), 
            None => Err(RegisterError::InvalidValue("PLL2 is not initialized."))
        }
    }
    fn as_source(&self) -> UsbClkSource { UsbClkSource::Pll2 }
}
pub struct UsbClkConfig {
    source: UsbClkSource,
    div: UsbClkDiv
}
impl UsbClkConfig {
    pub fn new<T: UsbClkSources>(source: &mut T, div: UsbClkDiv) -> Result<Self, RegisterError> {
        let freq = source.hz()? / div.value();
        if freq == USBCLK_HZ {
            Ok(Self { source: source.as_source(), div })
        } else {
            Err(RegisterError::InvalidValue("USBCLK frequency must be 48MHz."))
        }
    }
    pub fn source(&self) -> UsbClkSource { self.source }
    pub fn div(&self) -> UsbClkDiv { self.div }
}
clock_impl_core!(UsbClk);
impl UsbClk {
    pub fn is_available(&mut self) -> bool {
        match (self.get_source(), self.get_div()) {
            (Some(_), Some(_)) => true,
            _ => false
        }
    }
    pub fn get_source(&mut self) -> Option<UsbClkSource> {
        self._with_cs(|sysc| unsafe {
            UsbClkSource::from_u8(sysc.usbckcr().read().usbcksel().get().0)
        })
    }
    pub fn get_div(&mut self) -> Option<UsbClkDiv> {
        self._with_cs(|sysc| unsafe {
            UsbClkDiv::from_u8(sysc.usbckdivcr().read().usbckdiv().get().0)
        })
    }
    pub fn set_config(&mut self, cfg: UsbClkConfig) -> Result<(), RegisterError> {
        self._with_prcr(|sysc| unsafe {
            sysc.usbckcr().write(sysc.usbckcr().read().usbcksreq().set(pac::sysc::usbckcr::Usbcksreq::_1));
            while sysc.usbckcr().read().usbcksrdy().get() == pac::sysc::usbckcr::Usbcksrdy::_0 {}
            sysc.usbckdivcr().write(sysc.usbckdivcr().read().usbckdiv().set(
                match cfg.div {
                    UsbClkDiv::Div4 => pac::sysc::usbckdivcr::Usbckdiv::_010,
                    UsbClkDiv::Div3 => pac::sysc::usbckdivcr::Usbckdiv::_101,
                    UsbClkDiv::Div5 => pac::sysc::usbckdivcr::Usbckdiv::_110,
                }
            ));
            sysc.usbckcr().write(sysc.usbckcr().read().usbcksel().set(
                match cfg.source {
                    UsbClkSource::Pll => pac::sysc::usbckcr::Usbcksel::_101,
                    UsbClkSource::Pll2 => pac::sysc::usbckcr::Usbcksel::_110,
                }
            ));
            sysc.usbckcr().write(sysc.usbckcr().read().usbcksreq().set(pac::sysc::usbckcr::Usbcksreq::_0));
            while sysc.usbckcr().read().usbcksrdy().get() == pac::sysc::usbckcr::Usbcksrdy::_1 {}
        });
        Ok(())
    }
}

pub struct OctaClk { _id: () }
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OctaClkDiv { Div1=0b000, Div2=0b001, Div4=0b010, Div6=0b011, Div8=0b100 }
impl OctaClkDiv {
    #[inline] fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b000=>Some(Self::Div1), 0b001=>Some(Self::Div2), 0b010=>Some(Self::Div4), 
            0b011=>Some(Self::Div6), 0b100=>Some(Self::Div8), _=>None
        }
    }
    #[inline] pub fn value(self) -> u32 { match self { Self::Div1=>1, div=>div as u32 * 2 } }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OctaClkSource { Hoco=0b000, Moco=0b001, Loco=0b010, Mosc=0b011, Sosc=0b100, Pll=0b101, Pll2=0b110 }
impl OctaClkSource {
    fn from_u8(value: u8) -> Option<Self> { match value { 0b101=>Some(Self::Pll), 0b110=>Some(Self::Pll2), _=>None } }
}
pub trait OctaClkSources {
    fn hz(&mut self) -> Result<u32, RegisterError>;
    fn as_source(&self) -> OctaClkSource;
}
impl OctaClkSources for Hoco {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        if let Some(freq) = self.get_freq() { Ok(freq.hz()) }
        else { Err(RegisterError::InvalidValue("HOCO has unknown frequency.")) }
    }
    fn as_source(&self) -> OctaClkSource { OctaClkSource::Hoco }
}
impl OctaClkSources for Moco {
    fn hz(&mut self) -> Result<u32, RegisterError> { Ok(MOCO_HZ) }
    fn as_source(&self) -> OctaClkSource { OctaClkSource::Moco }
}
impl OctaClkSources for Loco {
    fn hz(&mut self) -> Result<u32, RegisterError> { Ok(LOCO_HZ) }
    fn as_source(&self) -> OctaClkSource { OctaClkSource::Loco }
}
impl OctaClkSources for Mosc {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        match self.freq {
            Some(freq) => Ok(freq), 
            None => Err(RegisterError::InvalidValue("MOSC is not initialized."))
        }
    }
    fn as_source(&self) -> OctaClkSource { OctaClkSource::Mosc }
}
impl OctaClkSources for Sosc {
    fn hz(&mut self) -> Result<u32, RegisterError> { Ok(SOSC_HZ) }
    fn as_source(&self) -> OctaClkSource { OctaClkSource::Sosc }
}
impl OctaClkSources for Pll {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        match self.freq {
            Some(freq) => Ok(freq), 
            None => Err(RegisterError::InvalidValue("PLL is not initialized."))
        }
    }
    fn as_source(&self) -> OctaClkSource { OctaClkSource::Pll }
}
impl OctaClkSources for Pll2 {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        match self.freq {
            Some(freq) => Ok(freq), 
            None => Err(RegisterError::InvalidValue("PLL2 is not initialized."))
        }
    }
    fn as_source(&self) -> OctaClkSource { OctaClkSource::Pll2 }
}
pub struct OctaClkConfig {
    freq: u32,
    source: OctaClkSource,
    div: OctaClkDiv
}
impl OctaClkConfig {
    pub fn new<T: OctaClkSources>(source: &mut T, div: OctaClkDiv) -> Result<Self, RegisterError> {
        let freq = source.hz()? / div.value();
        if freq > OCTACLK_HZ_MAX {
            Ok(Self { freq, source: source.as_source(), div })
        } else {
            Err(RegisterError::InvalidValue("Max OCTACLK frequency is 200MHz."))
        }
    }
    pub fn freq(&self) -> u32 { self.freq }
    pub fn source(&self) -> OctaClkSource { self.source }
    pub fn div(&self) -> OctaClkDiv { self.div }
}
clock_impl_core!(OctaClk);
impl OctaClk {
    pub fn is_available(&mut self) -> bool {
        match (self.get_source(), self.get_div()) {
            (Some(_), Some(_)) => true,
            _ => false
        }
    }
    pub fn get_source(&mut self) -> Option<OctaClkSource> {
        self._with_cs(|sysc| unsafe {
            OctaClkSource::from_u8(sysc.octackcr().read().octacksel().get().0)
        })
    }
    pub fn get_div(&mut self) -> Option<OctaClkDiv> {
        self._with_cs(|sysc| unsafe {
            OctaClkDiv::from_u8(sysc.octackdivcr().read().octackdiv().get().0)
        })
    }
    pub fn set_config(&mut self, cfg: OctaClkConfig) -> Result<(), RegisterError> {
        self._with_prcr(|sysc| unsafe {
            sysc.octackcr().write(sysc.octackcr().read().octacksreq().set(pac::sysc::octackcr::Octacksreq::_1));
            while sysc.octackcr().read().octacksrdy().get() == pac::sysc::octackcr::Octacksrdy::_0 {}
            sysc.octackdivcr().write(sysc.octackdivcr().read().octackdiv().set(
                match cfg.div {
                    OctaClkDiv::Div1 => pac::sysc::octackdivcr::Octackdiv::_000,
                    OctaClkDiv::Div2 => pac::sysc::octackdivcr::Octackdiv::_001,
                    OctaClkDiv::Div4 => pac::sysc::octackdivcr::Octackdiv::_010,
                    OctaClkDiv::Div6 => pac::sysc::octackdivcr::Octackdiv::_011,
                    OctaClkDiv::Div8 => pac::sysc::octackdivcr::Octackdiv::_100,
                }
            ));
            sysc.octackcr().write(sysc.octackcr().read().octacksel().set(
                match cfg.source {
                    OctaClkSource::Hoco => pac::sysc::octackcr::Octacksel::_000,
                    OctaClkSource::Moco => pac::sysc::octackcr::Octacksel::_001,
                    OctaClkSource::Loco => pac::sysc::octackcr::Octacksel::_010,
                    OctaClkSource::Mosc => pac::sysc::octackcr::Octacksel::_011,
                    OctaClkSource::Sosc => pac::sysc::octackcr::Octacksel::_100,
                    OctaClkSource::Pll => pac::sysc::octackcr::Octacksel::_101,
                    OctaClkSource::Pll2 => pac::sysc::octackcr::Octacksel::_110,
                }
            ));
            sysc.octackcr().write(sysc.octackcr().read().octacksreq().set(pac::sysc::octackcr::Octacksreq::_0));
            while sysc.octackcr().read().octacksrdy().get() == pac::sysc::octackcr::Octacksrdy::_1 {}
        });
        Ok(())
    }
}

pub struct CanfdClk { _id: () }
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CanfdClkDiv { Div1=0b000, Div2=0b001, Div4=0b010, Div6=0b011 }
impl CanfdClkDiv {
    #[inline] fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b000=>Some(Self::Div1), 0b001=>Some(Self::Div2), 0b010=>Some(Self::Div4), 
            0b011=>Some(Self::Div6), _=>None
        }
    }
    #[inline] pub fn value(self) -> u32 { match self { Self::Div1=>1, div=>div as u32 * 2 } }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CanfdClkSource { Pll=0b101, Pll2=0b110 }
impl CanfdClkSource {
    fn from_u8(value: u8) -> Option<Self> { match value { 0b101=>Some(Self::Pll), 0b110=>Some(Self::Pll2), _=>None } }
}
pub trait CanfdClkSources {
    fn hz(&mut self) -> Result<u32, RegisterError>;
    fn as_source(&self) -> CanfdClkSource;
}
impl CanfdClkSources for Pll {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        match self.freq {
            Some(freq) => Ok(freq), 
            None => Err(RegisterError::InvalidValue("PLL is not initialized."))
        }
    }
    fn as_source(&self) -> CanfdClkSource { CanfdClkSource::Pll }
}
impl CanfdClkSources for Pll2 {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        match self.freq {
            Some(freq) => Ok(freq), 
            None => Err(RegisterError::InvalidValue("PLL2 is not initialized."))
        }
    }
    fn as_source(&self) -> CanfdClkSource { CanfdClkSource::Pll2 }
}
pub struct CanfdClkConfig {
    freq: u32,
    source: CanfdClkSource,
    div: CanfdClkDiv
}
impl CanfdClkConfig {
    pub fn new<T: CanfdClkSources>(source: &mut T, div: CanfdClkDiv) -> Result<Self, RegisterError> {
        let freq = source.hz()? / div.value();
        if freq > CANFDCLK_HZ_MAX {
            Ok(Self { freq, source: source.as_source(), div })
        } else {
            Err(RegisterError::InvalidValue("Max CANFDCLK frequency is 40MHz."))
        }
    }
    pub fn freq(&self) -> u32 { self.freq }
    pub fn source(&self) -> CanfdClkSource { self.source }
    pub fn div(&self) -> CanfdClkDiv { self.div }
}
clock_impl_core!(CanfdClk);
impl CanfdClk {
    pub fn is_available(&mut self) -> bool {
        match (self.get_source(), self.get_div()) {
            (Some(_), Some(_)) => true,
            _ => false
        }
    }
    pub fn get_source(&mut self) -> Option<CanfdClkSource> {
        self._with_cs(|sysc| unsafe {
            CanfdClkSource::from_u8(sysc.canfdckcr().read().canfdcksel().get().0)
        })
    }
    pub fn get_div(&mut self) -> Option<CanfdClkDiv> {
        self._with_cs(|sysc| unsafe {
            CanfdClkDiv::from_u8(sysc.canfdckdivcr().read().canfdckdiv().get().0)
        })
    }
    pub fn set_config(&mut self, cfg: CanfdClkConfig) -> Result<(), RegisterError> {
        self._with_prcr(|sysc| unsafe {
            sysc.canfdckcr().write(sysc.canfdckcr().read().canfdcksreq().set(pac::sysc::canfdckcr::Canfdcksreq::_1));
            while sysc.canfdckcr().read().canfdcksrdy().get() == pac::sysc::canfdckcr::Canfdcksrdy::_0 {}
            sysc.canfdckdivcr().write(sysc.canfdckdivcr().read().set_raw((cfg.div as u8).into()));
            sysc.canfdckcr().write(sysc.canfdckcr().read().canfdcksel().set(
                match cfg.source {
                    CanfdClkSource::Pll => pac::sysc::canfdckcr::Canfdcksel::_101,
                    CanfdClkSource::Pll2 => pac::sysc::canfdckcr::Canfdcksel::_110,
                }
            ));
            sysc.canfdckcr().write(sysc.canfdckcr().read().canfdcksreq().set(pac::sysc::canfdckcr::Canfdcksreq::_0));
            while sysc.canfdckcr().read().canfdcksrdy().get() == pac::sysc::canfdckcr::Canfdcksrdy::_1 {}
        });
        Ok(())
    }
}

pub struct Usb60Clk { _id: () }
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Usb60ClkDiv { Div1=0b000, Div2=0b001, Div4=0b010, Div6=0b011, Div8=0b100, Div3=0b101, Div5=0b110 }
impl Usb60ClkDiv {
    #[inline] fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b000=>Some(Self::Div1), 0b001=>Some(Self::Div2), 0b010=>Some(Self::Div4), 
            0b011=>Some(Self::Div6), 0b100=>Some(Self::Div8), 0b101=>Some(Self::Div3),
            0b110=>Some(Self::Div5), _=>None
        }
    }
    #[inline] pub fn value(self) -> u32 {
        match self {
            Self::Div1=>1,
            Self::Div3=>3,
            Self::Div5=>5,
            div=>div as u32 * 2 
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Usb60ClkSource { Pll=0b101, Pll2=0b110 }
impl Usb60ClkSource {
    fn from_u8(value: u8) -> Option<Self> { match value { 0b101=>Some(Self::Pll), 0b110=>Some(Self::Pll2), _=>None } }
}
pub trait Usb60ClkSources {
    fn hz(&mut self) -> Result<u32, RegisterError>;
    fn as_source(&self) -> Usb60ClkSource;
}
impl Usb60ClkSources for Pll {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        match self.freq {
            Some(freq) => Ok(freq), 
            None => Err(RegisterError::InvalidValue("PLL is not initialized."))
        }
    }
    fn as_source(&self) -> Usb60ClkSource { Usb60ClkSource::Pll }
}
impl Usb60ClkSources for Pll2 {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        match self.freq {
            Some(freq) => Ok(freq), 
            None => Err(RegisterError::InvalidValue("PLL2 is not initialized."))
        }
    }
    fn as_source(&self) -> Usb60ClkSource { Usb60ClkSource::Pll2 }
}
pub struct Usb60ClkConfig {
    source: Usb60ClkSource,
    div: Usb60ClkDiv
}
impl Usb60ClkConfig {
    pub fn new<T: Usb60ClkSources>(source: &mut T, div: Usb60ClkDiv) -> Result<Self, RegisterError> {
        let freq = source.hz()? / div.value();
        if freq == USB60CLK_HZ {
            Ok(Self { source: source.as_source(), div })
        } else {
            Err(RegisterError::InvalidValue("USB60CLK frequency must be 60MHz."))
        }
    }
    pub fn source(&self) -> Usb60ClkSource { self.source }
    pub fn div(&self) -> Usb60ClkDiv { self.div }
}
clock_impl_core!(Usb60Clk);
impl Usb60Clk {
    pub fn is_available(&mut self) -> bool {
        match (self.get_source(), self.get_div()) {
            (Some(_), Some(_)) => true,
            _ => false
        }
    }
    pub fn get_source(&mut self) -> Option<Usb60ClkSource> {
        self._with_cs(|sysc| unsafe {
            Usb60ClkSource::from_u8(sysc.usb60ckcr().read().usb60cksel().get().0)
        })
    }
    pub fn get_div(&mut self) -> Option<Usb60ClkDiv> {
        self._with_cs(|sysc| unsafe {
            Usb60ClkDiv::from_u8(sysc.usb60ckdivcr().read().usb60ckdiv().get().0)
        })
    }
    pub fn set_config(&mut self, cfg: Usb60ClkConfig) -> Result<(), RegisterError> {
        self._with_prcr(|sysc| unsafe {
            sysc.usb60ckcr().write(sysc.usb60ckcr().read().usb60cksreq().set(pac::sysc::usb60ckcr::Usb60Cksreq::_1));
            while sysc.usb60ckcr().read().usb60cksrdy().get() == pac::sysc::usb60ckcr::Usb60Cksrdy::_0 {}
            sysc.usb60ckdivcr().write(sysc.usb60ckdivcr().read().usb60ckdiv().set(
                match cfg.div {
                    Usb60ClkDiv::Div1 => pac::sysc::usb60ckdivcr::Usb60Ckdiv::_000,
                    Usb60ClkDiv::Div2 => pac::sysc::usb60ckdivcr::Usb60Ckdiv::_001,
                    Usb60ClkDiv::Div4 => pac::sysc::usb60ckdivcr::Usb60Ckdiv::_010,
                    Usb60ClkDiv::Div6 => pac::sysc::usb60ckdivcr::Usb60Ckdiv::_011,
                    Usb60ClkDiv::Div8 => pac::sysc::usb60ckdivcr::Usb60Ckdiv::_100,
                    Usb60ClkDiv::Div3 => pac::sysc::usb60ckdivcr::Usb60Ckdiv::_101,
                    Usb60ClkDiv::Div5 => pac::sysc::usb60ckdivcr::Usb60Ckdiv::_110,
                }
            ));
            sysc.usb60ckcr().write(sysc.usb60ckcr().read().usb60cksel().set(
                match cfg.source {
                    Usb60ClkSource::Pll => pac::sysc::usb60ckcr::Usb60Cksel::_101,
                    Usb60ClkSource::Pll2 => pac::sysc::usb60ckcr::Usb60Cksel::_110,
                }
            ));
            sysc.usb60ckcr().write(sysc.usb60ckcr().read().usb60cksreq().set(pac::sysc::usb60ckcr::Usb60Cksreq::_0));
            while sysc.usb60ckcr().read().usb60cksrdy().get() == pac::sysc::usb60ckcr::Usb60Cksrdy::_1 {}
        });
        Ok(())
    }
}

pub struct CecClk { _id: () }
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CecClkDiv { Div1=0b000, Div2=0b001, Div4=0b010, Div6=0b011, Div8=0b100, Div3=0b101, Div5=0b110 }
impl CecClkDiv {
    #[inline] fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b000=>Some(Self::Div1), 0b001=>Some(Self::Div2), 0b010=>Some(Self::Div4), 
            0b011=>Some(Self::Div6), 0b100=>Some(Self::Div8), 0b101=>Some(Self::Div3),
            0b110=>Some(Self::Div5), _=>None
        }
    }
    #[inline] pub fn value(self) -> u32 {
        match self {
            Self::Div1=>1,
            Self::Div3=>3,
            Self::Div5=>5,
            div=>div as u32 * 2 
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CecClkSource { Mosc=0b011, Sosc=0b100 }
impl CecClkSource {
    fn from_u8(value: u8) -> Option<Self> { match value { 0b011=>Some(Self::Mosc), 0b100=>Some(Self::Sosc), _=>None } }
}
pub trait CecClkSources {
    fn hz(&mut self) -> Result<u32, RegisterError>;
    fn as_source(&self) -> CecClkSource;
}
impl CecClkSources for Mosc {
    fn hz(&mut self) -> Result<u32, RegisterError> {
        match self.freq {
            Some(freq) => Ok(freq), 
            None => Err(RegisterError::InvalidValue("MOSC is not initialized."))
        }
    }
    fn as_source(&self) -> CecClkSource { CecClkSource::Mosc }
}
impl CecClkSources for Sosc {
    fn hz(&mut self) -> Result<u32, RegisterError> { Ok(SOSC_HZ) }
    fn as_source(&self) -> CecClkSource { CecClkSource::Sosc }
}
pub struct CecClkConfig {
    freq: u32,
    source: CecClkSource,
    div: CecClkDiv
}
impl CecClkConfig {
    pub fn new<T: CecClkSources>(source: &mut T, div: CecClkDiv) -> Result<Self, RegisterError> {
        let freq = source.hz()? / div.value();
        if freq > CECCLK_HZ_MAX {
            Ok(Self { freq, source: source.as_source(), div })
        } else {
            Err(RegisterError::InvalidValue("Max CECCLK frequency is 20MHz."))
        }
    }
    pub fn freq(&self) -> u32 { self.freq }
    pub fn source(&self) -> CecClkSource { self.source }
    pub fn div(&self) -> CecClkDiv { self.div }
}
clock_impl_core!(CecClk);
impl CecClk {
    pub fn is_available(&mut self) -> bool {
        match (self.get_source(), self.get_div()) {
            (Some(_), Some(_)) => true,
            _ => false
        }
    }
    pub fn get_source(&mut self) -> Option<CecClkSource> {
        self._with_cs(|sysc| unsafe {
            CecClkSource::from_u8(sysc.cecckcr().read().ceccksel().get().0)
        })
    }
    pub fn get_div(&mut self) -> Option<CecClkDiv> {
        self._with_cs(|sysc| unsafe {
            CecClkDiv::from_u8(sysc.cecckdivcr().read().cecckdiv().get().0)
        })
    }
    pub fn set_config(&mut self, cfg: CecClkConfig) -> Result<(), RegisterError> {
        self._with_prcr(|sysc| unsafe {
            sysc.cecckcr().write(sysc.cecckcr().read().ceccksreq().set(pac::sysc::cecckcr::Ceccksreq::_1));
            while sysc.cecckcr().read().ceccksrdy().get() == pac::sysc::cecckcr::Ceccksrdy::_0 {}
            sysc.cecckdivcr().write(sysc.cecckdivcr().read().set_raw((cfg.div as u8).into()));
            sysc.cecckcr().write(sysc.cecckcr().read().ceccksel().set(
                match cfg.source {
                    CecClkSource::Mosc => pac::sysc::cecckcr::Ceccksel::_011,
                    CecClkSource::Sosc => pac::sysc::cecckcr::Ceccksel::_100,
                }
            ));
            sysc.cecckcr().write(sysc.cecckcr().read().ceccksreq().set(pac::sysc::cecckcr::Ceccksreq::_0));
            while sysc.cecckcr().read().ceccksrdy().get() == pac::sysc::cecckcr::Ceccksrdy::_1 {}
        });
        Ok(())
    }
}

pub struct TrClk { _id: () }
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrClkDiv { Div1=0x0, Div2=0x1, Div4=0x2 }
impl TrClkDiv {
    #[inline] fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x0=>Some(Self::Div1), 0x1=>Some(Self::Div2), 0x2=>Some(Self::Div4), _=>None
        }
    }
    #[inline] pub fn value(self) -> u32 { 1 << (self as u32) }
}
clock_impl_core!(TrClk);
clock_impl_with!(TrClk, Dbg);
impl TrClk {
    pub fn is_enabled(&mut self) -> bool {
        self._with_cs(|sysc| unsafe {
            sysc.trckcr().read().trcken().get() == pac::sysc::trckcr::Trcken::_1
        })
    }
    pub fn is_debugger_connected(&mut self, _dbg: &mut Dbg) -> bool {
        self._with_cs_dbg(|_, dbg| unsafe {
            dbg.dbgstr().read().cdbgpwrupreq().get() == pac::dbg::dbgstr::Cdbgpwrupreq::_1
        })
    }
    fn _set_trcken(&mut self, enable: bool, _dbg: &mut Dbg) -> Result<(), RegisterError> {
        if !self.is_debugger_connected(_dbg) {
            return Err(RegisterError::NotReadyToWrite("Debugger is not connected."));
        } else if self.is_enabled() == enable {
            return Err(RegisterError::NotReadyToWrite(if enable { "TRCLK is already enabled."} else { "TRCLK is already disabled." }));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.trckcr().write(sysc.trckcr().read().trcken().set(
                if enable { pac::sysc::trckcr::Trcken::_1 } else { pac::sysc::trckcr::Trcken::_0 }
            ))
        });
        Ok(())
    }
    pub fn enable(&mut self, _dbg: &mut Dbg) -> Result<(), RegisterError> { self._set_trcken(true, _dbg) }
    pub fn disable(&mut self, _dbg: &mut Dbg) -> Result<(), RegisterError> { self._set_trcken(false, _dbg) }
    pub fn get_div(&mut self) -> Option<TrClkDiv> {
        self._with_cs(|sysc| unsafe {
            TrClkDiv::from_u8(sysc.trckcr().read().trck().get().0)
        })
    }
    pub fn set_div(&mut self, div: TrClkDiv, _dbg: &mut Dbg) -> Result<(), RegisterError> {
        if !self.is_debugger_connected(_dbg) {
            return Err(RegisterError::NotReadyToWrite("Debugger is not connected."));
        } else if self.is_enabled() {
            return Err(RegisterError::NotReadyToWrite("TRCLK is not disabled." ));
        }
        self._with_prcr(|sysc| unsafe {
            sysc.trckcr().write(sysc.trckcr().read().trck().set(
                match div {
                    TrClkDiv::Div1 => pac::sysc::trckcr::Trck::_0_X_0,
                    TrClkDiv::Div2 => pac::sysc::trckcr::Trck::_0_X_1,
                    TrClkDiv::Div4 => pac::sysc::trckcr::Trck::_0_X_2,
                }
            ));
        });
        Ok(())
    }
}