#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RF_OSC_CTRL {
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
pub struct BB_XTAL_ALC_COUNT_SELR {
    bits: u8,
}
impl BB_XTAL_ALC_COUNT_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BB_XTAL_ALC_ONR {
    bits: bool,
}
impl BB_XTAL_ALC_ONR {
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
pub struct RF_OSC_BYPASS_ENR {
    bits: bool,
}
impl RF_OSC_BYPASS_ENR {
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
pub struct BB_XTAL_COMP_BIASR {
    bits: u8,
}
impl BB_XTAL_COMP_BIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BB_XTAL_DC_COUP_MODE_ENR {
    bits: bool,
}
impl BB_XTAL_DC_COUP_MODE_ENR {
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
pub struct BB_XTAL_DIAGSELR {
    bits: bool,
}
impl BB_XTAL_DIAGSELR {
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
pub struct BB_XTAL_DIG_CLK_ONR {
    bits: bool,
}
impl BB_XTAL_DIG_CLK_ONR {
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
pub struct BB_XTAL_GMR {
    bits: u8,
}
impl BB_XTAL_GMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BB_XTAL_ON_OVRDR {
    bits: bool,
}
impl BB_XTAL_ON_OVRDR {
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
pub struct BB_XTAL_ON_OVRD_ONR {
    bits: bool,
}
impl BB_XTAL_ON_OVRD_ONR {
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
pub struct BB_XTAL_READY_COUNT_SELR {
    bits: u8,
}
impl BB_XTAL_READY_COUNT_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RADIO_EXT_OSC_RF_EN_SELR {
    bits: bool,
}
impl RADIO_EXT_OSC_RF_EN_SELR {
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
pub struct RADIO_EXT_OSC_OVRDR {
    bits: bool,
}
impl RADIO_EXT_OSC_OVRDR {
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
pub struct RADIO_EXT_OSC_OVRD_ENR {
    bits: bool,
}
impl RADIO_EXT_OSC_OVRD_ENR {
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
pub struct RADIO_RF_ABORT_OVRDR {
    bits: bool,
}
impl RADIO_RF_ABORT_OVRDR {
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
pub struct RADIO_RF_ABORT_OVRD_ENR {
    bits: bool,
}
impl RADIO_RF_ABORT_OVRD_ENR {
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
pub struct _BB_XTAL_ALC_COUNT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_ALC_COUNT_SELW<'a> {
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
pub struct _BB_XTAL_ALC_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_ALC_ONW<'a> {
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
pub struct _RF_OSC_BYPASS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RF_OSC_BYPASS_ENW<'a> {
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
pub struct _BB_XTAL_COMP_BIASW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_COMP_BIASW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_XTAL_DC_COUP_MODE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_DC_COUP_MODE_ENW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_XTAL_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_DIAGSELW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_XTAL_DIG_CLK_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_DIG_CLK_ONW<'a> {
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
pub struct _BB_XTAL_GMW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_GMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_XTAL_ON_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_ON_OVRDW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_XTAL_ON_OVRD_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_ON_OVRD_ONW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BB_XTAL_READY_COUNT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_XTAL_READY_COUNT_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RADIO_EXT_OSC_RF_EN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _RADIO_EXT_OSC_RF_EN_SELW<'a> {
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
pub struct _RADIO_EXT_OSC_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RADIO_EXT_OSC_OVRDW<'a> {
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
#[doc = r" Proxy"]
pub struct _RADIO_EXT_OSC_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RADIO_EXT_OSC_OVRD_ENW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RADIO_RF_ABORT_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RADIO_RF_ABORT_OVRDW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RADIO_RF_ABORT_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RADIO_RF_ABORT_OVRD_ENW<'a> {
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
    #[doc = "Bits 0:1 - rmap_bb_xtal_alc_count_sel_hv[1:0]"]
    #[inline]
    pub fn bb_xtal_alc_count_sel(&self) -> BB_XTAL_ALC_COUNT_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_XTAL_ALC_COUNT_SELR { bits }
    }
    #[doc = "Bit 2 - rmap_bb_xtal_alc_on_hv"]
    #[inline]
    pub fn bb_xtal_alc_on(&self) -> BB_XTAL_ALC_ONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_XTAL_ALC_ONR { bits }
    }
    #[doc = "Bit 3 - RF Ref Osc Bypass Enable"]
    #[inline]
    pub fn rf_osc_bypass_en(&self) -> RF_OSC_BYPASS_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RF_OSC_BYPASS_ENR { bits }
    }
    #[doc = "Bits 4:8 - rmap_bb_xtal_comp_bias_hv[4:0]"]
    #[inline]
    pub fn bb_xtal_comp_bias(&self) -> BB_XTAL_COMP_BIASR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_XTAL_COMP_BIASR { bits }
    }
    #[doc = "Bit 9 - rmap_bb_xtal_dc_coup_mode_en_hv"]
    #[inline]
    pub fn bb_xtal_dc_coup_mode_en(&self) -> BB_XTAL_DC_COUP_MODE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_XTAL_DC_COUP_MODE_ENR { bits }
    }
    #[doc = "Bit 10 - rmap_bb_xtal_diagsel_hv"]
    #[inline]
    pub fn bb_xtal_diagsel(&self) -> BB_XTAL_DIAGSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_XTAL_DIAGSELR { bits }
    }
    #[doc = "Bit 11 - rmap_bb_xtal_dig_clk_on_hv"]
    #[inline]
    pub fn bb_xtal_dig_clk_on(&self) -> BB_XTAL_DIG_CLK_ONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_XTAL_DIG_CLK_ONR { bits }
    }
    #[doc = "Bits 12:16 - rmap_bb_xtal_gm_hv[4:0]"]
    #[inline]
    pub fn bb_xtal_gm(&self) -> BB_XTAL_GMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_XTAL_GMR { bits }
    }
    #[doc = "Bit 17 - rmap_bb_xtal_on_ovrd_hv"]
    #[inline]
    pub fn bb_xtal_on_ovrd(&self) -> BB_XTAL_ON_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_XTAL_ON_OVRDR { bits }
    }
    #[doc = "Bit 18 - rmap_bb_xtal_on_ovrd_on_hv"]
    #[inline]
    pub fn bb_xtal_on_ovrd_on(&self) -> BB_XTAL_ON_OVRD_ONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BB_XTAL_ON_OVRD_ONR { bits }
    }
    #[doc = "Bits 20:21 - rmap_bb_xtal_ready_count_sel_hv[1:0]"]
    #[inline]
    pub fn bb_xtal_ready_count_sel(&self) -> BB_XTAL_READY_COUNT_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BB_XTAL_READY_COUNT_SELR { bits }
    }
    #[doc = "Bit 27 - Radio External Request for RF OSC Select"]
    #[inline]
    pub fn radio_ext_osc_rf_en_sel(&self) -> RADIO_EXT_OSC_RF_EN_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RADIO_EXT_OSC_RF_EN_SELR { bits }
    }
    #[doc = "Bit 28 - Radio External Request for RF OSC Override"]
    #[inline]
    pub fn radio_ext_osc_ovrd(&self) -> RADIO_EXT_OSC_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RADIO_EXT_OSC_OVRDR { bits }
    }
    #[doc = "Bit 29 - Radio External Request for RF OSC Override Enable"]
    #[inline]
    pub fn radio_ext_osc_ovrd_en(&self) -> RADIO_EXT_OSC_OVRD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RADIO_EXT_OSC_OVRD_ENR { bits }
    }
    #[doc = "Bit 30 - Radio RF Abort Override"]
    #[inline]
    pub fn radio_rf_abort_ovrd(&self) -> RADIO_RF_ABORT_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RADIO_RF_ABORT_OVRDR { bits }
    }
    #[doc = "Bit 31 - Radio RF Abort Override Enable"]
    #[inline]
    pub fn radio_rf_abort_ovrd_en(&self) -> RADIO_RF_ABORT_OVRD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RADIO_RF_ABORT_OVRD_ENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2111494 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - rmap_bb_xtal_alc_count_sel_hv[1:0]"]
    #[inline]
    pub fn bb_xtal_alc_count_sel(&mut self) -> _BB_XTAL_ALC_COUNT_SELW {
        _BB_XTAL_ALC_COUNT_SELW { w: self }
    }
    #[doc = "Bit 2 - rmap_bb_xtal_alc_on_hv"]
    #[inline]
    pub fn bb_xtal_alc_on(&mut self) -> _BB_XTAL_ALC_ONW {
        _BB_XTAL_ALC_ONW { w: self }
    }
    #[doc = "Bit 3 - RF Ref Osc Bypass Enable"]
    #[inline]
    pub fn rf_osc_bypass_en(&mut self) -> _RF_OSC_BYPASS_ENW {
        _RF_OSC_BYPASS_ENW { w: self }
    }
    #[doc = "Bits 4:8 - rmap_bb_xtal_comp_bias_hv[4:0]"]
    #[inline]
    pub fn bb_xtal_comp_bias(&mut self) -> _BB_XTAL_COMP_BIASW {
        _BB_XTAL_COMP_BIASW { w: self }
    }
    #[doc = "Bit 9 - rmap_bb_xtal_dc_coup_mode_en_hv"]
    #[inline]
    pub fn bb_xtal_dc_coup_mode_en(&mut self) -> _BB_XTAL_DC_COUP_MODE_ENW {
        _BB_XTAL_DC_COUP_MODE_ENW { w: self }
    }
    #[doc = "Bit 10 - rmap_bb_xtal_diagsel_hv"]
    #[inline]
    pub fn bb_xtal_diagsel(&mut self) -> _BB_XTAL_DIAGSELW {
        _BB_XTAL_DIAGSELW { w: self }
    }
    #[doc = "Bit 11 - rmap_bb_xtal_dig_clk_on_hv"]
    #[inline]
    pub fn bb_xtal_dig_clk_on(&mut self) -> _BB_XTAL_DIG_CLK_ONW {
        _BB_XTAL_DIG_CLK_ONW { w: self }
    }
    #[doc = "Bits 12:16 - rmap_bb_xtal_gm_hv[4:0]"]
    #[inline]
    pub fn bb_xtal_gm(&mut self) -> _BB_XTAL_GMW {
        _BB_XTAL_GMW { w: self }
    }
    #[doc = "Bit 17 - rmap_bb_xtal_on_ovrd_hv"]
    #[inline]
    pub fn bb_xtal_on_ovrd(&mut self) -> _BB_XTAL_ON_OVRDW {
        _BB_XTAL_ON_OVRDW { w: self }
    }
    #[doc = "Bit 18 - rmap_bb_xtal_on_ovrd_on_hv"]
    #[inline]
    pub fn bb_xtal_on_ovrd_on(&mut self) -> _BB_XTAL_ON_OVRD_ONW {
        _BB_XTAL_ON_OVRD_ONW { w: self }
    }
    #[doc = "Bits 20:21 - rmap_bb_xtal_ready_count_sel_hv[1:0]"]
    #[inline]
    pub fn bb_xtal_ready_count_sel(&mut self) -> _BB_XTAL_READY_COUNT_SELW {
        _BB_XTAL_READY_COUNT_SELW { w: self }
    }
    #[doc = "Bit 27 - Radio External Request for RF OSC Select"]
    #[inline]
    pub fn radio_ext_osc_rf_en_sel(&mut self) -> _RADIO_EXT_OSC_RF_EN_SELW {
        _RADIO_EXT_OSC_RF_EN_SELW { w: self }
    }
    #[doc = "Bit 28 - Radio External Request for RF OSC Override"]
    #[inline]
    pub fn radio_ext_osc_ovrd(&mut self) -> _RADIO_EXT_OSC_OVRDW {
        _RADIO_EXT_OSC_OVRDW { w: self }
    }
    #[doc = "Bit 29 - Radio External Request for RF OSC Override Enable"]
    #[inline]
    pub fn radio_ext_osc_ovrd_en(&mut self) -> _RADIO_EXT_OSC_OVRD_ENW {
        _RADIO_EXT_OSC_OVRD_ENW { w: self }
    }
    #[doc = "Bit 30 - Radio RF Abort Override"]
    #[inline]
    pub fn radio_rf_abort_ovrd(&mut self) -> _RADIO_RF_ABORT_OVRDW {
        _RADIO_RF_ABORT_OVRDW { w: self }
    }
    #[doc = "Bit 31 - Radio RF Abort Override Enable"]
    #[inline]
    pub fn radio_rf_abort_ovrd_en(&mut self) -> _RADIO_RF_ABORT_OVRD_ENW {
        _RADIO_RF_ABORT_OVRD_ENW { w: self }
    }
}
