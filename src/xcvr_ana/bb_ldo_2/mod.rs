#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BB_LDO_2 {
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
#[doc = "Possible values of the field `BB_LDO_PD_BYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_PD_BYPR {
    #[doc = "Bypass disabled."]
    _0,
    #[doc = "Bypass enabled"]
    _1,
}
impl BB_LDO_PD_BYPR {
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
            BB_LDO_PD_BYPR::_0 => false,
            BB_LDO_PD_BYPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_PD_BYPR {
        match value {
            false => BB_LDO_PD_BYPR::_0,
            true => BB_LDO_PD_BYPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_PD_BYPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_PD_BYPR::_1
    }
}
#[doc = "Possible values of the field `BB_LDO_PD_DIAGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_PD_DIAGSELR {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_PD_DIAGSELR {
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
            BB_LDO_PD_DIAGSELR::_0 => false,
            BB_LDO_PD_DIAGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_PD_DIAGSELR {
        match value {
            false => BB_LDO_PD_DIAGSELR::_0,
            true => BB_LDO_PD_DIAGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_PD_DIAGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_PD_DIAGSELR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_PD_SPARER {
    bits: u8,
}
impl BB_LDO_PD_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BB_LDO_PD_TRIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_PD_TRIMR {
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
impl BB_LDO_PD_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BB_LDO_PD_TRIMR::_0 => 0,
            BB_LDO_PD_TRIMR::_1 => 1,
            BB_LDO_PD_TRIMR::_2 => 2,
            BB_LDO_PD_TRIMR::_3 => 3,
            BB_LDO_PD_TRIMR::_4 => 4,
            BB_LDO_PD_TRIMR::_5 => 5,
            BB_LDO_PD_TRIMR::_6 => 6,
            BB_LDO_PD_TRIMR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BB_LDO_PD_TRIMR {
        match value {
            0 => BB_LDO_PD_TRIMR::_0,
            1 => BB_LDO_PD_TRIMR::_1,
            2 => BB_LDO_PD_TRIMR::_2,
            3 => BB_LDO_PD_TRIMR::_3,
            4 => BB_LDO_PD_TRIMR::_4,
            5 => BB_LDO_PD_TRIMR::_5,
            6 => BB_LDO_PD_TRIMR::_6,
            7 => BB_LDO_PD_TRIMR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_PD_TRIMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_PD_TRIMR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == BB_LDO_PD_TRIMR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == BB_LDO_PD_TRIMR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == BB_LDO_PD_TRIMR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == BB_LDO_PD_TRIMR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == BB_LDO_PD_TRIMR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == BB_LDO_PD_TRIMR::_7
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_VCO_SPARER {
    bits: u8,
}
impl BB_LDO_VCO_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BB_LDO_VCOLO_BYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_VCOLO_BYPR {
    #[doc = "Bypass disabled."]
    _0,
    #[doc = "Bypass enabled"]
    _1,
}
impl BB_LDO_VCOLO_BYPR {
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
            BB_LDO_VCOLO_BYPR::_0 => false,
            BB_LDO_VCOLO_BYPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_VCOLO_BYPR {
        match value {
            false => BB_LDO_VCOLO_BYPR::_0,
            true => BB_LDO_VCOLO_BYPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_VCOLO_BYPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_VCOLO_BYPR::_1
    }
}
#[doc = "Possible values of the field `BB_LDO_VCOLO_DIAGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_VCOLO_DIAGSELR {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_VCOLO_DIAGSELR {
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
            BB_LDO_VCOLO_DIAGSELR::_0 => false,
            BB_LDO_VCOLO_DIAGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_VCOLO_DIAGSELR {
        match value {
            false => BB_LDO_VCOLO_DIAGSELR::_0,
            true => BB_LDO_VCOLO_DIAGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_VCOLO_DIAGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_VCOLO_DIAGSELR::_1
    }
}
#[doc = "Possible values of the field `BB_LDO_VCOLO_TRIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_VCOLO_TRIMR {
    #[doc = "1.138/1.117 V ( Default )"]
    _0,
    #[doc = "1.076/1.058 V"]
    _1,
    #[doc = "1.027/1.012 V"]
    _2,
    #[doc = "0.98/0.97 V"]
    _3,
    #[doc = "1.22/1.19 V"]
    _4,
    #[doc = "1.33/1.3 V"]
    _5,
    #[doc = "1.5/1.4 V"]
    _6,
    #[doc = "1.82/1.4 V"]
    _7,
}
impl BB_LDO_VCOLO_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BB_LDO_VCOLO_TRIMR::_0 => 0,
            BB_LDO_VCOLO_TRIMR::_1 => 1,
            BB_LDO_VCOLO_TRIMR::_2 => 2,
            BB_LDO_VCOLO_TRIMR::_3 => 3,
            BB_LDO_VCOLO_TRIMR::_4 => 4,
            BB_LDO_VCOLO_TRIMR::_5 => 5,
            BB_LDO_VCOLO_TRIMR::_6 => 6,
            BB_LDO_VCOLO_TRIMR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BB_LDO_VCOLO_TRIMR {
        match value {
            0 => BB_LDO_VCOLO_TRIMR::_0,
            1 => BB_LDO_VCOLO_TRIMR::_1,
            2 => BB_LDO_VCOLO_TRIMR::_2,
            3 => BB_LDO_VCOLO_TRIMR::_3,
            4 => BB_LDO_VCOLO_TRIMR::_4,
            5 => BB_LDO_VCOLO_TRIMR::_5,
            6 => BB_LDO_VCOLO_TRIMR::_6,
            7 => BB_LDO_VCOLO_TRIMR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_VCOLO_TRIMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_VCOLO_TRIMR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == BB_LDO_VCOLO_TRIMR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == BB_LDO_VCOLO_TRIMR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == BB_LDO_VCOLO_TRIMR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == BB_LDO_VCOLO_TRIMR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == BB_LDO_VCOLO_TRIMR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == BB_LDO_VCOLO_TRIMR::_7
    }
}
#[doc = "Possible values of the field `BB_LDO_VTREF_DIAGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_VTREF_DIAGSELR {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_VTREF_DIAGSELR {
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
            BB_LDO_VTREF_DIAGSELR::_0 => false,
            BB_LDO_VTREF_DIAGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_VTREF_DIAGSELR {
        match value {
            false => BB_LDO_VTREF_DIAGSELR::_0,
            true => BB_LDO_VTREF_DIAGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_VTREF_DIAGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_VTREF_DIAGSELR::_1
    }
}
#[doc = "Possible values of the field `BB_LDO_VTREF_TC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_VTREF_TCR {
    #[doc = "1.117/1.176 V"]
    _0,
    #[doc = "1.134/1.188 V"]
    _1,
    #[doc = "1.10/1.162 V"]
    _2,
    #[doc = "1.09/1.152 V"]
    _3,
}
impl BB_LDO_VTREF_TCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BB_LDO_VTREF_TCR::_0 => 0,
            BB_LDO_VTREF_TCR::_1 => 1,
            BB_LDO_VTREF_TCR::_2 => 2,
            BB_LDO_VTREF_TCR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BB_LDO_VTREF_TCR {
        match value {
            0 => BB_LDO_VTREF_TCR::_0,
            1 => BB_LDO_VTREF_TCR::_1,
            2 => BB_LDO_VTREF_TCR::_2,
            3 => BB_LDO_VTREF_TCR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_VTREF_TCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_VTREF_TCR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == BB_LDO_VTREF_TCR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == BB_LDO_VTREF_TCR::_3
    }
}
#[doc = "Values that can be written to the field `BB_LDO_PD_BYP`"]
pub enum BB_LDO_PD_BYPW {
    #[doc = "Bypass disabled."]
    _0,
    #[doc = "Bypass enabled"]
    _1,
}
impl BB_LDO_PD_BYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_PD_BYPW::_0 => false,
            BB_LDO_PD_BYPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_PD_BYPW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_PD_BYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_PD_BYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bypass disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_PD_BYPW::_0)
    }
    #[doc = "Bypass enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_PD_BYPW::_1)
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
#[doc = "Values that can be written to the field `BB_LDO_PD_DIAGSEL`"]
pub enum BB_LDO_PD_DIAGSELW {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_PD_DIAGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_PD_DIAGSELW::_0 => false,
            BB_LDO_PD_DIAGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_PD_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_PD_DIAGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_PD_DIAGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diag disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_PD_DIAGSELW::_0)
    }
    #[doc = "Diag enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_PD_DIAGSELW::_1)
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
pub struct _BB_LDO_PD_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_PD_SPAREW<'a> {
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
#[doc = "Values that can be written to the field `BB_LDO_PD_TRIM`"]
pub enum BB_LDO_PD_TRIMW {
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
impl BB_LDO_PD_TRIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BB_LDO_PD_TRIMW::_0 => 0,
            BB_LDO_PD_TRIMW::_1 => 1,
            BB_LDO_PD_TRIMW::_2 => 2,
            BB_LDO_PD_TRIMW::_3 => 3,
            BB_LDO_PD_TRIMW::_4 => 4,
            BB_LDO_PD_TRIMW::_5 => 5,
            BB_LDO_PD_TRIMW::_6 => 6,
            BB_LDO_PD_TRIMW::_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_PD_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_PD_TRIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_PD_TRIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.20 V ( Default )"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_PD_TRIMW::_0)
    }
    #[doc = "1.25 V"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_PD_TRIMW::_1)
    }
    #[doc = "1.28 V"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(BB_LDO_PD_TRIMW::_2)
    }
    #[doc = "1.33 V"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(BB_LDO_PD_TRIMW::_3)
    }
    #[doc = "1.40 V"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(BB_LDO_PD_TRIMW::_4)
    }
    #[doc = "1.44 V"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(BB_LDO_PD_TRIMW::_5)
    }
    #[doc = "1.50 V"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(BB_LDO_PD_TRIMW::_6)
    }
    #[doc = "1.66 V"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(BB_LDO_PD_TRIMW::_7)
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
#[doc = r" Proxy"]
pub struct _BB_LDO_VCO_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VCO_SPAREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB_LDO_VCOLO_BYP`"]
pub enum BB_LDO_VCOLO_BYPW {
    #[doc = "Bypass disabled."]
    _0,
    #[doc = "Bypass enabled"]
    _1,
}
impl BB_LDO_VCOLO_BYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_VCOLO_BYPW::_0 => false,
            BB_LDO_VCOLO_BYPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_VCOLO_BYPW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VCOLO_BYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_VCOLO_BYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bypass disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_BYPW::_0)
    }
    #[doc = "Bypass enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_BYPW::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB_LDO_VCOLO_DIAGSEL`"]
pub enum BB_LDO_VCOLO_DIAGSELW {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_VCOLO_DIAGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_VCOLO_DIAGSELW::_0 => false,
            BB_LDO_VCOLO_DIAGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_VCOLO_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VCOLO_DIAGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_VCOLO_DIAGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diag disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_DIAGSELW::_0)
    }
    #[doc = "Diag enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_DIAGSELW::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB_LDO_VCOLO_TRIM`"]
pub enum BB_LDO_VCOLO_TRIMW {
    #[doc = "1.138/1.117 V ( Default )"]
    _0,
    #[doc = "1.076/1.058 V"]
    _1,
    #[doc = "1.027/1.012 V"]
    _2,
    #[doc = "0.98/0.97 V"]
    _3,
    #[doc = "1.22/1.19 V"]
    _4,
    #[doc = "1.33/1.3 V"]
    _5,
    #[doc = "1.5/1.4 V"]
    _6,
    #[doc = "1.82/1.4 V"]
    _7,
}
impl BB_LDO_VCOLO_TRIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BB_LDO_VCOLO_TRIMW::_0 => 0,
            BB_LDO_VCOLO_TRIMW::_1 => 1,
            BB_LDO_VCOLO_TRIMW::_2 => 2,
            BB_LDO_VCOLO_TRIMW::_3 => 3,
            BB_LDO_VCOLO_TRIMW::_4 => 4,
            BB_LDO_VCOLO_TRIMW::_5 => 5,
            BB_LDO_VCOLO_TRIMW::_6 => 6,
            BB_LDO_VCOLO_TRIMW::_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_VCOLO_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VCOLO_TRIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_VCOLO_TRIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.138/1.117 V ( Default )"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_TRIMW::_0)
    }
    #[doc = "1.076/1.058 V"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_TRIMW::_1)
    }
    #[doc = "1.027/1.012 V"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_TRIMW::_2)
    }
    #[doc = "0.98/0.97 V"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_TRIMW::_3)
    }
    #[doc = "1.22/1.19 V"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_TRIMW::_4)
    }
    #[doc = "1.33/1.3 V"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_TRIMW::_5)
    }
    #[doc = "1.5/1.4 V"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_TRIMW::_6)
    }
    #[doc = "1.82/1.4 V"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_TRIMW::_7)
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
#[doc = "Values that can be written to the field `BB_LDO_VTREF_DIAGSEL`"]
pub enum BB_LDO_VTREF_DIAGSELW {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl BB_LDO_VTREF_DIAGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_VTREF_DIAGSELW::_0 => false,
            BB_LDO_VTREF_DIAGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_VTREF_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VTREF_DIAGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_VTREF_DIAGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diag disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_VTREF_DIAGSELW::_0)
    }
    #[doc = "Diag enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_VTREF_DIAGSELW::_1)
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
#[doc = "Values that can be written to the field `BB_LDO_VTREF_TC`"]
pub enum BB_LDO_VTREF_TCW {
    #[doc = "1.117/1.176 V"]
    _0,
    #[doc = "1.134/1.188 V"]
    _1,
    #[doc = "1.10/1.162 V"]
    _2,
    #[doc = "1.09/1.152 V"]
    _3,
}
impl BB_LDO_VTREF_TCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BB_LDO_VTREF_TCW::_0 => 0,
            BB_LDO_VTREF_TCW::_1 => 1,
            BB_LDO_VTREF_TCW::_2 => 2,
            BB_LDO_VTREF_TCW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_VTREF_TCW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VTREF_TCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_VTREF_TCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.117/1.176 V"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_VTREF_TCW::_0)
    }
    #[doc = "1.134/1.188 V"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_VTREF_TCW::_1)
    }
    #[doc = "1.10/1.162 V"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(BB_LDO_VTREF_TCW::_2)
    }
    #[doc = "1.09/1.152 V"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(BB_LDO_VTREF_TCW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 17;
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
    #[doc = "Bit 0 - rmap_bb_ldo_pd_byp"]
    #[inline]
    pub fn bb_ldo_pd_byp(&self) -> BB_LDO_PD_BYPR {
        BB_LDO_PD_BYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - rmap_bb_ldo_pd_diagsel"]
    #[inline]
    pub fn bb_ldo_pd_diagsel(&self) -> BB_LDO_PD_DIAGSELR {
        BB_LDO_PD_DIAGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - rmap_bb_ldo_pd_spare[1:0]"]
    #[inline]
    pub fn bb_ldo_pd_spare(&self) -> BB_LDO_PD_SPARER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_LDO_PD_SPARER { bits }
    }
    #[doc = "Bits 4:6 - rmap_bb_ldo_pd_trim[2:0]"]
    #[inline]
    pub fn bb_ldo_pd_trim(&self) -> BB_LDO_PD_TRIMR {
        BB_LDO_PD_TRIMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - rmap_bb_ldo_vco_spare[1:0]"]
    #[inline]
    pub fn bb_ldo_vco_spare(&self) -> BB_LDO_VCO_SPARER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_LDO_VCO_SPARER { bits }
    }
    #[doc = "Bit 10 - rmap_bb_ldo_vcolo_byp"]
    #[inline]
    pub fn bb_ldo_vcolo_byp(&self) -> BB_LDO_VCOLO_BYPR {
        BB_LDO_VCOLO_BYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - rmap_bb_ldo_vcolo_diagsel"]
    #[inline]
    pub fn bb_ldo_vcolo_diagsel(&self) -> BB_LDO_VCOLO_DIAGSELR {
        BB_LDO_VCOLO_DIAGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:14 - rmap_bb_ldo_vcolo_trim[2:0]"]
    #[inline]
    pub fn bb_ldo_vcolo_trim(&self) -> BB_LDO_VCOLO_TRIMR {
        BB_LDO_VCOLO_TRIMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - rmap_bb_ldo_vtref_diagsel"]
    #[inline]
    pub fn bb_ldo_vtref_diagsel(&self) -> BB_LDO_VTREF_DIAGSELR {
        BB_LDO_VTREF_DIAGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:18 - rmap_bb_ldo_vtref_tc[1:0]"]
    #[inline]
    pub fn bb_ldo_vtref_tc(&self) -> BB_LDO_VTREF_TCR {
        BB_LDO_VTREF_TCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
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
    #[doc = "Bit 0 - rmap_bb_ldo_pd_byp"]
    #[inline]
    pub fn bb_ldo_pd_byp(&mut self) -> _BB_LDO_PD_BYPW {
        _BB_LDO_PD_BYPW { w: self }
    }
    #[doc = "Bit 1 - rmap_bb_ldo_pd_diagsel"]
    #[inline]
    pub fn bb_ldo_pd_diagsel(&mut self) -> _BB_LDO_PD_DIAGSELW {
        _BB_LDO_PD_DIAGSELW { w: self }
    }
    #[doc = "Bits 2:3 - rmap_bb_ldo_pd_spare[1:0]"]
    #[inline]
    pub fn bb_ldo_pd_spare(&mut self) -> _BB_LDO_PD_SPAREW {
        _BB_LDO_PD_SPAREW { w: self }
    }
    #[doc = "Bits 4:6 - rmap_bb_ldo_pd_trim[2:0]"]
    #[inline]
    pub fn bb_ldo_pd_trim(&mut self) -> _BB_LDO_PD_TRIMW {
        _BB_LDO_PD_TRIMW { w: self }
    }
    #[doc = "Bits 8:9 - rmap_bb_ldo_vco_spare[1:0]"]
    #[inline]
    pub fn bb_ldo_vco_spare(&mut self) -> _BB_LDO_VCO_SPAREW {
        _BB_LDO_VCO_SPAREW { w: self }
    }
    #[doc = "Bit 10 - rmap_bb_ldo_vcolo_byp"]
    #[inline]
    pub fn bb_ldo_vcolo_byp(&mut self) -> _BB_LDO_VCOLO_BYPW {
        _BB_LDO_VCOLO_BYPW { w: self }
    }
    #[doc = "Bit 11 - rmap_bb_ldo_vcolo_diagsel"]
    #[inline]
    pub fn bb_ldo_vcolo_diagsel(&mut self) -> _BB_LDO_VCOLO_DIAGSELW {
        _BB_LDO_VCOLO_DIAGSELW { w: self }
    }
    #[doc = "Bits 12:14 - rmap_bb_ldo_vcolo_trim[2:0]"]
    #[inline]
    pub fn bb_ldo_vcolo_trim(&mut self) -> _BB_LDO_VCOLO_TRIMW {
        _BB_LDO_VCOLO_TRIMW { w: self }
    }
    #[doc = "Bit 16 - rmap_bb_ldo_vtref_diagsel"]
    #[inline]
    pub fn bb_ldo_vtref_diagsel(&mut self) -> _BB_LDO_VTREF_DIAGSELW {
        _BB_LDO_VTREF_DIAGSELW { w: self }
    }
    #[doc = "Bits 17:18 - rmap_bb_ldo_vtref_tc[1:0]"]
    #[inline]
    pub fn bb_ldo_vtref_tc(&mut self) -> _BB_LDO_VTREF_TCW {
        _BB_LDO_VTREF_TCW { w: self }
    }
}
