#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MOD_CTRL {
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
pub struct MODULATION_WORD_MANUALR {
    bits: u16,
}
impl MODULATION_WORD_MANUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MOD_DISABLER {
    bits: bool,
}
impl MOD_DISABLER {
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
#[doc = r" Value of the field"]
pub struct HPM_MOD_MANUALR {
    bits: u8,
}
impl HPM_MOD_MANUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPM_MOD_DISABLER {
    bits: bool,
}
impl HPM_MOD_DISABLER {
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
#[doc = r" Value of the field"]
pub struct HPM_SDM_OUT_MANUALR {
    bits: u8,
}
impl HPM_SDM_OUT_MANUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPM_SDM_OUT_DISABLER {
    bits: bool,
}
impl HPM_SDM_OUT_DISABLER {
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
pub struct _MODULATION_WORD_MANUALW<'a> {
    w: &'a mut W,
}
impl<'a> _MODULATION_WORD_MANUALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MOD_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _MOD_DISABLEW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_MOD_MANUALW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_MOD_MANUALW<'a> {
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
pub struct _HPM_MOD_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_MOD_DISABLEW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_SDM_OUT_MANUALW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_SDM_OUT_MANUALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_SDM_OUT_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_SDM_OUT_DISABLEW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:12 - Manual Modulation Word"]
    #[inline]
    pub fn modulation_word_manual(&self) -> MODULATION_WORD_MANUALR {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MODULATION_WORD_MANUALR { bits }
    }
    #[doc = "Bit 15 - Disable Modulation Word"]
    #[inline]
    pub fn mod_disable(&self) -> MOD_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MOD_DISABLER { bits }
    }
    #[doc = "Bits 16:23 - Manual HPM Modulation"]
    #[inline]
    pub fn hpm_mod_manual(&self) -> HPM_MOD_MANUALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPM_MOD_MANUALR { bits }
    }
    #[doc = "Bit 27 - Disable HPM Modulation"]
    #[inline]
    pub fn hpm_mod_disable(&self) -> HPM_MOD_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_MOD_DISABLER { bits }
    }
    #[doc = "Bits 28:29 - Manual HPM SDM out"]
    #[inline]
    pub fn hpm_sdm_out_manual(&self) -> HPM_SDM_OUT_MANUALR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPM_SDM_OUT_MANUALR { bits }
    }
    #[doc = "Bit 31 - Disable HPM SDM out"]
    #[inline]
    pub fn hpm_sdm_out_disable(&self) -> HPM_SDM_OUT_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_SDM_OUT_DISABLER { bits }
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
    #[doc = "Bits 0:12 - Manual Modulation Word"]
    #[inline]
    pub fn modulation_word_manual(&mut self) -> _MODULATION_WORD_MANUALW {
        _MODULATION_WORD_MANUALW { w: self }
    }
    #[doc = "Bit 15 - Disable Modulation Word"]
    #[inline]
    pub fn mod_disable(&mut self) -> _MOD_DISABLEW {
        _MOD_DISABLEW { w: self }
    }
    #[doc = "Bits 16:23 - Manual HPM Modulation"]
    #[inline]
    pub fn hpm_mod_manual(&mut self) -> _HPM_MOD_MANUALW {
        _HPM_MOD_MANUALW { w: self }
    }
    #[doc = "Bit 27 - Disable HPM Modulation"]
    #[inline]
    pub fn hpm_mod_disable(&mut self) -> _HPM_MOD_DISABLEW {
        _HPM_MOD_DISABLEW { w: self }
    }
    #[doc = "Bits 28:29 - Manual HPM SDM out"]
    #[inline]
    pub fn hpm_sdm_out_manual(&mut self) -> _HPM_SDM_OUT_MANUALW {
        _HPM_SDM_OUT_MANUALW { w: self }
    }
    #[doc = "Bit 31 - Disable HPM SDM out"]
    #[inline]
    pub fn hpm_sdm_out_disable(&mut self) -> _HPM_SDM_OUT_DISABLEW {
        _HPM_SDM_OUT_DISABLEW { w: self }
    }
}
