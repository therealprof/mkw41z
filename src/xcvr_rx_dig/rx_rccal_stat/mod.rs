#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RX_RCCAL_STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RCCAL_CODER {
    bits: u8,
}
impl RCCAL_CODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADC_RCCALR {
    bits: u8,
}
impl ADC_RCCALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA2_RCCALR {
    bits: u8,
}
impl BBA2_RCCALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_RCCALR {
    bits: u8,
}
impl BBA_RCCALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TZA_RCCALR {
    bits: u8,
}
impl TZA_RCCALR {
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
    #[doc = "Bits 0:4 - RC Calibration code"]
    #[inline]
    pub fn rccal_code(&self) -> RCCAL_CODER {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCCAL_CODER { bits }
    }
    #[doc = "Bits 5:9 - ADC RC Calibration"]
    #[inline]
    pub fn adc_rccal(&self) -> ADC_RCCALR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADC_RCCALR { bits }
    }
    #[doc = "Bits 10:14 - BBA2 RC Calibration"]
    #[inline]
    pub fn bba2_rccal(&self) -> BBA2_RCCALR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA2_RCCALR { bits }
    }
    #[doc = "Bits 16:20 - BBA RC Calibration"]
    #[inline]
    pub fn bba_rccal(&self) -> BBA_RCCALR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_RCCALR { bits }
    }
    #[doc = "Bits 21:25 - TZA RC Calibration"]
    #[inline]
    pub fn tza_rccal(&self) -> TZA_RCCALR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TZA_RCCALR { bits }
    }
}
