#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OVRD1 {
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
#[doc = "Possible values of the field `SY_LO_RX_BUF_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_LO_RX_BUF_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_LO_RX_BUF_EN_OVRD to override the signal \"sy_lo_rx_buf_en\"."]
    _1,
}
impl SY_LO_RX_BUF_EN_OVRD_ENR {
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
            SY_LO_RX_BUF_EN_OVRD_ENR::_0 => false,
            SY_LO_RX_BUF_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_LO_RX_BUF_EN_OVRD_ENR {
        match value {
            false => SY_LO_RX_BUF_EN_OVRD_ENR::_0,
            true => SY_LO_RX_BUF_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_LO_RX_BUF_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_LO_RX_BUF_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_LO_RX_BUF_EN_OVRDR {
    bits: bool,
}
impl SY_LO_RX_BUF_EN_OVRDR {
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
#[doc = "Possible values of the field `SY_LO_TX_BUF_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_LO_TX_BUF_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_LO_TX_BUF_EN_OVRD to override the signal \"sy_lo_tx_buf_en\"."]
    _1,
}
impl SY_LO_TX_BUF_EN_OVRD_ENR {
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
            SY_LO_TX_BUF_EN_OVRD_ENR::_0 => false,
            SY_LO_TX_BUF_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_LO_TX_BUF_EN_OVRD_ENR {
        match value {
            false => SY_LO_TX_BUF_EN_OVRD_ENR::_0,
            true => SY_LO_TX_BUF_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_LO_TX_BUF_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_LO_TX_BUF_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_LO_TX_BUF_EN_OVRDR {
    bits: bool,
}
impl SY_LO_TX_BUF_EN_OVRDR {
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
#[doc = "Possible values of the field `SY_DIVN_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_DIVN_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_DIVN_EN_OVRD to override the signal \"sy_divn_en\"."]
    _1,
}
impl SY_DIVN_EN_OVRD_ENR {
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
            SY_DIVN_EN_OVRD_ENR::_0 => false,
            SY_DIVN_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_DIVN_EN_OVRD_ENR {
        match value {
            false => SY_DIVN_EN_OVRD_ENR::_0,
            true => SY_DIVN_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_DIVN_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_DIVN_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_DIVN_EN_OVRDR {
    bits: bool,
}
impl SY_DIVN_EN_OVRDR {
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
#[doc = "Possible values of the field `SY_PD_FILTER_CHARGE_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_PD_FILTER_CHARGE_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_PD_FILTER_CHARGE_EN_OVRD to override the signal \"sy_pd_filter_charge_en\"."]
    _1,
}
impl SY_PD_FILTER_CHARGE_EN_OVRD_ENR {
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
            SY_PD_FILTER_CHARGE_EN_OVRD_ENR::_0 => false,
            SY_PD_FILTER_CHARGE_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_PD_FILTER_CHARGE_EN_OVRD_ENR {
        match value {
            false => SY_PD_FILTER_CHARGE_EN_OVRD_ENR::_0,
            true => SY_PD_FILTER_CHARGE_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_PD_FILTER_CHARGE_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_PD_FILTER_CHARGE_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_PD_FILTER_CHARGE_EN_OVRDR {
    bits: bool,
}
impl SY_PD_FILTER_CHARGE_EN_OVRDR {
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
#[doc = "Possible values of the field `SY_PD_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_PD_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_PD_EN_OVRD to override the signal \"sy_pd_en\"."]
    _1,
}
impl SY_PD_EN_OVRD_ENR {
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
            SY_PD_EN_OVRD_ENR::_0 => false,
            SY_PD_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_PD_EN_OVRD_ENR {
        match value {
            false => SY_PD_EN_OVRD_ENR::_0,
            true => SY_PD_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_PD_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_PD_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_PD_EN_OVRDR {
    bits: bool,
}
impl SY_PD_EN_OVRDR {
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
#[doc = "Possible values of the field `SY_LO_DIVN_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_LO_DIVN_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_LO_DIVN_EN_OVRD to override the signal \"sy_lo_divn_en\"."]
    _1,
}
impl SY_LO_DIVN_EN_OVRD_ENR {
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
            SY_LO_DIVN_EN_OVRD_ENR::_0 => false,
            SY_LO_DIVN_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_LO_DIVN_EN_OVRD_ENR {
        match value {
            false => SY_LO_DIVN_EN_OVRD_ENR::_0,
            true => SY_LO_DIVN_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_LO_DIVN_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_LO_DIVN_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_LO_DIVN_EN_OVRDR {
    bits: bool,
}
impl SY_LO_DIVN_EN_OVRDR {
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
#[doc = "Possible values of the field `SY_LO_RX_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_LO_RX_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_LO_RX_EN_OVRD to override the signal \"sy_lo_rx_en\"."]
    _1,
}
impl SY_LO_RX_EN_OVRD_ENR {
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
            SY_LO_RX_EN_OVRD_ENR::_0 => false,
            SY_LO_RX_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_LO_RX_EN_OVRD_ENR {
        match value {
            false => SY_LO_RX_EN_OVRD_ENR::_0,
            true => SY_LO_RX_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_LO_RX_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_LO_RX_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_LO_RX_EN_OVRDR {
    bits: bool,
}
impl SY_LO_RX_EN_OVRDR {
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
#[doc = "Possible values of the field `SY_LO_TX_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_LO_TX_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_LO_TX_EN_OVRD to override the signal \"sy_lo_tx_en\"."]
    _1,
}
impl SY_LO_TX_EN_OVRD_ENR {
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
            SY_LO_TX_EN_OVRD_ENR::_0 => false,
            SY_LO_TX_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_LO_TX_EN_OVRD_ENR {
        match value {
            false => SY_LO_TX_EN_OVRD_ENR::_0,
            true => SY_LO_TX_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_LO_TX_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_LO_TX_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_LO_TX_EN_OVRDR {
    bits: bool,
}
impl SY_LO_TX_EN_OVRDR {
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
#[doc = "Possible values of the field `SY_DIVN_CAL_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SY_DIVN_CAL_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_DIVN_CAL_EN_OVRD to override the signal \"sy_divn_cal_en\"."]
    _1,
}
impl SY_DIVN_CAL_EN_OVRD_ENR {
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
            SY_DIVN_CAL_EN_OVRD_ENR::_0 => false,
            SY_DIVN_CAL_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SY_DIVN_CAL_EN_OVRD_ENR {
        match value {
            false => SY_DIVN_CAL_EN_OVRD_ENR::_0,
            true => SY_DIVN_CAL_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SY_DIVN_CAL_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SY_DIVN_CAL_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SY_DIVN_CAL_EN_OVRDR {
    bits: bool,
}
impl SY_DIVN_CAL_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_MIXER_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_MIXER_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_MIXER_EN_OVRD to override the signal \"rx_mixer_en\"."]
    _1,
}
impl RX_MIXER_EN_OVRD_ENR {
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
            RX_MIXER_EN_OVRD_ENR::_0 => false,
            RX_MIXER_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_MIXER_EN_OVRD_ENR {
        match value {
            false => RX_MIXER_EN_OVRD_ENR::_0,
            true => RX_MIXER_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_MIXER_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_MIXER_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_MIXER_EN_OVRDR {
    bits: bool,
}
impl RX_MIXER_EN_OVRDR {
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
#[doc = "Possible values of the field `TX_PA_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_PA_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TX_PA_EN_OVRD to override the signal \"tx_pa_en\"."]
    _1,
}
impl TX_PA_EN_OVRD_ENR {
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
            TX_PA_EN_OVRD_ENR::_0 => false,
            TX_PA_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_PA_EN_OVRD_ENR {
        match value {
            false => TX_PA_EN_OVRD_ENR::_0,
            true => TX_PA_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_PA_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_PA_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TX_PA_EN_OVRDR {
    bits: bool,
}
impl TX_PA_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_ADC_I_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ADC_I_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_ADC_I_EN_OVRD to override the signal \"rx_adc_i_en\"."]
    _1,
}
impl RX_ADC_I_EN_OVRD_ENR {
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
            RX_ADC_I_EN_OVRD_ENR::_0 => false,
            RX_ADC_I_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_ADC_I_EN_OVRD_ENR {
        match value {
            false => RX_ADC_I_EN_OVRD_ENR::_0,
            true => RX_ADC_I_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_ADC_I_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_ADC_I_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_ADC_I_EN_OVRDR {
    bits: bool,
}
impl RX_ADC_I_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_ADC_Q_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ADC_Q_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_ADC_Q_EN_OVRD to override the signal \"rx_adc_q_en\"."]
    _1,
}
impl RX_ADC_Q_EN_OVRD_ENR {
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
            RX_ADC_Q_EN_OVRD_ENR::_0 => false,
            RX_ADC_Q_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_ADC_Q_EN_OVRD_ENR {
        match value {
            false => RX_ADC_Q_EN_OVRD_ENR::_0,
            true => RX_ADC_Q_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_ADC_Q_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_ADC_Q_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_ADC_Q_EN_OVRDR {
    bits: bool,
}
impl RX_ADC_Q_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_ADC_RESET_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ADC_RESET_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_ADC_RESET_EN_OVRD to override the signal \"rx_adc_reset_en\"."]
    _1,
}
impl RX_ADC_RESET_EN_OVRD_ENR {
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
            RX_ADC_RESET_EN_OVRD_ENR::_0 => false,
            RX_ADC_RESET_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_ADC_RESET_EN_OVRD_ENR {
        match value {
            false => RX_ADC_RESET_EN_OVRD_ENR::_0,
            true => RX_ADC_RESET_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_ADC_RESET_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_ADC_RESET_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_ADC_RESET_EN_OVRDR {
    bits: bool,
}
impl RX_ADC_RESET_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_BBA_I_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BBA_I_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_BBA_I_EN_OVRD to override the signal \"rx_bba_i_en\"."]
    _1,
}
impl RX_BBA_I_EN_OVRD_ENR {
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
            RX_BBA_I_EN_OVRD_ENR::_0 => false,
            RX_BBA_I_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_BBA_I_EN_OVRD_ENR {
        match value {
            false => RX_BBA_I_EN_OVRD_ENR::_0,
            true => RX_BBA_I_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_BBA_I_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_BBA_I_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_BBA_I_EN_OVRDR {
    bits: bool,
}
impl RX_BBA_I_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_BBA_Q_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BBA_Q_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_BBA_Q_EN_OVRD to override the signal \"rx_bba_q_en\"."]
    _1,
}
impl RX_BBA_Q_EN_OVRD_ENR {
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
            RX_BBA_Q_EN_OVRD_ENR::_0 => false,
            RX_BBA_Q_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_BBA_Q_EN_OVRD_ENR {
        match value {
            false => RX_BBA_Q_EN_OVRD_ENR::_0,
            true => RX_BBA_Q_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_BBA_Q_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_BBA_Q_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_BBA_Q_EN_OVRDR {
    bits: bool,
}
impl RX_BBA_Q_EN_OVRDR {
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
#[doc = "Values that can be written to the field `SY_LO_RX_BUF_EN_OVRD_EN`"]
pub enum SY_LO_RX_BUF_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_LO_RX_BUF_EN_OVRD to override the signal \"sy_lo_rx_buf_en\"."]
    _1,
}
impl SY_LO_RX_BUF_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_LO_RX_BUF_EN_OVRD_ENW::_0 => false,
            SY_LO_RX_BUF_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_LO_RX_BUF_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_RX_BUF_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_LO_RX_BUF_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_LO_RX_BUF_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of SY_LO_RX_BUF_EN_OVRD to override the signal \"sy_lo_rx_buf_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_LO_RX_BUF_EN_OVRD_ENW::_1)
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
pub struct _SY_LO_RX_BUF_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_RX_BUF_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `SY_LO_TX_BUF_EN_OVRD_EN`"]
pub enum SY_LO_TX_BUF_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_LO_TX_BUF_EN_OVRD to override the signal \"sy_lo_tx_buf_en\"."]
    _1,
}
impl SY_LO_TX_BUF_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_LO_TX_BUF_EN_OVRD_ENW::_0 => false,
            SY_LO_TX_BUF_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_LO_TX_BUF_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_TX_BUF_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_LO_TX_BUF_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_LO_TX_BUF_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of SY_LO_TX_BUF_EN_OVRD to override the signal \"sy_lo_tx_buf_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_LO_TX_BUF_EN_OVRD_ENW::_1)
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
pub struct _SY_LO_TX_BUF_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_TX_BUF_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `SY_DIVN_EN_OVRD_EN`"]
pub enum SY_DIVN_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_DIVN_EN_OVRD to override the signal \"sy_divn_en\"."]
    _1,
}
impl SY_DIVN_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_DIVN_EN_OVRD_ENW::_0 => false,
            SY_DIVN_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_DIVN_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_DIVN_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_DIVN_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_DIVN_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of SY_DIVN_EN_OVRD to override the signal \"sy_divn_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_DIVN_EN_OVRD_ENW::_1)
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
pub struct _SY_DIVN_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_DIVN_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `SY_PD_FILTER_CHARGE_EN_OVRD_EN`"]
pub enum SY_PD_FILTER_CHARGE_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_PD_FILTER_CHARGE_EN_OVRD to override the signal \"sy_pd_filter_charge_en\"."]
    _1,
}
impl SY_PD_FILTER_CHARGE_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_PD_FILTER_CHARGE_EN_OVRD_ENW::_0 => false,
            SY_PD_FILTER_CHARGE_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_PD_FILTER_CHARGE_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_PD_FILTER_CHARGE_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_PD_FILTER_CHARGE_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_PD_FILTER_CHARGE_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of SY_PD_FILTER_CHARGE_EN_OVRD to override the signal \"sy_pd_filter_charge_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_PD_FILTER_CHARGE_EN_OVRD_ENW::_1)
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
pub struct _SY_PD_FILTER_CHARGE_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_PD_FILTER_CHARGE_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `SY_PD_EN_OVRD_EN`"]
pub enum SY_PD_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_PD_EN_OVRD to override the signal \"sy_pd_en\"."]
    _1,
}
impl SY_PD_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_PD_EN_OVRD_ENW::_0 => false,
            SY_PD_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_PD_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_PD_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_PD_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_PD_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of SY_PD_EN_OVRD to override the signal \"sy_pd_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_PD_EN_OVRD_ENW::_1)
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
pub struct _SY_PD_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_PD_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `SY_LO_DIVN_EN_OVRD_EN`"]
pub enum SY_LO_DIVN_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_LO_DIVN_EN_OVRD to override the signal \"sy_lo_divn_en\"."]
    _1,
}
impl SY_LO_DIVN_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_LO_DIVN_EN_OVRD_ENW::_0 => false,
            SY_LO_DIVN_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_LO_DIVN_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_DIVN_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_LO_DIVN_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_LO_DIVN_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of SY_LO_DIVN_EN_OVRD to override the signal \"sy_lo_divn_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_LO_DIVN_EN_OVRD_ENW::_1)
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
pub struct _SY_LO_DIVN_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_DIVN_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `SY_LO_RX_EN_OVRD_EN`"]
pub enum SY_LO_RX_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_LO_RX_EN_OVRD to override the signal \"sy_lo_rx_en\"."]
    _1,
}
impl SY_LO_RX_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_LO_RX_EN_OVRD_ENW::_0 => false,
            SY_LO_RX_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_LO_RX_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_RX_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_LO_RX_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_LO_RX_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of SY_LO_RX_EN_OVRD to override the signal \"sy_lo_rx_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_LO_RX_EN_OVRD_ENW::_1)
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
pub struct _SY_LO_RX_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_RX_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `SY_LO_TX_EN_OVRD_EN`"]
pub enum SY_LO_TX_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_LO_TX_EN_OVRD to override the signal \"sy_lo_tx_en\"."]
    _1,
}
impl SY_LO_TX_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_LO_TX_EN_OVRD_ENW::_0 => false,
            SY_LO_TX_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_LO_TX_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_TX_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_LO_TX_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_LO_TX_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of SY_LO_TX_EN_OVRD to override the signal \"sy_lo_tx_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_LO_TX_EN_OVRD_ENW::_1)
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
pub struct _SY_LO_TX_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_TX_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `SY_DIVN_CAL_EN_OVRD_EN`"]
pub enum SY_DIVN_CAL_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SY_DIVN_CAL_EN_OVRD to override the signal \"sy_divn_cal_en\"."]
    _1,
}
impl SY_DIVN_CAL_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SY_DIVN_CAL_EN_OVRD_ENW::_0 => false,
            SY_DIVN_CAL_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SY_DIVN_CAL_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_DIVN_CAL_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SY_DIVN_CAL_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SY_DIVN_CAL_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of SY_DIVN_CAL_EN_OVRD to override the signal \"sy_divn_cal_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SY_DIVN_CAL_EN_OVRD_ENW::_1)
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
pub struct _SY_DIVN_CAL_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_DIVN_CAL_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_MIXER_EN_OVRD_EN`"]
pub enum RX_MIXER_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_MIXER_EN_OVRD to override the signal \"rx_mixer_en\"."]
    _1,
}
impl RX_MIXER_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_MIXER_EN_OVRD_ENW::_0 => false,
            RX_MIXER_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_MIXER_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_MIXER_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_MIXER_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_MIXER_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_MIXER_EN_OVRD to override the signal \"rx_mixer_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_MIXER_EN_OVRD_ENW::_1)
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
pub struct _RX_MIXER_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_MIXER_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `TX_PA_EN_OVRD_EN`"]
pub enum TX_PA_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TX_PA_EN_OVRD to override the signal \"tx_pa_en\"."]
    _1,
}
impl TX_PA_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_PA_EN_OVRD_ENW::_0 => false,
            TX_PA_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_PA_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_PA_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_PA_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_PA_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of TX_PA_EN_OVRD to override the signal \"tx_pa_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_PA_EN_OVRD_ENW::_1)
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
pub struct _TX_PA_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_PA_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_ADC_I_EN_OVRD_EN`"]
pub enum RX_ADC_I_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_ADC_I_EN_OVRD to override the signal \"rx_adc_i_en\"."]
    _1,
}
impl RX_ADC_I_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_ADC_I_EN_OVRD_ENW::_0 => false,
            RX_ADC_I_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_ADC_I_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_I_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_ADC_I_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_ADC_I_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_ADC_I_EN_OVRD to override the signal \"rx_adc_i_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_ADC_I_EN_OVRD_ENW::_1)
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
pub struct _RX_ADC_I_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_I_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_ADC_Q_EN_OVRD_EN`"]
pub enum RX_ADC_Q_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_ADC_Q_EN_OVRD to override the signal \"rx_adc_q_en\"."]
    _1,
}
impl RX_ADC_Q_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_ADC_Q_EN_OVRD_ENW::_0 => false,
            RX_ADC_Q_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_ADC_Q_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_Q_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_ADC_Q_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_ADC_Q_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_ADC_Q_EN_OVRD to override the signal \"rx_adc_q_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_ADC_Q_EN_OVRD_ENW::_1)
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
pub struct _RX_ADC_Q_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_Q_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_ADC_RESET_EN_OVRD_EN`"]
pub enum RX_ADC_RESET_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_ADC_RESET_EN_OVRD to override the signal \"rx_adc_reset_en\"."]
    _1,
}
impl RX_ADC_RESET_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_ADC_RESET_EN_OVRD_ENW::_0 => false,
            RX_ADC_RESET_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_ADC_RESET_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_RESET_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_ADC_RESET_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_ADC_RESET_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_ADC_RESET_EN_OVRD to override the signal \"rx_adc_reset_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_ADC_RESET_EN_OVRD_ENW::_1)
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
pub struct _RX_ADC_RESET_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_RESET_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_BBA_I_EN_OVRD_EN`"]
pub enum RX_BBA_I_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_BBA_I_EN_OVRD to override the signal \"rx_bba_i_en\"."]
    _1,
}
impl RX_BBA_I_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_BBA_I_EN_OVRD_ENW::_0 => false,
            RX_BBA_I_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_BBA_I_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_I_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_BBA_I_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_BBA_I_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_BBA_I_EN_OVRD to override the signal \"rx_bba_i_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_BBA_I_EN_OVRD_ENW::_1)
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
pub struct _RX_BBA_I_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_I_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_BBA_Q_EN_OVRD_EN`"]
pub enum RX_BBA_Q_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_BBA_Q_EN_OVRD to override the signal \"rx_bba_q_en\"."]
    _1,
}
impl RX_BBA_Q_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_BBA_Q_EN_OVRD_ENW::_0 => false,
            RX_BBA_Q_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_BBA_Q_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_Q_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_BBA_Q_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_BBA_Q_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_BBA_Q_EN_OVRD to override the signal \"rx_bba_q_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_BBA_Q_EN_OVRD_ENW::_1)
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
pub struct _RX_BBA_Q_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_Q_EN_OVRDW<'a> {
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
    #[doc = "Bit 0 - Override control for SY_LO_RX_BUF_EN"]
    #[inline]
    pub fn sy_lo_rx_buf_en_ovrd_en(&self) -> SY_LO_RX_BUF_EN_OVRD_ENR {
        SY_LO_RX_BUF_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Override value for SY_LO_RX_BUF_EN"]
    #[inline]
    pub fn sy_lo_rx_buf_en_ovrd(&self) -> SY_LO_RX_BUF_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_LO_RX_BUF_EN_OVRDR { bits }
    }
    #[doc = "Bit 2 - Override control for SY_LO_TX_BUF_EN"]
    #[inline]
    pub fn sy_lo_tx_buf_en_ovrd_en(&self) -> SY_LO_TX_BUF_EN_OVRD_ENR {
        SY_LO_TX_BUF_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Override value for SY_LO_TX_BUF_EN"]
    #[inline]
    pub fn sy_lo_tx_buf_en_ovrd(&self) -> SY_LO_TX_BUF_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_LO_TX_BUF_EN_OVRDR { bits }
    }
    #[doc = "Bit 4 - Override control for SY_DIVN_EN"]
    #[inline]
    pub fn sy_divn_en_ovrd_en(&self) -> SY_DIVN_EN_OVRD_ENR {
        SY_DIVN_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Override value for SY_DIVN_EN"]
    #[inline]
    pub fn sy_divn_en_ovrd(&self) -> SY_DIVN_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_DIVN_EN_OVRDR { bits }
    }
    #[doc = "Bit 6 - Override control for SY_PD_FILTER_CHARGE_EN"]
    #[inline]
    pub fn sy_pd_filter_charge_en_ovrd_en(&self) -> SY_PD_FILTER_CHARGE_EN_OVRD_ENR {
        SY_PD_FILTER_CHARGE_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Override value for SY_PD_FILTER_CHARGE_EN"]
    #[inline]
    pub fn sy_pd_filter_charge_en_ovrd(&self) -> SY_PD_FILTER_CHARGE_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_PD_FILTER_CHARGE_EN_OVRDR { bits }
    }
    #[doc = "Bit 8 - Override control for SY_PD_EN"]
    #[inline]
    pub fn sy_pd_en_ovrd_en(&self) -> SY_PD_EN_OVRD_ENR {
        SY_PD_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Override value for SY_PD_EN"]
    #[inline]
    pub fn sy_pd_en_ovrd(&self) -> SY_PD_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_PD_EN_OVRDR { bits }
    }
    #[doc = "Bit 10 - Override control for SY_LO_DIVN_EN"]
    #[inline]
    pub fn sy_lo_divn_en_ovrd_en(&self) -> SY_LO_DIVN_EN_OVRD_ENR {
        SY_LO_DIVN_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Override value for SY_LO_DIVN_EN"]
    #[inline]
    pub fn sy_lo_divn_en_ovrd(&self) -> SY_LO_DIVN_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_LO_DIVN_EN_OVRDR { bits }
    }
    #[doc = "Bit 12 - Override control for SY_LO_RX_EN"]
    #[inline]
    pub fn sy_lo_rx_en_ovrd_en(&self) -> SY_LO_RX_EN_OVRD_ENR {
        SY_LO_RX_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Override value for SY_LO_RX_EN"]
    #[inline]
    pub fn sy_lo_rx_en_ovrd(&self) -> SY_LO_RX_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_LO_RX_EN_OVRDR { bits }
    }
    #[doc = "Bit 14 - Override control for SY_LO_TX_EN"]
    #[inline]
    pub fn sy_lo_tx_en_ovrd_en(&self) -> SY_LO_TX_EN_OVRD_ENR {
        SY_LO_TX_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Override value for SY_LO_TX_EN"]
    #[inline]
    pub fn sy_lo_tx_en_ovrd(&self) -> SY_LO_TX_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_LO_TX_EN_OVRDR { bits }
    }
    #[doc = "Bit 16 - Override control for SY_DIVN_CAL_EN"]
    #[inline]
    pub fn sy_divn_cal_en_ovrd_en(&self) -> SY_DIVN_CAL_EN_OVRD_ENR {
        SY_DIVN_CAL_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Override value for SY_DIVN_CAL_EN"]
    #[inline]
    pub fn sy_divn_cal_en_ovrd(&self) -> SY_DIVN_CAL_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SY_DIVN_CAL_EN_OVRDR { bits }
    }
    #[doc = "Bit 18 - Override control for RX_MIXER_EN"]
    #[inline]
    pub fn rx_mixer_en_ovrd_en(&self) -> RX_MIXER_EN_OVRD_ENR {
        RX_MIXER_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Override value for RX_MIXER_EN"]
    #[inline]
    pub fn rx_mixer_en_ovrd(&self) -> RX_MIXER_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_MIXER_EN_OVRDR { bits }
    }
    #[doc = "Bit 20 - Override control for TX_PA_EN"]
    #[inline]
    pub fn tx_pa_en_ovrd_en(&self) -> TX_PA_EN_OVRD_ENR {
        TX_PA_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Override value for TX_PA_EN"]
    #[inline]
    pub fn tx_pa_en_ovrd(&self) -> TX_PA_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_PA_EN_OVRDR { bits }
    }
    #[doc = "Bit 22 - Override control for RX_ADC_I_EN"]
    #[inline]
    pub fn rx_adc_i_en_ovrd_en(&self) -> RX_ADC_I_EN_OVRD_ENR {
        RX_ADC_I_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Override value for RX_ADC_I_EN"]
    #[inline]
    pub fn rx_adc_i_en_ovrd(&self) -> RX_ADC_I_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_ADC_I_EN_OVRDR { bits }
    }
    #[doc = "Bit 24 - Override control for RX_ADC_Q_EN"]
    #[inline]
    pub fn rx_adc_q_en_ovrd_en(&self) -> RX_ADC_Q_EN_OVRD_ENR {
        RX_ADC_Q_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Override value for RX_ADC_Q_EN"]
    #[inline]
    pub fn rx_adc_q_en_ovrd(&self) -> RX_ADC_Q_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_ADC_Q_EN_OVRDR { bits }
    }
    #[doc = "Bit 26 - Override control for RX_ADC_RESET_EN"]
    #[inline]
    pub fn rx_adc_reset_en_ovrd_en(&self) -> RX_ADC_RESET_EN_OVRD_ENR {
        RX_ADC_RESET_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Override value for RX_ADC_RESET_EN"]
    #[inline]
    pub fn rx_adc_reset_en_ovrd(&self) -> RX_ADC_RESET_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_ADC_RESET_EN_OVRDR { bits }
    }
    #[doc = "Bit 28 - Override control for RX_BBA_I_EN"]
    #[inline]
    pub fn rx_bba_i_en_ovrd_en(&self) -> RX_BBA_I_EN_OVRD_ENR {
        RX_BBA_I_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Override value for RX_BBA_I_EN"]
    #[inline]
    pub fn rx_bba_i_en_ovrd(&self) -> RX_BBA_I_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_BBA_I_EN_OVRDR { bits }
    }
    #[doc = "Bit 30 - Override control for RX_BBA_Q_EN"]
    #[inline]
    pub fn rx_bba_q_en_ovrd_en(&self) -> RX_BBA_Q_EN_OVRD_ENR {
        RX_BBA_Q_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Override value for RX_BBA_Q_EN"]
    #[inline]
    pub fn rx_bba_q_en_ovrd(&self) -> RX_BBA_Q_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_BBA_Q_EN_OVRDR { bits }
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
    #[doc = "Bit 0 - Override control for SY_LO_RX_BUF_EN"]
    #[inline]
    pub fn sy_lo_rx_buf_en_ovrd_en(&mut self) -> _SY_LO_RX_BUF_EN_OVRD_ENW {
        _SY_LO_RX_BUF_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 1 - Override value for SY_LO_RX_BUF_EN"]
    #[inline]
    pub fn sy_lo_rx_buf_en_ovrd(&mut self) -> _SY_LO_RX_BUF_EN_OVRDW {
        _SY_LO_RX_BUF_EN_OVRDW { w: self }
    }
    #[doc = "Bit 2 - Override control for SY_LO_TX_BUF_EN"]
    #[inline]
    pub fn sy_lo_tx_buf_en_ovrd_en(&mut self) -> _SY_LO_TX_BUF_EN_OVRD_ENW {
        _SY_LO_TX_BUF_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 3 - Override value for SY_LO_TX_BUF_EN"]
    #[inline]
    pub fn sy_lo_tx_buf_en_ovrd(&mut self) -> _SY_LO_TX_BUF_EN_OVRDW {
        _SY_LO_TX_BUF_EN_OVRDW { w: self }
    }
    #[doc = "Bit 4 - Override control for SY_DIVN_EN"]
    #[inline]
    pub fn sy_divn_en_ovrd_en(&mut self) -> _SY_DIVN_EN_OVRD_ENW {
        _SY_DIVN_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 5 - Override value for SY_DIVN_EN"]
    #[inline]
    pub fn sy_divn_en_ovrd(&mut self) -> _SY_DIVN_EN_OVRDW {
        _SY_DIVN_EN_OVRDW { w: self }
    }
    #[doc = "Bit 6 - Override control for SY_PD_FILTER_CHARGE_EN"]
    #[inline]
    pub fn sy_pd_filter_charge_en_ovrd_en(&mut self) -> _SY_PD_FILTER_CHARGE_EN_OVRD_ENW {
        _SY_PD_FILTER_CHARGE_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 7 - Override value for SY_PD_FILTER_CHARGE_EN"]
    #[inline]
    pub fn sy_pd_filter_charge_en_ovrd(&mut self) -> _SY_PD_FILTER_CHARGE_EN_OVRDW {
        _SY_PD_FILTER_CHARGE_EN_OVRDW { w: self }
    }
    #[doc = "Bit 8 - Override control for SY_PD_EN"]
    #[inline]
    pub fn sy_pd_en_ovrd_en(&mut self) -> _SY_PD_EN_OVRD_ENW {
        _SY_PD_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 9 - Override value for SY_PD_EN"]
    #[inline]
    pub fn sy_pd_en_ovrd(&mut self) -> _SY_PD_EN_OVRDW {
        _SY_PD_EN_OVRDW { w: self }
    }
    #[doc = "Bit 10 - Override control for SY_LO_DIVN_EN"]
    #[inline]
    pub fn sy_lo_divn_en_ovrd_en(&mut self) -> _SY_LO_DIVN_EN_OVRD_ENW {
        _SY_LO_DIVN_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 11 - Override value for SY_LO_DIVN_EN"]
    #[inline]
    pub fn sy_lo_divn_en_ovrd(&mut self) -> _SY_LO_DIVN_EN_OVRDW {
        _SY_LO_DIVN_EN_OVRDW { w: self }
    }
    #[doc = "Bit 12 - Override control for SY_LO_RX_EN"]
    #[inline]
    pub fn sy_lo_rx_en_ovrd_en(&mut self) -> _SY_LO_RX_EN_OVRD_ENW {
        _SY_LO_RX_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 13 - Override value for SY_LO_RX_EN"]
    #[inline]
    pub fn sy_lo_rx_en_ovrd(&mut self) -> _SY_LO_RX_EN_OVRDW {
        _SY_LO_RX_EN_OVRDW { w: self }
    }
    #[doc = "Bit 14 - Override control for SY_LO_TX_EN"]
    #[inline]
    pub fn sy_lo_tx_en_ovrd_en(&mut self) -> _SY_LO_TX_EN_OVRD_ENW {
        _SY_LO_TX_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 15 - Override value for SY_LO_TX_EN"]
    #[inline]
    pub fn sy_lo_tx_en_ovrd(&mut self) -> _SY_LO_TX_EN_OVRDW {
        _SY_LO_TX_EN_OVRDW { w: self }
    }
    #[doc = "Bit 16 - Override control for SY_DIVN_CAL_EN"]
    #[inline]
    pub fn sy_divn_cal_en_ovrd_en(&mut self) -> _SY_DIVN_CAL_EN_OVRD_ENW {
        _SY_DIVN_CAL_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 17 - Override value for SY_DIVN_CAL_EN"]
    #[inline]
    pub fn sy_divn_cal_en_ovrd(&mut self) -> _SY_DIVN_CAL_EN_OVRDW {
        _SY_DIVN_CAL_EN_OVRDW { w: self }
    }
    #[doc = "Bit 18 - Override control for RX_MIXER_EN"]
    #[inline]
    pub fn rx_mixer_en_ovrd_en(&mut self) -> _RX_MIXER_EN_OVRD_ENW {
        _RX_MIXER_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 19 - Override value for RX_MIXER_EN"]
    #[inline]
    pub fn rx_mixer_en_ovrd(&mut self) -> _RX_MIXER_EN_OVRDW {
        _RX_MIXER_EN_OVRDW { w: self }
    }
    #[doc = "Bit 20 - Override control for TX_PA_EN"]
    #[inline]
    pub fn tx_pa_en_ovrd_en(&mut self) -> _TX_PA_EN_OVRD_ENW {
        _TX_PA_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 21 - Override value for TX_PA_EN"]
    #[inline]
    pub fn tx_pa_en_ovrd(&mut self) -> _TX_PA_EN_OVRDW {
        _TX_PA_EN_OVRDW { w: self }
    }
    #[doc = "Bit 22 - Override control for RX_ADC_I_EN"]
    #[inline]
    pub fn rx_adc_i_en_ovrd_en(&mut self) -> _RX_ADC_I_EN_OVRD_ENW {
        _RX_ADC_I_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 23 - Override value for RX_ADC_I_EN"]
    #[inline]
    pub fn rx_adc_i_en_ovrd(&mut self) -> _RX_ADC_I_EN_OVRDW {
        _RX_ADC_I_EN_OVRDW { w: self }
    }
    #[doc = "Bit 24 - Override control for RX_ADC_Q_EN"]
    #[inline]
    pub fn rx_adc_q_en_ovrd_en(&mut self) -> _RX_ADC_Q_EN_OVRD_ENW {
        _RX_ADC_Q_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 25 - Override value for RX_ADC_Q_EN"]
    #[inline]
    pub fn rx_adc_q_en_ovrd(&mut self) -> _RX_ADC_Q_EN_OVRDW {
        _RX_ADC_Q_EN_OVRDW { w: self }
    }
    #[doc = "Bit 26 - Override control for RX_ADC_RESET_EN"]
    #[inline]
    pub fn rx_adc_reset_en_ovrd_en(&mut self) -> _RX_ADC_RESET_EN_OVRD_ENW {
        _RX_ADC_RESET_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 27 - Override value for RX_ADC_RESET_EN"]
    #[inline]
    pub fn rx_adc_reset_en_ovrd(&mut self) -> _RX_ADC_RESET_EN_OVRDW {
        _RX_ADC_RESET_EN_OVRDW { w: self }
    }
    #[doc = "Bit 28 - Override control for RX_BBA_I_EN"]
    #[inline]
    pub fn rx_bba_i_en_ovrd_en(&mut self) -> _RX_BBA_I_EN_OVRD_ENW {
        _RX_BBA_I_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 29 - Override value for RX_BBA_I_EN"]
    #[inline]
    pub fn rx_bba_i_en_ovrd(&mut self) -> _RX_BBA_I_EN_OVRDW {
        _RX_BBA_I_EN_OVRDW { w: self }
    }
    #[doc = "Bit 30 - Override control for RX_BBA_Q_EN"]
    #[inline]
    pub fn rx_bba_q_en_ovrd_en(&mut self) -> _RX_BBA_Q_EN_OVRD_ENW {
        _RX_BBA_Q_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 31 - Override value for RX_BBA_Q_EN"]
    #[inline]
    pub fn rx_bba_q_en_ovrd(&mut self) -> _RX_BBA_Q_EN_OVRDW {
        _RX_BBA_Q_EN_OVRDW { w: self }
    }
}
