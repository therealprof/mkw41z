#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DCOC_CAL_BETA_I {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_CAL_BETA_IR {
    bits: u32,
}
impl DCOC_CAL_BETA_IR {
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
    #[doc = "Bits 0:16 - DCOC_CAL_BETA_I"]
    #[inline]
    pub fn dcoc_cal_beta_i(&self) -> DCOC_CAL_BETA_IR {
        let bits = {
            const MASK: u32 = 131071;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DCOC_CAL_BETA_IR { bits }
    }
}
