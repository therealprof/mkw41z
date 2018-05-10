#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CTUNE_CNT6 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CTUNE_COUNT_6R {
    bits: u16,
}
impl CTUNE_COUNT_6R {
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
    #[doc = "Bits 0:12 - CTUNE Count 6"]
    #[inline]
    pub fn ctune_count_6(&self) -> CTUNE_COUNT_6R {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CTUNE_COUNT_6R { bits }
    }
}
