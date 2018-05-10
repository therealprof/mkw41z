#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPM_CTRL {
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
pub struct PLL_LD_MANUALR {
    bits: u8,
}
impl PLL_LD_MANUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PLL_LD_DISABLER {
    bits: bool,
}
impl PLL_LD_DISABLER {
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
pub struct LPFFR {
    bits: bool,
}
impl LPFFR {
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
pub struct LPM_SDM_INVR {
    bits: bool,
}
impl LPM_SDM_INVR {
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
pub struct LPM_DISABLER {
    bits: bool,
}
impl LPM_DISABLER {
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
#[doc = "Possible values of the field `LPM_DTH_SCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM_DTH_SCLR {
    #[doc = "-128 to 96"]
    _0101,
    #[doc = "-256 to 192"]
    _0110,
    #[doc = "-512 to 384"]
    _0111,
    #[doc = "-1024 to 768"]
    _1000,
    #[doc = "-2048 to 1536"]
    _1001,
    #[doc = "-4096 to 3072"]
    _1010,
    #[doc = "-8192 to 6144"]
    _1011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPM_DTH_SCLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPM_DTH_SCLR::_0101 => 5,
            LPM_DTH_SCLR::_0110 => 6,
            LPM_DTH_SCLR::_0111 => 7,
            LPM_DTH_SCLR::_1000 => 8,
            LPM_DTH_SCLR::_1001 => 9,
            LPM_DTH_SCLR::_1010 => 10,
            LPM_DTH_SCLR::_1011 => 11,
            LPM_DTH_SCLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPM_DTH_SCLR {
        match value {
            5 => LPM_DTH_SCLR::_0101,
            6 => LPM_DTH_SCLR::_0110,
            7 => LPM_DTH_SCLR::_0111,
            8 => LPM_DTH_SCLR::_1000,
            9 => LPM_DTH_SCLR::_1001,
            10 => LPM_DTH_SCLR::_1010,
            11 => LPM_DTH_SCLR::_1011,
            i => LPM_DTH_SCLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == LPM_DTH_SCLR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == LPM_DTH_SCLR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == LPM_DTH_SCLR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == LPM_DTH_SCLR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == LPM_DTH_SCLR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == LPM_DTH_SCLR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == LPM_DTH_SCLR::_1011
    }
}
#[doc = r" Value of the field"]
pub struct LPM_D_CTRLR {
    bits: bool,
}
impl LPM_D_CTRLR {
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
pub struct LPM_D_OVRDR {
    bits: bool,
}
impl LPM_D_OVRDR {
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
#[doc = "Possible values of the field `LPM_SCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM_SCALER {
    #[doc = "No Scaling"]
    _0000,
    #[doc = "Multiply by 2"]
    _0001,
    #[doc = "Multiply by 4"]
    _0010,
    #[doc = "Multiply by 8"]
    _0011,
    #[doc = "Multiply by 16"]
    _0100,
    #[doc = "Multiply by 32"]
    _0101,
    #[doc = "Multiply by 64"]
    _0110,
    #[doc = "Multiply by 128"]
    _0111,
    #[doc = "Multiply by 256"]
    _1000,
    #[doc = "Multiply by 512"]
    _1001,
    #[doc = "Multiply by 1024"]
    _1010,
    #[doc = "Multiply by 2048"]
    _1011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPM_SCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPM_SCALER::_0000 => 0,
            LPM_SCALER::_0001 => 1,
            LPM_SCALER::_0010 => 2,
            LPM_SCALER::_0011 => 3,
            LPM_SCALER::_0100 => 4,
            LPM_SCALER::_0101 => 5,
            LPM_SCALER::_0110 => 6,
            LPM_SCALER::_0111 => 7,
            LPM_SCALER::_1000 => 8,
            LPM_SCALER::_1001 => 9,
            LPM_SCALER::_1010 => 10,
            LPM_SCALER::_1011 => 11,
            LPM_SCALER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPM_SCALER {
        match value {
            0 => LPM_SCALER::_0000,
            1 => LPM_SCALER::_0001,
            2 => LPM_SCALER::_0010,
            3 => LPM_SCALER::_0011,
            4 => LPM_SCALER::_0100,
            5 => LPM_SCALER::_0101,
            6 => LPM_SCALER::_0110,
            7 => LPM_SCALER::_0111,
            8 => LPM_SCALER::_1000,
            9 => LPM_SCALER::_1001,
            10 => LPM_SCALER::_1010,
            11 => LPM_SCALER::_1011,
            i => LPM_SCALER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == LPM_SCALER::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == LPM_SCALER::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == LPM_SCALER::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == LPM_SCALER::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == LPM_SCALER::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == LPM_SCALER::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == LPM_SCALER::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == LPM_SCALER::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == LPM_SCALER::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == LPM_SCALER::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == LPM_SCALER::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == LPM_SCALER::_1011
    }
}
#[doc = r" Value of the field"]
pub struct LPM_SDM_USE_NEGR {
    bits: bool,
}
impl LPM_SDM_USE_NEGR {
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
pub struct _PLL_LD_MANUALW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_LD_MANUALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLL_LD_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_LD_DISABLEW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPFFW<'a> {
    w: &'a mut W,
}
impl<'a> _LPFFW<'a> {
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
#[doc = r" Proxy"]
pub struct _LPM_SDM_INVW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_SDM_INVW<'a> {
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
pub struct _LPM_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_DISABLEW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPM_DTH_SCL`"]
pub enum LPM_DTH_SCLW {
    #[doc = "-128 to 96"]
    _0101,
    #[doc = "-256 to 192"]
    _0110,
    #[doc = "-512 to 384"]
    _0111,
    #[doc = "-1024 to 768"]
    _1000,
    #[doc = "-2048 to 1536"]
    _1001,
    #[doc = "-4096 to 3072"]
    _1010,
    #[doc = "-8192 to 6144"]
    _1011,
}
impl LPM_DTH_SCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPM_DTH_SCLW::_0101 => 5,
            LPM_DTH_SCLW::_0110 => 6,
            LPM_DTH_SCLW::_0111 => 7,
            LPM_DTH_SCLW::_1000 => 8,
            LPM_DTH_SCLW::_1001 => 9,
            LPM_DTH_SCLW::_1010 => 10,
            LPM_DTH_SCLW::_1011 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPM_DTH_SCLW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_DTH_SCLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPM_DTH_SCLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "-128 to 96"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(LPM_DTH_SCLW::_0101)
    }
    #[doc = "-256 to 192"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(LPM_DTH_SCLW::_0110)
    }
    #[doc = "-512 to 384"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(LPM_DTH_SCLW::_0111)
    }
    #[doc = "-1024 to 768"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(LPM_DTH_SCLW::_1000)
    }
    #[doc = "-2048 to 1536"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(LPM_DTH_SCLW::_1001)
    }
    #[doc = "-4096 to 3072"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(LPM_DTH_SCLW::_1010)
    }
    #[doc = "-8192 to 6144"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(LPM_DTH_SCLW::_1011)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPM_D_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_D_CTRLW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPM_D_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_D_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `LPM_SCALE`"]
pub enum LPM_SCALEW {
    #[doc = "No Scaling"]
    _0000,
    #[doc = "Multiply by 2"]
    _0001,
    #[doc = "Multiply by 4"]
    _0010,
    #[doc = "Multiply by 8"]
    _0011,
    #[doc = "Multiply by 16"]
    _0100,
    #[doc = "Multiply by 32"]
    _0101,
    #[doc = "Multiply by 64"]
    _0110,
    #[doc = "Multiply by 128"]
    _0111,
    #[doc = "Multiply by 256"]
    _1000,
    #[doc = "Multiply by 512"]
    _1001,
    #[doc = "Multiply by 1024"]
    _1010,
    #[doc = "Multiply by 2048"]
    _1011,
}
impl LPM_SCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPM_SCALEW::_0000 => 0,
            LPM_SCALEW::_0001 => 1,
            LPM_SCALEW::_0010 => 2,
            LPM_SCALEW::_0011 => 3,
            LPM_SCALEW::_0100 => 4,
            LPM_SCALEW::_0101 => 5,
            LPM_SCALEW::_0110 => 6,
            LPM_SCALEW::_0111 => 7,
            LPM_SCALEW::_1000 => 8,
            LPM_SCALEW::_1001 => 9,
            LPM_SCALEW::_1010 => 10,
            LPM_SCALEW::_1011 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPM_SCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_SCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPM_SCALEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Scaling"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(LPM_SCALEW::_0000)
    }
    #[doc = "Multiply by 2"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(LPM_SCALEW::_0001)
    }
    #[doc = "Multiply by 4"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(LPM_SCALEW::_0010)
    }
    #[doc = "Multiply by 8"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(LPM_SCALEW::_0011)
    }
    #[doc = "Multiply by 16"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(LPM_SCALEW::_0100)
    }
    #[doc = "Multiply by 32"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(LPM_SCALEW::_0101)
    }
    #[doc = "Multiply by 64"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(LPM_SCALEW::_0110)
    }
    #[doc = "Multiply by 128"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(LPM_SCALEW::_0111)
    }
    #[doc = "Multiply by 256"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(LPM_SCALEW::_1000)
    }
    #[doc = "Multiply by 512"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(LPM_SCALEW::_1001)
    }
    #[doc = "Multiply by 1024"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(LPM_SCALEW::_1010)
    }
    #[doc = "Multiply by 2048"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(LPM_SCALEW::_1011)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPM_SDM_USE_NEGW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_SDM_USE_NEGW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:5 - Manual PLL Loop Divider value"]
    #[inline]
    pub fn pll_ld_manual(&self) -> PLL_LD_MANUALR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLL_LD_MANUALR { bits }
    }
    #[doc = "Bit 11 - Disable PLL Loop Divider"]
    #[inline]
    pub fn pll_ld_disable(&self) -> PLL_LD_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_LD_DISABLER { bits }
    }
    #[doc = "Bit 13 - LPM SDM Invalid Flag"]
    #[inline]
    pub fn lpff(&self) -> LPFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPFFR { bits }
    }
    #[doc = "Bit 14 - Invert LPM SDM"]
    #[inline]
    pub fn lpm_sdm_inv(&self) -> LPM_SDM_INVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPM_SDM_INVR { bits }
    }
    #[doc = "Bit 15 - Disable LPM SDM"]
    #[inline]
    pub fn lpm_disable(&self) -> LPM_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPM_DISABLER { bits }
    }
    #[doc = "Bits 16:19 - LPM Dither Scale"]
    #[inline]
    pub fn lpm_dth_scl(&self) -> LPM_DTH_SCLR {
        LPM_DTH_SCLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - LPM Dither Control in Override Mode"]
    #[inline]
    pub fn lpm_d_ctrl(&self) -> LPM_D_CTRLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPM_D_CTRLR { bits }
    }
    #[doc = "Bit 23 - LPM Dither Override Mode Select"]
    #[inline]
    pub fn lpm_d_ovrd(&self) -> LPM_D_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPM_D_OVRDR { bits }
    }
    #[doc = "Bits 24:27 - LPM Scale Factor"]
    #[inline]
    pub fn lpm_scale(&self) -> LPM_SCALER {
        LPM_SCALER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Use the Negedge of the Sigma Delta clock"]
    #[inline]
    pub fn lpm_sdm_use_neg(&self) -> LPM_SDM_USE_NEGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPM_SDM_USE_NEGR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 134742016 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Manual PLL Loop Divider value"]
    #[inline]
    pub fn pll_ld_manual(&mut self) -> _PLL_LD_MANUALW {
        _PLL_LD_MANUALW { w: self }
    }
    #[doc = "Bit 11 - Disable PLL Loop Divider"]
    #[inline]
    pub fn pll_ld_disable(&mut self) -> _PLL_LD_DISABLEW {
        _PLL_LD_DISABLEW { w: self }
    }
    #[doc = "Bit 13 - LPM SDM Invalid Flag"]
    #[inline]
    pub fn lpff(&mut self) -> _LPFFW {
        _LPFFW { w: self }
    }
    #[doc = "Bit 14 - Invert LPM SDM"]
    #[inline]
    pub fn lpm_sdm_inv(&mut self) -> _LPM_SDM_INVW {
        _LPM_SDM_INVW { w: self }
    }
    #[doc = "Bit 15 - Disable LPM SDM"]
    #[inline]
    pub fn lpm_disable(&mut self) -> _LPM_DISABLEW {
        _LPM_DISABLEW { w: self }
    }
    #[doc = "Bits 16:19 - LPM Dither Scale"]
    #[inline]
    pub fn lpm_dth_scl(&mut self) -> _LPM_DTH_SCLW {
        _LPM_DTH_SCLW { w: self }
    }
    #[doc = "Bit 22 - LPM Dither Control in Override Mode"]
    #[inline]
    pub fn lpm_d_ctrl(&mut self) -> _LPM_D_CTRLW {
        _LPM_D_CTRLW { w: self }
    }
    #[doc = "Bit 23 - LPM Dither Override Mode Select"]
    #[inline]
    pub fn lpm_d_ovrd(&mut self) -> _LPM_D_OVRDW {
        _LPM_D_OVRDW { w: self }
    }
    #[doc = "Bits 24:27 - LPM Scale Factor"]
    #[inline]
    pub fn lpm_scale(&mut self) -> _LPM_SCALEW {
        _LPM_SCALEW { w: self }
    }
    #[doc = "Bit 31 - Use the Negedge of the Sigma Delta clock"]
    #[inline]
    pub fn lpm_sdm_use_neg(&mut self) -> _LPM_SDM_USE_NEGW {
        _LPM_SDM_USE_NEGW { w: self }
    }
}
