#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AUXPLL_FCAL_CNT5_4 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FCAL_COUNT_4R {
    bits: u16,
}
impl FCAL_COUNT_4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FCAL_COUNT_5R {
    bits: u16,
}
impl FCAL_COUNT_5R {
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
    #[doc = "Bits 0:9 - Aux PLL Frequency Calibration Count 4"]
    #[inline]
    pub fn fcal_count_4(&self) -> FCAL_COUNT_4R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FCAL_COUNT_4R { bits }
    }
    #[doc = "Bits 16:25 - Aux PLL Frequency Calibration Count 5"]
    #[inline]
    pub fn fcal_count_5(&self) -> FCAL_COUNT_5R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FCAL_COUNT_5R { bits }
    }
}
