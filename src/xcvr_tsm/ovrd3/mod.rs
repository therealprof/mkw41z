#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OVRD3 {
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
#[doc = "Possible values of the field `TSM_SPARE0_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_SPARE0_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TSM_SPARE0_EN_OVRD to override the signal \"tsm_spare0_en\"."]
    _1,
}
impl TSM_SPARE0_EN_OVRD_ENR {
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
            TSM_SPARE0_EN_OVRD_ENR::_0 => false,
            TSM_SPARE0_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSM_SPARE0_EN_OVRD_ENR {
        match value {
            false => TSM_SPARE0_EN_OVRD_ENR::_0,
            true => TSM_SPARE0_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSM_SPARE0_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSM_SPARE0_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TSM_SPARE0_EN_OVRDR {
    bits: bool,
}
impl TSM_SPARE0_EN_OVRDR {
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
#[doc = "Possible values of the field `TSM_SPARE1_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_SPARE1_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TSM_SPARE1_EN_OVRD to override the signal \"tsm_spare1_en\"."]
    _1,
}
impl TSM_SPARE1_EN_OVRD_ENR {
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
            TSM_SPARE1_EN_OVRD_ENR::_0 => false,
            TSM_SPARE1_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSM_SPARE1_EN_OVRD_ENR {
        match value {
            false => TSM_SPARE1_EN_OVRD_ENR::_0,
            true => TSM_SPARE1_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSM_SPARE1_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSM_SPARE1_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TSM_SPARE1_EN_OVRDR {
    bits: bool,
}
impl TSM_SPARE1_EN_OVRDR {
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
#[doc = "Possible values of the field `TSM_SPARE2_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_SPARE2_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TSM_SPARE2_EN_OVRD to override the signal \"tsm_spare2_en\"."]
    _1,
}
impl TSM_SPARE2_EN_OVRD_ENR {
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
            TSM_SPARE2_EN_OVRD_ENR::_0 => false,
            TSM_SPARE2_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSM_SPARE2_EN_OVRD_ENR {
        match value {
            false => TSM_SPARE2_EN_OVRD_ENR::_0,
            true => TSM_SPARE2_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSM_SPARE2_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSM_SPARE2_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TSM_SPARE2_EN_OVRDR {
    bits: bool,
}
impl TSM_SPARE2_EN_OVRDR {
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
#[doc = "Possible values of the field `TSM_SPARE3_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_SPARE3_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TSM_SPARE3_EN_OVRD to override the signal \"tsm_spare3_en\"."]
    _1,
}
impl TSM_SPARE3_EN_OVRD_ENR {
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
            TSM_SPARE3_EN_OVRD_ENR::_0 => false,
            TSM_SPARE3_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSM_SPARE3_EN_OVRD_ENR {
        match value {
            false => TSM_SPARE3_EN_OVRD_ENR::_0,
            true => TSM_SPARE3_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSM_SPARE3_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSM_SPARE3_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TSM_SPARE3_EN_OVRDR {
    bits: bool,
}
impl TSM_SPARE3_EN_OVRDR {
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
#[doc = "Possible values of the field `RXTX_AUXPLL_BIAS_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTX_AUXPLL_BIAS_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_BIAS_EN_OVRD to override the signal \"rxtx_auxpll_bias_en\"."]
    _1,
}
impl RXTX_AUXPLL_BIAS_EN_OVRD_ENR {
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
            RXTX_AUXPLL_BIAS_EN_OVRD_ENR::_0 => false,
            RXTX_AUXPLL_BIAS_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTX_AUXPLL_BIAS_EN_OVRD_ENR {
        match value {
            false => RXTX_AUXPLL_BIAS_EN_OVRD_ENR::_0,
            true => RXTX_AUXPLL_BIAS_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXTX_AUXPLL_BIAS_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXTX_AUXPLL_BIAS_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RXTX_AUXPLL_BIAS_EN_OVRDR {
    bits: bool,
}
impl RXTX_AUXPLL_BIAS_EN_OVRDR {
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
#[doc = "Possible values of the field `RXTX_AUXPLL_VCO_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTX_AUXPLL_VCO_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_VCO_EN_OVRD to override the signal \"rxtx_auxpll_vco_en\"."]
    _1,
}
impl RXTX_AUXPLL_VCO_EN_OVRD_ENR {
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
            RXTX_AUXPLL_VCO_EN_OVRD_ENR::_0 => false,
            RXTX_AUXPLL_VCO_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTX_AUXPLL_VCO_EN_OVRD_ENR {
        match value {
            false => RXTX_AUXPLL_VCO_EN_OVRD_ENR::_0,
            true => RXTX_AUXPLL_VCO_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXTX_AUXPLL_VCO_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXTX_AUXPLL_VCO_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RXTX_AUXPLL_VCO_EN_OVRDR {
    bits: bool,
}
impl RXTX_AUXPLL_VCO_EN_OVRDR {
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
#[doc = "Possible values of the field `RXTX_AUXPLL_FCAL_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTX_AUXPLL_FCAL_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_FCAL_EN_OVRD to override the signal \"rxtx_auxpll_fcal_en\"."]
    _1,
}
impl RXTX_AUXPLL_FCAL_EN_OVRD_ENR {
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
            RXTX_AUXPLL_FCAL_EN_OVRD_ENR::_0 => false,
            RXTX_AUXPLL_FCAL_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTX_AUXPLL_FCAL_EN_OVRD_ENR {
        match value {
            false => RXTX_AUXPLL_FCAL_EN_OVRD_ENR::_0,
            true => RXTX_AUXPLL_FCAL_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXTX_AUXPLL_FCAL_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXTX_AUXPLL_FCAL_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RXTX_AUXPLL_FCAL_EN_OVRDR {
    bits: bool,
}
impl RXTX_AUXPLL_FCAL_EN_OVRDR {
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
#[doc = "Possible values of the field `RXTX_AUXPLL_LF_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTX_AUXPLL_LF_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_LF_EN_OVRD to override the signal \"rxtx_auxpll_lf_en\"."]
    _1,
}
impl RXTX_AUXPLL_LF_EN_OVRD_ENR {
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
            RXTX_AUXPLL_LF_EN_OVRD_ENR::_0 => false,
            RXTX_AUXPLL_LF_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTX_AUXPLL_LF_EN_OVRD_ENR {
        match value {
            false => RXTX_AUXPLL_LF_EN_OVRD_ENR::_0,
            true => RXTX_AUXPLL_LF_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXTX_AUXPLL_LF_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXTX_AUXPLL_LF_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RXTX_AUXPLL_LF_EN_OVRDR {
    bits: bool,
}
impl RXTX_AUXPLL_LF_EN_OVRDR {
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
#[doc = "Possible values of the field `RXTX_AUXPLL_PD_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTX_AUXPLL_PD_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_PD_EN_OVRD to override the signal \"rxtx_auxpll_pd_en\"."]
    _1,
}
impl RXTX_AUXPLL_PD_EN_OVRD_ENR {
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
            RXTX_AUXPLL_PD_EN_OVRD_ENR::_0 => false,
            RXTX_AUXPLL_PD_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTX_AUXPLL_PD_EN_OVRD_ENR {
        match value {
            false => RXTX_AUXPLL_PD_EN_OVRD_ENR::_0,
            true => RXTX_AUXPLL_PD_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXTX_AUXPLL_PD_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXTX_AUXPLL_PD_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RXTX_AUXPLL_PD_EN_OVRDR {
    bits: bool,
}
impl RXTX_AUXPLL_PD_EN_OVRDR {
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
#[doc = "Possible values of the field `RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD to override the signal \"rxtx_auxpll_pd_lf_filter_charge_en\"."]
    _1,
}
impl RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENR {
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
            RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENR::_0 => false,
            RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENR {
        match value {
            false => RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENR::_0,
            true => RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRDR {
    bits: bool,
}
impl RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRDR {
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
#[doc = "Possible values of the field `RXTX_AUXPLL_ADC_BUF_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_ADC_BUF_EN_OVRD to override the signal \"rxtx_auxpll_adc_buf_en\"."]
    _1,
}
impl RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENR {
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
            RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENR::_0 => false,
            RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENR {
        match value {
            false => RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENR::_0,
            true => RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RXTX_AUXPLL_ADC_BUF_EN_OVRDR {
    bits: bool,
}
impl RXTX_AUXPLL_ADC_BUF_EN_OVRDR {
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
#[doc = "Possible values of the field `RXTX_AUXPLL_DIG_BUF_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_DIG_BUF_EN_OVRD to override the signal \"rxtx_auxpll_dig_buf_en\"."]
    _1,
}
impl RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENR {
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
            RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENR::_0 => false,
            RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENR {
        match value {
            false => RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENR::_0,
            true => RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RXTX_AUXPLL_DIG_BUF_EN_OVRDR {
    bits: bool,
}
impl RXTX_AUXPLL_DIG_BUF_EN_OVRDR {
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
#[doc = "Possible values of the field `RXTX_RCCAL_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTX_RCCAL_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_RCCAL_EN_OVRD to override the signal \"rxtx_rccal_en\"."]
    _1,
}
impl RXTX_RCCAL_EN_OVRD_ENR {
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
            RXTX_RCCAL_EN_OVRD_ENR::_0 => false,
            RXTX_RCCAL_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTX_RCCAL_EN_OVRD_ENR {
        match value {
            false => RXTX_RCCAL_EN_OVRD_ENR::_0,
            true => RXTX_RCCAL_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXTX_RCCAL_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXTX_RCCAL_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RXTX_RCCAL_EN_OVRDR {
    bits: bool,
}
impl RXTX_RCCAL_EN_OVRDR {
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
#[doc = "Possible values of the field `TX_HPM_DAC_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_HPM_DAC_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TX_HPM_DAC_EN_OVRD to override the signal \"tx_hpm_dac_en\"."]
    _1,
}
impl TX_HPM_DAC_EN_OVRD_ENR {
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
            TX_HPM_DAC_EN_OVRD_ENR::_0 => false,
            TX_HPM_DAC_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_HPM_DAC_EN_OVRD_ENR {
        match value {
            false => TX_HPM_DAC_EN_OVRD_ENR::_0,
            true => TX_HPM_DAC_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_HPM_DAC_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_HPM_DAC_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TX_HPM_DAC_EN_OVRDR {
    bits: bool,
}
impl TX_HPM_DAC_EN_OVRDR {
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
#[doc = "Possible values of the field `TX_MODE_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_MODE_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TX_MODE_OVRD to override the signal \"tx_mode\"."]
    _1,
}
impl TX_MODE_OVRD_ENR {
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
            TX_MODE_OVRD_ENR::_0 => false,
            TX_MODE_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_MODE_OVRD_ENR {
        match value {
            false => TX_MODE_OVRD_ENR::_0,
            true => TX_MODE_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_MODE_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_MODE_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TX_MODE_OVRDR {
    bits: bool,
}
impl TX_MODE_OVRDR {
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
#[doc = "Possible values of the field `RX_MODE_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_MODE_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_MODE_OVRD to override the signal \"rx_mode\"."]
    _1,
}
impl RX_MODE_OVRD_ENR {
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
            RX_MODE_OVRD_ENR::_0 => false,
            RX_MODE_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_MODE_OVRD_ENR {
        match value {
            false => RX_MODE_OVRD_ENR::_0,
            true => RX_MODE_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_MODE_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_MODE_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_MODE_OVRDR {
    bits: bool,
}
impl RX_MODE_OVRDR {
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
#[doc = "Values that can be written to the field `TSM_SPARE0_EN_OVRD_EN`"]
pub enum TSM_SPARE0_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TSM_SPARE0_EN_OVRD to override the signal \"tsm_spare0_en\"."]
    _1,
}
impl TSM_SPARE0_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSM_SPARE0_EN_OVRD_ENW::_0 => false,
            TSM_SPARE0_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSM_SPARE0_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_SPARE0_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSM_SPARE0_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSM_SPARE0_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of TSM_SPARE0_EN_OVRD to override the signal \"tsm_spare0_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSM_SPARE0_EN_OVRD_ENW::_1)
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
pub struct _TSM_SPARE0_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_SPARE0_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `TSM_SPARE1_EN_OVRD_EN`"]
pub enum TSM_SPARE1_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TSM_SPARE1_EN_OVRD to override the signal \"tsm_spare1_en\"."]
    _1,
}
impl TSM_SPARE1_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSM_SPARE1_EN_OVRD_ENW::_0 => false,
            TSM_SPARE1_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSM_SPARE1_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_SPARE1_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSM_SPARE1_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSM_SPARE1_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of TSM_SPARE1_EN_OVRD to override the signal \"tsm_spare1_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSM_SPARE1_EN_OVRD_ENW::_1)
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
pub struct _TSM_SPARE1_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_SPARE1_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `TSM_SPARE2_EN_OVRD_EN`"]
pub enum TSM_SPARE2_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TSM_SPARE2_EN_OVRD to override the signal \"tsm_spare2_en\"."]
    _1,
}
impl TSM_SPARE2_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSM_SPARE2_EN_OVRD_ENW::_0 => false,
            TSM_SPARE2_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSM_SPARE2_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_SPARE2_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSM_SPARE2_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSM_SPARE2_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of TSM_SPARE2_EN_OVRD to override the signal \"tsm_spare2_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSM_SPARE2_EN_OVRD_ENW::_1)
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
pub struct _TSM_SPARE2_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_SPARE2_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `TSM_SPARE3_EN_OVRD_EN`"]
pub enum TSM_SPARE3_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TSM_SPARE3_EN_OVRD to override the signal \"tsm_spare3_en\"."]
    _1,
}
impl TSM_SPARE3_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSM_SPARE3_EN_OVRD_ENW::_0 => false,
            TSM_SPARE3_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSM_SPARE3_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_SPARE3_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSM_SPARE3_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSM_SPARE3_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of TSM_SPARE3_EN_OVRD to override the signal \"tsm_spare3_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSM_SPARE3_EN_OVRD_ENW::_1)
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
pub struct _TSM_SPARE3_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_SPARE3_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RXTX_AUXPLL_BIAS_EN_OVRD_EN`"]
pub enum RXTX_AUXPLL_BIAS_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_BIAS_EN_OVRD to override the signal \"rxtx_auxpll_bias_en\"."]
    _1,
}
impl RXTX_AUXPLL_BIAS_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTX_AUXPLL_BIAS_EN_OVRD_ENW::_0 => false,
            RXTX_AUXPLL_BIAS_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTX_AUXPLL_BIAS_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_BIAS_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTX_AUXPLL_BIAS_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_BIAS_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RXTX_AUXPLL_BIAS_EN_OVRD to override the signal \"rxtx_auxpll_bias_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_BIAS_EN_OVRD_ENW::_1)
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
pub struct _RXTX_AUXPLL_BIAS_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_BIAS_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RXTX_AUXPLL_VCO_EN_OVRD_EN`"]
pub enum RXTX_AUXPLL_VCO_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_VCO_EN_OVRD to override the signal \"rxtx_auxpll_vco_en\"."]
    _1,
}
impl RXTX_AUXPLL_VCO_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTX_AUXPLL_VCO_EN_OVRD_ENW::_0 => false,
            RXTX_AUXPLL_VCO_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTX_AUXPLL_VCO_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_VCO_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTX_AUXPLL_VCO_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_VCO_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RXTX_AUXPLL_VCO_EN_OVRD to override the signal \"rxtx_auxpll_vco_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_VCO_EN_OVRD_ENW::_1)
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
pub struct _RXTX_AUXPLL_VCO_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_VCO_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RXTX_AUXPLL_FCAL_EN_OVRD_EN`"]
pub enum RXTX_AUXPLL_FCAL_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_FCAL_EN_OVRD to override the signal \"rxtx_auxpll_fcal_en\"."]
    _1,
}
impl RXTX_AUXPLL_FCAL_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTX_AUXPLL_FCAL_EN_OVRD_ENW::_0 => false,
            RXTX_AUXPLL_FCAL_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTX_AUXPLL_FCAL_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_FCAL_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTX_AUXPLL_FCAL_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_FCAL_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RXTX_AUXPLL_FCAL_EN_OVRD to override the signal \"rxtx_auxpll_fcal_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_FCAL_EN_OVRD_ENW::_1)
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
pub struct _RXTX_AUXPLL_FCAL_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_FCAL_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RXTX_AUXPLL_LF_EN_OVRD_EN`"]
pub enum RXTX_AUXPLL_LF_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_LF_EN_OVRD to override the signal \"rxtx_auxpll_lf_en\"."]
    _1,
}
impl RXTX_AUXPLL_LF_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTX_AUXPLL_LF_EN_OVRD_ENW::_0 => false,
            RXTX_AUXPLL_LF_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTX_AUXPLL_LF_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_LF_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTX_AUXPLL_LF_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_LF_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RXTX_AUXPLL_LF_EN_OVRD to override the signal \"rxtx_auxpll_lf_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_LF_EN_OVRD_ENW::_1)
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
pub struct _RXTX_AUXPLL_LF_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_LF_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RXTX_AUXPLL_PD_EN_OVRD_EN`"]
pub enum RXTX_AUXPLL_PD_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_PD_EN_OVRD to override the signal \"rxtx_auxpll_pd_en\"."]
    _1,
}
impl RXTX_AUXPLL_PD_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTX_AUXPLL_PD_EN_OVRD_ENW::_0 => false,
            RXTX_AUXPLL_PD_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTX_AUXPLL_PD_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_PD_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTX_AUXPLL_PD_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_PD_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RXTX_AUXPLL_PD_EN_OVRD to override the signal \"rxtx_auxpll_pd_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_PD_EN_OVRD_ENW::_1)
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
pub struct _RXTX_AUXPLL_PD_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_PD_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_EN`"]
pub enum RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD to override the signal \"rxtx_auxpll_pd_lf_filter_charge_en\"."]
    _1,
}
impl RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENW::_0 => false,
            RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD to override the signal \"rxtx_auxpll_pd_lf_filter_charge_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENW::_1)
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
pub struct _RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RXTX_AUXPLL_ADC_BUF_EN_OVRD_EN`"]
pub enum RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_ADC_BUF_EN_OVRD to override the signal \"rxtx_auxpll_adc_buf_en\"."]
    _1,
}
impl RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENW::_0 => false,
            RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RXTX_AUXPLL_ADC_BUF_EN_OVRD to override the signal \"rxtx_auxpll_adc_buf_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENW::_1)
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
pub struct _RXTX_AUXPLL_ADC_BUF_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_ADC_BUF_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RXTX_AUXPLL_DIG_BUF_EN_OVRD_EN`"]
pub enum RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_AUXPLL_DIG_BUF_EN_OVRD to override the signal \"rxtx_auxpll_dig_buf_en\"."]
    _1,
}
impl RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENW::_0 => false,
            RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RXTX_AUXPLL_DIG_BUF_EN_OVRD to override the signal \"rxtx_auxpll_dig_buf_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENW::_1)
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
pub struct _RXTX_AUXPLL_DIG_BUF_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_DIG_BUF_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RXTX_RCCAL_EN_OVRD_EN`"]
pub enum RXTX_RCCAL_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RXTX_RCCAL_EN_OVRD to override the signal \"rxtx_rccal_en\"."]
    _1,
}
impl RXTX_RCCAL_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTX_RCCAL_EN_OVRD_ENW::_0 => false,
            RXTX_RCCAL_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTX_RCCAL_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_RCCAL_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTX_RCCAL_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTX_RCCAL_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RXTX_RCCAL_EN_OVRD to override the signal \"rxtx_rccal_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTX_RCCAL_EN_OVRD_ENW::_1)
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
pub struct _RXTX_RCCAL_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_RCCAL_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `TX_HPM_DAC_EN_OVRD_EN`"]
pub enum TX_HPM_DAC_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TX_HPM_DAC_EN_OVRD to override the signal \"tx_hpm_dac_en\"."]
    _1,
}
impl TX_HPM_DAC_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_HPM_DAC_EN_OVRD_ENW::_0 => false,
            TX_HPM_DAC_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_HPM_DAC_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_HPM_DAC_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_HPM_DAC_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_HPM_DAC_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of TX_HPM_DAC_EN_OVRD to override the signal \"tx_hpm_dac_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_HPM_DAC_EN_OVRD_ENW::_1)
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
pub struct _TX_HPM_DAC_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_HPM_DAC_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `TX_MODE_OVRD_EN`"]
pub enum TX_MODE_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TX_MODE_OVRD to override the signal \"tx_mode\"."]
    _1,
}
impl TX_MODE_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_MODE_OVRD_ENW::_0 => false,
            TX_MODE_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_MODE_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_MODE_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_MODE_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_MODE_OVRD_ENW::_0)
    }
    #[doc = "Use the state of TX_MODE_OVRD to override the signal \"tx_mode\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_MODE_OVRD_ENW::_1)
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
pub struct _TX_MODE_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_MODE_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_MODE_OVRD_EN`"]
pub enum RX_MODE_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_MODE_OVRD to override the signal \"rx_mode\"."]
    _1,
}
impl RX_MODE_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_MODE_OVRD_ENW::_0 => false,
            RX_MODE_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_MODE_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_MODE_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_MODE_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_MODE_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_MODE_OVRD to override the signal \"rx_mode\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_MODE_OVRD_ENW::_1)
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
pub struct _RX_MODE_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_MODE_OVRDW<'a> {
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
    #[doc = "Bit 0 - Override control for TSM_SPARE0_EN"]
    #[inline]
    pub fn tsm_spare0_en_ovrd_en(&self) -> TSM_SPARE0_EN_OVRD_ENR {
        TSM_SPARE0_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Override value for TSM_SPARE0_EN"]
    #[inline]
    pub fn tsm_spare0_en_ovrd(&self) -> TSM_SPARE0_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSM_SPARE0_EN_OVRDR { bits }
    }
    #[doc = "Bit 2 - Override control for TSM_SPARE1_EN"]
    #[inline]
    pub fn tsm_spare1_en_ovrd_en(&self) -> TSM_SPARE1_EN_OVRD_ENR {
        TSM_SPARE1_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Override value for TSM_SPARE1_EN"]
    #[inline]
    pub fn tsm_spare1_en_ovrd(&self) -> TSM_SPARE1_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSM_SPARE1_EN_OVRDR { bits }
    }
    #[doc = "Bit 4 - Override control for TSM_SPARE2_EN"]
    #[inline]
    pub fn tsm_spare2_en_ovrd_en(&self) -> TSM_SPARE2_EN_OVRD_ENR {
        TSM_SPARE2_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Override value for TSM_SPARE2_EN"]
    #[inline]
    pub fn tsm_spare2_en_ovrd(&self) -> TSM_SPARE2_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSM_SPARE2_EN_OVRDR { bits }
    }
    #[doc = "Bit 6 - Override control for TSM_SPARE3_EN"]
    #[inline]
    pub fn tsm_spare3_en_ovrd_en(&self) -> TSM_SPARE3_EN_OVRD_ENR {
        TSM_SPARE3_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Override value for TSM_SPARE3_EN"]
    #[inline]
    pub fn tsm_spare3_en_ovrd(&self) -> TSM_SPARE3_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSM_SPARE3_EN_OVRDR { bits }
    }
    #[doc = "Bit 8 - Override control for RXTX_AUXPLL_BIAS_EN"]
    #[inline]
    pub fn rxtx_auxpll_bias_en_ovrd_en(&self) -> RXTX_AUXPLL_BIAS_EN_OVRD_ENR {
        RXTX_AUXPLL_BIAS_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Override value for RXTX_AUXPLL_BIAS_EN"]
    #[inline]
    pub fn rxtx_auxpll_bias_en_ovrd(&self) -> RXTX_AUXPLL_BIAS_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXTX_AUXPLL_BIAS_EN_OVRDR { bits }
    }
    #[doc = "Bit 10 - Override control for RXTX_AUXPLL_VCO_EN"]
    #[inline]
    pub fn rxtx_auxpll_vco_en_ovrd_en(&self) -> RXTX_AUXPLL_VCO_EN_OVRD_ENR {
        RXTX_AUXPLL_VCO_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Override value for RXTX_AUXPLL_VCO_EN"]
    #[inline]
    pub fn rxtx_auxpll_vco_en_ovrd(&self) -> RXTX_AUXPLL_VCO_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXTX_AUXPLL_VCO_EN_OVRDR { bits }
    }
    #[doc = "Bit 12 - Override control for RXTX_AUXPLL_FCAL_EN"]
    #[inline]
    pub fn rxtx_auxpll_fcal_en_ovrd_en(&self) -> RXTX_AUXPLL_FCAL_EN_OVRD_ENR {
        RXTX_AUXPLL_FCAL_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Override value for RXTX_AUXPLL_FCAL_EN"]
    #[inline]
    pub fn rxtx_auxpll_fcal_en_ovrd(&self) -> RXTX_AUXPLL_FCAL_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXTX_AUXPLL_FCAL_EN_OVRDR { bits }
    }
    #[doc = "Bit 14 - Override control for RXTX_AUXPLL_LF_EN"]
    #[inline]
    pub fn rxtx_auxpll_lf_en_ovrd_en(&self) -> RXTX_AUXPLL_LF_EN_OVRD_ENR {
        RXTX_AUXPLL_LF_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Override value for RXTX_AUXPLL_LF_EN"]
    #[inline]
    pub fn rxtx_auxpll_lf_en_ovrd(&self) -> RXTX_AUXPLL_LF_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXTX_AUXPLL_LF_EN_OVRDR { bits }
    }
    #[doc = "Bit 16 - Override control for RXTX_AUXPLL_PD_EN"]
    #[inline]
    pub fn rxtx_auxpll_pd_en_ovrd_en(&self) -> RXTX_AUXPLL_PD_EN_OVRD_ENR {
        RXTX_AUXPLL_PD_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Override value for RXTX_AUXPLL_PD_EN"]
    #[inline]
    pub fn rxtx_auxpll_pd_en_ovrd(&self) -> RXTX_AUXPLL_PD_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXTX_AUXPLL_PD_EN_OVRDR { bits }
    }
    #[doc = "Bit 18 - Override control for RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN"]
    #[inline]
    pub fn rxtx_auxpll_pd_lf_filter_charge_en_ovrd_en(
        &self,
    ) -> RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENR {
        RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Override value for RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN"]
    #[inline]
    pub fn rxtx_auxpll_pd_lf_filter_charge_en_ovrd(
        &self,
    ) -> RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRDR { bits }
    }
    #[doc = "Bit 20 - Override control for RXTX_AUXPLL_ADC_BUF_EN"]
    #[inline]
    pub fn rxtx_auxpll_adc_buf_en_ovrd_en(&self) -> RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENR {
        RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Override value for RXTX_AUXPLL_ADC_BUF_EN"]
    #[inline]
    pub fn rxtx_auxpll_adc_buf_en_ovrd(&self) -> RXTX_AUXPLL_ADC_BUF_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXTX_AUXPLL_ADC_BUF_EN_OVRDR { bits }
    }
    #[doc = "Bit 22 - Override control for RXTX_AUXPLL_DIG_BUF_EN"]
    #[inline]
    pub fn rxtx_auxpll_dig_buf_en_ovrd_en(&self) -> RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENR {
        RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Override value for RXTX_AUXPLL_DIG_BUF_EN"]
    #[inline]
    pub fn rxtx_auxpll_dig_buf_en_ovrd(&self) -> RXTX_AUXPLL_DIG_BUF_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXTX_AUXPLL_DIG_BUF_EN_OVRDR { bits }
    }
    #[doc = "Bit 24 - Override control for RXTX_RCCAL_EN"]
    #[inline]
    pub fn rxtx_rccal_en_ovrd_en(&self) -> RXTX_RCCAL_EN_OVRD_ENR {
        RXTX_RCCAL_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Override value for RXTX_RCCAL_EN"]
    #[inline]
    pub fn rxtx_rccal_en_ovrd(&self) -> RXTX_RCCAL_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXTX_RCCAL_EN_OVRDR { bits }
    }
    #[doc = "Bit 26 - Override control for TX_HPM_DAC_EN"]
    #[inline]
    pub fn tx_hpm_dac_en_ovrd_en(&self) -> TX_HPM_DAC_EN_OVRD_ENR {
        TX_HPM_DAC_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Override value for TX_HPM_DAC_EN"]
    #[inline]
    pub fn tx_hpm_dac_en_ovrd(&self) -> TX_HPM_DAC_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_HPM_DAC_EN_OVRDR { bits }
    }
    #[doc = "Bit 28 - Override control for TX_MODE"]
    #[inline]
    pub fn tx_mode_ovrd_en(&self) -> TX_MODE_OVRD_ENR {
        TX_MODE_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Override value for TX_MODE"]
    #[inline]
    pub fn tx_mode_ovrd(&self) -> TX_MODE_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_MODE_OVRDR { bits }
    }
    #[doc = "Bit 30 - Override control for RX_MODE"]
    #[inline]
    pub fn rx_mode_ovrd_en(&self) -> RX_MODE_OVRD_ENR {
        RX_MODE_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Override value for RX_MODE"]
    #[inline]
    pub fn rx_mode_ovrd(&self) -> RX_MODE_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_MODE_OVRDR { bits }
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
    #[doc = "Bit 0 - Override control for TSM_SPARE0_EN"]
    #[inline]
    pub fn tsm_spare0_en_ovrd_en(&mut self) -> _TSM_SPARE0_EN_OVRD_ENW {
        _TSM_SPARE0_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 1 - Override value for TSM_SPARE0_EN"]
    #[inline]
    pub fn tsm_spare0_en_ovrd(&mut self) -> _TSM_SPARE0_EN_OVRDW {
        _TSM_SPARE0_EN_OVRDW { w: self }
    }
    #[doc = "Bit 2 - Override control for TSM_SPARE1_EN"]
    #[inline]
    pub fn tsm_spare1_en_ovrd_en(&mut self) -> _TSM_SPARE1_EN_OVRD_ENW {
        _TSM_SPARE1_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 3 - Override value for TSM_SPARE1_EN"]
    #[inline]
    pub fn tsm_spare1_en_ovrd(&mut self) -> _TSM_SPARE1_EN_OVRDW {
        _TSM_SPARE1_EN_OVRDW { w: self }
    }
    #[doc = "Bit 4 - Override control for TSM_SPARE2_EN"]
    #[inline]
    pub fn tsm_spare2_en_ovrd_en(&mut self) -> _TSM_SPARE2_EN_OVRD_ENW {
        _TSM_SPARE2_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 5 - Override value for TSM_SPARE2_EN"]
    #[inline]
    pub fn tsm_spare2_en_ovrd(&mut self) -> _TSM_SPARE2_EN_OVRDW {
        _TSM_SPARE2_EN_OVRDW { w: self }
    }
    #[doc = "Bit 6 - Override control for TSM_SPARE3_EN"]
    #[inline]
    pub fn tsm_spare3_en_ovrd_en(&mut self) -> _TSM_SPARE3_EN_OVRD_ENW {
        _TSM_SPARE3_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 7 - Override value for TSM_SPARE3_EN"]
    #[inline]
    pub fn tsm_spare3_en_ovrd(&mut self) -> _TSM_SPARE3_EN_OVRDW {
        _TSM_SPARE3_EN_OVRDW { w: self }
    }
    #[doc = "Bit 8 - Override control for RXTX_AUXPLL_BIAS_EN"]
    #[inline]
    pub fn rxtx_auxpll_bias_en_ovrd_en(&mut self) -> _RXTX_AUXPLL_BIAS_EN_OVRD_ENW {
        _RXTX_AUXPLL_BIAS_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 9 - Override value for RXTX_AUXPLL_BIAS_EN"]
    #[inline]
    pub fn rxtx_auxpll_bias_en_ovrd(&mut self) -> _RXTX_AUXPLL_BIAS_EN_OVRDW {
        _RXTX_AUXPLL_BIAS_EN_OVRDW { w: self }
    }
    #[doc = "Bit 10 - Override control for RXTX_AUXPLL_VCO_EN"]
    #[inline]
    pub fn rxtx_auxpll_vco_en_ovrd_en(&mut self) -> _RXTX_AUXPLL_VCO_EN_OVRD_ENW {
        _RXTX_AUXPLL_VCO_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 11 - Override value for RXTX_AUXPLL_VCO_EN"]
    #[inline]
    pub fn rxtx_auxpll_vco_en_ovrd(&mut self) -> _RXTX_AUXPLL_VCO_EN_OVRDW {
        _RXTX_AUXPLL_VCO_EN_OVRDW { w: self }
    }
    #[doc = "Bit 12 - Override control for RXTX_AUXPLL_FCAL_EN"]
    #[inline]
    pub fn rxtx_auxpll_fcal_en_ovrd_en(&mut self) -> _RXTX_AUXPLL_FCAL_EN_OVRD_ENW {
        _RXTX_AUXPLL_FCAL_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 13 - Override value for RXTX_AUXPLL_FCAL_EN"]
    #[inline]
    pub fn rxtx_auxpll_fcal_en_ovrd(&mut self) -> _RXTX_AUXPLL_FCAL_EN_OVRDW {
        _RXTX_AUXPLL_FCAL_EN_OVRDW { w: self }
    }
    #[doc = "Bit 14 - Override control for RXTX_AUXPLL_LF_EN"]
    #[inline]
    pub fn rxtx_auxpll_lf_en_ovrd_en(&mut self) -> _RXTX_AUXPLL_LF_EN_OVRD_ENW {
        _RXTX_AUXPLL_LF_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 15 - Override value for RXTX_AUXPLL_LF_EN"]
    #[inline]
    pub fn rxtx_auxpll_lf_en_ovrd(&mut self) -> _RXTX_AUXPLL_LF_EN_OVRDW {
        _RXTX_AUXPLL_LF_EN_OVRDW { w: self }
    }
    #[doc = "Bit 16 - Override control for RXTX_AUXPLL_PD_EN"]
    #[inline]
    pub fn rxtx_auxpll_pd_en_ovrd_en(&mut self) -> _RXTX_AUXPLL_PD_EN_OVRD_ENW {
        _RXTX_AUXPLL_PD_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 17 - Override value for RXTX_AUXPLL_PD_EN"]
    #[inline]
    pub fn rxtx_auxpll_pd_en_ovrd(&mut self) -> _RXTX_AUXPLL_PD_EN_OVRDW {
        _RXTX_AUXPLL_PD_EN_OVRDW { w: self }
    }
    #[doc = "Bit 18 - Override control for RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN"]
    #[inline]
    pub fn rxtx_auxpll_pd_lf_filter_charge_en_ovrd_en(
        &mut self,
    ) -> _RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENW {
        _RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 19 - Override value for RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN"]
    #[inline]
    pub fn rxtx_auxpll_pd_lf_filter_charge_en_ovrd(
        &mut self,
    ) -> _RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRDW {
        _RXTX_AUXPLL_PD_LF_FILTER_CHARGE_EN_OVRDW { w: self }
    }
    #[doc = "Bit 20 - Override control for RXTX_AUXPLL_ADC_BUF_EN"]
    #[inline]
    pub fn rxtx_auxpll_adc_buf_en_ovrd_en(&mut self) -> _RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENW {
        _RXTX_AUXPLL_ADC_BUF_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 21 - Override value for RXTX_AUXPLL_ADC_BUF_EN"]
    #[inline]
    pub fn rxtx_auxpll_adc_buf_en_ovrd(&mut self) -> _RXTX_AUXPLL_ADC_BUF_EN_OVRDW {
        _RXTX_AUXPLL_ADC_BUF_EN_OVRDW { w: self }
    }
    #[doc = "Bit 22 - Override control for RXTX_AUXPLL_DIG_BUF_EN"]
    #[inline]
    pub fn rxtx_auxpll_dig_buf_en_ovrd_en(&mut self) -> _RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENW {
        _RXTX_AUXPLL_DIG_BUF_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 23 - Override value for RXTX_AUXPLL_DIG_BUF_EN"]
    #[inline]
    pub fn rxtx_auxpll_dig_buf_en_ovrd(&mut self) -> _RXTX_AUXPLL_DIG_BUF_EN_OVRDW {
        _RXTX_AUXPLL_DIG_BUF_EN_OVRDW { w: self }
    }
    #[doc = "Bit 24 - Override control for RXTX_RCCAL_EN"]
    #[inline]
    pub fn rxtx_rccal_en_ovrd_en(&mut self) -> _RXTX_RCCAL_EN_OVRD_ENW {
        _RXTX_RCCAL_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 25 - Override value for RXTX_RCCAL_EN"]
    #[inline]
    pub fn rxtx_rccal_en_ovrd(&mut self) -> _RXTX_RCCAL_EN_OVRDW {
        _RXTX_RCCAL_EN_OVRDW { w: self }
    }
    #[doc = "Bit 26 - Override control for TX_HPM_DAC_EN"]
    #[inline]
    pub fn tx_hpm_dac_en_ovrd_en(&mut self) -> _TX_HPM_DAC_EN_OVRD_ENW {
        _TX_HPM_DAC_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 27 - Override value for TX_HPM_DAC_EN"]
    #[inline]
    pub fn tx_hpm_dac_en_ovrd(&mut self) -> _TX_HPM_DAC_EN_OVRDW {
        _TX_HPM_DAC_EN_OVRDW { w: self }
    }
    #[doc = "Bit 28 - Override control for TX_MODE"]
    #[inline]
    pub fn tx_mode_ovrd_en(&mut self) -> _TX_MODE_OVRD_ENW {
        _TX_MODE_OVRD_ENW { w: self }
    }
    #[doc = "Bit 29 - Override value for TX_MODE"]
    #[inline]
    pub fn tx_mode_ovrd(&mut self) -> _TX_MODE_OVRDW {
        _TX_MODE_OVRDW { w: self }
    }
    #[doc = "Bit 30 - Override control for RX_MODE"]
    #[inline]
    pub fn rx_mode_ovrd_en(&mut self) -> _RX_MODE_OVRD_ENW {
        _RX_MODE_OVRD_ENW { w: self }
    }
    #[doc = "Bit 31 - Override value for RX_MODE"]
    #[inline]
    pub fn rx_mode_ovrd(&mut self) -> _RX_MODE_OVRDW {
        _RX_MODE_OVRDW { w: self }
    }
}
