#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DC_RESID_CTRL {
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
pub struct DC_RESID_NWINR {
    bits: u8,
}
impl DC_RESID_NWINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DC_RESID_ITER_FREEZER {
    bits: u8,
}
impl DC_RESID_ITER_FREEZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DC_RESID_ALPHAR {
    bits: u8,
}
impl DC_RESID_ALPHAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DC_RESID_DLYR {
    bits: u8,
}
impl DC_RESID_DLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DC_RESID_EXT_DC_ENR {
    bits: bool,
}
impl DC_RESID_EXT_DC_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _DC_RESID_NWINW<'a> {
    w: &'a mut W,
}
impl<'a> _DC_RESID_NWINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DC_RESID_ITER_FREEZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DC_RESID_ITER_FREEZEW<'a> {
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
pub struct _DC_RESID_ALPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _DC_RESID_ALPHAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DC_RESID_DLYW<'a> {
    w: &'a mut W,
}
impl<'a> _DC_RESID_DLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DC_RESID_EXT_DC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DC_RESID_EXT_DC_ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DC_RESID_MIN_AGC_IDXW<'a> {
    w: &'a mut W,
}
impl<'a> _DC_RESID_MIN_AGC_IDXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 0:6 - DC Residual NWIN"]
    #[inline]
    pub fn dc_resid_nwin(&self) -> DC_RESID_NWINR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DC_RESID_NWINR { bits }
    }
    #[doc = "Bits 8:11 - DC Residual Iteration Freeze"]
    #[inline]
    pub fn dc_resid_iter_freeze(&self) -> DC_RESID_ITER_FREEZER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DC_RESID_ITER_FREEZER { bits }
    }
    #[doc = "Bits 12:14 - DC Residual Alpha"]
    #[inline]
    pub fn dc_resid_alpha(&self) -> DC_RESID_ALPHAR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DC_RESID_ALPHAR { bits }
    }
    #[doc = "Bits 16:18 - DC Residual Delay"]
    #[inline]
    pub fn dc_resid_dly(&self) -> DC_RESID_DLYR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DC_RESID_DLYR { bits }
    }
    #[doc = "Bit 20 - DC Residual External DC Enable"]
    #[inline]
    pub fn dc_resid_ext_dc_en(&self) -> DC_RESID_EXT_DC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DC_RESID_EXT_DC_ENR { bits }
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
    #[doc = "Bits 0:6 - DC Residual NWIN"]
    #[inline]
    pub fn dc_resid_nwin(&mut self) -> _DC_RESID_NWINW {
        _DC_RESID_NWINW { w: self }
    }
    #[doc = "Bits 8:11 - DC Residual Iteration Freeze"]
    #[inline]
    pub fn dc_resid_iter_freeze(&mut self) -> _DC_RESID_ITER_FREEZEW {
        _DC_RESID_ITER_FREEZEW { w: self }
    }
    #[doc = "Bits 12:14 - DC Residual Alpha"]
    #[inline]
    pub fn dc_resid_alpha(&mut self) -> _DC_RESID_ALPHAW {
        _DC_RESID_ALPHAW { w: self }
    }
    #[doc = "Bits 16:18 - DC Residual Delay"]
    #[inline]
    pub fn dc_resid_dly(&mut self) -> _DC_RESID_DLYW {
        _DC_RESID_DLYW { w: self }
    }
    #[doc = "Bit 20 - DC Residual External DC Enable"]
    #[inline]
    pub fn dc_resid_ext_dc_en(&mut self) -> _DC_RESID_EXT_DC_ENW {
        _DC_RESID_EXT_DC_ENW { w: self }
    }
    #[doc = "Bits 24:28 - DC Residual Minimum AGC Table Index"]
    #[inline]
    pub fn dc_resid_min_agc_idx(&mut self) -> _DC_RESID_MIN_AGC_IDXW {
        _DC_RESID_MIN_AGC_IDXW { w: self }
    }
}
