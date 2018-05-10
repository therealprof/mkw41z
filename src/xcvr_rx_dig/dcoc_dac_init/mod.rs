#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCOC_DAC_INIT {
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
pub struct BBA_DCOC_INIT_IR {
    bits: u8,
}
impl BBA_DCOC_INIT_IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_DCOC_INIT_QR {
    bits: u8,
}
impl BBA_DCOC_INIT_QR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TZA_DCOC_INIT_IR {
    bits: u8,
}
impl TZA_DCOC_INIT_IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TZA_DCOC_INIT_QR {
    bits: u8,
}
impl TZA_DCOC_INIT_QR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BBA_DCOC_INIT_IW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_DCOC_INIT_IW<'a> {
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
pub struct _BBA_DCOC_INIT_QW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_DCOC_INIT_QW<'a> {
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
pub struct _TZA_DCOC_INIT_IW<'a> {
    w: &'a mut W,
}
impl<'a> _TZA_DCOC_INIT_IW<'a> {
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
pub struct _TZA_DCOC_INIT_QW<'a> {
    w: &'a mut W,
}
impl<'a> _TZA_DCOC_INIT_QW<'a> {
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
    #[doc = "Bits 0:5 - DCOC BBA Init I"]
    #[inline]
    pub fn bba_dcoc_init_i(&self) -> BBA_DCOC_INIT_IR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_DCOC_INIT_IR { bits }
    }
    #[doc = "Bits 8:13 - DCOC BBA Init Q"]
    #[inline]
    pub fn bba_dcoc_init_q(&self) -> BBA_DCOC_INIT_QR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_DCOC_INIT_QR { bits }
    }
    #[doc = "Bits 16:23 - DCOC TZA Init I"]
    #[inline]
    pub fn tza_dcoc_init_i(&self) -> TZA_DCOC_INIT_IR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TZA_DCOC_INIT_IR { bits }
    }
    #[doc = "Bits 24:31 - DCOC TZA Init Q"]
    #[inline]
    pub fn tza_dcoc_init_q(&self) -> TZA_DCOC_INIT_QR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TZA_DCOC_INIT_QR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2155880480 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - DCOC BBA Init I"]
    #[inline]
    pub fn bba_dcoc_init_i(&mut self) -> _BBA_DCOC_INIT_IW {
        _BBA_DCOC_INIT_IW { w: self }
    }
    #[doc = "Bits 8:13 - DCOC BBA Init Q"]
    #[inline]
    pub fn bba_dcoc_init_q(&mut self) -> _BBA_DCOC_INIT_QW {
        _BBA_DCOC_INIT_QW { w: self }
    }
    #[doc = "Bits 16:23 - DCOC TZA Init I"]
    #[inline]
    pub fn tza_dcoc_init_i(&mut self) -> _TZA_DCOC_INIT_IW {
        _TZA_DCOC_INIT_IW { w: self }
    }
    #[doc = "Bits 24:31 - DCOC TZA Init Q"]
    #[inline]
    pub fn tza_dcoc_init_q(&mut self) -> _TZA_DCOC_INIT_QW {
        _TZA_DCOC_INIT_QW { w: self }
    }
}
