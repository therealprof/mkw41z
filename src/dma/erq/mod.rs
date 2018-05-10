#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ERQ {
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
#[doc = "Possible values of the field `ERQ0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ0R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ0R {
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
            ERQ0R::_0 => false,
            ERQ0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ0R {
        match value {
            false => ERQ0R::_0,
            true => ERQ0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ0R::_1
    }
}
#[doc = "Possible values of the field `ERQ1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ1R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ1R {
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
            ERQ1R::_0 => false,
            ERQ1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ1R {
        match value {
            false => ERQ1R::_0,
            true => ERQ1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ1R::_1
    }
}
#[doc = "Possible values of the field `ERQ2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ2R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ2R {
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
            ERQ2R::_0 => false,
            ERQ2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ2R {
        match value {
            false => ERQ2R::_0,
            true => ERQ2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ2R::_1
    }
}
#[doc = "Possible values of the field `ERQ3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ3R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ3R {
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
            ERQ3R::_0 => false,
            ERQ3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ3R {
        match value {
            false => ERQ3R::_0,
            true => ERQ3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ3R::_1
    }
}
#[doc = "Values that can be written to the field `ERQ0`"]
pub enum ERQ0W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ0W::_0 => false,
            ERQ0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ0W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ0W::_1)
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
#[doc = "Values that can be written to the field `ERQ1`"]
pub enum ERQ1W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ1W::_0 => false,
            ERQ1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ1W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ1W::_1)
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
#[doc = "Values that can be written to the field `ERQ2`"]
pub enum ERQ2W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ2W::_0 => false,
            ERQ2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ2W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ2W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ2W::_1)
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
#[doc = "Values that can be written to the field `ERQ3`"]
pub enum ERQ3W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ3W::_0 => false,
            ERQ3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ3W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ3W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ3W::_1)
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
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline]
    pub fn erq0(&self) -> ERQ0R {
        ERQ0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline]
    pub fn erq1(&self) -> ERQ1R {
        ERQ1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline]
    pub fn erq2(&self) -> ERQ2R {
        ERQ2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline]
    pub fn erq3(&self) -> ERQ3R {
        ERQ3R::_from({
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
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline]
    pub fn erq0(&mut self) -> _ERQ0W {
        _ERQ0W { w: self }
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline]
    pub fn erq1(&mut self) -> _ERQ1W {
        _ERQ1W { w: self }
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline]
    pub fn erq2(&mut self) -> _ERQ2W {
        _ERQ2W { w: self }
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline]
    pub fn erq3(&mut self) -> _ERQ3W {
        _ERQ3W { w: self }
    }
}
