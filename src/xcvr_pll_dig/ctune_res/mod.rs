#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CTUNE_RES {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CTUNE_SELECTEDR {
    bits: u8,
}
impl CTUNE_SELECTEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTUNE_BEST_DIFFR {
    bits: u8,
}
impl CTUNE_BEST_DIFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTUNE_FREQ_SELECTEDR {
    bits: u16,
}
impl CTUNE_FREQ_SELECTEDR {
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
    #[doc = "Bits 0:6 - Coarse Tune Setting to VCO"]
    #[inline]
    pub fn ctune_selected(&self) -> CTUNE_SELECTEDR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTUNE_SELECTEDR { bits }
    }
    #[doc = "Bits 8:15 - Coarse Tune Absolute Best Difference"]
    #[inline]
    pub fn ctune_best_diff(&self) -> CTUNE_BEST_DIFFR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTUNE_BEST_DIFFR { bits }
    }
    #[doc = "Bits 16:27 - Coarse Tune Frequency Selected"]
    #[inline]
    pub fn ctune_freq_selected(&self) -> CTUNE_FREQ_SELECTEDR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CTUNE_FREQ_SELECTEDR { bits }
    }
}
