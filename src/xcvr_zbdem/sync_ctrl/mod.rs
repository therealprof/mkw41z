#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYNC_CTRL {
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
pub struct SYNC_PERR {
    bits: u8,
}
impl SYNC_PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TRACK_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACK_ENABLER {
    #[doc = "symbol timing synchronization tracking disabled in Rx frontend"]
    _0,
    #[doc = "symbol timing synchronization tracking enabled in Rx frontend (default)"]
    _1,
}
impl TRACK_ENABLER {
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
            TRACK_ENABLER::_0 => false,
            TRACK_ENABLER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRACK_ENABLER {
        match value {
            false => TRACK_ENABLER::_0,
            true => TRACK_ENABLER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRACK_ENABLER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRACK_ENABLER::_1
    }
}
#[doc = r" Proxy"]
pub struct _SYNC_PERW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC_PERW<'a> {
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
#[doc = "Values that can be written to the field `TRACK_ENABLE`"]
pub enum TRACK_ENABLEW {
    #[doc = "symbol timing synchronization tracking disabled in Rx frontend"]
    _0,
    #[doc = "symbol timing synchronization tracking enabled in Rx frontend (default)"]
    _1,
}
impl TRACK_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRACK_ENABLEW::_0 => false,
            TRACK_ENABLEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRACK_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACK_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRACK_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "symbol timing synchronization tracking disabled in Rx frontend"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRACK_ENABLEW::_0)
    }
    #[doc = "symbol timing synchronization tracking enabled in Rx frontend (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRACK_ENABLEW::_1)
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bits 0:2 - Symbol Sync Tracking Period"]
    #[inline]
    pub fn sync_per(&self) -> SYNC_PERR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYNC_PERR { bits }
    }
    #[doc = "Bit 3 - TRACK_ENABLE"]
    #[inline]
    pub fn track_enable(&self) -> TRACK_ENABLER {
        TRACK_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bits 0:2 - Symbol Sync Tracking Period"]
    #[inline]
    pub fn sync_per(&mut self) -> _SYNC_PERW {
        _SYNC_PERW { w: self }
    }
    #[doc = "Bit 3 - TRACK_ENABLE"]
    #[inline]
    pub fn track_enable(&mut self) -> _TRACK_ENABLEW {
        _TRACK_ENABLEW { w: self }
    }
}
