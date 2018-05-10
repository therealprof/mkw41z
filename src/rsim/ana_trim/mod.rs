#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ANA_TRIM {
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
pub struct BB_LDO_LS_SPARER {
    bits: u8,
}
impl BB_LDO_LS_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_LS_TRIMR {
    bits: u8,
}
impl BB_LDO_LS_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_XO_SPARER {
    bits: u8,
}
impl BB_LDO_XO_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BB_LDO_XO_TRIMR {
    bits: u8,
}
impl BB_LDO_XO_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BB_XTAL_SPARER {
    bits: u8,
}
impl BB_XTAL_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BB_XTAL_TRIMR {
    bits: u8,
}
impl BB_XTAL_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BG_1V_TRIMR {
    bits: u8,
}
impl BG_1V_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BG_IBIAS_5U_TRIMR {
    bits: u8,
}
impl BG_IBIAS_5U_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_LS_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_LS_SPAREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_LS_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_LS_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_XO_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_XO_SPAREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_XO_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_XO_TRIMW<'a> {
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
pub struct _BB_XTAL_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_SPAREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_XTAL_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_TRIMW<'a> {
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
pub struct _BG_1V_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BG_1V_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BG_IBIAS_5U_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BG_IBIAS_5U_TRIMW<'a> {
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
    #[doc = "Bits 0:1 - rmap_bb_ldo_ls_spare_hv[1:0]"]
    #[inline]
    pub fn bb_ldo_ls_spare(&self) -> BB_LDO_LS_SPARER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_LDO_LS_SPARER { bits }
    }
    #[doc = "Bits 3:5 - rmap_bb_ldo_ls_trim_hv[2:0]"]
    #[inline]
    pub fn bb_ldo_ls_trim(&self) -> BB_LDO_LS_TRIMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_LDO_LS_TRIMR { bits }
    }
    #[doc = "Bits 6:7 - rmap_bb_ldo_xo_spare_hv[1:0]"]
    #[inline]
    pub fn bb_ldo_xo_spare(&self) -> BB_LDO_XO_SPARER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_LDO_XO_SPARER { bits }
    }
    #[doc = "Bits 8:10 - rmap_bb_ldo_xo_trim_hv[2:0]"]
    #[inline]
    pub fn bb_ldo_xo_trim(&self) -> BB_LDO_XO_TRIMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_LDO_XO_TRIMR { bits }
    }
    #[doc = "Bits 11:15 - rmap_bb_xtal_spare_hv[4:0]"]
    #[inline]
    pub fn bb_xtal_spare(&self) -> BB_XTAL_SPARER {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_XTAL_SPARER { bits }
    }
    #[doc = "Bits 16:23 - rmap_bb_xtal_trim_hv[7:0]"]
    #[inline]
    pub fn bb_xtal_trim(&self) -> BB_XTAL_TRIMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_XTAL_TRIMR { bits }
    }
    #[doc = "Bits 24:27 - rmap_bg_1v_trim_hv[3:0]"]
    #[inline]
    pub fn bg_1v_trim(&self) -> BG_1V_TRIMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BG_1V_TRIMR { bits }
    }
    #[doc = "Bits 28:31 - rmap_bg_ibias_5u_trim_hv[3:0]"]
    #[inline]
    pub fn bg_ibias_5u_trim(&self) -> BG_IBIAS_5U_TRIMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BG_IBIAS_5U_TRIMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2018181120 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - rmap_bb_ldo_ls_spare_hv[1:0]"]
    #[inline]
    pub fn bb_ldo_ls_spare(&mut self) -> _BB_LDO_LS_SPAREW {
        _BB_LDO_LS_SPAREW { w: self }
    }
    #[doc = "Bits 3:5 - rmap_bb_ldo_ls_trim_hv[2:0]"]
    #[inline]
    pub fn bb_ldo_ls_trim(&mut self) -> _BB_LDO_LS_TRIMW {
        _BB_LDO_LS_TRIMW { w: self }
    }
    #[doc = "Bits 6:7 - rmap_bb_ldo_xo_spare_hv[1:0]"]
    #[inline]
    pub fn bb_ldo_xo_spare(&mut self) -> _BB_LDO_XO_SPAREW {
        _BB_LDO_XO_SPAREW { w: self }
    }
    #[doc = "Bits 8:10 - rmap_bb_ldo_xo_trim_hv[2:0]"]
    #[inline]
    pub fn bb_ldo_xo_trim(&mut self) -> _BB_LDO_XO_TRIMW {
        _BB_LDO_XO_TRIMW { w: self }
    }
    #[doc = "Bits 11:15 - rmap_bb_xtal_spare_hv[4:0]"]
    #[inline]
    pub fn bb_xtal_spare(&mut self) -> _BB_XTAL_SPAREW {
        _BB_XTAL_SPAREW { w: self }
    }
    #[doc = "Bits 16:23 - rmap_bb_xtal_trim_hv[7:0]"]
    #[inline]
    pub fn bb_xtal_trim(&mut self) -> _BB_XTAL_TRIMW {
        _BB_XTAL_TRIMW { w: self }
    }
    #[doc = "Bits 24:27 - rmap_bg_1v_trim_hv[3:0]"]
    #[inline]
    pub fn bg_1v_trim(&mut self) -> _BG_1V_TRIMW {
        _BG_1V_TRIMW { w: self }
    }
    #[doc = "Bits 28:31 - rmap_bg_ibias_5u_trim_hv[3:0]"]
    #[inline]
    pub fn bg_ibias_5u_trim(&mut self) -> _BG_IBIAS_5U_TRIMW {
        _BG_IBIAS_5U_TRIMW { w: self }
    }
}
