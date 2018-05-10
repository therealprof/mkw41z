#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_DIG_CTRL {
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
#[doc = "Possible values of the field `RX_ADC_NEGEDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ADC_NEGEDGER {
    #[doc = "Register ADC data on positive edge of clock"]
    _0,
    #[doc = "Register ADC data on negative edge of clock"]
    _1,
}
impl RX_ADC_NEGEDGER {
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
            RX_ADC_NEGEDGER::_0 => false,
            RX_ADC_NEGEDGER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_ADC_NEGEDGER {
        match value {
            false => RX_ADC_NEGEDGER::_0,
            true => RX_ADC_NEGEDGER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_ADC_NEGEDGER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_ADC_NEGEDGER::_1
    }
}
#[doc = "Possible values of the field `RX_CH_FILT_BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_CH_FILT_BYPASSR {
    #[doc = "Channel filter is enabled."]
    _0,
    #[doc = "Disable and bypass channel filter."]
    _1,
}
impl RX_CH_FILT_BYPASSR {
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
            RX_CH_FILT_BYPASSR::_0 => false,
            RX_CH_FILT_BYPASSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_CH_FILT_BYPASSR {
        match value {
            false => RX_CH_FILT_BYPASSR::_0,
            true => RX_CH_FILT_BYPASSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_CH_FILT_BYPASSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_CH_FILT_BYPASSR::_1
    }
}
#[doc = "Possible values of the field `RX_ADC_RAW_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ADC_RAW_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "The decimation filter's 12bit output consists of ADC samples in the 8 LSBs. This is for test purposes only to observe ADC output via XCVR DMA or DTEST."]
    _1,
}
impl RX_ADC_RAW_ENR {
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
            RX_ADC_RAW_ENR::_0 => false,
            RX_ADC_RAW_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_ADC_RAW_ENR {
        match value {
            false => RX_ADC_RAW_ENR::_0,
            true => RX_ADC_RAW_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_ADC_RAW_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_ADC_RAW_ENR::_1
    }
}
#[doc = "Possible values of the field `RX_ADC_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ADC_POLR {
    #[doc = "ADC output of 1'b0 maps to -1, 1'b1 maps to +1 (default)"]
    _0,
    #[doc = "ADC output of 1'b0 maps to +1, 1'b1 maps to -1"]
    _1,
}
impl RX_ADC_POLR {
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
            RX_ADC_POLR::_0 => false,
            RX_ADC_POLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_ADC_POLR {
        match value {
            false => RX_ADC_POLR::_0,
            true => RX_ADC_POLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_ADC_POLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_ADC_POLR::_1
    }
}
#[doc = "Possible values of the field `RX_DEC_FILT_OSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DEC_FILT_OSRR {
    #[doc = "OSR 4"]
    _0,
    #[doc = "OSR 8"]
    _1,
    #[doc = "OSR 16"]
    _2,
    #[doc = "OSR 32"]
    _4,
    #[doc = "OSR 6"]
    _3,
    #[doc = "OSR 12"]
    _5,
    #[doc = "OSR 24"]
    _6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RX_DEC_FILT_OSRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_DEC_FILT_OSRR::_0 => 0,
            RX_DEC_FILT_OSRR::_1 => 1,
            RX_DEC_FILT_OSRR::_2 => 2,
            RX_DEC_FILT_OSRR::_4 => 4,
            RX_DEC_FILT_OSRR::_3 => 3,
            RX_DEC_FILT_OSRR::_5 => 5,
            RX_DEC_FILT_OSRR::_6 => 6,
            RX_DEC_FILT_OSRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_DEC_FILT_OSRR {
        match value {
            0 => RX_DEC_FILT_OSRR::_0,
            1 => RX_DEC_FILT_OSRR::_1,
            2 => RX_DEC_FILT_OSRR::_2,
            4 => RX_DEC_FILT_OSRR::_4,
            3 => RX_DEC_FILT_OSRR::_3,
            5 => RX_DEC_FILT_OSRR::_5,
            6 => RX_DEC_FILT_OSRR::_6,
            i => RX_DEC_FILT_OSRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_DEC_FILT_OSRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_DEC_FILT_OSRR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == RX_DEC_FILT_OSRR::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == RX_DEC_FILT_OSRR::_4
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == RX_DEC_FILT_OSRR::_3
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == RX_DEC_FILT_OSRR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == RX_DEC_FILT_OSRR::_6
    }
}
#[doc = "Possible values of the field `RX_FSK_ZB_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FSK_ZB_SELR {
    #[doc = "FSK demodulator."]
    _0,
    #[doc = "802.15.4 demodulator."]
    _1,
}
impl RX_FSK_ZB_SELR {
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
            RX_FSK_ZB_SELR::_0 => false,
            RX_FSK_ZB_SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_FSK_ZB_SELR {
        match value {
            false => RX_FSK_ZB_SELR::_0,
            true => RX_FSK_ZB_SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_FSK_ZB_SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_FSK_ZB_SELR::_1
    }
}
#[doc = "Possible values of the field `RX_NORM_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_NORM_ENR {
    #[doc = "Normalizer is disabled."]
    _0,
    #[doc = "Normalizer is enabled."]
    _1,
}
impl RX_NORM_ENR {
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
            RX_NORM_ENR::_0 => false,
            RX_NORM_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_NORM_ENR {
        match value {
            false => RX_NORM_ENR::_0,
            true => RX_NORM_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_NORM_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_NORM_ENR::_1
    }
}
#[doc = "Possible values of the field `RX_RSSI_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_RSSI_ENR {
    #[doc = "RSSI measurement is disabled."]
    _0,
    #[doc = "RSSI measurement is enabled."]
    _1,
}
impl RX_RSSI_ENR {
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
            RX_RSSI_ENR::_0 => false,
            RX_RSSI_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_RSSI_ENR {
        match value {
            false => RX_RSSI_ENR::_0,
            true => RX_RSSI_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_RSSI_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_RSSI_ENR::_1
    }
}
#[doc = "Possible values of the field `RX_AGC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_AGC_ENR {
    #[doc = "AGC is disabled."]
    _0,
    #[doc = "AGC is enabled."]
    _1,
}
impl RX_AGC_ENR {
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
            RX_AGC_ENR::_0 => false,
            RX_AGC_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_AGC_ENR {
        match value {
            false => RX_AGC_ENR::_0,
            true => RX_AGC_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_AGC_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_AGC_ENR::_1
    }
}
#[doc = "Possible values of the field `RX_DCOC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DCOC_ENR {
    #[doc = "DCOC is disabled."]
    _0,
    #[doc = "DCOC is enabled."]
    _1,
}
impl RX_DCOC_ENR {
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
            RX_DCOC_ENR::_0 => false,
            RX_DCOC_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_DCOC_ENR {
        match value {
            false => RX_DCOC_ENR::_0,
            true => RX_DCOC_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_DCOC_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_DCOC_ENR::_1
    }
}
#[doc = "Possible values of the field `RX_DCOC_CAL_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DCOC_CAL_ENR {
    #[doc = "DCOC calibration is disabled."]
    _0,
    #[doc = "DCOC calibration is enabled."]
    _1,
}
impl RX_DCOC_CAL_ENR {
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
            RX_DCOC_CAL_ENR::_0 => false,
            RX_DCOC_CAL_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_DCOC_CAL_ENR {
        match value {
            false => RX_DCOC_CAL_ENR::_0,
            true => RX_DCOC_CAL_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_DCOC_CAL_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_DCOC_CAL_ENR::_1
    }
}
#[doc = "Possible values of the field `RX_IQ_SWAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_IQ_SWAPR {
    #[doc = "IQ swap is disabled."]
    _0,
    #[doc = "IQ swap is enabled."]
    _1,
}
impl RX_IQ_SWAPR {
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
            RX_IQ_SWAPR::_0 => false,
            RX_IQ_SWAPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_IQ_SWAPR {
        match value {
            false => RX_IQ_SWAPR::_0,
            true => RX_IQ_SWAPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_IQ_SWAPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_IQ_SWAPR::_1
    }
}
#[doc = "Possible values of the field `RX_DC_RESID_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DC_RESID_ENR {
    #[doc = "DC Residual block is disabled."]
    _0,
    #[doc = "DC Residual block is enabled."]
    _1,
}
impl RX_DC_RESID_ENR {
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
            RX_DC_RESID_ENR::_0 => false,
            RX_DC_RESID_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_DC_RESID_ENR {
        match value {
            false => RX_DC_RESID_ENR::_0,
            true => RX_DC_RESID_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_DC_RESID_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_DC_RESID_ENR::_1
    }
}
#[doc = "Possible values of the field `RX_SRC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_SRC_ENR {
    #[doc = "SRC is disabled."]
    _0,
    #[doc = "SRC is enabled."]
    _1,
}
impl RX_SRC_ENR {
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
            RX_SRC_ENR::_0 => false,
            RX_SRC_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_SRC_ENR {
        match value {
            false => RX_SRC_ENR::_0,
            true => RX_SRC_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_SRC_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_SRC_ENR::_1
    }
}
#[doc = "Possible values of the field `RX_SRC_RATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_SRC_RATER {
    #[doc = "SRC is configured for a First Order Hold rate of 8/13."]
    _0,
    #[doc = "SRC is configured for a Zero Order Hold rate of 12/13."]
    _1,
}
impl RX_SRC_RATER {
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
            RX_SRC_RATER::_0 => false,
            RX_SRC_RATER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_SRC_RATER {
        match value {
            false => RX_SRC_RATER::_0,
            true => RX_SRC_RATER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_SRC_RATER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_SRC_RATER::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_DMA_DTEST_ENR {
    bits: bool,
}
impl RX_DMA_DTEST_ENR {
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
pub struct RX_DEC_FILT_GAINR {
    bits: u8,
}
impl RX_DEC_FILT_GAINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RX_DEC_FILT_HZD_CORR_DISR {
    bits: bool,
}
impl RX_DEC_FILT_HZD_CORR_DISR {
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
#[doc = "Possible values of the field `RX_DEC_FILT_HAZARD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DEC_FILT_HAZARDR {
    #[doc = "A hazard condition has not been detected"]
    _0,
    #[doc = "A hazard condition has been detected"]
    _1,
}
impl RX_DEC_FILT_HAZARDR {
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
            RX_DEC_FILT_HAZARDR::_0 => false,
            RX_DEC_FILT_HAZARDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_DEC_FILT_HAZARDR {
        match value {
            false => RX_DEC_FILT_HAZARDR::_0,
            true => RX_DEC_FILT_HAZARDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_DEC_FILT_HAZARDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_DEC_FILT_HAZARDR::_1
    }
}
#[doc = "Possible values of the field `RX_RSSI_FILT_HAZARD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_RSSI_FILT_HAZARDR {
    #[doc = "A hazard condition has not been detected"]
    _0,
    #[doc = "A hazard condition has been detected"]
    _1,
}
impl RX_RSSI_FILT_HAZARDR {
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
            RX_RSSI_FILT_HAZARDR::_0 => false,
            RX_RSSI_FILT_HAZARDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_RSSI_FILT_HAZARDR {
        match value {
            false => RX_RSSI_FILT_HAZARDR::_0,
            true => RX_RSSI_FILT_HAZARDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_RSSI_FILT_HAZARDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_RSSI_FILT_HAZARDR::_1
    }
}
#[doc = "Possible values of the field `RX_DEC_FILT_SAT_I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DEC_FILT_SAT_IR {
    #[doc = "A saturation condition has not occurred."]
    _0,
    #[doc = "A saturation condition has occurred."]
    _1,
}
impl RX_DEC_FILT_SAT_IR {
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
            RX_DEC_FILT_SAT_IR::_0 => false,
            RX_DEC_FILT_SAT_IR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_DEC_FILT_SAT_IR {
        match value {
            false => RX_DEC_FILT_SAT_IR::_0,
            true => RX_DEC_FILT_SAT_IR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_DEC_FILT_SAT_IR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_DEC_FILT_SAT_IR::_1
    }
}
#[doc = "Possible values of the field `RX_DEC_FILT_SAT_Q`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DEC_FILT_SAT_QR {
    #[doc = "A saturation condition has not occurred."]
    _0,
    #[doc = "A saturation condition has occurred."]
    _1,
}
impl RX_DEC_FILT_SAT_QR {
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
            RX_DEC_FILT_SAT_QR::_0 => false,
            RX_DEC_FILT_SAT_QR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_DEC_FILT_SAT_QR {
        match value {
            false => RX_DEC_FILT_SAT_QR::_0,
            true => RX_DEC_FILT_SAT_QR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_DEC_FILT_SAT_QR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_DEC_FILT_SAT_QR::_1
    }
}
#[doc = "Values that can be written to the field `RX_ADC_NEGEDGE`"]
pub enum RX_ADC_NEGEDGEW {
    #[doc = "Register ADC data on positive edge of clock"]
    _0,
    #[doc = "Register ADC data on negative edge of clock"]
    _1,
}
impl RX_ADC_NEGEDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_ADC_NEGEDGEW::_0 => false,
            RX_ADC_NEGEDGEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_ADC_NEGEDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_NEGEDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_ADC_NEGEDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Register ADC data on positive edge of clock"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_ADC_NEGEDGEW::_0)
    }
    #[doc = "Register ADC data on negative edge of clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_ADC_NEGEDGEW::_1)
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
#[doc = "Values that can be written to the field `RX_CH_FILT_BYPASS`"]
pub enum RX_CH_FILT_BYPASSW {
    #[doc = "Channel filter is enabled."]
    _0,
    #[doc = "Disable and bypass channel filter."]
    _1,
}
impl RX_CH_FILT_BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_CH_FILT_BYPASSW::_0 => false,
            RX_CH_FILT_BYPASSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_CH_FILT_BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_CH_FILT_BYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_CH_FILT_BYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel filter is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_CH_FILT_BYPASSW::_0)
    }
    #[doc = "Disable and bypass channel filter."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_CH_FILT_BYPASSW::_1)
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
#[doc = "Values that can be written to the field `RX_ADC_RAW_EN`"]
pub enum RX_ADC_RAW_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "The decimation filter's 12bit output consists of ADC samples in the 8 LSBs. This is for test purposes only to observe ADC output via XCVR DMA or DTEST."]
    _1,
}
impl RX_ADC_RAW_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_ADC_RAW_ENW::_0 => false,
            RX_ADC_RAW_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_ADC_RAW_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_RAW_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_ADC_RAW_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_ADC_RAW_ENW::_0)
    }
    #[doc = "The decimation filter's 12bit output consists of ADC samples in the 8 LSBs. This is for test purposes only to observe ADC output via XCVR DMA or DTEST."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_ADC_RAW_ENW::_1)
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
#[doc = "Values that can be written to the field `RX_ADC_POL`"]
pub enum RX_ADC_POLW {
    #[doc = "ADC output of 1'b0 maps to -1, 1'b1 maps to +1 (default)"]
    _0,
    #[doc = "ADC output of 1'b0 maps to +1, 1'b1 maps to -1"]
    _1,
}
impl RX_ADC_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_ADC_POLW::_0 => false,
            RX_ADC_POLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_ADC_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_ADC_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC output of 1'b0 maps to -1, 1'b1 maps to +1 (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_ADC_POLW::_0)
    }
    #[doc = "ADC output of 1'b0 maps to +1, 1'b1 maps to -1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_ADC_POLW::_1)
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
#[doc = "Values that can be written to the field `RX_DEC_FILT_OSR`"]
pub enum RX_DEC_FILT_OSRW {
    #[doc = "OSR 4"]
    _0,
    #[doc = "OSR 8"]
    _1,
    #[doc = "OSR 16"]
    _2,
    #[doc = "OSR 32"]
    _4,
    #[doc = "OSR 6"]
    _3,
    #[doc = "OSR 12"]
    _5,
    #[doc = "OSR 24"]
    _6,
}
impl RX_DEC_FILT_OSRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_DEC_FILT_OSRW::_0 => 0,
            RX_DEC_FILT_OSRW::_1 => 1,
            RX_DEC_FILT_OSRW::_2 => 2,
            RX_DEC_FILT_OSRW::_4 => 4,
            RX_DEC_FILT_OSRW::_3 => 3,
            RX_DEC_FILT_OSRW::_5 => 5,
            RX_DEC_FILT_OSRW::_6 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_DEC_FILT_OSRW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DEC_FILT_OSRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_DEC_FILT_OSRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "OSR 4"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_DEC_FILT_OSRW::_0)
    }
    #[doc = "OSR 8"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_DEC_FILT_OSRW::_1)
    }
    #[doc = "OSR 16"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(RX_DEC_FILT_OSRW::_2)
    }
    #[doc = "OSR 32"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(RX_DEC_FILT_OSRW::_4)
    }
    #[doc = "OSR 6"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(RX_DEC_FILT_OSRW::_3)
    }
    #[doc = "OSR 12"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(RX_DEC_FILT_OSRW::_5)
    }
    #[doc = "OSR 24"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(RX_DEC_FILT_OSRW::_6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_FSK_ZB_SEL`"]
pub enum RX_FSK_ZB_SELW {
    #[doc = "FSK demodulator."]
    _0,
    #[doc = "802.15.4 demodulator."]
    _1,
}
impl RX_FSK_ZB_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_FSK_ZB_SELW::_0 => false,
            RX_FSK_ZB_SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_FSK_ZB_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_FSK_ZB_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_FSK_ZB_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FSK demodulator."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_FSK_ZB_SELW::_0)
    }
    #[doc = "802.15.4 demodulator."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_FSK_ZB_SELW::_1)
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
#[doc = "Values that can be written to the field `RX_NORM_EN`"]
pub enum RX_NORM_ENW {
    #[doc = "Normalizer is disabled."]
    _0,
    #[doc = "Normalizer is enabled."]
    _1,
}
impl RX_NORM_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_NORM_ENW::_0 => false,
            RX_NORM_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_NORM_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_NORM_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_NORM_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normalizer is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_NORM_ENW::_0)
    }
    #[doc = "Normalizer is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_NORM_ENW::_1)
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
#[doc = "Values that can be written to the field `RX_RSSI_EN`"]
pub enum RX_RSSI_ENW {
    #[doc = "RSSI measurement is disabled."]
    _0,
    #[doc = "RSSI measurement is enabled."]
    _1,
}
impl RX_RSSI_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_RSSI_ENW::_0 => false,
            RX_RSSI_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_RSSI_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_RSSI_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_RSSI_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RSSI measurement is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_RSSI_ENW::_0)
    }
    #[doc = "RSSI measurement is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_RSSI_ENW::_1)
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
#[doc = "Values that can be written to the field `RX_AGC_EN`"]
pub enum RX_AGC_ENW {
    #[doc = "AGC is disabled."]
    _0,
    #[doc = "AGC is enabled."]
    _1,
}
impl RX_AGC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_AGC_ENW::_0 => false,
            RX_AGC_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_AGC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_AGC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_AGC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AGC is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_AGC_ENW::_0)
    }
    #[doc = "AGC is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_AGC_ENW::_1)
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
#[doc = "Values that can be written to the field `RX_DCOC_EN`"]
pub enum RX_DCOC_ENW {
    #[doc = "DCOC is disabled."]
    _0,
    #[doc = "DCOC is enabled."]
    _1,
}
impl RX_DCOC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_DCOC_ENW::_0 => false,
            RX_DCOC_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_DCOC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DCOC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_DCOC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DCOC is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_DCOC_ENW::_0)
    }
    #[doc = "DCOC is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_DCOC_ENW::_1)
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
#[doc = "Values that can be written to the field `RX_DCOC_CAL_EN`"]
pub enum RX_DCOC_CAL_ENW {
    #[doc = "DCOC calibration is disabled."]
    _0,
    #[doc = "DCOC calibration is enabled."]
    _1,
}
impl RX_DCOC_CAL_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_DCOC_CAL_ENW::_0 => false,
            RX_DCOC_CAL_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_DCOC_CAL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DCOC_CAL_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_DCOC_CAL_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DCOC calibration is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_DCOC_CAL_ENW::_0)
    }
    #[doc = "DCOC calibration is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_DCOC_CAL_ENW::_1)
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
#[doc = "Values that can be written to the field `RX_IQ_SWAP`"]
pub enum RX_IQ_SWAPW {
    #[doc = "IQ swap is disabled."]
    _0,
    #[doc = "IQ swap is enabled."]
    _1,
}
impl RX_IQ_SWAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_IQ_SWAPW::_0 => false,
            RX_IQ_SWAPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_IQ_SWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_IQ_SWAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_IQ_SWAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IQ swap is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_IQ_SWAPW::_0)
    }
    #[doc = "IQ swap is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_IQ_SWAPW::_1)
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
#[doc = "Values that can be written to the field `RX_DC_RESID_EN`"]
pub enum RX_DC_RESID_ENW {
    #[doc = "DC Residual block is disabled."]
    _0,
    #[doc = "DC Residual block is enabled."]
    _1,
}
impl RX_DC_RESID_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_DC_RESID_ENW::_0 => false,
            RX_DC_RESID_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_DC_RESID_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DC_RESID_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_DC_RESID_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DC Residual block is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_DC_RESID_ENW::_0)
    }
    #[doc = "DC Residual block is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_DC_RESID_ENW::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_SRC_EN`"]
pub enum RX_SRC_ENW {
    #[doc = "SRC is disabled."]
    _0,
    #[doc = "SRC is enabled."]
    _1,
}
impl RX_SRC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_SRC_ENW::_0 => false,
            RX_SRC_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_SRC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_SRC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_SRC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SRC is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_SRC_ENW::_0)
    }
    #[doc = "SRC is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_SRC_ENW::_1)
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
#[doc = "Values that can be written to the field `RX_SRC_RATE`"]
pub enum RX_SRC_RATEW {
    #[doc = "SRC is configured for a First Order Hold rate of 8/13."]
    _0,
    #[doc = "SRC is configured for a Zero Order Hold rate of 12/13."]
    _1,
}
impl RX_SRC_RATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_SRC_RATEW::_0 => false,
            RX_SRC_RATEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_SRC_RATEW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_SRC_RATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_SRC_RATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SRC is configured for a First Order Hold rate of 8/13."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_SRC_RATEW::_0)
    }
    #[doc = "SRC is configured for a Zero Order Hold rate of 12/13."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_SRC_RATEW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_DMA_DTEST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DMA_DTEST_ENW<'a> {
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
pub struct _RX_DEC_FILT_GAINW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DEC_FILT_GAINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_DEC_FILT_HZD_CORR_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DEC_FILT_HZD_CORR_DISW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Receive ADC Negative Edge Selection"]
    #[inline]
    pub fn rx_adc_negedge(&self) -> RX_ADC_NEGEDGER {
        RX_ADC_NEGEDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receive Channel Filter Bypass"]
    #[inline]
    pub fn rx_ch_filt_bypass(&self) -> RX_CH_FILT_BYPASSR {
        RX_CH_FILT_BYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - ADC Raw Mode selection"]
    #[inline]
    pub fn rx_adc_raw_en(&self) -> RX_ADC_RAW_ENR {
        RX_ADC_RAW_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Receive ADC Polarity"]
    #[inline]
    pub fn rx_adc_pol(&self) -> RX_ADC_POLR {
        RX_ADC_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Decimation Filter Oversampling"]
    #[inline]
    pub fn rx_dec_filt_osr(&self) -> RX_DEC_FILT_OSRR {
        RX_DEC_FILT_OSRR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - FSK / 802.15.4 demodulator select"]
    #[inline]
    pub fn rx_fsk_zb_sel(&self) -> RX_FSK_ZB_SELR {
        RX_FSK_ZB_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Normalizer Enable"]
    #[inline]
    pub fn rx_norm_en(&self) -> RX_NORM_ENR {
        RX_NORM_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - RSSI Measurement Enable"]
    #[inline]
    pub fn rx_rssi_en(&self) -> RX_RSSI_ENR {
        RX_RSSI_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - AGC Global Enable"]
    #[inline]
    pub fn rx_agc_en(&self) -> RX_AGC_ENR {
        RX_AGC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - DCOC Enable"]
    #[inline]
    pub fn rx_dcoc_en(&self) -> RX_DCOC_ENR {
        RX_DCOC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - DCOC Calibration Enable"]
    #[inline]
    pub fn rx_dcoc_cal_en(&self) -> RX_DCOC_CAL_ENR {
        RX_DCOC_CAL_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - RX IQ Swap"]
    #[inline]
    pub fn rx_iq_swap(&self) -> RX_IQ_SWAPR {
        RX_IQ_SWAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - DC Residual Enable"]
    #[inline]
    pub fn rx_dc_resid_en(&self) -> RX_DC_RESID_ENR {
        RX_DC_RESID_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - RX Sample Rate Converter Enable"]
    #[inline]
    pub fn rx_src_en(&self) -> RX_SRC_ENR {
        RX_SRC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - RX Sample Rate Converter Rate Selections"]
    #[inline]
    pub fn rx_src_rate(&self) -> RX_SRC_RATER {
        RX_SRC_RATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - RX DMA and DTEST enable"]
    #[inline]
    pub fn rx_dma_dtest_en(&self) -> RX_DMA_DTEST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_DMA_DTEST_ENR { bits }
    }
    #[doc = "Bits 20:24 - Decimation Filter Fractional Gain"]
    #[inline]
    pub fn rx_dec_filt_gain(&self) -> RX_DEC_FILT_GAINR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_DEC_FILT_GAINR { bits }
    }
    #[doc = "Bit 25 - Decimator filter hazard correction disable"]
    #[inline]
    pub fn rx_dec_filt_hzd_corr_dis(&self) -> RX_DEC_FILT_HZD_CORR_DISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_DEC_FILT_HZD_CORR_DISR { bits }
    }
    #[doc = "Bit 28 - Decimator output, hazard condition detected"]
    #[inline]
    pub fn rx_dec_filt_hazard(&self) -> RX_DEC_FILT_HAZARDR {
        RX_DEC_FILT_HAZARDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Decimator output for RSSI, hazard condition detected"]
    #[inline]
    pub fn rx_rssi_filt_hazard(&self) -> RX_RSSI_FILT_HAZARDR {
        RX_RSSI_FILT_HAZARDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Decimator output, saturation detected for I channel"]
    #[inline]
    pub fn rx_dec_filt_sat_i(&self) -> RX_DEC_FILT_SAT_IR {
        RX_DEC_FILT_SAT_IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Decimator output, saturation detected for Q channel"]
    #[inline]
    pub fn rx_dec_filt_sat_q(&self) -> RX_DEC_FILT_SAT_QR {
        RX_DEC_FILT_SAT_QR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - Receive ADC Negative Edge Selection"]
    #[inline]
    pub fn rx_adc_negedge(&mut self) -> _RX_ADC_NEGEDGEW {
        _RX_ADC_NEGEDGEW { w: self }
    }
    #[doc = "Bit 1 - Receive Channel Filter Bypass"]
    #[inline]
    pub fn rx_ch_filt_bypass(&mut self) -> _RX_CH_FILT_BYPASSW {
        _RX_CH_FILT_BYPASSW { w: self }
    }
    #[doc = "Bit 2 - ADC Raw Mode selection"]
    #[inline]
    pub fn rx_adc_raw_en(&mut self) -> _RX_ADC_RAW_ENW {
        _RX_ADC_RAW_ENW { w: self }
    }
    #[doc = "Bit 3 - Receive ADC Polarity"]
    #[inline]
    pub fn rx_adc_pol(&mut self) -> _RX_ADC_POLW {
        _RX_ADC_POLW { w: self }
    }
    #[doc = "Bits 4:6 - Decimation Filter Oversampling"]
    #[inline]
    pub fn rx_dec_filt_osr(&mut self) -> _RX_DEC_FILT_OSRW {
        _RX_DEC_FILT_OSRW { w: self }
    }
    #[doc = "Bit 8 - FSK / 802.15.4 demodulator select"]
    #[inline]
    pub fn rx_fsk_zb_sel(&mut self) -> _RX_FSK_ZB_SELW {
        _RX_FSK_ZB_SELW { w: self }
    }
    #[doc = "Bit 9 - Normalizer Enable"]
    #[inline]
    pub fn rx_norm_en(&mut self) -> _RX_NORM_ENW {
        _RX_NORM_ENW { w: self }
    }
    #[doc = "Bit 10 - RSSI Measurement Enable"]
    #[inline]
    pub fn rx_rssi_en(&mut self) -> _RX_RSSI_ENW {
        _RX_RSSI_ENW { w: self }
    }
    #[doc = "Bit 11 - AGC Global Enable"]
    #[inline]
    pub fn rx_agc_en(&mut self) -> _RX_AGC_ENW {
        _RX_AGC_ENW { w: self }
    }
    #[doc = "Bit 12 - DCOC Enable"]
    #[inline]
    pub fn rx_dcoc_en(&mut self) -> _RX_DCOC_ENW {
        _RX_DCOC_ENW { w: self }
    }
    #[doc = "Bit 13 - DCOC Calibration Enable"]
    #[inline]
    pub fn rx_dcoc_cal_en(&mut self) -> _RX_DCOC_CAL_ENW {
        _RX_DCOC_CAL_ENW { w: self }
    }
    #[doc = "Bit 14 - RX IQ Swap"]
    #[inline]
    pub fn rx_iq_swap(&mut self) -> _RX_IQ_SWAPW {
        _RX_IQ_SWAPW { w: self }
    }
    #[doc = "Bit 15 - DC Residual Enable"]
    #[inline]
    pub fn rx_dc_resid_en(&mut self) -> _RX_DC_RESID_ENW {
        _RX_DC_RESID_ENW { w: self }
    }
    #[doc = "Bit 16 - RX Sample Rate Converter Enable"]
    #[inline]
    pub fn rx_src_en(&mut self) -> _RX_SRC_ENW {
        _RX_SRC_ENW { w: self }
    }
    #[doc = "Bit 17 - RX Sample Rate Converter Rate Selections"]
    #[inline]
    pub fn rx_src_rate(&mut self) -> _RX_SRC_RATEW {
        _RX_SRC_RATEW { w: self }
    }
    #[doc = "Bit 18 - RX DMA and DTEST enable"]
    #[inline]
    pub fn rx_dma_dtest_en(&mut self) -> _RX_DMA_DTEST_ENW {
        _RX_DMA_DTEST_ENW { w: self }
    }
    #[doc = "Bits 20:24 - Decimation Filter Fractional Gain"]
    #[inline]
    pub fn rx_dec_filt_gain(&mut self) -> _RX_DEC_FILT_GAINW {
        _RX_DEC_FILT_GAINW { w: self }
    }
    #[doc = "Bit 25 - Decimator filter hazard correction disable"]
    #[inline]
    pub fn rx_dec_filt_hzd_corr_dis(&mut self) -> _RX_DEC_FILT_HZD_CORR_DISW {
        _RX_DEC_FILT_HZD_CORR_DISW { w: self }
    }
}
