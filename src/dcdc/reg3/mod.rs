#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG3 {
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
#[doc = "Possible values of the field `DCDC_VDD1P8CTRL_TRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_VDD1P8CTRL_TRGR {
    #[doc = "1.65 V"]
    _0,
    #[doc = "1.8 V"]
    _110,
    #[doc = "2.075 V"]
    _10001,
    #[doc = "2.8 V"]
    _100000,
    #[doc = "3.3 V"]
    _110100,
    #[doc = "3.575 V"]
    _111111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DCDC_VDD1P8CTRL_TRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCDC_VDD1P8CTRL_TRGR::_0 => 0,
            DCDC_VDD1P8CTRL_TRGR::_110 => 6,
            DCDC_VDD1P8CTRL_TRGR::_10001 => 17,
            DCDC_VDD1P8CTRL_TRGR::_100000 => 32,
            DCDC_VDD1P8CTRL_TRGR::_110100 => 52,
            DCDC_VDD1P8CTRL_TRGR::_111111 => 63,
            DCDC_VDD1P8CTRL_TRGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCDC_VDD1P8CTRL_TRGR {
        match value {
            0 => DCDC_VDD1P8CTRL_TRGR::_0,
            6 => DCDC_VDD1P8CTRL_TRGR::_110,
            17 => DCDC_VDD1P8CTRL_TRGR::_10001,
            32 => DCDC_VDD1P8CTRL_TRGR::_100000,
            52 => DCDC_VDD1P8CTRL_TRGR::_110100,
            63 => DCDC_VDD1P8CTRL_TRGR::_111111,
            i => DCDC_VDD1P8CTRL_TRGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCDC_VDD1P8CTRL_TRGR::_0
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == DCDC_VDD1P8CTRL_TRGR::_110
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline]
    pub fn is_10001(&self) -> bool {
        *self == DCDC_VDD1P8CTRL_TRGR::_10001
    }
    #[doc = "Checks if the value of the field is `_100000`"]
    #[inline]
    pub fn is_100000(&self) -> bool {
        *self == DCDC_VDD1P8CTRL_TRGR::_100000
    }
    #[doc = "Checks if the value of the field is `_110100`"]
    #[inline]
    pub fn is_110100(&self) -> bool {
        *self == DCDC_VDD1P8CTRL_TRGR::_110100
    }
    #[doc = "Checks if the value of the field is `_111111`"]
    #[inline]
    pub fn is_111111(&self) -> bool {
        *self == DCDC_VDD1P8CTRL_TRGR::_111111
    }
}
#[doc = "Possible values of the field `DCDC_VDD1P5CTRL_TRG_BUCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_VDD1P5CTRL_TRG_BUCKR {
    #[doc = "1.65 V"]
    _1111,
    #[doc = "1.5 V"]
    _1001,
    #[doc = "1.275 V"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DCDC_VDD1P5CTRL_TRG_BUCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCDC_VDD1P5CTRL_TRG_BUCKR::_1111 => 15,
            DCDC_VDD1P5CTRL_TRG_BUCKR::_1001 => 9,
            DCDC_VDD1P5CTRL_TRG_BUCKR::_0 => 0,
            DCDC_VDD1P5CTRL_TRG_BUCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCDC_VDD1P5CTRL_TRG_BUCKR {
        match value {
            15 => DCDC_VDD1P5CTRL_TRG_BUCKR::_1111,
            9 => DCDC_VDD1P5CTRL_TRG_BUCKR::_1001,
            0 => DCDC_VDD1P5CTRL_TRG_BUCKR::_0,
            i => DCDC_VDD1P5CTRL_TRG_BUCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == DCDC_VDD1P5CTRL_TRG_BUCKR::_1111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == DCDC_VDD1P5CTRL_TRG_BUCKR::_1001
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCDC_VDD1P5CTRL_TRG_BUCKR::_0
    }
}
#[doc = "Possible values of the field `DCDC_VDD1P5CTRL_TRG_BOOST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_VDD1P5CTRL_TRG_BOOSTR {
    #[doc = "1.8 V"]
    _10101,
    #[doc = "1.65 V"]
    _1111,
    #[doc = "1.5 V"]
    _1001,
    #[doc = "1.275 V"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DCDC_VDD1P5CTRL_TRG_BOOSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCDC_VDD1P5CTRL_TRG_BOOSTR::_10101 => 21,
            DCDC_VDD1P5CTRL_TRG_BOOSTR::_1111 => 15,
            DCDC_VDD1P5CTRL_TRG_BOOSTR::_1001 => 9,
            DCDC_VDD1P5CTRL_TRG_BOOSTR::_0 => 0,
            DCDC_VDD1P5CTRL_TRG_BOOSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCDC_VDD1P5CTRL_TRG_BOOSTR {
        match value {
            21 => DCDC_VDD1P5CTRL_TRG_BOOSTR::_10101,
            15 => DCDC_VDD1P5CTRL_TRG_BOOSTR::_1111,
            9 => DCDC_VDD1P5CTRL_TRG_BOOSTR::_1001,
            0 => DCDC_VDD1P5CTRL_TRG_BOOSTR::_0,
            i => DCDC_VDD1P5CTRL_TRG_BOOSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline]
    pub fn is_10101(&self) -> bool {
        *self == DCDC_VDD1P5CTRL_TRG_BOOSTR::_10101
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == DCDC_VDD1P5CTRL_TRG_BOOSTR::_1111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == DCDC_VDD1P5CTRL_TRG_BOOSTR::_1001
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCDC_VDD1P5CTRL_TRG_BOOSTR::_0
    }
}
#[doc = r" Value of the field"]
pub struct DCDC_VDD1P5CTRL_ADJTNR {
    bits: u8,
}
impl DCDC_VDD1P5CTRL_ADJTNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCDC_MINPWR_DC_HALFCLK_PULSEDR {
    bits: bool,
}
impl DCDC_MINPWR_DC_HALFCLK_PULSEDR {
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
pub struct DCDC_MINPWR_DOUBLE_FETS_PULSEDR {
    bits: bool,
}
impl DCDC_MINPWR_DOUBLE_FETS_PULSEDR {
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
pub struct DCDC_MINPWR_HALF_FETS_PULSEDR {
    bits: bool,
}
impl DCDC_MINPWR_HALF_FETS_PULSEDR {
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
pub struct DCDC_MINPWR_DC_HALFCLKR {
    bits: bool,
}
impl DCDC_MINPWR_DC_HALFCLKR {
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
pub struct DCDC_MINPWR_DOUBLE_FETSR {
    bits: bool,
}
impl DCDC_MINPWR_DOUBLE_FETSR {
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
pub struct DCDC_MINPWR_HALF_FETSR {
    bits: bool,
}
impl DCDC_MINPWR_HALF_FETSR {
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
pub struct DCDC_VDD1P5CTRL_DISABLE_STEPR {
    bits: bool,
}
impl DCDC_VDD1P5CTRL_DISABLE_STEPR {
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
pub struct DCDC_VDD1P8CTRL_DISABLE_STEPR {
    bits: bool,
}
impl DCDC_VDD1P8CTRL_DISABLE_STEPR {
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
#[doc = "Values that can be written to the field `DCDC_VDD1P8CTRL_TRG`"]
pub enum DCDC_VDD1P8CTRL_TRGW {
    #[doc = "1.65 V"]
    _0,
    #[doc = "1.8 V"]
    _110,
    #[doc = "2.075 V"]
    _10001,
    #[doc = "2.8 V"]
    _100000,
    #[doc = "3.3 V"]
    _110100,
    #[doc = "3.575 V"]
    _111111,
}
impl DCDC_VDD1P8CTRL_TRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCDC_VDD1P8CTRL_TRGW::_0 => 0,
            DCDC_VDD1P8CTRL_TRGW::_110 => 6,
            DCDC_VDD1P8CTRL_TRGW::_10001 => 17,
            DCDC_VDD1P8CTRL_TRGW::_100000 => 32,
            DCDC_VDD1P8CTRL_TRGW::_110100 => 52,
            DCDC_VDD1P8CTRL_TRGW::_111111 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_VDD1P8CTRL_TRGW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_VDD1P8CTRL_TRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCDC_VDD1P8CTRL_TRGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1.65 V"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCDC_VDD1P8CTRL_TRGW::_0)
    }
    #[doc = "1.8 V"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(DCDC_VDD1P8CTRL_TRGW::_110)
    }
    #[doc = "2.075 V"]
    #[inline]
    pub fn _10001(self) -> &'a mut W {
        self.variant(DCDC_VDD1P8CTRL_TRGW::_10001)
    }
    #[doc = "2.8 V"]
    #[inline]
    pub fn _100000(self) -> &'a mut W {
        self.variant(DCDC_VDD1P8CTRL_TRGW::_100000)
    }
    #[doc = "3.3 V"]
    #[inline]
    pub fn _110100(self) -> &'a mut W {
        self.variant(DCDC_VDD1P8CTRL_TRGW::_110100)
    }
    #[doc = "3.575 V"]
    #[inline]
    pub fn _111111(self) -> &'a mut W {
        self.variant(DCDC_VDD1P8CTRL_TRGW::_111111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCDC_VDD1P5CTRL_TRG_BUCK`"]
pub enum DCDC_VDD1P5CTRL_TRG_BUCKW {
    #[doc = "1.65 V"]
    _1111,
    #[doc = "1.5 V"]
    _1001,
    #[doc = "1.275 V"]
    _0,
}
impl DCDC_VDD1P5CTRL_TRG_BUCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCDC_VDD1P5CTRL_TRG_BUCKW::_1111 => 15,
            DCDC_VDD1P5CTRL_TRG_BUCKW::_1001 => 9,
            DCDC_VDD1P5CTRL_TRG_BUCKW::_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_VDD1P5CTRL_TRG_BUCKW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_VDD1P5CTRL_TRG_BUCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCDC_VDD1P5CTRL_TRG_BUCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1.65 V"]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(DCDC_VDD1P5CTRL_TRG_BUCKW::_1111)
    }
    #[doc = "1.5 V"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(DCDC_VDD1P5CTRL_TRG_BUCKW::_1001)
    }
    #[doc = "1.275 V"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCDC_VDD1P5CTRL_TRG_BUCKW::_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCDC_VDD1P5CTRL_TRG_BOOST`"]
pub enum DCDC_VDD1P5CTRL_TRG_BOOSTW {
    #[doc = "1.8 V"]
    _10101,
    #[doc = "1.65 V"]
    _1111,
    #[doc = "1.5 V"]
    _1001,
    #[doc = "1.275 V"]
    _0,
}
impl DCDC_VDD1P5CTRL_TRG_BOOSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCDC_VDD1P5CTRL_TRG_BOOSTW::_10101 => 21,
            DCDC_VDD1P5CTRL_TRG_BOOSTW::_1111 => 15,
            DCDC_VDD1P5CTRL_TRG_BOOSTW::_1001 => 9,
            DCDC_VDD1P5CTRL_TRG_BOOSTW::_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_VDD1P5CTRL_TRG_BOOSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_VDD1P5CTRL_TRG_BOOSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCDC_VDD1P5CTRL_TRG_BOOSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1.8 V"]
    #[inline]
    pub fn _10101(self) -> &'a mut W {
        self.variant(DCDC_VDD1P5CTRL_TRG_BOOSTW::_10101)
    }
    #[doc = "1.65 V"]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(DCDC_VDD1P5CTRL_TRG_BOOSTW::_1111)
    }
    #[doc = "1.5 V"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(DCDC_VDD1P5CTRL_TRG_BOOSTW::_1001)
    }
    #[doc = "1.275 V"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCDC_VDD1P5CTRL_TRG_BOOSTW::_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_VDD1P5CTRL_ADJTNW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_VDD1P5CTRL_ADJTNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_MINPWR_DC_HALFCLK_PULSEDW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_MINPWR_DC_HALFCLK_PULSEDW<'a> {
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
#[doc = r" Proxy"]
pub struct _DCDC_MINPWR_DOUBLE_FETS_PULSEDW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_MINPWR_DOUBLE_FETS_PULSEDW<'a> {
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
pub struct _DCDC_MINPWR_HALF_FETS_PULSEDW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_MINPWR_HALF_FETS_PULSEDW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_MINPWR_DC_HALFCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_MINPWR_DC_HALFCLKW<'a> {
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
pub struct _DCDC_MINPWR_DOUBLE_FETSW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_MINPWR_DOUBLE_FETSW<'a> {
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
pub struct _DCDC_MINPWR_HALF_FETSW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_MINPWR_HALF_FETSW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_VDD1P5CTRL_DISABLE_STEPW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_VDD1P5CTRL_DISABLE_STEPW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_VDD1P8CTRL_DISABLE_STEPW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_VDD1P8CTRL_DISABLE_STEPW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Target value of VDD1P8, 25 mV each step in two ranges, from 0x00 to 0x11 and 0x20 to 0x3F."]
    #[inline]
    pub fn dcdc_vdd1p8ctrl_trg(&self) -> DCDC_VDD1P8CTRL_TRGR {
        DCDC_VDD1P8CTRL_TRGR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:10 - Target value of VDD1P5 in buck mode, 25 mV each step from 0x00 to 0x0F"]
    #[inline]
    pub fn dcdc_vdd1p5ctrl_trg_buck(&self) -> DCDC_VDD1P5CTRL_TRG_BUCKR {
        DCDC_VDD1P5CTRL_TRG_BUCKR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:15 - Target value of VDD1P5 in boost mode, 25 mV each step from 0x00 to 0x0F"]
    #[inline]
    pub fn dcdc_vdd1p5ctrl_trg_boost(&self) -> DCDC_VDD1P5CTRL_TRG_BOOSTR {
        DCDC_VDD1P5CTRL_TRG_BOOSTR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:20 - Adjust value of duty cycle when switching between VDD1P5 and VDD1P8. The unit is 1/32 or 3.125%."]
    #[inline]
    pub fn dcdc_vdd1p5ctrl_adjtn(&self) -> DCDC_VDD1P5CTRL_ADJTNR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCDC_VDD1P5CTRL_ADJTNR { bits }
    }
    #[doc = "Bit 21 - Set DCDC clock to half frequency for the Pulsed mode."]
    #[inline]
    pub fn dcdc_minpwr_dc_halfclk_pulsed(&self) -> DCDC_MINPWR_DC_HALFCLK_PULSEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_MINPWR_DC_HALFCLK_PULSEDR { bits }
    }
    #[doc = "Bit 22 - Use double switch FET for the Pulsed mode."]
    #[inline]
    pub fn dcdc_minpwr_double_fets_pulsed(&self) -> DCDC_MINPWR_DOUBLE_FETS_PULSEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_MINPWR_DOUBLE_FETS_PULSEDR { bits }
    }
    #[doc = "Bit 23 - Use half switch FET for the Pulsed mode."]
    #[inline]
    pub fn dcdc_minpwr_half_fets_pulsed(&self) -> DCDC_MINPWR_HALF_FETS_PULSEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_MINPWR_HALF_FETS_PULSEDR { bits }
    }
    #[doc = "Bit 24 - Set DCDC clock to half frequency for the continuous mode."]
    #[inline]
    pub fn dcdc_minpwr_dc_halfclk(&self) -> DCDC_MINPWR_DC_HALFCLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_MINPWR_DC_HALFCLKR { bits }
    }
    #[doc = "Bit 25 - Use double switch FET for the continuous mode."]
    #[inline]
    pub fn dcdc_minpwr_double_fets(&self) -> DCDC_MINPWR_DOUBLE_FETSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_MINPWR_DOUBLE_FETSR { bits }
    }
    #[doc = "Bit 26 - Use half switch FET for the continuous mode."]
    #[inline]
    pub fn dcdc_minpwr_half_fets(&self) -> DCDC_MINPWR_HALF_FETSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_MINPWR_HALF_FETSR { bits }
    }
    #[doc = "Bit 29 - Disable stepping for VDD1P5. Must set this bit before enter low power modes."]
    #[inline]
    pub fn dcdc_vdd1p5ctrl_disable_step(&self) -> DCDC_VDD1P5CTRL_DISABLE_STEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_VDD1P5CTRL_DISABLE_STEPR { bits }
    }
    #[doc = "Bit 30 - Disable stepping for VDD1P8. Must set this bit before enter low power modes."]
    #[inline]
    pub fn dcdc_vdd1p8ctrl_disable_step(&self) -> DCDC_VDD1P8CTRL_DISABLE_STEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_VDD1P8CTRL_DISABLE_STEPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 43590 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Target value of VDD1P8, 25 mV each step in two ranges, from 0x00 to 0x11 and 0x20 to 0x3F."]
    #[inline]
    pub fn dcdc_vdd1p8ctrl_trg(&mut self) -> _DCDC_VDD1P8CTRL_TRGW {
        _DCDC_VDD1P8CTRL_TRGW { w: self }
    }
    #[doc = "Bits 6:10 - Target value of VDD1P5 in buck mode, 25 mV each step from 0x00 to 0x0F"]
    #[inline]
    pub fn dcdc_vdd1p5ctrl_trg_buck(&mut self) -> _DCDC_VDD1P5CTRL_TRG_BUCKW {
        _DCDC_VDD1P5CTRL_TRG_BUCKW { w: self }
    }
    #[doc = "Bits 11:15 - Target value of VDD1P5 in boost mode, 25 mV each step from 0x00 to 0x0F"]
    #[inline]
    pub fn dcdc_vdd1p5ctrl_trg_boost(&mut self) -> _DCDC_VDD1P5CTRL_TRG_BOOSTW {
        _DCDC_VDD1P5CTRL_TRG_BOOSTW { w: self }
    }
    #[doc = "Bits 17:20 - Adjust value of duty cycle when switching between VDD1P5 and VDD1P8. The unit is 1/32 or 3.125%."]
    #[inline]
    pub fn dcdc_vdd1p5ctrl_adjtn(&mut self) -> _DCDC_VDD1P5CTRL_ADJTNW {
        _DCDC_VDD1P5CTRL_ADJTNW { w: self }
    }
    #[doc = "Bit 21 - Set DCDC clock to half frequency for the Pulsed mode."]
    #[inline]
    pub fn dcdc_minpwr_dc_halfclk_pulsed(&mut self) -> _DCDC_MINPWR_DC_HALFCLK_PULSEDW {
        _DCDC_MINPWR_DC_HALFCLK_PULSEDW { w: self }
    }
    #[doc = "Bit 22 - Use double switch FET for the Pulsed mode."]
    #[inline]
    pub fn dcdc_minpwr_double_fets_pulsed(&mut self) -> _DCDC_MINPWR_DOUBLE_FETS_PULSEDW {
        _DCDC_MINPWR_DOUBLE_FETS_PULSEDW { w: self }
    }
    #[doc = "Bit 23 - Use half switch FET for the Pulsed mode."]
    #[inline]
    pub fn dcdc_minpwr_half_fets_pulsed(&mut self) -> _DCDC_MINPWR_HALF_FETS_PULSEDW {
        _DCDC_MINPWR_HALF_FETS_PULSEDW { w: self }
    }
    #[doc = "Bit 24 - Set DCDC clock to half frequency for the continuous mode."]
    #[inline]
    pub fn dcdc_minpwr_dc_halfclk(&mut self) -> _DCDC_MINPWR_DC_HALFCLKW {
        _DCDC_MINPWR_DC_HALFCLKW { w: self }
    }
    #[doc = "Bit 25 - Use double switch FET for the continuous mode."]
    #[inline]
    pub fn dcdc_minpwr_double_fets(&mut self) -> _DCDC_MINPWR_DOUBLE_FETSW {
        _DCDC_MINPWR_DOUBLE_FETSW { w: self }
    }
    #[doc = "Bit 26 - Use half switch FET for the continuous mode."]
    #[inline]
    pub fn dcdc_minpwr_half_fets(&mut self) -> _DCDC_MINPWR_HALF_FETSW {
        _DCDC_MINPWR_HALF_FETSW { w: self }
    }
    #[doc = "Bit 29 - Disable stepping for VDD1P5. Must set this bit before enter low power modes."]
    #[inline]
    pub fn dcdc_vdd1p5ctrl_disable_step(&mut self) -> _DCDC_VDD1P5CTRL_DISABLE_STEPW {
        _DCDC_VDD1P5CTRL_DISABLE_STEPW { w: self }
    }
    #[doc = "Bit 30 - Disable stepping for VDD1P8. Must set this bit before enter low power modes."]
    #[inline]
    pub fn dcdc_vdd1p8ctrl_disable_step(&mut self) -> _DCDC_VDD1P8CTRL_DISABLE_STEPW {
        _DCDC_VDD1P8CTRL_DISABLE_STEPW { w: self }
    }
}
