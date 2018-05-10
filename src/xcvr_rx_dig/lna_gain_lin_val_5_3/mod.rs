#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LNA_GAIN_LIN_VAL_5_3 {
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
pub struct LNA_GAIN_LIN_VAL_3R {
    bits: u16,
}
impl LNA_GAIN_LIN_VAL_3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_LIN_VAL_4R {
    bits: u16,
}
impl LNA_GAIN_LIN_VAL_4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_GAIN_LIN_VAL_5R {
    bits: u16,
}
impl LNA_GAIN_LIN_VAL_5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _LNA_GAIN_LIN_VAL_3W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_LIN_VAL_3W<'a> {
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
pub struct _LNA_GAIN_LIN_VAL_4W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_LIN_VAL_4W<'a> {
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
pub struct _LNA_GAIN_LIN_VAL_5W<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_GAIN_LIN_VAL_5W<'a> {
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
    #[doc = "Bits 0:9 - LNA Linear Gain Step 3"]
    #[inline]
    pub fn lna_gain_lin_val_3(&self) -> LNA_GAIN_LIN_VAL_3R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LNA_GAIN_LIN_VAL_3R { bits }
    }
    #[doc = "Bits 10:19 - LNA Linear Gain Step 4"]
    #[inline]
    pub fn lna_gain_lin_val_4(&self) -> LNA_GAIN_LIN_VAL_4R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LNA_GAIN_LIN_VAL_4R { bits }
    }
    #[doc = "Bits 20:29 - LNA Linear Gain Step 5"]
    #[inline]
    pub fn lna_gain_lin_val_5(&self) -> LNA_GAIN_LIN_VAL_5R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LNA_GAIN_LIN_VAL_5R { bits }
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
    #[doc = "Bits 0:9 - LNA Linear Gain Step 3"]
    #[inline]
    pub fn lna_gain_lin_val_3(&mut self) -> _LNA_GAIN_LIN_VAL_3W {
        _LNA_GAIN_LIN_VAL_3W { w: self }
    }
    #[doc = "Bits 10:19 - LNA Linear Gain Step 4"]
    #[inline]
    pub fn lna_gain_lin_val_4(&mut self) -> _LNA_GAIN_LIN_VAL_4W {
        _LNA_GAIN_LIN_VAL_4W { w: self }
    }
    #[doc = "Bits 20:29 - LNA Linear Gain Step 5"]
    #[inline]
    pub fn lna_gain_lin_val_5(&mut self) -> _LNA_GAIN_LIN_VAL_5W {
        _LNA_GAIN_LIN_VAL_5W { w: self }
    }
}
