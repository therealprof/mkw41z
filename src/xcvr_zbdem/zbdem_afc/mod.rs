#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ZBDEM_AFC {
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
#[doc = "Possible values of the field `AFC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFC_ENR {
    #[doc = "AFC is disabled"]
    _0,
    #[doc = "AFC is enabled"]
    _1,
}
impl AFC_ENR {
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
            AFC_ENR::_0 => false,
            AFC_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AFC_ENR {
        match value {
            false => AFC_ENR::_0,
            true => AFC_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AFC_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AFC_ENR::_1
    }
}
#[doc = "Possible values of the field `DCD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCD_ENR {
    #[doc = "NCD Mode (default)"]
    _0,
    #[doc = "DCD Mode"]
    _1,
}
impl DCD_ENR {
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
            DCD_ENR::_0 => false,
            DCD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCD_ENR {
        match value {
            false => DCD_ENR::_0,
            true => DCD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct AFC_OUTR {
    bits: u8,
}
impl AFC_OUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `AFC_EN`"]
pub enum AFC_ENW {
    #[doc = "AFC is disabled"]
    _0,
    #[doc = "AFC is enabled"]
    _1,
}
impl AFC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AFC_ENW::_0 => false,
            AFC_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AFC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AFC is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AFC_ENW::_0)
    }
    #[doc = "AFC is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AFC_ENW::_1)
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
#[doc = "Values that can be written to the field `DCD_EN`"]
pub enum DCD_ENW {
    #[doc = "NCD Mode (default)"]
    _0,
    #[doc = "DCD Mode"]
    _1,
}
impl DCD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCD_ENW::_0 => false,
            DCD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NCD Mode (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCD_ENW::_0)
    }
    #[doc = "DCD Mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCD_ENW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - AFC_EN"]
    #[inline]
    pub fn afc_en(&self) -> AFC_ENR {
        AFC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DCD_EN"]
    #[inline]
    pub fn dcd_en(&self) -> DCD_ENR {
        DCD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - AFC_OUT"]
    #[inline]
    pub fn afc_out(&self) -> AFC_OUTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFC_OUTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - AFC_EN"]
    #[inline]
    pub fn afc_en(&mut self) -> _AFC_ENW {
        _AFC_ENW { w: self }
    }
    #[doc = "Bit 1 - DCD_EN"]
    #[inline]
    pub fn dcd_en(&mut self) -> _DCD_ENW {
        _DCD_ENW { w: self }
    }
}
