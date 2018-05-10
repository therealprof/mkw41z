#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AGC_GAIN_TBL_23_20 {
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
pub struct BBA_GAIN_20R {
    bits: u8,
}
impl BBA_GAIN_20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_20R {
    bits: u8,
}
impl LNA_GAIN_20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_GAIN_21R {
    bits: u8,
}
impl BBA_GAIN_21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_21R {
    bits: u8,
}
impl LNA_GAIN_21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_GAIN_22R {
    bits: u8,
}
impl BBA_GAIN_22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_22R {
    bits: u8,
}
impl LNA_GAIN_22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_GAIN_23R {
    bits: u8,
}
impl BBA_GAIN_23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_23R {
    bits: u8,
}
impl LNA_GAIN_23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BBA_GAIN_20W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_20W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LNA_GAIN_20W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_20W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BBA_GAIN_21W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_21W<'a> {
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
pub struct _LNA_GAIN_21W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_21W<'a> {
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
pub struct _BBA_GAIN_22W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_22W<'a> {
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
pub struct _LNA_GAIN_22W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_22W<'a> {
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
pub struct _BBA_GAIN_23W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_23W<'a> {
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
pub struct _LNA_GAIN_23W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_23W<'a> {
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
    #[doc = "Bits 0:3 - BBA Gain 20"]
    #[inline]
    pub fn bba_gain_20(&self) -> BBA_GAIN_20R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_20R { bits }
    }
    #[doc = "Bits 4:7 - LNA Gain 20"]
    #[inline]
    pub fn lna_gain_20(&self) -> LNA_GAIN_20R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_20R { bits }
    }
    #[doc = "Bits 8:11 - BBA Gain 21"]
    #[inline]
    pub fn bba_gain_21(&self) -> BBA_GAIN_21R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_21R { bits }
    }
    #[doc = "Bits 12:15 - LNA Gain 21"]
    #[inline]
    pub fn lna_gain_21(&self) -> LNA_GAIN_21R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_21R { bits }
    }
    #[doc = "Bits 16:19 - BBA Gain 22"]
    #[inline]
    pub fn bba_gain_22(&self) -> BBA_GAIN_22R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_22R { bits }
    }
    #[doc = "Bits 20:23 - LNA Gain 22"]
    #[inline]
    pub fn lna_gain_22(&self) -> LNA_GAIN_22R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_22R { bits }
    }
    #[doc = "Bits 24:27 - BBA Gain 23"]
    #[inline]
    pub fn bba_gain_23(&self) -> BBA_GAIN_23R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_23R { bits }
    }
    #[doc = "Bits 28:31 - LNA Gain 23"]
    #[inline]
    pub fn lna_gain_23(&self) -> LNA_GAIN_23R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_23R { bits }
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
    #[doc = "Bits 0:3 - BBA Gain 20"]
    #[inline]
    pub fn bba_gain_20(&mut self) -> _BBA_GAIN_20W {
        _BBA_GAIN_20W { w: self }
    }
    #[doc = "Bits 4:7 - LNA Gain 20"]
    #[inline]
    pub fn lna_gain_20(&mut self) -> _LNA_GAIN_20W {
        _LNA_GAIN_20W { w: self }
    }
    #[doc = "Bits 8:11 - BBA Gain 21"]
    #[inline]
    pub fn bba_gain_21(&mut self) -> _BBA_GAIN_21W {
        _BBA_GAIN_21W { w: self }
    }
    #[doc = "Bits 12:15 - LNA Gain 21"]
    #[inline]
    pub fn lna_gain_21(&mut self) -> _LNA_GAIN_21W {
        _LNA_GAIN_21W { w: self }
    }
    #[doc = "Bits 16:19 - BBA Gain 22"]
    #[inline]
    pub fn bba_gain_22(&mut self) -> _BBA_GAIN_22W {
        _BBA_GAIN_22W { w: self }
    }
    #[doc = "Bits 20:23 - LNA Gain 22"]
    #[inline]
    pub fn lna_gain_22(&mut self) -> _LNA_GAIN_22W {
        _LNA_GAIN_22W { w: self }
    }
    #[doc = "Bits 24:27 - BBA Gain 23"]
    #[inline]
    pub fn bba_gain_23(&mut self) -> _BBA_GAIN_23W {
        _BBA_GAIN_23W { w: self }
    }
    #[doc = "Bits 28:31 - LNA Gain 23"]
    #[inline]
    pub fn lna_gain_23(&mut self) -> _LNA_GAIN_23W {
        _LNA_GAIN_23W { w: self }
    }
}
