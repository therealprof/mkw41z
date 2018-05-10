#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LQI_AND_RSSI {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct LQI_VALUER {
    bits: u8,
}
impl LQI_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSSIR {
    bits: u8,
}
impl RSSIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CCA1_ED_FNLR {
    bits: u8,
}
impl CCA1_ED_FNLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - LQI Value"]
    #[inline]
    pub fn lqi_value(&self) -> LQI_VALUER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LQI_VALUER { bits }
    }
    #[doc = "Bits 8:15 - RSSI Value"]
    #[inline]
    pub fn rssi(&self) -> RSSIR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSSIR { bits }
    }
    #[doc = "Bits 16:23 - Final Result for CCA Mode 1 and Energy Detect"]
    #[inline]
    pub fn cca1_ed_fnl(&self) -> CCA1_ED_FNLR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCA1_ED_FNLR { bits }
    }
}
