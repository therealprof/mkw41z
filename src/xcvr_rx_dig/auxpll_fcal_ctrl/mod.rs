#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUXPLL_FCAL_CTRL {
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
pub struct DAC_CAL_ADJUST_MANUALR {
    bits: u8,
}
impl DAC_CAL_ADJUST_MANUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `AUXPLL_DAC_CAL_ADJUST_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUXPLL_DAC_CAL_ADJUST_DISR {
    #[doc = "Calibration is enabled"]
    _0,
    #[doc = "Calibration is disabled"]
    _1,
}
impl AUXPLL_DAC_CAL_ADJUST_DISR {
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
            AUXPLL_DAC_CAL_ADJUST_DISR::_0 => false,
            AUXPLL_DAC_CAL_ADJUST_DISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUXPLL_DAC_CAL_ADJUST_DISR {
        match value {
            false => AUXPLL_DAC_CAL_ADJUST_DISR::_0,
            true => AUXPLL_DAC_CAL_ADJUST_DISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AUXPLL_DAC_CAL_ADJUST_DISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AUXPLL_DAC_CAL_ADJUST_DISR::_1
    }
}
#[doc = "Possible values of the field `FCAL_RUN_CNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCAL_RUN_CNTR {
    #[doc = "Run count is 256 clock cycles"]
    _0,
    #[doc = "Run count is 512 clock cycles"]
    _1,
}
impl FCAL_RUN_CNTR {
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
            FCAL_RUN_CNTR::_0 => false,
            FCAL_RUN_CNTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCAL_RUN_CNTR {
        match value {
            false => FCAL_RUN_CNTR::_0,
            true => FCAL_RUN_CNTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FCAL_RUN_CNTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FCAL_RUN_CNTR::_1
    }
}
#[doc = "Possible values of the field `FCAL_COMP_INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCAL_COMP_INVR {
    #[doc = "(Default) The comparison associated with the count is not inverted."]
    _0,
    #[doc = "The comparison associated with the count is inverted"]
    _1,
}
impl FCAL_COMP_INVR {
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
            FCAL_COMP_INVR::_0 => false,
            FCAL_COMP_INVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCAL_COMP_INVR {
        match value {
            false => FCAL_COMP_INVR::_0,
            true => FCAL_COMP_INVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FCAL_COMP_INVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FCAL_COMP_INVR::_1
    }
}
#[doc = "Possible values of the field `FCAL_SMP_DLY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCAL_SMP_DLYR {
    #[doc = "The count signal is sampled 1 clk cycle after fcal_run signal is deasserted"]
    _00,
    #[doc = "The count signal is sampled 2 clk cycle after fcal_run signal is deasserted"]
    _01,
    #[doc = "The count signal is sampled 3 clk cycle after fcal_run signal is deasserted"]
    _10,
    #[doc = "The count signal is sampled 4 clk cycle after fcal_run signal is deasserted"]
    _11,
}
impl FCAL_SMP_DLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FCAL_SMP_DLYR::_00 => 0,
            FCAL_SMP_DLYR::_01 => 1,
            FCAL_SMP_DLYR::_10 => 2,
            FCAL_SMP_DLYR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FCAL_SMP_DLYR {
        match value {
            0 => FCAL_SMP_DLYR::_00,
            1 => FCAL_SMP_DLYR::_01,
            2 => FCAL_SMP_DLYR::_10,
            3 => FCAL_SMP_DLYR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FCAL_SMP_DLYR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FCAL_SMP_DLYR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FCAL_SMP_DLYR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FCAL_SMP_DLYR::_11
    }
}
#[doc = r" Value of the field"]
pub struct DAC_CAL_ADJUSTR {
    bits: u8,
}
impl DAC_CAL_ADJUSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DAC_CAL_ADJUST_MANUALW<'a> {
    w: &'a mut W,
}
impl<'a> _DAC_CAL_ADJUST_MANUALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUXPLL_DAC_CAL_ADJUST_DIS`"]
pub enum AUXPLL_DAC_CAL_ADJUST_DISW {
    #[doc = "Calibration is enabled"]
    _0,
    #[doc = "Calibration is disabled"]
    _1,
}
impl AUXPLL_DAC_CAL_ADJUST_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUXPLL_DAC_CAL_ADJUST_DISW::_0 => false,
            AUXPLL_DAC_CAL_ADJUST_DISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUXPLL_DAC_CAL_ADJUST_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _AUXPLL_DAC_CAL_ADJUST_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUXPLL_DAC_CAL_ADJUST_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Calibration is enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AUXPLL_DAC_CAL_ADJUST_DISW::_0)
    }
    #[doc = "Calibration is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AUXPLL_DAC_CAL_ADJUST_DISW::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FCAL_RUN_CNT`"]
pub enum FCAL_RUN_CNTW {
    #[doc = "Run count is 256 clock cycles"]
    _0,
    #[doc = "Run count is 512 clock cycles"]
    _1,
}
impl FCAL_RUN_CNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCAL_RUN_CNTW::_0 => false,
            FCAL_RUN_CNTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCAL_RUN_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _FCAL_RUN_CNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCAL_RUN_CNTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Run count is 256 clock cycles"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCAL_RUN_CNTW::_0)
    }
    #[doc = "Run count is 512 clock cycles"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCAL_RUN_CNTW::_1)
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
#[doc = "Values that can be written to the field `FCAL_COMP_INV`"]
pub enum FCAL_COMP_INVW {
    #[doc = "(Default) The comparison associated with the count is not inverted."]
    _0,
    #[doc = "The comparison associated with the count is inverted"]
    _1,
}
impl FCAL_COMP_INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCAL_COMP_INVW::_0 => false,
            FCAL_COMP_INVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCAL_COMP_INVW<'a> {
    w: &'a mut W,
}
impl<'a> _FCAL_COMP_INVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCAL_COMP_INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "(Default) The comparison associated with the count is not inverted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCAL_COMP_INVW::_0)
    }
    #[doc = "The comparison associated with the count is inverted"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCAL_COMP_INVW::_1)
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
#[doc = "Values that can be written to the field `FCAL_SMP_DLY`"]
pub enum FCAL_SMP_DLYW {
    #[doc = "The count signal is sampled 1 clk cycle after fcal_run signal is deasserted"]
    _00,
    #[doc = "The count signal is sampled 2 clk cycle after fcal_run signal is deasserted"]
    _01,
    #[doc = "The count signal is sampled 3 clk cycle after fcal_run signal is deasserted"]
    _10,
    #[doc = "The count signal is sampled 4 clk cycle after fcal_run signal is deasserted"]
    _11,
}
impl FCAL_SMP_DLYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FCAL_SMP_DLYW::_00 => 0,
            FCAL_SMP_DLYW::_01 => 1,
            FCAL_SMP_DLYW::_10 => 2,
            FCAL_SMP_DLYW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCAL_SMP_DLYW<'a> {
    w: &'a mut W,
}
impl<'a> _FCAL_SMP_DLYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCAL_SMP_DLYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The count signal is sampled 1 clk cycle after fcal_run signal is deasserted"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FCAL_SMP_DLYW::_00)
    }
    #[doc = "The count signal is sampled 2 clk cycle after fcal_run signal is deasserted"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FCAL_SMP_DLYW::_01)
    }
    #[doc = "The count signal is sampled 3 clk cycle after fcal_run signal is deasserted"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FCAL_SMP_DLYW::_10)
    }
    #[doc = "The count signal is sampled 4 clk cycle after fcal_run signal is deasserted"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FCAL_SMP_DLYW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:6 - Aux PLL Frequency DAC Calibration Adjust Manual value"]
    #[inline]
    pub fn dac_cal_adjust_manual(&self) -> DAC_CAL_ADJUST_MANUALR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DAC_CAL_ADJUST_MANUALR { bits }
    }
    #[doc = "Bit 7 - Aux PLL Frequency Calibration Disable"]
    #[inline]
    pub fn auxpll_dac_cal_adjust_dis(&self) -> AUXPLL_DAC_CAL_ADJUST_DISR {
        AUXPLL_DAC_CAL_ADJUST_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Aux PLL Frequency Calibration Run Count"]
    #[inline]
    pub fn fcal_run_cnt(&self) -> FCAL_RUN_CNTR {
        FCAL_RUN_CNTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Aux PLL Frequency Calibration Comparison Invert"]
    #[inline]
    pub fn fcal_comp_inv(&self) -> FCAL_COMP_INVR {
        FCAL_COMP_INVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:11 - Aux PLL Frequency Calibration Sample Delay"]
    #[inline]
    pub fn fcal_smp_dly(&self) -> FCAL_SMP_DLYR {
        FCAL_SMP_DLYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:22 - Aux PLL DAC Calibration Adjust value"]
    #[inline]
    pub fn dac_cal_adjust(&self) -> DAC_CAL_ADJUSTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DAC_CAL_ADJUSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4194304 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Aux PLL Frequency DAC Calibration Adjust Manual value"]
    #[inline]
    pub fn dac_cal_adjust_manual(&mut self) -> _DAC_CAL_ADJUST_MANUALW {
        _DAC_CAL_ADJUST_MANUALW { w: self }
    }
    #[doc = "Bit 7 - Aux PLL Frequency Calibration Disable"]
    #[inline]
    pub fn auxpll_dac_cal_adjust_dis(&mut self) -> _AUXPLL_DAC_CAL_ADJUST_DISW {
        _AUXPLL_DAC_CAL_ADJUST_DISW { w: self }
    }
    #[doc = "Bit 8 - Aux PLL Frequency Calibration Run Count"]
    #[inline]
    pub fn fcal_run_cnt(&mut self) -> _FCAL_RUN_CNTW {
        _FCAL_RUN_CNTW { w: self }
    }
    #[doc = "Bit 9 - Aux PLL Frequency Calibration Comparison Invert"]
    #[inline]
    pub fn fcal_comp_inv(&mut self) -> _FCAL_COMP_INVW {
        _FCAL_COMP_INVW { w: self }
    }
    #[doc = "Bits 10:11 - Aux PLL Frequency Calibration Sample Delay"]
    #[inline]
    pub fn fcal_smp_dly(&mut self) -> _FCAL_SMP_DLYW {
        _FCAL_SMP_DLYW { w: self }
    }
}
