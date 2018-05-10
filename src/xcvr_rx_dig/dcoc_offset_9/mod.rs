#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCOC_OFFSET_9 {
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
pub struct DCOC_BBA_OFFSET_IR {
    bits: u8,
}
impl DCOC_BBA_OFFSET_IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_BBA_OFFSET_QR {
    bits: u8,
}
impl DCOC_BBA_OFFSET_QR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_TZA_OFFSET_IR {
    bits: u8,
}
impl DCOC_TZA_OFFSET_IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCOC_TZA_OFFSET_QR {
    bits: u8,
}
impl DCOC_TZA_OFFSET_QR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_BBA_OFFSET_IW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_BBA_OFFSET_IW<'a> {
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
pub struct _DCOC_BBA_OFFSET_QW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_BBA_OFFSET_QW<'a> {
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
pub struct _DCOC_TZA_OFFSET_IW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_TZA_OFFSET_IW<'a> {
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
pub struct _DCOC_TZA_OFFSET_QW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_TZA_OFFSET_QW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:5 - DCOC BBA I-channel offset"]
    #[inline]
    pub fn dcoc_bba_offset_i(&self) -> DCOC_BBA_OFFSET_IR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_BBA_OFFSET_IR { bits }
    }
    #[doc = "Bits 8:13 - DCOC BBA Q-channel offset"]
    #[inline]
    pub fn dcoc_bba_offset_q(&self) -> DCOC_BBA_OFFSET_QR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_BBA_OFFSET_QR { bits }
    }
    #[doc = "Bits 16:23 - DCOC TZA I-channel offset"]
    #[inline]
    pub fn dcoc_tza_offset_i(&self) -> DCOC_TZA_OFFSET_IR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_TZA_OFFSET_IR { bits }
    }
    #[doc = "Bits 24:31 - DCOC TZA Q-channel offset"]
    #[inline]
    pub fn dcoc_tza_offset_q(&self) -> DCOC_TZA_OFFSET_QR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCOC_TZA_OFFSET_QR { bits }
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
    #[doc = "Bits 0:5 - DCOC BBA I-channel offset"]
    #[inline]
    pub fn dcoc_bba_offset_i(&mut self) -> _DCOC_BBA_OFFSET_IW {
        _DCOC_BBA_OFFSET_IW { w: self }
    }
    #[doc = "Bits 8:13 - DCOC BBA Q-channel offset"]
    #[inline]
    pub fn dcoc_bba_offset_q(&mut self) -> _DCOC_BBA_OFFSET_QW {
        _DCOC_BBA_OFFSET_QW { w: self }
    }
    #[doc = "Bits 16:23 - DCOC TZA I-channel offset"]
    #[inline]
    pub fn dcoc_tza_offset_i(&mut self) -> _DCOC_TZA_OFFSET_IW {
        _DCOC_TZA_OFFSET_IW { w: self }
    }
    #[doc = "Bits 24:31 - DCOC TZA Q-channel offset"]
    #[inline]
    pub fn dcoc_tza_offset_q(&mut self) -> _DCOC_TZA_OFFSET_QW {
        _DCOC_TZA_OFFSET_QW { w: self }
    }
}
