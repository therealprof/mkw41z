#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INT_STATUS {
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
#[doc = "Possible values of the field `HW_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW_ERRR {
    #[doc = "no error"]
    _0,
    #[doc = "error detected."]
    _1,
}
impl HW_ERRR {
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
            HW_ERRR::_0 => false,
            HW_ERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HW_ERRR {
        match value {
            false => HW_ERRR::_0,
            true => HW_ERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HW_ERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HW_ERRR::_1
    }
}
#[doc = "Possible values of the field `ENT_VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENT_VALR {
    #[doc = "Busy generation entropy. Any value read is invalid."]
    _0,
    #[doc = "TRNG can be stopped and entropy is valid if read."]
    _1,
}
impl ENT_VALR {
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
            ENT_VALR::_0 => false,
            ENT_VALR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENT_VALR {
        match value {
            false => ENT_VALR::_0,
            true => ENT_VALR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENT_VALR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENT_VALR::_1
    }
}
#[doc = "Possible values of the field `FRQ_CT_FAIL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRQ_CT_FAILR {
    #[doc = "No hardware nor self test frequency errors."]
    _0,
    #[doc = "The frequency counter has detected a failure."]
    _1,
}
impl FRQ_CT_FAILR {
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
            FRQ_CT_FAILR::_0 => false,
            FRQ_CT_FAILR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRQ_CT_FAILR {
        match value {
            false => FRQ_CT_FAILR::_0,
            true => FRQ_CT_FAILR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FRQ_CT_FAILR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FRQ_CT_FAILR::_1
    }
}
#[doc = "Values that can be written to the field `FRQ_CT_FAIL`"]
pub enum FRQ_CT_FAILW {
    #[doc = "No hardware nor self test frequency errors."]
    _0,
    #[doc = "The frequency counter has detected a failure."]
    _1,
}
impl FRQ_CT_FAILW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRQ_CT_FAILW::_0 => false,
            FRQ_CT_FAILW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRQ_CT_FAILW<'a> {
    w: &'a mut W,
}
impl<'a> _FRQ_CT_FAILW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRQ_CT_FAILW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No hardware nor self test frequency errors."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRQ_CT_FAILW::_0)
    }
    #[doc = "The frequency counter has detected a failure."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRQ_CT_FAILW::_1)
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
    #[doc = "Bit 0 - Read: Error status"]
    #[inline]
    pub fn hw_err(&self) -> HW_ERRR {
        HW_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Read only: Entropy Valid"]
    #[inline]
    pub fn ent_val(&self) -> ENT_VALR {
        ENT_VALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Read only: Frequency Count Fail"]
    #[inline]
    pub fn frq_ct_fail(&self) -> FRQ_CT_FAILR {
        FRQ_CT_FAILR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Read only: Frequency Count Fail"]
    #[inline]
    pub fn frq_ct_fail(&mut self) -> _FRQ_CT_FAILW {
        _FRQ_CT_FAILW { w: self }
    }
}
