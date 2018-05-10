#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCA_ED_LQI_CTRL_0 {
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
pub struct LQI_CORR_THRESHR {
    bits: u8,
}
impl LQI_CORR_THRESHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CORR_CNTR_THRESHR {
    bits: u8,
}
impl CORR_CNTR_THRESHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LQI_CNTRR {
    bits: u8,
}
impl LQI_CNTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SNR_ADJR {
    bits: u8,
}
impl SNR_ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _LQI_CORR_THRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _LQI_CORR_THRESHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CORR_CNTR_THRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _CORR_CNTR_THRESHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LQI_CNTRW<'a> {
    w: &'a mut W,
}
impl<'a> _LQI_CNTRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SNR_ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _SNR_ADJW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bits 0:7 - LQI Correlation Threshold"]
    #[inline]
    pub fn lqi_corr_thresh(&self) -> LQI_CORR_THRESHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LQI_CORR_THRESHR { bits }
    }
    #[doc = "Bits 8:15 - Correlation Count Threshold"]
    #[inline]
    pub fn corr_cntr_thresh(&self) -> CORR_CNTR_THRESHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CORR_CNTR_THRESHR { bits }
    }
    #[doc = "Bits 16:23 - LQI Counter"]
    #[inline]
    pub fn lqi_cntr(&self) -> LQI_CNTRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LQI_CNTRR { bits }
    }
    #[doc = "Bits 24:29 - SNR calculation adjustment"]
    #[inline]
    pub fn snr_adj(&self) -> SNR_ADJR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SNR_ADJR { bits }
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
    #[doc = "Bits 0:7 - LQI Correlation Threshold"]
    #[inline]
    pub fn lqi_corr_thresh(&mut self) -> _LQI_CORR_THRESHW {
        _LQI_CORR_THRESHW { w: self }
    }
    #[doc = "Bits 8:15 - Correlation Count Threshold"]
    #[inline]
    pub fn corr_cntr_thresh(&mut self) -> _CORR_CNTR_THRESHW {
        _CORR_CNTR_THRESHW { w: self }
    }
    #[doc = "Bits 16:23 - LQI Counter"]
    #[inline]
    pub fn lqi_cntr(&mut self) -> _LQI_CNTRW {
        _LQI_CNTRW { w: self }
    }
    #[doc = "Bits 24:29 - SNR calculation adjustment"]
    #[inline]
    pub fn snr_adj(&mut self) -> _SNR_ADJW {
        _SNR_ADJW { w: self }
    }
}
