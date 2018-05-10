#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCOC_CTRL_0 {
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
#[doc = "Possible values of the field `DCOC_MIDPWR_TRK_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_MIDPWR_TRK_DISR {
    #[doc = "Tracking corrections are enabled as determined by DCOC_CORRECT_SRC and DCOC_TRK_MIN_AGC_IDX."]
    _0,
    #[doc = "Tracking corrections are disabled when either the TZA or BBA lo peak detector asserts."]
    _1,
}
impl DCOC_MIDPWR_TRK_DISR {
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
            DCOC_MIDPWR_TRK_DISR::_0 => false,
            DCOC_MIDPWR_TRK_DISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCOC_MIDPWR_TRK_DISR {
        match value {
            false => DCOC_MIDPWR_TRK_DISR::_0,
            true => DCOC_MIDPWR_TRK_DISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCOC_MIDPWR_TRK_DISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCOC_MIDPWR_TRK_DISR::_1
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_MANR {
    bits: bool,
}
impl DCOC_MANR {
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
#[doc = "Possible values of the field `DCOC_TRK_EST_OVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_TRK_EST_OVRR {
    #[doc = "The tracking estimator is enabled only as needed by the corrector"]
    _0,
    #[doc = "The tracking estimator remains enabled whenever the DCOC is active"]
    _1,
}
impl DCOC_TRK_EST_OVRR {
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
            DCOC_TRK_EST_OVRR::_0 => false,
            DCOC_TRK_EST_OVRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCOC_TRK_EST_OVRR {
        match value {
            false => DCOC_TRK_EST_OVRR::_0,
            true => DCOC_TRK_EST_OVRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCOC_TRK_EST_OVRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCOC_TRK_EST_OVRR::_1
    }
}
#[doc = "Possible values of the field `DCOC_CORRECT_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_CORRECT_SRCR {
    #[doc = "If correction is enabled, the DCOC will use only the DCOC calibration table to correct the DC offset."]
    _0,
    #[doc = "If correction is enabled, the DCOC will use the DCOC calibration table and then the tracking estimator to correct the DC offset."]
    _1,
}
impl DCOC_CORRECT_SRCR {
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
            DCOC_CORRECT_SRCR::_0 => false,
            DCOC_CORRECT_SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCOC_CORRECT_SRCR {
        match value {
            false => DCOC_CORRECT_SRCR::_0,
            true => DCOC_CORRECT_SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCOC_CORRECT_SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCOC_CORRECT_SRCR::_1
    }
}
#[doc = "Possible values of the field `DCOC_CORRECT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_CORRECT_ENR {
    #[doc = "Correction disabled. The DCOC will not correct the DC offset."]
    _0,
    #[doc = "Correction enabled. The DCOC will use the TZA and BBA DACs, and apply digital corrections (if DCOC_CORRECT_SRC=1) to correct the DC offset."]
    _1,
}
impl DCOC_CORRECT_ENR {
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
            DCOC_CORRECT_ENR::_0 => false,
            DCOC_CORRECT_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCOC_CORRECT_ENR {
        match value {
            false => DCOC_CORRECT_ENR::_0,
            true => DCOC_CORRECT_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCOC_CORRECT_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCOC_CORRECT_ENR::_1
    }
}
#[doc = "Possible values of the field `TRACK_FROM_ZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACK_FROM_ZEROR {
    #[doc = "Track from current I/Q sample."]
    _0,
    #[doc = "Track from zero."]
    _1,
}
impl TRACK_FROM_ZEROR {
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
            TRACK_FROM_ZEROR::_0 => false,
            TRACK_FROM_ZEROR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRACK_FROM_ZEROR {
        match value {
            false => TRACK_FROM_ZEROR::_0,
            true => TRACK_FROM_ZEROR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRACK_FROM_ZEROR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRACK_FROM_ZEROR::_1
    }
}
#[doc = "Possible values of the field `BBA_CORR_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBA_CORR_POLR {
    #[doc = "Normal polarity."]
    _0,
    #[doc = "Negative polarity. This should be set if the ADC output is inverted, or if the BBA DACs were implemented with negative polarity."]
    _1,
}
impl BBA_CORR_POLR {
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
            BBA_CORR_POLR::_0 => false,
            BBA_CORR_POLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BBA_CORR_POLR {
        match value {
            false => BBA_CORR_POLR::_0,
            true => BBA_CORR_POLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BBA_CORR_POLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BBA_CORR_POLR::_1
    }
}
#[doc = "Possible values of the field `TZA_CORR_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZA_CORR_POLR {
    #[doc = "Normal polarity."]
    _0,
    #[doc = "Negative polarity. This should be set if the ADC output is inverted, or if the TZA DACs were implemented with negative polarity."]
    _1,
}
impl TZA_CORR_POLR {
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
            TZA_CORR_POLR::_0 => false,
            TZA_CORR_POLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TZA_CORR_POLR {
        match value {
            false => TZA_CORR_POLR::_0,
            true => TZA_CORR_POLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TZA_CORR_POLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TZA_CORR_POLR::_1
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_CAL_DURATIONR {
    bits: u8,
}
impl DCOC_CAL_DURATIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_CORR_DLYR {
    bits: u8,
}
impl DCOC_CORR_DLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DCOC_CORR_HOLD_TIME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_CORR_HOLD_TIMER {
    #[doc = "The DC correction is not frozen."]
    _127,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DCOC_CORR_HOLD_TIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCOC_CORR_HOLD_TIMER::_127 => 127,
            DCOC_CORR_HOLD_TIMER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCOC_CORR_HOLD_TIMER {
        match value {
            127 => DCOC_CORR_HOLD_TIMER::_127,
            i => DCOC_CORR_HOLD_TIMER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_127`"]
    #[inline]
    pub fn is_127(&self) -> bool {
        *self == DCOC_CORR_HOLD_TIMER::_127
    }
}
#[doc = "Values that can be written to the field `DCOC_MIDPWR_TRK_DIS`"]
pub enum DCOC_MIDPWR_TRK_DISW {
    #[doc = "Tracking corrections are enabled as determined by DCOC_CORRECT_SRC and DCOC_TRK_MIN_AGC_IDX."]
    _0,
    #[doc = "Tracking corrections are disabled when either the TZA or BBA lo peak detector asserts."]
    _1,
}
impl DCOC_MIDPWR_TRK_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCOC_MIDPWR_TRK_DISW::_0 => false,
            DCOC_MIDPWR_TRK_DISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_MIDPWR_TRK_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_MIDPWR_TRK_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_MIDPWR_TRK_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Tracking corrections are enabled as determined by DCOC_CORRECT_SRC and DCOC_TRK_MIN_AGC_IDX."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCOC_MIDPWR_TRK_DISW::_0)
    }
    #[doc = "Tracking corrections are disabled when either the TZA or BBA lo peak detector asserts."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCOC_MIDPWR_TRK_DISW::_1)
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
#[doc = r" Proxy"]
pub struct _DCOC_MANW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_MANW<'a> {
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
#[doc = "Values that can be written to the field `DCOC_TRK_EST_OVR`"]
pub enum DCOC_TRK_EST_OVRW {
    #[doc = "The tracking estimator is enabled only as needed by the corrector"]
    _0,
    #[doc = "The tracking estimator remains enabled whenever the DCOC is active"]
    _1,
}
impl DCOC_TRK_EST_OVRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCOC_TRK_EST_OVRW::_0 => false,
            DCOC_TRK_EST_OVRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_TRK_EST_OVRW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_TRK_EST_OVRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_TRK_EST_OVRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The tracking estimator is enabled only as needed by the corrector"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCOC_TRK_EST_OVRW::_0)
    }
    #[doc = "The tracking estimator remains enabled whenever the DCOC is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCOC_TRK_EST_OVRW::_1)
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
#[doc = "Values that can be written to the field `DCOC_CORRECT_SRC`"]
pub enum DCOC_CORRECT_SRCW {
    #[doc = "If correction is enabled, the DCOC will use only the DCOC calibration table to correct the DC offset."]
    _0,
    #[doc = "If correction is enabled, the DCOC will use the DCOC calibration table and then the tracking estimator to correct the DC offset."]
    _1,
}
impl DCOC_CORRECT_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCOC_CORRECT_SRCW::_0 => false,
            DCOC_CORRECT_SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_CORRECT_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_CORRECT_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_CORRECT_SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If correction is enabled, the DCOC will use only the DCOC calibration table to correct the DC offset."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCOC_CORRECT_SRCW::_0)
    }
    #[doc = "If correction is enabled, the DCOC will use the DCOC calibration table and then the tracking estimator to correct the DC offset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCOC_CORRECT_SRCW::_1)
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
#[doc = "Values that can be written to the field `DCOC_CORRECT_EN`"]
pub enum DCOC_CORRECT_ENW {
    #[doc = "Correction disabled. The DCOC will not correct the DC offset."]
    _0,
    #[doc = "Correction enabled. The DCOC will use the TZA and BBA DACs, and apply digital corrections (if DCOC_CORRECT_SRC=1) to correct the DC offset."]
    _1,
}
impl DCOC_CORRECT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCOC_CORRECT_ENW::_0 => false,
            DCOC_CORRECT_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_CORRECT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_CORRECT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_CORRECT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Correction disabled. The DCOC will not correct the DC offset."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCOC_CORRECT_ENW::_0)
    }
    #[doc = "Correction enabled. The DCOC will use the TZA and BBA DACs, and apply digital corrections (if DCOC_CORRECT_SRC=1) to correct the DC offset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCOC_CORRECT_ENW::_1)
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
#[doc = "Values that can be written to the field `TRACK_FROM_ZERO`"]
pub enum TRACK_FROM_ZEROW {
    #[doc = "Track from current I/Q sample."]
    _0,
    #[doc = "Track from zero."]
    _1,
}
impl TRACK_FROM_ZEROW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRACK_FROM_ZEROW::_0 => false,
            TRACK_FROM_ZEROW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRACK_FROM_ZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACK_FROM_ZEROW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRACK_FROM_ZEROW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Track from current I/Q sample."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRACK_FROM_ZEROW::_0)
    }
    #[doc = "Track from zero."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRACK_FROM_ZEROW::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BBA_CORR_POL`"]
pub enum BBA_CORR_POLW {
    #[doc = "Normal polarity."]
    _0,
    #[doc = "Negative polarity. This should be set if the ADC output is inverted, or if the BBA DACs were implemented with negative polarity."]
    _1,
}
impl BBA_CORR_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BBA_CORR_POLW::_0 => false,
            BBA_CORR_POLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BBA_CORR_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_CORR_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BBA_CORR_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal polarity."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BBA_CORR_POLW::_0)
    }
    #[doc = "Negative polarity. This should be set if the ADC output is inverted, or if the BBA DACs were implemented with negative polarity."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BBA_CORR_POLW::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TZA_CORR_POL`"]
pub enum TZA_CORR_POLW {
    #[doc = "Normal polarity."]
    _0,
    #[doc = "Negative polarity. This should be set if the ADC output is inverted, or if the TZA DACs were implemented with negative polarity."]
    _1,
}
impl TZA_CORR_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TZA_CORR_POLW::_0 => false,
            TZA_CORR_POLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TZA_CORR_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TZA_CORR_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TZA_CORR_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal polarity."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TZA_CORR_POLW::_0)
    }
    #[doc = "Negative polarity. This should be set if the ADC output is inverted, or if the TZA DACs were implemented with negative polarity."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TZA_CORR_POLW::_1)
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
pub struct _DCOC_CAL_DURATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_CAL_DURATIONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_CORR_DLYW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_CORR_DLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCOC_CORR_HOLD_TIME`"]
pub enum DCOC_CORR_HOLD_TIMEW {
    #[doc = "The DC correction is not frozen."]
    _127,
}
impl DCOC_CORR_HOLD_TIMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCOC_CORR_HOLD_TIMEW::_127 => 127,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_CORR_HOLD_TIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_CORR_HOLD_TIMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_CORR_HOLD_TIMEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The DC correction is not frozen."]
    #[inline]
    pub fn _127(self) -> &'a mut W {
        self.variant(DCOC_CORR_HOLD_TIMEW::_127)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
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
    #[doc = "Bit 0 - DCOC Mid Power Tracking Disable"]
    #[inline]
    pub fn dcoc_midpwr_trk_dis(&self) -> DCOC_MIDPWR_TRK_DISR {
        DCOC_MIDPWR_TRK_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DCOC Manual Override"]
    #[inline]
    pub fn dcoc_man(&self) -> DCOC_MANR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCOC_MANR { bits }
    }
    #[doc = "Bit 2 - Override for the DCOC tracking estimator"]
    #[inline]
    pub fn dcoc_trk_est_ovr(&self) -> DCOC_TRK_EST_OVRR {
        DCOC_TRK_EST_OVRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - DCOC Corrector Source"]
    #[inline]
    pub fn dcoc_correct_src(&self) -> DCOC_CORRECT_SRCR {
        DCOC_CORRECT_SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - DCOC Correction Enable"]
    #[inline]
    pub fn dcoc_correct_en(&self) -> DCOC_CORRECT_ENR {
        DCOC_CORRECT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Track from Zero"]
    #[inline]
    pub fn track_from_zero(&self) -> TRACK_FROM_ZEROR {
        TRACK_FROM_ZEROR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - BBA Correction Polarity"]
    #[inline]
    pub fn bba_corr_pol(&self) -> BBA_CORR_POLR {
        BBA_CORR_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - TZA Correction Polarity"]
    #[inline]
    pub fn tza_corr_pol(&self) -> TZA_CORR_POLR {
        TZA_CORR_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - DCOC Calibration Duration"]
    #[inline]
    pub fn dcoc_cal_duration(&self) -> DCOC_CAL_DURATIONR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_CAL_DURATIONR { bits }
    }
    #[doc = "Bits 16:20 - DCOC Correction Delay"]
    #[inline]
    pub fn dcoc_corr_dly(&self) -> DCOC_CORR_DLYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_CORR_DLYR { bits }
    }
    #[doc = "Bits 24:30 - DCOC Correction Hold Time"]
    #[inline]
    pub fn dcoc_corr_hold_time(&self) -> DCOC_CORR_HOLD_TIMER {
        DCOC_CORR_HOLD_TIMER::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 0 - DCOC Mid Power Tracking Disable"]
    #[inline]
    pub fn dcoc_midpwr_trk_dis(&mut self) -> _DCOC_MIDPWR_TRK_DISW {
        _DCOC_MIDPWR_TRK_DISW { w: self }
    }
    #[doc = "Bit 1 - DCOC Manual Override"]
    #[inline]
    pub fn dcoc_man(&mut self) -> _DCOC_MANW {
        _DCOC_MANW { w: self }
    }
    #[doc = "Bit 2 - Override for the DCOC tracking estimator"]
    #[inline]
    pub fn dcoc_trk_est_ovr(&mut self) -> _DCOC_TRK_EST_OVRW {
        _DCOC_TRK_EST_OVRW { w: self }
    }
    #[doc = "Bit 3 - DCOC Corrector Source"]
    #[inline]
    pub fn dcoc_correct_src(&mut self) -> _DCOC_CORRECT_SRCW {
        _DCOC_CORRECT_SRCW { w: self }
    }
    #[doc = "Bit 4 - DCOC Correction Enable"]
    #[inline]
    pub fn dcoc_correct_en(&mut self) -> _DCOC_CORRECT_ENW {
        _DCOC_CORRECT_ENW { w: self }
    }
    #[doc = "Bit 5 - Track from Zero"]
    #[inline]
    pub fn track_from_zero(&mut self) -> _TRACK_FROM_ZEROW {
        _TRACK_FROM_ZEROW { w: self }
    }
    #[doc = "Bit 6 - BBA Correction Polarity"]
    #[inline]
    pub fn bba_corr_pol(&mut self) -> _BBA_CORR_POLW {
        _BBA_CORR_POLW { w: self }
    }
    #[doc = "Bit 7 - TZA Correction Polarity"]
    #[inline]
    pub fn tza_corr_pol(&mut self) -> _TZA_CORR_POLW {
        _TZA_CORR_POLW { w: self }
    }
    #[doc = "Bits 8:12 - DCOC Calibration Duration"]
    #[inline]
    pub fn dcoc_cal_duration(&mut self) -> _DCOC_CAL_DURATIONW {
        _DCOC_CAL_DURATIONW { w: self }
    }
    #[doc = "Bits 16:20 - DCOC Correction Delay"]
    #[inline]
    pub fn dcoc_corr_dly(&mut self) -> _DCOC_CORR_DLYW {
        _DCOC_CORR_DLYW { w: self }
    }
    #[doc = "Bits 24:30 - DCOC Correction Hold Time"]
    #[inline]
    pub fn dcoc_corr_hold_time(&mut self) -> _DCOC_CORR_HOLD_TIMEW {
        _DCOC_CORR_HOLD_TIMEW { w: self }
    }
}
