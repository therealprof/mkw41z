#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WHITEN_SZ_THR {
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
pub struct WHITEN_SZ_THRR {
    bits: u16,
}
impl WHITEN_SZ_THRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LENGTH_MAXR {
    bits: u8,
}
impl LENGTH_MAXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `REC_BAD_PKT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REC_BAD_PKTR {
    #[doc = "packets which fail H0, H1, or LENGTH_MAX result in an automatic recycle after the header is received and parsed"]
    _0,
    #[doc = "packets which fail H0, H1, or LENGTH_MAX are received in their entirety"]
    _1,
}
impl REC_BAD_PKTR {
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
            REC_BAD_PKTR::_0 => false,
            REC_BAD_PKTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REC_BAD_PKTR {
        match value {
            false => REC_BAD_PKTR::_0,
            true => REC_BAD_PKTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == REC_BAD_PKTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == REC_BAD_PKTR::_1
    }
}
#[doc = r" Proxy"]
pub struct _WHITEN_SZ_THRW<'a> {
    w: &'a mut W,
}
impl<'a> _WHITEN_SZ_THRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LENGTH_MAXW<'a> {
    w: &'a mut W,
}
impl<'a> _LENGTH_MAXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REC_BAD_PKT`"]
pub enum REC_BAD_PKTW {
    #[doc = "packets which fail H0, H1, or LENGTH_MAX result in an automatic recycle after the header is received and parsed"]
    _0,
    #[doc = "packets which fail H0, H1, or LENGTH_MAX are received in their entirety"]
    _1,
}
impl REC_BAD_PKTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REC_BAD_PKTW::_0 => false,
            REC_BAD_PKTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REC_BAD_PKTW<'a> {
    w: &'a mut W,
}
impl<'a> _REC_BAD_PKTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REC_BAD_PKTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "packets which fail H0, H1, or LENGTH_MAX result in an automatic recycle after the header is received and parsed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(REC_BAD_PKTW::_0)
    }
    #[doc = "packets which fail H0, H1, or LENGTH_MAX are received in their entirety"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(REC_BAD_PKTW::_1)
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bits 0:11 - Whitener Size Threshold"]
    #[inline]
    pub fn whiten_sz_thr(&self) -> WHITEN_SZ_THRR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WHITEN_SZ_THRR { bits }
    }
    #[doc = "Bits 16:22 - Maximum Length for Received Packets"]
    #[inline]
    pub fn length_max(&self) -> LENGTH_MAXR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LENGTH_MAXR { bits }
    }
    #[doc = "Bit 23 - Receive Bad Packets"]
    #[inline]
    pub fn rec_bad_pkt(&self) -> REC_BAD_PKTR {
        REC_BAD_PKTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2048 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:11 - Whitener Size Threshold"]
    #[inline]
    pub fn whiten_sz_thr(&mut self) -> _WHITEN_SZ_THRW {
        _WHITEN_SZ_THRW { w: self }
    }
    #[doc = "Bits 16:22 - Maximum Length for Received Packets"]
    #[inline]
    pub fn length_max(&mut self) -> _LENGTH_MAXW {
        _LENGTH_MAXW { w: self }
    }
    #[doc = "Bit 23 - Receive Bad Packets"]
    #[inline]
    pub fn rec_bad_pkt(&mut self) -> _REC_BAD_PKTW {
        _REC_BAD_PKTW { w: self }
    }
}
