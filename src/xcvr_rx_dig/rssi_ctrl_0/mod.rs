#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RSSI_CTRL_0 {
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
pub struct RSSI_USE_VALSR {
    bits: bool,
}
impl RSSI_USE_VALSR {
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
#[doc = "Possible values of the field `RSSI_HOLD_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSI_HOLD_SRCR {
    #[doc = "Access Address match"]
    _00,
    #[doc = "Preamble Detect"]
    _01,
    #[doc = "802.15.4 LQI done (1=freeze, 0=run AGC)"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RSSI_HOLD_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSSI_HOLD_SRCR::_00 => 0,
            RSSI_HOLD_SRCR::_01 => 1,
            RSSI_HOLD_SRCR::_11 => 3,
            RSSI_HOLD_SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSSI_HOLD_SRCR {
        match value {
            0 => RSSI_HOLD_SRCR::_00,
            1 => RSSI_HOLD_SRCR::_01,
            3 => RSSI_HOLD_SRCR::_11,
            i => RSSI_HOLD_SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RSSI_HOLD_SRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RSSI_HOLD_SRCR::_01
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == RSSI_HOLD_SRCR::_11
    }
}
#[doc = r" Value of the field"]
pub struct RSSI_HOLD_ENR {
    bits: bool,
}
impl RSSI_HOLD_ENR {
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
#[doc = "Possible values of the field `RSSI_IIR_CW_WEIGHT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSI_IIR_CW_WEIGHTR {
    #[doc = "Bypass"]
    _0,
    #[doc = "1/8"]
    _1,
    #[doc = "1/16"]
    _2,
    #[doc = "1/32"]
    _3,
}
impl RSSI_IIR_CW_WEIGHTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSSI_IIR_CW_WEIGHTR::_0 => 0,
            RSSI_IIR_CW_WEIGHTR::_1 => 1,
            RSSI_IIR_CW_WEIGHTR::_2 => 2,
            RSSI_IIR_CW_WEIGHTR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSSI_IIR_CW_WEIGHTR {
        match value {
            0 => RSSI_IIR_CW_WEIGHTR::_0,
            1 => RSSI_IIR_CW_WEIGHTR::_1,
            2 => RSSI_IIR_CW_WEIGHTR::_2,
            3 => RSSI_IIR_CW_WEIGHTR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RSSI_IIR_CW_WEIGHTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSSI_IIR_CW_WEIGHTR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == RSSI_IIR_CW_WEIGHTR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == RSSI_IIR_CW_WEIGHTR::_3
    }
}
#[doc = "Possible values of the field `RSSI_N_WINDOW_AVG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSI_N_WINDOW_AVGR {
    #[doc = "No averaging"]
    _0,
    #[doc = "Averaging window length is 2 samples"]
    _1,
    #[doc = "Averaging window length is 4 samples"]
    _2,
    #[doc = "Averaging window length is 8 samples"]
    _3,
}
impl RSSI_N_WINDOW_AVGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSSI_N_WINDOW_AVGR::_0 => 0,
            RSSI_N_WINDOW_AVGR::_1 => 1,
            RSSI_N_WINDOW_AVGR::_2 => 2,
            RSSI_N_WINDOW_AVGR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSSI_N_WINDOW_AVGR {
        match value {
            0 => RSSI_N_WINDOW_AVGR::_0,
            1 => RSSI_N_WINDOW_AVGR::_1,
            2 => RSSI_N_WINDOW_AVGR::_2,
            3 => RSSI_N_WINDOW_AVGR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RSSI_N_WINDOW_AVGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSSI_N_WINDOW_AVGR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == RSSI_N_WINDOW_AVGR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == RSSI_N_WINDOW_AVGR::_3
    }
}
#[doc = r" Value of the field"]
pub struct RSSI_HOLD_DELAYR {
    bits: u8,
}
impl RSSI_HOLD_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RSSI_IIR_WEIGHT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSI_IIR_WEIGHTR {
    #[doc = "Bypass"]
    _0,
    #[doc = "1/2"]
    _1,
    #[doc = "1/4"]
    _2,
    #[doc = "1/8"]
    _3,
    #[doc = "1/16"]
    _4,
    #[doc = "1/32"]
    _5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RSSI_IIR_WEIGHTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSSI_IIR_WEIGHTR::_0 => 0,
            RSSI_IIR_WEIGHTR::_1 => 1,
            RSSI_IIR_WEIGHTR::_2 => 2,
            RSSI_IIR_WEIGHTR::_3 => 3,
            RSSI_IIR_WEIGHTR::_4 => 4,
            RSSI_IIR_WEIGHTR::_5 => 5,
            RSSI_IIR_WEIGHTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSSI_IIR_WEIGHTR {
        match value {
            0 => RSSI_IIR_WEIGHTR::_0,
            1 => RSSI_IIR_WEIGHTR::_1,
            2 => RSSI_IIR_WEIGHTR::_2,
            3 => RSSI_IIR_WEIGHTR::_3,
            4 => RSSI_IIR_WEIGHTR::_4,
            5 => RSSI_IIR_WEIGHTR::_5,
            i => RSSI_IIR_WEIGHTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RSSI_IIR_WEIGHTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSSI_IIR_WEIGHTR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == RSSI_IIR_WEIGHTR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == RSSI_IIR_WEIGHTR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == RSSI_IIR_WEIGHTR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == RSSI_IIR_WEIGHTR::_5
    }
}
#[doc = r" Value of the field"]
pub struct RSSI_VLD_SETTLER {
    bits: u8,
}
impl RSSI_VLD_SETTLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSSI_ADJR {
    bits: u8,
}
impl RSSI_ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RSSI_USE_VALSW<'a> {
    w: &'a mut W,
}
impl<'a> _RSSI_USE_VALSW<'a> {
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
#[doc = "Values that can be written to the field `RSSI_HOLD_SRC`"]
pub enum RSSI_HOLD_SRCW {
    #[doc = "Access Address match"]
    _00,
    #[doc = "Preamble Detect"]
    _01,
    #[doc = "802.15.4 LQI done (1=freeze, 0=run AGC)"]
    _11,
}
impl RSSI_HOLD_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSSI_HOLD_SRCW::_00 => 0,
            RSSI_HOLD_SRCW::_01 => 1,
            RSSI_HOLD_SRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSSI_HOLD_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _RSSI_HOLD_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSSI_HOLD_SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access Address match"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RSSI_HOLD_SRCW::_00)
    }
    #[doc = "Preamble Detect"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RSSI_HOLD_SRCW::_01)
    }
    #[doc = "802.15.4 LQI done (1=freeze, 0=run AGC)"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RSSI_HOLD_SRCW::_11)
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
pub struct _RSSI_HOLD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RSSI_HOLD_ENW<'a> {
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
#[doc = "Values that can be written to the field `RSSI_IIR_CW_WEIGHT`"]
pub enum RSSI_IIR_CW_WEIGHTW {
    #[doc = "Bypass"]
    _0,
    #[doc = "1/8"]
    _1,
    #[doc = "1/16"]
    _2,
    #[doc = "1/32"]
    _3,
}
impl RSSI_IIR_CW_WEIGHTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSSI_IIR_CW_WEIGHTW::_0 => 0,
            RSSI_IIR_CW_WEIGHTW::_1 => 1,
            RSSI_IIR_CW_WEIGHTW::_2 => 2,
            RSSI_IIR_CW_WEIGHTW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSSI_IIR_CW_WEIGHTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSSI_IIR_CW_WEIGHTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSSI_IIR_CW_WEIGHTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bypass"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSSI_IIR_CW_WEIGHTW::_0)
    }
    #[doc = "1/8"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSSI_IIR_CW_WEIGHTW::_1)
    }
    #[doc = "1/16"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(RSSI_IIR_CW_WEIGHTW::_2)
    }
    #[doc = "1/32"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(RSSI_IIR_CW_WEIGHTW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSSI_N_WINDOW_AVG`"]
pub enum RSSI_N_WINDOW_AVGW {
    #[doc = "No averaging"]
    _0,
    #[doc = "Averaging window length is 2 samples"]
    _1,
    #[doc = "Averaging window length is 4 samples"]
    _2,
    #[doc = "Averaging window length is 8 samples"]
    _3,
}
impl RSSI_N_WINDOW_AVGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSSI_N_WINDOW_AVGW::_0 => 0,
            RSSI_N_WINDOW_AVGW::_1 => 1,
            RSSI_N_WINDOW_AVGW::_2 => 2,
            RSSI_N_WINDOW_AVGW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSSI_N_WINDOW_AVGW<'a> {
    w: &'a mut W,
}
impl<'a> _RSSI_N_WINDOW_AVGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSSI_N_WINDOW_AVGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No averaging"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSSI_N_WINDOW_AVGW::_0)
    }
    #[doc = "Averaging window length is 2 samples"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSSI_N_WINDOW_AVGW::_1)
    }
    #[doc = "Averaging window length is 4 samples"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(RSSI_N_WINDOW_AVGW::_2)
    }
    #[doc = "Averaging window length is 8 samples"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(RSSI_N_WINDOW_AVGW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSSI_HOLD_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _RSSI_HOLD_DELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSSI_IIR_WEIGHT`"]
pub enum RSSI_IIR_WEIGHTW {
    #[doc = "Bypass"]
    _0,
    #[doc = "1/2"]
    _1,
    #[doc = "1/4"]
    _2,
    #[doc = "1/8"]
    _3,
    #[doc = "1/16"]
    _4,
    #[doc = "1/32"]
    _5,
}
impl RSSI_IIR_WEIGHTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSSI_IIR_WEIGHTW::_0 => 0,
            RSSI_IIR_WEIGHTW::_1 => 1,
            RSSI_IIR_WEIGHTW::_2 => 2,
            RSSI_IIR_WEIGHTW::_3 => 3,
            RSSI_IIR_WEIGHTW::_4 => 4,
            RSSI_IIR_WEIGHTW::_5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSSI_IIR_WEIGHTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSSI_IIR_WEIGHTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSSI_IIR_WEIGHTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Bypass"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSSI_IIR_WEIGHTW::_0)
    }
    #[doc = "1/2"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSSI_IIR_WEIGHTW::_1)
    }
    #[doc = "1/4"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(RSSI_IIR_WEIGHTW::_2)
    }
    #[doc = "1/8"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(RSSI_IIR_WEIGHTW::_3)
    }
    #[doc = "1/16"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(RSSI_IIR_WEIGHTW::_4)
    }
    #[doc = "1/32"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(RSSI_IIR_WEIGHTW::_5)
    }
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
pub struct _RSSI_VLD_SETTLEW<'a> {
    w: &'a mut W,
}
impl<'a> _RSSI_VLD_SETTLEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSSI_ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _RSSI_ADJW<'a> {
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
    #[doc = "Bit 0 - RSSI Values Selection"]
    #[inline]
    pub fn rssi_use_vals(&self) -> RSSI_USE_VALSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSSI_USE_VALSR { bits }
    }
    #[doc = "Bits 1:2 - RSSI Hold Source Selection"]
    #[inline]
    pub fn rssi_hold_src(&self) -> RSSI_HOLD_SRCR {
        RSSI_HOLD_SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - RSSI Hold Enable"]
    #[inline]
    pub fn rssi_hold_en(&self) -> RSSI_HOLD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSSI_HOLD_ENR { bits }
    }
    #[doc = "Bits 5:6 - RSSI IIR CW Weighting"]
    #[inline]
    pub fn rssi_iir_cw_weight(&self) -> RSSI_IIR_CW_WEIGHTR {
        RSSI_IIR_CW_WEIGHTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - RSSI N Window Average"]
    #[inline]
    pub fn rssi_n_window_avg(&self) -> RSSI_N_WINDOW_AVGR {
        RSSI_N_WINDOW_AVGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:15 - RSSI Hold Delay"]
    #[inline]
    pub fn rssi_hold_delay(&self) -> RSSI_HOLD_DELAYR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSSI_HOLD_DELAYR { bits }
    }
    #[doc = "Bits 16:19 - RSSI IIR Weighting"]
    #[inline]
    pub fn rssi_iir_weight(&self) -> RSSI_IIR_WEIGHTR {
        RSSI_IIR_WEIGHTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - RSSI Valid Settle"]
    #[inline]
    pub fn rssi_vld_settle(&self) -> RSSI_VLD_SETTLER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSSI_VLD_SETTLER { bits }
    }
    #[doc = "Bits 24:31 - RSSI Adjustment"]
    #[inline]
    pub fn rssi_adj(&self) -> RSSI_ADJR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSSI_ADJR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3145728 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - RSSI Values Selection"]
    #[inline]
    pub fn rssi_use_vals(&mut self) -> _RSSI_USE_VALSW {
        _RSSI_USE_VALSW { w: self }
    }
    #[doc = "Bits 1:2 - RSSI Hold Source Selection"]
    #[inline]
    pub fn rssi_hold_src(&mut self) -> _RSSI_HOLD_SRCW {
        _RSSI_HOLD_SRCW { w: self }
    }
    #[doc = "Bit 3 - RSSI Hold Enable"]
    #[inline]
    pub fn rssi_hold_en(&mut self) -> _RSSI_HOLD_ENW {
        _RSSI_HOLD_ENW { w: self }
    }
    #[doc = "Bits 5:6 - RSSI IIR CW Weighting"]
    #[inline]
    pub fn rssi_iir_cw_weight(&mut self) -> _RSSI_IIR_CW_WEIGHTW {
        _RSSI_IIR_CW_WEIGHTW { w: self }
    }
    #[doc = "Bits 8:9 - RSSI N Window Average"]
    #[inline]
    pub fn rssi_n_window_avg(&mut self) -> _RSSI_N_WINDOW_AVGW {
        _RSSI_N_WINDOW_AVGW { w: self }
    }
    #[doc = "Bits 10:15 - RSSI Hold Delay"]
    #[inline]
    pub fn rssi_hold_delay(&mut self) -> _RSSI_HOLD_DELAYW {
        _RSSI_HOLD_DELAYW { w: self }
    }
    #[doc = "Bits 16:19 - RSSI IIR Weighting"]
    #[inline]
    pub fn rssi_iir_weight(&mut self) -> _RSSI_IIR_WEIGHTW {
        _RSSI_IIR_WEIGHTW { w: self }
    }
    #[doc = "Bits 20:22 - RSSI Valid Settle"]
    #[inline]
    pub fn rssi_vld_settle(&mut self) -> _RSSI_VLD_SETTLEW {
        _RSSI_VLD_SETTLEW { w: self }
    }
    #[doc = "Bits 24:31 - RSSI Adjustment"]
    #[inline]
    pub fn rssi_adj(&mut self) -> _RSSI_ADJW {
        _RSSI_ADJW { w: self }
    }
}
