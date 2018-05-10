#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ANA_TEST {
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
pub struct BB_LDO_LS_BYPR {
    bits: bool,
}
impl BB_LDO_LS_BYPR {
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
pub struct BB_LDO_LS_DIAGSELR {
    bits: bool,
}
impl BB_LDO_LS_DIAGSELR {
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
pub struct BB_LDO_XO_BYP_ONR {
    bits: bool,
}
impl BB_LDO_XO_BYP_ONR {
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
pub struct BB_LDO_XO_DIAGSELR {
    bits: bool,
}
impl BB_LDO_XO_DIAGSELR {
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
pub struct BB_XTAL_TESTR {
    bits: bool,
}
impl BB_XTAL_TESTR {
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
pub struct BG_DIAGBUFR {
    bits: bool,
}
impl BG_DIAGBUFR {
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
pub struct BG_DIAGSELR {
    bits: bool,
}
impl BG_DIAGSELR {
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
pub struct BG_STARTUPFORCER {
    bits: bool,
}
impl BG_STARTUPFORCER {
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
pub struct DIAG_1234_ONR {
    bits: bool,
}
impl DIAG_1234_ONR {
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
pub struct DIAG2SOCADC_DECR {
    bits: u8,
}
impl DIAG2SOCADC_DECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIAG2SOCADC_DEC_ONR {
    bits: bool,
}
impl DIAG2SOCADC_DEC_ONR {
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
pub struct DIAGCODER {
    bits: u8,
}
impl DIAGCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_LS_BYPW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_LS_BYPW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_LS_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_LS_DIAGSELW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_XO_BYP_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_XO_BYP_ONW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_LDO_XO_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_LDO_XO_DIAGSELW<'a> {
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
pub struct _BB_XTAL_TESTW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_TESTW<'a> {
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
pub struct _BG_DIAGBUFW<'a> {
    w: &'a mut W,
}
impl<'a> _BG_DIAGBUFW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BG_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BG_DIAGSELW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BG_STARTUPFORCEW<'a> {
    w: &'a mut W,
}
impl<'a> _BG_STARTUPFORCEW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIAG_1234_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _DIAG_1234_ONW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIAG2SOCADC_DECW<'a> {
    w: &'a mut W,
}
impl<'a> _DIAG2SOCADC_DECW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIAG2SOCADC_DEC_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _DIAG2SOCADC_DEC_ONW<'a> {
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
#[doc = r" Proxy"]
pub struct _DIAGCODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DIAGCODEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - rmap_bb_ldo_ls_byp_hv"]
    #[inline]
    pub fn bb_ldo_ls_byp(&self) -> BB_LDO_LS_BYPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_LS_BYPR { bits }
    }
    #[doc = "Bit 1 - rmap_bb_ldo_ls_diagsel_hv"]
    #[inline]
    pub fn bb_ldo_ls_diagsel(&self) -> BB_LDO_LS_DIAGSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_LS_DIAGSELR { bits }
    }
    #[doc = "Bit 2 - rmap_bb_ldo_xo_byp_on_hv"]
    #[inline]
    pub fn bb_ldo_xo_byp_on(&self) -> BB_LDO_XO_BYP_ONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_XO_BYP_ONR { bits }
    }
    #[doc = "Bit 3 - rmap_bb_ldo_xo_diagsel_hv"]
    #[inline]
    pub fn bb_ldo_xo_diagsel(&self) -> BB_LDO_XO_DIAGSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_LDO_XO_DIAGSELR { bits }
    }
    #[doc = "Bit 4 - rmap_bb_xtal_test_en_hv"]
    #[inline]
    pub fn bb_xtal_test(&self) -> BB_XTAL_TESTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_XTAL_TESTR { bits }
    }
    #[doc = "Bit 5 - rmap_bg_diagbuf_hv"]
    #[inline]
    pub fn bg_diagbuf(&self) -> BG_DIAGBUFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BG_DIAGBUFR { bits }
    }
    #[doc = "Bit 6 - rmap_bg_diagsel_hv"]
    #[inline]
    pub fn bg_diagsel(&self) -> BG_DIAGSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BG_DIAGSELR { bits }
    }
    #[doc = "Bit 7 - rmap_bg_startupforce_hv"]
    #[inline]
    pub fn bg_startupforce(&self) -> BG_STARTUPFORCER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BG_STARTUPFORCER { bits }
    }
    #[doc = "Bit 8 - rmap_diag_1234_on_hv"]
    #[inline]
    pub fn diag_1234_on(&self) -> DIAG_1234_ONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIAG_1234_ONR { bits }
    }
    #[doc = "Bits 9:10 - rmap_diag2socadc_dec_hv[1:0]"]
    #[inline]
    pub fn diag2socadc_dec(&self) -> DIAG2SOCADC_DECR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIAG2SOCADC_DECR { bits }
    }
    #[doc = "Bit 11 - rmap_diag2socadc_dec_on_hv"]
    #[inline]
    pub fn diag2socadc_dec_on(&self) -> DIAG2SOCADC_DEC_ONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIAG2SOCADC_DEC_ONR { bits }
    }
    #[doc = "Bits 12:14 - rmap_diagcode_hv[2:0]"]
    #[inline]
    pub fn diagcode(&self) -> DIAGCODER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIAGCODER { bits }
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
    #[doc = "Bit 0 - rmap_bb_ldo_ls_byp_hv"]
    #[inline]
    pub fn bb_ldo_ls_byp(&mut self) -> _BB_LDO_LS_BYPW {
        _BB_LDO_LS_BYPW { w: self }
    }
    #[doc = "Bit 1 - rmap_bb_ldo_ls_diagsel_hv"]
    #[inline]
    pub fn bb_ldo_ls_diagsel(&mut self) -> _BB_LDO_LS_DIAGSELW {
        _BB_LDO_LS_DIAGSELW { w: self }
    }
    #[doc = "Bit 2 - rmap_bb_ldo_xo_byp_on_hv"]
    #[inline]
    pub fn bb_ldo_xo_byp_on(&mut self) -> _BB_LDO_XO_BYP_ONW {
        _BB_LDO_XO_BYP_ONW { w: self }
    }
    #[doc = "Bit 3 - rmap_bb_ldo_xo_diagsel_hv"]
    #[inline]
    pub fn bb_ldo_xo_diagsel(&mut self) -> _BB_LDO_XO_DIAGSELW {
        _BB_LDO_XO_DIAGSELW { w: self }
    }
    #[doc = "Bit 4 - rmap_bb_xtal_test_en_hv"]
    #[inline]
    pub fn bb_xtal_test(&mut self) -> _BB_XTAL_TESTW {
        _BB_XTAL_TESTW { w: self }
    }
    #[doc = "Bit 5 - rmap_bg_diagbuf_hv"]
    #[inline]
    pub fn bg_diagbuf(&mut self) -> _BG_DIAGBUFW {
        _BG_DIAGBUFW { w: self }
    }
    #[doc = "Bit 6 - rmap_bg_diagsel_hv"]
    #[inline]
    pub fn bg_diagsel(&mut self) -> _BG_DIAGSELW {
        _BG_DIAGSELW { w: self }
    }
    #[doc = "Bit 7 - rmap_bg_startupforce_hv"]
    #[inline]
    pub fn bg_startupforce(&mut self) -> _BG_STARTUPFORCEW {
        _BG_STARTUPFORCEW { w: self }
    }
    #[doc = "Bit 8 - rmap_diag_1234_on_hv"]
    #[inline]
    pub fn diag_1234_on(&mut self) -> _DIAG_1234_ONW {
        _DIAG_1234_ONW { w: self }
    }
    #[doc = "Bits 9:10 - rmap_diag2socadc_dec_hv[1:0]"]
    #[inline]
    pub fn diag2socadc_dec(&mut self) -> _DIAG2SOCADC_DECW {
        _DIAG2SOCADC_DECW { w: self }
    }
    #[doc = "Bit 11 - rmap_diag2socadc_dec_on_hv"]
    #[inline]
    pub fn diag2socadc_dec_on(&mut self) -> _DIAG2SOCADC_DEC_ONW {
        _DIAG2SOCADC_DEC_ONW { w: self }
    }
    #[doc = "Bits 12:14 - rmap_diagcode_hv[2:0]"]
    #[inline]
    pub fn diagcode(&mut self) -> _DIAGCODEW {
        _DIAGCODEW { w: self }
    }
}
