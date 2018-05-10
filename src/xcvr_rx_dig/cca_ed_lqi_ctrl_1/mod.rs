#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCA_ED_LQI_CTRL_1 {
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
pub struct RSSI_NOISE_AVG_DELAYR {
    bits: u8,
}
impl RSSI_NOISE_AVG_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RSSI_NOISE_AVG_FACTOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSI_NOISE_AVG_FACTORR {
    #[doc = "1"]
    _0,
    #[doc = "64"]
    _1,
    #[doc = "70"]
    _2,
    #[doc = "128"]
    _3,
    #[doc = "139"]
    _4,
    #[doc = "256"]
    _5,
    #[doc = "277"]
    _6,
    #[doc = "512"]
    _7,
}
impl RSSI_NOISE_AVG_FACTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSSI_NOISE_AVG_FACTORR::_0 => 0,
            RSSI_NOISE_AVG_FACTORR::_1 => 1,
            RSSI_NOISE_AVG_FACTORR::_2 => 2,
            RSSI_NOISE_AVG_FACTORR::_3 => 3,
            RSSI_NOISE_AVG_FACTORR::_4 => 4,
            RSSI_NOISE_AVG_FACTORR::_5 => 5,
            RSSI_NOISE_AVG_FACTORR::_6 => 6,
            RSSI_NOISE_AVG_FACTORR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSSI_NOISE_AVG_FACTORR {
        match value {
            0 => RSSI_NOISE_AVG_FACTORR::_0,
            1 => RSSI_NOISE_AVG_FACTORR::_1,
            2 => RSSI_NOISE_AVG_FACTORR::_2,
            3 => RSSI_NOISE_AVG_FACTORR::_3,
            4 => RSSI_NOISE_AVG_FACTORR::_4,
            5 => RSSI_NOISE_AVG_FACTORR::_5,
            6 => RSSI_NOISE_AVG_FACTORR::_6,
            7 => RSSI_NOISE_AVG_FACTORR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RSSI_NOISE_AVG_FACTORR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSSI_NOISE_AVG_FACTORR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == RSSI_NOISE_AVG_FACTORR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == RSSI_NOISE_AVG_FACTORR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == RSSI_NOISE_AVG_FACTORR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == RSSI_NOISE_AVG_FACTORR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == RSSI_NOISE_AVG_FACTORR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == RSSI_NOISE_AVG_FACTORR::_7
    }
}
#[doc = "Possible values of the field `LQI_RSSI_WEIGHT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LQI_RSSI_WEIGHTR {
    #[doc = "2.0"]
    _0,
    #[doc = "2.125"]
    _1,
    #[doc = "2.25"]
    _2,
    #[doc = "2.375"]
    _3,
    #[doc = "2.5"]
    _4,
    #[doc = "2.625"]
    _5,
    #[doc = "2.75"]
    _6,
    #[doc = "2.875"]
    _7,
}
impl LQI_RSSI_WEIGHTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LQI_RSSI_WEIGHTR::_0 => 0,
            LQI_RSSI_WEIGHTR::_1 => 1,
            LQI_RSSI_WEIGHTR::_2 => 2,
            LQI_RSSI_WEIGHTR::_3 => 3,
            LQI_RSSI_WEIGHTR::_4 => 4,
            LQI_RSSI_WEIGHTR::_5 => 5,
            LQI_RSSI_WEIGHTR::_6 => 6,
            LQI_RSSI_WEIGHTR::_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LQI_RSSI_WEIGHTR {
        match value {
            0 => LQI_RSSI_WEIGHTR::_0,
            1 => LQI_RSSI_WEIGHTR::_1,
            2 => LQI_RSSI_WEIGHTR::_2,
            3 => LQI_RSSI_WEIGHTR::_3,
            4 => LQI_RSSI_WEIGHTR::_4,
            5 => LQI_RSSI_WEIGHTR::_5,
            6 => LQI_RSSI_WEIGHTR::_6,
            7 => LQI_RSSI_WEIGHTR::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LQI_RSSI_WEIGHTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LQI_RSSI_WEIGHTR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == LQI_RSSI_WEIGHTR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == LQI_RSSI_WEIGHTR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == LQI_RSSI_WEIGHTR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == LQI_RSSI_WEIGHTR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == LQI_RSSI_WEIGHTR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == LQI_RSSI_WEIGHTR::_7
    }
}
#[doc = r" Value of the field"]
pub struct LQI_RSSI_SENSR {
    bits: u8,
}
impl LQI_RSSI_SENSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SNR_LQI_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SNR_LQI_DISR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "The RX_DIG CCA/ED/LQI block ignores the AA match input which starts an LQI measurement."]
    _1,
}
impl SNR_LQI_DISR {
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
            SNR_LQI_DISR::_0 => false,
            SNR_LQI_DISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SNR_LQI_DISR {
        match value {
            false => SNR_LQI_DISR::_0,
            true => SNR_LQI_DISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SNR_LQI_DISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SNR_LQI_DISR::_1
    }
}
#[doc = "Possible values of the field `SEL_SNR_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_SNR_MODER {
    #[doc = "SNR estimate"]
    _0,
    #[doc = "Mapped correlation magnitude"]
    _1,
}
impl SEL_SNR_MODER {
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
            SEL_SNR_MODER::_0 => false,
            SEL_SNR_MODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEL_SNR_MODER {
        match value {
            false => SEL_SNR_MODER::_0,
            true => SEL_SNR_MODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SEL_SNR_MODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SEL_SNR_MODER::_1
    }
}
#[doc = "Possible values of the field `MEAS_TRANS_TO_IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEAS_TRANS_TO_IDLER {
    #[doc = "Module transitions to RSSI state"]
    _0,
    #[doc = "Module transitions to IDLE state"]
    _1,
}
impl MEAS_TRANS_TO_IDLER {
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
            MEAS_TRANS_TO_IDLER::_0 => false,
            MEAS_TRANS_TO_IDLER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEAS_TRANS_TO_IDLER {
        match value {
            false => MEAS_TRANS_TO_IDLER::_0,
            true => MEAS_TRANS_TO_IDLER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MEAS_TRANS_TO_IDLER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MEAS_TRANS_TO_IDLER::_1
    }
}
#[doc = "Possible values of the field `CCA1_ED_EN_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCA1_ED_EN_DISR {
    #[doc = "Normal operation"]
    _0,
    #[doc = "CCA1_ED_EN input is disabled"]
    _1,
}
impl CCA1_ED_EN_DISR {
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
            CCA1_ED_EN_DISR::_0 => false,
            CCA1_ED_EN_DISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCA1_ED_EN_DISR {
        match value {
            false => CCA1_ED_EN_DISR::_0,
            true => CCA1_ED_EN_DISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCA1_ED_EN_DISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCA1_ED_EN_DISR::_1
    }
}
#[doc = "Possible values of the field `MAN_MEAS_COMPLETE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAN_MEAS_COMPLETER {
    #[doc = "Normal operation"]
    _0,
    #[doc = "Manually asserts the measurement complete signal for the RX_DIG CCA/ED/LQI blocks. Intended to be used only for debug."]
    _1,
}
impl MAN_MEAS_COMPLETER {
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
            MAN_MEAS_COMPLETER::_0 => false,
            MAN_MEAS_COMPLETER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAN_MEAS_COMPLETER {
        match value {
            false => MAN_MEAS_COMPLETER::_0,
            true => MAN_MEAS_COMPLETER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MAN_MEAS_COMPLETER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MAN_MEAS_COMPLETER::_1
    }
}
#[doc = "Possible values of the field `MAN_AA_MATCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAN_AA_MATCHR {
    #[doc = "Normal operation"]
    _0,
    #[doc = "Manually asserts the AA match signal for the RX_DIG CCA/ED/LQI and AGC blocks. Intended to be used only for debug."]
    _1,
}
impl MAN_AA_MATCHR {
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
            MAN_AA_MATCHR::_0 => false,
            MAN_AA_MATCHR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAN_AA_MATCHR {
        match value {
            false => MAN_AA_MATCHR::_0,
            true => MAN_AA_MATCHR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MAN_AA_MATCHR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MAN_AA_MATCHR::_1
    }
}
#[doc = "Possible values of the field `SNR_LQI_WEIGHT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SNR_LQI_WEIGHTR {
    #[doc = "0.0"]
    _0,
    #[doc = "1.0"]
    _1,
    #[doc = "1.125"]
    _2,
    #[doc = "1.25"]
    _3,
    #[doc = "1.375"]
    _4,
    #[doc = "1.5"]
    _5,
    #[doc = "1.625"]
    _6,
    #[doc = "1.75"]
    _7,
    #[doc = "1.875"]
    _8,
    #[doc = "2.0"]
    _9,
    #[doc = "2.125"]
    _10,
    #[doc = "2.25"]
    _11,
    #[doc = "2.375"]
    _12,
    #[doc = "2.5"]
    _13,
    #[doc = "2.625"]
    _14,
    #[doc = "2.75"]
    _15,
}
impl SNR_LQI_WEIGHTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SNR_LQI_WEIGHTR::_0 => 0,
            SNR_LQI_WEIGHTR::_1 => 1,
            SNR_LQI_WEIGHTR::_2 => 2,
            SNR_LQI_WEIGHTR::_3 => 3,
            SNR_LQI_WEIGHTR::_4 => 4,
            SNR_LQI_WEIGHTR::_5 => 5,
            SNR_LQI_WEIGHTR::_6 => 6,
            SNR_LQI_WEIGHTR::_7 => 7,
            SNR_LQI_WEIGHTR::_8 => 8,
            SNR_LQI_WEIGHTR::_9 => 9,
            SNR_LQI_WEIGHTR::_10 => 10,
            SNR_LQI_WEIGHTR::_11 => 11,
            SNR_LQI_WEIGHTR::_12 => 12,
            SNR_LQI_WEIGHTR::_13 => 13,
            SNR_LQI_WEIGHTR::_14 => 14,
            SNR_LQI_WEIGHTR::_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SNR_LQI_WEIGHTR {
        match value {
            0 => SNR_LQI_WEIGHTR::_0,
            1 => SNR_LQI_WEIGHTR::_1,
            2 => SNR_LQI_WEIGHTR::_2,
            3 => SNR_LQI_WEIGHTR::_3,
            4 => SNR_LQI_WEIGHTR::_4,
            5 => SNR_LQI_WEIGHTR::_5,
            6 => SNR_LQI_WEIGHTR::_6,
            7 => SNR_LQI_WEIGHTR::_7,
            8 => SNR_LQI_WEIGHTR::_8,
            9 => SNR_LQI_WEIGHTR::_9,
            10 => SNR_LQI_WEIGHTR::_10,
            11 => SNR_LQI_WEIGHTR::_11,
            12 => SNR_LQI_WEIGHTR::_12,
            13 => SNR_LQI_WEIGHTR::_13,
            14 => SNR_LQI_WEIGHTR::_14,
            15 => SNR_LQI_WEIGHTR::_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline]
    pub fn is_9(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline]
    pub fn is_12(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline]
    pub fn is_13(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline]
    pub fn is_14(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline]
    pub fn is_15(&self) -> bool {
        *self == SNR_LQI_WEIGHTR::_15
    }
}
#[doc = r" Value of the field"]
pub struct LQI_BIASR {
    bits: u8,
}
impl LQI_BIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RSSI_NOISE_AVG_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _RSSI_NOISE_AVG_DELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSSI_NOISE_AVG_FACTOR`"]
pub enum RSSI_NOISE_AVG_FACTORW {
    #[doc = "1"]
    _0,
    #[doc = "64"]
    _1,
    #[doc = "70"]
    _2,
    #[doc = "128"]
    _3,
    #[doc = "139"]
    _4,
    #[doc = "256"]
    _5,
    #[doc = "277"]
    _6,
    #[doc = "512"]
    _7,
}
impl RSSI_NOISE_AVG_FACTORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSSI_NOISE_AVG_FACTORW::_0 => 0,
            RSSI_NOISE_AVG_FACTORW::_1 => 1,
            RSSI_NOISE_AVG_FACTORW::_2 => 2,
            RSSI_NOISE_AVG_FACTORW::_3 => 3,
            RSSI_NOISE_AVG_FACTORW::_4 => 4,
            RSSI_NOISE_AVG_FACTORW::_5 => 5,
            RSSI_NOISE_AVG_FACTORW::_6 => 6,
            RSSI_NOISE_AVG_FACTORW::_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSSI_NOISE_AVG_FACTORW<'a> {
    w: &'a mut W,
}
impl<'a> _RSSI_NOISE_AVG_FACTORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSSI_NOISE_AVG_FACTORW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSSI_NOISE_AVG_FACTORW::_0)
    }
    #[doc = "64"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSSI_NOISE_AVG_FACTORW::_1)
    }
    #[doc = "70"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(RSSI_NOISE_AVG_FACTORW::_2)
    }
    #[doc = "128"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(RSSI_NOISE_AVG_FACTORW::_3)
    }
    #[doc = "139"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(RSSI_NOISE_AVG_FACTORW::_4)
    }
    #[doc = "256"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(RSSI_NOISE_AVG_FACTORW::_5)
    }
    #[doc = "277"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(RSSI_NOISE_AVG_FACTORW::_6)
    }
    #[doc = "512"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(RSSI_NOISE_AVG_FACTORW::_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LQI_RSSI_WEIGHT`"]
pub enum LQI_RSSI_WEIGHTW {
    #[doc = "2.0"]
    _0,
    #[doc = "2.125"]
    _1,
    #[doc = "2.25"]
    _2,
    #[doc = "2.375"]
    _3,
    #[doc = "2.5"]
    _4,
    #[doc = "2.625"]
    _5,
    #[doc = "2.75"]
    _6,
    #[doc = "2.875"]
    _7,
}
impl LQI_RSSI_WEIGHTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LQI_RSSI_WEIGHTW::_0 => 0,
            LQI_RSSI_WEIGHTW::_1 => 1,
            LQI_RSSI_WEIGHTW::_2 => 2,
            LQI_RSSI_WEIGHTW::_3 => 3,
            LQI_RSSI_WEIGHTW::_4 => 4,
            LQI_RSSI_WEIGHTW::_5 => 5,
            LQI_RSSI_WEIGHTW::_6 => 6,
            LQI_RSSI_WEIGHTW::_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LQI_RSSI_WEIGHTW<'a> {
    w: &'a mut W,
}
impl<'a> _LQI_RSSI_WEIGHTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LQI_RSSI_WEIGHTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "2.0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LQI_RSSI_WEIGHTW::_0)
    }
    #[doc = "2.125"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LQI_RSSI_WEIGHTW::_1)
    }
    #[doc = "2.25"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(LQI_RSSI_WEIGHTW::_2)
    }
    #[doc = "2.375"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(LQI_RSSI_WEIGHTW::_3)
    }
    #[doc = "2.5"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(LQI_RSSI_WEIGHTW::_4)
    }
    #[doc = "2.625"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(LQI_RSSI_WEIGHTW::_5)
    }
    #[doc = "2.75"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(LQI_RSSI_WEIGHTW::_6)
    }
    #[doc = "2.875"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(LQI_RSSI_WEIGHTW::_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LQI_RSSI_SENSW<'a> {
    w: &'a mut W,
}
impl<'a> _LQI_RSSI_SENSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SNR_LQI_DIS`"]
pub enum SNR_LQI_DISW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "The RX_DIG CCA/ED/LQI block ignores the AA match input which starts an LQI measurement."]
    _1,
}
impl SNR_LQI_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SNR_LQI_DISW::_0 => false,
            SNR_LQI_DISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SNR_LQI_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _SNR_LQI_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SNR_LQI_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNR_LQI_DISW::_0)
    }
    #[doc = "The RX_DIG CCA/ED/LQI block ignores the AA match input which starts an LQI measurement."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNR_LQI_DISW::_1)
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
#[doc = "Values that can be written to the field `SEL_SNR_MODE`"]
pub enum SEL_SNR_MODEW {
    #[doc = "SNR estimate"]
    _0,
    #[doc = "Mapped correlation magnitude"]
    _1,
}
impl SEL_SNR_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEL_SNR_MODEW::_0 => false,
            SEL_SNR_MODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEL_SNR_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SEL_SNR_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEL_SNR_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SNR estimate"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEL_SNR_MODEW::_0)
    }
    #[doc = "Mapped correlation magnitude"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEL_SNR_MODEW::_1)
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
#[doc = "Values that can be written to the field `MEAS_TRANS_TO_IDLE`"]
pub enum MEAS_TRANS_TO_IDLEW {
    #[doc = "Module transitions to RSSI state"]
    _0,
    #[doc = "Module transitions to IDLE state"]
    _1,
}
impl MEAS_TRANS_TO_IDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEAS_TRANS_TO_IDLEW::_0 => false,
            MEAS_TRANS_TO_IDLEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEAS_TRANS_TO_IDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _MEAS_TRANS_TO_IDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEAS_TRANS_TO_IDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Module transitions to RSSI state"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MEAS_TRANS_TO_IDLEW::_0)
    }
    #[doc = "Module transitions to IDLE state"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MEAS_TRANS_TO_IDLEW::_1)
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
#[doc = "Values that can be written to the field `CCA1_ED_EN_DIS`"]
pub enum CCA1_ED_EN_DISW {
    #[doc = "Normal operation"]
    _0,
    #[doc = "CCA1_ED_EN input is disabled"]
    _1,
}
impl CCA1_ED_EN_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCA1_ED_EN_DISW::_0 => false,
            CCA1_ED_EN_DISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCA1_ED_EN_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _CCA1_ED_EN_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCA1_ED_EN_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCA1_ED_EN_DISW::_0)
    }
    #[doc = "CCA1_ED_EN input is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCA1_ED_EN_DISW::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MAN_MEAS_COMPLETE`"]
pub enum MAN_MEAS_COMPLETEW {
    #[doc = "Normal operation"]
    _0,
    #[doc = "Manually asserts the measurement complete signal for the RX_DIG CCA/ED/LQI blocks. Intended to be used only for debug."]
    _1,
}
impl MAN_MEAS_COMPLETEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAN_MEAS_COMPLETEW::_0 => false,
            MAN_MEAS_COMPLETEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAN_MEAS_COMPLETEW<'a> {
    w: &'a mut W,
}
impl<'a> _MAN_MEAS_COMPLETEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAN_MEAS_COMPLETEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAN_MEAS_COMPLETEW::_0)
    }
    #[doc = "Manually asserts the measurement complete signal for the RX_DIG CCA/ED/LQI blocks. Intended to be used only for debug."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAN_MEAS_COMPLETEW::_1)
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
#[doc = "Values that can be written to the field `MAN_AA_MATCH`"]
pub enum MAN_AA_MATCHW {
    #[doc = "Normal operation"]
    _0,
    #[doc = "Manually asserts the AA match signal for the RX_DIG CCA/ED/LQI and AGC blocks. Intended to be used only for debug."]
    _1,
}
impl MAN_AA_MATCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAN_AA_MATCHW::_0 => false,
            MAN_AA_MATCHW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAN_AA_MATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _MAN_AA_MATCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAN_AA_MATCHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAN_AA_MATCHW::_0)
    }
    #[doc = "Manually asserts the AA match signal for the RX_DIG CCA/ED/LQI and AGC blocks. Intended to be used only for debug."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAN_AA_MATCHW::_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SNR_LQI_WEIGHT`"]
pub enum SNR_LQI_WEIGHTW {
    #[doc = "0.0"]
    _0,
    #[doc = "1.0"]
    _1,
    #[doc = "1.125"]
    _2,
    #[doc = "1.25"]
    _3,
    #[doc = "1.375"]
    _4,
    #[doc = "1.5"]
    _5,
    #[doc = "1.625"]
    _6,
    #[doc = "1.75"]
    _7,
    #[doc = "1.875"]
    _8,
    #[doc = "2.0"]
    _9,
    #[doc = "2.125"]
    _10,
    #[doc = "2.25"]
    _11,
    #[doc = "2.375"]
    _12,
    #[doc = "2.5"]
    _13,
    #[doc = "2.625"]
    _14,
    #[doc = "2.75"]
    _15,
}
impl SNR_LQI_WEIGHTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SNR_LQI_WEIGHTW::_0 => 0,
            SNR_LQI_WEIGHTW::_1 => 1,
            SNR_LQI_WEIGHTW::_2 => 2,
            SNR_LQI_WEIGHTW::_3 => 3,
            SNR_LQI_WEIGHTW::_4 => 4,
            SNR_LQI_WEIGHTW::_5 => 5,
            SNR_LQI_WEIGHTW::_6 => 6,
            SNR_LQI_WEIGHTW::_7 => 7,
            SNR_LQI_WEIGHTW::_8 => 8,
            SNR_LQI_WEIGHTW::_9 => 9,
            SNR_LQI_WEIGHTW::_10 => 10,
            SNR_LQI_WEIGHTW::_11 => 11,
            SNR_LQI_WEIGHTW::_12 => 12,
            SNR_LQI_WEIGHTW::_13 => 13,
            SNR_LQI_WEIGHTW::_14 => 14,
            SNR_LQI_WEIGHTW::_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SNR_LQI_WEIGHTW<'a> {
    w: &'a mut W,
}
impl<'a> _SNR_LQI_WEIGHTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SNR_LQI_WEIGHTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0.0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_0)
    }
    #[doc = "1.0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_1)
    }
    #[doc = "1.125"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_2)
    }
    #[doc = "1.25"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_3)
    }
    #[doc = "1.375"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_4)
    }
    #[doc = "1.5"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_5)
    }
    #[doc = "1.625"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_6)
    }
    #[doc = "1.75"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_7)
    }
    #[doc = "1.875"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_8)
    }
    #[doc = "2.0"]
    #[inline]
    pub fn _9(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_9)
    }
    #[doc = "2.125"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_10)
    }
    #[doc = "2.25"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_11)
    }
    #[doc = "2.375"]
    #[inline]
    pub fn _12(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_12)
    }
    #[doc = "2.5"]
    #[inline]
    pub fn _13(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_13)
    }
    #[doc = "2.625"]
    #[inline]
    pub fn _14(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_14)
    }
    #[doc = "2.75"]
    #[inline]
    pub fn _15(self) -> &'a mut W {
        self.variant(SNR_LQI_WEIGHTW::_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LQI_BIASW<'a> {
    w: &'a mut W,
}
impl<'a> _LQI_BIASW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:5 - RSSI Noise Averaging Delay"]
    #[inline]
    pub fn rssi_noise_avg_delay(&self) -> RSSI_NOISE_AVG_DELAYR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSSI_NOISE_AVG_DELAYR { bits }
    }
    #[doc = "Bits 6:8 - RSSI Noise Averaging Factor"]
    #[inline]
    pub fn rssi_noise_avg_factor(&self) -> RSSI_NOISE_AVG_FACTORR {
        RSSI_NOISE_AVG_FACTORR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:11 - LQI RSSI Weight"]
    #[inline]
    pub fn lqi_rssi_weight(&self) -> LQI_RSSI_WEIGHTR {
        LQI_RSSI_WEIGHTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - LQI RSSI Sensitivity"]
    #[inline]
    pub fn lqi_rssi_sens(&self) -> LQI_RSSI_SENSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LQI_RSSI_SENSR { bits }
    }
    #[doc = "Bit 16 - SNR LQI Disable"]
    #[inline]
    pub fn snr_lqi_dis(&self) -> SNR_LQI_DISR {
        SNR_LQI_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Select SNR Mode"]
    #[inline]
    pub fn sel_snr_mode(&self) -> SEL_SNR_MODER {
        SEL_SNR_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Measurement Transition to IDLE"]
    #[inline]
    pub fn meas_trans_to_idle(&self) -> MEAS_TRANS_TO_IDLER {
        MEAS_TRANS_TO_IDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - CCA1_ED_EN Disable"]
    #[inline]
    pub fn cca1_ed_en_dis(&self) -> CCA1_ED_EN_DISR {
        CCA1_ED_EN_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Manual measurement complete"]
    #[inline]
    pub fn man_meas_complete(&self) -> MAN_MEAS_COMPLETER {
        MAN_MEAS_COMPLETER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Manual AA Match"]
    #[inline]
    pub fn man_aa_match(&self) -> MAN_AA_MATCHR {
        MAN_AA_MATCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:27 - SNR LQI Weight"]
    #[inline]
    pub fn snr_lqi_weight(&self) -> SNR_LQI_WEIGHTR {
        SNR_LQI_WEIGHTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - LQI Bias."]
    #[inline]
    pub fn lqi_bias(&self) -> LQI_BIASR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LQI_BIASR { bits }
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
    #[doc = "Bits 0:5 - RSSI Noise Averaging Delay"]
    #[inline]
    pub fn rssi_noise_avg_delay(&mut self) -> _RSSI_NOISE_AVG_DELAYW {
        _RSSI_NOISE_AVG_DELAYW { w: self }
    }
    #[doc = "Bits 6:8 - RSSI Noise Averaging Factor"]
    #[inline]
    pub fn rssi_noise_avg_factor(&mut self) -> _RSSI_NOISE_AVG_FACTORW {
        _RSSI_NOISE_AVG_FACTORW { w: self }
    }
    #[doc = "Bits 9:11 - LQI RSSI Weight"]
    #[inline]
    pub fn lqi_rssi_weight(&mut self) -> _LQI_RSSI_WEIGHTW {
        _LQI_RSSI_WEIGHTW { w: self }
    }
    #[doc = "Bits 12:15 - LQI RSSI Sensitivity"]
    #[inline]
    pub fn lqi_rssi_sens(&mut self) -> _LQI_RSSI_SENSW {
        _LQI_RSSI_SENSW { w: self }
    }
    #[doc = "Bit 16 - SNR LQI Disable"]
    #[inline]
    pub fn snr_lqi_dis(&mut self) -> _SNR_LQI_DISW {
        _SNR_LQI_DISW { w: self }
    }
    #[doc = "Bit 17 - Select SNR Mode"]
    #[inline]
    pub fn sel_snr_mode(&mut self) -> _SEL_SNR_MODEW {
        _SEL_SNR_MODEW { w: self }
    }
    #[doc = "Bit 18 - Measurement Transition to IDLE"]
    #[inline]
    pub fn meas_trans_to_idle(&mut self) -> _MEAS_TRANS_TO_IDLEW {
        _MEAS_TRANS_TO_IDLEW { w: self }
    }
    #[doc = "Bit 19 - CCA1_ED_EN Disable"]
    #[inline]
    pub fn cca1_ed_en_dis(&mut self) -> _CCA1_ED_EN_DISW {
        _CCA1_ED_EN_DISW { w: self }
    }
    #[doc = "Bit 20 - Manual measurement complete"]
    #[inline]
    pub fn man_meas_complete(&mut self) -> _MAN_MEAS_COMPLETEW {
        _MAN_MEAS_COMPLETEW { w: self }
    }
    #[doc = "Bit 21 - Manual AA Match"]
    #[inline]
    pub fn man_aa_match(&mut self) -> _MAN_AA_MATCHW {
        _MAN_AA_MATCHW { w: self }
    }
    #[doc = "Bits 24:27 - SNR LQI Weight"]
    #[inline]
    pub fn snr_lqi_weight(&mut self) -> _SNR_LQI_WEIGHTW {
        _SNR_LQI_WEIGHTW { w: self }
    }
    #[doc = "Bits 28:31 - LQI Bias."]
    #[inline]
    pub fn lqi_bias(&mut self) -> _LQI_BIASW {
        _LQI_BIASW { w: self }
    }
}
