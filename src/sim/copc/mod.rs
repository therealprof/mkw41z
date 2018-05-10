#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::COPC {
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
#[doc = "Possible values of the field `COPW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPWR {
    #[doc = "Normal mode"]
    _0,
    #[doc = "Windowed mode"]
    _1,
}
impl COPWR {
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
            COPWR::_0 => false,
            COPWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COPWR {
        match value {
            false => COPWR::_0,
            true => COPWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COPWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COPWR::_1
    }
}
#[doc = "Possible values of the field `COPCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPCLKSR {
    #[doc = "COP configured for short timeout"]
    _0,
    #[doc = "COP configured for long timeout"]
    _1,
}
impl COPCLKSR {
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
            COPCLKSR::_0 => false,
            COPCLKSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COPCLKSR {
        match value {
            false => COPCLKSR::_0,
            true => COPCLKSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COPCLKSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COPCLKSR::_1
    }
}
#[doc = "Possible values of the field `COPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPTR {
    #[doc = "COP disabled"]
    _00,
    #[doc = "COP timeout after 25 cycles for short timeout or 213 cycles for long timeout"]
    _01,
    #[doc = "COP timeout after 28 cycles for short timeout or 216 cycles for long timeout"]
    _10,
    #[doc = "COP timeout after 210 cycles for short timeout or 218 cycles for long timeout"]
    _11,
}
impl COPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COPTR::_00 => 0,
            COPTR::_01 => 1,
            COPTR::_10 => 2,
            COPTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COPTR {
        match value {
            0 => COPTR::_00,
            1 => COPTR::_01,
            2 => COPTR::_10,
            3 => COPTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == COPTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == COPTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == COPTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == COPTR::_11
    }
}
#[doc = "Possible values of the field `COPSTPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPSTPENR {
    #[doc = "COP is disabled and the counter is reset in Stop modes"]
    _0,
    #[doc = "COP is enabled in Stop modes"]
    _1,
}
impl COPSTPENR {
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
            COPSTPENR::_0 => false,
            COPSTPENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COPSTPENR {
        match value {
            false => COPSTPENR::_0,
            true => COPSTPENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COPSTPENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COPSTPENR::_1
    }
}
#[doc = "Possible values of the field `COPDBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPDBGENR {
    #[doc = "COP is disabled and the counter is reset in Debug mode"]
    _0,
    #[doc = "COP is enabled in Debug mode"]
    _1,
}
impl COPDBGENR {
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
            COPDBGENR::_0 => false,
            COPDBGENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COPDBGENR {
        match value {
            false => COPDBGENR::_0,
            true => COPDBGENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COPDBGENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COPDBGENR::_1
    }
}
#[doc = "Possible values of the field `COPCLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPCLKSELR {
    #[doc = "LPO clock (1 kHz)"]
    _00,
    #[doc = "MCGIRCLK"]
    _01,
    #[doc = "OSCERCLK"]
    _10,
    #[doc = "Bus clock"]
    _11,
}
impl COPCLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COPCLKSELR::_00 => 0,
            COPCLKSELR::_01 => 1,
            COPCLKSELR::_10 => 2,
            COPCLKSELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COPCLKSELR {
        match value {
            0 => COPCLKSELR::_00,
            1 => COPCLKSELR::_01,
            2 => COPCLKSELR::_10,
            3 => COPCLKSELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == COPCLKSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == COPCLKSELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == COPCLKSELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == COPCLKSELR::_11
    }
}
#[doc = "Values that can be written to the field `COPW`"]
pub enum COPWW {
    #[doc = "Normal mode"]
    _0,
    #[doc = "Windowed mode"]
    _1,
}
impl COPWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COPWW::_0 => false,
            COPWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COPWW<'a> {
    w: &'a mut W,
}
impl<'a> _COPWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COPWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COPWW::_0)
    }
    #[doc = "Windowed mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COPWW::_1)
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
#[doc = "Values that can be written to the field `COPCLKS`"]
pub enum COPCLKSW {
    #[doc = "COP configured for short timeout"]
    _0,
    #[doc = "COP configured for long timeout"]
    _1,
}
impl COPCLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COPCLKSW::_0 => false,
            COPCLKSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COPCLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _COPCLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COPCLKSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COP configured for short timeout"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COPCLKSW::_0)
    }
    #[doc = "COP configured for long timeout"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COPCLKSW::_1)
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
#[doc = "Values that can be written to the field `COPT`"]
pub enum COPTW {
    #[doc = "COP disabled"]
    _00,
    #[doc = "COP timeout after 25 cycles for short timeout or 213 cycles for long timeout"]
    _01,
    #[doc = "COP timeout after 28 cycles for short timeout or 216 cycles for long timeout"]
    _10,
    #[doc = "COP timeout after 210 cycles for short timeout or 218 cycles for long timeout"]
    _11,
}
impl COPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COPTW::_00 => 0,
            COPTW::_01 => 1,
            COPTW::_10 => 2,
            COPTW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COPTW<'a> {
    w: &'a mut W,
}
impl<'a> _COPTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COPTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "COP disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(COPTW::_00)
    }
    #[doc = "COP timeout after 25 cycles for short timeout or 213 cycles for long timeout"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(COPTW::_01)
    }
    #[doc = "COP timeout after 28 cycles for short timeout or 216 cycles for long timeout"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(COPTW::_10)
    }
    #[doc = "COP timeout after 210 cycles for short timeout or 218 cycles for long timeout"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(COPTW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COPSTPEN`"]
pub enum COPSTPENW {
    #[doc = "COP is disabled and the counter is reset in Stop modes"]
    _0,
    #[doc = "COP is enabled in Stop modes"]
    _1,
}
impl COPSTPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COPSTPENW::_0 => false,
            COPSTPENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COPSTPENW<'a> {
    w: &'a mut W,
}
impl<'a> _COPSTPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COPSTPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COP is disabled and the counter is reset in Stop modes"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COPSTPENW::_0)
    }
    #[doc = "COP is enabled in Stop modes"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COPSTPENW::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COPDBGEN`"]
pub enum COPDBGENW {
    #[doc = "COP is disabled and the counter is reset in Debug mode"]
    _0,
    #[doc = "COP is enabled in Debug mode"]
    _1,
}
impl COPDBGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COPDBGENW::_0 => false,
            COPDBGENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COPDBGENW<'a> {
    w: &'a mut W,
}
impl<'a> _COPDBGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COPDBGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COP is disabled and the counter is reset in Debug mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COPDBGENW::_0)
    }
    #[doc = "COP is enabled in Debug mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COPDBGENW::_1)
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
#[doc = "Values that can be written to the field `COPCLKSEL`"]
pub enum COPCLKSELW {
    #[doc = "LPO clock (1 kHz)"]
    _00,
    #[doc = "MCGIRCLK"]
    _01,
    #[doc = "OSCERCLK"]
    _10,
    #[doc = "Bus clock"]
    _11,
}
impl COPCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COPCLKSELW::_00 => 0,
            COPCLKSELW::_01 => 1,
            COPCLKSELW::_10 => 2,
            COPCLKSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COPCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _COPCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COPCLKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(COPCLKSELW::_00)
    }
    #[doc = "MCGIRCLK"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(COPCLKSELW::_01)
    }
    #[doc = "OSCERCLK"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(COPCLKSELW::_10)
    }
    #[doc = "Bus clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(COPCLKSELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - COP Windowed Mode"]
    #[inline]
    pub fn copw(&self) -> COPWR {
        COPWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - COP Clock Select"]
    #[inline]
    pub fn copclks(&self) -> COPCLKSR {
        COPCLKSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - COP Watchdog Timeout"]
    #[inline]
    pub fn copt(&self) -> COPTR {
        COPTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - COP Stop Enable"]
    #[inline]
    pub fn copstpen(&self) -> COPSTPENR {
        COPSTPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - COP Debug Enable"]
    #[inline]
    pub fn copdbgen(&self) -> COPDBGENR {
        COPDBGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - COP Clock Select"]
    #[inline]
    pub fn copclksel(&self) -> COPCLKSELR {
        COPCLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 12 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - COP Windowed Mode"]
    #[inline]
    pub fn copw(&mut self) -> _COPWW {
        _COPWW { w: self }
    }
    #[doc = "Bit 1 - COP Clock Select"]
    #[inline]
    pub fn copclks(&mut self) -> _COPCLKSW {
        _COPCLKSW { w: self }
    }
    #[doc = "Bits 2:3 - COP Watchdog Timeout"]
    #[inline]
    pub fn copt(&mut self) -> _COPTW {
        _COPTW { w: self }
    }
    #[doc = "Bit 4 - COP Stop Enable"]
    #[inline]
    pub fn copstpen(&mut self) -> _COPSTPENW {
        _COPSTPENW { w: self }
    }
    #[doc = "Bit 5 - COP Debug Enable"]
    #[inline]
    pub fn copdbgen(&mut self) -> _COPDBGENW {
        _COPDBGENW { w: self }
    }
    #[doc = "Bits 6:7 - COP Clock Select"]
    #[inline]
    pub fn copclksel(&mut self) -> _COPCLKSELW {
        _COPCLKSELW { w: self }
    }
}
