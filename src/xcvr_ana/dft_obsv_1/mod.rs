#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DFT_OBSV_1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DFT_FREQ_COUNTERR {
    bits: u32,
}
impl DFT_FREQ_COUNTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTUNE_MAX_DIFFR {
    bits: u8,
}
impl CTUNE_MAX_DIFFR {
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
    #[doc = "Bits 0:18 - VCO Frequency Counter Value"]
    #[inline]
    pub fn dft_freq_counter(&self) -> DFT_FREQ_COUNTERR {
        let bits = {
            const MASK: u32 = 524287;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DFT_FREQ_COUNTERR { bits }
    }
    #[doc = "Bits 20:27 - Maximum Frequency Count Difference found by the Coarse Tune BIST"]
    #[inline]
    pub fn ctune_max_diff(&self) -> CTUNE_MAX_DIFFR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTUNE_MAX_DIFFR { bits }
    }
}
