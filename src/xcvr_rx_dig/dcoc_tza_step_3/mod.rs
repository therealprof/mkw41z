#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCOC_TZA_STEP_3 {
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
pub struct DCOC_TZA_STEP_RCP_3R {
    bits: u16,
}
impl DCOC_TZA_STEP_RCP_3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_TZA_STEP_GAIN_3R {
    bits: u16,
}
impl DCOC_TZA_STEP_GAIN_3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_TZA_STEP_RCP_3W<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_TZA_STEP_RCP_3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_TZA_STEP_GAIN_3W<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_TZA_STEP_GAIN_3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
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
    #[doc = "Bits 0:12 - DCOC_TZA_STEP_RCP_3"]
    #[inline]
    pub fn dcoc_tza_step_rcp_3(&self) -> DCOC_TZA_STEP_RCP_3R {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DCOC_TZA_STEP_RCP_3R { bits }
    }
    #[doc = "Bits 16:27 - DCOC_TZA_STEP_GAIN_3"]
    #[inline]
    pub fn dcoc_tza_step_gain_3(&self) -> DCOC_TZA_STEP_GAIN_3R {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DCOC_TZA_STEP_GAIN_3R { bits }
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
    #[doc = "Bits 0:12 - DCOC_TZA_STEP_RCP_3"]
    #[inline]
    pub fn dcoc_tza_step_rcp_3(&mut self) -> _DCOC_TZA_STEP_RCP_3W {
        _DCOC_TZA_STEP_RCP_3W { w: self }
    }
    #[doc = "Bits 16:27 - DCOC_TZA_STEP_GAIN_3"]
    #[inline]
    pub fn dcoc_tza_step_gain_3(&mut self) -> _DCOC_TZA_STEP_GAIN_3W {
        _DCOC_TZA_STEP_GAIN_3W { w: self }
    }
}
