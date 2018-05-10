#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG1 {
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
pub struct POSLIMIT_BUCK_INR {
    bits: u8,
}
impl POSLIMIT_BUCK_INR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct POSLIMIT_BOOST_INR {
    bits: u8,
}
impl POSLIMIT_BOOST_INR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCDC_LOOPCTRL_CM_HST_THRESHR {
    bits: bool,
}
impl DCDC_LOOPCTRL_CM_HST_THRESHR {
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
pub struct DCDC_LOOPCTRL_DF_HST_THRESHR {
    bits: bool,
}
impl DCDC_LOOPCTRL_DF_HST_THRESHR {
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
pub struct DCDC_LOOPCTRL_EN_CM_HYSTR {
    bits: bool,
}
impl DCDC_LOOPCTRL_EN_CM_HYSTR {
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
pub struct DCDC_LOOPCTRL_EN_DF_HYSTR {
    bits: bool,
}
impl DCDC_LOOPCTRL_EN_DF_HYSTR {
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
pub struct _POSLIMIT_BUCK_INW<'a> {
    w: &'a mut W,
}
impl<'a> _POSLIMIT_BUCK_INW<'a> {
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
#[doc = r" Proxy"]
pub struct _POSLIMIT_BOOST_INW<'a> {
    w: &'a mut W,
}
impl<'a> _POSLIMIT_BOOST_INW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_LOOPCTRL_CM_HST_THRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_LOOPCTRL_CM_HST_THRESHW<'a> {
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
pub struct _DCDC_LOOPCTRL_DF_HST_THRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_LOOPCTRL_DF_HST_THRESHW<'a> {
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
pub struct _DCDC_LOOPCTRL_EN_CM_HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_LOOPCTRL_EN_CM_HYSTW<'a> {
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
pub struct _DCDC_LOOPCTRL_EN_DF_HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_LOOPCTRL_EN_DF_HYSTW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Upper limit duty cycle limit in DC-DC converter"]
    #[inline]
    pub fn poslimit_buck_in(&self) -> POSLIMIT_BUCK_INR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        POSLIMIT_BUCK_INR { bits }
    }
    #[doc = "Bits 7:13 - Upper limit duty cycle limit in DC-DC converter"]
    #[inline]
    pub fn poslimit_boost_in(&self) -> POSLIMIT_BOOST_INR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        POSLIMIT_BOOST_INR { bits }
    }
    #[doc = "Bit 21 - Enable hysteresis in switching converter common mode analog comparators"]
    #[inline]
    pub fn dcdc_loopctrl_cm_hst_thresh(&self) -> DCDC_LOOPCTRL_CM_HST_THRESHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_LOOPCTRL_CM_HST_THRESHR { bits }
    }
    #[doc = "Bit 22 - Enable hysteresis in switching converter differential mode analog comparators"]
    #[inline]
    pub fn dcdc_loopctrl_df_hst_thresh(&self) -> DCDC_LOOPCTRL_DF_HST_THRESHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_LOOPCTRL_DF_HST_THRESHR { bits }
    }
    #[doc = "Bit 23 - Enable hysteresis in switching converter common mode analog comparators"]
    #[inline]
    pub fn dcdc_loopctrl_en_cm_hyst(&self) -> DCDC_LOOPCTRL_EN_CM_HYSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_LOOPCTRL_EN_CM_HYSTR { bits }
    }
    #[doc = "Bit 24 - Enable hysteresis in switching converter differential mode analog comparators"]
    #[inline]
    pub fn dcdc_loopctrl_en_df_hyst(&self) -> DCDC_LOOPCTRL_EN_DF_HYSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_LOOPCTRL_EN_DF_HYSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1557020 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Upper limit duty cycle limit in DC-DC converter"]
    #[inline]
    pub fn poslimit_buck_in(&mut self) -> _POSLIMIT_BUCK_INW {
        _POSLIMIT_BUCK_INW { w: self }
    }
    #[doc = "Bits 7:13 - Upper limit duty cycle limit in DC-DC converter"]
    #[inline]
    pub fn poslimit_boost_in(&mut self) -> _POSLIMIT_BOOST_INW {
        _POSLIMIT_BOOST_INW { w: self }
    }
    #[doc = "Bit 21 - Enable hysteresis in switching converter common mode analog comparators"]
    #[inline]
    pub fn dcdc_loopctrl_cm_hst_thresh(&mut self) -> _DCDC_LOOPCTRL_CM_HST_THRESHW {
        _DCDC_LOOPCTRL_CM_HST_THRESHW { w: self }
    }
    #[doc = "Bit 22 - Enable hysteresis in switching converter differential mode analog comparators"]
    #[inline]
    pub fn dcdc_loopctrl_df_hst_thresh(&mut self) -> _DCDC_LOOPCTRL_DF_HST_THRESHW {
        _DCDC_LOOPCTRL_DF_HST_THRESHW { w: self }
    }
    #[doc = "Bit 23 - Enable hysteresis in switching converter common mode analog comparators"]
    #[inline]
    pub fn dcdc_loopctrl_en_cm_hyst(&mut self) -> _DCDC_LOOPCTRL_EN_CM_HYSTW {
        _DCDC_LOOPCTRL_EN_CM_HYSTW { w: self }
    }
    #[doc = "Bit 24 - Enable hysteresis in switching converter differential mode analog comparators"]
    #[inline]
    pub fn dcdc_loopctrl_en_df_hyst(&mut self) -> _DCDC_LOOPCTRL_EN_DF_HYSTW {
        _DCDC_LOOPCTRL_EN_DF_HYSTW { w: self }
    }
}
