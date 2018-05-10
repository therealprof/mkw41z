#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_WATERMARK {
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
pub struct RX_WATERMARKR {
    bits: u8,
}
impl RX_WATERMARKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BYTE_COUNTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE_COUNTERR {
    #[doc = "First Byte of Network Address"]
    _0,
    #[doc = "Second Byte of Network Address"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BYTE_COUNTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYTE_COUNTERR::_0 => 0,
            BYTE_COUNTERR::_1 => 1,
            BYTE_COUNTERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYTE_COUNTERR {
        match value {
            0 => BYTE_COUNTERR::_0,
            1 => BYTE_COUNTERR::_1,
            i => BYTE_COUNTERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BYTE_COUNTERR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BYTE_COUNTERR::_1
    }
}
#[doc = r" Proxy"]
pub struct _RX_WATERMARKW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_WATERMARKW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - RX Watermark"]
    #[inline]
    pub fn rx_watermark(&self) -> RX_WATERMARKR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_WATERMARKR { bits }
    }
    #[doc = "Bits 16:22 - Byte Counter"]
    #[inline]
    pub fn byte_counter(&self) -> BYTE_COUNTERR {
        BYTE_COUNTERR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 127 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - RX Watermark"]
    #[inline]
    pub fn rx_watermark(&mut self) -> _RX_WATERMARKW {
        _RX_WATERMARKW { w: self }
    }
}
