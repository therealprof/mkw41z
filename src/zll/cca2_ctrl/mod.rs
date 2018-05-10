#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCA2_CTRL {
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
pub struct CCA2_NUM_CORR_PEAKSR {
    bits: u8,
}
impl CCA2_NUM_CORR_PEAKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CCA2_MIN_NUM_CORR_THR {
    bits: u8,
}
impl CCA2_MIN_NUM_CORR_THR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CCA2_CORR_THRESHR {
    bits: u8,
}
impl CCA2_CORR_THRESHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CCA2_MIN_NUM_CORR_THW<'a> {
    w: &'a mut W,
}
impl<'a> _CCA2_MIN_NUM_CORR_THW<'a> {
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
#[doc = r" Proxy"]
pub struct _CCA2_CORR_THRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _CCA2_CORR_THRESHW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - CCA Mode 2 Number of Correlation Peaks Detected"]
    #[inline]
    pub fn cca2_num_corr_peaks(&self) -> CCA2_NUM_CORR_PEAKSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCA2_NUM_CORR_PEAKSR { bits }
    }
    #[doc = "Bits 4:6 - CCA Mode 2 Threshold Number of Correlation Peaks"]
    #[inline]
    pub fn cca2_min_num_corr_th(&self) -> CCA2_MIN_NUM_CORR_THR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCA2_MIN_NUM_CORR_THR { bits }
    }
    #[doc = "Bits 8:15 - CCA Mode 2 Correlation Threshold"]
    #[inline]
    pub fn cca2_corr_thresh(&self) -> CCA2_CORR_THRESHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCA2_CORR_THRESHR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 33328 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 4:6 - CCA Mode 2 Threshold Number of Correlation Peaks"]
    #[inline]
    pub fn cca2_min_num_corr_th(&mut self) -> _CCA2_MIN_NUM_CORR_THW {
        _CCA2_MIN_NUM_CORR_THW { w: self }
    }
    #[doc = "Bits 8:15 - CCA Mode 2 Correlation Threshold"]
    #[inline]
    pub fn cca2_corr_thresh(&mut self) -> _CCA2_CORR_THRESHW {
        _CCA2_CORR_THRESHW { w: self }
    }
}
