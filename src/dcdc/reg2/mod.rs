#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG2 {
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
pub struct DCDC_LOOPCTRL_HYST_SIGNR {
    bits: bool,
}
impl DCDC_LOOPCTRL_HYST_SIGNR {
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
pub struct DCDC_BATTMONITOR_EN_BATADJR {
    bits: bool,
}
impl DCDC_BATTMONITOR_EN_BATADJR {
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
pub struct DCDC_BATTMONITOR_BATT_VALR {
    bits: u16,
}
impl DCDC_BATTMONITOR_BATT_VALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_LOOPCTRL_HYST_SIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_LOOPCTRL_HYST_SIGNW<'a> {
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
#[doc = r" Proxy"]
pub struct _DCDC_BATTMONITOR_EN_BATADJW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_BATTMONITOR_EN_BATADJW<'a> {
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
#[doc = r" Proxy"]
pub struct _DCDC_BATTMONITOR_BATT_VALW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_BATTMONITOR_BATT_VALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 13 - Invert the sign of the hysteresis in DC-DC analog comparators. This bit is set when in Pulsed mode."]
    #[inline]
    pub fn dcdc_loopctrl_hyst_sign(&self) -> DCDC_LOOPCTRL_HYST_SIGNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_LOOPCTRL_HYST_SIGNR { bits }
    }
    #[doc = "Bit 15 - This bit enables the DC-DC to improve efficiency and minimize ripple using the information from the BATT_VAL field"]
    #[inline]
    pub fn dcdc_battmonitor_en_batadj(&self) -> DCDC_BATTMONITOR_EN_BATADJR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_BATTMONITOR_EN_BATADJR { bits }
    }
    #[doc = "Bits 16:25 - Software should be configured to place the battery voltage in this register measured with an 8 mV LSB resolution through the ADC"]
    #[inline]
    pub fn dcdc_battmonitor_batt_val(&self) -> DCDC_BATTMONITOR_BATT_VALR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DCDC_BATTMONITOR_BATT_VALR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16393 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 13 - Invert the sign of the hysteresis in DC-DC analog comparators. This bit is set when in Pulsed mode."]
    #[inline]
    pub fn dcdc_loopctrl_hyst_sign(&mut self) -> _DCDC_LOOPCTRL_HYST_SIGNW {
        _DCDC_LOOPCTRL_HYST_SIGNW { w: self }
    }
    #[doc = "Bit 15 - This bit enables the DC-DC to improve efficiency and minimize ripple using the information from the BATT_VAL field"]
    #[inline]
    pub fn dcdc_battmonitor_en_batadj(&mut self) -> _DCDC_BATTMONITOR_EN_BATADJW {
        _DCDC_BATTMONITOR_EN_BATADJW { w: self }
    }
    #[doc = "Bits 16:25 - Software should be configured to place the battery voltage in this register measured with an 8 mV LSB resolution through the ADC"]
    #[inline]
    pub fn dcdc_battmonitor_batt_val(&mut self) -> _DCDC_BATTMONITOR_BATT_VALW {
        _DCDC_BATTMONITOR_BATT_VALW { w: self }
    }
}
