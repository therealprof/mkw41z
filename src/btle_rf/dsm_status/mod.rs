#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::DSM_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ORF_SYSCLK_REQR {
    bits: bool,
}
impl ORF_SYSCLK_REQR {
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
pub struct RIF_LL_ACTIVER {
    bits: bool,
}
impl RIF_LL_ACTIVER {
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
#[doc = "Possible values of the field `XCVR_BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XCVR_BUSYR {
    #[doc = "RF Channel in available (TSM is idle)"]
    _0,
    #[doc = "RF Channel in use (TSM is busy)"]
    _1,
}
impl XCVR_BUSYR {
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
            XCVR_BUSYR::_0 => false,
            XCVR_BUSYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XCVR_BUSYR {
        match value {
            false => XCVR_BUSYR::_0,
            true => XCVR_BUSYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == XCVR_BUSYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == XCVR_BUSYR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - RF Oscillator Requested"]
    #[inline]
    pub fn orf_sysclk_req(&self) -> ORF_SYSCLK_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        ORF_SYSCLK_REQR { bits }
    }
    #[doc = "Bit 1 - Link Layer Active"]
    #[inline]
    pub fn rif_ll_active(&self) -> RIF_LL_ACTIVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RIF_LL_ACTIVER { bits }
    }
    #[doc = "Bit 2 - Transceiver Busy Status Bit"]
    #[inline]
    pub fn xcvr_busy(&self) -> XCVR_BUSYR {
        XCVR_BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
