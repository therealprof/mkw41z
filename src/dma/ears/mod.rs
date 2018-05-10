#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EARS {
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
#[doc = "Possible values of the field `EDREQ_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_0R {
    #[doc = "Disable asynchronous DMA request for channel 0."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 0."]
    _1,
}
impl EDREQ_0R {
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
            EDREQ_0R::_0 => false,
            EDREQ_0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_0R {
        match value {
            false => EDREQ_0R::_0,
            true => EDREQ_0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_0R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_1R {
    #[doc = "Disable asynchronous DMA request for channel 1"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 1."]
    _1,
}
impl EDREQ_1R {
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
            EDREQ_1R::_0 => false,
            EDREQ_1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_1R {
        match value {
            false => EDREQ_1R::_0,
            true => EDREQ_1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_1R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_2R {
    #[doc = "Disable asynchronous DMA request for channel 2."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 2."]
    _1,
}
impl EDREQ_2R {
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
            EDREQ_2R::_0 => false,
            EDREQ_2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_2R {
        match value {
            false => EDREQ_2R::_0,
            true => EDREQ_2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_2R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_3R {
    #[doc = "Disable asynchronous DMA request for channel 3."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 3."]
    _1,
}
impl EDREQ_3R {
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
            EDREQ_3R::_0 => false,
            EDREQ_3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_3R {
        match value {
            false => EDREQ_3R::_0,
            true => EDREQ_3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_3R::_1
    }
}
#[doc = "Values that can be written to the field `EDREQ_0`"]
pub enum EDREQ_0W {
    #[doc = "Disable asynchronous DMA request for channel 0."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 0."]
    _1,
}
impl EDREQ_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_0W::_0 => false,
            EDREQ_0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_0W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_0W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_0W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_1`"]
pub enum EDREQ_1W {
    #[doc = "Disable asynchronous DMA request for channel 1"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 1."]
    _1,
}
impl EDREQ_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_1W::_0 => false,
            EDREQ_1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_1W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_1W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_1W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_2`"]
pub enum EDREQ_2W {
    #[doc = "Disable asynchronous DMA request for channel 2."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 2."]
    _1,
}
impl EDREQ_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_2W::_0 => false,
            EDREQ_2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_2W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 2."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_2W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 2."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_2W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_3`"]
pub enum EDREQ_3W {
    #[doc = "Disable asynchronous DMA request for channel 3."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 3."]
    _1,
}
impl EDREQ_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_3W::_0 => false,
            EDREQ_3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_3W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 3."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_3W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 3."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_3W::_1)
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline]
    pub fn edreq_0(&self) -> EDREQ_0R {
        EDREQ_0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline]
    pub fn edreq_1(&self) -> EDREQ_1R {
        EDREQ_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline]
    pub fn edreq_2(&self) -> EDREQ_2R {
        EDREQ_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline]
    pub fn edreq_3(&self) -> EDREQ_3R {
        EDREQ_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline]
    pub fn edreq_0(&mut self) -> _EDREQ_0W {
        _EDREQ_0W { w: self }
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline]
    pub fn edreq_1(&mut self) -> _EDREQ_1W {
        _EDREQ_1W { w: self }
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline]
    pub fn edreq_2(&mut self) -> _EDREQ_2W {
        _EDREQ_2W { w: self }
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline]
    pub fn edreq_3(&mut self) -> _EDREQ_3W {
        _EDREQ_3W { w: self }
    }
}
