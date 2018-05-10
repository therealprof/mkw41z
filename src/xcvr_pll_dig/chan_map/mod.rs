#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHAN_MAP {
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
#[doc = r" Value of the field"]
pub struct CHANNEL_NUMR {
    bits: u8,
}
impl CHANNEL_NUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOCR {
    #[doc = "BLE channel number comes from the BLE Link Layer"]
    _0,
    #[doc = "BLE channel number comes from the CHANNEL_NUM register (BLE protocols 0 and 2)"]
    _1,
}
impl BOCR {
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
            BOCR::_0 => false,
            BOCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOCR {
        match value {
            false => BOCR::_0,
            true => BOCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BOCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BOCR::_1
    }
}
#[doc = "Possible values of the field `BMR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMRR {
    #[doc = "BLE channel 39 is mapped to BLE channel 39, 2.480 GHz"]
    _0,
    #[doc = "BLE channel 39 is mapped to MBAN channel 39, 2.399 GHz"]
    _1,
}
impl BMRR {
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
            BMRR::_0 => false,
            BMRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BMRR {
        match value {
            false => BMRR::_0,
            true => BMRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BMRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BMRR::_1
    }
}
#[doc = "Possible values of the field `ZOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZOCR {
    #[doc = "802.15.4 channel number comes from the 802.15.4 Link Layer."]
    _0,
    #[doc = "802.15.4 channel number comes from the CHANNEL_NUM register (802.15.4 protocols 4 and 5)"]
    _1,
}
impl ZOCR {
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
            ZOCR::_0 => false,
            ZOCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZOCR {
        match value {
            false => ZOCR::_0,
            true => ZOCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ZOCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ZOCR::_1
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL_NUMW<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL_NUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOC`"]
pub enum BOCW {
    #[doc = "BLE channel number comes from the BLE Link Layer"]
    _0,
    #[doc = "BLE channel number comes from the CHANNEL_NUM register (BLE protocols 0 and 2)"]
    _1,
}
impl BOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOCW::_0 => false,
            BOCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOCW<'a> {
    w: &'a mut W,
}
impl<'a> _BOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BLE channel number comes from the BLE Link Layer"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOCW::_0)
    }
    #[doc = "BLE channel number comes from the CHANNEL_NUM register (BLE protocols 0 and 2)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOCW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BMR`"]
pub enum BMRW {
    #[doc = "BLE channel 39 is mapped to BLE channel 39, 2.480 GHz"]
    _0,
    #[doc = "BLE channel 39 is mapped to MBAN channel 39, 2.399 GHz"]
    _1,
}
impl BMRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BMRW::_0 => false,
            BMRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BMRW<'a> {
    w: &'a mut W,
}
impl<'a> _BMRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BMRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BLE channel 39 is mapped to BLE channel 39, 2.480 GHz"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BMRW::_0)
    }
    #[doc = "BLE channel 39 is mapped to MBAN channel 39, 2.399 GHz"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BMRW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ZOC`"]
pub enum ZOCW {
    #[doc = "802.15.4 channel number comes from the 802.15.4 Link Layer."]
    _0,
    #[doc = "802.15.4 channel number comes from the CHANNEL_NUM register (802.15.4 protocols 4 and 5)"]
    _1,
}
impl ZOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZOCW::_0 => false,
            ZOCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZOCW<'a> {
    w: &'a mut W,
}
impl<'a> _ZOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZOCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "802.15.4 channel number comes from the 802.15.4 Link Layer."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ZOCW::_0)
    }
    #[doc = "802.15.4 channel number comes from the CHANNEL_NUM register (802.15.4 protocols 4 and 5)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ZOCW::_1)
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
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:6 - Protocol specific Channel Number for PLL Frequency Mapping"]
    #[inline]
    pub fn channel_num(&self) -> CHANNEL_NUMR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHANNEL_NUMR { bits }
    }
    #[doc = "Bit 8 - BLE Channel Number Override"]
    #[inline]
    pub fn boc(&self) -> BOCR {
        BOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - BLE MBAN Channel Remap"]
    #[inline]
    pub fn bmr(&self) -> BMRR {
        BMRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - 802.15.4 Channel Number Override"]
    #[inline]
    pub fn zoc(&self) -> ZOCR {
        ZOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 512 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Protocol specific Channel Number for PLL Frequency Mapping"]
    #[inline]
    pub fn channel_num(&mut self) -> _CHANNEL_NUMW {
        _CHANNEL_NUMW { w: self }
    }
    #[doc = "Bit 8 - BLE Channel Number Override"]
    #[inline]
    pub fn boc(&mut self) -> _BOCW {
        _BOCW { w: self }
    }
    #[doc = "Bit 9 - BLE MBAN Channel Remap"]
    #[inline]
    pub fn bmr(&mut self) -> _BMRW {
        _BMRW { w: self }
    }
    #[doc = "Bit 10 - 802.15.4 Channel Number Override"]
    #[inline]
    pub fn zoc(&mut self) -> _ZOCW {
        _ZOCW { w: self }
    }
}
