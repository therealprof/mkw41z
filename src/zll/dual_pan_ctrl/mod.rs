#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DUAL_PAN_CTRL {
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
#[doc = "Possible values of the field `ACTIVE_NETWORK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVE_NETWORKR {
    #[doc = "Select PAN0"]
    _0,
    #[doc = "Select PAN1"]
    _1,
}
impl ACTIVE_NETWORKR {
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
            ACTIVE_NETWORKR::_0 => false,
            ACTIVE_NETWORKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACTIVE_NETWORKR {
        match value {
            false => ACTIVE_NETWORKR::_0,
            true => ACTIVE_NETWORKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ACTIVE_NETWORKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ACTIVE_NETWORKR::_1
    }
}
#[doc = r" Value of the field"]
pub struct DUAL_PAN_AUTOR {
    bits: bool,
}
impl DUAL_PAN_AUTOR {
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
pub struct PANCORDNTR1R {
    bits: bool,
}
impl PANCORDNTR1R {
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
#[doc = "Possible values of the field `CURRENT_NETWORK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURRENT_NETWORKR {
    #[doc = "PAN0 is selected"]
    _0,
    #[doc = "PAN1 is selected"]
    _1,
}
impl CURRENT_NETWORKR {
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
            CURRENT_NETWORKR::_0 => false,
            CURRENT_NETWORKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CURRENT_NETWORKR {
        match value {
            false => CURRENT_NETWORKR::_0,
            true => CURRENT_NETWORKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CURRENT_NETWORKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CURRENT_NETWORKR::_1
    }
}
#[doc = r" Value of the field"]
pub struct ZB_DP_CHAN_OVRD_ENR {
    bits: bool,
}
impl ZB_DP_CHAN_OVRD_ENR {
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
pub struct ZB_DP_CHAN_OVRD_SELR {
    bits: bool,
}
impl ZB_DP_CHAN_OVRD_SELR {
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
pub struct DUAL_PAN_DWELLR {
    bits: u8,
}
impl DUAL_PAN_DWELLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DUAL_PAN_REMAINR {
    bits: u8,
}
impl DUAL_PAN_REMAINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RECD_ON_PAN0R {
    bits: bool,
}
impl RECD_ON_PAN0R {
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
pub struct RECD_ON_PAN1R {
    bits: bool,
}
impl RECD_ON_PAN1R {
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
#[doc = "Values that can be written to the field `ACTIVE_NETWORK`"]
pub enum ACTIVE_NETWORKW {
    #[doc = "Select PAN0"]
    _0,
    #[doc = "Select PAN1"]
    _1,
}
impl ACTIVE_NETWORKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACTIVE_NETWORKW::_0 => false,
            ACTIVE_NETWORKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE_NETWORKW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE_NETWORKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACTIVE_NETWORKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select PAN0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACTIVE_NETWORKW::_0)
    }
    #[doc = "Select PAN1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACTIVE_NETWORKW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DUAL_PAN_AUTOW<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL_PAN_AUTOW<'a> {
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
pub struct _PANCORDNTR1W<'a> {
    w: &'a mut W,
}
impl<'a> _PANCORDNTR1W<'a> {
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
pub struct _ZB_DP_CHAN_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ZB_DP_CHAN_OVRD_ENW<'a> {
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
pub struct _ZB_DP_CHAN_OVRD_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _ZB_DP_CHAN_OVRD_SELW<'a> {
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
pub struct _DUAL_PAN_DWELLW<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL_PAN_DWELLW<'a> {
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
    #[doc = "Bit 0 - Active Network Selector"]
    #[inline]
    pub fn active_network(&self) -> ACTIVE_NETWORKR {
        ACTIVE_NETWORKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Activates automatic Dual PAN operating mode"]
    #[inline]
    pub fn dual_pan_auto(&self) -> DUAL_PAN_AUTOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DUAL_PAN_AUTOR { bits }
    }
    #[doc = "Bit 2 - Device is a PAN Coordinator on PAN1"]
    #[inline]
    pub fn pancordntr1(&self) -> PANCORDNTR1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PANCORDNTR1R { bits }
    }
    #[doc = "Bit 3 - Indicates which PAN is currently selected by hardware"]
    #[inline]
    pub fn current_network(&self) -> CURRENT_NETWORKR {
        CURRENT_NETWORKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Dual PAN Channel Override Enable"]
    #[inline]
    pub fn zb_dp_chan_ovrd_en(&self) -> ZB_DP_CHAN_OVRD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ZB_DP_CHAN_OVRD_ENR { bits }
    }
    #[doc = "Bit 5 - Dual PAN Channel Override Selector"]
    #[inline]
    pub fn zb_dp_chan_ovrd_sel(&self) -> ZB_DP_CHAN_OVRD_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ZB_DP_CHAN_OVRD_SELR { bits }
    }
    #[doc = "Bits 8:15 - Dual PAN Channel Frequency Dwell Time"]
    #[inline]
    pub fn dual_pan_dwell(&self) -> DUAL_PAN_DWELLR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DUAL_PAN_DWELLR { bits }
    }
    #[doc = "Bits 16:21 - Time Remaining before next PAN switch in auto Dual PAN mode"]
    #[inline]
    pub fn dual_pan_remain(&self) -> DUAL_PAN_REMAINR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DUAL_PAN_REMAINR { bits }
    }
    #[doc = "Bit 22 - Last Packet was Received on PAN0"]
    #[inline]
    pub fn recd_on_pan0(&self) -> RECD_ON_PAN0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RECD_ON_PAN0R { bits }
    }
    #[doc = "Bit 23 - Last Packet was Received on PAN1"]
    #[inline]
    pub fn recd_on_pan1(&self) -> RECD_ON_PAN1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RECD_ON_PAN1R { bits }
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
    #[doc = "Bit 0 - Active Network Selector"]
    #[inline]
    pub fn active_network(&mut self) -> _ACTIVE_NETWORKW {
        _ACTIVE_NETWORKW { w: self }
    }
    #[doc = "Bit 1 - Activates automatic Dual PAN operating mode"]
    #[inline]
    pub fn dual_pan_auto(&mut self) -> _DUAL_PAN_AUTOW {
        _DUAL_PAN_AUTOW { w: self }
    }
    #[doc = "Bit 2 - Device is a PAN Coordinator on PAN1"]
    #[inline]
    pub fn pancordntr1(&mut self) -> _PANCORDNTR1W {
        _PANCORDNTR1W { w: self }
    }
    #[doc = "Bit 4 - Dual PAN Channel Override Enable"]
    #[inline]
    pub fn zb_dp_chan_ovrd_en(&mut self) -> _ZB_DP_CHAN_OVRD_ENW {
        _ZB_DP_CHAN_OVRD_ENW { w: self }
    }
    #[doc = "Bit 5 - Dual PAN Channel Override Selector"]
    #[inline]
    pub fn zb_dp_chan_ovrd_sel(&mut self) -> _ZB_DP_CHAN_OVRD_SELW {
        _ZB_DP_CHAN_OVRD_SELW { w: self }
    }
    #[doc = "Bits 8:15 - Dual PAN Channel Frequency Dwell Time"]
    #[inline]
    pub fn dual_pan_dwell(&mut self) -> _DUAL_PAN_DWELLW {
        _DUAL_PAN_DWELLW { w: self }
    }
}
