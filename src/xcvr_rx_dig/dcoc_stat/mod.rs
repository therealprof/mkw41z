#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DCOC_STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct BBA_DCOC_IR {
    bits: u8,
}
impl BBA_DCOC_IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_DCOC_QR {
    bits: u8,
}
impl BBA_DCOC_QR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TZA_DCOC_IR {
    bits: u8,
}
impl TZA_DCOC_IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TZA_DCOC_QR {
    bits: u8,
}
impl TZA_DCOC_QR {
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
    #[doc = "Bits 0:5 - DCOC BBA DAC I"]
    #[inline]
    pub fn bba_dcoc_i(&self) -> BBA_DCOC_IR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_DCOC_IR { bits }
    }
    #[doc = "Bits 8:13 - DCOC BBA DAC Q"]
    #[inline]
    pub fn bba_dcoc_q(&self) -> BBA_DCOC_QR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_DCOC_QR { bits }
    }
    #[doc = "Bits 16:23 - DCOC TZA DAC I"]
    #[inline]
    pub fn tza_dcoc_i(&self) -> TZA_DCOC_IR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TZA_DCOC_IR { bits }
    }
    #[doc = "Bits 24:31 - DCOC TZA DAC Q"]
    #[inline]
    pub fn tza_dcoc_q(&self) -> TZA_DCOC_QR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TZA_DCOC_QR { bits }
    }
}
