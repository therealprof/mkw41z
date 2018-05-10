#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SAM_TABLE {
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
pub struct SAM_INDEXR {
    bits: u8,
}
impl SAM_INDEXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAM_CHECKSUMR {
    bits: u16,
}
impl SAM_CHECKSUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ACK_FRM_PNDR {
    bits: bool,
}
impl ACK_FRM_PNDR {
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
#[doc = "Possible values of the field `ACK_FRM_PND_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACK_FRM_PND_CTRLR {
    #[doc = "the FramePending field of the Frame Control Field of the next automatic TX acknowledge packet is determined by hardware"]
    _0,
    #[doc = "the FramePending field of the Frame Control Field of the next automatic TX acknowledge packet tracks ACK_FRM_PEND"]
    _1,
}
impl ACK_FRM_PND_CTRLR {
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
            ACK_FRM_PND_CTRLR::_0 => false,
            ACK_FRM_PND_CTRLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACK_FRM_PND_CTRLR {
        match value {
            false => ACK_FRM_PND_CTRLR::_0,
            true => ACK_FRM_PND_CTRLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ACK_FRM_PND_CTRLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ACK_FRM_PND_CTRLR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SAM_BUSYR {
    bits: bool,
}
impl SAM_BUSYR {
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
pub struct _SAM_INDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _SAM_INDEXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAM_INDEX_WRW<'a> {
    w: &'a mut W,
}
impl<'a> _SAM_INDEX_WRW<'a> {
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
pub struct _SAM_CHECKSUMW<'a> {
    w: &'a mut W,
}
impl<'a> _SAM_CHECKSUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAM_INDEX_INVW<'a> {
    w: &'a mut W,
}
impl<'a> _SAM_INDEX_INVW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAM_INDEX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SAM_INDEX_ENW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACK_FRM_PNDW<'a> {
    w: &'a mut W,
}
impl<'a> _ACK_FRM_PNDW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACK_FRM_PND_CTRL`"]
pub enum ACK_FRM_PND_CTRLW {
    #[doc = "the FramePending field of the Frame Control Field of the next automatic TX acknowledge packet is determined by hardware"]
    _0,
    #[doc = "the FramePending field of the Frame Control Field of the next automatic TX acknowledge packet tracks ACK_FRM_PEND"]
    _1,
}
impl ACK_FRM_PND_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACK_FRM_PND_CTRLW::_0 => false,
            ACK_FRM_PND_CTRLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACK_FRM_PND_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _ACK_FRM_PND_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACK_FRM_PND_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the FramePending field of the Frame Control Field of the next automatic TX acknowledge packet is determined by hardware"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACK_FRM_PND_CTRLW::_0)
    }
    #[doc = "the FramePending field of the Frame Control Field of the next automatic TX acknowledge packet tracks ACK_FRM_PEND"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACK_FRM_PND_CTRLW::_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FIND_FREE_IDXW<'a> {
    w: &'a mut W,
}
impl<'a> _FIND_FREE_IDXW<'a> {
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
pub struct _INVALIDATE_ALLW<'a> {
    w: &'a mut W,
}
impl<'a> _INVALIDATE_ALLW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Contains the SAM table index to be enabled or invalidated"]
    #[inline]
    pub fn sam_index(&self) -> SAM_INDEXR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAM_INDEXR { bits }
    }
    #[doc = "Bits 8:23 - Software-computed source address checksum, to be installed into a table index"]
    #[inline]
    pub fn sam_checksum(&self) -> SAM_CHECKSUMR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SAM_CHECKSUMR { bits }
    }
    #[doc = "Bit 26 - State of AutoTxAck FramePending field when SAM Accelleration is Disabled"]
    #[inline]
    pub fn ack_frm_pnd(&self) -> ACK_FRM_PNDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACK_FRM_PNDR { bits }
    }
    #[doc = "Bit 27 - Manual Control for AutoTxAck FramePending field"]
    #[inline]
    pub fn ack_frm_pnd_ctrl(&self) -> ACK_FRM_PND_CTRLR {
        ACK_FRM_PND_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - SAM Table Update Status Bit"]
    #[inline]
    pub fn sam_busy(&self) -> SAM_BUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SAM_BUSYR { bits }
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
    #[doc = "Bits 0:6 - Contains the SAM table index to be enabled or invalidated"]
    #[inline]
    pub fn sam_index(&mut self) -> _SAM_INDEXW {
        _SAM_INDEXW { w: self }
    }
    #[doc = "Bit 7 - Enables SAM Table Contents to be updated"]
    #[inline]
    pub fn sam_index_wr(&mut self) -> _SAM_INDEX_WRW {
        _SAM_INDEX_WRW { w: self }
    }
    #[doc = "Bits 8:23 - Software-computed source address checksum, to be installed into a table index"]
    #[inline]
    pub fn sam_checksum(&mut self) -> _SAM_CHECKSUMW {
        _SAM_CHECKSUMW { w: self }
    }
    #[doc = "Bit 24 - Invalidate the SAM table index selected by SAM_INDEX"]
    #[inline]
    pub fn sam_index_inv(&mut self) -> _SAM_INDEX_INVW {
        _SAM_INDEX_INVW { w: self }
    }
    #[doc = "Bit 25 - Enable the SAM table index selected by SAM_INDEX"]
    #[inline]
    pub fn sam_index_en(&mut self) -> _SAM_INDEX_ENW {
        _SAM_INDEX_ENW { w: self }
    }
    #[doc = "Bit 26 - State of AutoTxAck FramePending field when SAM Accelleration is Disabled"]
    #[inline]
    pub fn ack_frm_pnd(&mut self) -> _ACK_FRM_PNDW {
        _ACK_FRM_PNDW { w: self }
    }
    #[doc = "Bit 27 - Manual Control for AutoTxAck FramePending field"]
    #[inline]
    pub fn ack_frm_pnd_ctrl(&mut self) -> _ACK_FRM_PND_CTRLW {
        _ACK_FRM_PND_CTRLW { w: self }
    }
    #[doc = "Bit 28 - Find First Free Index"]
    #[inline]
    pub fn find_free_idx(&mut self) -> _FIND_FREE_IDXW {
        _FIND_FREE_IDXW { w: self }
    }
    #[doc = "Bit 29 - Invalidate Entire SAM Table"]
    #[inline]
    pub fn invalidate_all(&mut self) -> _INVALIDATE_ALLW {
        _INVALIDATE_ALLW { w: self }
    }
}
