#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AUXPLL_FCAL_CNT6 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FCAL_COUNT_6R {
    bits: u16,
}
impl FCAL_COUNT_6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FCAL_BESTDIFFR {
    bits: u16,
}
impl FCAL_BESTDIFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - Aux PLL Frequency Calibration Count 6"]
    #[inline]
    pub fn fcal_count_6(&self) -> FCAL_COUNT_6R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FCAL_COUNT_6R { bits }
    }
    #[doc = "Bits 16:25 - Aux PLL Frequency Calibration Best Difference"]
    #[inline]
    pub fn fcal_bestdiff(&self) -> FCAL_BESTDIFFR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FCAL_BESTDIFFR { bits }
    }
}
