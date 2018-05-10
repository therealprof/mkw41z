#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCA_LQI_SRC {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `CCA1_FROM_RX_DIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCA1_FROM_RX_DIGR {
    #[doc = "Use the CCA1 information computed internally in the 802.15.4 Demod"]
    _0,
    #[doc = "Use the CCA1 information computed by the RX Digital"]
    _1,
}
impl CCA1_FROM_RX_DIGR {
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
            CCA1_FROM_RX_DIGR::_0 => false,
            CCA1_FROM_RX_DIGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCA1_FROM_RX_DIGR {
        match value {
            false => CCA1_FROM_RX_DIGR::_0,
            true => CCA1_FROM_RX_DIGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCA1_FROM_RX_DIGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCA1_FROM_RX_DIGR::_1
    }
}
#[doc = "Possible values of the field `LQI_FROM_RX_DIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LQI_FROM_RX_DIGR {
    #[doc = "Use the LQI information computed internally in the 802.15.4 Demod"]
    _0,
    #[doc = "Use the LQI information computed by the RX Digital"]
    _1,
}
impl LQI_FROM_RX_DIGR {
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
            LQI_FROM_RX_DIGR::_0 => false,
            LQI_FROM_RX_DIGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LQI_FROM_RX_DIGR {
        match value {
            false => LQI_FROM_RX_DIGR::_0,
            true => LQI_FROM_RX_DIGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LQI_FROM_RX_DIGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LQI_FROM_RX_DIGR::_1
    }
}
#[doc = "Possible values of the field `LQI_START_AT_SFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LQI_START_AT_SFDR {
    #[doc = "Start LQI computation at Preamble Detection (similar to previous Freescale 802.15.4 products)"]
    _0,
    #[doc = "Start LQI computation at SFD (Start of Frame Delimiter) Detection"]
    _1,
}
impl LQI_START_AT_SFDR {
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
            LQI_START_AT_SFDR::_0 => false,
            LQI_START_AT_SFDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LQI_START_AT_SFDR {
        match value {
            false => LQI_START_AT_SFDR::_0,
            true => LQI_START_AT_SFDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LQI_START_AT_SFDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LQI_START_AT_SFDR::_1
    }
}
#[doc = "Values that can be written to the field `CCA1_FROM_RX_DIG`"]
pub enum CCA1_FROM_RX_DIGW {
    #[doc = "Use the CCA1 information computed internally in the 802.15.4 Demod"]
    _0,
    #[doc = "Use the CCA1 information computed by the RX Digital"]
    _1,
}
impl CCA1_FROM_RX_DIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCA1_FROM_RX_DIGW::_0 => false,
            CCA1_FROM_RX_DIGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCA1_FROM_RX_DIGW<'a> {
    w: &'a mut W,
}
impl<'a> _CCA1_FROM_RX_DIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCA1_FROM_RX_DIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use the CCA1 information computed internally in the 802.15.4 Demod"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCA1_FROM_RX_DIGW::_0)
    }
    #[doc = "Use the CCA1 information computed by the RX Digital"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCA1_FROM_RX_DIGW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LQI_FROM_RX_DIG`"]
pub enum LQI_FROM_RX_DIGW {
    #[doc = "Use the LQI information computed internally in the 802.15.4 Demod"]
    _0,
    #[doc = "Use the LQI information computed by the RX Digital"]
    _1,
}
impl LQI_FROM_RX_DIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LQI_FROM_RX_DIGW::_0 => false,
            LQI_FROM_RX_DIGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LQI_FROM_RX_DIGW<'a> {
    w: &'a mut W,
}
impl<'a> _LQI_FROM_RX_DIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LQI_FROM_RX_DIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use the LQI information computed internally in the 802.15.4 Demod"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LQI_FROM_RX_DIGW::_0)
    }
    #[doc = "Use the LQI information computed by the RX Digital"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LQI_FROM_RX_DIGW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LQI_START_AT_SFD`"]
pub enum LQI_START_AT_SFDW {
    #[doc = "Start LQI computation at Preamble Detection (similar to previous Freescale 802.15.4 products)"]
    _0,
    #[doc = "Start LQI computation at SFD (Start of Frame Delimiter) Detection"]
    _1,
}
impl LQI_START_AT_SFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LQI_START_AT_SFDW::_0 => false,
            LQI_START_AT_SFDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LQI_START_AT_SFDW<'a> {
    w: &'a mut W,
}
impl<'a> _LQI_START_AT_SFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LQI_START_AT_SFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Start LQI computation at Preamble Detection (similar to previous Freescale 802.15.4 products)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LQI_START_AT_SFDW::_0)
    }
    #[doc = "Start LQI computation at SFD (Start of Frame Delimiter) Detection"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LQI_START_AT_SFDW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Selects the Source of CCA1 (Clear Channel Assessment Mode 1) Information Provided to the 802.15.4 Link Layer"]
    #[inline]
    pub fn cca1_from_rx_dig(&self) -> CCA1_FROM_RX_DIGR {
        CCA1_FROM_RX_DIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Selects the Source of LQI (Link Quality Indicator) Information Provided to the 802.15.4 Link Layer"]
    #[inline]
    pub fn lqi_from_rx_dig(&self) -> LQI_FROM_RX_DIGR {
        LQI_FROM_RX_DIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Select Start Point for LQI Computation"]
    #[inline]
    pub fn lqi_start_at_sfd(&self) -> LQI_START_AT_SFDR {
        LQI_START_AT_SFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Selects the Source of CCA1 (Clear Channel Assessment Mode 1) Information Provided to the 802.15.4 Link Layer"]
    #[inline]
    pub fn cca1_from_rx_dig(&mut self) -> _CCA1_FROM_RX_DIGW {
        _CCA1_FROM_RX_DIGW { w: self }
    }
    #[doc = "Bit 1 - Selects the Source of LQI (Link Quality Indicator) Information Provided to the 802.15.4 Link Layer"]
    #[inline]
    pub fn lqi_from_rx_dig(&mut self) -> _LQI_FROM_RX_DIGW {
        _LQI_FROM_RX_DIGW { w: self }
    }
    #[doc = "Bit 2 - Select Start Point for LQI Computation"]
    #[inline]
    pub fn lqi_start_at_sfd(&mut self) -> _LQI_START_AT_SFDW {
        _LQI_START_AT_SFDW { w: self }
    }
}
