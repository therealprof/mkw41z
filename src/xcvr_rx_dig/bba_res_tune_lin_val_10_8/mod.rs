#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BBA_RES_TUNE_LIN_VAL_10_8 {
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
pub struct BBA_RES_TUNE_LIN_VAL_8R {
    bits: u16,
}
impl BBA_RES_TUNE_LIN_VAL_8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_RES_TUNE_LIN_VAL_9R {
    bits: u16,
}
impl BBA_RES_TUNE_LIN_VAL_9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_RES_TUNE_LIN_VAL_10R {
    bits: u16,
}
impl BBA_RES_TUNE_LIN_VAL_10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BBA_RES_TUNE_LIN_VAL_8W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_RES_TUNE_LIN_VAL_8W<'a> {
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
pub struct _BBA_RES_TUNE_LIN_VAL_9W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_RES_TUNE_LIN_VAL_9W<'a> {
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
pub struct _BBA_RES_TUNE_LIN_VAL_10W<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_RES_TUNE_LIN_VAL_10W<'a> {
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
    #[doc = "Bits 0:9 - BBA Resistor Tune Linear Gain Step 8"]
    #[inline]
    pub fn bba_res_tune_lin_val_8(&self) -> BBA_RES_TUNE_LIN_VAL_8R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BBA_RES_TUNE_LIN_VAL_8R { bits }
    }
    #[doc = "Bits 10:19 - BBA Resistor Tune Linear Gain Step 9"]
    #[inline]
    pub fn bba_res_tune_lin_val_9(&self) -> BBA_RES_TUNE_LIN_VAL_9R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BBA_RES_TUNE_LIN_VAL_9R { bits }
    }
    #[doc = "Bits 20:29 - BBA Resistor Tune Linear Gain Step 10"]
    #[inline]
    pub fn bba_res_tune_lin_val_10(&self) -> BBA_RES_TUNE_LIN_VAL_10R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BBA_RES_TUNE_LIN_VAL_10R { bits }
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
    #[doc = "Bits 0:9 - BBA Resistor Tune Linear Gain Step 8"]
    #[inline]
    pub fn bba_res_tune_lin_val_8(&mut self) -> _BBA_RES_TUNE_LIN_VAL_8W {
        _BBA_RES_TUNE_LIN_VAL_8W { w: self }
    }
    #[doc = "Bits 10:19 - BBA Resistor Tune Linear Gain Step 9"]
    #[inline]
    pub fn bba_res_tune_lin_val_9(&mut self) -> _BBA_RES_TUNE_LIN_VAL_9W {
        _BBA_RES_TUNE_LIN_VAL_9W { w: self }
    }
    #[doc = "Bits 20:29 - BBA Resistor Tune Linear Gain Step 10"]
    #[inline]
    pub fn bba_res_tune_lin_val_10(&mut self) -> _BBA_RES_TUNE_LIN_VAL_10W {
        _BBA_RES_TUNE_LIN_VAL_10W { w: self }
    }
}
