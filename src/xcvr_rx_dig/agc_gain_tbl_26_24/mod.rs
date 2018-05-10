#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AGC_GAIN_TBL_26_24 {
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
pub struct BBA_GAIN_24R {
    bits: u8,
}
impl BBA_GAIN_24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_24R {
    bits: u8,
}
impl LNA_GAIN_24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_GAIN_25R {
    bits: u8,
}
impl BBA_GAIN_25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_25R {
    bits: u8,
}
impl LNA_GAIN_25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_GAIN_26R {
    bits: u8,
}
impl BBA_GAIN_26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_26R {
    bits: u8,
}
impl LNA_GAIN_26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BBA_GAIN_24W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_24W<'a> {
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
pub struct _LNA_GAIN_24W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_24W<'a> {
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
pub struct _BBA_GAIN_25W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_25W<'a> {
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
pub struct _LNA_GAIN_25W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_25W<'a> {
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
pub struct _BBA_GAIN_26W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_GAIN_26W<'a> {
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
pub struct _LNA_GAIN_26W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_26W<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - BBA Gain 24"]
    #[inline]
    pub fn bba_gain_24(&self) -> BBA_GAIN_24R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_24R { bits }
    }
    #[doc = "Bits 4:7 - LNA Gain 24"]
    #[inline]
    pub fn lna_gain_24(&self) -> LNA_GAIN_24R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_24R { bits }
    }
    #[doc = "Bits 8:11 - BBA Gain 25"]
    #[inline]
    pub fn bba_gain_25(&self) -> BBA_GAIN_25R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_25R { bits }
    }
    #[doc = "Bits 12:15 - LNA Gain 25"]
    #[inline]
    pub fn lna_gain_25(&self) -> LNA_GAIN_25R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_25R { bits }
    }
    #[doc = "Bits 16:19 - BBA Gain 26"]
    #[inline]
    pub fn bba_gain_26(&self) -> BBA_GAIN_26R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_GAIN_26R { bits }
    }
    #[doc = "Bits 20:23 - LNA Gain 26"]
    #[inline]
    pub fn lna_gain_26(&self) -> LNA_GAIN_26R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_26R { bits }
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
    #[doc = "Bits 0:3 - BBA Gain 24"]
    #[inline]
    pub fn bba_gain_24(&mut self) -> _BBA_GAIN_24W {
        _BBA_GAIN_24W { w: self }
    }
    #[doc = "Bits 4:7 - LNA Gain 24"]
    #[inline]
    pub fn lna_gain_24(&mut self) -> _LNA_GAIN_24W {
        _LNA_GAIN_24W { w: self }
    }
    #[doc = "Bits 8:11 - BBA Gain 25"]
    #[inline]
    pub fn bba_gain_25(&mut self) -> _BBA_GAIN_25W {
        _BBA_GAIN_25W { w: self }
    }
    #[doc = "Bits 12:15 - LNA Gain 25"]
    #[inline]
    pub fn lna_gain_25(&mut self) -> _LNA_GAIN_25W {
        _LNA_GAIN_25W { w: self }
    }
    #[doc = "Bits 16:19 - BBA Gain 26"]
    #[inline]
    pub fn bba_gain_26(&mut self) -> _BBA_GAIN_26W {
        _BBA_GAIN_26W { w: self }
    }
    #[doc = "Bits 20:23 - LNA Gain 26"]
    #[inline]
    pub fn lna_gain_26(&mut self) -> _LNA_GAIN_26W {
        _LNA_GAIN_26W { w: self }
    }
}
