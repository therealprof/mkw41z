#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EEI {
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
#[doc = "Possible values of the field `EEI0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI0R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI0R {
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
            EEI0R::_0 => false,
            EEI0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI0R {
        match value {
            false => EEI0R::_0,
            true => EEI0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI0R::_1
    }
}
#[doc = "Possible values of the field `EEI1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI1R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI1R {
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
            EEI1R::_0 => false,
            EEI1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI1R {
        match value {
            false => EEI1R::_0,
            true => EEI1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI1R::_1
    }
}
#[doc = "Possible values of the field `EEI2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI2R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI2R {
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
            EEI2R::_0 => false,
            EEI2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI2R {
        match value {
            false => EEI2R::_0,
            true => EEI2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI2R::_1
    }
}
#[doc = "Possible values of the field `EEI3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI3R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI3R {
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
            EEI3R::_0 => false,
            EEI3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI3R {
        match value {
            false => EEI3R::_0,
            true => EEI3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI3R::_1
    }
}
#[doc = "Values that can be written to the field `EEI0`"]
pub enum EEI0W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI0W::_0 => false,
            EEI0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI0W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI0W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI0W::_1)
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
#[doc = "Values that can be written to the field `EEI1`"]
pub enum EEI1W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI1W::_0 => false,
            EEI1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI1W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI1W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI1W::_1)
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
#[doc = "Values that can be written to the field `EEI2`"]
pub enum EEI2W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI2W::_0 => false,
            EEI2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI2W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI2W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI2W::_1)
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
#[doc = "Values that can be written to the field `EEI3`"]
pub enum EEI3W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI3W::_0 => false,
            EEI3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI3W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI3W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI3W::_1)
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
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline]
    pub fn eei0(&self) -> EEI0R {
        EEI0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline]
    pub fn eei1(&self) -> EEI1R {
        EEI1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline]
    pub fn eei2(&self) -> EEI2R {
        EEI2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline]
    pub fn eei3(&self) -> EEI3R {
        EEI3R::_from({
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
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline]
    pub fn eei0(&mut self) -> _EEI0W {
        _EEI0W { w: self }
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline]
    pub fn eei1(&mut self) -> _EEI1W {
        _EEI1W { w: self }
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline]
    pub fn eei2(&mut self) -> _EEI2W {
        _EEI2W { w: self }
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline]
    pub fn eei3(&mut self) -> _EEI3W {
        _EEI3W { w: self }
    }
}
