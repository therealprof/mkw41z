#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LPM_SDM_RES2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct LPM_DENOM_SELECTEDR {
    bits: u32,
}
impl LPM_DENOM_SELECTEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:27 - Low Port Modulation Denominator Selected"]
    #[inline]
    pub fn lpm_denom_selected(&self) -> LPM_DENOM_SELECTEDR {
        let bits = {
            const MASK: u32 = 268435455;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        LPM_DENOM_SELECTEDR { bits }
    }
}
