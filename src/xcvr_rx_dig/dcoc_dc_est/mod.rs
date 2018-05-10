#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DCOC_DC_EST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DC_EST_IR {
    bits: u16,
}
impl DC_EST_IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DC_EST_QR {
    bits: u16,
}
impl DC_EST_QR {
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
    #[doc = "Bits 0:11 - DCOC DC Estimate I"]
    #[inline]
    pub fn dc_est_i(&self) -> DC_EST_IR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DC_EST_IR { bits }
    }
    #[doc = "Bits 16:27 - DCOC DC Estimate Q"]
    #[inline]
    pub fn dc_est_q(&self) -> DC_EST_QR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DC_EST_QR { bits }
    }
}
