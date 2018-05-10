#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMING17 {
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
pub struct SY_LO_TX_BUF_EN_TX_HIR {
    bits: u8,
}
impl SY_LO_TX_BUF_EN_TX_HIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SY_LO_TX_BUF_EN_TX_LOR {
    bits: u8,
}
impl SY_LO_TX_BUF_EN_TX_LOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SY_LO_TX_BUF_EN_TX_HIW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_TX_BUF_EN_TX_HIW<'a> {
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
pub struct _SY_LO_TX_BUF_EN_TX_LOW<'a> {
    w: &'a mut W,
}
impl<'a> _SY_LO_TX_BUF_EN_TX_LOW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Assertion time setting for SY_LO_TX_BUF_EN (TX)"]
    #[inline]
    pub fn sy_lo_tx_buf_en_tx_hi(&self) -> SY_LO_TX_BUF_EN_TX_HIR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SY_LO_TX_BUF_EN_TX_HIR { bits }
    }
    #[doc = "Bits 8:15 - De-assertion time setting for SY_LO_TX_BUF_EN (TX)"]
    #[inline]
    pub fn sy_lo_tx_buf_en_tx_lo(&self) -> SY_LO_TX_BUF_EN_TX_LOR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SY_LO_TX_BUF_EN_TX_LOR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294928986 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Assertion time setting for SY_LO_TX_BUF_EN (TX)"]
    #[inline]
    pub fn sy_lo_tx_buf_en_tx_hi(&mut self) -> _SY_LO_TX_BUF_EN_TX_HIW {
        _SY_LO_TX_BUF_EN_TX_HIW { w: self }
    }
    #[doc = "Bits 8:15 - De-assertion time setting for SY_LO_TX_BUF_EN (TX)"]
    #[inline]
    pub fn sy_lo_tx_buf_en_tx_lo(&mut self) -> _SY_LO_TX_BUF_EN_TX_LOW {
        _SY_LO_TX_BUF_EN_TX_LOW { w: self }
    }
}
