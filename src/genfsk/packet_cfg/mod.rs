#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PACKET_CFG {
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
pub struct LENGTH_SZR {
    bits: u8,
}
impl LENGTH_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `LENGTH_BIT_ORD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LENGTH_BIT_ORDR {
    #[doc = "LS Bit First"]
    _0,
    #[doc = "MS Bit First"]
    _1,
}
impl LENGTH_BIT_ORDR {
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
            LENGTH_BIT_ORDR::_0 => false,
            LENGTH_BIT_ORDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LENGTH_BIT_ORDR {
        match value {
            false => LENGTH_BIT_ORDR::_0,
            true => LENGTH_BIT_ORDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LENGTH_BIT_ORDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LENGTH_BIT_ORDR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SYNC_ADDR_SZR {
    bits: u8,
}
impl SYNC_ADDR_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LENGTH_ADJR {
    bits: u8,
}
impl LENGTH_ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LENGTH_FAILR {
    bits: bool,
}
impl LENGTH_FAILR {
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
pub struct H0_SZR {
    bits: u8,
}
impl H0_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct H0_FAILR {
    bits: bool,
}
impl H0_FAILR {
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
pub struct H1_SZR {
    bits: u8,
}
impl H1_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct H1_FAILR {
    bits: bool,
}
impl H1_FAILR {
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
pub struct _LENGTH_SZW<'a> {
    w: &'a mut W,
}
impl<'a> _LENGTH_SZW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LENGTH_BIT_ORD`"]
pub enum LENGTH_BIT_ORDW {
    #[doc = "LS Bit First"]
    _0,
    #[doc = "MS Bit First"]
    _1,
}
impl LENGTH_BIT_ORDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LENGTH_BIT_ORDW::_0 => false,
            LENGTH_BIT_ORDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LENGTH_BIT_ORDW<'a> {
    w: &'a mut W,
}
impl<'a> _LENGTH_BIT_ORDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LENGTH_BIT_ORDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LS Bit First"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LENGTH_BIT_ORDW::_0)
    }
    #[doc = "MS Bit First"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LENGTH_BIT_ORDW::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYNC_ADDR_SZW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC_ADDR_SZW<'a> {
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
pub struct _LENGTH_ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _LENGTH_ADJW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _H0_SZW<'a> {
    w: &'a mut W,
}
impl<'a> _H0_SZW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _H1_SZW<'a> {
    w: &'a mut W,
}
impl<'a> _H1_SZW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:4 - LENGTH Size"]
    #[inline]
    pub fn length_sz(&self) -> LENGTH_SZR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LENGTH_SZR { bits }
    }
    #[doc = "Bit 5 - LENGTH Bit Order"]
    #[inline]
    pub fn length_bit_ord(&self) -> LENGTH_BIT_ORDR {
        LENGTH_BIT_ORDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Sync Address Size"]
    #[inline]
    pub fn sync_addr_sz(&self) -> SYNC_ADDR_SZR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYNC_ADDR_SZR { bits }
    }
    #[doc = "Bits 8:13 - Length Adjustment"]
    #[inline]
    pub fn length_adj(&self) -> LENGTH_ADJR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LENGTH_ADJR { bits }
    }
    #[doc = "Bit 15 - Maximum Length Violated Status Bit"]
    #[inline]
    pub fn length_fail(&self) -> LENGTH_FAILR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LENGTH_FAILR { bits }
    }
    #[doc = "Bits 16:20 - H0 Size"]
    #[inline]
    pub fn h0_sz(&self) -> H0_SZR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        H0_SZR { bits }
    }
    #[doc = "Bit 23 - H0 Violated Status Bit"]
    #[inline]
    pub fn h0_fail(&self) -> H0_FAILR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        H0_FAILR { bits }
    }
    #[doc = "Bits 24:28 - H1 Size"]
    #[inline]
    pub fn h1_sz(&self) -> H1_SZR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        H1_SZR { bits }
    }
    #[doc = "Bit 31 - H1 Violated Status Bit"]
    #[inline]
    pub fn h1_fail(&self) -> H1_FAILR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        H1_FAILR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 64 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - LENGTH Size"]
    #[inline]
    pub fn length_sz(&mut self) -> _LENGTH_SZW {
        _LENGTH_SZW { w: self }
    }
    #[doc = "Bit 5 - LENGTH Bit Order"]
    #[inline]
    pub fn length_bit_ord(&mut self) -> _LENGTH_BIT_ORDW {
        _LENGTH_BIT_ORDW { w: self }
    }
    #[doc = "Bits 6:7 - Sync Address Size"]
    #[inline]
    pub fn sync_addr_sz(&mut self) -> _SYNC_ADDR_SZW {
        _SYNC_ADDR_SZW { w: self }
    }
    #[doc = "Bits 8:13 - Length Adjustment"]
    #[inline]
    pub fn length_adj(&mut self) -> _LENGTH_ADJW {
        _LENGTH_ADJW { w: self }
    }
    #[doc = "Bits 16:20 - H0 Size"]
    #[inline]
    pub fn h0_sz(&mut self) -> _H0_SZW {
        _H0_SZW { w: self }
    }
    #[doc = "Bits 24:28 - H1 Size"]
    #[inline]
    pub fn h1_sz(&mut self) -> _H1_SZW {
        _H1_SZW { w: self }
    }
}
