#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BITRATE {
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
#[doc = "Possible values of the field `BITRATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITRATER {
    #[doc = "1Mbit/sec"]
    _0,
    #[doc = "500Kbit/sec"]
    _1,
    #[doc = "250Kbit/sec (not supported if WHITEN_CFG[MANCHESTER_EN]=1"]
    _2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BITRATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BITRATER::_0 => 0,
            BITRATER::_1 => 1,
            BITRATER::_2 => 2,
            BITRATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BITRATER {
        match value {
            0 => BITRATER::_0,
            1 => BITRATER::_1,
            2 => BITRATER::_2,
            i => BITRATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BITRATER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BITRATER::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == BITRATER::_2
    }
}
#[doc = "Values that can be written to the field `BITRATE`"]
pub enum BITRATEW {
    #[doc = "1Mbit/sec"]
    _0,
    #[doc = "500Kbit/sec"]
    _1,
    #[doc = "250Kbit/sec (not supported if WHITEN_CFG[MANCHESTER_EN]=1"]
    _2,
}
impl BITRATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BITRATEW::_0 => 0,
            BITRATEW::_1 => 1,
            BITRATEW::_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BITRATEW<'a> {
    w: &'a mut W,
}
impl<'a> _BITRATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BITRATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1Mbit/sec"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BITRATEW::_0)
    }
    #[doc = "500Kbit/sec"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BITRATEW::_1)
    }
    #[doc = "250Kbit/sec (not supported if WHITEN_CFG[MANCHESTER_EN]=1"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(BITRATEW::_2)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Bit Rate"]
    #[inline]
    pub fn bitrate(&self) -> BITRATER {
        BITRATER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - Bit Rate"]
    #[inline]
    pub fn bitrate(&mut self) -> _BITRATEW {
        _BITRATEW { w: self }
    }
}
