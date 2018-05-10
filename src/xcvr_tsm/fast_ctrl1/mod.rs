#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FAST_CTRL1 {
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
#[doc = "Possible values of the field `FAST_TX_WU_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_TX_WU_ENR {
    #[doc = "Fast TSM TX Warmups are disabled"]
    _0,
    #[doc = "Fast TSM TX Warmups are enabled, if the RF channel has not changed since the last TX warmup, and for BLE mode, the RF channel is not an advertising channel."]
    _1,
}
impl FAST_TX_WU_ENR {
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
            FAST_TX_WU_ENR::_0 => false,
            FAST_TX_WU_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAST_TX_WU_ENR {
        match value {
            false => FAST_TX_WU_ENR::_0,
            true => FAST_TX_WU_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAST_TX_WU_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAST_TX_WU_ENR::_1
    }
}
#[doc = "Possible values of the field `FAST_RX_WU_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_RX_WU_ENR {
    #[doc = "Fast TSM RX Warmups are disabled"]
    _0,
    #[doc = "Fast TSM RX Warmups are enabled, if the RF channel has not changed since the last RX warmup, and for BLE mode, the RF channel is not an advertising channel."]
    _1,
}
impl FAST_RX_WU_ENR {
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
            FAST_RX_WU_ENR::_0 => false,
            FAST_RX_WU_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAST_RX_WU_ENR {
        match value {
            false => FAST_RX_WU_ENR::_0,
            true => FAST_RX_WU_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAST_RX_WU_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAST_RX_WU_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct FAST_RX2TX_ENR {
    bits: bool,
}
impl FAST_RX2TX_ENR {
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
pub struct FAST_WU_CLEARR {
    bits: bool,
}
impl FAST_WU_CLEARR {
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
pub struct FAST_RX2TX_STARTR {
    bits: u8,
}
impl FAST_RX2TX_STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `FAST_TX_WU_EN`"]
pub enum FAST_TX_WU_ENW {
    #[doc = "Fast TSM TX Warmups are disabled"]
    _0,
    #[doc = "Fast TSM TX Warmups are enabled, if the RF channel has not changed since the last TX warmup, and for BLE mode, the RF channel is not an advertising channel."]
    _1,
}
impl FAST_TX_WU_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FAST_TX_WU_ENW::_0 => false,
            FAST_TX_WU_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAST_TX_WU_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FAST_TX_WU_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAST_TX_WU_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fast TSM TX Warmups are disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAST_TX_WU_ENW::_0)
    }
    #[doc = "Fast TSM TX Warmups are enabled, if the RF channel has not changed since the last TX warmup, and for BLE mode, the RF channel is not an advertising channel."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAST_TX_WU_ENW::_1)
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
#[doc = "Values that can be written to the field `FAST_RX_WU_EN`"]
pub enum FAST_RX_WU_ENW {
    #[doc = "Fast TSM RX Warmups are disabled"]
    _0,
    #[doc = "Fast TSM RX Warmups are enabled, if the RF channel has not changed since the last RX warmup, and for BLE mode, the RF channel is not an advertising channel."]
    _1,
}
impl FAST_RX_WU_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FAST_RX_WU_ENW::_0 => false,
            FAST_RX_WU_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAST_RX_WU_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FAST_RX_WU_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAST_RX_WU_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fast TSM RX Warmups are disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAST_RX_WU_ENW::_0)
    }
    #[doc = "Fast TSM RX Warmups are enabled, if the RF channel has not changed since the last RX warmup, and for BLE mode, the RF channel is not an advertising channel."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAST_RX_WU_ENW::_1)
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
#[doc = r" Proxy"]
pub struct _FAST_RX2TX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FAST_RX2TX_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _FAST_WU_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _FAST_WU_CLEARW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FAST_RX2TX_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _FAST_RX2TX_STARTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Fast TSM TX Warmup Enable"]
    #[inline]
    pub fn fast_tx_wu_en(&self) -> FAST_TX_WU_ENR {
        FAST_TX_WU_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Fast TSM RX Warmup Enable"]
    #[inline]
    pub fn fast_rx_wu_en(&self) -> FAST_RX_WU_ENR {
        FAST_RX_WU_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Fast TSM RX-to-TX Transition Enable"]
    #[inline]
    pub fn fast_rx2tx_en(&self) -> FAST_RX2TX_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FAST_RX2TX_ENR { bits }
    }
    #[doc = "Bit 3 - Fast TSM Warmup Clear State"]
    #[inline]
    pub fn fast_wu_clear(&self) -> FAST_WU_CLEARR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FAST_WU_CLEARR { bits }
    }
    #[doc = "Bits 8:15 - TSM \"Jump-to\" point for a Fast TSM RX-to-TX Transition."]
    #[inline]
    pub fn fast_rx2tx_start(&self) -> FAST_RX2TX_STARTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FAST_RX2TX_STARTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65280 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Fast TSM TX Warmup Enable"]
    #[inline]
    pub fn fast_tx_wu_en(&mut self) -> _FAST_TX_WU_ENW {
        _FAST_TX_WU_ENW { w: self }
    }
    #[doc = "Bit 1 - Fast TSM RX Warmup Enable"]
    #[inline]
    pub fn fast_rx_wu_en(&mut self) -> _FAST_RX_WU_ENW {
        _FAST_RX_WU_ENW { w: self }
    }
    #[doc = "Bit 2 - Fast TSM RX-to-TX Transition Enable"]
    #[inline]
    pub fn fast_rx2tx_en(&mut self) -> _FAST_RX2TX_ENW {
        _FAST_RX2TX_ENW { w: self }
    }
    #[doc = "Bit 3 - Fast TSM Warmup Clear State"]
    #[inline]
    pub fn fast_wu_clear(&mut self) -> _FAST_WU_CLEARW {
        _FAST_WU_CLEARW { w: self }
    }
    #[doc = "Bits 8:15 - TSM \"Jump-to\" point for a Fast TSM RX-to-TX Transition."]
    #[inline]
    pub fn fast_rx2tx_start(&mut self) -> _FAST_RX2TX_STARTW {
        _FAST_RX2TX_STARTW { w: self }
    }
}
