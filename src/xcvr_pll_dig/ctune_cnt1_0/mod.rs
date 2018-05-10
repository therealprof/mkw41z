#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CTUNE_CNT1_0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CTUNE_COUNT_0R {
    bits: u16,
}
impl CTUNE_COUNT_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTUNE_COUNT_1R {
    bits: u16,
}
impl CTUNE_COUNT_1R {
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
    #[doc = "Bits 0:12 - CTUNE Count 0"]
    #[inline]
    pub fn ctune_count_0(&self) -> CTUNE_COUNT_0R {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CTUNE_COUNT_0R { bits }
    }
    #[doc = "Bits 16:28 - CTUNE Count 1"]
    #[inline]
    pub fn ctune_count_1(&self) -> CTUNE_COUNT_1R {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CTUNE_COUNT_1R { bits }
    }
}
