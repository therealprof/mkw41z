#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ERR {
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
#[doc = "Possible values of the field `ERR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR0R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR0R {
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
            ERR0R::_0 => false,
            ERR0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR0R {
        match value {
            false => ERR0R::_0,
            true => ERR0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR0R::_1
    }
}
#[doc = "Possible values of the field `ERR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR1R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR1R {
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
            ERR1R::_0 => false,
            ERR1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR1R {
        match value {
            false => ERR1R::_0,
            true => ERR1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR1R::_1
    }
}
#[doc = "Possible values of the field `ERR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR2R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR2R {
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
            ERR2R::_0 => false,
            ERR2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR2R {
        match value {
            false => ERR2R::_0,
            true => ERR2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR2R::_1
    }
}
#[doc = "Possible values of the field `ERR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR3R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR3R {
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
            ERR3R::_0 => false,
            ERR3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR3R {
        match value {
            false => ERR3R::_0,
            true => ERR3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR3R::_1
    }
}
#[doc = "Values that can be written to the field `ERR0`"]
pub enum ERR0W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR0W::_0 => false,
            ERR0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR0W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR0W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR0W::_1)
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
#[doc = "Values that can be written to the field `ERR1`"]
pub enum ERR1W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR1W::_0 => false,
            ERR1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR1W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR1W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR1W::_1)
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
#[doc = "Values that can be written to the field `ERR2`"]
pub enum ERR2W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR2W::_0 => false,
            ERR2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR2W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR2W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR2W::_1)
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
#[doc = "Values that can be written to the field `ERR3`"]
pub enum ERR3W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR3W::_0 => false,
            ERR3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR3W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR3W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR3W::_1)
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
    #[doc = "Bit 0 - Error In Channel 0"]
    #[inline]
    pub fn err0(&self) -> ERR0R {
        ERR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Error In Channel 1"]
    #[inline]
    pub fn err1(&self) -> ERR1R {
        ERR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Error In Channel 2"]
    #[inline]
    pub fn err2(&self) -> ERR2R {
        ERR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Error In Channel 3"]
    #[inline]
    pub fn err3(&self) -> ERR3R {
        ERR3R::_from({
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
    #[doc = "Bit 0 - Error In Channel 0"]
    #[inline]
    pub fn err0(&mut self) -> _ERR0W {
        _ERR0W { w: self }
    }
    #[doc = "Bit 1 - Error In Channel 1"]
    #[inline]
    pub fn err1(&mut self) -> _ERR1W {
        _ERR1W { w: self }
    }
    #[doc = "Bit 2 - Error In Channel 2"]
    #[inline]
    pub fn err2(&mut self) -> _ERR2W {
        _ERR2W { w: self }
    }
    #[doc = "Bit 3 - Error In Channel 3"]
    #[inline]
    pub fn err3(&mut self) -> _ERR3W {
        _ERR3W { w: self }
    }
}
