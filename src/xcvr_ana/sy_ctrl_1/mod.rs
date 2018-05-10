#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SY_CTRL_1 {
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
pub struct SY_DIVN_SPARER {
    bits: bool,
}
impl SY_DIVN_SPARER {
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
pub struct SY_FCAL_SPARER {
    bits: bool,
}
impl SY_FCAL_SPARER {
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
#[doc = "Possible values of the field `SY_LO_BUMP_RTLO_FDBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_LO_BUMP_RTLO_FDBKR {
    #[doc = "1.045 V"]
    _0,
    #[doc = "1.084 V"]
    _1,
    #[doc = "1.097 V"]
    _2,
    #[doc = "1.10 V"]
    _3,
}
impl SY_LO_BUMP_RTLO_FDBKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SY_LO_BUMP_RTLO_FDBKR::_0 => 0,
            SY_LO_BUMP_RTLO_FDBKR::_1 => 1,
            SY_LO_BUMP_RTLO_FDBKR::_2 => 2,
            SY_LO_BUMP_RTLO_FDBKR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SY_LO_BUMP_RTLO_FDBKR {
        match value {
            0 => SY_LO_BUMP_RTLO_FDBKR::_0,
            1 => SY_LO_BUMP_RTLO_FDBKR::_1,
            2 => SY_LO_BUMP_RTLO_FDBKR::_2,
            3 => SY_LO_BUMP_RTLO_FDBKR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_LO_BUMP_RTLO_FDBKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_LO_BUMP_RTLO_FDBKR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == SY_LO_BUMP_RTLO_FDBKR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == SY_LO_BUMP_RTLO_FDBKR::_3
    }
}
#[doc = "Possible values of the field `SY_LO_BUMP_RTLO_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_LO_BUMP_RTLO_RXR {
    #[doc = "1.051/1.037 V"]
    _0,
    #[doc = "1.082/1.075 V"]
    _1,
    #[doc = "1.092/1.088 V"]
    _2,
    #[doc = "1.098/1.094 V"]
    _3,
}
impl SY_LO_BUMP_RTLO_RXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SY_LO_BUMP_RTLO_RXR::_0 => 0,
            SY_LO_BUMP_RTLO_RXR::_1 => 1,
            SY_LO_BUMP_RTLO_RXR::_2 => 2,
            SY_LO_BUMP_RTLO_RXR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SY_LO_BUMP_RTLO_RXR {
        match value {
            0 => SY_LO_BUMP_RTLO_RXR::_0,
            1 => SY_LO_BUMP_RTLO_RXR::_1,
            2 => SY_LO_BUMP_RTLO_RXR::_2,
            3 => SY_LO_BUMP_RTLO_RXR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_LO_BUMP_RTLO_RXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_LO_BUMP_RTLO_RXR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == SY_LO_BUMP_RTLO_RXR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == SY_LO_BUMP_RTLO_RXR::_3
    }
}
#[doc = "Possible values of the field `SY_LO_BUMP_RTLO_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_LO_BUMP_RTLO_TXR {
    #[doc = "1.071/1.065 V"]
    _0,
    #[doc = "1.092/1.090 V"]
    _1,
    #[doc = "1.099/1.098 V"]
    _2,
    #[doc = "1.10/1.1 V"]
    _3,
}
impl SY_LO_BUMP_RTLO_TXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SY_LO_BUMP_RTLO_TXR::_0 => 0,
            SY_LO_BUMP_RTLO_TXR::_1 => 1,
            SY_LO_BUMP_RTLO_TXR::_2 => 2,
            SY_LO_BUMP_RTLO_TXR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SY_LO_BUMP_RTLO_TXR {
        match value {
            0 => SY_LO_BUMP_RTLO_TXR::_0,
            1 => SY_LO_BUMP_RTLO_TXR::_1,
            2 => SY_LO_BUMP_RTLO_TXR::_2,
            3 => SY_LO_BUMP_RTLO_TXR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_LO_BUMP_RTLO_TXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_LO_BUMP_RTLO_TXR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == SY_LO_BUMP_RTLO_TXR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == SY_LO_BUMP_RTLO_TXR::_3
    }
}
#[doc = "Possible values of the field `SY_LO_DIAGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_LO_DIAGSELR {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl SY_LO_DIAGSELR {
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
            SY_LO_DIAGSELR::_0 => false,
            SY_LO_DIAGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_LO_DIAGSELR {
        match value {
            false => SY_LO_DIAGSELR::_0,
            true => SY_LO_DIAGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_LO_DIAGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_LO_DIAGSELR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_LO_SPARER {
    bits: u8,
}
impl SY_LO_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SY_LPF_FILT_CTRLR {
    bits: u8,
}
impl SY_LPF_FILT_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SY_LPF_SPARER {
    bits: bool,
}
impl SY_LPF_SPARER {
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
pub struct SY_PD_DIAGSELR {
    bits: bool,
}
impl SY_PD_DIAGSELR {
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
pub struct SY_PD_PCH_TUNER {
    bits: u8,
}
impl SY_PD_PCH_TUNER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SY_PD_PCH_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_PD_PCH_SELR {
    #[doc = "inverter based precharge"]
    _0,
    #[doc = "resistor divider based precharge"]
    _1,
}
impl SY_PD_PCH_SELR {
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
            SY_PD_PCH_SELR::_0 => false,
            SY_PD_PCH_SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_PD_PCH_SELR {
        match value {
            false => SY_PD_PCH_SELR::_0,
            true => SY_PD_PCH_SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_PD_PCH_SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_PD_PCH_SELR::_1
    }
}
#[doc = "Possible values of the field `SY_PD_SPARE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_PD_SPARER {
    #[doc = "Default ;"]
    _0,
    #[doc = "PD output is pulled down."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SY_PD_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SY_PD_SPARER::_0 => 0,
            SY_PD_SPARER::_1 => 1,
            SY_PD_SPARER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SY_PD_SPARER {
        match value {
            0 => SY_PD_SPARER::_0,
            1 => SY_PD_SPARER::_1,
            i => SY_PD_SPARER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_PD_SPARER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_PD_SPARER::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_PD_VTUNE_OVERRIDE_TEST_MODER {
    bits: bool,
}
impl SY_PD_VTUNE_OVERRIDE_TEST_MODER {
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
pub struct _SY_DIVN_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_DIVN_SPAREW<'a> {
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
pub struct _SY_FCAL_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_FCAL_SPAREW<'a> {
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
#[doc = "Values that can be written to the field `SY_LO_BUMP_RTLO_FDBK`"]
pub enum SY_LO_BUMP_RTLO_FDBKW {
    #[doc = "1.045 V"]
    _0,
    #[doc = "1.084 V"]
    _1,
    #[doc = "1.097 V"]
    _2,
    #[doc = "1.10 V"]
    _3,
}
impl SY_LO_BUMP_RTLO_FDBKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SY_LO_BUMP_RTLO_FDBKW::_0 => 0,
            SY_LO_BUMP_RTLO_FDBKW::_1 => 1,
            SY_LO_BUMP_RTLO_FDBKW::_2 => 2,
            SY_LO_BUMP_RTLO_FDBKW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_LO_BUMP_RTLO_FDBKW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_BUMP_RTLO_FDBKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_LO_BUMP_RTLO_FDBKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.045 V"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_LO_BUMP_RTLO_FDBKW::_0)
    }
    #[doc = "1.084 V"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_LO_BUMP_RTLO_FDBKW::_1)
    }
    #[doc = "1.097 V"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(SY_LO_BUMP_RTLO_FDBKW::_2)
    }
    #[doc = "1.10 V"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(SY_LO_BUMP_RTLO_FDBKW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SY_LO_BUMP_RTLO_RX`"]
pub enum SY_LO_BUMP_RTLO_RXW {
    #[doc = "1.051/1.037 V"]
    _0,
    #[doc = "1.082/1.075 V"]
    _1,
    #[doc = "1.092/1.088 V"]
    _2,
    #[doc = "1.098/1.094 V"]
    _3,
}
impl SY_LO_BUMP_RTLO_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SY_LO_BUMP_RTLO_RXW::_0 => 0,
            SY_LO_BUMP_RTLO_RXW::_1 => 1,
            SY_LO_BUMP_RTLO_RXW::_2 => 2,
            SY_LO_BUMP_RTLO_RXW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_LO_BUMP_RTLO_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_BUMP_RTLO_RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_LO_BUMP_RTLO_RXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.051/1.037 V"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_LO_BUMP_RTLO_RXW::_0)
    }
    #[doc = "1.082/1.075 V"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_LO_BUMP_RTLO_RXW::_1)
    }
    #[doc = "1.092/1.088 V"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(SY_LO_BUMP_RTLO_RXW::_2)
    }
    #[doc = "1.098/1.094 V"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(SY_LO_BUMP_RTLO_RXW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SY_LO_BUMP_RTLO_TX`"]
pub enum SY_LO_BUMP_RTLO_TXW {
    #[doc = "1.071/1.065 V"]
    _0,
    #[doc = "1.092/1.090 V"]
    _1,
    #[doc = "1.099/1.098 V"]
    _2,
    #[doc = "1.10/1.1 V"]
    _3,
}
impl SY_LO_BUMP_RTLO_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SY_LO_BUMP_RTLO_TXW::_0 => 0,
            SY_LO_BUMP_RTLO_TXW::_1 => 1,
            SY_LO_BUMP_RTLO_TXW::_2 => 2,
            SY_LO_BUMP_RTLO_TXW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_LO_BUMP_RTLO_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_BUMP_RTLO_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_LO_BUMP_RTLO_TXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.071/1.065 V"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_LO_BUMP_RTLO_TXW::_0)
    }
    #[doc = "1.092/1.090 V"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_LO_BUMP_RTLO_TXW::_1)
    }
    #[doc = "1.099/1.098 V"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(SY_LO_BUMP_RTLO_TXW::_2)
    }
    #[doc = "1.10/1.1 V"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(SY_LO_BUMP_RTLO_TXW::_3)
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
#[doc = "Values that can be written to the field `SY_LO_DIAGSEL`"]
pub enum SY_LO_DIAGSELW {
    #[doc = "Diag disable"]
    _0,
    #[doc = "Diag enable"]
    _1,
}
impl SY_LO_DIAGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_LO_DIAGSELW::_0 => false,
            SY_LO_DIAGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_LO_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_DIAGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_LO_DIAGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diag disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_LO_DIAGSELW::_0)
    }
    #[doc = "Diag enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_LO_DIAGSELW::_1)
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
#[doc = r" Proxy"]
pub struct _SY_LO_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_SPAREW<'a> {
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
#[doc = r" Proxy"]
pub struct _SY_LPF_FILT_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LPF_FILT_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SY_LPF_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LPF_SPAREW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SY_PD_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_PD_DIAGSELW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SY_PD_PCH_TUNEW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_PD_PCH_TUNEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SY_PD_PCH_SEL`"]
pub enum SY_PD_PCH_SELW {
    #[doc = "inverter based precharge"]
    _0,
    #[doc = "resistor divider based precharge"]
    _1,
}
impl SY_PD_PCH_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_PD_PCH_SELW::_0 => false,
            SY_PD_PCH_SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_PD_PCH_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_PD_PCH_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_PD_PCH_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "inverter based precharge"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_PD_PCH_SELW::_0)
    }
    #[doc = "resistor divider based precharge"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_PD_PCH_SELW::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SY_PD_SPARE`"]
pub enum SY_PD_SPAREW {
    #[doc = "Default ;"]
    _0,
    #[doc = "PD output is pulled down."]
    _1,
}
impl SY_PD_SPAREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SY_PD_SPAREW::_0 => 0,
            SY_PD_SPAREW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_PD_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_PD_SPAREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_PD_SPAREW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Default ;"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_PD_SPAREW::_0)
    }
    #[doc = "PD output is pulled down."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_PD_SPAREW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SY_PD_VTUNE_OVERRIDE_TEST_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_PD_VTUNE_OVERRIDE_TEST_MODEW<'a> {
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
    #[doc = "Bit 0 - rmap_sy_divn_spare"]
    #[inline]
    pub fn sy_divn_spare(&self) -> SY_DIVN_SPARER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_DIVN_SPARER { bits }
    }
    #[doc = "Bit 1 - rmap_sy_fcal_spare"]
    #[inline]
    pub fn sy_fcal_spare(&self) -> SY_FCAL_SPARER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_FCAL_SPARER { bits }
    }
    #[doc = "Bits 4:5 - rmap_sy_lo_bump_rtlo_fdbk[1:0]"]
    #[inline]
    pub fn sy_lo_bump_rtlo_fdbk(&self) -> SY_LO_BUMP_RTLO_FDBKR {
        SY_LO_BUMP_RTLO_FDBKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - rmap_sy_lo_bump_rtlo_rx[1:0]"]
    #[inline]
    pub fn sy_lo_bump_rtlo_rx(&self) -> SY_LO_BUMP_RTLO_RXR {
        SY_LO_BUMP_RTLO_RXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - rmap_sy_lo_bump_rtlo_tx[1:0]"]
    #[inline]
    pub fn sy_lo_bump_rtlo_tx(&self) -> SY_LO_BUMP_RTLO_TXR {
        SY_LO_BUMP_RTLO_TXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - rmap_sy_lo_diagsel"]
    #[inline]
    pub fn sy_lo_diagsel(&self) -> SY_LO_DIAGSELR {
        SY_LO_DIAGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:14 - rmap_sy_lo_spare[2:0]"]
    #[inline]
    pub fn sy_lo_spare(&self) -> SY_LO_SPARER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SY_LO_SPARER { bits }
    }
    #[doc = "Bits 16:18 - rmap_sy_lpf_filt_ctrl[2:0]"]
    #[inline]
    pub fn sy_lpf_filt_ctrl(&self) -> SY_LPF_FILT_CTRLR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SY_LPF_FILT_CTRLR { bits }
    }
    #[doc = "Bit 19 - rmap_sy_lpf_spare"]
    #[inline]
    pub fn sy_lpf_spare(&self) -> SY_LPF_SPARER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_LPF_SPARER { bits }
    }
    #[doc = "Bit 20 - rmap_sy_pd_diagsel"]
    #[inline]
    pub fn sy_pd_diagsel(&self) -> SY_PD_DIAGSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_PD_DIAGSELR { bits }
    }
    #[doc = "Bits 21:22 - rmap_sy_pd_pch_tune[1:0]"]
    #[inline]
    pub fn sy_pd_pch_tune(&self) -> SY_PD_PCH_TUNER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SY_PD_PCH_TUNER { bits }
    }
    #[doc = "Bit 23 - rmap_sy_pd_pch_sel"]
    #[inline]
    pub fn sy_pd_pch_sel(&self) -> SY_PD_PCH_SELR {
        SY_PD_PCH_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:25 - rmap_sy_pd_spare[1:0]"]
    #[inline]
    pub fn sy_pd_spare(&self) -> SY_PD_SPARER {
        SY_PD_SPARER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - rmap_sy_pd_vtune_override_test_mode"]
    #[inline]
    pub fn sy_pd_vtune_override_test_mode(&self) -> SY_PD_VTUNE_OVERRIDE_TEST_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_PD_VTUNE_OVERRIDE_TEST_MODER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 336 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - rmap_sy_divn_spare"]
    #[inline]
    pub fn sy_divn_spare(&mut self) -> _SY_DIVN_SPAREW {
        _SY_DIVN_SPAREW { w: self }
    }
    #[doc = "Bit 1 - rmap_sy_fcal_spare"]
    #[inline]
    pub fn sy_fcal_spare(&mut self) -> _SY_FCAL_SPAREW {
        _SY_FCAL_SPAREW { w: self }
    }
    #[doc = "Bits 4:5 - rmap_sy_lo_bump_rtlo_fdbk[1:0]"]
    #[inline]
    pub fn sy_lo_bump_rtlo_fdbk(&mut self) -> _SY_LO_BUMP_RTLO_FDBKW {
        _SY_LO_BUMP_RTLO_FDBKW { w: self }
    }
    #[doc = "Bits 6:7 - rmap_sy_lo_bump_rtlo_rx[1:0]"]
    #[inline]
    pub fn sy_lo_bump_rtlo_rx(&mut self) -> _SY_LO_BUMP_RTLO_RXW {
        _SY_LO_BUMP_RTLO_RXW { w: self }
    }
    #[doc = "Bits 8:9 - rmap_sy_lo_bump_rtlo_tx[1:0]"]
    #[inline]
    pub fn sy_lo_bump_rtlo_tx(&mut self) -> _SY_LO_BUMP_RTLO_TXW {
        _SY_LO_BUMP_RTLO_TXW { w: self }
    }
    #[doc = "Bit 10 - rmap_sy_lo_diagsel"]
    #[inline]
    pub fn sy_lo_diagsel(&mut self) -> _SY_LO_DIAGSELW {
        _SY_LO_DIAGSELW { w: self }
    }
    #[doc = "Bits 12:14 - rmap_sy_lo_spare[2:0]"]
    #[inline]
    pub fn sy_lo_spare(&mut self) -> _SY_LO_SPAREW {
        _SY_LO_SPAREW { w: self }
    }
    #[doc = "Bits 16:18 - rmap_sy_lpf_filt_ctrl[2:0]"]
    #[inline]
    pub fn sy_lpf_filt_ctrl(&mut self) -> _SY_LPF_FILT_CTRLW {
        _SY_LPF_FILT_CTRLW { w: self }
    }
    #[doc = "Bit 19 - rmap_sy_lpf_spare"]
    #[inline]
    pub fn sy_lpf_spare(&mut self) -> _SY_LPF_SPAREW {
        _SY_LPF_SPAREW { w: self }
    }
    #[doc = "Bit 20 - rmap_sy_pd_diagsel"]
    #[inline]
    pub fn sy_pd_diagsel(&mut self) -> _SY_PD_DIAGSELW {
        _SY_PD_DIAGSELW { w: self }
    }
    #[doc = "Bits 21:22 - rmap_sy_pd_pch_tune[1:0]"]
    #[inline]
    pub fn sy_pd_pch_tune(&mut self) -> _SY_PD_PCH_TUNEW {
        _SY_PD_PCH_TUNEW { w: self }
    }
    #[doc = "Bit 23 - rmap_sy_pd_pch_sel"]
    #[inline]
    pub fn sy_pd_pch_sel(&mut self) -> _SY_PD_PCH_SELW {
        _SY_PD_PCH_SELW { w: self }
    }
    #[doc = "Bits 24:25 - rmap_sy_pd_spare[1:0]"]
    #[inline]
    pub fn sy_pd_spare(&mut self) -> _SY_PD_SPAREW {
        _SY_PD_SPAREW { w: self }
    }
    #[doc = "Bit 28 - rmap_sy_pd_vtune_override_test_mode"]
    #[inline]
    pub fn sy_pd_vtune_override_test_mode(&mut self) -> _SY_PD_VTUNE_OVERRIDE_TEST_MODEW {
        _SY_PD_VTUNE_OVERRIDE_TEST_MODEW { w: self }
    }
}
