#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_AUXPLL {
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
pub struct BIAS_TRIMR {
    bits: u8,
}
impl BIAS_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIAGSEL1R {
    bits: bool,
}
impl DIAGSEL1R {
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
pub struct DIAGSEL2R {
    bits: bool,
}
impl DIAGSEL2R {
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
pub struct LF_CNTLR {
    bits: u8,
}
impl LF_CNTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPARER {
    bits: u8,
}
impl SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VCO_DAC_REF_ADJUSTR {
    bits: u8,
}
impl VCO_DAC_REF_ADJUSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VTUNE_TESTMODER {
    bits: bool,
}
impl VTUNE_TESTMODER {
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
#[doc = "Possible values of the field `RXTX_BAL_BIAST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTX_BAL_BIASTR {
    #[doc = "0.6"]
    _0,
    #[doc = "0.4"]
    _1,
    #[doc = "0.9"]
    _2,
    #[doc = "1.2"]
    _3,
}
impl RXTX_BAL_BIASTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXTX_BAL_BIASTR::_0 => 0,
            RXTX_BAL_BIASTR::_1 => 1,
            RXTX_BAL_BIASTR::_2 => 2,
            RXTX_BAL_BIASTR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXTX_BAL_BIASTR {
        match value {
            0 => RXTX_BAL_BIASTR::_0,
            1 => RXTX_BAL_BIASTR::_1,
            2 => RXTX_BAL_BIASTR::_2,
            3 => RXTX_BAL_BIASTR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXTX_BAL_BIASTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXTX_BAL_BIASTR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == RXTX_BAL_BIASTR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == RXTX_BAL_BIASTR::_3
    }
}
#[doc = r" Value of the field"]
pub struct RXTX_BAL_SPARER {
    bits: u8,
}
impl RXTX_BAL_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXTX_RCCAL_DIAGSELR {
    bits: bool,
}
impl RXTX_RCCAL_DIAGSELR {
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
pub struct _BIAS_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BIAS_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIAGSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _DIAGSEL1W<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIAGSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _DIAGSEL2W<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LF_CNTLW<'a> {
    w: &'a mut W,
}
impl<'a> _LF_CNTLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _SPAREW<'a> {
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
pub struct _VCO_DAC_REF_ADJUSTW<'a> {
    w: &'a mut W,
}
impl<'a> _VCO_DAC_REF_ADJUSTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VTUNE_TESTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _VTUNE_TESTMODEW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXTX_BAL_BIAST`"]
pub enum RXTX_BAL_BIASTW {
    #[doc = "0.6"]
    _0,
    #[doc = "0.4"]
    _1,
    #[doc = "0.9"]
    _2,
    #[doc = "1.2"]
    _3,
}
impl RXTX_BAL_BIASTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXTX_BAL_BIASTW::_0 => 0,
            RXTX_BAL_BIASTW::_1 => 1,
            RXTX_BAL_BIASTW::_2 => 2,
            RXTX_BAL_BIASTW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTX_BAL_BIASTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_BAL_BIASTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTX_BAL_BIASTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0.6"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTX_BAL_BIASTW::_0)
    }
    #[doc = "0.4"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTX_BAL_BIASTW::_1)
    }
    #[doc = "0.9"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(RXTX_BAL_BIASTW::_2)
    }
    #[doc = "1.2"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(RXTX_BAL_BIASTW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXTX_BAL_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_BAL_SPAREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXTX_RCCAL_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTX_RCCAL_DIAGSELW<'a> {
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
    #[doc = "Bits 0:2 - rmap_rxtx_auxpll_bias_trim[2:0]"]
    #[inline]
    pub fn bias_trim(&self) -> BIAS_TRIMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BIAS_TRIMR { bits }
    }
    #[doc = "Bit 3 - rmap_rxtx_auxpll_diagsel1"]
    #[inline]
    pub fn diagsel1(&self) -> DIAGSEL1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIAGSEL1R { bits }
    }
    #[doc = "Bit 4 - rmap_rxtx_auxpll_diagsel2"]
    #[inline]
    pub fn diagsel2(&self) -> DIAGSEL2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIAGSEL2R { bits }
    }
    #[doc = "Bits 5:7 - rmap_rxtx_auxpll_lf_cntl[2:0]"]
    #[inline]
    pub fn lf_cntl(&self) -> LF_CNTLR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LF_CNTLR { bits }
    }
    #[doc = "Bits 8:11 - rmap_rxtx_auxpll_spare[3:0]"]
    #[inline]
    pub fn spare(&self) -> SPARER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPARER { bits }
    }
    #[doc = "Bits 12:15 - rmap_rxtx_auxpll_vco_dac_ref_adjust[3:0]"]
    #[inline]
    pub fn vco_dac_ref_adjust(&self) -> VCO_DAC_REF_ADJUSTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VCO_DAC_REF_ADJUSTR { bits }
    }
    #[doc = "Bit 16 - rmap_rxtx_auxpll_vtune_testmode"]
    #[inline]
    pub fn vtune_testmode(&self) -> VTUNE_TESTMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VTUNE_TESTMODER { bits }
    }
    #[doc = "Bits 20:21 - rmap_rxtx_bal_biast[1:0]"]
    #[inline]
    pub fn rxtx_bal_biast(&self) -> RXTX_BAL_BIASTR {
        RXTX_BAL_BIASTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - rmap_rxtx_bal_spare[2:0]"]
    #[inline]
    pub fn rxtx_bal_spare(&self) -> RXTX_BAL_SPARER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXTX_BAL_SPARER { bits }
    }
    #[doc = "Bit 28 - rmap_rxtx_rccal_diagsel"]
    #[inline]
    pub fn rxtx_rccal_diagsel(&self) -> RXTX_RCCAL_DIAGSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXTX_RCCAL_DIAGSELR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 36866 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - rmap_rxtx_auxpll_bias_trim[2:0]"]
    #[inline]
    pub fn bias_trim(&mut self) -> _BIAS_TRIMW {
        _BIAS_TRIMW { w: self }
    }
    #[doc = "Bit 3 - rmap_rxtx_auxpll_diagsel1"]
    #[inline]
    pub fn diagsel1(&mut self) -> _DIAGSEL1W {
        _DIAGSEL1W { w: self }
    }
    #[doc = "Bit 4 - rmap_rxtx_auxpll_diagsel2"]
    #[inline]
    pub fn diagsel2(&mut self) -> _DIAGSEL2W {
        _DIAGSEL2W { w: self }
    }
    #[doc = "Bits 5:7 - rmap_rxtx_auxpll_lf_cntl[2:0]"]
    #[inline]
    pub fn lf_cntl(&mut self) -> _LF_CNTLW {
        _LF_CNTLW { w: self }
    }
    #[doc = "Bits 8:11 - rmap_rxtx_auxpll_spare[3:0]"]
    #[inline]
    pub fn spare(&mut self) -> _SPAREW {
        _SPAREW { w: self }
    }
    #[doc = "Bits 12:15 - rmap_rxtx_auxpll_vco_dac_ref_adjust[3:0]"]
    #[inline]
    pub fn vco_dac_ref_adjust(&mut self) -> _VCO_DAC_REF_ADJUSTW {
        _VCO_DAC_REF_ADJUSTW { w: self }
    }
    #[doc = "Bit 16 - rmap_rxtx_auxpll_vtune_testmode"]
    #[inline]
    pub fn vtune_testmode(&mut self) -> _VTUNE_TESTMODEW {
        _VTUNE_TESTMODEW { w: self }
    }
    #[doc = "Bits 20:21 - rmap_rxtx_bal_biast[1:0]"]
    #[inline]
    pub fn rxtx_bal_biast(&mut self) -> _RXTX_BAL_BIASTW {
        _RXTX_BAL_BIASTW { w: self }
    }
    #[doc = "Bits 24:26 - rmap_rxtx_bal_spare[2:0]"]
    #[inline]
    pub fn rxtx_bal_spare(&mut self) -> _RXTX_BAL_SPAREW {
        _RXTX_BAL_SPAREW { w: self }
    }
    #[doc = "Bit 28 - rmap_rxtx_rccal_diagsel"]
    #[inline]
    pub fn rxtx_rccal_diagsel(&mut self) -> _RXTX_RCCAL_DIAGSELW {
        _RXTX_RCCAL_DIAGSELW { w: self }
    }
}
