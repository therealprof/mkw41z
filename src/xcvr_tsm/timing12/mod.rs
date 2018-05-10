#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMING12 {
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
pub struct RXTX_AUXPLL_VCO_REF_CLK_EN_RX_HIR {
    bits: u8,
}
impl RXTX_AUXPLL_VCO_REF_CLK_EN_RX_HIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXTX_AUXPLL_VCO_REF_CLK_EN_RX_LOR {
    bits: u8,
}
impl RXTX_AUXPLL_VCO_REF_CLK_EN_RX_LOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RXTX_AUXPLL_VCO_REF_CLK_EN_RX_HIW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_VCO_REF_CLK_EN_RX_HIW<'a> {
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
pub struct _RXTX_AUXPLL_VCO_REF_CLK_EN_RX_LOW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_AUXPLL_VCO_REF_CLK_EN_RX_LOW<'a> {
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
    #[doc = "Bits 16:23 - Assertion time setting for RXTX_AUXPLL_VCO_REF_CLK_EN (RX)"]
    #[inline]
    pub fn rxtx_auxpll_vco_ref_clk_en_rx_hi(&self) -> RXTX_AUXPLL_VCO_REF_CLK_EN_RX_HIR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXTX_AUXPLL_VCO_REF_CLK_EN_RX_HIR { bits }
    }
    #[doc = "Bits 24:31 - De-assertion time setting for RXTX_AUXPLL_VCO_REF_CLK_EN (RX)"]
    #[inline]
    pub fn rxtx_auxpll_vco_ref_clk_en_rx_lo(&self) -> RXTX_AUXPLL_VCO_REF_CLK_EN_RX_LOR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXTX_AUXPLL_VCO_REF_CLK_EN_RX_LOR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1728315391 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 16:23 - Assertion time setting for RXTX_AUXPLL_VCO_REF_CLK_EN (RX)"]
    #[inline]
    pub fn rxtx_auxpll_vco_ref_clk_en_rx_hi(&mut self) -> _RXTX_AUXPLL_VCO_REF_CLK_EN_RX_HIW {
        _RXTX_AUXPLL_VCO_REF_CLK_EN_RX_HIW { w: self }
    }
    #[doc = "Bits 24:31 - De-assertion time setting for RXTX_AUXPLL_VCO_REF_CLK_EN (RX)"]
    #[inline]
    pub fn rxtx_auxpll_vco_ref_clk_en_rx_lo(&mut self) -> _RXTX_AUXPLL_VCO_REF_CLK_EN_RX_LOW {
        _RXTX_AUXPLL_VCO_REF_CLK_EN_RX_LOW { w: self }
    }
}
