#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCA_LQI_CTRL {
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
pub struct CCA1_THRESHR {
    bits: u8,
}
impl CCA1_THRESHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LQI_OFFSET_COMPR {
    bits: u8,
}
impl LQI_OFFSET_COMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CCA3_AND_NOT_OR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCA3_AND_NOT_ORR {
    #[doc = "CCA1 or CCA2"]
    _0,
    #[doc = "CCA1 and CCA2"]
    _1,
}
impl CCA3_AND_NOT_ORR {
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
            CCA3_AND_NOT_ORR::_0 => false,
            CCA3_AND_NOT_ORR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCA3_AND_NOT_ORR {
        match value {
            false => CCA3_AND_NOT_ORR::_0,
            true => CCA3_AND_NOT_ORR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCA3_AND_NOT_ORR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCA3_AND_NOT_ORR::_1
    }
}
#[doc = r" Proxy"]
pub struct _CCA1_THRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _CCA1_THRESHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LQI_OFFSET_COMPW<'a> {
    w: &'a mut W,
}
impl<'a> _LQI_OFFSET_COMPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCA3_AND_NOT_OR`"]
pub enum CCA3_AND_NOT_ORW {
    #[doc = "CCA1 or CCA2"]
    _0,
    #[doc = "CCA1 and CCA2"]
    _1,
}
impl CCA3_AND_NOT_ORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCA3_AND_NOT_ORW::_0 => false,
            CCA3_AND_NOT_ORW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCA3_AND_NOT_ORW<'a> {
    w: &'a mut W,
}
impl<'a> _CCA3_AND_NOT_ORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCA3_AND_NOT_ORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCA1 or CCA2"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCA3_AND_NOT_ORW::_0)
    }
    #[doc = "CCA1 and CCA2"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCA3_AND_NOT_ORW::_1)
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:7 - CCA Mode 1 Threshold"]
    #[inline]
    pub fn cca1_thresh(&self) -> CCA1_THRESHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCA1_THRESHR { bits }
    }
    #[doc = "Bits 16:23 - LQI Offset Compensation"]
    #[inline]
    pub fn lqi_offset_comp(&self) -> LQI_OFFSET_COMPR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LQI_OFFSET_COMPR { bits }
    }
    #[doc = "Bit 27 - CCA Mode 3 AND not OR"]
    #[inline]
    pub fn cca3_and_not_or(&self) -> CCA3_AND_NOT_ORR {
        CCA3_AND_NOT_ORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 140902475 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - CCA Mode 1 Threshold"]
    #[inline]
    pub fn cca1_thresh(&mut self) -> _CCA1_THRESHW {
        _CCA1_THRESHW { w: self }
    }
    #[doc = "Bits 16:23 - LQI Offset Compensation"]
    #[inline]
    pub fn lqi_offset_comp(&mut self) -> _LQI_OFFSET_COMPW {
        _LQI_OFFSET_COMPW { w: self }
    }
    #[doc = "Bit 27 - CCA Mode 3 AND not OR"]
    #[inline]
    pub fn cca3_and_not_or(&mut self) -> _CCA3_AND_NOT_ORW {
        _CCA3_AND_NOT_ORW { w: self }
    }
}
