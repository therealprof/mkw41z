#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OVRD0 {
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
#[doc = "Possible values of the field `BB_LDO_HF_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_HF_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_HF_EN_OVRD to override the signal \"bb_ldo_hf_en\"."]
    _1,
}
impl BB_LDO_HF_EN_OVRD_ENR {
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
            BB_LDO_HF_EN_OVRD_ENR::_0 => false,
            BB_LDO_HF_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_HF_EN_OVRD_ENR {
        match value {
            false => BB_LDO_HF_EN_OVRD_ENR::_0,
            true => BB_LDO_HF_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_HF_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_HF_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_HF_EN_OVRDR {
    bits: bool,
}
impl BB_LDO_HF_EN_OVRDR {
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
#[doc = "Possible values of the field `BB_LDO_ADCDAC_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_ADCDAC_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_ADCDAC_EN_OVRD to override the signal \"bb_ldo_adcdac_en\"."]
    _1,
}
impl BB_LDO_ADCDAC_EN_OVRD_ENR {
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
            BB_LDO_ADCDAC_EN_OVRD_ENR::_0 => false,
            BB_LDO_ADCDAC_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_ADCDAC_EN_OVRD_ENR {
        match value {
            false => BB_LDO_ADCDAC_EN_OVRD_ENR::_0,
            true => BB_LDO_ADCDAC_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_ADCDAC_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_ADCDAC_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_ADCDAC_EN_OVRDR {
    bits: bool,
}
impl BB_LDO_ADCDAC_EN_OVRDR {
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
#[doc = "Possible values of the field `BB_LDO_BBA_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_BBA_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_BBA_EN_OVRD to override the signal \"bb_ldo_bba_en\"."]
    _1,
}
impl BB_LDO_BBA_EN_OVRD_ENR {
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
            BB_LDO_BBA_EN_OVRD_ENR::_0 => false,
            BB_LDO_BBA_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_BBA_EN_OVRD_ENR {
        match value {
            false => BB_LDO_BBA_EN_OVRD_ENR::_0,
            true => BB_LDO_BBA_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_BBA_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_BBA_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_BBA_EN_OVRDR {
    bits: bool,
}
impl BB_LDO_BBA_EN_OVRDR {
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
#[doc = "Possible values of the field `BB_LDO_PD_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_PD_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_PD_EN_OVRD to override the signal \"bb_ldo_pd_en\"."]
    _1,
}
impl BB_LDO_PD_EN_OVRD_ENR {
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
            BB_LDO_PD_EN_OVRD_ENR::_0 => false,
            BB_LDO_PD_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_PD_EN_OVRD_ENR {
        match value {
            false => BB_LDO_PD_EN_OVRD_ENR::_0,
            true => BB_LDO_PD_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_PD_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_PD_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_PD_EN_OVRDR {
    bits: bool,
}
impl BB_LDO_PD_EN_OVRDR {
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
#[doc = "Possible values of the field `BB_LDO_FDBK_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_FDBK_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_FDBK_EN_OVRD to override the signal \"bb_ldo_fdbk_en\"."]
    _1,
}
impl BB_LDO_FDBK_EN_OVRD_ENR {
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
            BB_LDO_FDBK_EN_OVRD_ENR::_0 => false,
            BB_LDO_FDBK_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_FDBK_EN_OVRD_ENR {
        match value {
            false => BB_LDO_FDBK_EN_OVRD_ENR::_0,
            true => BB_LDO_FDBK_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_FDBK_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_FDBK_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_FDBK_EN_OVRDR {
    bits: bool,
}
impl BB_LDO_FDBK_EN_OVRDR {
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
#[doc = "Possible values of the field `BB_LDO_VCOLO_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_VCOLO_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_VCOLO_EN_OVRD to override the signal \"bb_ldo_vcolo_en\"."]
    _1,
}
impl BB_LDO_VCOLO_EN_OVRD_ENR {
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
            BB_LDO_VCOLO_EN_OVRD_ENR::_0 => false,
            BB_LDO_VCOLO_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_VCOLO_EN_OVRD_ENR {
        match value {
            false => BB_LDO_VCOLO_EN_OVRD_ENR::_0,
            true => BB_LDO_VCOLO_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_VCOLO_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_VCOLO_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_VCOLO_EN_OVRDR {
    bits: bool,
}
impl BB_LDO_VCOLO_EN_OVRDR {
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
#[doc = "Possible values of the field `BB_LDO_VTREF_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_VTREF_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_VTREF_EN_OVRD to override the signal \"bb_ldo_vtref_en\"."]
    _1,
}
impl BB_LDO_VTREF_EN_OVRD_ENR {
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
            BB_LDO_VTREF_EN_OVRD_ENR::_0 => false,
            BB_LDO_VTREF_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_VTREF_EN_OVRD_ENR {
        match value {
            false => BB_LDO_VTREF_EN_OVRD_ENR::_0,
            true => BB_LDO_VTREF_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_VTREF_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_VTREF_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_VTREF_EN_OVRDR {
    bits: bool,
}
impl BB_LDO_VTREF_EN_OVRDR {
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
#[doc = "Possible values of the field `BB_LDO_FDBK_BLEED_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_FDBK_BLEED_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_FDBK_BLEED_EN_OVRD to override the signal \"bb_ldo_fdbk_bleed_en\"."]
    _1,
}
impl BB_LDO_FDBK_BLEED_EN_OVRD_ENR {
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
            BB_LDO_FDBK_BLEED_EN_OVRD_ENR::_0 => false,
            BB_LDO_FDBK_BLEED_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_FDBK_BLEED_EN_OVRD_ENR {
        match value {
            false => BB_LDO_FDBK_BLEED_EN_OVRD_ENR::_0,
            true => BB_LDO_FDBK_BLEED_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_FDBK_BLEED_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_FDBK_BLEED_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_FDBK_BLEED_EN_OVRDR {
    bits: bool,
}
impl BB_LDO_FDBK_BLEED_EN_OVRDR {
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
#[doc = "Possible values of the field `BB_LDO_VCOLO_BLEED_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_VCOLO_BLEED_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_VCOLO_BLEED_EN_OVRD to override the signal \"bb_ldo_vcolo_bleed_en\"."]
    _1,
}
impl BB_LDO_VCOLO_BLEED_EN_OVRD_ENR {
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
            BB_LDO_VCOLO_BLEED_EN_OVRD_ENR::_0 => false,
            BB_LDO_VCOLO_BLEED_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_VCOLO_BLEED_EN_OVRD_ENR {
        match value {
            false => BB_LDO_VCOLO_BLEED_EN_OVRD_ENR::_0,
            true => BB_LDO_VCOLO_BLEED_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_VCOLO_BLEED_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_VCOLO_BLEED_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_VCOLO_BLEED_EN_OVRDR {
    bits: bool,
}
impl BB_LDO_VCOLO_BLEED_EN_OVRDR {
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
#[doc = "Possible values of the field `BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_VCOLO_FASTCHARGE_EN_OVRD to override the signal \"bb_ldo_vcolo_fastcharge_en\"."]
    _1,
}
impl BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENR {
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
            BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENR::_0 => false,
            BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENR {
        match value {
            false => BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENR::_0,
            true => BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_VCOLO_FASTCHARGE_EN_OVRDR {
    bits: bool,
}
impl BB_LDO_VCOLO_FASTCHARGE_EN_OVRDR {
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
#[doc = "Possible values of the field `BB_XTAL_PLL_REF_CLK_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_XTAL_PLL_REF_CLK_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_XTAL_PLL_REF_CLK_EN_OVRD to override the signal \"bb_xtal_pll_ref_clk_en\"."]
    _1,
}
impl BB_XTAL_PLL_REF_CLK_EN_OVRD_ENR {
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
            BB_XTAL_PLL_REF_CLK_EN_OVRD_ENR::_0 => false,
            BB_XTAL_PLL_REF_CLK_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_XTAL_PLL_REF_CLK_EN_OVRD_ENR {
        match value {
            false => BB_XTAL_PLL_REF_CLK_EN_OVRD_ENR::_0,
            true => BB_XTAL_PLL_REF_CLK_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_XTAL_PLL_REF_CLK_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_XTAL_PLL_REF_CLK_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_XTAL_PLL_REF_CLK_EN_OVRDR {
    bits: bool,
}
impl BB_XTAL_PLL_REF_CLK_EN_OVRDR {
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
#[doc = "Possible values of the field `BB_XTAL_DAC_REF_CLK_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_XTAL_DAC_REF_CLK_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_XTAL_DAC_REF_CLK_EN_OVRD to override the signal \"bb_xtal_dac_ref_clk_en\"."]
    _1,
}
impl BB_XTAL_DAC_REF_CLK_EN_OVRD_ENR {
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
            BB_XTAL_DAC_REF_CLK_EN_OVRD_ENR::_0 => false,
            BB_XTAL_DAC_REF_CLK_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_XTAL_DAC_REF_CLK_EN_OVRD_ENR {
        match value {
            false => BB_XTAL_DAC_REF_CLK_EN_OVRD_ENR::_0,
            true => BB_XTAL_DAC_REF_CLK_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_XTAL_DAC_REF_CLK_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_XTAL_DAC_REF_CLK_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_XTAL_DAC_REF_CLK_EN_OVRDR {
    bits: bool,
}
impl BB_XTAL_DAC_REF_CLK_EN_OVRDR {
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
#[doc = "Possible values of the field `BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_XTAL_AUXPLL_REF_CLK_EN_OVRD to override the signal \"bb_xtal_auxpll_ref_clk_en\"."]
    _1,
}
impl BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENR {
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
            BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENR::_0 => false,
            BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENR {
        match value {
            false => BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENR::_0,
            true => BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BB_XTAL_AUXPLL_REF_CLK_EN_OVRDR {
    bits: bool,
}
impl BB_XTAL_AUXPLL_REF_CLK_EN_OVRDR {
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
#[doc = "Possible values of the field `SY_VCO_AUTOTUNE_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_VCO_AUTOTUNE_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_VCO_AUTOTUNE_EN_OVRD to override the signal \"sy_vco_autotune_en\"."]
    _1,
}
impl SY_VCO_AUTOTUNE_EN_OVRD_ENR {
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
            SY_VCO_AUTOTUNE_EN_OVRD_ENR::_0 => false,
            SY_VCO_AUTOTUNE_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_VCO_AUTOTUNE_EN_OVRD_ENR {
        match value {
            false => SY_VCO_AUTOTUNE_EN_OVRD_ENR::_0,
            true => SY_VCO_AUTOTUNE_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_VCO_AUTOTUNE_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_VCO_AUTOTUNE_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_VCO_AUTOTUNE_EN_OVRDR {
    bits: bool,
}
impl SY_VCO_AUTOTUNE_EN_OVRDR {
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
#[doc = "Possible values of the field `SY_PD_CYCLE_SLIP_LD_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_PD_CYCLE_SLIP_LD_EN_OVRD to override the signal \"sy_pd_cycle_slip_ld_en\"."]
    _1,
}
impl SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENR {
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
            SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENR::_0 => false,
            SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENR {
        match value {
            false => SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENR::_0,
            true => SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_PD_CYCLE_SLIP_LD_EN_OVRDR {
    bits: bool,
}
impl SY_PD_CYCLE_SLIP_LD_EN_OVRDR {
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
#[doc = "Possible values of the field `SY_VCO_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_VCO_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_VCO_EN_OVRD to override the signal \"sy_vco_en\"."]
    _1,
}
impl SY_VCO_EN_OVRD_ENR {
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
            SY_VCO_EN_OVRD_ENR::_0 => false,
            SY_VCO_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_VCO_EN_OVRD_ENR {
        match value {
            false => SY_VCO_EN_OVRD_ENR::_0,
            true => SY_VCO_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_VCO_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_VCO_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_VCO_EN_OVRDR {
    bits: bool,
}
impl SY_VCO_EN_OVRDR {
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
#[doc = "Values that can be written to the field `BB_LDO_HF_EN_OVRD_EN`"]
pub enum BB_LDO_HF_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_HF_EN_OVRD to override the signal \"bb_ldo_hf_en\"."]
    _1,
}
impl BB_LDO_HF_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_HF_EN_OVRD_ENW::_0 => false,
            BB_LDO_HF_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_HF_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_HF_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_HF_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_HF_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of BB_LDO_HF_EN_OVRD to override the signal \"bb_ldo_hf_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_HF_EN_OVRD_ENW::_1)
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
#[doc = r" Proxy"]
pub struct _BB_LDO_HF_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_HF_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `BB_LDO_ADCDAC_EN_OVRD_EN`"]
pub enum BB_LDO_ADCDAC_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_ADCDAC_EN_OVRD to override the signal \"bb_ldo_adcdac_en\"."]
    _1,
}
impl BB_LDO_ADCDAC_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_ADCDAC_EN_OVRD_ENW::_0 => false,
            BB_LDO_ADCDAC_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_ADCDAC_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_ADCDAC_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_ADCDAC_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of BB_LDO_ADCDAC_EN_OVRD to override the signal \"bb_ldo_adcdac_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_ADCDAC_EN_OVRD_ENW::_1)
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
pub struct _BB_LDO_ADCDAC_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_ADCDAC_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `BB_LDO_BBA_EN_OVRD_EN`"]
pub enum BB_LDO_BBA_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_BBA_EN_OVRD to override the signal \"bb_ldo_bba_en\"."]
    _1,
}
impl BB_LDO_BBA_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_BBA_EN_OVRD_ENW::_0 => false,
            BB_LDO_BBA_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_BBA_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_BBA_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_BBA_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of BB_LDO_BBA_EN_OVRD to override the signal \"bb_ldo_bba_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_BBA_EN_OVRD_ENW::_1)
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
#[doc = r" Proxy"]
pub struct _BB_LDO_BBA_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_BBA_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `BB_LDO_PD_EN_OVRD_EN`"]
pub enum BB_LDO_PD_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_PD_EN_OVRD to override the signal \"bb_ldo_pd_en\"."]
    _1,
}
impl BB_LDO_PD_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_PD_EN_OVRD_ENW::_0 => false,
            BB_LDO_PD_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_PD_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_PD_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_PD_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_PD_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of BB_LDO_PD_EN_OVRD to override the signal \"bb_ldo_pd_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_PD_EN_OVRD_ENW::_1)
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
#[doc = r" Proxy"]
pub struct _BB_LDO_PD_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_PD_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `BB_LDO_FDBK_EN_OVRD_EN`"]
pub enum BB_LDO_FDBK_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_FDBK_EN_OVRD to override the signal \"bb_ldo_fdbk_en\"."]
    _1,
}
impl BB_LDO_FDBK_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_FDBK_EN_OVRD_ENW::_0 => false,
            BB_LDO_FDBK_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_FDBK_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_FDBK_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_FDBK_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of BB_LDO_FDBK_EN_OVRD to override the signal \"bb_ldo_fdbk_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_EN_OVRD_ENW::_1)
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
#[doc = r" Proxy"]
pub struct _BB_LDO_FDBK_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_FDBK_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `BB_LDO_VCOLO_EN_OVRD_EN`"]
pub enum BB_LDO_VCOLO_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_VCOLO_EN_OVRD to override the signal \"bb_ldo_vcolo_en\"."]
    _1,
}
impl BB_LDO_VCOLO_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_VCOLO_EN_OVRD_ENW::_0 => false,
            BB_LDO_VCOLO_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_VCOLO_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VCOLO_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_VCOLO_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of BB_LDO_VCOLO_EN_OVRD to override the signal \"bb_ldo_vcolo_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_EN_OVRD_ENW::_1)
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
pub struct _BB_LDO_VCOLO_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VCOLO_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `BB_LDO_VTREF_EN_OVRD_EN`"]
pub enum BB_LDO_VTREF_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_VTREF_EN_OVRD to override the signal \"bb_ldo_vtref_en\"."]
    _1,
}
impl BB_LDO_VTREF_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_VTREF_EN_OVRD_ENW::_0 => false,
            BB_LDO_VTREF_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_VTREF_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VTREF_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_VTREF_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_VTREF_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of BB_LDO_VTREF_EN_OVRD to override the signal \"bb_ldo_vtref_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_VTREF_EN_OVRD_ENW::_1)
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
pub struct _BB_LDO_VTREF_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VTREF_EN_OVRDW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB_LDO_FDBK_BLEED_EN_OVRD_EN`"]
pub enum BB_LDO_FDBK_BLEED_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_FDBK_BLEED_EN_OVRD to override the signal \"bb_ldo_fdbk_bleed_en\"."]
    _1,
}
impl BB_LDO_FDBK_BLEED_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_FDBK_BLEED_EN_OVRD_ENW::_0 => false,
            BB_LDO_FDBK_BLEED_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_FDBK_BLEED_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_FDBK_BLEED_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_FDBK_BLEED_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_BLEED_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of BB_LDO_FDBK_BLEED_EN_OVRD to override the signal \"bb_ldo_fdbk_bleed_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_FDBK_BLEED_EN_OVRD_ENW::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_FDBK_BLEED_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_FDBK_BLEED_EN_OVRDW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB_LDO_VCOLO_BLEED_EN_OVRD_EN`"]
pub enum BB_LDO_VCOLO_BLEED_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_VCOLO_BLEED_EN_OVRD to override the signal \"bb_ldo_vcolo_bleed_en\"."]
    _1,
}
impl BB_LDO_VCOLO_BLEED_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_VCOLO_BLEED_EN_OVRD_ENW::_0 => false,
            BB_LDO_VCOLO_BLEED_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_VCOLO_BLEED_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VCOLO_BLEED_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_VCOLO_BLEED_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_BLEED_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of BB_LDO_VCOLO_BLEED_EN_OVRD to override the signal \"bb_ldo_vcolo_bleed_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_BLEED_EN_OVRD_ENW::_1)
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
#[doc = r" Proxy"]
pub struct _BB_LDO_VCOLO_BLEED_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VCOLO_BLEED_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_EN`"]
pub enum BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_LDO_VCOLO_FASTCHARGE_EN_OVRD to override the signal \"bb_ldo_vcolo_fastcharge_en\"."]
    _1,
}
impl BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENW::_0 => false,
            BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of BB_LDO_VCOLO_FASTCHARGE_EN_OVRD to override the signal \"bb_ldo_vcolo_fastcharge_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENW::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_VCOLO_FASTCHARGE_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_VCOLO_FASTCHARGE_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `BB_XTAL_PLL_REF_CLK_EN_OVRD_EN`"]
pub enum BB_XTAL_PLL_REF_CLK_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_XTAL_PLL_REF_CLK_EN_OVRD to override the signal \"bb_xtal_pll_ref_clk_en\"."]
    _1,
}
impl BB_XTAL_PLL_REF_CLK_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_XTAL_PLL_REF_CLK_EN_OVRD_ENW::_0 => false,
            BB_XTAL_PLL_REF_CLK_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_XTAL_PLL_REF_CLK_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_PLL_REF_CLK_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_XTAL_PLL_REF_CLK_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_XTAL_PLL_REF_CLK_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of BB_XTAL_PLL_REF_CLK_EN_OVRD to override the signal \"bb_xtal_pll_ref_clk_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_XTAL_PLL_REF_CLK_EN_OVRD_ENW::_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_XTAL_PLL_REF_CLK_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_PLL_REF_CLK_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `BB_XTAL_DAC_REF_CLK_EN_OVRD_EN`"]
pub enum BB_XTAL_DAC_REF_CLK_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_XTAL_DAC_REF_CLK_EN_OVRD to override the signal \"bb_xtal_dac_ref_clk_en\"."]
    _1,
}
impl BB_XTAL_DAC_REF_CLK_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_XTAL_DAC_REF_CLK_EN_OVRD_ENW::_0 => false,
            BB_XTAL_DAC_REF_CLK_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_XTAL_DAC_REF_CLK_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_DAC_REF_CLK_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_XTAL_DAC_REF_CLK_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_XTAL_DAC_REF_CLK_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of BB_XTAL_DAC_REF_CLK_EN_OVRD to override the signal \"bb_xtal_dac_ref_clk_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_XTAL_DAC_REF_CLK_EN_OVRD_ENW::_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_XTAL_DAC_REF_CLK_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_DAC_REF_CLK_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_EN`"]
pub enum BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of BB_XTAL_AUXPLL_REF_CLK_EN_OVRD to override the signal \"bb_xtal_auxpll_ref_clk_en\"."]
    _1,
}
impl BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENW::_0 => false,
            BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of BB_XTAL_AUXPLL_REF_CLK_EN_OVRD to override the signal \"bb_xtal_auxpll_ref_clk_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENW::_1)
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
#[doc = r" Proxy"]
pub struct _BB_XTAL_AUXPLL_REF_CLK_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_AUXPLL_REF_CLK_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `SY_VCO_AUTOTUNE_EN_OVRD_EN`"]
pub enum SY_VCO_AUTOTUNE_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_VCO_AUTOTUNE_EN_OVRD to override the signal \"sy_vco_autotune_en\"."]
    _1,
}
impl SY_VCO_AUTOTUNE_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_VCO_AUTOTUNE_EN_OVRD_ENW::_0 => false,
            SY_VCO_AUTOTUNE_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_VCO_AUTOTUNE_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_VCO_AUTOTUNE_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_VCO_AUTOTUNE_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_VCO_AUTOTUNE_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of SY_VCO_AUTOTUNE_EN_OVRD to override the signal \"sy_vco_autotune_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_VCO_AUTOTUNE_EN_OVRD_ENW::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SY_VCO_AUTOTUNE_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_VCO_AUTOTUNE_EN_OVRDW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SY_PD_CYCLE_SLIP_LD_EN_OVRD_EN`"]
pub enum SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_PD_CYCLE_SLIP_LD_EN_OVRD to override the signal \"sy_pd_cycle_slip_ld_en\"."]
    _1,
}
impl SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENW::_0 => false,
            SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of SY_PD_CYCLE_SLIP_LD_EN_OVRD to override the signal \"sy_pd_cycle_slip_ld_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENW::_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SY_PD_CYCLE_SLIP_LD_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_PD_CYCLE_SLIP_LD_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `SY_VCO_EN_OVRD_EN`"]
pub enum SY_VCO_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_VCO_EN_OVRD to override the signal \"sy_vco_en\"."]
    _1,
}
impl SY_VCO_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_VCO_EN_OVRD_ENW::_0 => false,
            SY_VCO_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_VCO_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_VCO_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_VCO_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_VCO_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of SY_VCO_EN_OVRD to override the signal \"sy_vco_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_VCO_EN_OVRD_ENW::_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SY_VCO_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_VCO_EN_OVRDW<'a> {
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
    #[doc = "Bit 0 - Override control for BB_LDO_HF_EN"]
    #[inline]
    pub fn bb_ldo_hf_en_ovrd_en(&self) -> BB_LDO_HF_EN_OVRD_ENR {
        BB_LDO_HF_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Override value for BB_LDO_HF_EN"]
    #[inline]
    pub fn bb_ldo_hf_en_ovrd(&self) -> BB_LDO_HF_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_HF_EN_OVRDR { bits }
    }
    #[doc = "Bit 2 - Override control for BB_LDO_ADCDAC_EN"]
    #[inline]
    pub fn bb_ldo_adcdac_en_ovrd_en(&self) -> BB_LDO_ADCDAC_EN_OVRD_ENR {
        BB_LDO_ADCDAC_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Override value for BB_LDO_ADCDAC_EN"]
    #[inline]
    pub fn bb_ldo_adcdac_en_ovrd(&self) -> BB_LDO_ADCDAC_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_ADCDAC_EN_OVRDR { bits }
    }
    #[doc = "Bit 4 - Override control for BB_LDO_BBA_EN"]
    #[inline]
    pub fn bb_ldo_bba_en_ovrd_en(&self) -> BB_LDO_BBA_EN_OVRD_ENR {
        BB_LDO_BBA_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Override value for BB_LDO_BBA_EN"]
    #[inline]
    pub fn bb_ldo_bba_en_ovrd(&self) -> BB_LDO_BBA_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_BBA_EN_OVRDR { bits }
    }
    #[doc = "Bit 6 - Override control for BB_LDO_PD_EN"]
    #[inline]
    pub fn bb_ldo_pd_en_ovrd_en(&self) -> BB_LDO_PD_EN_OVRD_ENR {
        BB_LDO_PD_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Override value for BB_LDO_PD_EN"]
    #[inline]
    pub fn bb_ldo_pd_en_ovrd(&self) -> BB_LDO_PD_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_PD_EN_OVRDR { bits }
    }
    #[doc = "Bit 8 - Override control for BB_LDO_FDBK_EN"]
    #[inline]
    pub fn bb_ldo_fdbk_en_ovrd_en(&self) -> BB_LDO_FDBK_EN_OVRD_ENR {
        BB_LDO_FDBK_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Override value for BB_LDO_FDBK_EN"]
    #[inline]
    pub fn bb_ldo_fdbk_en_ovrd(&self) -> BB_LDO_FDBK_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_FDBK_EN_OVRDR { bits }
    }
    #[doc = "Bit 10 - Override control for BB_LDO_VCOLO_EN"]
    #[inline]
    pub fn bb_ldo_vcolo_en_ovrd_en(&self) -> BB_LDO_VCOLO_EN_OVRD_ENR {
        BB_LDO_VCOLO_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Override value for BB_LDO_VCOLO_EN"]
    #[inline]
    pub fn bb_ldo_vcolo_en_ovrd(&self) -> BB_LDO_VCOLO_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_VCOLO_EN_OVRDR { bits }
    }
    #[doc = "Bit 12 - Override control for BB_LDO_VTREF_EN"]
    #[inline]
    pub fn bb_ldo_vtref_en_ovrd_en(&self) -> BB_LDO_VTREF_EN_OVRD_ENR {
        BB_LDO_VTREF_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Override value for BB_LDO_VTREF_EN"]
    #[inline]
    pub fn bb_ldo_vtref_en_ovrd(&self) -> BB_LDO_VTREF_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_VTREF_EN_OVRDR { bits }
    }
    #[doc = "Bit 14 - Override control for BB_LDO_FDBK_BLEED_EN"]
    #[inline]
    pub fn bb_ldo_fdbk_bleed_en_ovrd_en(&self) -> BB_LDO_FDBK_BLEED_EN_OVRD_ENR {
        BB_LDO_FDBK_BLEED_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Override value for BB_LDO_FDBK_BLEED_EN"]
    #[inline]
    pub fn bb_ldo_fdbk_bleed_en_ovrd(&self) -> BB_LDO_FDBK_BLEED_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_FDBK_BLEED_EN_OVRDR { bits }
    }
    #[doc = "Bit 16 - Override control for BB_LDO_VCOLO_BLEED_EN"]
    #[inline]
    pub fn bb_ldo_vcolo_bleed_en_ovrd_en(&self) -> BB_LDO_VCOLO_BLEED_EN_OVRD_ENR {
        BB_LDO_VCOLO_BLEED_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Override value for BB_LDO_VCOLO_BLEED_EN"]
    #[inline]
    pub fn bb_ldo_vcolo_bleed_en_ovrd(&self) -> BB_LDO_VCOLO_BLEED_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_VCOLO_BLEED_EN_OVRDR { bits }
    }
    #[doc = "Bit 18 - Override control for BB_LDO_VCOLO_FASTCHARGE_EN"]
    #[inline]
    pub fn bb_ldo_vcolo_fastcharge_en_ovrd_en(&self) -> BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENR {
        BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Override value for BB_LDO_VCOLO_FASTCHARGE_EN"]
    #[inline]
    pub fn bb_ldo_vcolo_fastcharge_en_ovrd(&self) -> BB_LDO_VCOLO_FASTCHARGE_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_VCOLO_FASTCHARGE_EN_OVRDR { bits }
    }
    #[doc = "Bit 20 - Override control for BB_XTAL_PLL_REF_CLK_EN"]
    #[inline]
    pub fn bb_xtal_pll_ref_clk_en_ovrd_en(&self) -> BB_XTAL_PLL_REF_CLK_EN_OVRD_ENR {
        BB_XTAL_PLL_REF_CLK_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Override value for BB_XTAL_PLL_REF_CLK_EN"]
    #[inline]
    pub fn bb_xtal_pll_ref_clk_en_ovrd(&self) -> BB_XTAL_PLL_REF_CLK_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_XTAL_PLL_REF_CLK_EN_OVRDR { bits }
    }
    #[doc = "Bit 22 - Override control for BB_XTAL_DAC_REF_CLK_EN"]
    #[inline]
    pub fn bb_xtal_dac_ref_clk_en_ovrd_en(&self) -> BB_XTAL_DAC_REF_CLK_EN_OVRD_ENR {
        BB_XTAL_DAC_REF_CLK_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Override value for BB_XTAL_DAC_REF_CLK_EN"]
    #[inline]
    pub fn bb_xtal_dac_ref_clk_en_ovrd(&self) -> BB_XTAL_DAC_REF_CLK_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_XTAL_DAC_REF_CLK_EN_OVRDR { bits }
    }
    #[doc = "Bit 24 - Override control for BB_XTAL_AUXPLL_REF_CLK_EN"]
    #[inline]
    pub fn bb_xtal_auxpll_ref_clk_en_ovrd_en(&self) -> BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENR {
        BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Override value for BB_XTAL_AUXPLL_REF_CLK_EN"]
    #[inline]
    pub fn bb_xtal_auxpll_ref_clk_en_ovrd(&self) -> BB_XTAL_AUXPLL_REF_CLK_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_XTAL_AUXPLL_REF_CLK_EN_OVRDR { bits }
    }
    #[doc = "Bit 26 - Override control for SY_VCO_AUTOTUNE_EN"]
    #[inline]
    pub fn sy_vco_autotune_en_ovrd_en(&self) -> SY_VCO_AUTOTUNE_EN_OVRD_ENR {
        SY_VCO_AUTOTUNE_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Override value for SY_VCO_AUTOTUNE_EN"]
    #[inline]
    pub fn sy_vco_autotune_en_ovrd(&self) -> SY_VCO_AUTOTUNE_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_VCO_AUTOTUNE_EN_OVRDR { bits }
    }
    #[doc = "Bit 28 - Override control for SY_PD_CYCLE_SLIP_LD_EN"]
    #[inline]
    pub fn sy_pd_cycle_slip_ld_en_ovrd_en(&self) -> SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENR {
        SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Override value for SY_PD_CYCLE_SLIP_LD_EN"]
    #[inline]
    pub fn sy_pd_cycle_slip_ld_en_ovrd(&self) -> SY_PD_CYCLE_SLIP_LD_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_PD_CYCLE_SLIP_LD_EN_OVRDR { bits }
    }
    #[doc = "Bit 30 - Override control for SY_VCO_EN"]
    #[inline]
    pub fn sy_vco_en_ovrd_en(&self) -> SY_VCO_EN_OVRD_ENR {
        SY_VCO_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Override value for SY_VCO_EN"]
    #[inline]
    pub fn sy_vco_en_ovrd(&self) -> SY_VCO_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_VCO_EN_OVRDR { bits }
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
    #[doc = "Bit 0 - Override control for BB_LDO_HF_EN"]
    #[inline]
    pub fn bb_ldo_hf_en_ovrd_en(&mut self) -> _BB_LDO_HF_EN_OVRD_ENW {
        _BB_LDO_HF_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 1 - Override value for BB_LDO_HF_EN"]
    #[inline]
    pub fn bb_ldo_hf_en_ovrd(&mut self) -> _BB_LDO_HF_EN_OVRDW {
        _BB_LDO_HF_EN_OVRDW { w: self }
    }
    #[doc = "Bit 2 - Override control for BB_LDO_ADCDAC_EN"]
    #[inline]
    pub fn bb_ldo_adcdac_en_ovrd_en(&mut self) -> _BB_LDO_ADCDAC_EN_OVRD_ENW {
        _BB_LDO_ADCDAC_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 3 - Override value for BB_LDO_ADCDAC_EN"]
    #[inline]
    pub fn bb_ldo_adcdac_en_ovrd(&mut self) -> _BB_LDO_ADCDAC_EN_OVRDW {
        _BB_LDO_ADCDAC_EN_OVRDW { w: self }
    }
    #[doc = "Bit 4 - Override control for BB_LDO_BBA_EN"]
    #[inline]
    pub fn bb_ldo_bba_en_ovrd_en(&mut self) -> _BB_LDO_BBA_EN_OVRD_ENW {
        _BB_LDO_BBA_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 5 - Override value for BB_LDO_BBA_EN"]
    #[inline]
    pub fn bb_ldo_bba_en_ovrd(&mut self) -> _BB_LDO_BBA_EN_OVRDW {
        _BB_LDO_BBA_EN_OVRDW { w: self }
    }
    #[doc = "Bit 6 - Override control for BB_LDO_PD_EN"]
    #[inline]
    pub fn bb_ldo_pd_en_ovrd_en(&mut self) -> _BB_LDO_PD_EN_OVRD_ENW {
        _BB_LDO_PD_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 7 - Override value for BB_LDO_PD_EN"]
    #[inline]
    pub fn bb_ldo_pd_en_ovrd(&mut self) -> _BB_LDO_PD_EN_OVRDW {
        _BB_LDO_PD_EN_OVRDW { w: self }
    }
    #[doc = "Bit 8 - Override control for BB_LDO_FDBK_EN"]
    #[inline]
    pub fn bb_ldo_fdbk_en_ovrd_en(&mut self) -> _BB_LDO_FDBK_EN_OVRD_ENW {
        _BB_LDO_FDBK_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 9 - Override value for BB_LDO_FDBK_EN"]
    #[inline]
    pub fn bb_ldo_fdbk_en_ovrd(&mut self) -> _BB_LDO_FDBK_EN_OVRDW {
        _BB_LDO_FDBK_EN_OVRDW { w: self }
    }
    #[doc = "Bit 10 - Override control for BB_LDO_VCOLO_EN"]
    #[inline]
    pub fn bb_ldo_vcolo_en_ovrd_en(&mut self) -> _BB_LDO_VCOLO_EN_OVRD_ENW {
        _BB_LDO_VCOLO_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 11 - Override value for BB_LDO_VCOLO_EN"]
    #[inline]
    pub fn bb_ldo_vcolo_en_ovrd(&mut self) -> _BB_LDO_VCOLO_EN_OVRDW {
        _BB_LDO_VCOLO_EN_OVRDW { w: self }
    }
    #[doc = "Bit 12 - Override control for BB_LDO_VTREF_EN"]
    #[inline]
    pub fn bb_ldo_vtref_en_ovrd_en(&mut self) -> _BB_LDO_VTREF_EN_OVRD_ENW {
        _BB_LDO_VTREF_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 13 - Override value for BB_LDO_VTREF_EN"]
    #[inline]
    pub fn bb_ldo_vtref_en_ovrd(&mut self) -> _BB_LDO_VTREF_EN_OVRDW {
        _BB_LDO_VTREF_EN_OVRDW { w: self }
    }
    #[doc = "Bit 14 - Override control for BB_LDO_FDBK_BLEED_EN"]
    #[inline]
    pub fn bb_ldo_fdbk_bleed_en_ovrd_en(&mut self) -> _BB_LDO_FDBK_BLEED_EN_OVRD_ENW {
        _BB_LDO_FDBK_BLEED_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 15 - Override value for BB_LDO_FDBK_BLEED_EN"]
    #[inline]
    pub fn bb_ldo_fdbk_bleed_en_ovrd(&mut self) -> _BB_LDO_FDBK_BLEED_EN_OVRDW {
        _BB_LDO_FDBK_BLEED_EN_OVRDW { w: self }
    }
    #[doc = "Bit 16 - Override control for BB_LDO_VCOLO_BLEED_EN"]
    #[inline]
    pub fn bb_ldo_vcolo_bleed_en_ovrd_en(&mut self) -> _BB_LDO_VCOLO_BLEED_EN_OVRD_ENW {
        _BB_LDO_VCOLO_BLEED_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 17 - Override value for BB_LDO_VCOLO_BLEED_EN"]
    #[inline]
    pub fn bb_ldo_vcolo_bleed_en_ovrd(&mut self) -> _BB_LDO_VCOLO_BLEED_EN_OVRDW {
        _BB_LDO_VCOLO_BLEED_EN_OVRDW { w: self }
    }
    #[doc = "Bit 18 - Override control for BB_LDO_VCOLO_FASTCHARGE_EN"]
    #[inline]
    pub fn bb_ldo_vcolo_fastcharge_en_ovrd_en(&mut self) -> _BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENW {
        _BB_LDO_VCOLO_FASTCHARGE_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 19 - Override value for BB_LDO_VCOLO_FASTCHARGE_EN"]
    #[inline]
    pub fn bb_ldo_vcolo_fastcharge_en_ovrd(&mut self) -> _BB_LDO_VCOLO_FASTCHARGE_EN_OVRDW {
        _BB_LDO_VCOLO_FASTCHARGE_EN_OVRDW { w: self }
    }
    #[doc = "Bit 20 - Override control for BB_XTAL_PLL_REF_CLK_EN"]
    #[inline]
    pub fn bb_xtal_pll_ref_clk_en_ovrd_en(&mut self) -> _BB_XTAL_PLL_REF_CLK_EN_OVRD_ENW {
        _BB_XTAL_PLL_REF_CLK_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 21 - Override value for BB_XTAL_PLL_REF_CLK_EN"]
    #[inline]
    pub fn bb_xtal_pll_ref_clk_en_ovrd(&mut self) -> _BB_XTAL_PLL_REF_CLK_EN_OVRDW {
        _BB_XTAL_PLL_REF_CLK_EN_OVRDW { w: self }
    }
    #[doc = "Bit 22 - Override control for BB_XTAL_DAC_REF_CLK_EN"]
    #[inline]
    pub fn bb_xtal_dac_ref_clk_en_ovrd_en(&mut self) -> _BB_XTAL_DAC_REF_CLK_EN_OVRD_ENW {
        _BB_XTAL_DAC_REF_CLK_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 23 - Override value for BB_XTAL_DAC_REF_CLK_EN"]
    #[inline]
    pub fn bb_xtal_dac_ref_clk_en_ovrd(&mut self) -> _BB_XTAL_DAC_REF_CLK_EN_OVRDW {
        _BB_XTAL_DAC_REF_CLK_EN_OVRDW { w: self }
    }
    #[doc = "Bit 24 - Override control for BB_XTAL_AUXPLL_REF_CLK_EN"]
    #[inline]
    pub fn bb_xtal_auxpll_ref_clk_en_ovrd_en(&mut self) -> _BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENW {
        _BB_XTAL_AUXPLL_REF_CLK_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 25 - Override value for BB_XTAL_AUXPLL_REF_CLK_EN"]
    #[inline]
    pub fn bb_xtal_auxpll_ref_clk_en_ovrd(&mut self) -> _BB_XTAL_AUXPLL_REF_CLK_EN_OVRDW {
        _BB_XTAL_AUXPLL_REF_CLK_EN_OVRDW { w: self }
    }
    #[doc = "Bit 26 - Override control for SY_VCO_AUTOTUNE_EN"]
    #[inline]
    pub fn sy_vco_autotune_en_ovrd_en(&mut self) -> _SY_VCO_AUTOTUNE_EN_OVRD_ENW {
        _SY_VCO_AUTOTUNE_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 27 - Override value for SY_VCO_AUTOTUNE_EN"]
    #[inline]
    pub fn sy_vco_autotune_en_ovrd(&mut self) -> _SY_VCO_AUTOTUNE_EN_OVRDW {
        _SY_VCO_AUTOTUNE_EN_OVRDW { w: self }
    }
    #[doc = "Bit 28 - Override control for SY_PD_CYCLE_SLIP_LD_EN"]
    #[inline]
    pub fn sy_pd_cycle_slip_ld_en_ovrd_en(&mut self) -> _SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENW {
        _SY_PD_CYCLE_SLIP_LD_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 29 - Override value for SY_PD_CYCLE_SLIP_LD_EN"]
    #[inline]
    pub fn sy_pd_cycle_slip_ld_en_ovrd(&mut self) -> _SY_PD_CYCLE_SLIP_LD_EN_OVRDW {
        _SY_PD_CYCLE_SLIP_LD_EN_OVRDW { w: self }
    }
    #[doc = "Bit 30 - Override control for SY_VCO_EN"]
    #[inline]
    pub fn sy_vco_en_ovrd_en(&mut self) -> _SY_VCO_EN_OVRD_ENW {
        _SY_VCO_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 31 - Override value for SY_VCO_EN"]
    #[inline]
    pub fn sy_vco_en_ovrd(&mut self) -> _SY_VCO_EN_OVRDW {
        _SY_VCO_EN_OVRDW { w: self }
    }
}
