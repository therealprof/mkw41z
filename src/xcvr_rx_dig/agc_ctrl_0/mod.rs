#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AGC_CTRL_0 {
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
pub struct SLOW_AGC_ENR {
    bits: bool,
}
impl SLOW_AGC_ENR {
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
#[doc = "Possible values of the field `SLOW_AGC_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOW_AGC_SRCR {
    #[doc = "Access Address match (for active protocol)"]
    _0,
    #[doc = "Preamble Detect (for active protocol)"]
    _1,
    #[doc = "Fast AGC expire timer"]
    _2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SLOW_AGC_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLOW_AGC_SRCR::_0 => 0,
            SLOW_AGC_SRCR::_1 => 1,
            SLOW_AGC_SRCR::_2 => 2,
            SLOW_AGC_SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLOW_AGC_SRCR {
        match value {
            0 => SLOW_AGC_SRCR::_0,
            1 => SLOW_AGC_SRCR::_1,
            2 => SLOW_AGC_SRCR::_2,
            i => SLOW_AGC_SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLOW_AGC_SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLOW_AGC_SRCR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == SLOW_AGC_SRCR::_2
    }
}
#[doc = r" Value of the field"]
pub struct AGC_FREEZE_ENR {
    bits: bool,
}
impl AGC_FREEZE_ENR {
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
#[doc = "Possible values of the field `AGC_FREEZE_PRE_OR_AA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGC_FREEZE_PRE_OR_AAR {
    #[doc = "Access Address match (for active protocol)"]
    _0,
    #[doc = "Preamble Detect (for active protocol)"]
    _1,
}
impl AGC_FREEZE_PRE_OR_AAR {
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
            AGC_FREEZE_PRE_OR_AAR::_0 => false,
            AGC_FREEZE_PRE_OR_AAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AGC_FREEZE_PRE_OR_AAR {
        match value {
            false => AGC_FREEZE_PRE_OR_AAR::_0,
            true => AGC_FREEZE_PRE_OR_AAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AGC_FREEZE_PRE_OR_AAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AGC_FREEZE_PRE_OR_AAR::_1
    }
}
#[doc = r" Value of the field"]
pub struct AGC_UP_ENR {
    bits: bool,
}
impl AGC_UP_ENR {
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
#[doc = "Possible values of the field `AGC_UP_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGC_UP_SRCR {
    #[doc = "PDET LO"]
    _0,
    #[doc = "RSSI"]
    _1,
}
impl AGC_UP_SRCR {
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
            AGC_UP_SRCR::_0 => false,
            AGC_UP_SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AGC_UP_SRCR {
        match value {
            false => AGC_UP_SRCR::_0,
            true => AGC_UP_SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AGC_UP_SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AGC_UP_SRCR::_1
    }
}
#[doc = r" Value of the field"]
pub struct AGC_DOWN_BBA_STEP_SZR {
    bits: u8,
}
impl AGC_DOWN_BBA_STEP_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AGC_DOWN_LNA_STEP_SZR {
    bits: u8,
}
impl AGC_DOWN_LNA_STEP_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AGC_UP_RSSI_THRESHR {
    bits: u8,
}
impl AGC_UP_RSSI_THRESHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AGC_DOWN_RSSI_THRESHR {
    bits: u8,
}
impl AGC_DOWN_RSSI_THRESHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SLOW_AGC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOW_AGC_ENW<'a> {
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
#[doc = "Values that can be written to the field `SLOW_AGC_SRC`"]
pub enum SLOW_AGC_SRCW {
    #[doc = "Access Address match (for active protocol)"]
    _0,
    #[doc = "Preamble Detect (for active protocol)"]
    _1,
    #[doc = "Fast AGC expire timer"]
    _2,
}
impl SLOW_AGC_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SLOW_AGC_SRCW::_0 => 0,
            SLOW_AGC_SRCW::_1 => 1,
            SLOW_AGC_SRCW::_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLOW_AGC_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOW_AGC_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLOW_AGC_SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access Address match (for active protocol)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOW_AGC_SRCW::_0)
    }
    #[doc = "Preamble Detect (for active protocol)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOW_AGC_SRCW::_1)
    }
    #[doc = "Fast AGC expire timer"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(SLOW_AGC_SRCW::_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AGC_FREEZE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_FREEZE_ENW<'a> {
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
#[doc = "Values that can be written to the field `AGC_FREEZE_PRE_OR_AA`"]
pub enum AGC_FREEZE_PRE_OR_AAW {
    #[doc = "Access Address match (for active protocol)"]
    _0,
    #[doc = "Preamble Detect (for active protocol)"]
    _1,
}
impl AGC_FREEZE_PRE_OR_AAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AGC_FREEZE_PRE_OR_AAW::_0 => false,
            AGC_FREEZE_PRE_OR_AAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AGC_FREEZE_PRE_OR_AAW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_FREEZE_PRE_OR_AAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AGC_FREEZE_PRE_OR_AAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Access Address match (for active protocol)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGC_FREEZE_PRE_OR_AAW::_0)
    }
    #[doc = "Preamble Detect (for active protocol)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGC_FREEZE_PRE_OR_AAW::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AGC_UP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_UP_ENW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AGC_UP_SRC`"]
pub enum AGC_UP_SRCW {
    #[doc = "PDET LO"]
    _0,
    #[doc = "RSSI"]
    _1,
}
impl AGC_UP_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AGC_UP_SRCW::_0 => false,
            AGC_UP_SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AGC_UP_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_UP_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AGC_UP_SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDET LO"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGC_UP_SRCW::_0)
    }
    #[doc = "RSSI"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGC_UP_SRCW::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AGC_DOWN_BBA_STEP_SZW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_DOWN_BBA_STEP_SZW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AGC_DOWN_LNA_STEP_SZW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_DOWN_LNA_STEP_SZW<'a> {
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
pub struct _AGC_UP_RSSI_THRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_UP_RSSI_THRESHW<'a> {
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
#[doc = r" Proxy"]
pub struct _AGC_DOWN_RSSI_THRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_DOWN_RSSI_THRESHW<'a> {
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
    #[doc = "Bit 0 - Slow AGC Enable"]
    #[inline]
    pub fn slow_agc_en(&self) -> SLOW_AGC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLOW_AGC_ENR { bits }
    }
    #[doc = "Bits 1:2 - Slow AGC Source Selection"]
    #[inline]
    pub fn slow_agc_src(&self) -> SLOW_AGC_SRCR {
        SLOW_AGC_SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - AGC Freeze Enable"]
    #[inline]
    pub fn agc_freeze_en(&self) -> AGC_FREEZE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AGC_FREEZE_ENR { bits }
    }
    #[doc = "Bit 4 - AGC Freeze Source Selection"]
    #[inline]
    pub fn agc_freeze_pre_or_aa(&self) -> AGC_FREEZE_PRE_OR_AAR {
        AGC_FREEZE_PRE_OR_AAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - AGC Up Enable"]
    #[inline]
    pub fn agc_up_en(&self) -> AGC_UP_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AGC_UP_ENR { bits }
    }
    #[doc = "Bit 7 - AGC Up Source"]
    #[inline]
    pub fn agc_up_src(&self) -> AGC_UP_SRCR {
        AGC_UP_SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - AGC_DOWN_BBA_STEP_SZ"]
    #[inline]
    pub fn agc_down_bba_step_sz(&self) -> AGC_DOWN_BBA_STEP_SZR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AGC_DOWN_BBA_STEP_SZR { bits }
    }
    #[doc = "Bits 12:15 - AGC_DOWN_LNA_STEP_SZ"]
    #[inline]
    pub fn agc_down_lna_step_sz(&self) -> AGC_DOWN_LNA_STEP_SZR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AGC_DOWN_LNA_STEP_SZR { bits }
    }
    #[doc = "Bits 16:23 - AGC UP RSSI Threshold"]
    #[inline]
    pub fn agc_up_rssi_thresh(&self) -> AGC_UP_RSSI_THRESHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AGC_UP_RSSI_THRESHR { bits }
    }
    #[doc = "Bits 24:31 - AGC DOWN RSSI Threshold"]
    #[inline]
    pub fn agc_down_rssi_thresh(&self) -> AGC_DOWN_RSSI_THRESHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AGC_DOWN_RSSI_THRESHR { bits }
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
    #[doc = "Bit 0 - Slow AGC Enable"]
    #[inline]
    pub fn slow_agc_en(&mut self) -> _SLOW_AGC_ENW {
        _SLOW_AGC_ENW { w: self }
    }
    #[doc = "Bits 1:2 - Slow AGC Source Selection"]
    #[inline]
    pub fn slow_agc_src(&mut self) -> _SLOW_AGC_SRCW {
        _SLOW_AGC_SRCW { w: self }
    }
    #[doc = "Bit 3 - AGC Freeze Enable"]
    #[inline]
    pub fn agc_freeze_en(&mut self) -> _AGC_FREEZE_ENW {
        _AGC_FREEZE_ENW { w: self }
    }
    #[doc = "Bit 4 - AGC Freeze Source Selection"]
    #[inline]
    pub fn agc_freeze_pre_or_aa(&mut self) -> _AGC_FREEZE_PRE_OR_AAW {
        _AGC_FREEZE_PRE_OR_AAW { w: self }
    }
    #[doc = "Bit 6 - AGC Up Enable"]
    #[inline]
    pub fn agc_up_en(&mut self) -> _AGC_UP_ENW {
        _AGC_UP_ENW { w: self }
    }
    #[doc = "Bit 7 - AGC Up Source"]
    #[inline]
    pub fn agc_up_src(&mut self) -> _AGC_UP_SRCW {
        _AGC_UP_SRCW { w: self }
    }
    #[doc = "Bits 8:11 - AGC_DOWN_BBA_STEP_SZ"]
    #[inline]
    pub fn agc_down_bba_step_sz(&mut self) -> _AGC_DOWN_BBA_STEP_SZW {
        _AGC_DOWN_BBA_STEP_SZW { w: self }
    }
    #[doc = "Bits 12:15 - AGC_DOWN_LNA_STEP_SZ"]
    #[inline]
    pub fn agc_down_lna_step_sz(&mut self) -> _AGC_DOWN_LNA_STEP_SZW {
        _AGC_DOWN_LNA_STEP_SZW { w: self }
    }
    #[doc = "Bits 16:23 - AGC UP RSSI Threshold"]
    #[inline]
    pub fn agc_up_rssi_thresh(&mut self) -> _AGC_UP_RSSI_THRESHW {
        _AGC_UP_RSSI_THRESHW { w: self }
    }
    #[doc = "Bits 24:31 - AGC DOWN RSSI Threshold"]
    #[inline]
    pub fn agc_down_rssi_thresh(&mut self) -> _AGC_DOWN_RSSI_THRESHW {
        _AGC_DOWN_RSSI_THRESHW { w: self }
    }
}
