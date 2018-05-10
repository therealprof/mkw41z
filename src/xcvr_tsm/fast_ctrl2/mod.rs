#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FAST_CTRL2 {
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
pub struct FAST_START_TXR {
    bits: u8,
}
impl FAST_START_TXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FAST_DEST_TXR {
    bits: u8,
}
impl FAST_DEST_TXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FAST_START_RXR {
    bits: u8,
}
impl FAST_START_RXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FAST_DEST_RXR {
    bits: u8,
}
impl FAST_DEST_RXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _FAST_START_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _FAST_START_TXW<'a> {
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
pub struct _FAST_DEST_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _FAST_DEST_TXW<'a> {
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
pub struct _FAST_START_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _FAST_START_RXW<'a> {
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
pub struct _FAST_DEST_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _FAST_DEST_RXW<'a> {
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
    #[doc = "Bits 0:7 - Fast TSM TX \"Jump-from\" Point"]
    #[inline]
    pub fn fast_start_tx(&self) -> FAST_START_TXR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FAST_START_TXR { bits }
    }
    #[doc = "Bits 8:15 - Fast TSM TX \"Jump-to\" Point"]
    #[inline]
    pub fn fast_dest_tx(&self) -> FAST_DEST_TXR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FAST_DEST_TXR { bits }
    }
    #[doc = "Bits 16:23 - Fast TSM RX \"Jump-from\" Point"]
    #[inline]
    pub fn fast_start_rx(&self) -> FAST_START_RXR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FAST_START_RXR { bits }
    }
    #[doc = "Bits 24:31 - Fast TSM RX \"Jump-to\" Point"]
    #[inline]
    pub fn fast_dest_rx(&self) -> FAST_DEST_RXR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FAST_DEST_RXR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Fast TSM TX \"Jump-from\" Point"]
    #[inline]
    pub fn fast_start_tx(&mut self) -> _FAST_START_TXW {
        _FAST_START_TXW { w: self }
    }
    #[doc = "Bits 8:15 - Fast TSM TX \"Jump-to\" Point"]
    #[inline]
    pub fn fast_dest_tx(&mut self) -> _FAST_DEST_TXW {
        _FAST_DEST_TXW { w: self }
    }
    #[doc = "Bits 16:23 - Fast TSM RX \"Jump-from\" Point"]
    #[inline]
    pub fn fast_start_rx(&mut self) -> _FAST_START_RXW {
        _FAST_START_RXW { w: self }
    }
    #[doc = "Bits 24:31 - Fast TSM RX \"Jump-to\" Point"]
    #[inline]
    pub fn fast_dest_rx(&mut self) -> _FAST_DEST_RXW {
        _FAST_DEST_RXW { w: self }
    }
}
