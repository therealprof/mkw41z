#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AGC_CTRL_3 {
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
pub struct AGC_UNFREEZE_TIMER {
    bits: u16,
}
impl AGC_UNFREEZE_TIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AGC_PDET_LO_DLYR {
    bits: u8,
}
impl AGC_PDET_LO_DLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AGC_RSSI_DELT_H2SR {
    bits: u8,
}
impl AGC_RSSI_DELT_H2SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AGC_H2S_STEP_SZR {
    bits: u8,
}
impl AGC_H2S_STEP_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AGC_UP_STEP_SZR {
    bits: u8,
}
impl AGC_UP_STEP_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _AGC_UNFREEZE_TIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_UNFREEZE_TIMEW<'a> {
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
pub struct _AGC_PDET_LO_DLYW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_PDET_LO_DLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AGC_RSSI_DELT_H2SW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_RSSI_DELT_H2SW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AGC_H2S_STEP_SZW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_H2S_STEP_SZW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AGC_UP_STEP_SZW<'a> {
    w: &'a mut W,
}
impl<'a> _AGC_UP_STEP_SZW<'a> {
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
    #[doc = "Bits 0:12 - AGC Unfreeze Time"]
    #[inline]
    pub fn agc_unfreeze_time(&self) -> AGC_UNFREEZE_TIMER {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        AGC_UNFREEZE_TIMER { bits }
    }
    #[doc = "Bits 13:15 - AGC Peak Detect Low Delay"]
    #[inline]
    pub fn agc_pdet_lo_dly(&self) -> AGC_PDET_LO_DLYR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AGC_PDET_LO_DLYR { bits }
    }
    #[doc = "Bits 16:22 - AGC_RSSI_DELT_H2S"]
    #[inline]
    pub fn agc_rssi_delt_h2s(&self) -> AGC_RSSI_DELT_H2SR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AGC_RSSI_DELT_H2SR { bits }
    }
    #[doc = "Bits 23:27 - AGC_H2S_STEP_SZ"]
    #[inline]
    pub fn agc_h2s_step_sz(&self) -> AGC_H2S_STEP_SZR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AGC_H2S_STEP_SZR { bits }
    }
    #[doc = "Bits 28:31 - AGC Up Step Size"]
    #[inline]
    pub fn agc_up_step_sz(&self) -> AGC_UP_STEP_SZR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AGC_UP_STEP_SZR { bits }
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
    #[doc = "Bits 0:12 - AGC Unfreeze Time"]
    #[inline]
    pub fn agc_unfreeze_time(&mut self) -> _AGC_UNFREEZE_TIMEW {
        _AGC_UNFREEZE_TIMEW { w: self }
    }
    #[doc = "Bits 13:15 - AGC Peak Detect Low Delay"]
    #[inline]
    pub fn agc_pdet_lo_dly(&mut self) -> _AGC_PDET_LO_DLYW {
        _AGC_PDET_LO_DLYW { w: self }
    }
    #[doc = "Bits 16:22 - AGC_RSSI_DELT_H2S"]
    #[inline]
    pub fn agc_rssi_delt_h2s(&mut self) -> _AGC_RSSI_DELT_H2SW {
        _AGC_RSSI_DELT_H2SW { w: self }
    }
    #[doc = "Bits 23:27 - AGC_H2S_STEP_SZ"]
    #[inline]
    pub fn agc_h2s_step_sz(&mut self) -> _AGC_H2S_STEP_SZW {
        _AGC_H2S_STEP_SZW { w: self }
    }
    #[doc = "Bits 28:31 - AGC Up Step Size"]
    #[inline]
    pub fn agc_up_step_sz(&mut self) -> _AGC_UP_STEP_SZW {
        _AGC_UP_STEP_SZW { w: self }
    }
}
