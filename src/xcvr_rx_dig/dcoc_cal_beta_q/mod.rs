#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DCOC_CAL_BETA_Q {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_CAL_BETA_QR {
    bits: u32,
}
impl DCOC_CAL_BETA_QR {
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
    #[doc = "Bits 0:16 - DCOC_CAL_BETA_Q"]
    #[inline]
    pub fn dcoc_cal_beta_q(&self) -> DCOC_CAL_BETA_QR {
        let bits = {
            const MASK: u32 = 131071;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DCOC_CAL_BETA_QR { bits }
    }
}
