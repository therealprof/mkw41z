#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LNA_GAIN_LIN_VAL_8_6 {
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
pub struct LNA_GAIN_LIN_VAL_6R {
    bits: u16,
}
impl LNA_GAIN_LIN_VAL_6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_LIN_VAL_7R {
    bits: u16,
}
impl LNA_GAIN_LIN_VAL_7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_LIN_VAL_8R {
    bits: u16,
}
impl LNA_GAIN_LIN_VAL_8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _LNA_GAIN_LIN_VAL_6W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_LIN_VAL_6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LNA_GAIN_LIN_VAL_7W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_LIN_VAL_7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LNA_GAIN_LIN_VAL_8W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_LIN_VAL_8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
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
    #[doc = "Bits 0:9 - LNA Linear Gain Step 6"]
    #[inline]
    pub fn lna_gain_lin_val_6(&self) -> LNA_GAIN_LIN_VAL_6R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LNA_GAIN_LIN_VAL_6R { bits }
    }
    #[doc = "Bits 10:19 - LNA Linear Gain Step 7"]
    #[inline]
    pub fn lna_gain_lin_val_7(&self) -> LNA_GAIN_LIN_VAL_7R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LNA_GAIN_LIN_VAL_7R { bits }
    }
    #[doc = "Bits 20:29 - LNA Linear Gain Step 8"]
    #[inline]
    pub fn lna_gain_lin_val_8(&self) -> LNA_GAIN_LIN_VAL_8R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LNA_GAIN_LIN_VAL_8R { bits }
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
    #[doc = "Bits 0:9 - LNA Linear Gain Step 6"]
    #[inline]
    pub fn lna_gain_lin_val_6(&mut self) -> _LNA_GAIN_LIN_VAL_6W {
        _LNA_GAIN_LIN_VAL_6W { w: self }
    }
    #[doc = "Bits 10:19 - LNA Linear Gain Step 7"]
    #[inline]
    pub fn lna_gain_lin_val_7(&mut self) -> _LNA_GAIN_LIN_VAL_7W {
        _LNA_GAIN_LIN_VAL_7W { w: self }
    }
    #[doc = "Bits 20:29 - LNA Linear Gain Step 8"]
    #[inline]
    pub fn lna_gain_lin_val_8(&mut self) -> _LNA_GAIN_LIN_VAL_8W {
        _LNA_GAIN_LIN_VAL_8W { w: self }
    }
}
