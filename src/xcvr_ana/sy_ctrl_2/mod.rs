#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SY_CTRL_2 {
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
#[doc = "Possible values of the field `SY_VCO_BIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_VCO_BIASR {
    #[doc = "0.97V"]
    _0,
    #[doc = "1.033V"]
    _1,
    #[doc = "1.06V"]
    _2,
    #[doc = "1.07V"]
    _3,
    #[doc = "1.08V"]
    _4,
    #[doc = "1.085V"]
    _5,
    #[doc = "1.090V"]
    _6,
    #[doc = "1.095V"]
    _7,
}
impl SY_VCO_BIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SY_VCO_BIASR::_0 => 0,
            SY_VCO_BIASR::_1 => 1,
            SY_VCO_BIASR::_2 => 2,
            SY_VCO_BIASR::_3 => 3,
            SY_VCO_BIASR::_4 => 4,
            SY_VCO_BIASR::_5 => 5,
            SY_VCO_BIASR::_6 => 6,
            SY_VCO_BIASR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SY_VCO_BIASR {
        match value {
            0 => SY_VCO_BIASR::_0,
            1 => SY_VCO_BIASR::_1,
            2 => SY_VCO_BIASR::_2,
            3 => SY_VCO_BIASR::_3,
            4 => SY_VCO_BIASR::_4,
            5 => SY_VCO_BIASR::_5,
            6 => SY_VCO_BIASR::_6,
            7 => SY_VCO_BIASR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_VCO_BIASR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_VCO_BIASR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == SY_VCO_BIASR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == SY_VCO_BIASR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == SY_VCO_BIASR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == SY_VCO_BIASR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == SY_VCO_BIASR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == SY_VCO_BIASR::_7
    }
}
#[doc = "Possible values of the field `SY_VCO_DIAGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_VCO_DIAGSELR {
    #[doc = "Diag enable"]
    _1,
    #[doc = "Diag disable"]
    _0,
}
impl SY_VCO_DIAGSELR {
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
            SY_VCO_DIAGSELR::_1 => true,
            SY_VCO_DIAGSELR::_0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_VCO_DIAGSELR {
        match value {
            true => SY_VCO_DIAGSELR::_1,
            false => SY_VCO_DIAGSELR::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_VCO_DIAGSELR::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_VCO_DIAGSELR::_0
    }
}
#[doc = "Possible values of the field `SY_VCO_KV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_VCO_KVR {
    #[doc = "50MHz/V"]
    _0,
    #[doc = "60MHz/V"]
    _1,
    #[doc = "70MHz/V"]
    _2,
    #[doc = "80MHz/V"]
    _3,
    #[doc = "80MHz/V"]
    _4,
    #[doc = "80MHz/V"]
    _5,
    #[doc = "80MHz/V"]
    _6,
    #[doc = "80MHz/V"]
    _7,
}
impl SY_VCO_KVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SY_VCO_KVR::_0 => 0,
            SY_VCO_KVR::_1 => 1,
            SY_VCO_KVR::_2 => 2,
            SY_VCO_KVR::_3 => 3,
            SY_VCO_KVR::_4 => 4,
            SY_VCO_KVR::_5 => 5,
            SY_VCO_KVR::_6 => 6,
            SY_VCO_KVR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SY_VCO_KVR {
        match value {
            0 => SY_VCO_KVR::_0,
            1 => SY_VCO_KVR::_1,
            2 => SY_VCO_KVR::_2,
            3 => SY_VCO_KVR::_3,
            4 => SY_VCO_KVR::_4,
            5 => SY_VCO_KVR::_5,
            6 => SY_VCO_KVR::_6,
            7 => SY_VCO_KVR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_VCO_KVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_VCO_KVR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == SY_VCO_KVR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == SY_VCO_KVR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == SY_VCO_KVR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == SY_VCO_KVR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == SY_VCO_KVR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == SY_VCO_KVR::_7
    }
}
#[doc = "Possible values of the field `SY_VCO_KVM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_VCO_KVMR {
    #[doc = "10MHz/V"]
    _0,
    #[doc = "20MHz/V"]
    _1,
    #[doc = "30MHz/V"]
    _2,
    #[doc = "40MHz/V"]
    _3,
    #[doc = "40MHz/V"]
    _4,
    #[doc = "40MHz/V"]
    _5,
    #[doc = "40MHz/V"]
    _6,
    #[doc = "40MHz/V"]
    _7,
}
impl SY_VCO_KVMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SY_VCO_KVMR::_0 => 0,
            SY_VCO_KVMR::_1 => 1,
            SY_VCO_KVMR::_2 => 2,
            SY_VCO_KVMR::_3 => 3,
            SY_VCO_KVMR::_4 => 4,
            SY_VCO_KVMR::_5 => 5,
            SY_VCO_KVMR::_6 => 6,
            SY_VCO_KVMR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SY_VCO_KVMR {
        match value {
            0 => SY_VCO_KVMR::_0,
            1 => SY_VCO_KVMR::_1,
            2 => SY_VCO_KVMR::_2,
            3 => SY_VCO_KVMR::_3,
            4 => SY_VCO_KVMR::_4,
            5 => SY_VCO_KVMR::_5,
            6 => SY_VCO_KVMR::_6,
            7 => SY_VCO_KVMR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_VCO_KVMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_VCO_KVMR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == SY_VCO_KVMR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == SY_VCO_KVMR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == SY_VCO_KVMR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == SY_VCO_KVMR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == SY_VCO_KVMR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == SY_VCO_KVMR::_7
    }
}
#[doc = "Possible values of the field `SY_VCO_PK_DET_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_VCO_PK_DET_ONR {
    #[doc = "Enable"]
    _1,
    #[doc = "Disable"]
    _0,
}
impl SY_VCO_PK_DET_ONR {
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
            SY_VCO_PK_DET_ONR::_1 => true,
            SY_VCO_PK_DET_ONR::_0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_VCO_PK_DET_ONR {
        match value {
            true => SY_VCO_PK_DET_ONR::_1,
            false => SY_VCO_PK_DET_ONR::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_VCO_PK_DET_ONR::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_VCO_PK_DET_ONR::_0
    }
}
#[doc = r" Value of the field"]
pub struct SY_VCO_SPARER {
    bits: u8,
}
impl SY_VCO_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SY_VCO_BIAS`"]
pub enum SY_VCO_BIASW {
    #[doc = "0.97V"]
    _0,
    #[doc = "1.033V"]
    _1,
    #[doc = "1.06V"]
    _2,
    #[doc = "1.07V"]
    _3,
    #[doc = "1.08V"]
    _4,
    #[doc = "1.085V"]
    _5,
    #[doc = "1.090V"]
    _6,
    #[doc = "1.095V"]
    _7,
}
impl SY_VCO_BIASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SY_VCO_BIASW::_0 => 0,
            SY_VCO_BIASW::_1 => 1,
            SY_VCO_BIASW::_2 => 2,
            SY_VCO_BIASW::_3 => 3,
            SY_VCO_BIASW::_4 => 4,
            SY_VCO_BIASW::_5 => 5,
            SY_VCO_BIASW::_6 => 6,
            SY_VCO_BIASW::_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_VCO_BIASW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_VCO_BIASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_VCO_BIASW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0.97V"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_VCO_BIASW::_0)
    }
    #[doc = "1.033V"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_VCO_BIASW::_1)
    }
    #[doc = "1.06V"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(SY_VCO_BIASW::_2)
    }
    #[doc = "1.07V"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(SY_VCO_BIASW::_3)
    }
    #[doc = "1.08V"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(SY_VCO_BIASW::_4)
    }
    #[doc = "1.085V"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(SY_VCO_BIASW::_5)
    }
    #[doc = "1.090V"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(SY_VCO_BIASW::_6)
    }
    #[doc = "1.095V"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(SY_VCO_BIASW::_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SY_VCO_DIAGSEL`"]
pub enum SY_VCO_DIAGSELW {
    #[doc = "Diag enable"]
    _1,
    #[doc = "Diag disable"]
    _0,
}
impl SY_VCO_DIAGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_VCO_DIAGSELW::_1 => true,
            SY_VCO_DIAGSELW::_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_VCO_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_VCO_DIAGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_VCO_DIAGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diag enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_VCO_DIAGSELW::_1)
    }
    #[doc = "Diag disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_VCO_DIAGSELW::_0)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SY_VCO_KV`"]
pub enum SY_VCO_KVW {
    #[doc = "50MHz/V"]
    _0,
    #[doc = "60MHz/V"]
    _1,
    #[doc = "70MHz/V"]
    _2,
    #[doc = "80MHz/V"]
    _3,
    #[doc = "80MHz/V"]
    _4,
    #[doc = "80MHz/V"]
    _5,
    #[doc = "80MHz/V"]
    _6,
    #[doc = "80MHz/V"]
    _7,
}
impl SY_VCO_KVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SY_VCO_KVW::_0 => 0,
            SY_VCO_KVW::_1 => 1,
            SY_VCO_KVW::_2 => 2,
            SY_VCO_KVW::_3 => 3,
            SY_VCO_KVW::_4 => 4,
            SY_VCO_KVW::_5 => 5,
            SY_VCO_KVW::_6 => 6,
            SY_VCO_KVW::_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_VCO_KVW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_VCO_KVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_VCO_KVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "50MHz/V"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_VCO_KVW::_0)
    }
    #[doc = "60MHz/V"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_VCO_KVW::_1)
    }
    #[doc = "70MHz/V"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(SY_VCO_KVW::_2)
    }
    #[doc = "80MHz/V"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(SY_VCO_KVW::_3)
    }
    #[doc = "80MHz/V"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(SY_VCO_KVW::_4)
    }
    #[doc = "80MHz/V"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(SY_VCO_KVW::_5)
    }
    #[doc = "80MHz/V"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(SY_VCO_KVW::_6)
    }
    #[doc = "80MHz/V"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(SY_VCO_KVW::_7)
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
#[doc = "Values that can be written to the field `SY_VCO_KVM`"]
pub enum SY_VCO_KVMW {
    #[doc = "10MHz/V"]
    _0,
    #[doc = "20MHz/V"]
    _1,
    #[doc = "30MHz/V"]
    _2,
    #[doc = "40MHz/V"]
    _3,
    #[doc = "40MHz/V"]
    _4,
    #[doc = "40MHz/V"]
    _5,
    #[doc = "40MHz/V"]
    _6,
    #[doc = "40MHz/V"]
    _7,
}
impl SY_VCO_KVMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SY_VCO_KVMW::_0 => 0,
            SY_VCO_KVMW::_1 => 1,
            SY_VCO_KVMW::_2 => 2,
            SY_VCO_KVMW::_3 => 3,
            SY_VCO_KVMW::_4 => 4,
            SY_VCO_KVMW::_5 => 5,
            SY_VCO_KVMW::_6 => 6,
            SY_VCO_KVMW::_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_VCO_KVMW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_VCO_KVMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_VCO_KVMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "10MHz/V"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_VCO_KVMW::_0)
    }
    #[doc = "20MHz/V"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_VCO_KVMW::_1)
    }
    #[doc = "30MHz/V"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(SY_VCO_KVMW::_2)
    }
    #[doc = "40MHz/V"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(SY_VCO_KVMW::_3)
    }
    #[doc = "40MHz/V"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(SY_VCO_KVMW::_4)
    }
    #[doc = "40MHz/V"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(SY_VCO_KVMW::_5)
    }
    #[doc = "40MHz/V"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(SY_VCO_KVMW::_6)
    }
    #[doc = "40MHz/V"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(SY_VCO_KVMW::_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SY_VCO_PK_DET_ON`"]
pub enum SY_VCO_PK_DET_ONW {
    #[doc = "Enable"]
    _1,
    #[doc = "Disable"]
    _0,
}
impl SY_VCO_PK_DET_ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_VCO_PK_DET_ONW::_1 => true,
            SY_VCO_PK_DET_ONW::_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_VCO_PK_DET_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_VCO_PK_DET_ONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_VCO_PK_DET_ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_VCO_PK_DET_ONW::_1)
    }
    #[doc = "Disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_VCO_PK_DET_ONW::_0)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SY_VCO_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_VCO_SPAREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:2 - rmap_sy_vco_bias[2:0]"]
    #[inline]
    pub fn sy_vco_bias(&self) -> SY_VCO_BIASR {
        SY_VCO_BIASR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - rmap_sy_vco_diagsel"]
    #[inline]
    pub fn sy_vco_diagsel(&self) -> SY_VCO_DIAGSELR {
        SY_VCO_DIAGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - rmap_sy_vco_kv[2:0]"]
    #[inline]
    pub fn sy_vco_kv(&self) -> SY_VCO_KVR {
        SY_VCO_KVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - rmap_sy_vco_kvm[2:0]"]
    #[inline]
    pub fn sy_vco_kvm(&self) -> SY_VCO_KVMR {
        SY_VCO_KVMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - rmap_sy_vco_pk_det_on"]
    #[inline]
    pub fn sy_vco_pk_det_on(&self) -> SY_VCO_PK_DET_ONR {
        SY_VCO_PK_DET_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:16 - rmap_sy_vco_spare[2:0]"]
    #[inline]
    pub fn sy_vco_spare(&self) -> SY_VCO_SPARER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SY_VCO_SPARER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 20 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - rmap_sy_vco_bias[2:0]"]
    #[inline]
    pub fn sy_vco_bias(&mut self) -> _SY_VCO_BIASW {
        _SY_VCO_BIASW { w: self }
    }
    #[doc = "Bit 3 - rmap_sy_vco_diagsel"]
    #[inline]
    pub fn sy_vco_diagsel(&mut self) -> _SY_VCO_DIAGSELW {
        _SY_VCO_DIAGSELW { w: self }
    }
    #[doc = "Bits 4:6 - rmap_sy_vco_kv[2:0]"]
    #[inline]
    pub fn sy_vco_kv(&mut self) -> _SY_VCO_KVW {
        _SY_VCO_KVW { w: self }
    }
    #[doc = "Bits 8:10 - rmap_sy_vco_kvm[2:0]"]
    #[inline]
    pub fn sy_vco_kvm(&mut self) -> _SY_VCO_KVMW {
        _SY_VCO_KVMW { w: self }
    }
    #[doc = "Bit 12 - rmap_sy_vco_pk_det_on"]
    #[inline]
    pub fn sy_vco_pk_det_on(&mut self) -> _SY_VCO_PK_DET_ONW {
        _SY_VCO_PK_DET_ONW { w: self }
    }
    #[doc = "Bits 14:16 - rmap_sy_vco_spare[2:0]"]
    #[inline]
    pub fn sy_vco_spare(&mut self) -> _SY_VCO_SPAREW {
        _SY_VCO_SPAREW { w: self }
    }
}
