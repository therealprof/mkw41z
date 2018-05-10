#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEQ_CTRL_STS {
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
pub struct CLR_NEW_SEQ_INHIBITR {
    bits: bool,
}
impl CLR_NEW_SEQ_INHIBITR {
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
pub struct EVENT_TMR_DO_NOT_LATCHR {
    bits: bool,
}
impl EVENT_TMR_DO_NOT_LATCHR {
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
#[doc = "Possible values of the field `LATCH_PREAMBLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATCH_PREAMBLER {
    #[doc = "Don't make PREAMBLE_DET and SFD_DET bits of PHY_STS (SEQ_STATE) Register \"sticky\", i.e, these status bits reflect the realtime, dynamic state of preamble_detect and sfd_detect"]
    _0,
    #[doc = "Make PREAMBLE_DET and SFD_DET bits of PHY_STS (SEQ_STATE) Register \"sticky\", i.e.,occurrences of preamble and SFD detection are latched and held until the start of the next autosequence"]
    _1,
}
impl LATCH_PREAMBLER {
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
            LATCH_PREAMBLER::_0 => false,
            LATCH_PREAMBLER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LATCH_PREAMBLER {
        match value {
            false => LATCH_PREAMBLER::_0,
            true => LATCH_PREAMBLER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LATCH_PREAMBLER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LATCH_PREAMBLER::_1
    }
}
#[doc = r" Value of the field"]
pub struct NO_RX_RECYCLER {
    bits: bool,
}
impl NO_RX_RECYCLER {
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
#[doc = "Possible values of the field `FORCE_CRC_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_CRC_ERRORR {
    #[doc = "normal operation"]
    _0,
    #[doc = "Force the next transmitted packet to have a CRC error"]
    _1,
}
impl FORCE_CRC_ERRORR {
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
            FORCE_CRC_ERRORR::_0 => false,
            FORCE_CRC_ERRORR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCE_CRC_ERRORR {
        match value {
            false => FORCE_CRC_ERRORR::_0,
            true => FORCE_CRC_ERRORR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FORCE_CRC_ERRORR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FORCE_CRC_ERRORR::_1
    }
}
#[doc = "Possible values of the field `CONTINUOUS_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTINUOUS_ENR {
    #[doc = "normal operation"]
    _0,
    #[doc = "Continuous TX or RX mode is enabled (depending on XCVSEQ setting)."]
    _1,
}
impl CONTINUOUS_ENR {
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
            CONTINUOUS_ENR::_0 => false,
            CONTINUOUS_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONTINUOUS_ENR {
        match value {
            false => CONTINUOUS_ENR::_0,
            true => CONTINUOUS_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CONTINUOUS_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CONTINUOUS_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct XCVSEQ_ACTUALR {
    bits: u8,
}
impl XCVSEQ_ACTUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SEQ_IDLER {
    bits: bool,
}
impl SEQ_IDLER {
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
pub struct NEW_SEQ_INHIBITR {
    bits: bool,
}
impl NEW_SEQ_INHIBITR {
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
pub struct RX_TIMEOUT_PENDINGR {
    bits: bool,
}
impl RX_TIMEOUT_PENDINGR {
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
pub struct RX_MODER {
    bits: bool,
}
impl RX_MODER {
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
pub struct TMR2_SEQ_TRIG_ARMEDR {
    bits: bool,
}
impl TMR2_SEQ_TRIG_ARMEDR {
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
pub struct SEQ_T_STATUSR {
    bits: u8,
}
impl SEQ_T_STATUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SW_ABORTEDR {
    bits: bool,
}
impl SW_ABORTEDR {
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
pub struct TC3_ABORTEDR {
    bits: bool,
}
impl TC3_ABORTEDR {
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
pub struct PLL_ABORTEDR {
    bits: bool,
}
impl PLL_ABORTEDR {
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
pub struct _CLR_NEW_SEQ_INHIBITW<'a> {
    w: &'a mut W,
}
impl<'a> _CLR_NEW_SEQ_INHIBITW<'a> {
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
pub struct _EVENT_TMR_DO_NOT_LATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENT_TMR_DO_NOT_LATCHW<'a> {
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
#[doc = "Values that can be written to the field `LATCH_PREAMBLE`"]
pub enum LATCH_PREAMBLEW {
    #[doc = "Don't make PREAMBLE_DET and SFD_DET bits of PHY_STS (SEQ_STATE) Register \"sticky\", i.e, these status bits reflect the realtime, dynamic state of preamble_detect and sfd_detect"]
    _0,
    #[doc = "Make PREAMBLE_DET and SFD_DET bits of PHY_STS (SEQ_STATE) Register \"sticky\", i.e.,occurrences of preamble and SFD detection are latched and held until the start of the next autosequence"]
    _1,
}
impl LATCH_PREAMBLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LATCH_PREAMBLEW::_0 => false,
            LATCH_PREAMBLEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LATCH_PREAMBLEW<'a> {
    w: &'a mut W,
}
impl<'a> _LATCH_PREAMBLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LATCH_PREAMBLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Don't make PREAMBLE_DET and SFD_DET bits of PHY_STS (SEQ_STATE) Register \"sticky\", i.e, these status bits reflect the realtime, dynamic state of preamble_detect and sfd_detect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LATCH_PREAMBLEW::_0)
    }
    #[doc = "Make PREAMBLE_DET and SFD_DET bits of PHY_STS (SEQ_STATE) Register \"sticky\", i.e.,occurrences of preamble and SFD detection are latched and held until the start of the next autosequence"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LATCH_PREAMBLEW::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NO_RX_RECYCLEW<'a> {
    w: &'a mut W,
}
impl<'a> _NO_RX_RECYCLEW<'a> {
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
#[doc = "Values that can be written to the field `FORCE_CRC_ERROR`"]
pub enum FORCE_CRC_ERRORW {
    #[doc = "normal operation"]
    _0,
    #[doc = "Force the next transmitted packet to have a CRC error"]
    _1,
}
impl FORCE_CRC_ERRORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FORCE_CRC_ERRORW::_0 => false,
            FORCE_CRC_ERRORW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORCE_CRC_ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCE_CRC_ERRORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCE_CRC_ERRORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "normal operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FORCE_CRC_ERRORW::_0)
    }
    #[doc = "Force the next transmitted packet to have a CRC error"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FORCE_CRC_ERRORW::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CONTINUOUS_EN`"]
pub enum CONTINUOUS_ENW {
    #[doc = "normal operation"]
    _0,
    #[doc = "Continuous TX or RX mode is enabled (depending on XCVSEQ setting)."]
    _1,
}
impl CONTINUOUS_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONTINUOUS_ENW::_0 => false,
            CONTINUOUS_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONTINUOUS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTINUOUS_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONTINUOUS_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "normal operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONTINUOUS_ENW::_0)
    }
    #[doc = "Continuous TX or RX mode is enabled (depending on XCVSEQ setting)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONTINUOUS_ENW::_1)
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 2 - Overrides the automatic hardware locking of the programmed XCVSEQ while an autosequence is underway"]
    #[inline]
    pub fn clr_new_seq_inhibit(&self) -> CLR_NEW_SEQ_INHIBITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLR_NEW_SEQ_INHIBITR { bits }
    }
    #[doc = "Bit 3 - Overrides the automatic hardware latching of the Event Timer"]
    #[inline]
    pub fn event_tmr_do_not_latch(&self) -> EVENT_TMR_DO_NOT_LATCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EVENT_TMR_DO_NOT_LATCHR { bits }
    }
    #[doc = "Bit 4 - Stickiness Control for Preamble Detection"]
    #[inline]
    pub fn latch_preamble(&self) -> LATCH_PREAMBLER {
        LATCH_PREAMBLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Disable Automatic RX Sequence Recycling"]
    #[inline]
    pub fn no_rx_recycle(&self) -> NO_RX_RECYCLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NO_RX_RECYCLER { bits }
    }
    #[doc = "Bit 6 - Induce a CRC Error in Transmitted Packets"]
    #[inline]
    pub fn force_crc_error(&self) -> FORCE_CRC_ERRORR {
        FORCE_CRC_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable Continuous TX or RX Mode"]
    #[inline]
    pub fn continuous_en(&self) -> CONTINUOUS_ENR {
        CONTINUOUS_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Indicates the programmed sequence that has been recognized by the ZSM Sequence Manager"]
    #[inline]
    pub fn xcvseq_actual(&self) -> XCVSEQ_ACTUALR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XCVSEQ_ACTUALR { bits }
    }
    #[doc = "Bit 11 - ZSM Sequence Idle Indicator"]
    #[inline]
    pub fn seq_idle(&self) -> SEQ_IDLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEQ_IDLER { bits }
    }
    #[doc = "Bit 12 - New Sequence Inhibit"]
    #[inline]
    pub fn new_seq_inhibit(&self) -> NEW_SEQ_INHIBITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NEW_SEQ_INHIBITR { bits }
    }
    #[doc = "Bit 13 - Indicates a TMR3 RX Timeout is Pending"]
    #[inline]
    pub fn rx_timeout_pending(&self) -> RX_TIMEOUT_PENDINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_TIMEOUT_PENDINGR { bits }
    }
    #[doc = "Bit 14 - RX Operation in Progress"]
    #[inline]
    pub fn rx_mode(&self) -> RX_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_MODER { bits }
    }
    #[doc = "Bit 15 - indicates that TMR2 has been programmed and is armed to trigger a new autosequence"]
    #[inline]
    pub fn tmr2_seq_trig_armed(&self) -> TMR2_SEQ_TRIG_ARMEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TMR2_SEQ_TRIG_ARMEDR { bits }
    }
    #[doc = "Bits 16:21 - Status of the just-completed or ongoing Sequence T or Sequence TR"]
    #[inline]
    pub fn seq_t_status(&self) -> SEQ_T_STATUSR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEQ_T_STATUSR { bits }
    }
    #[doc = "Bit 24 - Autosequence has terminated due to a Software abort."]
    #[inline]
    pub fn sw_aborted(&self) -> SW_ABORTEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SW_ABORTEDR { bits }
    }
    #[doc = "Bit 25 - autosequence has terminated due to an TMR3 timeout"]
    #[inline]
    pub fn tc3_aborted(&self) -> TC3_ABORTEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TC3_ABORTEDR { bits }
    }
    #[doc = "Bit 26 - Autosequence has terminated due to an PLL unlock event"]
    #[inline]
    pub fn pll_aborted(&self) -> PLL_ABORTEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_ABORTEDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Overrides the automatic hardware locking of the programmed XCVSEQ while an autosequence is underway"]
    #[inline]
    pub fn clr_new_seq_inhibit(&mut self) -> _CLR_NEW_SEQ_INHIBITW {
        _CLR_NEW_SEQ_INHIBITW { w: self }
    }
    #[doc = "Bit 3 - Overrides the automatic hardware latching of the Event Timer"]
    #[inline]
    pub fn event_tmr_do_not_latch(&mut self) -> _EVENT_TMR_DO_NOT_LATCHW {
        _EVENT_TMR_DO_NOT_LATCHW { w: self }
    }
    #[doc = "Bit 4 - Stickiness Control for Preamble Detection"]
    #[inline]
    pub fn latch_preamble(&mut self) -> _LATCH_PREAMBLEW {
        _LATCH_PREAMBLEW { w: self }
    }
    #[doc = "Bit 5 - Disable Automatic RX Sequence Recycling"]
    #[inline]
    pub fn no_rx_recycle(&mut self) -> _NO_RX_RECYCLEW {
        _NO_RX_RECYCLEW { w: self }
    }
    #[doc = "Bit 6 - Induce a CRC Error in Transmitted Packets"]
    #[inline]
    pub fn force_crc_error(&mut self) -> _FORCE_CRC_ERRORW {
        _FORCE_CRC_ERRORW { w: self }
    }
    #[doc = "Bit 7 - Enable Continuous TX or RX Mode"]
    #[inline]
    pub fn continuous_en(&mut self) -> _CONTINUOUS_ENW {
        _CONTINUOUS_ENW { w: self }
    }
}
