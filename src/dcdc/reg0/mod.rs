#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG0 {
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
pub struct DCDC_DISABLE_AUTO_CLK_SWITCHR {
    bits: bool,
}
impl DCDC_DISABLE_AUTO_CLK_SWITCHR {
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
pub struct DCDC_SEL_CLKR {
    bits: bool,
}
impl DCDC_SEL_CLKR {
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
pub struct DCDC_PWD_OSC_INTR {
    bits: bool,
}
impl DCDC_PWD_OSC_INTR {
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
pub struct DCDC_LP_DF_CMP_ENABLER {
    bits: bool,
}
impl DCDC_LP_DF_CMP_ENABLER {
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
pub struct DCDC_VBAT_DIV_CTRLR {
    bits: u8,
}
impl DCDC_VBAT_DIV_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DCDC_LP_STATE_HYS_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_LP_STATE_HYS_LR {
    #[doc = "Target voltage value - 0 mV"]
    _00,
    #[doc = "Target voltage value - 25 mV"]
    _01,
    #[doc = "Target voltage value - 50 mV"]
    _10,
    #[doc = "Target voltage value - 75 mV"]
    _11,
}
impl DCDC_LP_STATE_HYS_LR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCDC_LP_STATE_HYS_LR::_00 => 0,
            DCDC_LP_STATE_HYS_LR::_01 => 1,
            DCDC_LP_STATE_HYS_LR::_10 => 2,
            DCDC_LP_STATE_HYS_LR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCDC_LP_STATE_HYS_LR {
        match value {
            0 => DCDC_LP_STATE_HYS_LR::_00,
            1 => DCDC_LP_STATE_HYS_LR::_01,
            2 => DCDC_LP_STATE_HYS_LR::_10,
            3 => DCDC_LP_STATE_HYS_LR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DCDC_LP_STATE_HYS_LR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DCDC_LP_STATE_HYS_LR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DCDC_LP_STATE_HYS_LR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DCDC_LP_STATE_HYS_LR::_11
    }
}
#[doc = "Possible values of the field `DCDC_LP_STATE_HYS_H`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_LP_STATE_HYS_HR {
    #[doc = "Target voltage value + 0 mV"]
    _00,
    #[doc = "Target voltage value + 25 mV"]
    _01,
    #[doc = "Target voltage value + 50 mV"]
    _10,
    #[doc = "Target voltage value + 75 mV"]
    _11,
}
impl DCDC_LP_STATE_HYS_HR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCDC_LP_STATE_HYS_HR::_00 => 0,
            DCDC_LP_STATE_HYS_HR::_01 => 1,
            DCDC_LP_STATE_HYS_HR::_10 => 2,
            DCDC_LP_STATE_HYS_HR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCDC_LP_STATE_HYS_HR {
        match value {
            0 => DCDC_LP_STATE_HYS_HR::_00,
            1 => DCDC_LP_STATE_HYS_HR::_01,
            2 => DCDC_LP_STATE_HYS_HR::_10,
            3 => DCDC_LP_STATE_HYS_HR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DCDC_LP_STATE_HYS_HR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DCDC_LP_STATE_HYS_HR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DCDC_LP_STATE_HYS_HR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DCDC_LP_STATE_HYS_HR::_11
    }
}
#[doc = r" Value of the field"]
pub struct HYST_LP_COMP_ADJR {
    bits: bool,
}
impl HYST_LP_COMP_ADJR {
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
pub struct HYST_LP_CMP_DISABLER {
    bits: bool,
}
impl HYST_LP_CMP_DISABLER {
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
pub struct OFFSET_RSNS_LP_ADJR {
    bits: bool,
}
impl OFFSET_RSNS_LP_ADJR {
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
pub struct OFFSET_RSNS_LP_DISABLER {
    bits: bool,
}
impl OFFSET_RSNS_LP_DISABLER {
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
pub struct DCDC_LESS_IR {
    bits: bool,
}
impl DCDC_LESS_IR {
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
pub struct PWD_CMP_OFFSETR {
    bits: bool,
}
impl PWD_CMP_OFFSETR {
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
pub struct DCDC_XTALOK_DISABLER {
    bits: bool,
}
impl DCDC_XTALOK_DISABLER {
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
pub struct PSWITCH_STATUSR {
    bits: bool,
}
impl PSWITCH_STATUSR {
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
pub struct VLPS_CONFIG_DCDC_HPR {
    bits: bool,
}
impl VLPS_CONFIG_DCDC_HPR {
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
pub struct VLPR_VLPW_CONFIG_DCDC_HPR {
    bits: bool,
}
impl VLPR_VLPW_CONFIG_DCDC_HPR {
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
pub struct DCDC_STS_DC_OKR {
    bits: bool,
}
impl DCDC_STS_DC_OKR {
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
pub struct _DCDC_DISABLE_AUTO_CLK_SWITCHW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_DISABLE_AUTO_CLK_SWITCHW<'a> {
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
#[doc = r" Proxy"]
pub struct _DCDC_SEL_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_SEL_CLKW<'a> {
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
pub struct _DCDC_PWD_OSC_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_PWD_OSC_INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _DCDC_LP_DF_CMP_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_LP_DF_CMP_ENABLEW<'a> {
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
pub struct _DCDC_VBAT_DIV_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_VBAT_DIV_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCDC_LP_STATE_HYS_L`"]
pub enum DCDC_LP_STATE_HYS_LW {
    #[doc = "Target voltage value - 0 mV"]
    _00,
    #[doc = "Target voltage value - 25 mV"]
    _01,
    #[doc = "Target voltage value - 50 mV"]
    _10,
    #[doc = "Target voltage value - 75 mV"]
    _11,
}
impl DCDC_LP_STATE_HYS_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCDC_LP_STATE_HYS_LW::_00 => 0,
            DCDC_LP_STATE_HYS_LW::_01 => 1,
            DCDC_LP_STATE_HYS_LW::_10 => 2,
            DCDC_LP_STATE_HYS_LW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_LP_STATE_HYS_LW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_LP_STATE_HYS_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCDC_LP_STATE_HYS_LW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Target voltage value - 0 mV"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DCDC_LP_STATE_HYS_LW::_00)
    }
    #[doc = "Target voltage value - 25 mV"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DCDC_LP_STATE_HYS_LW::_01)
    }
    #[doc = "Target voltage value - 50 mV"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DCDC_LP_STATE_HYS_LW::_10)
    }
    #[doc = "Target voltage value - 75 mV"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DCDC_LP_STATE_HYS_LW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCDC_LP_STATE_HYS_H`"]
pub enum DCDC_LP_STATE_HYS_HW {
    #[doc = "Target voltage value + 0 mV"]
    _00,
    #[doc = "Target voltage value + 25 mV"]
    _01,
    #[doc = "Target voltage value + 50 mV"]
    _10,
    #[doc = "Target voltage value + 75 mV"]
    _11,
}
impl DCDC_LP_STATE_HYS_HW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCDC_LP_STATE_HYS_HW::_00 => 0,
            DCDC_LP_STATE_HYS_HW::_01 => 1,
            DCDC_LP_STATE_HYS_HW::_10 => 2,
            DCDC_LP_STATE_HYS_HW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_LP_STATE_HYS_HW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_LP_STATE_HYS_HW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCDC_LP_STATE_HYS_HW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Target voltage value + 0 mV"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DCDC_LP_STATE_HYS_HW::_00)
    }
    #[doc = "Target voltage value + 25 mV"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DCDC_LP_STATE_HYS_HW::_01)
    }
    #[doc = "Target voltage value + 50 mV"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DCDC_LP_STATE_HYS_HW::_10)
    }
    #[doc = "Target voltage value + 75 mV"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DCDC_LP_STATE_HYS_HW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HYST_LP_COMP_ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _HYST_LP_COMP_ADJW<'a> {
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
pub struct _HYST_LP_CMP_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _HYST_LP_CMP_DISABLEW<'a> {
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
pub struct _OFFSET_RSNS_LP_ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFSET_RSNS_LP_ADJW<'a> {
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
#[doc = r" Proxy"]
pub struct _OFFSET_RSNS_LP_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFSET_RSNS_LP_DISABLEW<'a> {
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
pub struct _DCDC_LESS_IW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_LESS_IW<'a> {
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
#[doc = r" Proxy"]
pub struct _PWD_CMP_OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PWD_CMP_OFFSETW<'a> {
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
pub struct _DCDC_XTALOK_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_XTALOK_DISABLEW<'a> {
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
#[doc = r" Proxy"]
pub struct _VLPS_CONFIG_DCDC_HPW<'a> {
    w: &'a mut W,
}
impl<'a> _VLPS_CONFIG_DCDC_HPW<'a> {
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
#[doc = r" Proxy"]
pub struct _VLPR_VLPW_CONFIG_DCDC_HPW<'a> {
    w: &'a mut W,
}
impl<'a> _VLPR_VLPW_CONFIG_DCDC_HPW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Disable automatic clock switch from internal oscillator to external clock."]
    #[inline]
    pub fn dcdc_disable_auto_clk_switch(&self) -> DCDC_DISABLE_AUTO_CLK_SWITCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_DISABLE_AUTO_CLK_SWITCHR { bits }
    }
    #[doc = "Bit 2 - Select external clock for DCDC when DCDC_DISABLE_AUTO_CLK_SWITCH is set."]
    #[inline]
    pub fn dcdc_sel_clk(&self) -> DCDC_SEL_CLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_SEL_CLKR { bits }
    }
    #[doc = "Bit 3 - Power down internal oscillator. Only set this bit when 32M crystal oscillator is available."]
    #[inline]
    pub fn dcdc_pwd_osc_int(&self) -> DCDC_PWD_OSC_INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_PWD_OSC_INTR { bits }
    }
    #[doc = "Bit 9 - Enable low power differential comparators, to sense lower supply in pulsed mode"]
    #[inline]
    pub fn dcdc_lp_df_cmp_enable(&self) -> DCDC_LP_DF_CMP_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_LP_DF_CMP_ENABLER { bits }
    }
    #[doc = "Bits 10:11 - Controls VBAT voltage divider"]
    #[inline]
    pub fn dcdc_vbat_div_ctrl(&self) -> DCDC_VBAT_DIV_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCDC_VBAT_DIV_CTRLR { bits }
    }
    #[doc = "Bits 17:18 - Configure the hysteretic lower threshold value in low power mode"]
    #[inline]
    pub fn dcdc_lp_state_hys_l(&self) -> DCDC_LP_STATE_HYS_LR {
        DCDC_LP_STATE_HYS_LR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:20 - Configure the hysteretic upper threshold value in low power mode"]
    #[inline]
    pub fn dcdc_lp_state_hys_h(&self) -> DCDC_LP_STATE_HYS_HR {
        DCDC_LP_STATE_HYS_HR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - Adjust hysteretic value in low power comparator."]
    #[inline]
    pub fn hyst_lp_comp_adj(&self) -> HYST_LP_COMP_ADJR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HYST_LP_COMP_ADJR { bits }
    }
    #[doc = "Bit 22 - Disable hysteresis in low power comparator."]
    #[inline]
    pub fn hyst_lp_cmp_disable(&self) -> HYST_LP_CMP_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HYST_LP_CMP_DISABLER { bits }
    }
    #[doc = "Bit 23 - Adjust hysteretic value in low power voltage sense."]
    #[inline]
    pub fn offset_rsns_lp_adj(&self) -> OFFSET_RSNS_LP_ADJR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OFFSET_RSNS_LP_ADJR { bits }
    }
    #[doc = "Bit 24 - Disable hysteresis in low power voltage sense."]
    #[inline]
    pub fn offset_rsns_lp_disable(&self) -> OFFSET_RSNS_LP_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OFFSET_RSNS_LP_DISABLER { bits }
    }
    #[doc = "Bit 25 - Reduce DCDC current. It will save approximately 20 uA in RUN."]
    #[inline]
    pub fn dcdc_less_i(&self) -> DCDC_LESS_IR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_LESS_IR { bits }
    }
    #[doc = "Bit 26 - Power down output range comparator"]
    #[inline]
    pub fn pwd_cmp_offset(&self) -> PWD_CMP_OFFSETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWD_CMP_OFFSETR { bits }
    }
    #[doc = "Bit 27 - Disable xtalok detection circuit."]
    #[inline]
    pub fn dcdc_xtalok_disable(&self) -> DCDC_XTALOK_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_XTALOK_DISABLER { bits }
    }
    #[doc = "Bit 28 - Status register to indicate PSWITCH status"]
    #[inline]
    pub fn pswitch_status(&self) -> PSWITCH_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PSWITCH_STATUSR { bits }
    }
    #[doc = "Bit 29 - Selects behavior of DCDC in device VLPS low power mode"]
    #[inline]
    pub fn vlps_config_dcdc_hp(&self) -> VLPS_CONFIG_DCDC_HPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VLPS_CONFIG_DCDC_HPR { bits }
    }
    #[doc = "Bit 30 - Selects behavior of DCDC in device VLPR and VLPW low power modes"]
    #[inline]
    pub fn vlpr_vlpw_config_dcdc_hp(&self) -> VLPR_VLPW_CONFIG_DCDC_HPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VLPR_VLPW_CONFIG_DCDC_HPR { bits }
    }
    #[doc = "Bit 31 - Status register to indicate DCDC lock"]
    #[inline]
    pub fn dcdc_sts_dc_ok(&self) -> DCDC_STS_DC_OKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_STS_DC_OKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 68681728 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Disable automatic clock switch from internal oscillator to external clock."]
    #[inline]
    pub fn dcdc_disable_auto_clk_switch(&mut self) -> _DCDC_DISABLE_AUTO_CLK_SWITCHW {
        _DCDC_DISABLE_AUTO_CLK_SWITCHW { w: self }
    }
    #[doc = "Bit 2 - Select external clock for DCDC when DCDC_DISABLE_AUTO_CLK_SWITCH is set."]
    #[inline]
    pub fn dcdc_sel_clk(&mut self) -> _DCDC_SEL_CLKW {
        _DCDC_SEL_CLKW { w: self }
    }
    #[doc = "Bit 3 - Power down internal oscillator. Only set this bit when 32M crystal oscillator is available."]
    #[inline]
    pub fn dcdc_pwd_osc_int(&mut self) -> _DCDC_PWD_OSC_INTW {
        _DCDC_PWD_OSC_INTW { w: self }
    }
    #[doc = "Bit 9 - Enable low power differential comparators, to sense lower supply in pulsed mode"]
    #[inline]
    pub fn dcdc_lp_df_cmp_enable(&mut self) -> _DCDC_LP_DF_CMP_ENABLEW {
        _DCDC_LP_DF_CMP_ENABLEW { w: self }
    }
    #[doc = "Bits 10:11 - Controls VBAT voltage divider"]
    #[inline]
    pub fn dcdc_vbat_div_ctrl(&mut self) -> _DCDC_VBAT_DIV_CTRLW {
        _DCDC_VBAT_DIV_CTRLW { w: self }
    }
    #[doc = "Bits 17:18 - Configure the hysteretic lower threshold value in low power mode"]
    #[inline]
    pub fn dcdc_lp_state_hys_l(&mut self) -> _DCDC_LP_STATE_HYS_LW {
        _DCDC_LP_STATE_HYS_LW { w: self }
    }
    #[doc = "Bits 19:20 - Configure the hysteretic upper threshold value in low power mode"]
    #[inline]
    pub fn dcdc_lp_state_hys_h(&mut self) -> _DCDC_LP_STATE_HYS_HW {
        _DCDC_LP_STATE_HYS_HW { w: self }
    }
    #[doc = "Bit 21 - Adjust hysteretic value in low power comparator."]
    #[inline]
    pub fn hyst_lp_comp_adj(&mut self) -> _HYST_LP_COMP_ADJW {
        _HYST_LP_COMP_ADJW { w: self }
    }
    #[doc = "Bit 22 - Disable hysteresis in low power comparator."]
    #[inline]
    pub fn hyst_lp_cmp_disable(&mut self) -> _HYST_LP_CMP_DISABLEW {
        _HYST_LP_CMP_DISABLEW { w: self }
    }
    #[doc = "Bit 23 - Adjust hysteretic value in low power voltage sense."]
    #[inline]
    pub fn offset_rsns_lp_adj(&mut self) -> _OFFSET_RSNS_LP_ADJW {
        _OFFSET_RSNS_LP_ADJW { w: self }
    }
    #[doc = "Bit 24 - Disable hysteresis in low power voltage sense."]
    #[inline]
    pub fn offset_rsns_lp_disable(&mut self) -> _OFFSET_RSNS_LP_DISABLEW {
        _OFFSET_RSNS_LP_DISABLEW { w: self }
    }
    #[doc = "Bit 25 - Reduce DCDC current. It will save approximately 20 uA in RUN."]
    #[inline]
    pub fn dcdc_less_i(&mut self) -> _DCDC_LESS_IW {
        _DCDC_LESS_IW { w: self }
    }
    #[doc = "Bit 26 - Power down output range comparator"]
    #[inline]
    pub fn pwd_cmp_offset(&mut self) -> _PWD_CMP_OFFSETW {
        _PWD_CMP_OFFSETW { w: self }
    }
    #[doc = "Bit 27 - Disable xtalok detection circuit."]
    #[inline]
    pub fn dcdc_xtalok_disable(&mut self) -> _DCDC_XTALOK_DISABLEW {
        _DCDC_XTALOK_DISABLEW { w: self }
    }
    #[doc = "Bit 29 - Selects behavior of DCDC in device VLPS low power mode"]
    #[inline]
    pub fn vlps_config_dcdc_hp(&mut self) -> _VLPS_CONFIG_DCDC_HPW {
        _VLPS_CONFIG_DCDC_HPW { w: self }
    }
    #[doc = "Bit 30 - Selects behavior of DCDC in device VLPR and VLPW low power modes"]
    #[inline]
    pub fn vlpr_vlpw_config_dcdc_hp(&mut self) -> _VLPR_VLPW_CONFIG_DCDC_HPW {
        _VLPR_VLPW_CONFIG_DCDC_HPW { w: self }
    }
}
