#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_LNA {
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
#[doc = "Possible values of the field `RX_LNA_BUMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_LNA_BUMPR {
    #[doc = "Default"]
    _0,
    #[doc = "-25%"]
    _1,
    #[doc = "+50%"]
    _2,
    #[doc = "+25%"]
    _3,
    #[doc = "CM 480mV"]
    _4,
    #[doc = "CM 600mV"]
    _8,
    #[doc = "CM 660mV"]
    _12,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RX_LNA_BUMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_LNA_BUMPR::_0 => 0,
            RX_LNA_BUMPR::_1 => 1,
            RX_LNA_BUMPR::_2 => 2,
            RX_LNA_BUMPR::_3 => 3,
            RX_LNA_BUMPR::_4 => 4,
            RX_LNA_BUMPR::_8 => 8,
            RX_LNA_BUMPR::_12 => 12,
            RX_LNA_BUMPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_LNA_BUMPR {
        match value {
            0 => RX_LNA_BUMPR::_0,
            1 => RX_LNA_BUMPR::_1,
            2 => RX_LNA_BUMPR::_2,
            3 => RX_LNA_BUMPR::_3,
            4 => RX_LNA_BUMPR::_4,
            8 => RX_LNA_BUMPR::_8,
            12 => RX_LNA_BUMPR::_12,
            i => RX_LNA_BUMPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_LNA_BUMPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_LNA_BUMPR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == RX_LNA_BUMPR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == RX_LNA_BUMPR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == RX_LNA_BUMPR::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == RX_LNA_BUMPR::_8
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline]
    pub fn is_12(&self) -> bool {
        *self == RX_LNA_BUMPR::_12
    }
}
#[doc = r" Value of the field"]
pub struct RX_LNA_HG_DIAGSELR {
    bits: bool,
}
impl RX_LNA_HG_DIAGSELR {
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
pub struct RX_LNA_HIZ_ENABLER {
    bits: bool,
}
impl RX_LNA_HIZ_ENABLER {
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
pub struct RX_LNA_LG_DIAGSELR {
    bits: bool,
}
impl RX_LNA_LG_DIAGSELR {
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
pub struct RX_LNA_SPARER {
    bits: u8,
}
impl RX_LNA_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RX_MIXER_BUMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_MIXER_BUMPR {
    #[doc = "825mV (Default)"]
    _0,
    #[doc = "750mV"]
    _1,
    #[doc = "900mV"]
    _2,
    #[doc = "975mV"]
    _3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RX_MIXER_BUMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_MIXER_BUMPR::_0 => 0,
            RX_MIXER_BUMPR::_1 => 1,
            RX_MIXER_BUMPR::_2 => 2,
            RX_MIXER_BUMPR::_3 => 3,
            RX_MIXER_BUMPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_MIXER_BUMPR {
        match value {
            0 => RX_MIXER_BUMPR::_0,
            1 => RX_MIXER_BUMPR::_1,
            2 => RX_MIXER_BUMPR::_2,
            3 => RX_MIXER_BUMPR::_3,
            i => RX_MIXER_BUMPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_MIXER_BUMPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_MIXER_BUMPR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == RX_MIXER_BUMPR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == RX_MIXER_BUMPR::_3
    }
}
#[doc = r" Value of the field"]
pub struct RX_MIXER_SPARER {
    bits: bool,
}
impl RX_MIXER_SPARER {
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
#[doc = "Values that can be written to the field `RX_LNA_BUMP`"]
pub enum RX_LNA_BUMPW {
    #[doc = "Default"]
    _0,
    #[doc = "-25%"]
    _1,
    #[doc = "+50%"]
    _2,
    #[doc = "+25%"]
    _3,
    #[doc = "CM 480mV"]
    _4,
    #[doc = "CM 600mV"]
    _8,
    #[doc = "CM 660mV"]
    _12,
}
impl RX_LNA_BUMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_LNA_BUMPW::_0 => 0,
            RX_LNA_BUMPW::_1 => 1,
            RX_LNA_BUMPW::_2 => 2,
            RX_LNA_BUMPW::_3 => 3,
            RX_LNA_BUMPW::_4 => 4,
            RX_LNA_BUMPW::_8 => 8,
            RX_LNA_BUMPW::_12 => 12,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_LNA_BUMPW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_LNA_BUMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_LNA_BUMPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Default"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_LNA_BUMPW::_0)
    }
    #[doc = "-25%"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_LNA_BUMPW::_1)
    }
    #[doc = "+50%"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(RX_LNA_BUMPW::_2)
    }
    #[doc = "+25%"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(RX_LNA_BUMPW::_3)
    }
    #[doc = "CM 480mV"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(RX_LNA_BUMPW::_4)
    }
    #[doc = "CM 600mV"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(RX_LNA_BUMPW::_8)
    }
    #[doc = "CM 660mV"]
    #[inline]
    pub fn _12(self) -> &'a mut W {
        self.variant(RX_LNA_BUMPW::_12)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_LNA_HG_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_LNA_HG_DIAGSELW<'a> {
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
pub struct _RX_LNA_HIZ_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_LNA_HIZ_ENABLEW<'a> {
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
pub struct _RX_LNA_LG_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_LNA_LG_DIAGSELW<'a> {
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
pub struct _RX_LNA_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_LNA_SPAREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_MIXER_BUMP`"]
pub enum RX_MIXER_BUMPW {
    #[doc = "825mV (Default)"]
    _0,
    #[doc = "750mV"]
    _1,
    #[doc = "900mV"]
    _2,
    #[doc = "975mV"]
    _3,
}
impl RX_MIXER_BUMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_MIXER_BUMPW::_0 => 0,
            RX_MIXER_BUMPW::_1 => 1,
            RX_MIXER_BUMPW::_2 => 2,
            RX_MIXER_BUMPW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_MIXER_BUMPW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_MIXER_BUMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_MIXER_BUMPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "825mV (Default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_MIXER_BUMPW::_0)
    }
    #[doc = "750mV"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_MIXER_BUMPW::_1)
    }
    #[doc = "900mV"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(RX_MIXER_BUMPW::_2)
    }
    #[doc = "975mV"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(RX_MIXER_BUMPW::_3)
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
pub struct _RX_MIXER_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_MIXER_SPAREW<'a> {
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:3 - rmap_rx_lna_bump[3:0]"]
    #[inline]
    pub fn rx_lna_bump(&self) -> RX_LNA_BUMPR {
        RX_LNA_BUMPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - rmap_rx_lna_hg_diagsel"]
    #[inline]
    pub fn rx_lna_hg_diagsel(&self) -> RX_LNA_HG_DIAGSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_LNA_HG_DIAGSELR { bits }
    }
    #[doc = "Bit 5 - rmap_rx_lna_hiZ_enable"]
    #[inline]
    pub fn rx_lna_hiz_enable(&self) -> RX_LNA_HIZ_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_LNA_HIZ_ENABLER { bits }
    }
    #[doc = "Bit 6 - rmap_rx_lna_lg_diagsel"]
    #[inline]
    pub fn rx_lna_lg_diagsel(&self) -> RX_LNA_LG_DIAGSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_LNA_LG_DIAGSELR { bits }
    }
    #[doc = "Bits 8:9 - rmap_rx_lna_spare[1:0]"]
    #[inline]
    pub fn rx_lna_spare(&self) -> RX_LNA_SPARER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_LNA_SPARER { bits }
    }
    #[doc = "Bits 16:19 - rmap_rx_mixer_bump[3:0]"]
    #[inline]
    pub fn rx_mixer_bump(&self) -> RX_MIXER_BUMPR {
        RX_MIXER_BUMPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - rmap_rx_mixer_spare"]
    #[inline]
    pub fn rx_mixer_spare(&self) -> RX_MIXER_SPARER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_MIXER_SPARER { bits }
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
    #[doc = "Bits 0:3 - rmap_rx_lna_bump[3:0]"]
    #[inline]
    pub fn rx_lna_bump(&mut self) -> _RX_LNA_BUMPW {
        _RX_LNA_BUMPW { w: self }
    }
    #[doc = "Bit 4 - rmap_rx_lna_hg_diagsel"]
    #[inline]
    pub fn rx_lna_hg_diagsel(&mut self) -> _RX_LNA_HG_DIAGSELW {
        _RX_LNA_HG_DIAGSELW { w: self }
    }
    #[doc = "Bit 5 - rmap_rx_lna_hiZ_enable"]
    #[inline]
    pub fn rx_lna_hiz_enable(&mut self) -> _RX_LNA_HIZ_ENABLEW {
        _RX_LNA_HIZ_ENABLEW { w: self }
    }
    #[doc = "Bit 6 - rmap_rx_lna_lg_diagsel"]
    #[inline]
    pub fn rx_lna_lg_diagsel(&mut self) -> _RX_LNA_LG_DIAGSELW {
        _RX_LNA_LG_DIAGSELW { w: self }
    }
    #[doc = "Bits 8:9 - rmap_rx_lna_spare[1:0]"]
    #[inline]
    pub fn rx_lna_spare(&mut self) -> _RX_LNA_SPAREW {
        _RX_LNA_SPAREW { w: self }
    }
    #[doc = "Bits 16:19 - rmap_rx_mixer_bump[3:0]"]
    #[inline]
    pub fn rx_mixer_bump(&mut self) -> _RX_MIXER_BUMPW {
        _RX_MIXER_BUMPW { w: self }
    }
    #[doc = "Bit 20 - rmap_rx_mixer_spare"]
    #[inline]
    pub fn rx_mixer_spare(&mut self) -> _RX_MIXER_SPAREW {
        _RX_MIXER_SPAREW { w: self }
    }
}
