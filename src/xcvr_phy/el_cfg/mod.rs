#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EL_CFG {
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
#[doc = "Possible values of the field `EL_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EL_ENABLER {
    #[doc = "Disable Early/Late"]
    _0,
    #[doc = "Enable Early/Late"]
    _1,
}
impl EL_ENABLER {
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
            EL_ENABLER::_0 => false,
            EL_ENABLER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EL_ENABLER {
        match value {
            false => EL_ENABLER::_0,
            true => EL_ENABLER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EL_ENABLER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EL_ENABLER::_1
    }
}
#[doc = "Possible values of the field `EL_ZB_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EL_ZB_ENABLER {
    #[doc = "Disable Early/Late"]
    _0,
    #[doc = "Enable Early/Late"]
    _1,
}
impl EL_ZB_ENABLER {
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
            EL_ZB_ENABLER::_0 => false,
            EL_ZB_ENABLER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EL_ZB_ENABLER {
        match value {
            false => EL_ZB_ENABLER::_0,
            true => EL_ZB_ENABLER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EL_ZB_ENABLER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EL_ZB_ENABLER::_1
    }
}
#[doc = "Possible values of the field `EL_ZB_WIN_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EL_ZB_WIN_SIZER {
    #[doc = "2 symbols"]
    _0,
    #[doc = "3 symbols"]
    _1,
}
impl EL_ZB_WIN_SIZER {
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
            EL_ZB_WIN_SIZER::_0 => false,
            EL_ZB_WIN_SIZER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EL_ZB_WIN_SIZER {
        match value {
            false => EL_ZB_WIN_SIZER::_0,
            true => EL_ZB_WIN_SIZER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EL_ZB_WIN_SIZER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EL_ZB_WIN_SIZER::_1
    }
}
#[doc = r" Value of the field"]
pub struct EL_WIN_SIZER {
    bits: u8,
}
impl EL_WIN_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EL_INTERVALR {
    bits: u8,
}
impl EL_INTERVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `EL_ENABLE`"]
pub enum EL_ENABLEW {
    #[doc = "Disable Early/Late"]
    _0,
    #[doc = "Enable Early/Late"]
    _1,
}
impl EL_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EL_ENABLEW::_0 => false,
            EL_ENABLEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EL_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _EL_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EL_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Early/Late"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EL_ENABLEW::_0)
    }
    #[doc = "Enable Early/Late"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EL_ENABLEW::_1)
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
#[doc = "Values that can be written to the field `EL_ZB_ENABLE`"]
pub enum EL_ZB_ENABLEW {
    #[doc = "Disable Early/Late"]
    _0,
    #[doc = "Enable Early/Late"]
    _1,
}
impl EL_ZB_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EL_ZB_ENABLEW::_0 => false,
            EL_ZB_ENABLEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EL_ZB_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _EL_ZB_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EL_ZB_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Early/Late"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EL_ZB_ENABLEW::_0)
    }
    #[doc = "Enable Early/Late"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EL_ZB_ENABLEW::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EL_ZB_WIN_SIZE`"]
pub enum EL_ZB_WIN_SIZEW {
    #[doc = "2 symbols"]
    _0,
    #[doc = "3 symbols"]
    _1,
}
impl EL_ZB_WIN_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EL_ZB_WIN_SIZEW::_0 => false,
            EL_ZB_WIN_SIZEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EL_ZB_WIN_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _EL_ZB_WIN_SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EL_ZB_WIN_SIZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "2 symbols"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EL_ZB_WIN_SIZEW::_0)
    }
    #[doc = "3 symbols"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EL_ZB_WIN_SIZEW::_1)
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
#[doc = r" Proxy"]
pub struct _EL_WIN_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _EL_WIN_SIZEW<'a> {
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
#[doc = r" Proxy"]
pub struct _EL_INTERVALW<'a> {
    w: &'a mut W,
}
impl<'a> _EL_INTERVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bit 0 - EL_ENABLE"]
    #[inline]
    pub fn el_enable(&self) -> EL_ENABLER {
        EL_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - EL_ZB_ENABLE"]
    #[inline]
    pub fn el_zb_enable(&self) -> EL_ZB_ENABLER {
        EL_ZB_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - EL_ZB_WIN_SIZE"]
    #[inline]
    pub fn el_zb_win_size(&self) -> EL_ZB_WIN_SIZER {
        EL_ZB_WIN_SIZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - EL_WIN_SIZE"]
    #[inline]
    pub fn el_win_size(&self) -> EL_WIN_SIZER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EL_WIN_SIZER { bits }
    }
    #[doc = "Bits 16:21 - EL_INTERVAL"]
    #[inline]
    pub fn el_interval(&self) -> EL_INTERVALR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EL_INTERVALR { bits }
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
    #[doc = "Bit 0 - EL_ENABLE"]
    #[inline]
    pub fn el_enable(&mut self) -> _EL_ENABLEW {
        _EL_ENABLEW { w: self }
    }
    #[doc = "Bit 1 - EL_ZB_ENABLE"]
    #[inline]
    pub fn el_zb_enable(&mut self) -> _EL_ZB_ENABLEW {
        _EL_ZB_ENABLEW { w: self }
    }
    #[doc = "Bit 2 - EL_ZB_WIN_SIZE"]
    #[inline]
    pub fn el_zb_win_size(&mut self) -> _EL_ZB_WIN_SIZEW {
        _EL_ZB_WIN_SIZEW { w: self }
    }
    #[doc = "Bits 8:11 - EL_WIN_SIZE"]
    #[inline]
    pub fn el_win_size(&mut self) -> _EL_WIN_SIZEW {
        _EL_WIN_SIZEW { w: self }
    }
    #[doc = "Bits 16:21 - EL_INTERVAL"]
    #[inline]
    pub fn el_interval(&mut self) -> _EL_INTERVALW {
        _EL_INTERVALW { w: self }
    }
}
