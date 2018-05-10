#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WHITEN_CFG {
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
#[doc = "Possible values of the field `WHITEN_START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WHITEN_STARTR {
    #[doc = "no whitening"]
    _0,
    #[doc = "start whitening at start-of-H0"]
    _1,
    #[doc = "start whitening at start-of-H1 but only if LENGTH > WHITEN_SZ_THR"]
    _2,
    #[doc = "start whitening at start-of-payload but only if LENGTH > WHITEN_SZ_THR"]
    _3,
}
impl WHITEN_STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WHITEN_STARTR::_0 => 0,
            WHITEN_STARTR::_1 => 1,
            WHITEN_STARTR::_2 => 2,
            WHITEN_STARTR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WHITEN_STARTR {
        match value {
            0 => WHITEN_STARTR::_0,
            1 => WHITEN_STARTR::_1,
            2 => WHITEN_STARTR::_2,
            3 => WHITEN_STARTR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WHITEN_STARTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WHITEN_STARTR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == WHITEN_STARTR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == WHITEN_STARTR::_3
    }
}
#[doc = "Possible values of the field `WHITEN_END`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WHITEN_ENDR {
    #[doc = "end whiten at end-of-payload"]
    _0,
    #[doc = "end whiten at end-of-crc"]
    _1,
}
impl WHITEN_ENDR {
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
            WHITEN_ENDR::_0 => false,
            WHITEN_ENDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WHITEN_ENDR {
        match value {
            false => WHITEN_ENDR::_0,
            true => WHITEN_ENDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WHITEN_ENDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WHITEN_ENDR::_1
    }
}
#[doc = "Possible values of the field `WHITEN_B4_CRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WHITEN_B4_CRCR {
    #[doc = "CRC before whiten/de-whiten"]
    _0,
    #[doc = "Whiten/de-whiten before CRC"]
    _1,
}
impl WHITEN_B4_CRCR {
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
            WHITEN_B4_CRCR::_0 => false,
            WHITEN_B4_CRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WHITEN_B4_CRCR {
        match value {
            false => WHITEN_B4_CRCR::_0,
            true => WHITEN_B4_CRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WHITEN_B4_CRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WHITEN_B4_CRCR::_1
    }
}
#[doc = r" Value of the field"]
pub struct WHITEN_POLY_TYPER {
    bits: bool,
}
impl WHITEN_POLY_TYPER {
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
pub struct WHITEN_REF_INR {
    bits: bool,
}
impl WHITEN_REF_INR {
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
#[doc = "Possible values of the field `WHITEN_PAYLOAD_REINIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WHITEN_PAYLOAD_REINITR {
    #[doc = "Don't re-initialize Whitener LFSR at start-of-payload"]
    _0,
    #[doc = "Re-initialize Whitener LFSR at start-of-payload"]
    _1,
}
impl WHITEN_PAYLOAD_REINITR {
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
            WHITEN_PAYLOAD_REINITR::_0 => false,
            WHITEN_PAYLOAD_REINITR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WHITEN_PAYLOAD_REINITR {
        match value {
            false => WHITEN_PAYLOAD_REINITR::_0,
            true => WHITEN_PAYLOAD_REINITR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WHITEN_PAYLOAD_REINITR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WHITEN_PAYLOAD_REINITR::_1
    }
}
#[doc = r" Value of the field"]
pub struct WHITEN_SIZER {
    bits: u8,
}
impl WHITEN_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MANCHESTER_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MANCHESTER_ENR {
    #[doc = "Disable Manchester encoding (TX) and decoding (RX)"]
    _0,
    #[doc = "Enable Manchester encoding (TX) and decoding (RX)"]
    _1,
}
impl MANCHESTER_ENR {
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
            MANCHESTER_ENR::_0 => false,
            MANCHESTER_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MANCHESTER_ENR {
        match value {
            false => MANCHESTER_ENR::_0,
            true => MANCHESTER_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MANCHESTER_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MANCHESTER_ENR::_1
    }
}
#[doc = "Possible values of the field `MANCHESTER_INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MANCHESTER_INVR {
    #[doc = "Manchester coding as per 802.3"]
    _0,
    #[doc = "Manchester coding as per 802.3 but with the encoding signal inverted"]
    _1,
}
impl MANCHESTER_INVR {
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
            MANCHESTER_INVR::_0 => false,
            MANCHESTER_INVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MANCHESTER_INVR {
        match value {
            false => MANCHESTER_INVR::_0,
            true => MANCHESTER_INVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MANCHESTER_INVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MANCHESTER_INVR::_1
    }
}
#[doc = "Possible values of the field `MANCHESTER_START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MANCHESTER_STARTR {
    #[doc = "Start Manchester coding at start-of-payload"]
    _0,
    #[doc = "Start Manchester coding at start-of-header"]
    _1,
}
impl MANCHESTER_STARTR {
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
            MANCHESTER_STARTR::_0 => false,
            MANCHESTER_STARTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MANCHESTER_STARTR {
        match value {
            false => MANCHESTER_STARTR::_0,
            true => MANCHESTER_STARTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MANCHESTER_STARTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MANCHESTER_STARTR::_1
    }
}
#[doc = r" Value of the field"]
pub struct WHITEN_INITR {
    bits: u16,
}
impl WHITEN_INITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `WHITEN_START`"]
pub enum WHITEN_STARTW {
    #[doc = "no whitening"]
    _0,
    #[doc = "start whitening at start-of-H0"]
    _1,
    #[doc = "start whitening at start-of-H1 but only if LENGTH > WHITEN_SZ_THR"]
    _2,
    #[doc = "start whitening at start-of-payload but only if LENGTH > WHITEN_SZ_THR"]
    _3,
}
impl WHITEN_STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WHITEN_STARTW::_0 => 0,
            WHITEN_STARTW::_1 => 1,
            WHITEN_STARTW::_2 => 2,
            WHITEN_STARTW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WHITEN_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _WHITEN_STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WHITEN_STARTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "no whitening"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WHITEN_STARTW::_0)
    }
    #[doc = "start whitening at start-of-H0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WHITEN_STARTW::_1)
    }
    #[doc = "start whitening at start-of-H1 but only if LENGTH > WHITEN_SZ_THR"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(WHITEN_STARTW::_2)
    }
    #[doc = "start whitening at start-of-payload but only if LENGTH > WHITEN_SZ_THR"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(WHITEN_STARTW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WHITEN_END`"]
pub enum WHITEN_ENDW {
    #[doc = "end whiten at end-of-payload"]
    _0,
    #[doc = "end whiten at end-of-crc"]
    _1,
}
impl WHITEN_ENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WHITEN_ENDW::_0 => false,
            WHITEN_ENDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WHITEN_ENDW<'a> {
    w: &'a mut W,
}
impl<'a> _WHITEN_ENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WHITEN_ENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "end whiten at end-of-payload"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WHITEN_ENDW::_0)
    }
    #[doc = "end whiten at end-of-crc"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WHITEN_ENDW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WHITEN_B4_CRC`"]
pub enum WHITEN_B4_CRCW {
    #[doc = "CRC before whiten/de-whiten"]
    _0,
    #[doc = "Whiten/de-whiten before CRC"]
    _1,
}
impl WHITEN_B4_CRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WHITEN_B4_CRCW::_0 => false,
            WHITEN_B4_CRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WHITEN_B4_CRCW<'a> {
    w: &'a mut W,
}
impl<'a> _WHITEN_B4_CRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WHITEN_B4_CRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CRC before whiten/de-whiten"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WHITEN_B4_CRCW::_0)
    }
    #[doc = "Whiten/de-whiten before CRC"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WHITEN_B4_CRCW::_1)
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
#[doc = r" Proxy"]
pub struct _WHITEN_POLY_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _WHITEN_POLY_TYPEW<'a> {
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
pub struct _WHITEN_REF_INW<'a> {
    w: &'a mut W,
}
impl<'a> _WHITEN_REF_INW<'a> {
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
#[doc = "Values that can be written to the field `WHITEN_PAYLOAD_REINIT`"]
pub enum WHITEN_PAYLOAD_REINITW {
    #[doc = "Don't re-initialize Whitener LFSR at start-of-payload"]
    _0,
    #[doc = "Re-initialize Whitener LFSR at start-of-payload"]
    _1,
}
impl WHITEN_PAYLOAD_REINITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WHITEN_PAYLOAD_REINITW::_0 => false,
            WHITEN_PAYLOAD_REINITW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WHITEN_PAYLOAD_REINITW<'a> {
    w: &'a mut W,
}
impl<'a> _WHITEN_PAYLOAD_REINITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WHITEN_PAYLOAD_REINITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Don't re-initialize Whitener LFSR at start-of-payload"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WHITEN_PAYLOAD_REINITW::_0)
    }
    #[doc = "Re-initialize Whitener LFSR at start-of-payload"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WHITEN_PAYLOAD_REINITW::_1)
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
#[doc = r" Proxy"]
pub struct _WHITEN_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _WHITEN_SIZEW<'a> {
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
#[doc = "Values that can be written to the field `MANCHESTER_EN`"]
pub enum MANCHESTER_ENW {
    #[doc = "Disable Manchester encoding (TX) and decoding (RX)"]
    _0,
    #[doc = "Enable Manchester encoding (TX) and decoding (RX)"]
    _1,
}
impl MANCHESTER_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MANCHESTER_ENW::_0 => false,
            MANCHESTER_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MANCHESTER_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _MANCHESTER_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MANCHESTER_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Manchester encoding (TX) and decoding (RX)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MANCHESTER_ENW::_0)
    }
    #[doc = "Enable Manchester encoding (TX) and decoding (RX)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MANCHESTER_ENW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MANCHESTER_INV`"]
pub enum MANCHESTER_INVW {
    #[doc = "Manchester coding as per 802.3"]
    _0,
    #[doc = "Manchester coding as per 802.3 but with the encoding signal inverted"]
    _1,
}
impl MANCHESTER_INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MANCHESTER_INVW::_0 => false,
            MANCHESTER_INVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MANCHESTER_INVW<'a> {
    w: &'a mut W,
}
impl<'a> _MANCHESTER_INVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MANCHESTER_INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Manchester coding as per 802.3"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MANCHESTER_INVW::_0)
    }
    #[doc = "Manchester coding as per 802.3 but with the encoding signal inverted"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MANCHESTER_INVW::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MANCHESTER_START`"]
pub enum MANCHESTER_STARTW {
    #[doc = "Start Manchester coding at start-of-payload"]
    _0,
    #[doc = "Start Manchester coding at start-of-header"]
    _1,
}
impl MANCHESTER_STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MANCHESTER_STARTW::_0 => false,
            MANCHESTER_STARTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MANCHESTER_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _MANCHESTER_STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MANCHESTER_STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Start Manchester coding at start-of-payload"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MANCHESTER_STARTW::_0)
    }
    #[doc = "Start Manchester coding at start-of-header"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MANCHESTER_STARTW::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WHITEN_INITW<'a> {
    w: &'a mut W,
}
impl<'a> _WHITEN_INITW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:1 - Configure Whitener Start Point"]
    #[inline]
    pub fn whiten_start(&self) -> WHITEN_STARTR {
        WHITEN_STARTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Configure end-of-whitening"]
    #[inline]
    pub fn whiten_end(&self) -> WHITEN_ENDR {
        WHITEN_ENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Congifure for Whitening-before-CRC"]
    #[inline]
    pub fn whiten_b4_crc(&self) -> WHITEN_B4_CRCR {
        WHITEN_B4_CRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Whiten Polynomial Type"]
    #[inline]
    pub fn whiten_poly_type(&self) -> WHITEN_POLY_TYPER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WHITEN_POLY_TYPER { bits }
    }
    #[doc = "Bit 5 - Whiten Reflect Input"]
    #[inline]
    pub fn whiten_ref_in(&self) -> WHITEN_REF_INR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WHITEN_REF_INR { bits }
    }
    #[doc = "Bit 6 - Configure for Whitener re-initialization"]
    #[inline]
    pub fn whiten_payload_reinit(&self) -> WHITEN_PAYLOAD_REINITR {
        WHITEN_PAYLOAD_REINITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Length of Whitener LFSR"]
    #[inline]
    pub fn whiten_size(&self) -> WHITEN_SIZER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WHITEN_SIZER { bits }
    }
    #[doc = "Bit 12 - Configure for Manchester Encoding/Decoding"]
    #[inline]
    pub fn manchester_en(&self) -> MANCHESTER_ENR {
        MANCHESTER_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Configure for Inverted Manchester Encoding"]
    #[inline]
    pub fn manchester_inv(&self) -> MANCHESTER_INVR {
        MANCHESTER_INVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Configure Manchester Encoding Start Point"]
    #[inline]
    pub fn manchester_start(&self) -> MANCHESTER_STARTR {
        MANCHESTER_STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:24 - Initialization Value for Whitening/De-whitening"]
    #[inline]
    pub fn whiten_init(&self) -> WHITEN_INITR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WHITEN_INITR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 33491224 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Configure Whitener Start Point"]
    #[inline]
    pub fn whiten_start(&mut self) -> _WHITEN_STARTW {
        _WHITEN_STARTW { w: self }
    }
    #[doc = "Bit 2 - Configure end-of-whitening"]
    #[inline]
    pub fn whiten_end(&mut self) -> _WHITEN_ENDW {
        _WHITEN_ENDW { w: self }
    }
    #[doc = "Bit 3 - Congifure for Whitening-before-CRC"]
    #[inline]
    pub fn whiten_b4_crc(&mut self) -> _WHITEN_B4_CRCW {
        _WHITEN_B4_CRCW { w: self }
    }
    #[doc = "Bit 4 - Whiten Polynomial Type"]
    #[inline]
    pub fn whiten_poly_type(&mut self) -> _WHITEN_POLY_TYPEW {
        _WHITEN_POLY_TYPEW { w: self }
    }
    #[doc = "Bit 5 - Whiten Reflect Input"]
    #[inline]
    pub fn whiten_ref_in(&mut self) -> _WHITEN_REF_INW {
        _WHITEN_REF_INW { w: self }
    }
    #[doc = "Bit 6 - Configure for Whitener re-initialization"]
    #[inline]
    pub fn whiten_payload_reinit(&mut self) -> _WHITEN_PAYLOAD_REINITW {
        _WHITEN_PAYLOAD_REINITW { w: self }
    }
    #[doc = "Bits 8:11 - Length of Whitener LFSR"]
    #[inline]
    pub fn whiten_size(&mut self) -> _WHITEN_SIZEW {
        _WHITEN_SIZEW { w: self }
    }
    #[doc = "Bit 12 - Configure for Manchester Encoding/Decoding"]
    #[inline]
    pub fn manchester_en(&mut self) -> _MANCHESTER_ENW {
        _MANCHESTER_ENW { w: self }
    }
    #[doc = "Bit 13 - Configure for Inverted Manchester Encoding"]
    #[inline]
    pub fn manchester_inv(&mut self) -> _MANCHESTER_INVW {
        _MANCHESTER_INVW { w: self }
    }
    #[doc = "Bit 14 - Configure Manchester Encoding Start Point"]
    #[inline]
    pub fn manchester_start(&mut self) -> _MANCHESTER_STARTW {
        _MANCHESTER_STARTW { w: self }
    }
    #[doc = "Bits 16:24 - Initialization Value for Whitening/De-whitening"]
    #[inline]
    pub fn whiten_init(&mut self) -> _WHITEN_INITW {
        _WHITEN_INITW { w: self }
    }
}
