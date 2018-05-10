#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PA_RAMP_TBL1 {
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
pub struct PA_RAMP4R {
    bits: u8,
}
impl PA_RAMP4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA_RAMP5R {
    bits: u8,
}
impl PA_RAMP5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA_RAMP6R {
    bits: u8,
}
impl PA_RAMP6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PA_RAMP7R {
    bits: u8,
}
impl PA_RAMP7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PA_RAMP4W<'a> {
    w: &'a mut W,
}
impl<'a> _PA_RAMP4W<'a> {
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
pub struct _PA_RAMP5W<'a> {
    w: &'a mut W,
}
impl<'a> _PA_RAMP5W<'a> {
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
pub struct _PA_RAMP6W<'a> {
    w: &'a mut W,
}
impl<'a> _PA_RAMP6W<'a> {
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
pub struct _PA_RAMP7W<'a> {
    w: &'a mut W,
}
impl<'a> _PA_RAMP7W<'a> {
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
    #[doc = "Bits 0:5 - PA_RAMP4"]
    #[inline]
    pub fn pa_ramp4(&self) -> PA_RAMP4R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA_RAMP4R { bits }
    }
    #[doc = "Bits 8:13 - PA_RAMP5"]
    #[inline]
    pub fn pa_ramp5(&self) -> PA_RAMP5R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA_RAMP5R { bits }
    }
    #[doc = "Bits 16:21 - PA_RAMP6"]
    #[inline]
    pub fn pa_ramp6(&self) -> PA_RAMP6R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA_RAMP6R { bits }
    }
    #[doc = "Bits 24:29 - PA_RAMP7"]
    #[inline]
    pub fn pa_ramp7(&self) -> PA_RAMP7R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA_RAMP7R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 976497180 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - PA_RAMP4"]
    #[inline]
    pub fn pa_ramp4(&mut self) -> _PA_RAMP4W {
        _PA_RAMP4W { w: self }
    }
    #[doc = "Bits 8:13 - PA_RAMP5"]
    #[inline]
    pub fn pa_ramp5(&mut self) -> _PA_RAMP5W {
        _PA_RAMP5W { w: self }
    }
    #[doc = "Bits 16:21 - PA_RAMP6"]
    #[inline]
    pub fn pa_ramp6(&mut self) -> _PA_RAMP6W {
        _PA_RAMP6W { w: self }
    }
    #[doc = "Bits 24:29 - PA_RAMP7"]
    #[inline]
    pub fn pa_ramp7(&mut self) -> _PA_RAMP7W {
        _PA_RAMP7W { w: self }
    }
}
