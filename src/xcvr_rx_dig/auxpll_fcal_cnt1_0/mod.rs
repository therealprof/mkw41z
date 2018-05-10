#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AUXPLL_FCAL_CNT1_0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FCAL_COUNT_0R {
    bits: u16,
}
impl FCAL_COUNT_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FCAL_COUNT_1R {
    bits: u16,
}
impl FCAL_COUNT_1R {
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
    #[doc = "Bits 0:9 - Frequency Calibration Count 0"]
    #[inline]
    pub fn fcal_count_0(&self) -> FCAL_COUNT_0R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FCAL_COUNT_0R { bits }
    }
    #[doc = "Bits 16:25 - Frequency Calibration Count 1"]
    #[inline]
    pub fn fcal_count_1(&self) -> FCAL_COUNT_1R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FCAL_COUNT_1R { bits }
    }
}
