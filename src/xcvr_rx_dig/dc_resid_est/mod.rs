#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DC_RESID_EST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DC_RESID_OFFSET_IR {
    bits: u16,
}
impl DC_RESID_OFFSET_IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DC_RESID_OFFSET_QR {
    bits: u16,
}
impl DC_RESID_OFFSET_QR {
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
    #[doc = "Bits 0:12 - DC Residual Offset I"]
    #[inline]
    pub fn dc_resid_offset_i(&self) -> DC_RESID_OFFSET_IR {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DC_RESID_OFFSET_IR { bits }
    }
    #[doc = "Bits 16:28 - DC Residual Offset Q"]
    #[inline]
    pub fn dc_resid_offset_q(&self) -> DC_RESID_OFFSET_QR {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DC_RESID_OFFSET_QR { bits }
    }
}
