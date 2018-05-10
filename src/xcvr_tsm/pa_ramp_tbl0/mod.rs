#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PA_RAMP_TBL0 {
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
pub struct PA_RAMP0R {
    bits: u8,
}
impl PA_RAMP0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA_RAMP1R {
    bits: u8,
}
impl PA_RAMP1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA_RAMP2R {
    bits: u8,
}
impl PA_RAMP2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA_RAMP3R {
    bits: u8,
}
impl PA_RAMP3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PA_RAMP0W<'a> {
    w: &'a mut W,
}
impl<'a> _PA_RAMP0W<'a> {
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
#[doc = r" Proxy"]
pub struct _PA_RAMP1W<'a> {
    w: &'a mut W,
}
impl<'a> _PA_RAMP1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PA_RAMP2W<'a> {
    w: &'a mut W,
}
impl<'a> _PA_RAMP2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PA_RAMP3W<'a> {
    w: &'a mut W,
}
impl<'a> _PA_RAMP3W<'a> {
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
    #[doc = "Bits 0:5 - PA_RAMP0"]
    #[inline]
    pub fn pa_ramp0(&self) -> PA_RAMP0R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA_RAMP0R { bits }
    }
    #[doc = "Bits 8:13 - PA_RAMP1"]
    #[inline]
    pub fn pa_ramp1(&self) -> PA_RAMP1R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA_RAMP1R { bits }
    }
    #[doc = "Bits 16:21 - PA_RAMP2"]
    #[inline]
    pub fn pa_ramp2(&self) -> PA_RAMP2R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA_RAMP2R { bits }
    }
    #[doc = "Bits 24:29 - PA_RAMP3"]
    #[inline]
    pub fn pa_ramp3(&self) -> PA_RAMP3R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA_RAMP3R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 268960770 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - PA_RAMP0"]
    #[inline]
    pub fn pa_ramp0(&mut self) -> _PA_RAMP0W {
        _PA_RAMP0W { w: self }
    }
    #[doc = "Bits 8:13 - PA_RAMP1"]
    #[inline]
    pub fn pa_ramp1(&mut self) -> _PA_RAMP1W {
        _PA_RAMP1W { w: self }
    }
    #[doc = "Bits 16:21 - PA_RAMP2"]
    #[inline]
    pub fn pa_ramp2(&mut self) -> _PA_RAMP2W {
        _PA_RAMP2W { w: self }
    }
    #[doc = "Bits 24:29 - PA_RAMP3"]
    #[inline]
    pub fn pa_ramp3(&mut self) -> _PA_RAMP3W {
        _PA_RAMP3W { w: self }
    }
}
