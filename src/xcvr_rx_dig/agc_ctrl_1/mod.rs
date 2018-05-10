#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AGC_CTRL_1 {
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
pub struct BBA_ALT_CODER {
    bits: u8,
}
impl BBA_ALT_CODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_ALT_CODER {
    bits: u8,
}
impl LNA_ALT_CODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_USER_GAINR {
    bits: u8,
}
impl LNA_USER_GAINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_USER_GAINR {
    bits: u8,
}
impl BBA_USER_GAINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USER_LNA_GAIN_ENR {
    bits: bool,
}
impl USER_LNA_GAIN_ENR {
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
pub struct USER_BBA_GAIN_ENR {
    bits: bool,
}
impl USER_BBA_GAIN_ENR {
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
#[doc = "Possible values of the field `PRESLOW_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESLOW_ENR {
    #[doc = "Pre-slow is disabled."]
    _0,
    #[doc = "Pre-slow is enabled."]
    _1,
}
impl PRESLOW_ENR {
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
            PRESLOW_ENR::_0 => false,
            PRESLOW_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRESLOW_ENR {
        match value {
            false => PRESLOW_ENR::_0,
            true => PRESLOW_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PRESLOW_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PRESLOW_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_SETTLE_TIMER {
    bits: u8,
}
impl LNA_GAIN_SETTLE_TIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BBA_ALT_CODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_ALT_CODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LNA_ALT_CODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_ALT_CODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LNA_USER_GAINW<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_USER_GAINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BBA_USER_GAINW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_USER_GAINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USER_LNA_GAIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USER_LNA_GAIN_ENW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USER_BBA_GAIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USER_BBA_GAIN_ENW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESLOW_EN`"]
pub enum PRESLOW_ENW {
    #[doc = "Pre-slow is disabled."]
    _0,
    #[doc = "Pre-slow is enabled."]
    _1,
}
impl PRESLOW_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRESLOW_ENW::_0 => false,
            PRESLOW_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESLOW_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESLOW_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESLOW_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pre-slow is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRESLOW_ENW::_0)
    }
    #[doc = "Pre-slow is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRESLOW_ENW::_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LNA_GAIN_SETTLE_TIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_SETTLE_TIMEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:3 - BBA_ALT_CODE"]
    #[inline]
    pub fn bba_alt_code(&self) -> BBA_ALT_CODER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_ALT_CODER { bits }
    }
    #[doc = "Bits 4:11 - LNA_ALT_CODE"]
    #[inline]
    pub fn lna_alt_code(&self) -> LNA_ALT_CODER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_ALT_CODER { bits }
    }
    #[doc = "Bits 12:15 - LNA_USER_GAIN"]
    #[inline]
    pub fn lna_user_gain(&self) -> LNA_USER_GAINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_USER_GAINR { bits }
    }
    #[doc = "Bits 16:19 - BBA_USER_GAIN"]
    #[inline]
    pub fn bba_user_gain(&self) -> BBA_USER_GAINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_USER_GAINR { bits }
    }
    #[doc = "Bit 20 - User LNA Gain Enable"]
    #[inline]
    pub fn user_lna_gain_en(&self) -> USER_LNA_GAIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USER_LNA_GAIN_ENR { bits }
    }
    #[doc = "Bit 21 - User BBA Gain Enable"]
    #[inline]
    pub fn user_bba_gain_en(&self) -> USER_BBA_GAIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USER_BBA_GAIN_ENR { bits }
    }
    #[doc = "Bit 22 - Pre-slow Enable"]
    #[inline]
    pub fn preslow_en(&self) -> PRESLOW_ENR {
        PRESLOW_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:31 - LNA_GAIN_SETTLE_TIME"]
    #[inline]
    pub fn lna_gain_settle_time(&self) -> LNA_GAIN_SETTLE_TIMER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_SETTLE_TIMER { bits }
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
    #[doc = "Bits 0:3 - BBA_ALT_CODE"]
    #[inline]
    pub fn bba_alt_code(&mut self) -> _BBA_ALT_CODEW {
        _BBA_ALT_CODEW { w: self }
    }
    #[doc = "Bits 4:11 - LNA_ALT_CODE"]
    #[inline]
    pub fn lna_alt_code(&mut self) -> _LNA_ALT_CODEW {
        _LNA_ALT_CODEW { w: self }
    }
    #[doc = "Bits 12:15 - LNA_USER_GAIN"]
    #[inline]
    pub fn lna_user_gain(&mut self) -> _LNA_USER_GAINW {
        _LNA_USER_GAINW { w: self }
    }
    #[doc = "Bits 16:19 - BBA_USER_GAIN"]
    #[inline]
    pub fn bba_user_gain(&mut self) -> _BBA_USER_GAINW {
        _BBA_USER_GAINW { w: self }
    }
    #[doc = "Bit 20 - User LNA Gain Enable"]
    #[inline]
    pub fn user_lna_gain_en(&mut self) -> _USER_LNA_GAIN_ENW {
        _USER_LNA_GAIN_ENW { w: self }
    }
    #[doc = "Bit 21 - User BBA Gain Enable"]
    #[inline]
    pub fn user_bba_gain_en(&mut self) -> _USER_BBA_GAIN_ENW {
        _USER_BBA_GAIN_ENW { w: self }
    }
    #[doc = "Bit 22 - Pre-slow Enable"]
    #[inline]
    pub fn preslow_en(&mut self) -> _PRESLOW_ENW {
        _PRESLOW_ENW { w: self }
    }
    #[doc = "Bits 24:31 - LNA_GAIN_SETTLE_TIME"]
    #[inline]
    pub fn lna_gain_settle_time(&mut self) -> _LNA_GAIN_SETTLE_TIMEW {
        _LNA_GAIN_SETTLE_TIMEW { w: self }
    }
}
