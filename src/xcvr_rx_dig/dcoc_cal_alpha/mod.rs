#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DCOC_CAL_ALPHA {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_CAL_ALPHA_IR {
    bits: u16,
}
impl DCOC_CAL_ALPHA_IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_CAL_ALPHA_QR {
    bits: u16,
}
impl DCOC_CAL_ALPHA_QR {
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
    #[doc = "Bits 0:10 - DCOC Calibration I-channel ALPHA constant"]
    #[inline]
    pub fn dcoc_cal_alpha_i(&self) -> DCOC_CAL_ALPHA_IR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DCOC_CAL_ALPHA_IR { bits }
    }
    #[doc = "Bits 16:26 - DCOC_CAL_ALPHA_Q"]
    #[inline]
    pub fn dcoc_cal_alpha_q(&self) -> DCOC_CAL_ALPHA_QR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DCOC_CAL_ALPHA_QR { bits }
    }
}
