#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AGC_GAIN_TBL_07_04 {
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
pub struct BBA_GAIN_04R {
    bits: u8,
}
impl BBA_GAIN_04R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_04R {
    bits: u8,
}
impl LNA_GAIN_04R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_GAIN_05R {
    bits: u8,
}
impl BBA_GAIN_05R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_05R {
    bits: u8,
}
impl LNA_GAIN_05R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_GAIN_06R {
    bits: u8,
}
impl BBA_GAIN_06R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_06R {
    bits: u8,
}
impl LNA_GAIN_06R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_GAIN_07R {
    bits: u8,
}
impl BBA_GAIN_07R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_07R {
    bits: u8,
}
impl LNA_GAIN_07R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BBA_GAIN_04W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_04W<'a> {
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
pub struct _LNA_GAIN_04W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_04W<'a> {
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
pub struct _BBA_GAIN_05W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_05W<'a> {
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
pub struct _LNA_GAIN_05W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_05W<'a> {
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
pub struct _BBA_GAIN_06W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_06W<'a> {
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
pub struct _LNA_GAIN_06W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_06W<'a> {
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
pub struct _BBA_GAIN_07W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_07W<'a> {
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
pub struct _LNA_GAIN_07W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_07W<'a> {
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
    #[doc = "Bits 0:3 - BBA Gain 04"]
    #[inline]
    pub fn bba_gain_04(&self) -> BBA_GAIN_04R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_04R { bits }
    }
    #[doc = "Bits 4:7 - LNA Gain 04"]
    #[inline]
    pub fn lna_gain_04(&self) -> LNA_GAIN_04R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_04R { bits }
    }
    #[doc = "Bits 8:11 - BBA Gain 05"]
    #[inline]
    pub fn bba_gain_05(&self) -> BBA_GAIN_05R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_05R { bits }
    }
    #[doc = "Bits 12:15 - LNA Gain 05"]
    #[inline]
    pub fn lna_gain_05(&self) -> LNA_GAIN_05R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_05R { bits }
    }
    #[doc = "Bits 16:19 - BBA Gain 06"]
    #[inline]
    pub fn bba_gain_06(&self) -> BBA_GAIN_06R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_06R { bits }
    }
    #[doc = "Bits 20:23 - LNA Gain 06"]
    #[inline]
    pub fn lna_gain_06(&self) -> LNA_GAIN_06R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_06R { bits }
    }
    #[doc = "Bits 24:27 - BBA Gain 07"]
    #[inline]
    pub fn bba_gain_07(&self) -> BBA_GAIN_07R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_07R { bits }
    }
    #[doc = "Bits 28:31 - LNA Gain 07"]
    #[inline]
    pub fn lna_gain_07(&self) -> LNA_GAIN_07R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_07R { bits }
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
    #[doc = "Bits 0:3 - BBA Gain 04"]
    #[inline]
    pub fn bba_gain_04(&mut self) -> _BBA_GAIN_04W {
        _BBA_GAIN_04W { w: self }
    }
    #[doc = "Bits 4:7 - LNA Gain 04"]
    #[inline]
    pub fn lna_gain_04(&mut self) -> _LNA_GAIN_04W {
        _LNA_GAIN_04W { w: self }
    }
    #[doc = "Bits 8:11 - BBA Gain 05"]
    #[inline]
    pub fn bba_gain_05(&mut self) -> _BBA_GAIN_05W {
        _BBA_GAIN_05W { w: self }
    }
    #[doc = "Bits 12:15 - LNA Gain 05"]
    #[inline]
    pub fn lna_gain_05(&mut self) -> _LNA_GAIN_05W {
        _LNA_GAIN_05W { w: self }
    }
    #[doc = "Bits 16:19 - BBA Gain 06"]
    #[inline]
    pub fn bba_gain_06(&mut self) -> _BBA_GAIN_06W {
        _BBA_GAIN_06W { w: self }
    }
    #[doc = "Bits 20:23 - LNA Gain 06"]
    #[inline]
    pub fn lna_gain_06(&mut self) -> _LNA_GAIN_06W {
        _LNA_GAIN_06W { w: self }
    }
    #[doc = "Bits 24:27 - BBA Gain 07"]
    #[inline]
    pub fn bba_gain_07(&mut self) -> _BBA_GAIN_07W {
        _BBA_GAIN_07W { w: self }
    }
    #[doc = "Bits 28:31 - LNA Gain 07"]
    #[inline]
    pub fn lna_gain_07(&mut self) -> _LNA_GAIN_07W {
        _LNA_GAIN_07W { w: self }
    }
}
