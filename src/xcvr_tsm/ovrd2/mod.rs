#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OVRD2 {
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
#[doc = "Possible values of the field `RX_BBA_PDET_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BBA_PDET_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_BBA_PDET_EN_OVRD to override the signal \"rx_bba_pdet_en\"."]
    _1,
}
impl RX_BBA_PDET_EN_OVRD_ENR {
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
            RX_BBA_PDET_EN_OVRD_ENR::_0 => false,
            RX_BBA_PDET_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_BBA_PDET_EN_OVRD_ENR {
        match value {
            false => RX_BBA_PDET_EN_OVRD_ENR::_0,
            true => RX_BBA_PDET_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_BBA_PDET_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_BBA_PDET_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_BBA_PDET_EN_OVRDR {
    bits: bool,
}
impl RX_BBA_PDET_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_BBA_DCOC_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BBA_DCOC_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_BBA_DCOC_EN_OVRD to override the signal \"rx_bba_dcoc_en\"."]
    _1,
}
impl RX_BBA_DCOC_EN_OVRD_ENR {
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
            RX_BBA_DCOC_EN_OVRD_ENR::_0 => false,
            RX_BBA_DCOC_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_BBA_DCOC_EN_OVRD_ENR {
        match value {
            false => RX_BBA_DCOC_EN_OVRD_ENR::_0,
            true => RX_BBA_DCOC_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_BBA_DCOC_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_BBA_DCOC_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_BBA_DCOC_EN_OVRDR {
    bits: bool,
}
impl RX_BBA_DCOC_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_LNA_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_LNA_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_LNA_EN_OVRD to override the signal \"rx_lna_en\"."]
    _1,
}
impl RX_LNA_EN_OVRD_ENR {
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
            RX_LNA_EN_OVRD_ENR::_0 => false,
            RX_LNA_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_LNA_EN_OVRD_ENR {
        match value {
            false => RX_LNA_EN_OVRD_ENR::_0,
            true => RX_LNA_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_LNA_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_LNA_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_LNA_EN_OVRDR {
    bits: bool,
}
impl RX_LNA_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_TZA_I_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_TZA_I_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_TZA_I_EN_OVRD to override the signal \"rx_tza_i_en\"."]
    _1,
}
impl RX_TZA_I_EN_OVRD_ENR {
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
            RX_TZA_I_EN_OVRD_ENR::_0 => false,
            RX_TZA_I_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_TZA_I_EN_OVRD_ENR {
        match value {
            false => RX_TZA_I_EN_OVRD_ENR::_0,
            true => RX_TZA_I_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_TZA_I_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_TZA_I_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_TZA_I_EN_OVRDR {
    bits: bool,
}
impl RX_TZA_I_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_TZA_Q_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_TZA_Q_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_TZA_Q_EN_OVRD to override the signal \"rx_tza_q_en\"."]
    _1,
}
impl RX_TZA_Q_EN_OVRD_ENR {
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
            RX_TZA_Q_EN_OVRD_ENR::_0 => false,
            RX_TZA_Q_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_TZA_Q_EN_OVRD_ENR {
        match value {
            false => RX_TZA_Q_EN_OVRD_ENR::_0,
            true => RX_TZA_Q_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_TZA_Q_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_TZA_Q_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_TZA_Q_EN_OVRDR {
    bits: bool,
}
impl RX_TZA_Q_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_TZA_PDET_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_TZA_PDET_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_TZA_PDET_EN_OVRD to override the signal \"rx_tza_pdet_en\"."]
    _1,
}
impl RX_TZA_PDET_EN_OVRD_ENR {
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
            RX_TZA_PDET_EN_OVRD_ENR::_0 => false,
            RX_TZA_PDET_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_TZA_PDET_EN_OVRD_ENR {
        match value {
            false => RX_TZA_PDET_EN_OVRD_ENR::_0,
            true => RX_TZA_PDET_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_TZA_PDET_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_TZA_PDET_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_TZA_PDET_EN_OVRDR {
    bits: bool,
}
impl RX_TZA_PDET_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_TZA_DCOC_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_TZA_DCOC_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_TZA_DCOC_EN_OVRD to override the signal \"rx_tza_dcoc_en\"."]
    _1,
}
impl RX_TZA_DCOC_EN_OVRD_ENR {
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
            RX_TZA_DCOC_EN_OVRD_ENR::_0 => false,
            RX_TZA_DCOC_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_TZA_DCOC_EN_OVRD_ENR {
        match value {
            false => RX_TZA_DCOC_EN_OVRD_ENR::_0,
            true => RX_TZA_DCOC_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_TZA_DCOC_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_TZA_DCOC_EN_OVRD_ENR::_1
    }
}
#[doc = "Possible values of the field `RX_TZA_DCOC_EN_OVRD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_TZA_DCOC_EN_OVRDR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_TZA_DCOC_EN_OVRD to override the signal \"rx_tza_dcoc_en\"."]
    _1,
}
impl RX_TZA_DCOC_EN_OVRDR {
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
            RX_TZA_DCOC_EN_OVRDR::_0 => false,
            RX_TZA_DCOC_EN_OVRDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_TZA_DCOC_EN_OVRDR {
        match value {
            false => RX_TZA_DCOC_EN_OVRDR::_0,
            true => RX_TZA_DCOC_EN_OVRDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_TZA_DCOC_EN_OVRDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_TZA_DCOC_EN_OVRDR::_1
    }
}
#[doc = "Possible values of the field `PLL_DIG_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_DIG_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of PLL_DIG_EN_OVRD to override the signal \"pll_dig_en\"."]
    _1,
}
impl PLL_DIG_EN_OVRD_ENR {
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
            PLL_DIG_EN_OVRD_ENR::_0 => false,
            PLL_DIG_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLL_DIG_EN_OVRD_ENR {
        match value {
            false => PLL_DIG_EN_OVRD_ENR::_0,
            true => PLL_DIG_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLL_DIG_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLL_DIG_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct PLL_DIG_EN_OVRDR {
    bits: bool,
}
impl PLL_DIG_EN_OVRDR {
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
#[doc = "Possible values of the field `TX_DIG_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DIG_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TX_DIG_EN_OVRD to override the signal \"tx_dig_en\"."]
    _1,
}
impl TX_DIG_EN_OVRD_ENR {
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
            TX_DIG_EN_OVRD_ENR::_0 => false,
            TX_DIG_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_DIG_EN_OVRD_ENR {
        match value {
            false => TX_DIG_EN_OVRD_ENR::_0,
            true => TX_DIG_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_DIG_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_DIG_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TX_DIG_EN_OVRDR {
    bits: bool,
}
impl TX_DIG_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_DIG_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DIG_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_DIG_EN_OVRD to override the signal \"rx_dig_en\"."]
    _1,
}
impl RX_DIG_EN_OVRD_ENR {
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
            RX_DIG_EN_OVRD_ENR::_0 => false,
            RX_DIG_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_DIG_EN_OVRD_ENR {
        match value {
            false => RX_DIG_EN_OVRD_ENR::_0,
            true => RX_DIG_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_DIG_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_DIG_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_DIG_EN_OVRDR {
    bits: bool,
}
impl RX_DIG_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_INIT_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_INIT_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_INIT_OVRD to override the signal \"rx_init\"."]
    _1,
}
impl RX_INIT_OVRD_ENR {
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
            RX_INIT_OVRD_ENR::_0 => false,
            RX_INIT_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_INIT_OVRD_ENR {
        match value {
            false => RX_INIT_OVRD_ENR::_0,
            true => RX_INIT_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_INIT_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_INIT_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_INIT_OVRDR {
    bits: bool,
}
impl RX_INIT_OVRDR {
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
#[doc = "Possible values of the field `SIGMA_DELTA_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGMA_DELTA_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SIGMA_DELTA_EN_OVRD to override the signal \"sigma_delta_en\"."]
    _1,
}
impl SIGMA_DELTA_EN_OVRD_ENR {
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
            SIGMA_DELTA_EN_OVRD_ENR::_0 => false,
            SIGMA_DELTA_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIGMA_DELTA_EN_OVRD_ENR {
        match value {
            false => SIGMA_DELTA_EN_OVRD_ENR::_0,
            true => SIGMA_DELTA_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SIGMA_DELTA_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SIGMA_DELTA_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SIGMA_DELTA_EN_OVRDR {
    bits: bool,
}
impl SIGMA_DELTA_EN_OVRDR {
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
#[doc = "Possible values of the field `RX_PHY_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_PHY_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_PHY_EN_OVRD to override the signal \"rx_phy_en\"."]
    _1,
}
impl RX_PHY_EN_OVRD_ENR {
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
            RX_PHY_EN_OVRD_ENR::_0 => false,
            RX_PHY_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_PHY_EN_OVRD_ENR {
        match value {
            false => RX_PHY_EN_OVRD_ENR::_0,
            true => RX_PHY_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_PHY_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_PHY_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_PHY_EN_OVRDR {
    bits: bool,
}
impl RX_PHY_EN_OVRDR {
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
#[doc = "Possible values of the field `DCOC_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of DCOC_EN_OVRD to override the signal \"dcoc_en\"."]
    _1,
}
impl DCOC_EN_OVRD_ENR {
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
            DCOC_EN_OVRD_ENR::_0 => false,
            DCOC_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCOC_EN_OVRD_ENR {
        match value {
            false => DCOC_EN_OVRD_ENR::_0,
            true => DCOC_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCOC_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCOC_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_EN_OVRDR {
    bits: bool,
}
impl DCOC_EN_OVRDR {
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
#[doc = "Possible values of the field `DCOC_INIT_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_INIT_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of DCOC_INIT_OVRD to override the signal \"dcoc_init\"."]
    _1,
}
impl DCOC_INIT_OVRD_ENR {
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
            DCOC_INIT_OVRD_ENR::_0 => false,
            DCOC_INIT_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCOC_INIT_OVRD_ENR {
        match value {
            false => DCOC_INIT_OVRD_ENR::_0,
            true => DCOC_INIT_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCOC_INIT_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCOC_INIT_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_INIT_OVRDR {
    bits: bool,
}
impl DCOC_INIT_OVRDR {
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
#[doc = "Possible values of the field `FREQ_TARG_LD_EN_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQ_TARG_LD_EN_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of FREQ_TARG_LD_EN_OVRD to override the signal \"freq_targ_ld_en\"."]
    _1,
}
impl FREQ_TARG_LD_EN_OVRD_ENR {
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
            FREQ_TARG_LD_EN_OVRD_ENR::_0 => false,
            FREQ_TARG_LD_EN_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FREQ_TARG_LD_EN_OVRD_ENR {
        match value {
            false => FREQ_TARG_LD_EN_OVRD_ENR::_0,
            true => FREQ_TARG_LD_EN_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FREQ_TARG_LD_EN_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FREQ_TARG_LD_EN_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct FREQ_TARG_LD_EN_OVRDR {
    bits: bool,
}
impl FREQ_TARG_LD_EN_OVRDR {
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
#[doc = "Values that can be written to the field `RX_BBA_PDET_EN_OVRD_EN`"]
pub enum RX_BBA_PDET_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_BBA_PDET_EN_OVRD to override the signal \"rx_bba_pdet_en\"."]
    _1,
}
impl RX_BBA_PDET_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_BBA_PDET_EN_OVRD_ENW::_0 => false,
            RX_BBA_PDET_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_BBA_PDET_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_PDET_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_BBA_PDET_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_BBA_PDET_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_BBA_PDET_EN_OVRD to override the signal \"rx_bba_pdet_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_BBA_PDET_EN_OVRD_ENW::_1)
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
pub struct _RX_BBA_PDET_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_PDET_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_BBA_DCOC_EN_OVRD_EN`"]
pub enum RX_BBA_DCOC_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_BBA_DCOC_EN_OVRD to override the signal \"rx_bba_dcoc_en\"."]
    _1,
}
impl RX_BBA_DCOC_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_BBA_DCOC_EN_OVRD_ENW::_0 => false,
            RX_BBA_DCOC_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_BBA_DCOC_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_DCOC_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_BBA_DCOC_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_BBA_DCOC_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_BBA_DCOC_EN_OVRD to override the signal \"rx_bba_dcoc_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_BBA_DCOC_EN_OVRD_ENW::_1)
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
pub struct _RX_BBA_DCOC_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BBA_DCOC_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_LNA_EN_OVRD_EN`"]
pub enum RX_LNA_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_LNA_EN_OVRD to override the signal \"rx_lna_en\"."]
    _1,
}
impl RX_LNA_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_LNA_EN_OVRD_ENW::_0 => false,
            RX_LNA_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_LNA_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_LNA_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_LNA_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_LNA_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_LNA_EN_OVRD to override the signal \"rx_lna_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_LNA_EN_OVRD_ENW::_1)
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
pub struct _RX_LNA_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_LNA_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_TZA_I_EN_OVRD_EN`"]
pub enum RX_TZA_I_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_TZA_I_EN_OVRD to override the signal \"rx_tza_i_en\"."]
    _1,
}
impl RX_TZA_I_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_TZA_I_EN_OVRD_ENW::_0 => false,
            RX_TZA_I_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_TZA_I_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_TZA_I_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_TZA_I_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_TZA_I_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_TZA_I_EN_OVRD to override the signal \"rx_tza_i_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_TZA_I_EN_OVRD_ENW::_1)
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
pub struct _RX_TZA_I_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_TZA_I_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_TZA_Q_EN_OVRD_EN`"]
pub enum RX_TZA_Q_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_TZA_Q_EN_OVRD to override the signal \"rx_tza_q_en\"."]
    _1,
}
impl RX_TZA_Q_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_TZA_Q_EN_OVRD_ENW::_0 => false,
            RX_TZA_Q_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_TZA_Q_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_TZA_Q_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_TZA_Q_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_TZA_Q_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_TZA_Q_EN_OVRD to override the signal \"rx_tza_q_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_TZA_Q_EN_OVRD_ENW::_1)
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
pub struct _RX_TZA_Q_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_TZA_Q_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_TZA_PDET_EN_OVRD_EN`"]
pub enum RX_TZA_PDET_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_TZA_PDET_EN_OVRD to override the signal \"rx_tza_pdet_en\"."]
    _1,
}
impl RX_TZA_PDET_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_TZA_PDET_EN_OVRD_ENW::_0 => false,
            RX_TZA_PDET_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_TZA_PDET_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_TZA_PDET_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_TZA_PDET_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_TZA_PDET_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_TZA_PDET_EN_OVRD to override the signal \"rx_tza_pdet_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_TZA_PDET_EN_OVRD_ENW::_1)
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
pub struct _RX_TZA_PDET_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_TZA_PDET_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_TZA_DCOC_EN_OVRD_EN`"]
pub enum RX_TZA_DCOC_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_TZA_DCOC_EN_OVRD to override the signal \"rx_tza_dcoc_en\"."]
    _1,
}
impl RX_TZA_DCOC_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_TZA_DCOC_EN_OVRD_ENW::_0 => false,
            RX_TZA_DCOC_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_TZA_DCOC_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_TZA_DCOC_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_TZA_DCOC_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_TZA_DCOC_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_TZA_DCOC_EN_OVRD to override the signal \"rx_tza_dcoc_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_TZA_DCOC_EN_OVRD_ENW::_1)
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
#[doc = "Values that can be written to the field `RX_TZA_DCOC_EN_OVRD`"]
pub enum RX_TZA_DCOC_EN_OVRDW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_TZA_DCOC_EN_OVRD to override the signal \"rx_tza_dcoc_en\"."]
    _1,
}
impl RX_TZA_DCOC_EN_OVRDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_TZA_DCOC_EN_OVRDW::_0 => false,
            RX_TZA_DCOC_EN_OVRDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_TZA_DCOC_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_TZA_DCOC_EN_OVRDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_TZA_DCOC_EN_OVRDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_TZA_DCOC_EN_OVRDW::_0)
    }
    #[doc = "Use the state of RX_TZA_DCOC_EN_OVRD to override the signal \"rx_tza_dcoc_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_TZA_DCOC_EN_OVRDW::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLL_DIG_EN_OVRD_EN`"]
pub enum PLL_DIG_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of PLL_DIG_EN_OVRD to override the signal \"pll_dig_en\"."]
    _1,
}
impl PLL_DIG_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLL_DIG_EN_OVRD_ENW::_0 => false,
            PLL_DIG_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL_DIG_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIG_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL_DIG_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLL_DIG_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of PLL_DIG_EN_OVRD to override the signal \"pll_dig_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLL_DIG_EN_OVRD_ENW::_1)
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
pub struct _PLL_DIG_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIG_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `TX_DIG_EN_OVRD_EN`"]
pub enum TX_DIG_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of TX_DIG_EN_OVRD to override the signal \"tx_dig_en\"."]
    _1,
}
impl TX_DIG_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_DIG_EN_OVRD_ENW::_0 => false,
            TX_DIG_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_DIG_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DIG_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_DIG_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_DIG_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of TX_DIG_EN_OVRD to override the signal \"tx_dig_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_DIG_EN_OVRD_ENW::_1)
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
pub struct _TX_DIG_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DIG_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_DIG_EN_OVRD_EN`"]
pub enum RX_DIG_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_DIG_EN_OVRD to override the signal \"rx_dig_en\"."]
    _1,
}
impl RX_DIG_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_DIG_EN_OVRD_ENW::_0 => false,
            RX_DIG_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_DIG_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DIG_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_DIG_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_DIG_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_DIG_EN_OVRD to override the signal \"rx_dig_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_DIG_EN_OVRD_ENW::_1)
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
pub struct _RX_DIG_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DIG_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_INIT_OVRD_EN`"]
pub enum RX_INIT_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_INIT_OVRD to override the signal \"rx_init\"."]
    _1,
}
impl RX_INIT_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_INIT_OVRD_ENW::_0 => false,
            RX_INIT_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_INIT_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_INIT_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_INIT_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_INIT_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_INIT_OVRD to override the signal \"rx_init\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_INIT_OVRD_ENW::_1)
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
pub struct _RX_INIT_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_INIT_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `SIGMA_DELTA_EN_OVRD_EN`"]
pub enum SIGMA_DELTA_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of SIGMA_DELTA_EN_OVRD to override the signal \"sigma_delta_en\"."]
    _1,
}
impl SIGMA_DELTA_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIGMA_DELTA_EN_OVRD_ENW::_0 => false,
            SIGMA_DELTA_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIGMA_DELTA_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGMA_DELTA_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIGMA_DELTA_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIGMA_DELTA_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of SIGMA_DELTA_EN_OVRD to override the signal \"sigma_delta_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIGMA_DELTA_EN_OVRD_ENW::_1)
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
pub struct _SIGMA_DELTA_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGMA_DELTA_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RX_PHY_EN_OVRD_EN`"]
pub enum RX_PHY_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RX_PHY_EN_OVRD to override the signal \"rx_phy_en\"."]
    _1,
}
impl RX_PHY_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_PHY_EN_OVRD_ENW::_0 => false,
            RX_PHY_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_PHY_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_PHY_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_PHY_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_PHY_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RX_PHY_EN_OVRD to override the signal \"rx_phy_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_PHY_EN_OVRD_ENW::_1)
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
pub struct _RX_PHY_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_PHY_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `DCOC_EN_OVRD_EN`"]
pub enum DCOC_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of DCOC_EN_OVRD to override the signal \"dcoc_en\"."]
    _1,
}
impl DCOC_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCOC_EN_OVRD_ENW::_0 => false,
            DCOC_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCOC_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of DCOC_EN_OVRD to override the signal \"dcoc_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCOC_EN_OVRD_ENW::_1)
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
pub struct _DCOC_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_EN_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `DCOC_INIT_OVRD_EN`"]
pub enum DCOC_INIT_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of DCOC_INIT_OVRD to override the signal \"dcoc_init\"."]
    _1,
}
impl DCOC_INIT_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCOC_INIT_OVRD_ENW::_0 => false,
            DCOC_INIT_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_INIT_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_INIT_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_INIT_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCOC_INIT_OVRD_ENW::_0)
    }
    #[doc = "Use the state of DCOC_INIT_OVRD to override the signal \"dcoc_init\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCOC_INIT_OVRD_ENW::_1)
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
pub struct _DCOC_INIT_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_INIT_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `FREQ_TARG_LD_EN_OVRD_EN`"]
pub enum FREQ_TARG_LD_EN_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of FREQ_TARG_LD_EN_OVRD to override the signal \"freq_targ_ld_en\"."]
    _1,
}
impl FREQ_TARG_LD_EN_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FREQ_TARG_LD_EN_OVRD_ENW::_0 => false,
            FREQ_TARG_LD_EN_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREQ_TARG_LD_EN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQ_TARG_LD_EN_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREQ_TARG_LD_EN_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FREQ_TARG_LD_EN_OVRD_ENW::_0)
    }
    #[doc = "Use the state of FREQ_TARG_LD_EN_OVRD to override the signal \"freq_targ_ld_en\"."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FREQ_TARG_LD_EN_OVRD_ENW::_1)
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
pub struct _FREQ_TARG_LD_EN_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQ_TARG_LD_EN_OVRDW<'a> {
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
    #[doc = "Bit 0 - Override control for RX_BBA_PDET_EN"]
    #[inline]
    pub fn rx_bba_pdet_en_ovrd_en(&self) -> RX_BBA_PDET_EN_OVRD_ENR {
        RX_BBA_PDET_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Override value for RX_BBA_PDET_EN"]
    #[inline]
    pub fn rx_bba_pdet_en_ovrd(&self) -> RX_BBA_PDET_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_BBA_PDET_EN_OVRDR { bits }
    }
    #[doc = "Bit 2 - Override control for RX_BBA_DCOC_EN"]
    #[inline]
    pub fn rx_bba_dcoc_en_ovrd_en(&self) -> RX_BBA_DCOC_EN_OVRD_ENR {
        RX_BBA_DCOC_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Override value for RX_BBA_DCOC_EN"]
    #[inline]
    pub fn rx_bba_dcoc_en_ovrd(&self) -> RX_BBA_DCOC_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_BBA_DCOC_EN_OVRDR { bits }
    }
    #[doc = "Bit 4 - Override control for RX_LNA_EN"]
    #[inline]
    pub fn rx_lna_en_ovrd_en(&self) -> RX_LNA_EN_OVRD_ENR {
        RX_LNA_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Override value for RX_LNA_EN"]
    #[inline]
    pub fn rx_lna_en_ovrd(&self) -> RX_LNA_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_LNA_EN_OVRDR { bits }
    }
    #[doc = "Bit 6 - Override control for RX_TZA_I_EN"]
    #[inline]
    pub fn rx_tza_i_en_ovrd_en(&self) -> RX_TZA_I_EN_OVRD_ENR {
        RX_TZA_I_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Override value for RX_TZA_I_EN"]
    #[inline]
    pub fn rx_tza_i_en_ovrd(&self) -> RX_TZA_I_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_TZA_I_EN_OVRDR { bits }
    }
    #[doc = "Bit 8 - Override control for RX_TZA_Q_EN"]
    #[inline]
    pub fn rx_tza_q_en_ovrd_en(&self) -> RX_TZA_Q_EN_OVRD_ENR {
        RX_TZA_Q_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Override value for RX_TZA_Q_EN"]
    #[inline]
    pub fn rx_tza_q_en_ovrd(&self) -> RX_TZA_Q_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_TZA_Q_EN_OVRDR { bits }
    }
    #[doc = "Bit 10 - Override control for RX_TZA_PDET_EN"]
    #[inline]
    pub fn rx_tza_pdet_en_ovrd_en(&self) -> RX_TZA_PDET_EN_OVRD_ENR {
        RX_TZA_PDET_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Override value for RX_TZA_PDET_EN"]
    #[inline]
    pub fn rx_tza_pdet_en_ovrd(&self) -> RX_TZA_PDET_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_TZA_PDET_EN_OVRDR { bits }
    }
    #[doc = "Bit 12 - Override control for RX_TZA_DCOC_EN"]
    #[inline]
    pub fn rx_tza_dcoc_en_ovrd_en(&self) -> RX_TZA_DCOC_EN_OVRD_ENR {
        RX_TZA_DCOC_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Override control for RX_TZA_DCOC_EN"]
    #[inline]
    pub fn rx_tza_dcoc_en_ovrd(&self) -> RX_TZA_DCOC_EN_OVRDR {
        RX_TZA_DCOC_EN_OVRDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Override control for PLL_DIG_EN"]
    #[inline]
    pub fn pll_dig_en_ovrd_en(&self) -> PLL_DIG_EN_OVRD_ENR {
        PLL_DIG_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Override value for PLL_DIG_EN"]
    #[inline]
    pub fn pll_dig_en_ovrd(&self) -> PLL_DIG_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIG_EN_OVRDR { bits }
    }
    #[doc = "Bit 16 - Override control for TX_DIG_EN"]
    #[inline]
    pub fn tx_dig_en_ovrd_en(&self) -> TX_DIG_EN_OVRD_ENR {
        TX_DIG_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Override value for TX_DIG_EN"]
    #[inline]
    pub fn tx_dig_en_ovrd(&self) -> TX_DIG_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_DIG_EN_OVRDR { bits }
    }
    #[doc = "Bit 18 - Override control for RX_DIG_EN"]
    #[inline]
    pub fn rx_dig_en_ovrd_en(&self) -> RX_DIG_EN_OVRD_ENR {
        RX_DIG_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Override value for RX_DIG_EN"]
    #[inline]
    pub fn rx_dig_en_ovrd(&self) -> RX_DIG_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_DIG_EN_OVRDR { bits }
    }
    #[doc = "Bit 20 - Override control for RX_INIT"]
    #[inline]
    pub fn rx_init_ovrd_en(&self) -> RX_INIT_OVRD_ENR {
        RX_INIT_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Override value for RX_INIT"]
    #[inline]
    pub fn rx_init_ovrd(&self) -> RX_INIT_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_INIT_OVRDR { bits }
    }
    #[doc = "Bit 22 - Override control for SIGMA_DELTA_EN"]
    #[inline]
    pub fn sigma_delta_en_ovrd_en(&self) -> SIGMA_DELTA_EN_OVRD_ENR {
        SIGMA_DELTA_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Override value for SIGMA_DELTA_EN"]
    #[inline]
    pub fn sigma_delta_en_ovrd(&self) -> SIGMA_DELTA_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SIGMA_DELTA_EN_OVRDR { bits }
    }
    #[doc = "Bit 24 - Override control for RX_PHY_EN"]
    #[inline]
    pub fn rx_phy_en_ovrd_en(&self) -> RX_PHY_EN_OVRD_ENR {
        RX_PHY_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Override value for RX_PHY_EN"]
    #[inline]
    pub fn rx_phy_en_ovrd(&self) -> RX_PHY_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_PHY_EN_OVRDR { bits }
    }
    #[doc = "Bit 26 - Override control for DCOC_EN"]
    #[inline]
    pub fn dcoc_en_ovrd_en(&self) -> DCOC_EN_OVRD_ENR {
        DCOC_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Override value for DCOC_EN"]
    #[inline]
    pub fn dcoc_en_ovrd(&self) -> DCOC_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCOC_EN_OVRDR { bits }
    }
    #[doc = "Bit 28 - Override control for DCOC_INIT"]
    #[inline]
    pub fn dcoc_init_ovrd_en(&self) -> DCOC_INIT_OVRD_ENR {
        DCOC_INIT_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Override value for DCOC_INIT"]
    #[inline]
    pub fn dcoc_init_ovrd(&self) -> DCOC_INIT_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCOC_INIT_OVRDR { bits }
    }
    #[doc = "Bit 30 - Override control for FREQ_TARG_LD_EN"]
    #[inline]
    pub fn freq_targ_ld_en_ovrd_en(&self) -> FREQ_TARG_LD_EN_OVRD_ENR {
        FREQ_TARG_LD_EN_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Override value for FREQ_TARG_LD_EN"]
    #[inline]
    pub fn freq_targ_ld_en_ovrd(&self) -> FREQ_TARG_LD_EN_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FREQ_TARG_LD_EN_OVRDR { bits }
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
    #[doc = "Bit 0 - Override control for RX_BBA_PDET_EN"]
    #[inline]
    pub fn rx_bba_pdet_en_ovrd_en(&mut self) -> _RX_BBA_PDET_EN_OVRD_ENW {
        _RX_BBA_PDET_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 1 - Override value for RX_BBA_PDET_EN"]
    #[inline]
    pub fn rx_bba_pdet_en_ovrd(&mut self) -> _RX_BBA_PDET_EN_OVRDW {
        _RX_BBA_PDET_EN_OVRDW { w: self }
    }
    #[doc = "Bit 2 - Override control for RX_BBA_DCOC_EN"]
    #[inline]
    pub fn rx_bba_dcoc_en_ovrd_en(&mut self) -> _RX_BBA_DCOC_EN_OVRD_ENW {
        _RX_BBA_DCOC_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 3 - Override value for RX_BBA_DCOC_EN"]
    #[inline]
    pub fn rx_bba_dcoc_en_ovrd(&mut self) -> _RX_BBA_DCOC_EN_OVRDW {
        _RX_BBA_DCOC_EN_OVRDW { w: self }
    }
    #[doc = "Bit 4 - Override control for RX_LNA_EN"]
    #[inline]
    pub fn rx_lna_en_ovrd_en(&mut self) -> _RX_LNA_EN_OVRD_ENW {
        _RX_LNA_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 5 - Override value for RX_LNA_EN"]
    #[inline]
    pub fn rx_lna_en_ovrd(&mut self) -> _RX_LNA_EN_OVRDW {
        _RX_LNA_EN_OVRDW { w: self }
    }
    #[doc = "Bit 6 - Override control for RX_TZA_I_EN"]
    #[inline]
    pub fn rx_tza_i_en_ovrd_en(&mut self) -> _RX_TZA_I_EN_OVRD_ENW {
        _RX_TZA_I_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 7 - Override value for RX_TZA_I_EN"]
    #[inline]
    pub fn rx_tza_i_en_ovrd(&mut self) -> _RX_TZA_I_EN_OVRDW {
        _RX_TZA_I_EN_OVRDW { w: self }
    }
    #[doc = "Bit 8 - Override control for RX_TZA_Q_EN"]
    #[inline]
    pub fn rx_tza_q_en_ovrd_en(&mut self) -> _RX_TZA_Q_EN_OVRD_ENW {
        _RX_TZA_Q_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 9 - Override value for RX_TZA_Q_EN"]
    #[inline]
    pub fn rx_tza_q_en_ovrd(&mut self) -> _RX_TZA_Q_EN_OVRDW {
        _RX_TZA_Q_EN_OVRDW { w: self }
    }
    #[doc = "Bit 10 - Override control for RX_TZA_PDET_EN"]
    #[inline]
    pub fn rx_tza_pdet_en_ovrd_en(&mut self) -> _RX_TZA_PDET_EN_OVRD_ENW {
        _RX_TZA_PDET_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 11 - Override value for RX_TZA_PDET_EN"]
    #[inline]
    pub fn rx_tza_pdet_en_ovrd(&mut self) -> _RX_TZA_PDET_EN_OVRDW {
        _RX_TZA_PDET_EN_OVRDW { w: self }
    }
    #[doc = "Bit 12 - Override control for RX_TZA_DCOC_EN"]
    #[inline]
    pub fn rx_tza_dcoc_en_ovrd_en(&mut self) -> _RX_TZA_DCOC_EN_OVRD_ENW {
        _RX_TZA_DCOC_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 13 - Override control for RX_TZA_DCOC_EN"]
    #[inline]
    pub fn rx_tza_dcoc_en_ovrd(&mut self) -> _RX_TZA_DCOC_EN_OVRDW {
        _RX_TZA_DCOC_EN_OVRDW { w: self }
    }
    #[doc = "Bit 14 - Override control for PLL_DIG_EN"]
    #[inline]
    pub fn pll_dig_en_ovrd_en(&mut self) -> _PLL_DIG_EN_OVRD_ENW {
        _PLL_DIG_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 15 - Override value for PLL_DIG_EN"]
    #[inline]
    pub fn pll_dig_en_ovrd(&mut self) -> _PLL_DIG_EN_OVRDW {
        _PLL_DIG_EN_OVRDW { w: self }
    }
    #[doc = "Bit 16 - Override control for TX_DIG_EN"]
    #[inline]
    pub fn tx_dig_en_ovrd_en(&mut self) -> _TX_DIG_EN_OVRD_ENW {
        _TX_DIG_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 17 - Override value for TX_DIG_EN"]
    #[inline]
    pub fn tx_dig_en_ovrd(&mut self) -> _TX_DIG_EN_OVRDW {
        _TX_DIG_EN_OVRDW { w: self }
    }
    #[doc = "Bit 18 - Override control for RX_DIG_EN"]
    #[inline]
    pub fn rx_dig_en_ovrd_en(&mut self) -> _RX_DIG_EN_OVRD_ENW {
        _RX_DIG_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 19 - Override value for RX_DIG_EN"]
    #[inline]
    pub fn rx_dig_en_ovrd(&mut self) -> _RX_DIG_EN_OVRDW {
        _RX_DIG_EN_OVRDW { w: self }
    }
    #[doc = "Bit 20 - Override control for RX_INIT"]
    #[inline]
    pub fn rx_init_ovrd_en(&mut self) -> _RX_INIT_OVRD_ENW {
        _RX_INIT_OVRD_ENW { w: self }
    }
    #[doc = "Bit 21 - Override value for RX_INIT"]
    #[inline]
    pub fn rx_init_ovrd(&mut self) -> _RX_INIT_OVRDW {
        _RX_INIT_OVRDW { w: self }
    }
    #[doc = "Bit 22 - Override control for SIGMA_DELTA_EN"]
    #[inline]
    pub fn sigma_delta_en_ovrd_en(&mut self) -> _SIGMA_DELTA_EN_OVRD_ENW {
        _SIGMA_DELTA_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 23 - Override value for SIGMA_DELTA_EN"]
    #[inline]
    pub fn sigma_delta_en_ovrd(&mut self) -> _SIGMA_DELTA_EN_OVRDW {
        _SIGMA_DELTA_EN_OVRDW { w: self }
    }
    #[doc = "Bit 24 - Override control for RX_PHY_EN"]
    #[inline]
    pub fn rx_phy_en_ovrd_en(&mut self) -> _RX_PHY_EN_OVRD_ENW {
        _RX_PHY_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 25 - Override value for RX_PHY_EN"]
    #[inline]
    pub fn rx_phy_en_ovrd(&mut self) -> _RX_PHY_EN_OVRDW {
        _RX_PHY_EN_OVRDW { w: self }
    }
    #[doc = "Bit 26 - Override control for DCOC_EN"]
    #[inline]
    pub fn dcoc_en_ovrd_en(&mut self) -> _DCOC_EN_OVRD_ENW {
        _DCOC_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 27 - Override value for DCOC_EN"]
    #[inline]
    pub fn dcoc_en_ovrd(&mut self) -> _DCOC_EN_OVRDW {
        _DCOC_EN_OVRDW { w: self }
    }
    #[doc = "Bit 28 - Override control for DCOC_INIT"]
    #[inline]
    pub fn dcoc_init_ovrd_en(&mut self) -> _DCOC_INIT_OVRD_ENW {
        _DCOC_INIT_OVRD_ENW { w: self }
    }
    #[doc = "Bit 29 - Override value for DCOC_INIT"]
    #[inline]
    pub fn dcoc_init_ovrd(&mut self) -> _DCOC_INIT_OVRDW {
        _DCOC_INIT_OVRDW { w: self }
    }
    #[doc = "Bit 30 - Override control for FREQ_TARG_LD_EN"]
    #[inline]
    pub fn freq_targ_ld_en_ovrd_en(&mut self) -> _FREQ_TARG_LD_EN_OVRD_ENW {
        _FREQ_TARG_LD_EN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 31 - Override value for FREQ_TARG_LD_EN"]
    #[inline]
    pub fn freq_targ_ld_en_ovrd(&mut self) -> _FREQ_TARG_LD_EN_OVRDW {
        _FREQ_TARG_LD_EN_OVRDW { w: self }
    }
}
