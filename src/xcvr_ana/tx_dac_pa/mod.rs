#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TX_DAC_PA {
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
#[doc = "Possible values of the field `TX_DAC_BUMP_CAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DAC_BUMP_CAPR {
    #[doc = "1pF(default)"]
    _0,
    #[doc = "1.5pF"]
    _1,
    #[doc = "1.5pF"]
    _2,
    #[doc = "2pF"]
    _3,
}
impl TX_DAC_BUMP_CAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX_DAC_BUMP_CAPR::_0 => 0,
            TX_DAC_BUMP_CAPR::_1 => 1,
            TX_DAC_BUMP_CAPR::_2 => 2,
            TX_DAC_BUMP_CAPR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX_DAC_BUMP_CAPR {
        match value {
            0 => TX_DAC_BUMP_CAPR::_0,
            1 => TX_DAC_BUMP_CAPR::_1,
            2 => TX_DAC_BUMP_CAPR::_2,
            3 => TX_DAC_BUMP_CAPR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_DAC_BUMP_CAPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_DAC_BUMP_CAPR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == TX_DAC_BUMP_CAPR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == TX_DAC_BUMP_CAPR::_3
    }
}
#[doc = "Possible values of the field `TX_DAC_BUMP_IDAC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DAC_BUMP_IDACR {
    #[doc = "250nA(default)"]
    _0,
    #[doc = "207nA"]
    _1,
    #[doc = "312nA"]
    _2,
    #[doc = "415nA"]
    _3,
}
impl TX_DAC_BUMP_IDACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX_DAC_BUMP_IDACR::_0 => 0,
            TX_DAC_BUMP_IDACR::_1 => 1,
            TX_DAC_BUMP_IDACR::_2 => 2,
            TX_DAC_BUMP_IDACR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX_DAC_BUMP_IDACR {
        match value {
            0 => TX_DAC_BUMP_IDACR::_0,
            1 => TX_DAC_BUMP_IDACR::_1,
            2 => TX_DAC_BUMP_IDACR::_2,
            3 => TX_DAC_BUMP_IDACR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_DAC_BUMP_IDACR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_DAC_BUMP_IDACR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == TX_DAC_BUMP_IDACR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == TX_DAC_BUMP_IDACR::_3
    }
}
#[doc = "Possible values of the field `TX_DAC_BUMP_RLOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DAC_BUMP_RLOADR {
    #[doc = "3.12 kohms(default)"]
    _0,
    #[doc = "2.34 kohms"]
    _1,
    #[doc = "3.9 kohms"]
    _2,
    #[doc = "4.6 kohms"]
    _3,
}
impl TX_DAC_BUMP_RLOADR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX_DAC_BUMP_RLOADR::_0 => 0,
            TX_DAC_BUMP_RLOADR::_1 => 1,
            TX_DAC_BUMP_RLOADR::_2 => 2,
            TX_DAC_BUMP_RLOADR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX_DAC_BUMP_RLOADR {
        match value {
            0 => TX_DAC_BUMP_RLOADR::_0,
            1 => TX_DAC_BUMP_RLOADR::_1,
            2 => TX_DAC_BUMP_RLOADR::_2,
            3 => TX_DAC_BUMP_RLOADR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_DAC_BUMP_RLOADR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_DAC_BUMP_RLOADR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == TX_DAC_BUMP_RLOADR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == TX_DAC_BUMP_RLOADR::_3
    }
}
#[doc = "Possible values of the field `TX_DAC_DIAGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DAC_DIAGSELR {
    #[doc = "Disable Diag"]
    _0,
    #[doc = "Enable Diag"]
    _1,
}
impl TX_DAC_DIAGSELR {
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
            TX_DAC_DIAGSELR::_0 => false,
            TX_DAC_DIAGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_DAC_DIAGSELR {
        match value {
            false => TX_DAC_DIAGSELR::_0,
            true => TX_DAC_DIAGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_DAC_DIAGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_DAC_DIAGSELR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TX_DAC_INVERT_CLKR {
    bits: bool,
}
impl TX_DAC_INVERT_CLKR {
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
#[doc = "Possible values of the field `TX_DAC_OPAMP_DIAGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DAC_OPAMP_DIAGSELR {
    #[doc = "Disable Diag"]
    _0,
    #[doc = "Enable Diag"]
    _1,
}
impl TX_DAC_OPAMP_DIAGSELR {
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
            TX_DAC_OPAMP_DIAGSELR::_0 => false,
            TX_DAC_OPAMP_DIAGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_DAC_OPAMP_DIAGSELR {
        match value {
            false => TX_DAC_OPAMP_DIAGSELR::_0,
            true => TX_DAC_OPAMP_DIAGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_DAC_OPAMP_DIAGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_DAC_OPAMP_DIAGSELR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TX_DAC_SPARER {
    bits: u8,
}
impl TX_DAC_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TX_PA_BUMP_VBIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_PA_BUMP_VBIASR {
    #[doc = "0.557"]
    _0,
    #[doc = "0.651"]
    _1,
    #[doc = "0.741"]
    _2,
    #[doc = "0.822"]
    _3,
    #[doc = "0.590"]
    _4,
    #[doc = "0.683"]
    _5,
    #[doc = "0.771"]
    _6,
    #[doc = "0.850"]
    _7,
}
impl TX_PA_BUMP_VBIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX_PA_BUMP_VBIASR::_0 => 0,
            TX_PA_BUMP_VBIASR::_1 => 1,
            TX_PA_BUMP_VBIASR::_2 => 2,
            TX_PA_BUMP_VBIASR::_3 => 3,
            TX_PA_BUMP_VBIASR::_4 => 4,
            TX_PA_BUMP_VBIASR::_5 => 5,
            TX_PA_BUMP_VBIASR::_6 => 6,
            TX_PA_BUMP_VBIASR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX_PA_BUMP_VBIASR {
        match value {
            0 => TX_PA_BUMP_VBIASR::_0,
            1 => TX_PA_BUMP_VBIASR::_1,
            2 => TX_PA_BUMP_VBIASR::_2,
            3 => TX_PA_BUMP_VBIASR::_3,
            4 => TX_PA_BUMP_VBIASR::_4,
            5 => TX_PA_BUMP_VBIASR::_5,
            6 => TX_PA_BUMP_VBIASR::_6,
            7 => TX_PA_BUMP_VBIASR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_PA_BUMP_VBIASR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_PA_BUMP_VBIASR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == TX_PA_BUMP_VBIASR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == TX_PA_BUMP_VBIASR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == TX_PA_BUMP_VBIASR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == TX_PA_BUMP_VBIASR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == TX_PA_BUMP_VBIASR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == TX_PA_BUMP_VBIASR::_7
    }
}
#[doc = r" Value of the field"]
pub struct TX_PA_DIAGSELR {
    bits: bool,
}
impl TX_PA_DIAGSELR {
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
pub struct TX_PA_SPARER {
    bits: u8,
}
impl TX_PA_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TX_DAC_BUMP_CAP`"]
pub enum TX_DAC_BUMP_CAPW {
    #[doc = "1pF(default)"]
    _0,
    #[doc = "1.5pF"]
    _1,
    #[doc = "1.5pF"]
    _2,
    #[doc = "2pF"]
    _3,
}
impl TX_DAC_BUMP_CAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TX_DAC_BUMP_CAPW::_0 => 0,
            TX_DAC_BUMP_CAPW::_1 => 1,
            TX_DAC_BUMP_CAPW::_2 => 2,
            TX_DAC_BUMP_CAPW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_DAC_BUMP_CAPW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DAC_BUMP_CAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_DAC_BUMP_CAPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1pF(default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_DAC_BUMP_CAPW::_0)
    }
    #[doc = "1.5pF"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_DAC_BUMP_CAPW::_1)
    }
    #[doc = "1.5pF"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(TX_DAC_BUMP_CAPW::_2)
    }
    #[doc = "2pF"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(TX_DAC_BUMP_CAPW::_3)
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
#[doc = "Values that can be written to the field `TX_DAC_BUMP_IDAC`"]
pub enum TX_DAC_BUMP_IDACW {
    #[doc = "250nA(default)"]
    _0,
    #[doc = "207nA"]
    _1,
    #[doc = "312nA"]
    _2,
    #[doc = "415nA"]
    _3,
}
impl TX_DAC_BUMP_IDACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TX_DAC_BUMP_IDACW::_0 => 0,
            TX_DAC_BUMP_IDACW::_1 => 1,
            TX_DAC_BUMP_IDACW::_2 => 2,
            TX_DAC_BUMP_IDACW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_DAC_BUMP_IDACW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DAC_BUMP_IDACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_DAC_BUMP_IDACW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "250nA(default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_DAC_BUMP_IDACW::_0)
    }
    #[doc = "207nA"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_DAC_BUMP_IDACW::_1)
    }
    #[doc = "312nA"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(TX_DAC_BUMP_IDACW::_2)
    }
    #[doc = "415nA"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(TX_DAC_BUMP_IDACW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_DAC_BUMP_RLOAD`"]
pub enum TX_DAC_BUMP_RLOADW {
    #[doc = "3.12 kohms(default)"]
    _0,
    #[doc = "2.34 kohms"]
    _1,
    #[doc = "3.9 kohms"]
    _2,
    #[doc = "4.6 kohms"]
    _3,
}
impl TX_DAC_BUMP_RLOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TX_DAC_BUMP_RLOADW::_0 => 0,
            TX_DAC_BUMP_RLOADW::_1 => 1,
            TX_DAC_BUMP_RLOADW::_2 => 2,
            TX_DAC_BUMP_RLOADW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_DAC_BUMP_RLOADW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DAC_BUMP_RLOADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_DAC_BUMP_RLOADW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "3.12 kohms(default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_DAC_BUMP_RLOADW::_0)
    }
    #[doc = "2.34 kohms"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_DAC_BUMP_RLOADW::_1)
    }
    #[doc = "3.9 kohms"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(TX_DAC_BUMP_RLOADW::_2)
    }
    #[doc = "4.6 kohms"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(TX_DAC_BUMP_RLOADW::_3)
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
#[doc = "Values that can be written to the field `TX_DAC_DIAGSEL`"]
pub enum TX_DAC_DIAGSELW {
    #[doc = "Disable Diag"]
    _0,
    #[doc = "Enable Diag"]
    _1,
}
impl TX_DAC_DIAGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_DAC_DIAGSELW::_0 => false,
            TX_DAC_DIAGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_DAC_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DAC_DIAGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_DAC_DIAGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Diag"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_DAC_DIAGSELW::_0)
    }
    #[doc = "Enable Diag"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_DAC_DIAGSELW::_1)
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
pub struct _TX_DAC_INVERT_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DAC_INVERT_CLKW<'a> {
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
#[doc = "Values that can be written to the field `TX_DAC_OPAMP_DIAGSEL`"]
pub enum TX_DAC_OPAMP_DIAGSELW {
    #[doc = "Disable Diag"]
    _0,
    #[doc = "Enable Diag"]
    _1,
}
impl TX_DAC_OPAMP_DIAGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_DAC_OPAMP_DIAGSELW::_0 => false,
            TX_DAC_OPAMP_DIAGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_DAC_OPAMP_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DAC_OPAMP_DIAGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_DAC_OPAMP_DIAGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Diag"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_DAC_OPAMP_DIAGSELW::_0)
    }
    #[doc = "Enable Diag"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_DAC_OPAMP_DIAGSELW::_1)
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
#[doc = r" Proxy"]
pub struct _TX_DAC_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DAC_SPAREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_PA_BUMP_VBIAS`"]
pub enum TX_PA_BUMP_VBIASW {
    #[doc = "0.557"]
    _0,
    #[doc = "0.651"]
    _1,
    #[doc = "0.741"]
    _2,
    #[doc = "0.822"]
    _3,
    #[doc = "0.590"]
    _4,
    #[doc = "0.683"]
    _5,
    #[doc = "0.771"]
    _6,
    #[doc = "0.850"]
    _7,
}
impl TX_PA_BUMP_VBIASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TX_PA_BUMP_VBIASW::_0 => 0,
            TX_PA_BUMP_VBIASW::_1 => 1,
            TX_PA_BUMP_VBIASW::_2 => 2,
            TX_PA_BUMP_VBIASW::_3 => 3,
            TX_PA_BUMP_VBIASW::_4 => 4,
            TX_PA_BUMP_VBIASW::_5 => 5,
            TX_PA_BUMP_VBIASW::_6 => 6,
            TX_PA_BUMP_VBIASW::_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_PA_BUMP_VBIASW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_PA_BUMP_VBIASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_PA_BUMP_VBIASW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0.557"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_PA_BUMP_VBIASW::_0)
    }
    #[doc = "0.651"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_PA_BUMP_VBIASW::_1)
    }
    #[doc = "0.741"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(TX_PA_BUMP_VBIASW::_2)
    }
    #[doc = "0.822"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(TX_PA_BUMP_VBIASW::_3)
    }
    #[doc = "0.590"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(TX_PA_BUMP_VBIASW::_4)
    }
    #[doc = "0.683"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(TX_PA_BUMP_VBIASW::_5)
    }
    #[doc = "0.771"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(TX_PA_BUMP_VBIASW::_6)
    }
    #[doc = "0.850"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(TX_PA_BUMP_VBIASW::_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX_PA_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_PA_DIAGSELW<'a> {
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
pub struct _TX_PA_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_PA_SPAREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 23;
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
    #[doc = "Bits 0:1 - rmap_tx_dac_bump_cap[1:0]"]
    #[inline]
    pub fn tx_dac_bump_cap(&self) -> TX_DAC_BUMP_CAPR {
        TX_DAC_BUMP_CAPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:4 - rmap_tx_dac_bump_idac[1:0]"]
    #[inline]
    pub fn tx_dac_bump_idac(&self) -> TX_DAC_BUMP_IDACR {
        TX_DAC_BUMP_IDACR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - rmap_tx_dac_bump_rload[1:0]"]
    #[inline]
    pub fn tx_dac_bump_rload(&self) -> TX_DAC_BUMP_RLOADR {
        TX_DAC_BUMP_RLOADR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 9 - rmap_tx_dac_diagsel"]
    #[inline]
    pub fn tx_dac_diagsel(&self) -> TX_DAC_DIAGSELR {
        TX_DAC_DIAGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - rmap_tx_dac_invert_clk"]
    #[inline]
    pub fn tx_dac_invert_clk(&self) -> TX_DAC_INVERT_CLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_DAC_INVERT_CLKR { bits }
    }
    #[doc = "Bit 11 - rmap_tx_dac_opamp_diagsel"]
    #[inline]
    pub fn tx_dac_opamp_diagsel(&self) -> TX_DAC_OPAMP_DIAGSELR {
        TX_DAC_OPAMP_DIAGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:15 - rmap_tx_dac_spare[2:0]"]
    #[inline]
    pub fn tx_dac_spare(&self) -> TX_DAC_SPARER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TX_DAC_SPARER { bits }
    }
    #[doc = "Bits 17:19 - rmap_tx_pa_bump_vbias[2:0]"]
    #[inline]
    pub fn tx_pa_bump_vbias(&self) -> TX_PA_BUMP_VBIASR {
        TX_PA_BUMP_VBIASR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - rmap_tx_pa_diagsel"]
    #[inline]
    pub fn tx_pa_diagsel(&self) -> TX_PA_DIAGSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_PA_DIAGSELR { bits }
    }
    #[doc = "Bits 23:25 - rmap_tx_pa_spare[2:0]"]
    #[inline]
    pub fn tx_pa_spare(&self) -> TX_PA_SPARER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TX_PA_SPARER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 131072 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - rmap_tx_dac_bump_cap[1:0]"]
    #[inline]
    pub fn tx_dac_bump_cap(&mut self) -> _TX_DAC_BUMP_CAPW {
        _TX_DAC_BUMP_CAPW { w: self }
    }
    #[doc = "Bits 3:4 - rmap_tx_dac_bump_idac[1:0]"]
    #[inline]
    pub fn tx_dac_bump_idac(&mut self) -> _TX_DAC_BUMP_IDACW {
        _TX_DAC_BUMP_IDACW { w: self }
    }
    #[doc = "Bits 6:7 - rmap_tx_dac_bump_rload[1:0]"]
    #[inline]
    pub fn tx_dac_bump_rload(&mut self) -> _TX_DAC_BUMP_RLOADW {
        _TX_DAC_BUMP_RLOADW { w: self }
    }
    #[doc = "Bit 9 - rmap_tx_dac_diagsel"]
    #[inline]
    pub fn tx_dac_diagsel(&mut self) -> _TX_DAC_DIAGSELW {
        _TX_DAC_DIAGSELW { w: self }
    }
    #[doc = "Bit 10 - rmap_tx_dac_invert_clk"]
    #[inline]
    pub fn tx_dac_invert_clk(&mut self) -> _TX_DAC_INVERT_CLKW {
        _TX_DAC_INVERT_CLKW { w: self }
    }
    #[doc = "Bit 11 - rmap_tx_dac_opamp_diagsel"]
    #[inline]
    pub fn tx_dac_opamp_diagsel(&mut self) -> _TX_DAC_OPAMP_DIAGSELW {
        _TX_DAC_OPAMP_DIAGSELW { w: self }
    }
    #[doc = "Bits 13:15 - rmap_tx_dac_spare[2:0]"]
    #[inline]
    pub fn tx_dac_spare(&mut self) -> _TX_DAC_SPAREW {
        _TX_DAC_SPAREW { w: self }
    }
    #[doc = "Bits 17:19 - rmap_tx_pa_bump_vbias[2:0]"]
    #[inline]
    pub fn tx_pa_bump_vbias(&mut self) -> _TX_PA_BUMP_VBIASW {
        _TX_PA_BUMP_VBIASW { w: self }
    }
    #[doc = "Bit 21 - rmap_tx_pa_diagsel"]
    #[inline]
    pub fn tx_pa_diagsel(&mut self) -> _TX_PA_DIAGSELW {
        _TX_PA_DIAGSELW { w: self }
    }
    #[doc = "Bits 23:25 - rmap_tx_pa_spare[2:0]"]
    #[inline]
    pub fn tx_pa_spare(&mut self) -> _TX_PA_SPAREW {
        _TX_PA_SPAREW { w: self }
    }
}
