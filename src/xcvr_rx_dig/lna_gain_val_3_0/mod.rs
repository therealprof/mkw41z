#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LNA_GAIN_VAL_3_0 {
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
pub struct LNA_GAIN_VAL_0R {
    bits: u8,
}
impl LNA_GAIN_VAL_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_VAL_1R {
    bits: u8,
}
impl LNA_GAIN_VAL_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_VAL_2R {
    bits: u8,
}
impl LNA_GAIN_VAL_2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_VAL_3R {
    bits: u8,
}
impl LNA_GAIN_VAL_3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _LNA_GAIN_VAL_0W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_VAL_0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LNA_GAIN_VAL_1W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_VAL_1W<'a> {
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
#[doc = r" Proxy"]
pub struct _LNA_GAIN_VAL_2W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_VAL_2W<'a> {
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
pub struct _LNA_GAIN_VAL_3W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_VAL_3W<'a> {
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
    #[doc = "Bits 0:7 - LNA_GAIN step 0"]
    #[inline]
    pub fn lna_gain_val_0(&self) -> LNA_GAIN_VAL_0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_VAL_0R { bits }
    }
    #[doc = "Bits 8:15 - LNA_GAIN step 1"]
    #[inline]
    pub fn lna_gain_val_1(&self) -> LNA_GAIN_VAL_1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_VAL_1R { bits }
    }
    #[doc = "Bits 16:23 - LNA_GAIN step 2"]
    #[inline]
    pub fn lna_gain_val_2(&self) -> LNA_GAIN_VAL_2R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_VAL_2R { bits }
    }
    #[doc = "Bits 24:31 - LNA_GAIN step 3"]
    #[inline]
    pub fn lna_gain_val_3(&self) -> LNA_GAIN_VAL_3R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_GAIN_VAL_3R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 940126749 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - LNA_GAIN step 0"]
    #[inline]
    pub fn lna_gain_val_0(&mut self) -> _LNA_GAIN_VAL_0W {
        _LNA_GAIN_VAL_0W { w: self }
    }
    #[doc = "Bits 8:15 - LNA_GAIN step 1"]
    #[inline]
    pub fn lna_gain_val_1(&mut self) -> _LNA_GAIN_VAL_1W {
        _LNA_GAIN_VAL_1W { w: self }
    }
    #[doc = "Bits 16:23 - LNA_GAIN step 2"]
    #[inline]
    pub fn lna_gain_val_2(&mut self) -> _LNA_GAIN_VAL_2W {
        _LNA_GAIN_VAL_2W { w: self }
    }
    #[doc = "Bits 24:31 - LNA_GAIN step 3"]
    #[inline]
    pub fn lna_gain_val_3(&mut self) -> _LNA_GAIN_VAL_3W {
        _LNA_GAIN_VAL_3W { w: self }
    }
}
