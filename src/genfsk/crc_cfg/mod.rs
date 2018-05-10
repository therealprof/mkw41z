#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CRC_CFG {
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
pub struct CRC_SZR {
    bits: u8,
}
impl CRC_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CRC_START_BYTER {
    bits: u8,
}
impl CRC_START_BYTER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CRC_REF_IN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_REF_INR {
    #[doc = "do not manipulate input data stream"]
    _0,
    #[doc = "reflect each byte in the input stream bitwise"]
    _1,
}
impl CRC_REF_INR {
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
            CRC_REF_INR::_0 => false,
            CRC_REF_INR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC_REF_INR {
        match value {
            false => CRC_REF_INR::_0,
            true => CRC_REF_INR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRC_REF_INR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRC_REF_INR::_1
    }
}
#[doc = "Possible values of the field `CRC_REF_OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_REF_OUTR {
    #[doc = "do not manipulate CRC result"]
    _0,
    #[doc = "CRC result is to be reflected bitwise (operated on entire word)"]
    _1,
}
impl CRC_REF_OUTR {
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
            CRC_REF_OUTR::_0 => false,
            CRC_REF_OUTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC_REF_OUTR {
        match value {
            false => CRC_REF_OUTR::_0,
            true => CRC_REF_OUTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRC_REF_OUTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRC_REF_OUTR::_1
    }
}
#[doc = "Possible values of the field `CRC_BYTE_ORD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_BYTE_ORDR {
    #[doc = "LS Byte First"]
    _0,
    #[doc = "MS Byte First"]
    _1,
}
impl CRC_BYTE_ORDR {
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
            CRC_BYTE_ORDR::_0 => false,
            CRC_BYTE_ORDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC_BYTE_ORDR {
        match value {
            false => CRC_BYTE_ORDR::_0,
            true => CRC_BYTE_ORDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRC_BYTE_ORDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRC_BYTE_ORDR::_1
    }
}
#[doc = r" Proxy"]
pub struct _CRC_SZW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_SZW<'a> {
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
#[doc = r" Proxy"]
pub struct _CRC_START_BYTEW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_START_BYTEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRC_REF_IN`"]
pub enum CRC_REF_INW {
    #[doc = "do not manipulate input data stream"]
    _0,
    #[doc = "reflect each byte in the input stream bitwise"]
    _1,
}
impl CRC_REF_INW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRC_REF_INW::_0 => false,
            CRC_REF_INW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC_REF_INW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_REF_INW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC_REF_INW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not manipulate input data stream"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_REF_INW::_0)
    }
    #[doc = "reflect each byte in the input stream bitwise"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_REF_INW::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRC_REF_OUT`"]
pub enum CRC_REF_OUTW {
    #[doc = "do not manipulate CRC result"]
    _0,
    #[doc = "CRC result is to be reflected bitwise (operated on entire word)"]
    _1,
}
impl CRC_REF_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRC_REF_OUTW::_0 => false,
            CRC_REF_OUTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC_REF_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_REF_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC_REF_OUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not manipulate CRC result"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_REF_OUTW::_0)
    }
    #[doc = "CRC result is to be reflected bitwise (operated on entire word)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_REF_OUTW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRC_BYTE_ORD`"]
pub enum CRC_BYTE_ORDW {
    #[doc = "LS Byte First"]
    _0,
    #[doc = "MS Byte First"]
    _1,
}
impl CRC_BYTE_ORDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRC_BYTE_ORDW::_0 => false,
            CRC_BYTE_ORDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC_BYTE_ORDW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_BYTE_ORDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC_BYTE_ORDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LS Byte First"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_BYTE_ORDW::_0)
    }
    #[doc = "MS Byte First"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_BYTE_ORDW::_1)
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bits 0:2 - CRC Size (in octets)"]
    #[inline]
    pub fn crc_sz(&self) -> CRC_SZR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CRC_SZR { bits }
    }
    #[doc = "Bits 8:11 - Configure CRC Start Point"]
    #[inline]
    pub fn crc_start_byte(&self) -> CRC_START_BYTER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CRC_START_BYTER { bits }
    }
    #[doc = "Bit 16 - CRC Reflect In"]
    #[inline]
    pub fn crc_ref_in(&self) -> CRC_REF_INR {
        CRC_REF_INR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - CRC Reflect Out"]
    #[inline]
    pub fn crc_ref_out(&self) -> CRC_REF_OUTR {
        CRC_REF_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - CRC Byte Order"]
    #[inline]
    pub fn crc_byte_ord(&self) -> CRC_BYTE_ORDR {
        CRC_BYTE_ORDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - CRC Size (in octets)"]
    #[inline]
    pub fn crc_sz(&mut self) -> _CRC_SZW {
        _CRC_SZW { w: self }
    }
    #[doc = "Bits 8:11 - Configure CRC Start Point"]
    #[inline]
    pub fn crc_start_byte(&mut self) -> _CRC_START_BYTEW {
        _CRC_START_BYTEW { w: self }
    }
    #[doc = "Bit 16 - CRC Reflect In"]
    #[inline]
    pub fn crc_ref_in(&mut self) -> _CRC_REF_INW {
        _CRC_REF_INW { w: self }
    }
    #[doc = "Bit 17 - CRC Reflect Out"]
    #[inline]
    pub fn crc_ref_out(&mut self) -> _CRC_REF_OUTW {
        _CRC_REF_OUTW { w: self }
    }
    #[doc = "Bit 18 - CRC Byte Order"]
    #[inline]
    pub fn crc_byte_ord(&mut self) -> _CRC_BYTE_ORDW {
        _CRC_BYTE_ORDW { w: self }
    }
}
