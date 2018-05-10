#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BB_LDO_1 {
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
#[doc = "Possible values of the field `BB_LDO_ADCDAC_BYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_ADCDAC_BYPR {
    #[doc = "Bypass disabled."]
    _0,
    #[doc = "Bypass enabled"]
    _1,
}
impl BB_LDO_ADCDAC_BYPR {
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
            BB_LDO_ADCDAC_BYPR::_0 => false,
            BB_LDO_ADCDAC_BYPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_ADCDAC_BYPR {
        match value {
            false => BB_LDO_ADCDAC_BYPR::_0,
            true => BB_LDO_ADCDAC_BYPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_ADCDAC_BYPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_ADCDAC_BYPR::_1
    }
}
#[doc = "Possible values of the field `BB_LDO_ADCDAC_DIAGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_ADCDAC_DIAGSELR {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_ADCDAC_DIAGSELR {
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
            BB_LDO_ADCDAC_DIAGSELR::_0 => false,
            BB_LDO_ADCDAC_DIAGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_ADCDAC_DIAGSELR {
        match value {
            false => BB_LDO_ADCDAC_DIAGSELR::_0,
            true => BB_LDO_ADCDAC_DIAGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_ADCDAC_DIAGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_ADCDAC_DIAGSELR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_ADCDAC_SPARER {
    bits: u8,
}
impl BB_LDO_ADCDAC_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BB_LDO_ADCDAC_TRIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_ADCDAC_TRIMR {
    #[doc = "1.20 V ( Default )"]
    _000,
    #[doc = "1.25 V"]
    _001,
    #[doc = "1.28 V"]
    _010,
    #[doc = "1.33 V"]
    _011,
    #[doc = "1.40 V"]
    _100,
    #[doc = "1.44 V"]
    _101,
    #[doc = "1.50 V"]
    _110,
    #[doc = "1.66 V"]
    _111,
}
impl BB_LDO_ADCDAC_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BB_LDO_ADCDAC_TRIMR::_000 => 0,
            BB_LDO_ADCDAC_TRIMR::_001 => 1,
            BB_LDO_ADCDAC_TRIMR::_010 => 2,
            BB_LDO_ADCDAC_TRIMR::_011 => 3,
            BB_LDO_ADCDAC_TRIMR::_100 => 4,
            BB_LDO_ADCDAC_TRIMR::_101 => 5,
            BB_LDO_ADCDAC_TRIMR::_110 => 6,
            BB_LDO_ADCDAC_TRIMR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BB_LDO_ADCDAC_TRIMR {
        match value {
            0 => BB_LDO_ADCDAC_TRIMR::_000,
            1 => BB_LDO_ADCDAC_TRIMR::_001,
            2 => BB_LDO_ADCDAC_TRIMR::_010,
            3 => BB_LDO_ADCDAC_TRIMR::_011,
            4 => BB_LDO_ADCDAC_TRIMR::_100,
            5 => BB_LDO_ADCDAC_TRIMR::_101,
            6 => BB_LDO_ADCDAC_TRIMR::_110,
            7 => BB_LDO_ADCDAC_TRIMR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == BB_LDO_ADCDAC_TRIMR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == BB_LDO_ADCDAC_TRIMR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == BB_LDO_ADCDAC_TRIMR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == BB_LDO_ADCDAC_TRIMR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == BB_LDO_ADCDAC_TRIMR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == BB_LDO_ADCDAC_TRIMR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == BB_LDO_ADCDAC_TRIMR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == BB_LDO_ADCDAC_TRIMR::_111
    }
}
#[doc = "Possible values of the field `BB_LDO_BBA_BYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_BBA_BYPR {
    #[doc = "Bypass disabled."]
    _0,
    #[doc = "Bypass enabled"]
    _1,
}
impl BB_LDO_BBA_BYPR {
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
            BB_LDO_BBA_BYPR::_0 => false,
            BB_LDO_BBA_BYPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_BBA_BYPR {
        match value {
            false => BB_LDO_BBA_BYPR::_0,
            true => BB_LDO_BBA_BYPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_BBA_BYPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_BBA_BYPR::_1
    }
}
#[doc = "Possible values of the field `BB_LDO_BBA_DIAGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_BBA_DIAGSELR {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_BBA_DIAGSELR {
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
            BB_LDO_BBA_DIAGSELR::_0 => false,
            BB_LDO_BBA_DIAGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_BBA_DIAGSELR {
        match value {
            false => BB_LDO_BBA_DIAGSELR::_0,
            true => BB_LDO_BBA_DIAGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_BBA_DIAGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_BBA_DIAGSELR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_BBA_SPARER {
    bits: u8,
}
impl BB_LDO_BBA_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BB_LDO_BBA_TRIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_BBA_TRIMR {
    #[doc = "1.20 V ( Default )"]
    _0,
    #[doc = "1.25 V"]
    _1,
    #[doc = "1.28 V"]
    _2,
    #[doc = "1.33 V"]
    _3,
    #[doc = "1.40 V"]
    _4,
    #[doc = "1.44 V"]
    _5,
    #[doc = "1.50 V"]
    _6,
    #[doc = "1.66 V"]
    _7,
}
impl BB_LDO_BBA_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BB_LDO_BBA_TRIMR::_0 => 0,
            BB_LDO_BBA_TRIMR::_1 => 1,
            BB_LDO_BBA_TRIMR::_2 => 2,
            BB_LDO_BBA_TRIMR::_3 => 3,
            BB_LDO_BBA_TRIMR::_4 => 4,
            BB_LDO_BBA_TRIMR::_5 => 5,
            BB_LDO_BBA_TRIMR::_6 => 6,
            BB_LDO_BBA_TRIMR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BB_LDO_BBA_TRIMR {
        match value {
            0 => BB_LDO_BBA_TRIMR::_0,
            1 => BB_LDO_BBA_TRIMR::_1,
            2 => BB_LDO_BBA_TRIMR::_2,
            3 => BB_LDO_BBA_TRIMR::_3,
            4 => BB_LDO_BBA_TRIMR::_4,
            5 => BB_LDO_BBA_TRIMR::_5,
            6 => BB_LDO_BBA_TRIMR::_6,
            7 => BB_LDO_BBA_TRIMR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_BBA_TRIMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_BBA_TRIMR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == BB_LDO_BBA_TRIMR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == BB_LDO_BBA_TRIMR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == BB_LDO_BBA_TRIMR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == BB_LDO_BBA_TRIMR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == BB_LDO_BBA_TRIMR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == BB_LDO_BBA_TRIMR::_7
    }
}
#[doc = "Possible values of the field `BB_LDO_FDBK_BYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_FDBK_BYPR {
    #[doc = "Bypass disabled."]
    _0,
    #[doc = "Bypass enabled"]
    _1,
}
impl BB_LDO_FDBK_BYPR {
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
            BB_LDO_FDBK_BYPR::_0 => false,
            BB_LDO_FDBK_BYPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_FDBK_BYPR {
        match value {
            false => BB_LDO_FDBK_BYPR::_0,
            true => BB_LDO_FDBK_BYPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_FDBK_BYPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_FDBK_BYPR::_1
    }
}
#[doc = "Possible values of the field `BB_LDO_FDBK_DIAGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_FDBK_DIAGSELR {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_FDBK_DIAGSELR {
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
            BB_LDO_FDBK_DIAGSELR::_0 => false,
            BB_LDO_FDBK_DIAGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_FDBK_DIAGSELR {
        match value {
            false => BB_LDO_FDBK_DIAGSELR::_0,
            true => BB_LDO_FDBK_DIAGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_FDBK_DIAGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_FDBK_DIAGSELR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_FDBK_SPARER {
    bits: u8,
}
impl BB_LDO_FDBK_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BB_LDO_FDBK_TRIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_FDBK_TRIMR {
    #[doc = "1.2/1.176 V ( Default )"]
    _0,
    #[doc = "1.138/1.115 V"]
    _1,
    #[doc = "1.085/1.066 V"]
    _2,
    #[doc = "1.04/1.025 V"]
    _3,
    #[doc = "1.28/1.25 V"]
    _4,
    #[doc = "1.4/1.35 V"]
    _5,
    #[doc = "1.55/1.4 V"]
    _6,
    #[doc = "1.78/1.4 V"]
    _7,
}
impl BB_LDO_FDBK_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BB_LDO_FDBK_TRIMR::_0 => 0,
            BB_LDO_FDBK_TRIMR::_1 => 1,
            BB_LDO_FDBK_TRIMR::_2 => 2,
            BB_LDO_FDBK_TRIMR::_3 => 3,
            BB_LDO_FDBK_TRIMR::_4 => 4,
            BB_LDO_FDBK_TRIMR::_5 => 5,
            BB_LDO_FDBK_TRIMR::_6 => 6,
            BB_LDO_FDBK_TRIMR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BB_LDO_FDBK_TRIMR {
        match value {
            0 => BB_LDO_FDBK_TRIMR::_0,
            1 => BB_LDO_FDBK_TRIMR::_1,
            2 => BB_LDO_FDBK_TRIMR::_2,
            3 => BB_LDO_FDBK_TRIMR::_3,
            4 => BB_LDO_FDBK_TRIMR::_4,
            5 => BB_LDO_FDBK_TRIMR::_5,
            6 => BB_LDO_FDBK_TRIMR::_6,
            7 => BB_LDO_FDBK_TRIMR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_FDBK_TRIMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_FDBK_TRIMR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == BB_LDO_FDBK_TRIMR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == BB_LDO_FDBK_TRIMR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == BB_LDO_FDBK_TRIMR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == BB_LDO_FDBK_TRIMR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == BB_LDO_FDBK_TRIMR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == BB_LDO_FDBK_TRIMR::_7
    }
}
#[doc = "Possible values of the field `BB_LDO_HF_BYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_HF_BYPR {
    #[doc = "Bypass disabled."]
    _0,
    #[doc = "Bypass enabled"]
    _1,
}
impl BB_LDO_HF_BYPR {
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
            BB_LDO_HF_BYPR::_0 => false,
            BB_LDO_HF_BYPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_HF_BYPR {
        match value {
            false => BB_LDO_HF_BYPR::_0,
            true => BB_LDO_HF_BYPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_HF_BYPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_HF_BYPR::_1
    }
}
#[doc = "Possible values of the field `BB_LDO_HF_DIAGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_HF_DIAGSELR {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_HF_DIAGSELR {
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
            BB_LDO_HF_DIAGSELR::_0 => false,
            BB_LDO_HF_DIAGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_HF_DIAGSELR {
        match value {
            false => BB_LDO_HF_DIAGSELR::_0,
            true => BB_LDO_HF_DIAGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_HF_DIAGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_HF_DIAGSELR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_HF_SPARER {
    bits: u8,
}
impl BB_LDO_HF_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BB_LDO_HF_TRIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_HF_TRIMR {
    #[doc = "1.20 V ( Default )"]
    _0,
    #[doc = "1.25 V"]
    _1,
    #[doc = "1.28 V"]
    _2,
    #[doc = "1.33 V"]
    _3,
    #[doc = "1.40 V"]
    _4,
    #[doc = "1.44 V"]
    _5,
    #[doc = "1.50 V"]
    _6,
    #[doc = "1.66 V"]
    _7,
}
impl BB_LDO_HF_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BB_LDO_HF_TRIMR::_0 => 0,
            BB_LDO_HF_TRIMR::_1 => 1,
            BB_LDO_HF_TRIMR::_2 => 2,
            BB_LDO_HF_TRIMR::_3 => 3,
            BB_LDO_HF_TRIMR::_4 => 4,
            BB_LDO_HF_TRIMR::_5 => 5,
            BB_LDO_HF_TRIMR::_6 => 6,
            BB_LDO_HF_TRIMR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BB_LDO_HF_TRIMR {
        match value {
            0 => BB_LDO_HF_TRIMR::_0,
            1 => BB_LDO_HF_TRIMR::_1,
            2 => BB_LDO_HF_TRIMR::_2,
            3 => BB_LDO_HF_TRIMR::_3,
            4 => BB_LDO_HF_TRIMR::_4,
            5 => BB_LDO_HF_TRIMR::_5,
            6 => BB_LDO_HF_TRIMR::_6,
            7 => BB_LDO_HF_TRIMR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_HF_TRIMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_HF_TRIMR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == BB_LDO_HF_TRIMR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == BB_LDO_HF_TRIMR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == BB_LDO_HF_TRIMR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == BB_LDO_HF_TRIMR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == BB_LDO_HF_TRIMR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == BB_LDO_HF_TRIMR::_7
    }
}
#[doc = "Values that can be written to the field `BB_LDO_ADCDAC_BYP`"]
pub enum BB_LDO_ADCDAC_BYPW {
    #[doc = "Bypass disabled."]
    _0,
    #[doc = "Bypass enabled"]
    _1,
}
impl BB_LDO_ADCDAC_BYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_ADCDAC_BYPW::_0 => false,
            BB_LDO_ADCDAC_BYPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_ADCDAC_BYPW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_ADCDAC_BYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_ADCDAC_BYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bypass disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_BYPW::_0)
    }
    #[doc = "Bypass enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_BYPW::_1)
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
#[doc = "Values that can be written to the field `BB_LDO_ADCDAC_DIAGSEL`"]
pub enum BB_LDO_ADCDAC_DIAGSELW {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_ADCDAC_DIAGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_ADCDAC_DIAGSELW::_0 => false,
            BB_LDO_ADCDAC_DIAGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_ADCDAC_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_ADCDAC_DIAGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_ADCDAC_DIAGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diag disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_DIAGSELW::_0)
    }
    #[doc = "Diag enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_DIAGSELW::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_ADCDAC_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_ADCDAC_SPAREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB_LDO_ADCDAC_TRIM`"]
pub enum BB_LDO_ADCDAC_TRIMW {
    #[doc = "1.20 V ( Default )"]
    _000,
    #[doc = "1.25 V"]
    _001,
    #[doc = "1.28 V"]
    _010,
    #[doc = "1.33 V"]
    _011,
    #[doc = "1.40 V"]
    _100,
    #[doc = "1.44 V"]
    _101,
    #[doc = "1.50 V"]
    _110,
    #[doc = "1.66 V"]
    _111,
}
impl BB_LDO_ADCDAC_TRIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BB_LDO_ADCDAC_TRIMW::_000 => 0,
            BB_LDO_ADCDAC_TRIMW::_001 => 1,
            BB_LDO_ADCDAC_TRIMW::_010 => 2,
            BB_LDO_ADCDAC_TRIMW::_011 => 3,
            BB_LDO_ADCDAC_TRIMW::_100 => 4,
            BB_LDO_ADCDAC_TRIMW::_101 => 5,
            BB_LDO_ADCDAC_TRIMW::_110 => 6,
            BB_LDO_ADCDAC_TRIMW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_ADCDAC_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_ADCDAC_TRIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_ADCDAC_TRIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.20 V ( Default )"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_TRIMW::_000)
    }
    #[doc = "1.25 V"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_TRIMW::_001)
    }
    #[doc = "1.28 V"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_TRIMW::_010)
    }
    #[doc = "1.33 V"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_TRIMW::_011)
    }
    #[doc = "1.40 V"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_TRIMW::_100)
    }
    #[doc = "1.44 V"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_TRIMW::_101)
    }
    #[doc = "1.50 V"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_TRIMW::_110)
    }
    #[doc = "1.66 V"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_TRIMW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB_LDO_BBA_BYP`"]
pub enum BB_LDO_BBA_BYPW {
    #[doc = "Bypass disabled."]
    _0,
    #[doc = "Bypass enabled"]
    _1,
}
impl BB_LDO_BBA_BYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_BBA_BYPW::_0 => false,
            BB_LDO_BBA_BYPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_BBA_BYPW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_BBA_BYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_BBA_BYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bypass disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_BYPW::_0)
    }
    #[doc = "Bypass enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_BYPW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB_LDO_BBA_DIAGSEL`"]
pub enum BB_LDO_BBA_DIAGSELW {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_BBA_DIAGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_BBA_DIAGSELW::_0 => false,
            BB_LDO_BBA_DIAGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_BBA_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_BBA_DIAGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_BBA_DIAGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diag disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_DIAGSELW::_0)
    }
    #[doc = "Diag enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_DIAGSELW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_BBA_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_BBA_SPAREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB_LDO_BBA_TRIM`"]
pub enum BB_LDO_BBA_TRIMW {
    #[doc = "1.20 V ( Default )"]
    _0,
    #[doc = "1.25 V"]
    _1,
    #[doc = "1.28 V"]
    _2,
    #[doc = "1.33 V"]
    _3,
    #[doc = "1.40 V"]
    _4,
    #[doc = "1.44 V"]
    _5,
    #[doc = "1.50 V"]
    _6,
    #[doc = "1.66 V"]
    _7,
}
impl BB_LDO_BBA_TRIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BB_LDO_BBA_TRIMW::_0 => 0,
            BB_LDO_BBA_TRIMW::_1 => 1,
            BB_LDO_BBA_TRIMW::_2 => 2,
            BB_LDO_BBA_TRIMW::_3 => 3,
            BB_LDO_BBA_TRIMW::_4 => 4,
            BB_LDO_BBA_TRIMW::_5 => 5,
            BB_LDO_BBA_TRIMW::_6 => 6,
            BB_LDO_BBA_TRIMW::_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_BBA_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_BBA_TRIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_BBA_TRIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.20 V ( Default )"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_TRIMW::_0)
    }
    #[doc = "1.25 V"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_TRIMW::_1)
    }
    #[doc = "1.28 V"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_TRIMW::_2)
    }
    #[doc = "1.33 V"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_TRIMW::_3)
    }
    #[doc = "1.40 V"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_TRIMW::_4)
    }
    #[doc = "1.44 V"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_TRIMW::_5)
    }
    #[doc = "1.50 V"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_TRIMW::_6)
    }
    #[doc = "1.66 V"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_TRIMW::_7)
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
#[doc = "Values that can be written to the field `BB_LDO_FDBK_BYP`"]
pub enum BB_LDO_FDBK_BYPW {
    #[doc = "Bypass disabled."]
    _0,
    #[doc = "Bypass enabled"]
    _1,
}
impl BB_LDO_FDBK_BYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_FDBK_BYPW::_0 => false,
            BB_LDO_FDBK_BYPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_FDBK_BYPW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_FDBK_BYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_FDBK_BYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bypass disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_BYPW::_0)
    }
    #[doc = "Bypass enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_BYPW::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB_LDO_FDBK_DIAGSEL`"]
pub enum BB_LDO_FDBK_DIAGSELW {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_FDBK_DIAGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_FDBK_DIAGSELW::_0 => false,
            BB_LDO_FDBK_DIAGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_FDBK_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_FDBK_DIAGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_FDBK_DIAGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diag disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_DIAGSELW::_0)
    }
    #[doc = "Diag enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_DIAGSELW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_FDBK_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_FDBK_SPAREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB_LDO_FDBK_TRIM`"]
pub enum BB_LDO_FDBK_TRIMW {
    #[doc = "1.2/1.176 V ( Default )"]
    _0,
    #[doc = "1.138/1.115 V"]
    _1,
    #[doc = "1.085/1.066 V"]
    _2,
    #[doc = "1.04/1.025 V"]
    _3,
    #[doc = "1.28/1.25 V"]
    _4,
    #[doc = "1.4/1.35 V"]
    _5,
    #[doc = "1.55/1.4 V"]
    _6,
    #[doc = "1.78/1.4 V"]
    _7,
}
impl BB_LDO_FDBK_TRIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BB_LDO_FDBK_TRIMW::_0 => 0,
            BB_LDO_FDBK_TRIMW::_1 => 1,
            BB_LDO_FDBK_TRIMW::_2 => 2,
            BB_LDO_FDBK_TRIMW::_3 => 3,
            BB_LDO_FDBK_TRIMW::_4 => 4,
            BB_LDO_FDBK_TRIMW::_5 => 5,
            BB_LDO_FDBK_TRIMW::_6 => 6,
            BB_LDO_FDBK_TRIMW::_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_FDBK_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_FDBK_TRIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_FDBK_TRIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.2/1.176 V ( Default )"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_TRIMW::_0)
    }
    #[doc = "1.138/1.115 V"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_TRIMW::_1)
    }
    #[doc = "1.085/1.066 V"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_TRIMW::_2)
    }
    #[doc = "1.04/1.025 V"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_TRIMW::_3)
    }
    #[doc = "1.28/1.25 V"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_TRIMW::_4)
    }
    #[doc = "1.4/1.35 V"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_TRIMW::_5)
    }
    #[doc = "1.55/1.4 V"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_TRIMW::_6)
    }
    #[doc = "1.78/1.4 V"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_TRIMW::_7)
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
#[doc = "Values that can be written to the field `BB_LDO_HF_BYP`"]
pub enum BB_LDO_HF_BYPW {
    #[doc = "Bypass disabled."]
    _0,
    #[doc = "Bypass enabled"]
    _1,
}
impl BB_LDO_HF_BYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_HF_BYPW::_0 => false,
            BB_LDO_HF_BYPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_HF_BYPW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_HF_BYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_HF_BYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bypass disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_HF_BYPW::_0)
    }
    #[doc = "Bypass enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_HF_BYPW::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB_LDO_HF_DIAGSEL`"]
pub enum BB_LDO_HF_DIAGSELW {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_HF_DIAGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_HF_DIAGSELW::_0 => false,
            BB_LDO_HF_DIAGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_HF_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_HF_DIAGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_HF_DIAGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diag disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_HF_DIAGSELW::_0)
    }
    #[doc = "Diag enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_HF_DIAGSELW::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_HF_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_HF_SPAREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB_LDO_HF_TRIM`"]
pub enum BB_LDO_HF_TRIMW {
    #[doc = "1.20 V ( Default )"]
    _0,
    #[doc = "1.25 V"]
    _1,
    #[doc = "1.28 V"]
    _2,
    #[doc = "1.33 V"]
    _3,
    #[doc = "1.40 V"]
    _4,
    #[doc = "1.44 V"]
    _5,
    #[doc = "1.50 V"]
    _6,
    #[doc = "1.66 V"]
    _7,
}
impl BB_LDO_HF_TRIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BB_LDO_HF_TRIMW::_0 => 0,
            BB_LDO_HF_TRIMW::_1 => 1,
            BB_LDO_HF_TRIMW::_2 => 2,
            BB_LDO_HF_TRIMW::_3 => 3,
            BB_LDO_HF_TRIMW::_4 => 4,
            BB_LDO_HF_TRIMW::_5 => 5,
            BB_LDO_HF_TRIMW::_6 => 6,
            BB_LDO_HF_TRIMW::_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_HF_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_HF_TRIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_HF_TRIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.20 V ( Default )"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_HF_TRIMW::_0)
    }
    #[doc = "1.25 V"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_HF_TRIMW::_1)
    }
    #[doc = "1.28 V"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(BB_LDO_HF_TRIMW::_2)
    }
    #[doc = "1.33 V"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(BB_LDO_HF_TRIMW::_3)
    }
    #[doc = "1.40 V"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(BB_LDO_HF_TRIMW::_4)
    }
    #[doc = "1.44 V"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(BB_LDO_HF_TRIMW::_5)
    }
    #[doc = "1.50 V"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(BB_LDO_HF_TRIMW::_6)
    }
    #[doc = "1.66 V"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(BB_LDO_HF_TRIMW::_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bit 0 - rmap_bb_ldo_adcdac_byp"]
    #[inline]
    pub fn bb_ldo_adcdac_byp(&self) -> BB_LDO_ADCDAC_BYPR {
        BB_LDO_ADCDAC_BYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - rmap_bb_ldo_adcdac_diagsel"]
    #[inline]
    pub fn bb_ldo_adcdac_diagsel(&self) -> BB_LDO_ADCDAC_DIAGSELR {
        BB_LDO_ADCDAC_DIAGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - rmap_bb_ldo_adcdac_spare[1:0]"]
    #[inline]
    pub fn bb_ldo_adcdac_spare(&self) -> BB_LDO_ADCDAC_SPARER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_LDO_ADCDAC_SPARER { bits }
    }
    #[doc = "Bits 4:6 - rmap_bb_ldo_adcdac_trim[2:0]"]
    #[inline]
    pub fn bb_ldo_adcdac_trim(&self) -> BB_LDO_ADCDAC_TRIMR {
        BB_LDO_ADCDAC_TRIMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - rmap_bb_ldo_bba_byp"]
    #[inline]
    pub fn bb_ldo_bba_byp(&self) -> BB_LDO_BBA_BYPR {
        BB_LDO_BBA_BYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - rmap_bb_ldo_bba_diagsel"]
    #[inline]
    pub fn bb_ldo_bba_diagsel(&self) -> BB_LDO_BBA_DIAGSELR {
        BB_LDO_BBA_DIAGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:11 - rmap_bb_ldo_bba_spare[1:0]"]
    #[inline]
    pub fn bb_ldo_bba_spare(&self) -> BB_LDO_BBA_SPARER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_LDO_BBA_SPARER { bits }
    }
    #[doc = "Bits 12:14 - rmap_bb_ldo_bba_trim[2:0]"]
    #[inline]
    pub fn bb_ldo_bba_trim(&self) -> BB_LDO_BBA_TRIMR {
        BB_LDO_BBA_TRIMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - rmap_bb_ldo_fdbk_byp"]
    #[inline]
    pub fn bb_ldo_fdbk_byp(&self) -> BB_LDO_FDBK_BYPR {
        BB_LDO_FDBK_BYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - rmap_bb_ldo_fdbk_diagsel"]
    #[inline]
    pub fn bb_ldo_fdbk_diagsel(&self) -> BB_LDO_FDBK_DIAGSELR {
        BB_LDO_FDBK_DIAGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:19 - rmap_bb_ldo_fdbk_spare[1:0]"]
    #[inline]
    pub fn bb_ldo_fdbk_spare(&self) -> BB_LDO_FDBK_SPARER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_LDO_FDBK_SPARER { bits }
    }
    #[doc = "Bits 20:22 - rmap_bb_ldo_fdbk_trim[2:0]"]
    #[inline]
    pub fn bb_ldo_fdbk_trim(&self) -> BB_LDO_FDBK_TRIMR {
        BB_LDO_FDBK_TRIMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - rmap_bb_ldo_hf_byp"]
    #[inline]
    pub fn bb_ldo_hf_byp(&self) -> BB_LDO_HF_BYPR {
        BB_LDO_HF_BYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - rmap_bb_ldo_hf_diagsel"]
    #[inline]
    pub fn bb_ldo_hf_diagsel(&self) -> BB_LDO_HF_DIAGSELR {
        BB_LDO_HF_DIAGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 26:27 - rmap_bb_ldo_hf_spare[1:0]"]
    #[inline]
    pub fn bb_ldo_hf_spare(&self) -> BB_LDO_HF_SPARER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_LDO_HF_SPARER { bits }
    }
    #[doc = "Bits 28:30 - rmap_bb_ldo_hf_trim[2:0]"]
    #[inline]
    pub fn bb_ldo_hf_trim(&self) -> BB_LDO_HF_TRIMR {
        BB_LDO_HF_TRIMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - rmap_bb_ldo_adcdac_byp"]
    #[inline]
    pub fn bb_ldo_adcdac_byp(&mut self) -> _BB_LDO_ADCDAC_BYPW {
        _BB_LDO_ADCDAC_BYPW { w: self }
    }
    #[doc = "Bit 1 - rmap_bb_ldo_adcdac_diagsel"]
    #[inline]
    pub fn bb_ldo_adcdac_diagsel(&mut self) -> _BB_LDO_ADCDAC_DIAGSELW {
        _BB_LDO_ADCDAC_DIAGSELW { w: self }
    }
    #[doc = "Bits 2:3 - rmap_bb_ldo_adcdac_spare[1:0]"]
    #[inline]
    pub fn bb_ldo_adcdac_spare(&mut self) -> _BB_LDO_ADCDAC_SPAREW {
        _BB_LDO_ADCDAC_SPAREW { w: self }
    }
    #[doc = "Bits 4:6 - rmap_bb_ldo_adcdac_trim[2:0]"]
    #[inline]
    pub fn bb_ldo_adcdac_trim(&mut self) -> _BB_LDO_ADCDAC_TRIMW {
        _BB_LDO_ADCDAC_TRIMW { w: self }
    }
    #[doc = "Bit 8 - rmap_bb_ldo_bba_byp"]
    #[inline]
    pub fn bb_ldo_bba_byp(&mut self) -> _BB_LDO_BBA_BYPW {
        _BB_LDO_BBA_BYPW { w: self }
    }
    #[doc = "Bit 9 - rmap_bb_ldo_bba_diagsel"]
    #[inline]
    pub fn bb_ldo_bba_diagsel(&mut self) -> _BB_LDO_BBA_DIAGSELW {
        _BB_LDO_BBA_DIAGSELW { w: self }
    }
    #[doc = "Bits 10:11 - rmap_bb_ldo_bba_spare[1:0]"]
    #[inline]
    pub fn bb_ldo_bba_spare(&mut self) -> _BB_LDO_BBA_SPAREW {
        _BB_LDO_BBA_SPAREW { w: self }
    }
    #[doc = "Bits 12:14 - rmap_bb_ldo_bba_trim[2:0]"]
    #[inline]
    pub fn bb_ldo_bba_trim(&mut self) -> _BB_LDO_BBA_TRIMW {
        _BB_LDO_BBA_TRIMW { w: self }
    }
    #[doc = "Bit 16 - rmap_bb_ldo_fdbk_byp"]
    #[inline]
    pub fn bb_ldo_fdbk_byp(&mut self) -> _BB_LDO_FDBK_BYPW {
        _BB_LDO_FDBK_BYPW { w: self }
    }
    #[doc = "Bit 17 - rmap_bb_ldo_fdbk_diagsel"]
    #[inline]
    pub fn bb_ldo_fdbk_diagsel(&mut self) -> _BB_LDO_FDBK_DIAGSELW {
        _BB_LDO_FDBK_DIAGSELW { w: self }
    }
    #[doc = "Bits 18:19 - rmap_bb_ldo_fdbk_spare[1:0]"]
    #[inline]
    pub fn bb_ldo_fdbk_spare(&mut self) -> _BB_LDO_FDBK_SPAREW {
        _BB_LDO_FDBK_SPAREW { w: self }
    }
    #[doc = "Bits 20:22 - rmap_bb_ldo_fdbk_trim[2:0]"]
    #[inline]
    pub fn bb_ldo_fdbk_trim(&mut self) -> _BB_LDO_FDBK_TRIMW {
        _BB_LDO_FDBK_TRIMW { w: self }
    }
    #[doc = "Bit 24 - rmap_bb_ldo_hf_byp"]
    #[inline]
    pub fn bb_ldo_hf_byp(&mut self) -> _BB_LDO_HF_BYPW {
        _BB_LDO_HF_BYPW { w: self }
    }
    #[doc = "Bit 25 - rmap_bb_ldo_hf_diagsel"]
    #[inline]
    pub fn bb_ldo_hf_diagsel(&mut self) -> _BB_LDO_HF_DIAGSELW {
        _BB_LDO_HF_DIAGSELW { w: self }
    }
    #[doc = "Bits 26:27 - rmap_bb_ldo_hf_spare[1:0]"]
    #[inline]
    pub fn bb_ldo_hf_spare(&mut self) -> _BB_LDO_HF_SPAREW {
        _BB_LDO_HF_SPAREW { w: self }
    }
    #[doc = "Bits 28:30 - rmap_bb_ldo_hf_trim[2:0]"]
    #[inline]
    pub fn bb_ldo_hf_trim(&mut self) -> _BB_LDO_HF_TRIMW {
        _BB_LDO_HF_TRIMW { w: self }
    }
}
