#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AGC_STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct BBA_PDET_LO_STATR {
    bits: bool,
}
impl BBA_PDET_LO_STATR {
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
pub struct BBA_PDET_HI_STATR {
    bits: bool,
}
impl BBA_PDET_HI_STATR {
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
pub struct TZA_PDET_LO_STATR {
    bits: bool,
}
impl TZA_PDET_LO_STATR {
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
pub struct TZA_PDET_HI_STATR {
    bits: bool,
}
impl TZA_PDET_HI_STATR {
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
pub struct CURR_AGC_IDXR {
    bits: u8,
}
impl CURR_AGC_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `AGC_FROZEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGC_FROZENR {
    #[doc = "AGC is not frozen."]
    _0,
    #[doc = "AGC is frozen."]
    _1,
}
impl AGC_FROZENR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AGC_FROZENR::_0 => false,
            AGC_FROZENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AGC_FROZENR {
        match value {
            false => AGC_FROZENR::_0,
            true => AGC_FROZENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AGC_FROZENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AGC_FROZENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RSSI_ADC_RAWR {
    bits: u8,
}
impl RSSI_ADC_RAWR {
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
    #[doc = "Bit 0 - BBA Peak Detector Low Status"]
    #[inline]
    pub fn bba_pdet_lo_stat(&self) -> BBA_PDET_LO_STATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BBA_PDET_LO_STATR { bits }
    }
    #[doc = "Bit 1 - BBA Peak Detector High Status"]
    #[inline]
    pub fn bba_pdet_hi_stat(&self) -> BBA_PDET_HI_STATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BBA_PDET_HI_STATR { bits }
    }
    #[doc = "Bit 2 - TZA Peak Detector Low Status"]
    #[inline]
    pub fn tza_pdet_lo_stat(&self) -> TZA_PDET_LO_STATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TZA_PDET_LO_STATR { bits }
    }
    #[doc = "Bit 3 - TZA Peak Detector High Status"]
    #[inline]
    pub fn tza_pdet_hi_stat(&self) -> TZA_PDET_HI_STATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TZA_PDET_HI_STATR { bits }
    }
    #[doc = "Bits 4:8 - Current AGC Gain Index"]
    #[inline]
    pub fn curr_agc_idx(&self) -> CURR_AGC_IDXR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CURR_AGC_IDXR { bits }
    }
    #[doc = "Bit 9 - AGC Frozen Status"]
    #[inline]
    pub fn agc_frozen(&self) -> AGC_FROZENR {
        AGC_FROZENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - ADC RAW RSSI Reading"]
    #[inline]
    pub fn rssi_adc_raw(&self) -> RSSI_ADC_RAWR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSSI_ADC_RAWR { bits }
    }
}
