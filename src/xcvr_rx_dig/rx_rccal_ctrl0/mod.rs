#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_RCCAL_CTRL0 {
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
pub struct BBA_RCCAL_OFFSETR {
    bits: u8,
}
impl BBA_RCCAL_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA_RCCAL_MANUALR {
    bits: u8,
}
impl BBA_RCCAL_MANUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BBA_RCCAL_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBA_RCCAL_DISR {
    #[doc = "BBA RC Calibration is enabled"]
    _0,
    #[doc = "BBA RC Calibration is disabled"]
    _1,
}
impl BBA_RCCAL_DISR {
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
            BBA_RCCAL_DISR::_0 => false,
            BBA_RCCAL_DISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BBA_RCCAL_DISR {
        match value {
            false => BBA_RCCAL_DISR::_0,
            true => BBA_RCCAL_DISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BBA_RCCAL_DISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BBA_RCCAL_DISR::_1
    }
}
#[doc = "Possible values of the field `RCCAL_SMP_DLY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCCAL_SMP_DLYR {
    #[doc = "The comp_out signal is sampled 0 clk cycle after sample signal is deasserted"]
    _00,
    #[doc = "The comp_out signal is sampled 1 clk cycle after sample signal is deasserted"]
    _01,
    #[doc = "The comp_out signal is sampled 2 clk cycle after sample signal is deasserted"]
    _10,
    #[doc = "The comp_out signal is sampled 3 clk cycle after sample signal is deasserted"]
    _11,
}
impl RCCAL_SMP_DLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RCCAL_SMP_DLYR::_00 => 0,
            RCCAL_SMP_DLYR::_01 => 1,
            RCCAL_SMP_DLYR::_10 => 2,
            RCCAL_SMP_DLYR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RCCAL_SMP_DLYR {
        match value {
            0 => RCCAL_SMP_DLYR::_00,
            1 => RCCAL_SMP_DLYR::_01,
            2 => RCCAL_SMP_DLYR::_10,
            3 => RCCAL_SMP_DLYR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RCCAL_SMP_DLYR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RCCAL_SMP_DLYR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RCCAL_SMP_DLYR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == RCCAL_SMP_DLYR::_11
    }
}
#[doc = "Possible values of the field `RCCAL_COMP_INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCCAL_COMP_INVR {
    #[doc = "The comp_out signal polarity is NOT inverted"]
    _0,
    #[doc = "The comp_out signal polarity is inverted"]
    _1,
}
impl RCCAL_COMP_INVR {
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
            RCCAL_COMP_INVR::_0 => false,
            RCCAL_COMP_INVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCCAL_COMP_INVR {
        match value {
            false => RCCAL_COMP_INVR::_0,
            true => RCCAL_COMP_INVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCCAL_COMP_INVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCCAL_COMP_INVR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TZA_RCCAL_OFFSETR {
    bits: u8,
}
impl TZA_RCCAL_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TZA_RCCAL_MANUALR {
    bits: u8,
}
impl TZA_RCCAL_MANUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TZA_RCCAL_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZA_RCCAL_DISR {
    #[doc = "TZA RC Calibration is enabled"]
    _0,
    #[doc = "TZA RC Calibration is disabled"]
    _1,
}
impl TZA_RCCAL_DISR {
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
            TZA_RCCAL_DISR::_0 => false,
            TZA_RCCAL_DISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TZA_RCCAL_DISR {
        match value {
            false => TZA_RCCAL_DISR::_0,
            true => TZA_RCCAL_DISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TZA_RCCAL_DISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TZA_RCCAL_DISR::_1
    }
}
#[doc = r" Proxy"]
pub struct _BBA_RCCAL_OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_RCCAL_OFFSETW<'a> {
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
pub struct _BBA_RCCAL_MANUALW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_RCCAL_MANUALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BBA_RCCAL_DIS`"]
pub enum BBA_RCCAL_DISW {
    #[doc = "BBA RC Calibration is enabled"]
    _0,
    #[doc = "BBA RC Calibration is disabled"]
    _1,
}
impl BBA_RCCAL_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BBA_RCCAL_DISW::_0 => false,
            BBA_RCCAL_DISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BBA_RCCAL_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA_RCCAL_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BBA_RCCAL_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BBA RC Calibration is enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BBA_RCCAL_DISW::_0)
    }
    #[doc = "BBA RC Calibration is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BBA_RCCAL_DISW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCCAL_SMP_DLY`"]
pub enum RCCAL_SMP_DLYW {
    #[doc = "The comp_out signal is sampled 0 clk cycle after sample signal is deasserted"]
    _00,
    #[doc = "The comp_out signal is sampled 1 clk cycle after sample signal is deasserted"]
    _01,
    #[doc = "The comp_out signal is sampled 2 clk cycle after sample signal is deasserted"]
    _10,
    #[doc = "The comp_out signal is sampled 3 clk cycle after sample signal is deasserted"]
    _11,
}
impl RCCAL_SMP_DLYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RCCAL_SMP_DLYW::_00 => 0,
            RCCAL_SMP_DLYW::_01 => 1,
            RCCAL_SMP_DLYW::_10 => 2,
            RCCAL_SMP_DLYW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCCAL_SMP_DLYW<'a> {
    w: &'a mut W,
}
impl<'a> _RCCAL_SMP_DLYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCCAL_SMP_DLYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The comp_out signal is sampled 0 clk cycle after sample signal is deasserted"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RCCAL_SMP_DLYW::_00)
    }
    #[doc = "The comp_out signal is sampled 1 clk cycle after sample signal is deasserted"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RCCAL_SMP_DLYW::_01)
    }
    #[doc = "The comp_out signal is sampled 2 clk cycle after sample signal is deasserted"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RCCAL_SMP_DLYW::_10)
    }
    #[doc = "The comp_out signal is sampled 3 clk cycle after sample signal is deasserted"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RCCAL_SMP_DLYW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCCAL_COMP_INV`"]
pub enum RCCAL_COMP_INVW {
    #[doc = "The comp_out signal polarity is NOT inverted"]
    _0,
    #[doc = "The comp_out signal polarity is inverted"]
    _1,
}
impl RCCAL_COMP_INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCCAL_COMP_INVW::_0 => false,
            RCCAL_COMP_INVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCCAL_COMP_INVW<'a> {
    w: &'a mut W,
}
impl<'a> _RCCAL_COMP_INVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCCAL_COMP_INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The comp_out signal polarity is NOT inverted"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCCAL_COMP_INVW::_0)
    }
    #[doc = "The comp_out signal polarity is inverted"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCCAL_COMP_INVW::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TZA_RCCAL_OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _TZA_RCCAL_OFFSETW<'a> {
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
pub struct _TZA_RCCAL_MANUALW<'a> {
    w: &'a mut W,
}
impl<'a> _TZA_RCCAL_MANUALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TZA_RCCAL_DIS`"]
pub enum TZA_RCCAL_DISW {
    #[doc = "TZA RC Calibration is enabled"]
    _0,
    #[doc = "TZA RC Calibration is disabled"]
    _1,
}
impl TZA_RCCAL_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TZA_RCCAL_DISW::_0 => false,
            TZA_RCCAL_DISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TZA_RCCAL_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _TZA_RCCAL_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TZA_RCCAL_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TZA RC Calibration is enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TZA_RCCAL_DISW::_0)
    }
    #[doc = "TZA RC Calibration is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TZA_RCCAL_DISW::_1)
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
        const OFFSET: u8 = 25;
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
    #[doc = "Bits 0:3 - BBA RC Calibration value offset"]
    #[inline]
    pub fn bba_rccal_offset(&self) -> BBA_RCCAL_OFFSETR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_RCCAL_OFFSETR { bits }
    }
    #[doc = "Bits 4:8 - BBA RC Calibration manual value"]
    #[inline]
    pub fn bba_rccal_manual(&self) -> BBA_RCCAL_MANUALR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA_RCCAL_MANUALR { bits }
    }
    #[doc = "Bit 9 - BBA RC Calibration Disable"]
    #[inline]
    pub fn bba_rccal_dis(&self) -> BBA_RCCAL_DISR {
        BBA_RCCAL_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - RC Calibration Sample Delay"]
    #[inline]
    pub fn rccal_smp_dly(&self) -> RCCAL_SMP_DLYR {
        RCCAL_SMP_DLYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - RC Calibration comp_out Invert"]
    #[inline]
    pub fn rccal_comp_inv(&self) -> RCCAL_COMP_INVR {
        RCCAL_COMP_INVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - TZA RC Calibration value offset"]
    #[inline]
    pub fn tza_rccal_offset(&self) -> TZA_RCCAL_OFFSETR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TZA_RCCAL_OFFSETR { bits }
    }
    #[doc = "Bits 20:24 - TZA RC Calibration manual value"]
    #[inline]
    pub fn tza_rccal_manual(&self) -> TZA_RCCAL_MANUALR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TZA_RCCAL_MANUALR { bits }
    }
    #[doc = "Bit 25 - TZA RC Calibration Disable"]
    #[inline]
    pub fn tza_rccal_dis(&self) -> TZA_RCCAL_DISR {
        TZA_RCCAL_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bits 0:3 - BBA RC Calibration value offset"]
    #[inline]
    pub fn bba_rccal_offset(&mut self) -> _BBA_RCCAL_OFFSETW {
        _BBA_RCCAL_OFFSETW { w: self }
    }
    #[doc = "Bits 4:8 - BBA RC Calibration manual value"]
    #[inline]
    pub fn bba_rccal_manual(&mut self) -> _BBA_RCCAL_MANUALW {
        _BBA_RCCAL_MANUALW { w: self }
    }
    #[doc = "Bit 9 - BBA RC Calibration Disable"]
    #[inline]
    pub fn bba_rccal_dis(&mut self) -> _BBA_RCCAL_DISW {
        _BBA_RCCAL_DISW { w: self }
    }
    #[doc = "Bits 12:13 - RC Calibration Sample Delay"]
    #[inline]
    pub fn rccal_smp_dly(&mut self) -> _RCCAL_SMP_DLYW {
        _RCCAL_SMP_DLYW { w: self }
    }
    #[doc = "Bit 15 - RC Calibration comp_out Invert"]
    #[inline]
    pub fn rccal_comp_inv(&mut self) -> _RCCAL_COMP_INVW {
        _RCCAL_COMP_INVW { w: self }
    }
    #[doc = "Bits 16:19 - TZA RC Calibration value offset"]
    #[inline]
    pub fn tza_rccal_offset(&mut self) -> _TZA_RCCAL_OFFSETW {
        _TZA_RCCAL_OFFSETW { w: self }
    }
    #[doc = "Bits 20:24 - TZA RC Calibration manual value"]
    #[inline]
    pub fn tza_rccal_manual(&mut self) -> _TZA_RCCAL_MANUALW {
        _TZA_RCCAL_MANUALW { w: self }
    }
    #[doc = "Bit 25 - TZA RC Calibration Disable"]
    #[inline]
    pub fn tza_rccal_dis(&mut self) -> _TZA_RCCAL_DISW {
        _TZA_RCCAL_DISW { w: self }
    }
}
