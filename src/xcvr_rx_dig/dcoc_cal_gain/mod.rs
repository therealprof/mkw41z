#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCOC_CAL_GAIN {
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
pub struct DCOC_BBA_CAL_GAIN1R {
    bits: u8,
}
impl DCOC_BBA_CAL_GAIN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_LNA_CAL_GAIN1R {
    bits: u8,
}
impl DCOC_LNA_CAL_GAIN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_BBA_CAL_GAIN2R {
    bits: u8,
}
impl DCOC_BBA_CAL_GAIN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_LNA_CAL_GAIN2R {
    bits: u8,
}
impl DCOC_LNA_CAL_GAIN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_BBA_CAL_GAIN3R {
    bits: u8,
}
impl DCOC_BBA_CAL_GAIN3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_LNA_CAL_GAIN3R {
    bits: u8,
}
impl DCOC_LNA_CAL_GAIN3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_BBA_CAL_GAIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_BBA_CAL_GAIN1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_LNA_CAL_GAIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_LNA_CAL_GAIN1W<'a> {
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
#[doc = r" Proxy"]
pub struct _DCOC_BBA_CAL_GAIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_BBA_CAL_GAIN2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_LNA_CAL_GAIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_LNA_CAL_GAIN2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_BBA_CAL_GAIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_BBA_CAL_GAIN3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_LNA_CAL_GAIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_LNA_CAL_GAIN3W<'a> {
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
    #[doc = "Bits 8:11 - DCOC BBA Calibration Gain 1"]
    #[inline]
    pub fn dcoc_bba_cal_gain1(&self) -> DCOC_BBA_CAL_GAIN1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_BBA_CAL_GAIN1R { bits }
    }
    #[doc = "Bits 12:15 - DCOC LNA Calibration Gain 1"]
    #[inline]
    pub fn dcoc_lna_cal_gain1(&self) -> DCOC_LNA_CAL_GAIN1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_LNA_CAL_GAIN1R { bits }
    }
    #[doc = "Bits 16:19 - DCOC BBA Calibration Gain 2"]
    #[inline]
    pub fn dcoc_bba_cal_gain2(&self) -> DCOC_BBA_CAL_GAIN2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_BBA_CAL_GAIN2R { bits }
    }
    #[doc = "Bits 20:23 - DCOC LNA Calibration Gain 2"]
    #[inline]
    pub fn dcoc_lna_cal_gain2(&self) -> DCOC_LNA_CAL_GAIN2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_LNA_CAL_GAIN2R { bits }
    }
    #[doc = "Bits 24:27 - DCOC BBA Calibration Gain 3"]
    #[inline]
    pub fn dcoc_bba_cal_gain3(&self) -> DCOC_BBA_CAL_GAIN3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_BBA_CAL_GAIN3R { bits }
    }
    #[doc = "Bits 28:31 - DCOC LNA Calibration Gain 3"]
    #[inline]
    pub fn dcoc_lna_cal_gain3(&self) -> DCOC_LNA_CAL_GAIN3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_LNA_CAL_GAIN3R { bits }
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
    #[doc = "Bits 8:11 - DCOC BBA Calibration Gain 1"]
    #[inline]
    pub fn dcoc_bba_cal_gain1(&mut self) -> _DCOC_BBA_CAL_GAIN1W {
        _DCOC_BBA_CAL_GAIN1W { w: self }
    }
    #[doc = "Bits 12:15 - DCOC LNA Calibration Gain 1"]
    #[inline]
    pub fn dcoc_lna_cal_gain1(&mut self) -> _DCOC_LNA_CAL_GAIN1W {
        _DCOC_LNA_CAL_GAIN1W { w: self }
    }
    #[doc = "Bits 16:19 - DCOC BBA Calibration Gain 2"]
    #[inline]
    pub fn dcoc_bba_cal_gain2(&mut self) -> _DCOC_BBA_CAL_GAIN2W {
        _DCOC_BBA_CAL_GAIN2W { w: self }
    }
    #[doc = "Bits 20:23 - DCOC LNA Calibration Gain 2"]
    #[inline]
    pub fn dcoc_lna_cal_gain2(&mut self) -> _DCOC_LNA_CAL_GAIN2W {
        _DCOC_LNA_CAL_GAIN2W { w: self }
    }
    #[doc = "Bits 24:27 - DCOC BBA Calibration Gain 3"]
    #[inline]
    pub fn dcoc_bba_cal_gain3(&mut self) -> _DCOC_BBA_CAL_GAIN3W {
        _DCOC_BBA_CAL_GAIN3W { w: self }
    }
    #[doc = "Bits 28:31 - DCOC LNA Calibration Gain 3"]
    #[inline]
    pub fn dcoc_lna_cal_gain3(&mut self) -> _DCOC_LNA_CAL_GAIN3W {
        _DCOC_LNA_CAL_GAIN3W { w: self }
    }
}
