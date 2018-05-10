#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FAD_CTRL {
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
#[doc = "Possible values of the field `FAD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAD_ENR {
    #[doc = "Fast Antenna Diversity disabled"]
    _0,
    #[doc = "Fast Antenna Diversity enabled for 802.15.4"]
    _1,
}
impl FAD_ENR {
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
            FAD_ENR::_0 => false,
            FAD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAD_ENR {
        match value {
            false => FAD_ENR::_0,
            true => FAD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct ANTXR {
    bits: bool,
}
impl ANTXR {
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
#[doc = "Possible values of the field `ANTX_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANTX_ENR {
    #[doc = "all disabled (held low)"]
    _00,
    #[doc = "only RX/TX_SWITCH enabled"]
    _01,
    #[doc = "only ANT_A/B enabled"]
    _10,
    #[doc = "all enabled"]
    _11,
}
impl ANTX_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ANTX_ENR::_00 => 0,
            ANTX_ENR::_01 => 1,
            ANTX_ENR::_10 => 2,
            ANTX_ENR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ANTX_ENR {
        match value {
            0 => ANTX_ENR::_00,
            1 => ANTX_ENR::_01,
            2 => ANTX_ENR::_10,
            3 => ANTX_ENR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == ANTX_ENR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == ANTX_ENR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == ANTX_ENR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == ANTX_ENR::_11
    }
}
#[doc = "Possible values of the field `ANTX_HZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANTX_HZR {
    #[doc = "ANT_A, ANT_B, RX_SWITCH and TX_SWITCH are actively driven outputs."]
    _0,
    #[doc = "Antenna controls high impedance- Set ANT_A, ANT_B, RX_SWITCH and TX_SWITCH in high impedance."]
    _1,
}
impl ANTX_HZR {
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
            ANTX_HZR::_0 => false,
            ANTX_HZR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANTX_HZR {
        match value {
            false => ANTX_HZR::_0,
            true => ANTX_HZR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ANTX_HZR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ANTX_HZR::_1
    }
}
#[doc = r" Value of the field"]
pub struct ANTX_CTRLMODER {
    bits: bool,
}
impl ANTX_CTRLMODER {
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
pub struct ANTX_POLR {
    bits: u8,
}
impl ANTX_POLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FAD_NOT_GPIOR {
    bits: u8,
}
impl FAD_NOT_GPIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `FAD_EN`"]
pub enum FAD_ENW {
    #[doc = "Fast Antenna Diversity disabled"]
    _0,
    #[doc = "Fast Antenna Diversity enabled for 802.15.4"]
    _1,
}
impl FAD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FAD_ENW::_0 => false,
            FAD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FAD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fast Antenna Diversity disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAD_ENW::_0)
    }
    #[doc = "Fast Antenna Diversity enabled for 802.15.4"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAD_ENW::_1)
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
#[doc = r" Proxy"]
pub struct _ANTXW<'a> {
    w: &'a mut W,
}
impl<'a> _ANTXW<'a> {
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
#[doc = "Values that can be written to the field `ANTX_EN`"]
pub enum ANTX_ENW {
    #[doc = "all disabled (held low)"]
    _00,
    #[doc = "only RX/TX_SWITCH enabled"]
    _01,
    #[doc = "only ANT_A/B enabled"]
    _10,
    #[doc = "all enabled"]
    _11,
}
impl ANTX_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ANTX_ENW::_00 => 0,
            ANTX_ENW::_01 => 1,
            ANTX_ENW::_10 => 2,
            ANTX_ENW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANTX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ANTX_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANTX_ENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "all disabled (held low)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(ANTX_ENW::_00)
    }
    #[doc = "only RX/TX_SWITCH enabled"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(ANTX_ENW::_01)
    }
    #[doc = "only ANT_A/B enabled"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(ANTX_ENW::_10)
    }
    #[doc = "all enabled"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(ANTX_ENW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ANTX_HZ`"]
pub enum ANTX_HZW {
    #[doc = "ANT_A, ANT_B, RX_SWITCH and TX_SWITCH are actively driven outputs."]
    _0,
    #[doc = "Antenna controls high impedance- Set ANT_A, ANT_B, RX_SWITCH and TX_SWITCH in high impedance."]
    _1,
}
impl ANTX_HZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANTX_HZW::_0 => false,
            ANTX_HZW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANTX_HZW<'a> {
    w: &'a mut W,
}
impl<'a> _ANTX_HZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANTX_HZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ANT_A, ANT_B, RX_SWITCH and TX_SWITCH are actively driven outputs."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANTX_HZW::_0)
    }
    #[doc = "Antenna controls high impedance- Set ANT_A, ANT_B, RX_SWITCH and TX_SWITCH in high impedance."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANTX_HZW::_1)
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
pub struct _ANTX_CTRLMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ANTX_CTRLMODEW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ANTX_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _ANTX_POLW<'a> {
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
pub struct _FAD_NOT_GPIOW<'a> {
    w: &'a mut W,
}
impl<'a> _FAD_NOT_GPIOW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Fast Antenna Diversity Enable"]
    #[inline]
    pub fn fad_en(&self) -> FAD_ENR {
        FAD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Antenna Selection State"]
    #[inline]
    pub fn antx(&self) -> ANTXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ANTXR { bits }
    }
    #[doc = "Bits 4:5 - FAD Antenna Controls Enable"]
    #[inline]
    pub fn antx_en(&self) -> ANTX_ENR {
        ANTX_ENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - FAD PAD Tristate Control"]
    #[inline]
    pub fn antx_hz(&self) -> ANTX_HZR {
        ANTX_HZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Antenna Diversity Control Mode"]
    #[inline]
    pub fn antx_ctrlmode(&self) -> ANTX_CTRLMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ANTX_CTRLMODER { bits }
    }
    #[doc = "Bits 8:11 - FAD Antenna Controls Polarity"]
    #[inline]
    pub fn antx_pol(&self) -> ANTX_POLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ANTX_POLR { bits }
    }
    #[doc = "Bits 12:15 - FAD versus GPIO Mode Selector"]
    #[inline]
    pub fn fad_not_gpio(&self) -> FAD_NOT_GPIOR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FAD_NOT_GPIOR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 61568 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Fast Antenna Diversity Enable"]
    #[inline]
    pub fn fad_en(&mut self) -> _FAD_ENW {
        _FAD_ENW { w: self }
    }
    #[doc = "Bit 1 - Antenna Selection State"]
    #[inline]
    pub fn antx(&mut self) -> _ANTXW {
        _ANTXW { w: self }
    }
    #[doc = "Bits 4:5 - FAD Antenna Controls Enable"]
    #[inline]
    pub fn antx_en(&mut self) -> _ANTX_ENW {
        _ANTX_ENW { w: self }
    }
    #[doc = "Bit 6 - FAD PAD Tristate Control"]
    #[inline]
    pub fn antx_hz(&mut self) -> _ANTX_HZW {
        _ANTX_HZW { w: self }
    }
    #[doc = "Bit 7 - Antenna Diversity Control Mode"]
    #[inline]
    pub fn antx_ctrlmode(&mut self) -> _ANTX_CTRLMODEW {
        _ANTX_CTRLMODEW { w: self }
    }
    #[doc = "Bits 8:11 - FAD Antenna Controls Polarity"]
    #[inline]
    pub fn antx_pol(&mut self) -> _ANTX_POLW {
        _ANTX_POLW { w: self }
    }
    #[doc = "Bits 12:15 - FAD versus GPIO Mode Selector"]
    #[inline]
    pub fn fad_not_gpio(&mut self) -> _FAD_NOT_GPIOW {
        _FAD_NOT_GPIOW { w: self }
    }
}
