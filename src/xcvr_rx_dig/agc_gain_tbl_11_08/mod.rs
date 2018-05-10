#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AGC_GAIN_TBL_11_08 {
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
pub struct BBA_GAIN_08R {
    bits: u8,
}
impl BBA_GAIN_08R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_08R {
    bits: u8,
}
impl LNA_GAIN_08R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_GAIN_09R {
    bits: u8,
}
impl BBA_GAIN_09R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_09R {
    bits: u8,
}
impl LNA_GAIN_09R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_GAIN_10R {
    bits: u8,
}
impl BBA_GAIN_10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_10R {
    bits: u8,
}
impl LNA_GAIN_10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_GAIN_11R {
    bits: u8,
}
impl BBA_GAIN_11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_11R {
    bits: u8,
}
impl LNA_GAIN_11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BBA_GAIN_08W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_08W<'a> {
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
pub struct _LNA_GAIN_08W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_08W<'a> {
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
pub struct _BBA_GAIN_09W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_09W<'a> {
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
pub struct _LNA_GAIN_09W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_09W<'a> {
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
pub struct _BBA_GAIN_10W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_10W<'a> {
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
pub struct _LNA_GAIN_10W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_10W<'a> {
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
pub struct _BBA_GAIN_11W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_11W<'a> {
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
pub struct _LNA_GAIN_11W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_11W<'a> {
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
    #[doc = "Bits 0:3 - BBA Gain 08"]
    #[inline]
    pub fn bba_gain_08(&self) -> BBA_GAIN_08R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_08R { bits }
    }
    #[doc = "Bits 4:7 - LNA Gain 08"]
    #[inline]
    pub fn lna_gain_08(&self) -> LNA_GAIN_08R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_08R { bits }
    }
    #[doc = "Bits 8:11 - BBA Gain 09"]
    #[inline]
    pub fn bba_gain_09(&self) -> BBA_GAIN_09R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_09R { bits }
    }
    #[doc = "Bits 12:15 - LNA Gain 09"]
    #[inline]
    pub fn lna_gain_09(&self) -> LNA_GAIN_09R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_09R { bits }
    }
    #[doc = "Bits 16:19 - BBA Gain 10"]
    #[inline]
    pub fn bba_gain_10(&self) -> BBA_GAIN_10R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_10R { bits }
    }
    #[doc = "Bits 20:23 - LNA Gain 10"]
    #[inline]
    pub fn lna_gain_10(&self) -> LNA_GAIN_10R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_10R { bits }
    }
    #[doc = "Bits 24:27 - BBA Gain 11"]
    #[inline]
    pub fn bba_gain_11(&self) -> BBA_GAIN_11R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_11R { bits }
    }
    #[doc = "Bits 28:31 - LNA Gain 11"]
    #[inline]
    pub fn lna_gain_11(&self) -> LNA_GAIN_11R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_11R { bits }
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
    #[doc = "Bits 0:3 - BBA Gain 08"]
    #[inline]
    pub fn bba_gain_08(&mut self) -> _BBA_GAIN_08W {
        _BBA_GAIN_08W { w: self }
    }
    #[doc = "Bits 4:7 - LNA Gain 08"]
    #[inline]
    pub fn lna_gain_08(&mut self) -> _LNA_GAIN_08W {
        _LNA_GAIN_08W { w: self }
    }
    #[doc = "Bits 8:11 - BBA Gain 09"]
    #[inline]
    pub fn bba_gain_09(&mut self) -> _BBA_GAIN_09W {
        _BBA_GAIN_09W { w: self }
    }
    #[doc = "Bits 12:15 - LNA Gain 09"]
    #[inline]
    pub fn lna_gain_09(&mut self) -> _LNA_GAIN_09W {
        _LNA_GAIN_09W { w: self }
    }
    #[doc = "Bits 16:19 - BBA Gain 10"]
    #[inline]
    pub fn bba_gain_10(&mut self) -> _BBA_GAIN_10W {
        _BBA_GAIN_10W { w: self }
    }
    #[doc = "Bits 20:23 - LNA Gain 10"]
    #[inline]
    pub fn lna_gain_10(&mut self) -> _LNA_GAIN_10W {
        _LNA_GAIN_10W { w: self }
    }
    #[doc = "Bits 24:27 - BBA Gain 11"]
    #[inline]
    pub fn bba_gain_11(&mut self) -> _BBA_GAIN_11W {
        _BBA_GAIN_11W { w: self }
    }
    #[doc = "Bits 28:31 - LNA Gain 11"]
    #[inline]
    pub fn lna_gain_11(&mut self) -> _LNA_GAIN_11W {
        _LNA_GAIN_11W { w: self }
    }
}
