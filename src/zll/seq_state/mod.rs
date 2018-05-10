#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SEQ_STATE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SEQ_STATER {
    bits: u8,
}
impl SEQ_STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PREAMBLE_DETR {
    bits: bool,
}
impl PREAMBLE_DETR {
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
pub struct SFD_DETR {
    bits: bool,
}
impl SFD_DETR {
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
pub struct FILTERFAIL_FLAG_SELR {
    bits: bool,
}
impl FILTERFAIL_FLAG_SELR {
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
#[doc = "Possible values of the field `CRCVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCVALIDR {
    #[doc = "Rx FCS != calculated CRC (incorrect)"]
    _0,
    #[doc = "Rx FCS = calculated CRC (correct)"]
    _1,
}
impl CRCVALIDR {
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
            CRCVALIDR::_0 => false,
            CRCVALIDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCVALIDR {
        match value {
            false => CRCVALIDR::_0,
            true => CRCVALIDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRCVALIDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRCVALIDR::_1
    }
}
#[doc = r" Value of the field"]
pub struct PLL_ABORTR {
    bits: bool,
}
impl PLL_ABORTR {
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
pub struct PLL_ABORTEDR {
    bits: bool,
}
impl PLL_ABORTEDR {
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
pub struct RX_BYTE_COUNTR {
    bits: u8,
}
impl RX_BYTE_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CCCA_BUSY_CNTR {
    bits: u8,
}
impl CCCA_BUSY_CNTR {
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
    #[doc = "Bits 0:4 - ZSM Sequence State"]
    #[inline]
    pub fn seq_state(&self) -> SEQ_STATER {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEQ_STATER { bits }
    }
    #[doc = "Bit 8 - Preamble Detected"]
    #[inline]
    pub fn preamble_det(&self) -> PREAMBLE_DETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PREAMBLE_DETR { bits }
    }
    #[doc = "Bit 9 - SFD Detected"]
    #[inline]
    pub fn sfd_det(&self) -> SFD_DETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFD_DETR { bits }
    }
    #[doc = "Bit 10 - Consolidated Filter Fail Flag"]
    #[inline]
    pub fn filterfail_flag_sel(&self) -> FILTERFAIL_FLAG_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FILTERFAIL_FLAG_SELR { bits }
    }
    #[doc = "Bit 11 - CRC Valid Indicator"]
    #[inline]
    pub fn crcvalid(&self) -> CRCVALIDR {
        CRCVALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Raw PLL Abort Signal"]
    #[inline]
    pub fn pll_abort(&self) -> PLL_ABORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_ABORTR { bits }
    }
    #[doc = "Bit 13 - Autosequence has terminated due to an PLL unlock event"]
    #[inline]
    pub fn pll_aborted(&self) -> PLL_ABORTEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_ABORTEDR { bits }
    }
    #[doc = "Bits 16:23 - Realtime Received Byte Count"]
    #[inline]
    pub fn rx_byte_count(&self) -> RX_BYTE_COUNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_BYTE_COUNTR { bits }
    }
    #[doc = "Bits 24:29 - Number of CCA Measurements resulting in Busy Channel"]
    #[inline]
    pub fn ccca_busy_cnt(&self) -> CCCA_BUSY_CNTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCCA_BUSY_CNTR { bits }
    }
}
