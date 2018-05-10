#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CORR_CTRL {
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
pub struct CORR_VTR {
    bits: u8,
}
impl CORR_VTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CORR_NVALR {
    bits: u8,
}
impl CORR_NVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAX_CORR_ENR {
    bits: bool,
}
impl MAX_CORR_ENR {
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
#[doc = "Possible values of the field `ZBDEM_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZBDEM_CLK_ONR {
    #[doc = "Normal Operation"]
    _0,
    #[doc = "Force 802.15.4 Demodulator Clock On (debug purposes only)"]
    _1,
}
impl ZBDEM_CLK_ONR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ZBDEM_CLK_ONR::_0 => false,
            ZBDEM_CLK_ONR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZBDEM_CLK_ONR {
        match value {
            false => ZBDEM_CLK_ONR::_0,
            true => ZBDEM_CLK_ONR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ZBDEM_CLK_ONR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ZBDEM_CLK_ONR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_MAX_CORRR {
    bits: u8,
}
impl RX_MAX_CORRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RX_MAX_PREAMBLER {
    bits: u8,
}
impl RX_MAX_PREAMBLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CORR_VTW<'a> {
    w: &'a mut W,
}
impl<'a> _CORR_VTW<'a> {
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
pub struct _CORR_NVALW<'a> {
    w: &'a mut W,
}
impl<'a> _CORR_NVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAX_CORR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _MAX_CORR_ENW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ZBDEM_CLK_ON`"]
pub enum ZBDEM_CLK_ONW {
    #[doc = "Normal Operation"]
    _0,
    #[doc = "Force 802.15.4 Demodulator Clock On (debug purposes only)"]
    _1,
}
impl ZBDEM_CLK_ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZBDEM_CLK_ONW::_0 => false,
            ZBDEM_CLK_ONW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZBDEM_CLK_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _ZBDEM_CLK_ONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZBDEM_CLK_ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal Operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ZBDEM_CLK_ONW::_0)
    }
    #[doc = "Force 802.15.4 Demodulator Clock On (debug purposes only)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ZBDEM_CLK_ONW::_1)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - CORR_VT"]
    #[inline]
    pub fn corr_vt(&self) -> CORR_VTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CORR_VTR { bits }
    }
    #[doc = "Bits 8:10 - CORR_NVAL"]
    #[inline]
    pub fn corr_nval(&self) -> CORR_NVALR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CORR_NVALR { bits }
    }
    #[doc = "Bit 11 - MAX_CORR_EN"]
    #[inline]
    pub fn max_corr_en(&self) -> MAX_CORR_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MAX_CORR_ENR { bits }
    }
    #[doc = "Bit 15 - Force 802.15.4 Demodulator Clock On"]
    #[inline]
    pub fn zbdem_clk_on(&self) -> ZBDEM_CLK_ONR {
        ZBDEM_CLK_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - RX_MAX_CORR"]
    #[inline]
    pub fn rx_max_corr(&self) -> RX_MAX_CORRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_MAX_CORRR { bits }
    }
    #[doc = "Bits 24:31 - RX_MAX_PREAMBLE"]
    #[inline]
    pub fn rx_max_preamble(&self) -> RX_MAX_PREAMBLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_MAX_PREAMBLER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1154 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - CORR_VT"]
    #[inline]
    pub fn corr_vt(&mut self) -> _CORR_VTW {
        _CORR_VTW { w: self }
    }
    #[doc = "Bits 8:10 - CORR_NVAL"]
    #[inline]
    pub fn corr_nval(&mut self) -> _CORR_NVALW {
        _CORR_NVALW { w: self }
    }
    #[doc = "Bit 11 - MAX_CORR_EN"]
    #[inline]
    pub fn max_corr_en(&mut self) -> _MAX_CORR_ENW {
        _MAX_CORR_ENW { w: self }
    }
    #[doc = "Bit 15 - Force 802.15.4 Demodulator Clock On"]
    #[inline]
    pub fn zbdem_clk_on(&mut self) -> _ZBDEM_CLK_ONW {
        _ZBDEM_CLK_ONW { w: self }
    }
}
