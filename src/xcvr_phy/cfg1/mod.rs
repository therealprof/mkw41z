#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG1 {
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
pub struct AA_PLAYBACKR {
    bits: bool,
}
impl AA_PLAYBACKR {
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
#[doc = "Possible values of the field `AA_OUTPUT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AA_OUTPUT_SELR {
    #[doc = "demodulated"]
    _0,
    #[doc = "matched"]
    _1,
}
impl AA_OUTPUT_SELR {
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
            AA_OUTPUT_SELR::_0 => false,
            AA_OUTPUT_SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AA_OUTPUT_SELR {
        match value {
            false => AA_OUTPUT_SELR::_0,
            true => AA_OUTPUT_SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AA_OUTPUT_SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AA_OUTPUT_SELR::_1
    }
}
#[doc = r" Value of the field"]
pub struct FSK_BIT_INVERTR {
    bits: bool,
}
impl FSK_BIT_INVERTR {
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
pub struct RFU00R {
    bits: bool,
}
impl RFU00R {
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
#[doc = "Possible values of the field `BSM_EN_BLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSM_EN_BLER {
    #[doc = "BSM for BLE disabled"]
    _0,
    #[doc = "BSM for BLE enabled"]
    _1,
}
impl BSM_EN_BLER {
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
            BSM_EN_BLER::_0 => false,
            BSM_EN_BLER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BSM_EN_BLER {
        match value {
            false => BSM_EN_BLER::_0,
            true => BSM_EN_BLER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BSM_EN_BLER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BSM_EN_BLER::_1
    }
}
#[doc = "Possible values of the field `DEMOD_CLK_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEMOD_CLK_MODER {
    #[doc = "Normal"]
    _0,
    #[doc = "Demodulate all samples"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DEMOD_CLK_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEMOD_CLK_MODER::_0 => 0,
            DEMOD_CLK_MODER::_1 => 1,
            DEMOD_CLK_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEMOD_CLK_MODER {
        match value {
            0 => DEMOD_CLK_MODER::_0,
            1 => DEMOD_CLK_MODER::_1,
            i => DEMOD_CLK_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DEMOD_CLK_MODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DEMOD_CLK_MODER::_1
    }
}
#[doc = r" Value of the field"]
pub struct CTS_THRESHR {
    bits: u8,
}
impl CTS_THRESHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FSK_FTS_TIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSK_FTS_TIMEOUTR {
    #[doc = "4 symbols"]
    _0,
    #[doc = "5 symbols"]
    _1,
    #[doc = "6 symbols"]
    _2,
    #[doc = "7 symbols"]
    _3,
    #[doc = "8 symbols"]
    _4,
    #[doc = "9 symbols"]
    _5,
    #[doc = "10 symbols"]
    _6,
    #[doc = "11 symbols"]
    _7,
}
impl FSK_FTS_TIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSK_FTS_TIMEOUTR::_0 => 0,
            FSK_FTS_TIMEOUTR::_1 => 1,
            FSK_FTS_TIMEOUTR::_2 => 2,
            FSK_FTS_TIMEOUTR::_3 => 3,
            FSK_FTS_TIMEOUTR::_4 => 4,
            FSK_FTS_TIMEOUTR::_5 => 5,
            FSK_FTS_TIMEOUTR::_6 => 6,
            FSK_FTS_TIMEOUTR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSK_FTS_TIMEOUTR {
        match value {
            0 => FSK_FTS_TIMEOUTR::_0,
            1 => FSK_FTS_TIMEOUTR::_1,
            2 => FSK_FTS_TIMEOUTR::_2,
            3 => FSK_FTS_TIMEOUTR::_3,
            4 => FSK_FTS_TIMEOUTR::_4,
            5 => FSK_FTS_TIMEOUTR::_5,
            6 => FSK_FTS_TIMEOUTR::_6,
            7 => FSK_FTS_TIMEOUTR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FSK_FTS_TIMEOUTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FSK_FTS_TIMEOUTR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == FSK_FTS_TIMEOUTR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == FSK_FTS_TIMEOUTR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == FSK_FTS_TIMEOUTR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == FSK_FTS_TIMEOUTR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == FSK_FTS_TIMEOUTR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == FSK_FTS_TIMEOUTR::_7
    }
}
#[doc = r" Value of the field"]
pub struct RFU01R {
    bits: bool,
}
impl RFU01R {
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
pub struct RFU02R {
    bits: bool,
}
impl RFU02R {
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
pub struct BLE_NTW_ADR_THRR {
    bits: u8,
}
impl BLE_NTW_ADR_THRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _AA_PLAYBACKW<'a> {
    w: &'a mut W,
}
impl<'a> _AA_PLAYBACKW<'a> {
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
#[doc = "Values that can be written to the field `AA_OUTPUT_SEL`"]
pub enum AA_OUTPUT_SELW {
    #[doc = "demodulated"]
    _0,
    #[doc = "matched"]
    _1,
}
impl AA_OUTPUT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AA_OUTPUT_SELW::_0 => false,
            AA_OUTPUT_SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AA_OUTPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _AA_OUTPUT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AA_OUTPUT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "demodulated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AA_OUTPUT_SELW::_0)
    }
    #[doc = "matched"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AA_OUTPUT_SELW::_1)
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
#[doc = r" Proxy"]
pub struct _FSK_BIT_INVERTW<'a> {
    w: &'a mut W,
}
impl<'a> _FSK_BIT_INVERTW<'a> {
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
#[doc = r" Proxy"]
pub struct _RFU00W<'a> {
    w: &'a mut W,
}
impl<'a> _RFU00W<'a> {
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
#[doc = "Values that can be written to the field `BSM_EN_BLE`"]
pub enum BSM_EN_BLEW {
    #[doc = "BSM for BLE disabled"]
    _0,
    #[doc = "BSM for BLE enabled"]
    _1,
}
impl BSM_EN_BLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BSM_EN_BLEW::_0 => false,
            BSM_EN_BLEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BSM_EN_BLEW<'a> {
    w: &'a mut W,
}
impl<'a> _BSM_EN_BLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BSM_EN_BLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BSM for BLE disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSM_EN_BLEW::_0)
    }
    #[doc = "BSM for BLE enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSM_EN_BLEW::_1)
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
#[doc = "Values that can be written to the field `DEMOD_CLK_MODE`"]
pub enum DEMOD_CLK_MODEW {
    #[doc = "Normal"]
    _0,
    #[doc = "Demodulate all samples"]
    _1,
}
impl DEMOD_CLK_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DEMOD_CLK_MODEW::_0 => 0,
            DEMOD_CLK_MODEW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEMOD_CLK_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEMOD_CLK_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEMOD_CLK_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEMOD_CLK_MODEW::_0)
    }
    #[doc = "Demodulate all samples"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEMOD_CLK_MODEW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTS_THRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _CTS_THRESHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FSK_FTS_TIMEOUT`"]
pub enum FSK_FTS_TIMEOUTW {
    #[doc = "4 symbols"]
    _0,
    #[doc = "5 symbols"]
    _1,
    #[doc = "6 symbols"]
    _2,
    #[doc = "7 symbols"]
    _3,
    #[doc = "8 symbols"]
    _4,
    #[doc = "9 symbols"]
    _5,
    #[doc = "10 symbols"]
    _6,
    #[doc = "11 symbols"]
    _7,
}
impl FSK_FTS_TIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FSK_FTS_TIMEOUTW::_0 => 0,
            FSK_FTS_TIMEOUTW::_1 => 1,
            FSK_FTS_TIMEOUTW::_2 => 2,
            FSK_FTS_TIMEOUTW::_3 => 3,
            FSK_FTS_TIMEOUTW::_4 => 4,
            FSK_FTS_TIMEOUTW::_5 => 5,
            FSK_FTS_TIMEOUTW::_6 => 6,
            FSK_FTS_TIMEOUTW::_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSK_FTS_TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _FSK_FTS_TIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSK_FTS_TIMEOUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4 symbols"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSK_FTS_TIMEOUTW::_0)
    }
    #[doc = "5 symbols"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSK_FTS_TIMEOUTW::_1)
    }
    #[doc = "6 symbols"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(FSK_FTS_TIMEOUTW::_2)
    }
    #[doc = "7 symbols"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(FSK_FTS_TIMEOUTW::_3)
    }
    #[doc = "8 symbols"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(FSK_FTS_TIMEOUTW::_4)
    }
    #[doc = "9 symbols"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(FSK_FTS_TIMEOUTW::_5)
    }
    #[doc = "10 symbols"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(FSK_FTS_TIMEOUTW::_6)
    }
    #[doc = "11 symbols"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(FSK_FTS_TIMEOUTW::_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFU01W<'a> {
    w: &'a mut W,
}
impl<'a> _RFU01W<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFU02W<'a> {
    w: &'a mut W,
}
impl<'a> _RFU02W<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BLE_NTW_ADR_THRW<'a> {
    w: &'a mut W,
}
impl<'a> _BLE_NTW_ADR_THRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
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
    #[doc = "Bit 1 - Access Address Playback"]
    #[inline]
    pub fn aa_playback(&self) -> AA_PLAYBACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AA_PLAYBACKR { bits }
    }
    #[doc = "Bit 2 - Access Address Output Select"]
    #[inline]
    pub fn aa_output_sel(&self) -> AA_OUTPUT_SELR {
        AA_OUTPUT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - FSK Bit Invert"]
    #[inline]
    pub fn fsk_bit_invert(&self) -> FSK_BIT_INVERTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSK_BIT_INVERTR { bits }
    }
    #[doc = "Bit 4 - Reserved for future use."]
    #[inline]
    pub fn rfu00(&self) -> RFU00R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFU00R { bits }
    }
    #[doc = "Bit 5 - BLE Bit Streaming Mode Enable bit"]
    #[inline]
    pub fn bsm_en_ble(&self) -> BSM_EN_BLER {
        BSM_EN_BLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Demodulator Clock Mode"]
    #[inline]
    pub fn demod_clk_mode(&self) -> DEMOD_CLK_MODER {
        DEMOD_CLK_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - CTS (Coarse Timing Search) Correlation Threshold"]
    #[inline]
    pub fn cts_thresh(&self) -> CTS_THRESHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTS_THRESHR { bits }
    }
    #[doc = "Bits 20:22 - FSK FTS Timeout"]
    #[inline]
    pub fn fsk_fts_timeout(&self) -> FSK_FTS_TIMEOUTR {
        FSK_FTS_TIMEOUTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Reserved for future use."]
    #[inline]
    pub fn rfu01(&self) -> RFU01R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFU01R { bits }
    }
    #[doc = "Bit 25 - Reserved for future use."]
    #[inline]
    pub fn rfu02(&self) -> RFU02R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFU02R { bits }
    }
    #[doc = "Bits 28:30 - BLE Network Address Match Bit Error Threshold"]
    #[inline]
    pub fn ble_ntw_adr_thr(&self) -> BLE_NTW_ADR_THRR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BLE_NTW_ADR_THRR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 275827990 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Access Address Playback"]
    #[inline]
    pub fn aa_playback(&mut self) -> _AA_PLAYBACKW {
        _AA_PLAYBACKW { w: self }
    }
    #[doc = "Bit 2 - Access Address Output Select"]
    #[inline]
    pub fn aa_output_sel(&mut self) -> _AA_OUTPUT_SELW {
        _AA_OUTPUT_SELW { w: self }
    }
    #[doc = "Bit 3 - FSK Bit Invert"]
    #[inline]
    pub fn fsk_bit_invert(&mut self) -> _FSK_BIT_INVERTW {
        _FSK_BIT_INVERTW { w: self }
    }
    #[doc = "Bit 4 - Reserved for future use."]
    #[inline]
    pub fn rfu00(&mut self) -> _RFU00W {
        _RFU00W { w: self }
    }
    #[doc = "Bit 5 - BLE Bit Streaming Mode Enable bit"]
    #[inline]
    pub fn bsm_en_ble(&mut self) -> _BSM_EN_BLEW {
        _BSM_EN_BLEW { w: self }
    }
    #[doc = "Bits 6:7 - Demodulator Clock Mode"]
    #[inline]
    pub fn demod_clk_mode(&mut self) -> _DEMOD_CLK_MODEW {
        _DEMOD_CLK_MODEW { w: self }
    }
    #[doc = "Bits 8:15 - CTS (Coarse Timing Search) Correlation Threshold"]
    #[inline]
    pub fn cts_thresh(&mut self) -> _CTS_THRESHW {
        _CTS_THRESHW { w: self }
    }
    #[doc = "Bits 20:22 - FSK FTS Timeout"]
    #[inline]
    pub fn fsk_fts_timeout(&mut self) -> _FSK_FTS_TIMEOUTW {
        _FSK_FTS_TIMEOUTW { w: self }
    }
    #[doc = "Bit 24 - Reserved for future use."]
    #[inline]
    pub fn rfu01(&mut self) -> _RFU01W {
        _RFU01W { w: self }
    }
    #[doc = "Bit 25 - Reserved for future use."]
    #[inline]
    pub fn rfu02(&mut self) -> _RFU02W {
        _RFU02W { w: self }
    }
    #[doc = "Bits 28:30 - BLE Network Address Match Bit Error Threshold"]
    #[inline]
    pub fn ble_ntw_adr_thr(&mut self) -> _BLE_NTW_ADR_THRW {
        _BLE_NTW_ADR_THRW { w: self }
    }
}
