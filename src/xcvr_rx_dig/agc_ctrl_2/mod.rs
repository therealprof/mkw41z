#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AGC_CTRL_2 {
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
pub struct BBA_PDET_RSTR {
    bits: bool,
}
impl BBA_PDET_RSTR {
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
pub struct TZA_PDET_RSTR {
    bits: bool,
}
impl TZA_PDET_RSTR {
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
#[doc = "Possible values of the field `MAN_PDET_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAN_PDET_RSTR {
    #[doc = "The peak detector reset signals are controlled automatically by the AGC."]
    _0,
    #[doc = "The BBA_PDET_RST and TZA_PDET_RST are used to manually control the peak detector reset signals."]
    _1,
}
impl MAN_PDET_RSTR {
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
            MAN_PDET_RSTR::_0 => false,
            MAN_PDET_RSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAN_PDET_RSTR {
        match value {
            false => MAN_PDET_RSTR::_0,
            true => MAN_PDET_RSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MAN_PDET_RSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MAN_PDET_RSTR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BBA_GAIN_SETTLE_TIMER {
    bits: u8,
}
impl BBA_GAIN_SETTLE_TIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BBA_PDET_SEL_LO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBA_PDET_SEL_LOR {
    #[doc = "0.600V"]
    _000,
    #[doc = "0.615V"]
    _001,
    #[doc = "0.630V"]
    _010,
    #[doc = "0.645V"]
    _011,
    #[doc = "0.660V"]
    _100,
    #[doc = "0.675V"]
    _101,
    #[doc = "0.690V"]
    _110,
    #[doc = "0.705V"]
    _111,
}
impl BBA_PDET_SEL_LOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BBA_PDET_SEL_LOR::_000 => 0,
            BBA_PDET_SEL_LOR::_001 => 1,
            BBA_PDET_SEL_LOR::_010 => 2,
            BBA_PDET_SEL_LOR::_011 => 3,
            BBA_PDET_SEL_LOR::_100 => 4,
            BBA_PDET_SEL_LOR::_101 => 5,
            BBA_PDET_SEL_LOR::_110 => 6,
            BBA_PDET_SEL_LOR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BBA_PDET_SEL_LOR {
        match value {
            0 => BBA_PDET_SEL_LOR::_000,
            1 => BBA_PDET_SEL_LOR::_001,
            2 => BBA_PDET_SEL_LOR::_010,
            3 => BBA_PDET_SEL_LOR::_011,
            4 => BBA_PDET_SEL_LOR::_100,
            5 => BBA_PDET_SEL_LOR::_101,
            6 => BBA_PDET_SEL_LOR::_110,
            7 => BBA_PDET_SEL_LOR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == BBA_PDET_SEL_LOR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == BBA_PDET_SEL_LOR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == BBA_PDET_SEL_LOR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == BBA_PDET_SEL_LOR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == BBA_PDET_SEL_LOR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == BBA_PDET_SEL_LOR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == BBA_PDET_SEL_LOR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == BBA_PDET_SEL_LOR::_111
    }
}
#[doc = "Possible values of the field `BBA_PDET_SEL_HI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBA_PDET_SEL_HIR {
    #[doc = "0.600V"]
    _000,
    #[doc = "0.795V"]
    _001,
    #[doc = "0.900V"]
    _010,
    #[doc = "0.945V"]
    _011,
    #[doc = "1.005V"]
    _100,
    #[doc = "1.050V"]
    _101,
    #[doc = "1.095V"]
    _110,
    #[doc = "1.155V"]
    _111,
}
impl BBA_PDET_SEL_HIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BBA_PDET_SEL_HIR::_000 => 0,
            BBA_PDET_SEL_HIR::_001 => 1,
            BBA_PDET_SEL_HIR::_010 => 2,
            BBA_PDET_SEL_HIR::_011 => 3,
            BBA_PDET_SEL_HIR::_100 => 4,
            BBA_PDET_SEL_HIR::_101 => 5,
            BBA_PDET_SEL_HIR::_110 => 6,
            BBA_PDET_SEL_HIR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BBA_PDET_SEL_HIR {
        match value {
            0 => BBA_PDET_SEL_HIR::_000,
            1 => BBA_PDET_SEL_HIR::_001,
            2 => BBA_PDET_SEL_HIR::_010,
            3 => BBA_PDET_SEL_HIR::_011,
            4 => BBA_PDET_SEL_HIR::_100,
            5 => BBA_PDET_SEL_HIR::_101,
            6 => BBA_PDET_SEL_HIR::_110,
            7 => BBA_PDET_SEL_HIR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == BBA_PDET_SEL_HIR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == BBA_PDET_SEL_HIR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == BBA_PDET_SEL_HIR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == BBA_PDET_SEL_HIR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == BBA_PDET_SEL_HIR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == BBA_PDET_SEL_HIR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == BBA_PDET_SEL_HIR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == BBA_PDET_SEL_HIR::_111
    }
}
#[doc = "Possible values of the field `TZA_PDET_SEL_LO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZA_PDET_SEL_LOR {
    #[doc = "0.600V"]
    _000,
    #[doc = "0.615V"]
    _001,
    #[doc = "0.630V"]
    _010,
    #[doc = "0.645V"]
    _011,
    #[doc = "0.660V"]
    _100,
    #[doc = "0.675V"]
    _101,
    #[doc = "0.690V"]
    _110,
    #[doc = "0.705V"]
    _111,
}
impl TZA_PDET_SEL_LOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TZA_PDET_SEL_LOR::_000 => 0,
            TZA_PDET_SEL_LOR::_001 => 1,
            TZA_PDET_SEL_LOR::_010 => 2,
            TZA_PDET_SEL_LOR::_011 => 3,
            TZA_PDET_SEL_LOR::_100 => 4,
            TZA_PDET_SEL_LOR::_101 => 5,
            TZA_PDET_SEL_LOR::_110 => 6,
            TZA_PDET_SEL_LOR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TZA_PDET_SEL_LOR {
        match value {
            0 => TZA_PDET_SEL_LOR::_000,
            1 => TZA_PDET_SEL_LOR::_001,
            2 => TZA_PDET_SEL_LOR::_010,
            3 => TZA_PDET_SEL_LOR::_011,
            4 => TZA_PDET_SEL_LOR::_100,
            5 => TZA_PDET_SEL_LOR::_101,
            6 => TZA_PDET_SEL_LOR::_110,
            7 => TZA_PDET_SEL_LOR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == TZA_PDET_SEL_LOR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == TZA_PDET_SEL_LOR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == TZA_PDET_SEL_LOR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == TZA_PDET_SEL_LOR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == TZA_PDET_SEL_LOR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == TZA_PDET_SEL_LOR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == TZA_PDET_SEL_LOR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == TZA_PDET_SEL_LOR::_111
    }
}
#[doc = "Possible values of the field `TZA_PDET_SEL_HI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZA_PDET_SEL_HIR {
    #[doc = "0.60V"]
    _000,
    #[doc = "0.63V"]
    _001,
    #[doc = "0.66V"]
    _010,
    #[doc = "0.69V"]
    _011,
    #[doc = "0.72V"]
    _100,
    #[doc = "0.75V"]
    _101,
    #[doc = "0.78V"]
    _110,
    #[doc = "0.81V"]
    _111,
}
impl TZA_PDET_SEL_HIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TZA_PDET_SEL_HIR::_000 => 0,
            TZA_PDET_SEL_HIR::_001 => 1,
            TZA_PDET_SEL_HIR::_010 => 2,
            TZA_PDET_SEL_HIR::_011 => 3,
            TZA_PDET_SEL_HIR::_100 => 4,
            TZA_PDET_SEL_HIR::_101 => 5,
            TZA_PDET_SEL_HIR::_110 => 6,
            TZA_PDET_SEL_HIR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TZA_PDET_SEL_HIR {
        match value {
            0 => TZA_PDET_SEL_HIR::_000,
            1 => TZA_PDET_SEL_HIR::_001,
            2 => TZA_PDET_SEL_HIR::_010,
            3 => TZA_PDET_SEL_HIR::_011,
            4 => TZA_PDET_SEL_HIR::_100,
            5 => TZA_PDET_SEL_HIR::_101,
            6 => TZA_PDET_SEL_HIR::_110,
            7 => TZA_PDET_SEL_HIR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == TZA_PDET_SEL_HIR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == TZA_PDET_SEL_HIR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == TZA_PDET_SEL_HIR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == TZA_PDET_SEL_HIR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == TZA_PDET_SEL_HIR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == TZA_PDET_SEL_HIR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == TZA_PDET_SEL_HIR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == TZA_PDET_SEL_HIR::_111
    }
}
#[doc = r" Value of the field"]
pub struct AGC_FAST_EXPIRER {
    bits: u8,
}
impl AGC_FAST_EXPIRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_LG_ON_OVRR {
    bits: bool,
}
impl LNA_LG_ON_OVRR {
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
pub struct LNA_HG_ON_OVRR {
    bits: bool,
}
impl LNA_HG_ON_OVRR {
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
#[doc = r" Proxy"]
pub struct _BBA_PDET_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_PDET_RSTW<'a> {
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
pub struct _TZA_PDET_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TZA_PDET_RSTW<'a> {
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
#[doc = "Values that can be written to the field `MAN_PDET_RST`"]
pub enum MAN_PDET_RSTW {
    #[doc = "The peak detector reset signals are controlled automatically by the AGC."]
    _0,
    #[doc = "The BBA_PDET_RST and TZA_PDET_RST are used to manually control the peak detector reset signals."]
    _1,
}
impl MAN_PDET_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAN_PDET_RSTW::_0 => false,
            MAN_PDET_RSTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAN_PDET_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MAN_PDET_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAN_PDET_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The peak detector reset signals are controlled automatically by the AGC."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAN_PDET_RSTW::_0)
    }
    #[doc = "The BBA_PDET_RST and TZA_PDET_RST are used to manually control the peak detector reset signals."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAN_PDET_RSTW::_1)
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
pub struct _BBA_GAIN_SETTLE_TIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_SETTLE_TIMEW<'a> {
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
#[doc = "Values that can be written to the field `BBA_PDET_SEL_LO`"]
pub enum BBA_PDET_SEL_LOW {
    #[doc = "0.600V"]
    _000,
    #[doc = "0.615V"]
    _001,
    #[doc = "0.630V"]
    _010,
    #[doc = "0.645V"]
    _011,
    #[doc = "0.660V"]
    _100,
    #[doc = "0.675V"]
    _101,
    #[doc = "0.690V"]
    _110,
    #[doc = "0.705V"]
    _111,
}
impl BBA_PDET_SEL_LOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BBA_PDET_SEL_LOW::_000 => 0,
            BBA_PDET_SEL_LOW::_001 => 1,
            BBA_PDET_SEL_LOW::_010 => 2,
            BBA_PDET_SEL_LOW::_011 => 3,
            BBA_PDET_SEL_LOW::_100 => 4,
            BBA_PDET_SEL_LOW::_101 => 5,
            BBA_PDET_SEL_LOW::_110 => 6,
            BBA_PDET_SEL_LOW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BBA_PDET_SEL_LOW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_PDET_SEL_LOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BBA_PDET_SEL_LOW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0.600V"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_LOW::_000)
    }
    #[doc = "0.615V"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_LOW::_001)
    }
    #[doc = "0.630V"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_LOW::_010)
    }
    #[doc = "0.645V"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_LOW::_011)
    }
    #[doc = "0.660V"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_LOW::_100)
    }
    #[doc = "0.675V"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_LOW::_101)
    }
    #[doc = "0.690V"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_LOW::_110)
    }
    #[doc = "0.705V"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_LOW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BBA_PDET_SEL_HI`"]
pub enum BBA_PDET_SEL_HIW {
    #[doc = "0.600V"]
    _000,
    #[doc = "0.795V"]
    _001,
    #[doc = "0.900V"]
    _010,
    #[doc = "0.945V"]
    _011,
    #[doc = "1.005V"]
    _100,
    #[doc = "1.050V"]
    _101,
    #[doc = "1.095V"]
    _110,
    #[doc = "1.155V"]
    _111,
}
impl BBA_PDET_SEL_HIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BBA_PDET_SEL_HIW::_000 => 0,
            BBA_PDET_SEL_HIW::_001 => 1,
            BBA_PDET_SEL_HIW::_010 => 2,
            BBA_PDET_SEL_HIW::_011 => 3,
            BBA_PDET_SEL_HIW::_100 => 4,
            BBA_PDET_SEL_HIW::_101 => 5,
            BBA_PDET_SEL_HIW::_110 => 6,
            BBA_PDET_SEL_HIW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BBA_PDET_SEL_HIW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_PDET_SEL_HIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BBA_PDET_SEL_HIW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0.600V"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_HIW::_000)
    }
    #[doc = "0.795V"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_HIW::_001)
    }
    #[doc = "0.900V"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_HIW::_010)
    }
    #[doc = "0.945V"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_HIW::_011)
    }
    #[doc = "1.005V"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_HIW::_100)
    }
    #[doc = "1.050V"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_HIW::_101)
    }
    #[doc = "1.095V"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_HIW::_110)
    }
    #[doc = "1.155V"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(BBA_PDET_SEL_HIW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TZA_PDET_SEL_LO`"]
pub enum TZA_PDET_SEL_LOW {
    #[doc = "0.600V"]
    _000,
    #[doc = "0.615V"]
    _001,
    #[doc = "0.630V"]
    _010,
    #[doc = "0.645V"]
    _011,
    #[doc = "0.660V"]
    _100,
    #[doc = "0.675V"]
    _101,
    #[doc = "0.690V"]
    _110,
    #[doc = "0.705V"]
    _111,
}
impl TZA_PDET_SEL_LOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TZA_PDET_SEL_LOW::_000 => 0,
            TZA_PDET_SEL_LOW::_001 => 1,
            TZA_PDET_SEL_LOW::_010 => 2,
            TZA_PDET_SEL_LOW::_011 => 3,
            TZA_PDET_SEL_LOW::_100 => 4,
            TZA_PDET_SEL_LOW::_101 => 5,
            TZA_PDET_SEL_LOW::_110 => 6,
            TZA_PDET_SEL_LOW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TZA_PDET_SEL_LOW<'a> {
    w: &'a mut W,
}
impl<'a> _TZA_PDET_SEL_LOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TZA_PDET_SEL_LOW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0.600V"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_LOW::_000)
    }
    #[doc = "0.615V"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_LOW::_001)
    }
    #[doc = "0.630V"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_LOW::_010)
    }
    #[doc = "0.645V"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_LOW::_011)
    }
    #[doc = "0.660V"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_LOW::_100)
    }
    #[doc = "0.675V"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_LOW::_101)
    }
    #[doc = "0.690V"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_LOW::_110)
    }
    #[doc = "0.705V"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_LOW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TZA_PDET_SEL_HI`"]
pub enum TZA_PDET_SEL_HIW {
    #[doc = "0.60V"]
    _000,
    #[doc = "0.63V"]
    _001,
    #[doc = "0.66V"]
    _010,
    #[doc = "0.69V"]
    _011,
    #[doc = "0.72V"]
    _100,
    #[doc = "0.75V"]
    _101,
    #[doc = "0.78V"]
    _110,
    #[doc = "0.81V"]
    _111,
}
impl TZA_PDET_SEL_HIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TZA_PDET_SEL_HIW::_000 => 0,
            TZA_PDET_SEL_HIW::_001 => 1,
            TZA_PDET_SEL_HIW::_010 => 2,
            TZA_PDET_SEL_HIW::_011 => 3,
            TZA_PDET_SEL_HIW::_100 => 4,
            TZA_PDET_SEL_HIW::_101 => 5,
            TZA_PDET_SEL_HIW::_110 => 6,
            TZA_PDET_SEL_HIW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TZA_PDET_SEL_HIW<'a> {
    w: &'a mut W,
}
impl<'a> _TZA_PDET_SEL_HIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TZA_PDET_SEL_HIW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0.60V"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_HIW::_000)
    }
    #[doc = "0.63V"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_HIW::_001)
    }
    #[doc = "0.66V"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_HIW::_010)
    }
    #[doc = "0.69V"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_HIW::_011)
    }
    #[doc = "0.72V"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_HIW::_100)
    }
    #[doc = "0.75V"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_HIW::_101)
    }
    #[doc = "0.78V"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_HIW::_110)
    }
    #[doc = "0.81V"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(TZA_PDET_SEL_HIW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AGC_FAST_EXPIREW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_FAST_EXPIREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LNA_LG_ON_OVRW<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_LG_ON_OVRW<'a> {
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
#[doc = r" Proxy"]
pub struct _LNA_HG_ON_OVRW<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_HG_ON_OVRW<'a> {
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
    #[doc = "Bit 0 - BBA PDET Reset"]
    #[inline]
    pub fn bba_pdet_rst(&self) -> BBA_PDET_RSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BBA_PDET_RSTR { bits }
    }
    #[doc = "Bit 1 - TZA PDET Reset"]
    #[inline]
    pub fn tza_pdet_rst(&self) -> TZA_PDET_RSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TZA_PDET_RSTR { bits }
    }
    #[doc = "Bit 2 - MAN PDET Reset"]
    #[inline]
    pub fn man_pdet_rst(&self) -> MAN_PDET_RSTR {
        MAN_PDET_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:11 - BBA Gain Settle Time"]
    #[inline]
    pub fn bba_gain_settle_time(&self) -> BBA_GAIN_SETTLE_TIMER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_SETTLE_TIMER { bits }
    }
    #[doc = "Bits 12:14 - BBA PDET Threshold Low"]
    #[inline]
    pub fn bba_pdet_sel_lo(&self) -> BBA_PDET_SEL_LOR {
        BBA_PDET_SEL_LOR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 15:17 - BBA PDET Threshold High"]
    #[inline]
    pub fn bba_pdet_sel_hi(&self) -> BBA_PDET_SEL_HIR {
        BBA_PDET_SEL_HIR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:20 - TZA PDET Threshold Low"]
    #[inline]
    pub fn tza_pdet_sel_lo(&self) -> TZA_PDET_SEL_LOR {
        TZA_PDET_SEL_LOR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:23 - TZA PDET Threshold High"]
    #[inline]
    pub fn tza_pdet_sel_hi(&self) -> TZA_PDET_SEL_HIR {
        TZA_PDET_SEL_HIR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - AGC Fast Expire"]
    #[inline]
    pub fn agc_fast_expire(&self) -> AGC_FAST_EXPIRER {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AGC_FAST_EXPIRER { bits }
    }
    #[doc = "Bit 30 - LNA_LG_ON override"]
    #[inline]
    pub fn lna_lg_on_ovr(&self) -> LNA_LG_ON_OVRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LNA_LG_ON_OVRR { bits }
    }
    #[doc = "Bit 31 - LNA_HG_ON override"]
    #[inline]
    pub fn lna_hg_on_ovr(&self) -> LNA_HG_ON_OVRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LNA_HG_ON_OVRR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 10915840 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - BBA PDET Reset"]
    #[inline]
    pub fn bba_pdet_rst(&mut self) -> _BBA_PDET_RSTW {
        _BBA_PDET_RSTW { w: self }
    }
    #[doc = "Bit 1 - TZA PDET Reset"]
    #[inline]
    pub fn tza_pdet_rst(&mut self) -> _TZA_PDET_RSTW {
        _TZA_PDET_RSTW { w: self }
    }
    #[doc = "Bit 2 - MAN PDET Reset"]
    #[inline]
    pub fn man_pdet_rst(&mut self) -> _MAN_PDET_RSTW {
        _MAN_PDET_RSTW { w: self }
    }
    #[doc = "Bits 4:11 - BBA Gain Settle Time"]
    #[inline]
    pub fn bba_gain_settle_time(&mut self) -> _BBA_GAIN_SETTLE_TIMEW {
        _BBA_GAIN_SETTLE_TIMEW { w: self }
    }
    #[doc = "Bits 12:14 - BBA PDET Threshold Low"]
    #[inline]
    pub fn bba_pdet_sel_lo(&mut self) -> _BBA_PDET_SEL_LOW {
        _BBA_PDET_SEL_LOW { w: self }
    }
    #[doc = "Bits 15:17 - BBA PDET Threshold High"]
    #[inline]
    pub fn bba_pdet_sel_hi(&mut self) -> _BBA_PDET_SEL_HIW {
        _BBA_PDET_SEL_HIW { w: self }
    }
    #[doc = "Bits 18:20 - TZA PDET Threshold Low"]
    #[inline]
    pub fn tza_pdet_sel_lo(&mut self) -> _TZA_PDET_SEL_LOW {
        _TZA_PDET_SEL_LOW { w: self }
    }
    #[doc = "Bits 21:23 - TZA PDET Threshold High"]
    #[inline]
    pub fn tza_pdet_sel_hi(&mut self) -> _TZA_PDET_SEL_HIW {
        _TZA_PDET_SEL_HIW { w: self }
    }
    #[doc = "Bits 24:29 - AGC Fast Expire"]
    #[inline]
    pub fn agc_fast_expire(&mut self) -> _AGC_FAST_EXPIREW {
        _AGC_FAST_EXPIREW { w: self }
    }
    #[doc = "Bit 30 - LNA_LG_ON override"]
    #[inline]
    pub fn lna_lg_on_ovr(&mut self) -> _LNA_LG_ON_OVRW {
        _LNA_LG_ON_OVRW { w: self }
    }
    #[doc = "Bit 31 - LNA_HG_ON override"]
    #[inline]
    pub fn lna_hg_on_ovr(&mut self) -> _LNA_HG_ON_OVRW {
        _LNA_HG_ON_OVRW { w: self }
    }
}
