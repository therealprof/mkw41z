#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCOC_CTRL_1 {
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
#[doc = "Possible values of the field `DCOC_SIGN_SCALE_IDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_SIGN_SCALE_IDXR {
    #[doc = "1/8"]
    _00,
    #[doc = "1/16"]
    _01,
    #[doc = "1/32"]
    _10,
    #[doc = "1/64"]
    _11,
}
impl DCOC_SIGN_SCALE_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCOC_SIGN_SCALE_IDXR::_00 => 0,
            DCOC_SIGN_SCALE_IDXR::_01 => 1,
            DCOC_SIGN_SCALE_IDXR::_10 => 2,
            DCOC_SIGN_SCALE_IDXR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCOC_SIGN_SCALE_IDXR {
        match value {
            0 => DCOC_SIGN_SCALE_IDXR::_00,
            1 => DCOC_SIGN_SCALE_IDXR::_01,
            2 => DCOC_SIGN_SCALE_IDXR::_10,
            3 => DCOC_SIGN_SCALE_IDXR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DCOC_SIGN_SCALE_IDXR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DCOC_SIGN_SCALE_IDXR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DCOC_SIGN_SCALE_IDXR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DCOC_SIGN_SCALE_IDXR::_11
    }
}
#[doc = "Possible values of the field `DCOC_ALPHAC_SCALE_IDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_ALPHAC_SCALE_IDXR {
    #[doc = "1/2"]
    _000,
    #[doc = "1/4"]
    _001,
    #[doc = "1/8"]
    _010,
    #[doc = "1/16"]
    _011,
    #[doc = "1/32"]
    _100,
    #[doc = "1/64"]
    _101,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DCOC_ALPHAC_SCALE_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCOC_ALPHAC_SCALE_IDXR::_000 => 0,
            DCOC_ALPHAC_SCALE_IDXR::_001 => 1,
            DCOC_ALPHAC_SCALE_IDXR::_010 => 2,
            DCOC_ALPHAC_SCALE_IDXR::_011 => 3,
            DCOC_ALPHAC_SCALE_IDXR::_100 => 4,
            DCOC_ALPHAC_SCALE_IDXR::_101 => 5,
            DCOC_ALPHAC_SCALE_IDXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCOC_ALPHAC_SCALE_IDXR {
        match value {
            0 => DCOC_ALPHAC_SCALE_IDXR::_000,
            1 => DCOC_ALPHAC_SCALE_IDXR::_001,
            2 => DCOC_ALPHAC_SCALE_IDXR::_010,
            3 => DCOC_ALPHAC_SCALE_IDXR::_011,
            4 => DCOC_ALPHAC_SCALE_IDXR::_100,
            5 => DCOC_ALPHAC_SCALE_IDXR::_101,
            i => DCOC_ALPHAC_SCALE_IDXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == DCOC_ALPHAC_SCALE_IDXR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == DCOC_ALPHAC_SCALE_IDXR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == DCOC_ALPHAC_SCALE_IDXR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == DCOC_ALPHAC_SCALE_IDXR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == DCOC_ALPHAC_SCALE_IDXR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == DCOC_ALPHAC_SCALE_IDXR::_101
    }
}
#[doc = "Possible values of the field `DCOC_ALPHA_RADIUS_IDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_ALPHA_RADIUS_IDXR {
    #[doc = "1"]
    _000,
    #[doc = "1/2"]
    _001,
    #[doc = "1/4"]
    _010,
    #[doc = "1/8"]
    _011,
    #[doc = "1/16"]
    _100,
    #[doc = "1/32"]
    _101,
    #[doc = "1/64"]
    _110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DCOC_ALPHA_RADIUS_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCOC_ALPHA_RADIUS_IDXR::_000 => 0,
            DCOC_ALPHA_RADIUS_IDXR::_001 => 1,
            DCOC_ALPHA_RADIUS_IDXR::_010 => 2,
            DCOC_ALPHA_RADIUS_IDXR::_011 => 3,
            DCOC_ALPHA_RADIUS_IDXR::_100 => 4,
            DCOC_ALPHA_RADIUS_IDXR::_101 => 5,
            DCOC_ALPHA_RADIUS_IDXR::_110 => 6,
            DCOC_ALPHA_RADIUS_IDXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCOC_ALPHA_RADIUS_IDXR {
        match value {
            0 => DCOC_ALPHA_RADIUS_IDXR::_000,
            1 => DCOC_ALPHA_RADIUS_IDXR::_001,
            2 => DCOC_ALPHA_RADIUS_IDXR::_010,
            3 => DCOC_ALPHA_RADIUS_IDXR::_011,
            4 => DCOC_ALPHA_RADIUS_IDXR::_100,
            5 => DCOC_ALPHA_RADIUS_IDXR::_101,
            6 => DCOC_ALPHA_RADIUS_IDXR::_110,
            i => DCOC_ALPHA_RADIUS_IDXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_IDXR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_IDXR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_IDXR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_IDXR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_IDXR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_IDXR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_IDXR::_110
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_TRK_EST_GS_CNTR {
    bits: u8,
}
impl DCOC_TRK_EST_GS_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DCOC_SIGN_SCALE_GS_IDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_SIGN_SCALE_GS_IDXR {
    #[doc = "1/8"]
    _00,
    #[doc = "1/16"]
    _01,
    #[doc = "1/32"]
    _10,
    #[doc = "1/64"]
    _11,
}
impl DCOC_SIGN_SCALE_GS_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCOC_SIGN_SCALE_GS_IDXR::_00 => 0,
            DCOC_SIGN_SCALE_GS_IDXR::_01 => 1,
            DCOC_SIGN_SCALE_GS_IDXR::_10 => 2,
            DCOC_SIGN_SCALE_GS_IDXR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCOC_SIGN_SCALE_GS_IDXR {
        match value {
            0 => DCOC_SIGN_SCALE_GS_IDXR::_00,
            1 => DCOC_SIGN_SCALE_GS_IDXR::_01,
            2 => DCOC_SIGN_SCALE_GS_IDXR::_10,
            3 => DCOC_SIGN_SCALE_GS_IDXR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DCOC_SIGN_SCALE_GS_IDXR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DCOC_SIGN_SCALE_GS_IDXR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DCOC_SIGN_SCALE_GS_IDXR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DCOC_SIGN_SCALE_GS_IDXR::_11
    }
}
#[doc = "Possible values of the field `DCOC_ALPHAC_SCALE_GS_IDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_ALPHAC_SCALE_GS_IDXR {
    #[doc = "1/2"]
    _000,
    #[doc = "1/4"]
    _001,
    #[doc = "1/8"]
    _010,
    #[doc = "1/16"]
    _011,
    #[doc = "1/32"]
    _100,
    #[doc = "1/64"]
    _101,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DCOC_ALPHAC_SCALE_GS_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCOC_ALPHAC_SCALE_GS_IDXR::_000 => 0,
            DCOC_ALPHAC_SCALE_GS_IDXR::_001 => 1,
            DCOC_ALPHAC_SCALE_GS_IDXR::_010 => 2,
            DCOC_ALPHAC_SCALE_GS_IDXR::_011 => 3,
            DCOC_ALPHAC_SCALE_GS_IDXR::_100 => 4,
            DCOC_ALPHAC_SCALE_GS_IDXR::_101 => 5,
            DCOC_ALPHAC_SCALE_GS_IDXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCOC_ALPHAC_SCALE_GS_IDXR {
        match value {
            0 => DCOC_ALPHAC_SCALE_GS_IDXR::_000,
            1 => DCOC_ALPHAC_SCALE_GS_IDXR::_001,
            2 => DCOC_ALPHAC_SCALE_GS_IDXR::_010,
            3 => DCOC_ALPHAC_SCALE_GS_IDXR::_011,
            4 => DCOC_ALPHAC_SCALE_GS_IDXR::_100,
            5 => DCOC_ALPHAC_SCALE_GS_IDXR::_101,
            i => DCOC_ALPHAC_SCALE_GS_IDXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == DCOC_ALPHAC_SCALE_GS_IDXR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == DCOC_ALPHAC_SCALE_GS_IDXR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == DCOC_ALPHAC_SCALE_GS_IDXR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == DCOC_ALPHAC_SCALE_GS_IDXR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == DCOC_ALPHAC_SCALE_GS_IDXR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == DCOC_ALPHAC_SCALE_GS_IDXR::_101
    }
}
#[doc = "Possible values of the field `DCOC_ALPHA_RADIUS_GS_IDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_ALPHA_RADIUS_GS_IDXR {
    #[doc = "1"]
    _000,
    #[doc = "1/2"]
    _001,
    #[doc = "1/4"]
    _010,
    #[doc = "1/8"]
    _011,
    #[doc = "1/16"]
    _100,
    #[doc = "1/32"]
    _101,
    #[doc = "1/64"]
    _110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DCOC_ALPHA_RADIUS_GS_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCOC_ALPHA_RADIUS_GS_IDXR::_000 => 0,
            DCOC_ALPHA_RADIUS_GS_IDXR::_001 => 1,
            DCOC_ALPHA_RADIUS_GS_IDXR::_010 => 2,
            DCOC_ALPHA_RADIUS_GS_IDXR::_011 => 3,
            DCOC_ALPHA_RADIUS_GS_IDXR::_100 => 4,
            DCOC_ALPHA_RADIUS_GS_IDXR::_101 => 5,
            DCOC_ALPHA_RADIUS_GS_IDXR::_110 => 6,
            DCOC_ALPHA_RADIUS_GS_IDXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCOC_ALPHA_RADIUS_GS_IDXR {
        match value {
            0 => DCOC_ALPHA_RADIUS_GS_IDXR::_000,
            1 => DCOC_ALPHA_RADIUS_GS_IDXR::_001,
            2 => DCOC_ALPHA_RADIUS_GS_IDXR::_010,
            3 => DCOC_ALPHA_RADIUS_GS_IDXR::_011,
            4 => DCOC_ALPHA_RADIUS_GS_IDXR::_100,
            5 => DCOC_ALPHA_RADIUS_GS_IDXR::_101,
            6 => DCOC_ALPHA_RADIUS_GS_IDXR::_110,
            i => DCOC_ALPHA_RADIUS_GS_IDXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_GS_IDXR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_GS_IDXR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_GS_IDXR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_GS_IDXR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_GS_IDXR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_GS_IDXR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == DCOC_ALPHA_RADIUS_GS_IDXR::_110
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_TRK_MIN_AGC_IDXR {
    bits: u8,
}
impl DCOC_TRK_MIN_AGC_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `DCOC_SIGN_SCALE_IDX`"]
pub enum DCOC_SIGN_SCALE_IDXW {
    #[doc = "1/8"]
    _00,
    #[doc = "1/16"]
    _01,
    #[doc = "1/32"]
    _10,
    #[doc = "1/64"]
    _11,
}
impl DCOC_SIGN_SCALE_IDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCOC_SIGN_SCALE_IDXW::_00 => 0,
            DCOC_SIGN_SCALE_IDXW::_01 => 1,
            DCOC_SIGN_SCALE_IDXW::_10 => 2,
            DCOC_SIGN_SCALE_IDXW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_SIGN_SCALE_IDXW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_SIGN_SCALE_IDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_SIGN_SCALE_IDXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1/8"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DCOC_SIGN_SCALE_IDXW::_00)
    }
    #[doc = "1/16"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DCOC_SIGN_SCALE_IDXW::_01)
    }
    #[doc = "1/32"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DCOC_SIGN_SCALE_IDXW::_10)
    }
    #[doc = "1/64"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DCOC_SIGN_SCALE_IDXW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCOC_ALPHAC_SCALE_IDX`"]
pub enum DCOC_ALPHAC_SCALE_IDXW {
    #[doc = "1/2"]
    _000,
    #[doc = "1/4"]
    _001,
    #[doc = "1/8"]
    _010,
    #[doc = "1/16"]
    _011,
    #[doc = "1/32"]
    _100,
    #[doc = "1/64"]
    _101,
}
impl DCOC_ALPHAC_SCALE_IDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCOC_ALPHAC_SCALE_IDXW::_000 => 0,
            DCOC_ALPHAC_SCALE_IDXW::_001 => 1,
            DCOC_ALPHAC_SCALE_IDXW::_010 => 2,
            DCOC_ALPHAC_SCALE_IDXW::_011 => 3,
            DCOC_ALPHAC_SCALE_IDXW::_100 => 4,
            DCOC_ALPHAC_SCALE_IDXW::_101 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_ALPHAC_SCALE_IDXW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_ALPHAC_SCALE_IDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_ALPHAC_SCALE_IDXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1/2"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(DCOC_ALPHAC_SCALE_IDXW::_000)
    }
    #[doc = "1/4"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(DCOC_ALPHAC_SCALE_IDXW::_001)
    }
    #[doc = "1/8"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(DCOC_ALPHAC_SCALE_IDXW::_010)
    }
    #[doc = "1/16"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(DCOC_ALPHAC_SCALE_IDXW::_011)
    }
    #[doc = "1/32"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(DCOC_ALPHAC_SCALE_IDXW::_100)
    }
    #[doc = "1/64"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(DCOC_ALPHAC_SCALE_IDXW::_101)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCOC_ALPHA_RADIUS_IDX`"]
pub enum DCOC_ALPHA_RADIUS_IDXW {
    #[doc = "1"]
    _000,
    #[doc = "1/2"]
    _001,
    #[doc = "1/4"]
    _010,
    #[doc = "1/8"]
    _011,
    #[doc = "1/16"]
    _100,
    #[doc = "1/32"]
    _101,
    #[doc = "1/64"]
    _110,
}
impl DCOC_ALPHA_RADIUS_IDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCOC_ALPHA_RADIUS_IDXW::_000 => 0,
            DCOC_ALPHA_RADIUS_IDXW::_001 => 1,
            DCOC_ALPHA_RADIUS_IDXW::_010 => 2,
            DCOC_ALPHA_RADIUS_IDXW::_011 => 3,
            DCOC_ALPHA_RADIUS_IDXW::_100 => 4,
            DCOC_ALPHA_RADIUS_IDXW::_101 => 5,
            DCOC_ALPHA_RADIUS_IDXW::_110 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_ALPHA_RADIUS_IDXW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_ALPHA_RADIUS_IDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_ALPHA_RADIUS_IDXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_IDXW::_000)
    }
    #[doc = "1/2"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_IDXW::_001)
    }
    #[doc = "1/4"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_IDXW::_010)
    }
    #[doc = "1/8"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_IDXW::_011)
    }
    #[doc = "1/16"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_IDXW::_100)
    }
    #[doc = "1/32"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_IDXW::_101)
    }
    #[doc = "1/64"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_IDXW::_110)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_TRK_EST_GS_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_TRK_EST_GS_CNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCOC_SIGN_SCALE_GS_IDX`"]
pub enum DCOC_SIGN_SCALE_GS_IDXW {
    #[doc = "1/8"]
    _00,
    #[doc = "1/16"]
    _01,
    #[doc = "1/32"]
    _10,
    #[doc = "1/64"]
    _11,
}
impl DCOC_SIGN_SCALE_GS_IDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCOC_SIGN_SCALE_GS_IDXW::_00 => 0,
            DCOC_SIGN_SCALE_GS_IDXW::_01 => 1,
            DCOC_SIGN_SCALE_GS_IDXW::_10 => 2,
            DCOC_SIGN_SCALE_GS_IDXW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_SIGN_SCALE_GS_IDXW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_SIGN_SCALE_GS_IDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_SIGN_SCALE_GS_IDXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1/8"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DCOC_SIGN_SCALE_GS_IDXW::_00)
    }
    #[doc = "1/16"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DCOC_SIGN_SCALE_GS_IDXW::_01)
    }
    #[doc = "1/32"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DCOC_SIGN_SCALE_GS_IDXW::_10)
    }
    #[doc = "1/64"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DCOC_SIGN_SCALE_GS_IDXW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCOC_ALPHAC_SCALE_GS_IDX`"]
pub enum DCOC_ALPHAC_SCALE_GS_IDXW {
    #[doc = "1/2"]
    _000,
    #[doc = "1/4"]
    _001,
    #[doc = "1/8"]
    _010,
    #[doc = "1/16"]
    _011,
    #[doc = "1/32"]
    _100,
    #[doc = "1/64"]
    _101,
}
impl DCOC_ALPHAC_SCALE_GS_IDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCOC_ALPHAC_SCALE_GS_IDXW::_000 => 0,
            DCOC_ALPHAC_SCALE_GS_IDXW::_001 => 1,
            DCOC_ALPHAC_SCALE_GS_IDXW::_010 => 2,
            DCOC_ALPHAC_SCALE_GS_IDXW::_011 => 3,
            DCOC_ALPHAC_SCALE_GS_IDXW::_100 => 4,
            DCOC_ALPHAC_SCALE_GS_IDXW::_101 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_ALPHAC_SCALE_GS_IDXW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_ALPHAC_SCALE_GS_IDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_ALPHAC_SCALE_GS_IDXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1/2"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(DCOC_ALPHAC_SCALE_GS_IDXW::_000)
    }
    #[doc = "1/4"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(DCOC_ALPHAC_SCALE_GS_IDXW::_001)
    }
    #[doc = "1/8"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(DCOC_ALPHAC_SCALE_GS_IDXW::_010)
    }
    #[doc = "1/16"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(DCOC_ALPHAC_SCALE_GS_IDXW::_011)
    }
    #[doc = "1/32"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(DCOC_ALPHAC_SCALE_GS_IDXW::_100)
    }
    #[doc = "1/64"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(DCOC_ALPHAC_SCALE_GS_IDXW::_101)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCOC_ALPHA_RADIUS_GS_IDX`"]
pub enum DCOC_ALPHA_RADIUS_GS_IDXW {
    #[doc = "1"]
    _000,
    #[doc = "1/2"]
    _001,
    #[doc = "1/4"]
    _010,
    #[doc = "1/8"]
    _011,
    #[doc = "1/16"]
    _100,
    #[doc = "1/32"]
    _101,
    #[doc = "1/64"]
    _110,
}
impl DCOC_ALPHA_RADIUS_GS_IDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCOC_ALPHA_RADIUS_GS_IDXW::_000 => 0,
            DCOC_ALPHA_RADIUS_GS_IDXW::_001 => 1,
            DCOC_ALPHA_RADIUS_GS_IDXW::_010 => 2,
            DCOC_ALPHA_RADIUS_GS_IDXW::_011 => 3,
            DCOC_ALPHA_RADIUS_GS_IDXW::_100 => 4,
            DCOC_ALPHA_RADIUS_GS_IDXW::_101 => 5,
            DCOC_ALPHA_RADIUS_GS_IDXW::_110 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_ALPHA_RADIUS_GS_IDXW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_ALPHA_RADIUS_GS_IDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_ALPHA_RADIUS_GS_IDXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_GS_IDXW::_000)
    }
    #[doc = "1/2"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_GS_IDXW::_001)
    }
    #[doc = "1/4"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_GS_IDXW::_010)
    }
    #[doc = "1/8"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_GS_IDXW::_011)
    }
    #[doc = "1/16"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_GS_IDXW::_100)
    }
    #[doc = "1/32"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_GS_IDXW::_101)
    }
    #[doc = "1/64"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(DCOC_ALPHA_RADIUS_GS_IDXW::_110)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_TRK_MIN_AGC_IDXW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_TRK_MIN_AGC_IDXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 0:1 - DCOC Sign Scaling"]
    #[inline]
    pub fn dcoc_sign_scale_idx(&self) -> DCOC_SIGN_SCALE_IDXR {
        DCOC_SIGN_SCALE_IDXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:4 - DCOC Alpha-C Scaling"]
    #[inline]
    pub fn dcoc_alphac_scale_idx(&self) -> DCOC_ALPHAC_SCALE_IDXR {
        DCOC_ALPHAC_SCALE_IDXR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:7 - Alpha-R Scaling"]
    #[inline]
    pub fn dcoc_alpha_radius_idx(&self) -> DCOC_ALPHA_RADIUS_IDXR {
        DCOC_ALPHA_RADIUS_IDXR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - DCOC Tracking Estimator Gearshift Count"]
    #[inline]
    pub fn dcoc_trk_est_gs_cnt(&self) -> DCOC_TRK_EST_GS_CNTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_TRK_EST_GS_CNTR { bits }
    }
    #[doc = "Bits 16:17 - DCOC Sign Scaling for Gearshift"]
    #[inline]
    pub fn dcoc_sign_scale_gs_idx(&self) -> DCOC_SIGN_SCALE_GS_IDXR {
        DCOC_SIGN_SCALE_GS_IDXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:20 - DCOC Alpha-C Scaling for Gearshift"]
    #[inline]
    pub fn dcoc_alphac_scale_gs_idx(&self) -> DCOC_ALPHAC_SCALE_GS_IDXR {
        DCOC_ALPHAC_SCALE_GS_IDXR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:23 - Alpha-R Scaling for Gearshift"]
    #[inline]
    pub fn dcoc_alpha_radius_gs_idx(&self) -> DCOC_ALPHA_RADIUS_GS_IDXR {
        DCOC_ALPHA_RADIUS_GS_IDXR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:28 - DCOC Tracking Minimum AGC Table Index"]
    #[inline]
    pub fn dcoc_trk_min_agc_idx(&self) -> DCOC_TRK_MIN_AGC_IDXR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_TRK_MIN_AGC_IDXR { bits }
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
    #[doc = "Bits 0:1 - DCOC Sign Scaling"]
    #[inline]
    pub fn dcoc_sign_scale_idx(&mut self) -> _DCOC_SIGN_SCALE_IDXW {
        _DCOC_SIGN_SCALE_IDXW { w: self }
    }
    #[doc = "Bits 2:4 - DCOC Alpha-C Scaling"]
    #[inline]
    pub fn dcoc_alphac_scale_idx(&mut self) -> _DCOC_ALPHAC_SCALE_IDXW {
        _DCOC_ALPHAC_SCALE_IDXW { w: self }
    }
    #[doc = "Bits 5:7 - Alpha-R Scaling"]
    #[inline]
    pub fn dcoc_alpha_radius_idx(&mut self) -> _DCOC_ALPHA_RADIUS_IDXW {
        _DCOC_ALPHA_RADIUS_IDXW { w: self }
    }
    #[doc = "Bits 12:14 - DCOC Tracking Estimator Gearshift Count"]
    #[inline]
    pub fn dcoc_trk_est_gs_cnt(&mut self) -> _DCOC_TRK_EST_GS_CNTW {
        _DCOC_TRK_EST_GS_CNTW { w: self }
    }
    #[doc = "Bits 16:17 - DCOC Sign Scaling for Gearshift"]
    #[inline]
    pub fn dcoc_sign_scale_gs_idx(&mut self) -> _DCOC_SIGN_SCALE_GS_IDXW {
        _DCOC_SIGN_SCALE_GS_IDXW { w: self }
    }
    #[doc = "Bits 18:20 - DCOC Alpha-C Scaling for Gearshift"]
    #[inline]
    pub fn dcoc_alphac_scale_gs_idx(&mut self) -> _DCOC_ALPHAC_SCALE_GS_IDXW {
        _DCOC_ALPHAC_SCALE_GS_IDXW { w: self }
    }
    #[doc = "Bits 21:23 - Alpha-R Scaling for Gearshift"]
    #[inline]
    pub fn dcoc_alpha_radius_gs_idx(&mut self) -> _DCOC_ALPHA_RADIUS_GS_IDXW {
        _DCOC_ALPHA_RADIUS_GS_IDXW { w: self }
    }
    #[doc = "Bits 24:28 - DCOC Tracking Minimum AGC Table Index"]
    #[inline]
    pub fn dcoc_trk_min_agc_idx(&mut self) -> _DCOC_TRK_MIN_AGC_IDXW {
        _DCOC_TRK_MIN_AGC_IDXW { w: self }
    }
}
