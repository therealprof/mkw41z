#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::END_OF_SEQ {
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
pub struct END_OF_TX_WUR {
    bits: u8,
}
impl END_OF_TX_WUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct END_OF_TX_WDR {
    bits: u8,
}
impl END_OF_TX_WDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct END_OF_RX_WUR {
    bits: u8,
}
impl END_OF_RX_WUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct END_OF_RX_WDR {
    bits: u8,
}
impl END_OF_RX_WDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _END_OF_TX_WUW<'a> {
    w: &'a mut W,
}
impl<'a> _END_OF_TX_WUW<'a> {
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
pub struct _END_OF_TX_WDW<'a> {
    w: &'a mut W,
}
impl<'a> _END_OF_TX_WDW<'a> {
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
pub struct _END_OF_RX_WUW<'a> {
    w: &'a mut W,
}
impl<'a> _END_OF_RX_WUW<'a> {
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
pub struct _END_OF_RX_WDW<'a> {
    w: &'a mut W,
}
impl<'a> _END_OF_RX_WDW<'a> {
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
    #[doc = "Bits 0:7 - End of TX Warmup"]
    #[inline]
    pub fn end_of_tx_wu(&self) -> END_OF_TX_WUR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        END_OF_TX_WUR { bits }
    }
    #[doc = "Bits 8:15 - End of TX Warmdown"]
    #[inline]
    pub fn end_of_tx_wd(&self) -> END_OF_TX_WDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        END_OF_TX_WDR { bits }
    }
    #[doc = "Bits 16:23 - End of RX Warmup"]
    #[inline]
    pub fn end_of_rx_wu(&self) -> END_OF_RX_WUR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        END_OF_RX_WUR { bits }
    }
    #[doc = "Bits 24:31 - End of RX Warmdown"]
    #[inline]
    pub fn end_of_rx_wd(&self) -> END_OF_RX_WDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        END_OF_RX_WDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1734765155 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - End of TX Warmup"]
    #[inline]
    pub fn end_of_tx_wu(&mut self) -> _END_OF_TX_WUW {
        _END_OF_TX_WUW { w: self }
    }
    #[doc = "Bits 8:15 - End of TX Warmdown"]
    #[inline]
    pub fn end_of_tx_wd(&mut self) -> _END_OF_TX_WDW {
        _END_OF_TX_WDW { w: self }
    }
    #[doc = "Bits 16:23 - End of RX Warmup"]
    #[inline]
    pub fn end_of_rx_wu(&mut self) -> _END_OF_RX_WUW {
        _END_OF_RX_WUW { w: self }
    }
    #[doc = "Bits 24:31 - End of RX Warmdown"]
    #[inline]
    pub fn end_of_rx_wd(&mut self) -> _END_OF_RX_WDW {
        _END_OF_RX_WDW { w: self }
    }
}
