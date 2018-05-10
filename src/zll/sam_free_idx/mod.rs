#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SAM_FREE_IDX {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SAP0_1ST_FREE_IDXR {
    bits: u8,
}
impl SAP0_1ST_FREE_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAA0_1ST_FREE_IDXR {
    bits: u8,
}
impl SAA0_1ST_FREE_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAP1_1ST_FREE_IDXR {
    bits: u8,
}
impl SAP1_1ST_FREE_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAA1_1ST_FREE_IDXR {
    bits: u8,
}
impl SAA1_1ST_FREE_IDXR {
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
    #[doc = "Bits 0:7 - First non-enabled (invalid) index in the SAP0 partition"]
    #[inline]
    pub fn sap0_1st_free_idx(&self) -> SAP0_1ST_FREE_IDXR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAP0_1ST_FREE_IDXR { bits }
    }
    #[doc = "Bits 8:15 - First non-enabled (invalid) index in the SAA0 partition"]
    #[inline]
    pub fn saa0_1st_free_idx(&self) -> SAA0_1ST_FREE_IDXR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAA0_1ST_FREE_IDXR { bits }
    }
    #[doc = "Bits 16:23 - First non-enabled (invalid) index in the SAP1 partition"]
    #[inline]
    pub fn sap1_1st_free_idx(&self) -> SAP1_1ST_FREE_IDXR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAP1_1ST_FREE_IDXR { bits }
    }
    #[doc = "Bits 24:31 - First non-enabled (invalid) index in the SAA1 partition"]
    #[inline]
    pub fn saa1_1st_free_idx(&self) -> SAA1_1ST_FREE_IDXR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAA1_1ST_FREE_IDXR { bits }
    }
}
