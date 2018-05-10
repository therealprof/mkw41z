#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CCA_ED_LQI_STAT_0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct LQI_OUTR {
    bits: u8,
}
impl LQI_OUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ED_OUTR {
    bits: u8,
}
impl ED_OUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SNR_OUTR {
    bits: u8,
}
impl SNR_OUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CCA1_STATER {
    bits: bool,
}
impl CCA1_STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct MEAS_COMPLETER {
    bits: bool,
}
impl MEAS_COMPLETER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - LQI output"]
    #[inline]
    pub fn lqi_out(&self) -> LQI_OUTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LQI_OUTR { bits }
    }
    #[doc = "Bits 8:15 - ED output"]
    #[inline]
    pub fn ed_out(&self) -> ED_OUTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ED_OUTR { bits }
    }
    #[doc = "Bits 16:23 - SNR output"]
    #[inline]
    pub fn snr_out(&self) -> SNR_OUTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SNR_OUTR { bits }
    }
    #[doc = "Bit 24 - CCA1 State"]
    #[inline]
    pub fn cca1_state(&self) -> CCA1_STATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CCA1_STATER { bits }
    }
    #[doc = "Bit 25 - Measurement Complete"]
    #[inline]
    pub fn meas_complete(&self) -> MEAS_COMPLETER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MEAS_COMPLETER { bits }
    }
}
