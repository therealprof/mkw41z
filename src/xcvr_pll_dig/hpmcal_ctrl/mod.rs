#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPMCAL_CTRL {
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
pub struct HPM_CAL_FACTORR {
    bits: u16,
}
impl HPM_CAL_FACTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPM_CAL_NOT_BUMPEDR {
    bits: bool,
}
impl HPM_CAL_NOT_BUMPEDR {
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
pub struct HPM_CAL_COUNT_SCALER {
    bits: bool,
}
impl HPM_CAL_COUNT_SCALER {
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
pub struct HP_CAL_DISABLER {
    bits: bool,
}
impl HP_CAL_DISABLER {
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
pub struct HPM_CAL_FACTOR_MANUALR {
    bits: u16,
}
impl HPM_CAL_FACTOR_MANUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `HPM_CAL_ARRAY_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPM_CAL_ARRAY_SIZER {
    #[doc = "128"]
    _0,
    #[doc = "256"]
    _1,
}
impl HPM_CAL_ARRAY_SIZER {
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
            HPM_CAL_ARRAY_SIZER::_0 => false,
            HPM_CAL_ARRAY_SIZER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPM_CAL_ARRAY_SIZER {
        match value {
            false => HPM_CAL_ARRAY_SIZER::_0,
            true => HPM_CAL_ARRAY_SIZER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HPM_CAL_ARRAY_SIZER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HPM_CAL_ARRAY_SIZER::_1
    }
}
#[doc = "Possible values of the field `HPM_CAL_TIME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPM_CAL_TIMER {
    #[doc = "25 us"]
    _0,
    #[doc = "50 us"]
    _1,
}
impl HPM_CAL_TIMER {
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
            HPM_CAL_TIMER::_0 => false,
            HPM_CAL_TIMER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPM_CAL_TIMER {
        match value {
            false => HPM_CAL_TIMER::_0,
            true => HPM_CAL_TIMER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HPM_CAL_TIMER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HPM_CAL_TIMER::_1
    }
}
#[doc = r" Proxy"]
pub struct _HPM_CAL_NOT_BUMPEDW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_CAL_NOT_BUMPEDW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_CAL_COUNT_SCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_CAL_COUNT_SCALEW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HP_CAL_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _HP_CAL_DISABLEW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_CAL_FACTOR_MANUALW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_CAL_FACTOR_MANUALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HPM_CAL_ARRAY_SIZE`"]
pub enum HPM_CAL_ARRAY_SIZEW {
    #[doc = "128"]
    _0,
    #[doc = "256"]
    _1,
}
impl HPM_CAL_ARRAY_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPM_CAL_ARRAY_SIZEW::_0 => false,
            HPM_CAL_ARRAY_SIZEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPM_CAL_ARRAY_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_CAL_ARRAY_SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPM_CAL_ARRAY_SIZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "128"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HPM_CAL_ARRAY_SIZEW::_0)
    }
    #[doc = "256"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HPM_CAL_ARRAY_SIZEW::_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HPM_CAL_TIME`"]
pub enum HPM_CAL_TIMEW {
    #[doc = "25 us"]
    _0,
    #[doc = "50 us"]
    _1,
}
impl HPM_CAL_TIMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPM_CAL_TIMEW::_0 => false,
            HPM_CAL_TIMEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPM_CAL_TIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_CAL_TIMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPM_CAL_TIMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "25 us"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HPM_CAL_TIMEW::_0)
    }
    #[doc = "50 us"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HPM_CAL_TIMEW::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:12 - High Port Modulation Calibration Factor"]
    #[inline]
    pub fn hpm_cal_factor(&self) -> HPM_CAL_FACTORR {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HPM_CAL_FACTORR { bits }
    }
    #[doc = "Bit 13 - HPM_CAL_NOT_BUMPED"]
    #[inline]
    pub fn hpm_cal_not_bumped(&self) -> HPM_CAL_NOT_BUMPEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_CAL_NOT_BUMPEDR { bits }
    }
    #[doc = "Bit 14 - HPM_CAL_COUNT_SCALE"]
    #[inline]
    pub fn hpm_cal_count_scale(&self) -> HPM_CAL_COUNT_SCALER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_CAL_COUNT_SCALER { bits }
    }
    #[doc = "Bit 15 - Disable HPM Manual Calibration"]
    #[inline]
    pub fn hp_cal_disable(&self) -> HP_CAL_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HP_CAL_DISABLER { bits }
    }
    #[doc = "Bits 16:28 - Manual HPM Calibration Factor"]
    #[inline]
    pub fn hpm_cal_factor_manual(&self) -> HPM_CAL_FACTOR_MANUALR {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HPM_CAL_FACTOR_MANUALR { bits }
    }
    #[doc = "Bit 30 - High Port Modulation Calibration Array Size"]
    #[inline]
    pub fn hpm_cal_array_size(&self) -> HPM_CAL_ARRAY_SIZER {
        HPM_CAL_ARRAY_SIZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - High Port Modulation Calibration Time"]
    #[inline]
    pub fn hpm_cal_time(&self) -> HPM_CAL_TIMER {
        HPM_CAL_TIMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1073742369 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 13 - HPM_CAL_NOT_BUMPED"]
    #[inline]
    pub fn hpm_cal_not_bumped(&mut self) -> _HPM_CAL_NOT_BUMPEDW {
        _HPM_CAL_NOT_BUMPEDW { w: self }
    }
    #[doc = "Bit 14 - HPM_CAL_COUNT_SCALE"]
    #[inline]
    pub fn hpm_cal_count_scale(&mut self) -> _HPM_CAL_COUNT_SCALEW {
        _HPM_CAL_COUNT_SCALEW { w: self }
    }
    #[doc = "Bit 15 - Disable HPM Manual Calibration"]
    #[inline]
    pub fn hp_cal_disable(&mut self) -> _HP_CAL_DISABLEW {
        _HP_CAL_DISABLEW { w: self }
    }
    #[doc = "Bits 16:28 - Manual HPM Calibration Factor"]
    #[inline]
    pub fn hpm_cal_factor_manual(&mut self) -> _HPM_CAL_FACTOR_MANUALW {
        _HPM_CAL_FACTOR_MANUALW { w: self }
    }
    #[doc = "Bit 30 - High Port Modulation Calibration Array Size"]
    #[inline]
    pub fn hpm_cal_array_size(&mut self) -> _HPM_CAL_ARRAY_SIZEW {
        _HPM_CAL_ARRAY_SIZEW { w: self }
    }
    #[doc = "Bit 31 - High Port Modulation Calibration Time"]
    #[inline]
    pub fn hpm_cal_time(&mut self) -> _HPM_CAL_TIMEW {
        _HPM_CAL_TIMEW { w: self }
    }
}
