#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_ADC {
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
pub struct RX_ADC_BUMPR {
    bits: u8,
}
impl RX_ADC_BUMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RX_ADC_FS_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ADC_FS_SELR {
    #[doc = "52MHz (2x26MHz)"]
    _0,
    #[doc = "64MHz (2x32MHz)"]
    _1,
    #[doc = "+13% of 64MHz"]
    _2,
    #[doc = "- 11% of 64MHz"]
    _3,
}
impl RX_ADC_FS_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_ADC_FS_SELR::_0 => 0,
            RX_ADC_FS_SELR::_1 => 1,
            RX_ADC_FS_SELR::_2 => 2,
            RX_ADC_FS_SELR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_ADC_FS_SELR {
        match value {
            0 => RX_ADC_FS_SELR::_0,
            1 => RX_ADC_FS_SELR::_1,
            2 => RX_ADC_FS_SELR::_2,
            3 => RX_ADC_FS_SELR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_ADC_FS_SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_ADC_FS_SELR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == RX_ADC_FS_SELR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == RX_ADC_FS_SELR::_3
    }
}
#[doc = r" Value of the field"]
pub struct RX_ADC_I_DIAGSELR {
    bits: bool,
}
impl RX_ADC_I_DIAGSELR {
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
pub struct RX_ADC_Q_DIAGSELR {
    bits: bool,
}
impl RX_ADC_Q_DIAGSELR {
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
pub struct RX_ADC_SPARER {
    bits: u8,
}
impl RX_ADC_SPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RX_ADC_BUMPW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_BUMPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_ADC_FS_SEL`"]
pub enum RX_ADC_FS_SELW {
    #[doc = "52MHz (2x26MHz)"]
    _0,
    #[doc = "64MHz (2x32MHz)"]
    _1,
    #[doc = "+13% of 64MHz"]
    _2,
    #[doc = "- 11% of 64MHz"]
    _3,
}
impl RX_ADC_FS_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_ADC_FS_SELW::_0 => 0,
            RX_ADC_FS_SELW::_1 => 1,
            RX_ADC_FS_SELW::_2 => 2,
            RX_ADC_FS_SELW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_ADC_FS_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_FS_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_ADC_FS_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "52MHz (2x26MHz)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_ADC_FS_SELW::_0)
    }
    #[doc = "64MHz (2x32MHz)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_ADC_FS_SELW::_1)
    }
    #[doc = "+13% of 64MHz"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(RX_ADC_FS_SELW::_2)
    }
    #[doc = "- 11% of 64MHz"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(RX_ADC_FS_SELW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_ADC_I_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_I_DIAGSELW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_ADC_Q_DIAGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_Q_DIAGSELW<'a> {
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
pub struct _RX_ADC_SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ADC_SPAREW<'a> {
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
    #[doc = "Bits 0:7 - rmap_rx_adc_bump[7:0]"]
    #[inline]
    pub fn rx_adc_bump(&self) -> RX_ADC_BUMPR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_ADC_BUMPR { bits }
    }
    #[doc = "Bits 8:9 - rmap_rx_adc_fs_sel[1:0]"]
    #[inline]
    pub fn rx_adc_fs_sel(&self) -> RX_ADC_FS_SELR {
        RX_ADC_FS_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - rmap_rx_adc_i_diagsel"]
    #[inline]
    pub fn rx_adc_i_diagsel(&self) -> RX_ADC_I_DIAGSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_ADC_I_DIAGSELR { bits }
    }
    #[doc = "Bit 11 - rmap_rx_adc_q_diagsel"]
    #[inline]
    pub fn rx_adc_q_diagsel(&self) -> RX_ADC_Q_DIAGSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_ADC_Q_DIAGSELR { bits }
    }
    #[doc = "Bits 12:15 - rmap_rx_adc_spare[3:0]"]
    #[inline]
    pub fn rx_adc_spare(&self) -> RX_ADC_SPARER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_ADC_SPARER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 320 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - rmap_rx_adc_bump[7:0]"]
    #[inline]
    pub fn rx_adc_bump(&mut self) -> _RX_ADC_BUMPW {
        _RX_ADC_BUMPW { w: self }
    }
    #[doc = "Bits 8:9 - rmap_rx_adc_fs_sel[1:0]"]
    #[inline]
    pub fn rx_adc_fs_sel(&mut self) -> _RX_ADC_FS_SELW {
        _RX_ADC_FS_SELW { w: self }
    }
    #[doc = "Bit 10 - rmap_rx_adc_i_diagsel"]
    #[inline]
    pub fn rx_adc_i_diagsel(&mut self) -> _RX_ADC_I_DIAGSELW {
        _RX_ADC_I_DIAGSELW { w: self }
    }
    #[doc = "Bit 11 - rmap_rx_adc_q_diagsel"]
    #[inline]
    pub fn rx_adc_q_diagsel(&mut self) -> _RX_ADC_Q_DIAGSELW {
        _RX_ADC_Q_DIAGSELW { w: self }
    }
    #[doc = "Bits 12:15 - rmap_rx_adc_spare[3:0]"]
    #[inline]
    pub fn rx_adc_spare(&mut self) -> _RX_ADC_SPAREW {
        _RX_ADC_SPAREW { w: self }
    }
}
