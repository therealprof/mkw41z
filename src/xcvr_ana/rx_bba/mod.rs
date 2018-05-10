#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_BBA {
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
#[doc = "Possible values of the field `RX_BBA_BW_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BBA_BW_SELR {
    #[doc = "1000K"]
    _000,
    #[doc = "900K"]
    _001,
    #[doc = "800K"]
    _010,
    #[doc = "700K Default"]
    _011,
    #[doc = "600K"]
    _100,
    #[doc = "500K"]
    _101,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RX_BBA_BW_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_BBA_BW_SELR::_000 => 0,
            RX_BBA_BW_SELR::_001 => 1,
            RX_BBA_BW_SELR::_010 => 2,
            RX_BBA_BW_SELR::_011 => 3,
            RX_BBA_BW_SELR::_100 => 4,
            RX_BBA_BW_SELR::_101 => 5,
            RX_BBA_BW_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_BBA_BW_SELR {
        match value {
            0 => RX_BBA_BW_SELR::_000,
            1 => RX_BBA_BW_SELR::_001,
            2 => RX_BBA_BW_SELR::_010,
            3 => RX_BBA_BW_SELR::_011,
            4 => RX_BBA_BW_SELR::_100,
            5 => RX_BBA_BW_SELR::_101,
            i => RX_BBA_BW_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == RX_BBA_BW_SELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == RX_BBA_BW_SELR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == RX_BBA_BW_SELR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == RX_BBA_BW_SELR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == RX_BBA_BW_SELR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == RX_BBA_BW_SELR::_101
    }
}
#[doc = r" Value of the field"]
pub struct RX_BBA_CUR_BUMPR {
    bits: bool,
}
impl RX_BBA_CUR_BUMPR {
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
#[doc = "Possible values of the field `RX_BBA_DIAGSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BBA_DIAGSEL1R {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl RX_BBA_DIAGSEL1R {
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
            RX_BBA_DIAGSEL1R::_0 => false,
            RX_BBA_DIAGSEL1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_BBA_DIAGSEL1R {
        match value {
            false => RX_BBA_DIAGSEL1R::_0,
            true => RX_BBA_DIAGSEL1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_BBA_DIAGSEL1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_BBA_DIAGSEL1R::_1
    }
}
#[doc = "Possible values of the field `RX_BBA_DIAGSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BBA_DIAGSEL2R {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl RX_BBA_DIAGSEL2R {
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
            RX_BBA_DIAGSEL2R::_0 => false,
            RX_BBA_DIAGSEL2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_BBA_DIAGSEL2R {
        match value {
            false => RX_BBA_DIAGSEL2R::_0,
            true => RX_BBA_DIAGSEL2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_BBA_DIAGSEL2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_BBA_DIAGSEL2R::_1
    }
}
#[doc = "Possible values of the field `RX_BBA_DIAGSEL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BBA_DIAGSEL3R {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl RX_BBA_DIAGSEL3R {
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
            RX_BBA_DIAGSEL3R::_0 => false,
            RX_BBA_DIAGSEL3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_BBA_DIAGSEL3R {
        match value {
            false => RX_BBA_DIAGSEL3R::_0,
            true => RX_BBA_DIAGSEL3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_BBA_DIAGSEL3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_BBA_DIAGSEL3R::_1
    }
}
#[doc = "Possible values of the field `RX_BBA_DIAGSEL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BBA_DIAGSEL4R {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl RX_BBA_DIAGSEL4R {
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
            RX_BBA_DIAGSEL4R::_0 => false,
            RX_BBA_DIAGSEL4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_BBA_DIAGSEL4R {
        match value {
            false => RX_BBA_DIAGSEL4R::_0,
            true => RX_BBA_DIAGSEL4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_BBA_DIAGSEL4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_BBA_DIAGSEL4R::_1
    }
}
#[doc = "Possible values of the field `RX_BBA_SPARE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BBA_SPARER {
    #[doc = "600mV (Default)"]
    _00,
    #[doc = "675mV"]
    _01,
    #[doc = "450mV"]
    _10,
    #[doc = "525mV"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RX_BBA_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_BBA_SPARER::_00 => 0,
            RX_BBA_SPARER::_01 => 1,
            RX_BBA_SPARER::_10 => 2,
            RX_BBA_SPARER::_11 => 3,
            RX_BBA_SPARER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_BBA_SPARER {
        match value {
            0 => RX_BBA_SPARER::_00,
            1 => RX_BBA_SPARER::_01,
            2 => RX_BBA_SPARER::_10,
            3 => RX_BBA_SPARER::_11,
            i => RX_BBA_SPARER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RX_BBA_SPARER::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RX_BBA_SPARER::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RX_BBA_SPARER::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == RX_BBA_SPARER::_11
    }
}
#[doc = "Possible values of the field `RX_BBA2_BW_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BBA2_BW_SELR {
    #[doc = "1000K"]
    _000,
    #[doc = "900K"]
    _001,
    #[doc = "800K"]
    _010,
    #[doc = "700K Default"]
    _011,
    #[doc = "600K"]
    _100,
    #[doc = "500K"]
    _101,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RX_BBA2_BW_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_BBA2_BW_SELR::_000 => 0,
            RX_BBA2_BW_SELR::_001 => 1,
            RX_BBA2_BW_SELR::_010 => 2,
            RX_BBA2_BW_SELR::_011 => 3,
            RX_BBA2_BW_SELR::_100 => 4,
            RX_BBA2_BW_SELR::_101 => 5,
            RX_BBA2_BW_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_BBA2_BW_SELR {
        match value {
            0 => RX_BBA2_BW_SELR::_000,
            1 => RX_BBA2_BW_SELR::_001,
            2 => RX_BBA2_BW_SELR::_010,
            3 => RX_BBA2_BW_SELR::_011,
            4 => RX_BBA2_BW_SELR::_100,
            5 => RX_BBA2_BW_SELR::_101,
            i => RX_BBA2_BW_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == RX_BBA2_BW_SELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == RX_BBA2_BW_SELR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == RX_BBA2_BW_SELR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == RX_BBA2_BW_SELR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == RX_BBA2_BW_SELR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == RX_BBA2_BW_SELR::_101
    }
}
#[doc = r" Value of the field"]
pub struct RX_BBA2_SPARER {
    bits: u8,
}
impl RX_BBA2_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `RX_BBA_BW_SEL`"]
pub enum RX_BBA_BW_SELW {
    #[doc = "1000K"]
    _000,
    #[doc = "900K"]
    _001,
    #[doc = "800K"]
    _010,
    #[doc = "700K Default"]
    _011,
    #[doc = "600K"]
    _100,
    #[doc = "500K"]
    _101,
}
impl RX_BBA_BW_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_BBA_BW_SELW::_000 => 0,
            RX_BBA_BW_SELW::_001 => 1,
            RX_BBA_BW_SELW::_010 => 2,
            RX_BBA_BW_SELW::_011 => 3,
            RX_BBA_BW_SELW::_100 => 4,
            RX_BBA_BW_SELW::_101 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_BBA_BW_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_BW_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_BBA_BW_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1000K"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(RX_BBA_BW_SELW::_000)
    }
    #[doc = "900K"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(RX_BBA_BW_SELW::_001)
    }
    #[doc = "800K"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(RX_BBA_BW_SELW::_010)
    }
    #[doc = "700K Default"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(RX_BBA_BW_SELW::_011)
    }
    #[doc = "600K"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(RX_BBA_BW_SELW::_100)
    }
    #[doc = "500K"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(RX_BBA_BW_SELW::_101)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_BBA_CUR_BUMPW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_CUR_BUMPW<'a> {
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
#[doc = "Values that can be written to the field `RX_BBA_DIAGSEL1`"]
pub enum RX_BBA_DIAGSEL1W {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl RX_BBA_DIAGSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_BBA_DIAGSEL1W::_0 => false,
            RX_BBA_DIAGSEL1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_BBA_DIAGSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_DIAGSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_BBA_DIAGSEL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diag disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_BBA_DIAGSEL1W::_0)
    }
    #[doc = "Diag enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_BBA_DIAGSEL1W::_1)
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
#[doc = "Values that can be written to the field `RX_BBA_DIAGSEL2`"]
pub enum RX_BBA_DIAGSEL2W {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl RX_BBA_DIAGSEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_BBA_DIAGSEL2W::_0 => false,
            RX_BBA_DIAGSEL2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_BBA_DIAGSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_DIAGSEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_BBA_DIAGSEL2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diag disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_BBA_DIAGSEL2W::_0)
    }
    #[doc = "Diag enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_BBA_DIAGSEL2W::_1)
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
#[doc = "Values that can be written to the field `RX_BBA_DIAGSEL3`"]
pub enum RX_BBA_DIAGSEL3W {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl RX_BBA_DIAGSEL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_BBA_DIAGSEL3W::_0 => false,
            RX_BBA_DIAGSEL3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_BBA_DIAGSEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_DIAGSEL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_BBA_DIAGSEL3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diag disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_BBA_DIAGSEL3W::_0)
    }
    #[doc = "Diag enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_BBA_DIAGSEL3W::_1)
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
#[doc = "Values that can be written to the field `RX_BBA_DIAGSEL4`"]
pub enum RX_BBA_DIAGSEL4W {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl RX_BBA_DIAGSEL4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_BBA_DIAGSEL4W::_0 => false,
            RX_BBA_DIAGSEL4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_BBA_DIAGSEL4W<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_DIAGSEL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_BBA_DIAGSEL4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diag disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_BBA_DIAGSEL4W::_0)
    }
    #[doc = "Diag enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_BBA_DIAGSEL4W::_1)
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
#[doc = "Values that can be written to the field `RX_BBA_SPARE`"]
pub enum RX_BBA_SPAREW {
    #[doc = "600mV (Default)"]
    _00,
    #[doc = "675mV"]
    _01,
    #[doc = "450mV"]
    _10,
    #[doc = "525mV"]
    _11,
}
impl RX_BBA_SPAREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_BBA_SPAREW::_00 => 0,
            RX_BBA_SPAREW::_01 => 1,
            RX_BBA_SPAREW::_10 => 2,
            RX_BBA_SPAREW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_BBA_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_SPAREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_BBA_SPAREW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "600mV (Default)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RX_BBA_SPAREW::_00)
    }
    #[doc = "675mV"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RX_BBA_SPAREW::_01)
    }
    #[doc = "450mV"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RX_BBA_SPAREW::_10)
    }
    #[doc = "525mV"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RX_BBA_SPAREW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_BBA2_BW_SEL`"]
pub enum RX_BBA2_BW_SELW {
    #[doc = "1000K"]
    _000,
    #[doc = "900K"]
    _001,
    #[doc = "800K"]
    _010,
    #[doc = "700K Default"]
    _011,
    #[doc = "600K"]
    _100,
    #[doc = "500K"]
    _101,
}
impl RX_BBA2_BW_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_BBA2_BW_SELW::_000 => 0,
            RX_BBA2_BW_SELW::_001 => 1,
            RX_BBA2_BW_SELW::_010 => 2,
            RX_BBA2_BW_SELW::_011 => 3,
            RX_BBA2_BW_SELW::_100 => 4,
            RX_BBA2_BW_SELW::_101 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_BBA2_BW_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA2_BW_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_BBA2_BW_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1000K"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(RX_BBA2_BW_SELW::_000)
    }
    #[doc = "900K"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(RX_BBA2_BW_SELW::_001)
    }
    #[doc = "800K"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(RX_BBA2_BW_SELW::_010)
    }
    #[doc = "700K Default"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(RX_BBA2_BW_SELW::_011)
    }
    #[doc = "600K"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(RX_BBA2_BW_SELW::_100)
    }
    #[doc = "500K"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(RX_BBA2_BW_SELW::_101)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_BBA2_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA2_SPAREW<'a> {
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
    #[doc = "Bits 0:2 - rmap_rx_bba_bw_sel[2:0]"]
    #[inline]
    pub fn rx_bba_bw_sel(&self) -> RX_BBA_BW_SELR {
        RX_BBA_BW_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - rmap_rx_bba_cur_bump"]
    #[inline]
    pub fn rx_bba_cur_bump(&self) -> RX_BBA_CUR_BUMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_BBA_CUR_BUMPR { bits }
    }
    #[doc = "Bit 4 - rmap_rx_bba_diagsel1"]
    #[inline]
    pub fn rx_bba_diagsel1(&self) -> RX_BBA_DIAGSEL1R {
        RX_BBA_DIAGSEL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - rmap_rx_bba_diagsel2"]
    #[inline]
    pub fn rx_bba_diagsel2(&self) -> RX_BBA_DIAGSEL2R {
        RX_BBA_DIAGSEL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - rmap_rx_bba_diagsel3"]
    #[inline]
    pub fn rx_bba_diagsel3(&self) -> RX_BBA_DIAGSEL3R {
        RX_BBA_DIAGSEL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - rmap_rx_bba_diagsel4"]
    #[inline]
    pub fn rx_bba_diagsel4(&self) -> RX_BBA_DIAGSEL4R {
        RX_BBA_DIAGSEL4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:21 - rmap_rx_bba_spare[5:0]"]
    #[inline]
    pub fn rx_bba_spare(&self) -> RX_BBA_SPARER {
        RX_BBA_SPARER::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - rmap_bba2_bw_sel[2:0]"]
    #[inline]
    pub fn rx_bba2_bw_sel(&self) -> RX_BBA2_BW_SELR {
        RX_BBA2_BW_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:30 - rmap_rx_bba2_spare[2:0]"]
    #[inline]
    pub fn rx_bba2_spare(&self) -> RX_BBA2_SPARER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_BBA2_SPARER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 50331651 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - rmap_rx_bba_bw_sel[2:0]"]
    #[inline]
    pub fn rx_bba_bw_sel(&mut self) -> _RX_BBA_BW_SELW {
        _RX_BBA_BW_SELW { w: self }
    }
    #[doc = "Bit 3 - rmap_rx_bba_cur_bump"]
    #[inline]
    pub fn rx_bba_cur_bump(&mut self) -> _RX_BBA_CUR_BUMPW {
        _RX_BBA_CUR_BUMPW { w: self }
    }
    #[doc = "Bit 4 - rmap_rx_bba_diagsel1"]
    #[inline]
    pub fn rx_bba_diagsel1(&mut self) -> _RX_BBA_DIAGSEL1W {
        _RX_BBA_DIAGSEL1W { w: self }
    }
    #[doc = "Bit 5 - rmap_rx_bba_diagsel2"]
    #[inline]
    pub fn rx_bba_diagsel2(&mut self) -> _RX_BBA_DIAGSEL2W {
        _RX_BBA_DIAGSEL2W { w: self }
    }
    #[doc = "Bit 6 - rmap_rx_bba_diagsel3"]
    #[inline]
    pub fn rx_bba_diagsel3(&mut self) -> _RX_BBA_DIAGSEL3W {
        _RX_BBA_DIAGSEL3W { w: self }
    }
    #[doc = "Bit 7 - rmap_rx_bba_diagsel4"]
    #[inline]
    pub fn rx_bba_diagsel4(&mut self) -> _RX_BBA_DIAGSEL4W {
        _RX_BBA_DIAGSEL4W { w: self }
    }
    #[doc = "Bits 16:21 - rmap_rx_bba_spare[5:0]"]
    #[inline]
    pub fn rx_bba_spare(&mut self) -> _RX_BBA_SPAREW {
        _RX_BBA_SPAREW { w: self }
    }
    #[doc = "Bits 24:26 - rmap_bba2_bw_sel[2:0]"]
    #[inline]
    pub fn rx_bba2_bw_sel(&mut self) -> _RX_BBA2_BW_SELW {
        _RX_BBA2_BW_SELW { w: self }
    }
    #[doc = "Bits 28:30 - rmap_rx_bba2_spare[2:0]"]
    #[inline]
    pub fn rx_bba2_spare(&mut self) -> _RX_BBA2_SPAREW {
        _RX_BBA2_SPAREW { w: self }
    }
}
