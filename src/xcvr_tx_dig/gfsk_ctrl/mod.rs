#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GFSK_CTRL {
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
pub struct GFSK_MULTIPLY_TABLE_MANUALR {
    bits: u16,
}
impl GFSK_MULTIPLY_TABLE_MANUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `GFSK_MI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFSK_MIR {
    #[doc = "0.32"]
    _00,
    #[doc = "0.50"]
    _01,
    #[doc = "0.70"]
    _10,
    #[doc = "1.00"]
    _11,
}
impl GFSK_MIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GFSK_MIR::_00 => 0,
            GFSK_MIR::_01 => 1,
            GFSK_MIR::_10 => 2,
            GFSK_MIR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GFSK_MIR {
        match value {
            0 => GFSK_MIR::_00,
            1 => GFSK_MIR::_01,
            2 => GFSK_MIR::_10,
            3 => GFSK_MIR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == GFSK_MIR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == GFSK_MIR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == GFSK_MIR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == GFSK_MIR::_11
    }
}
#[doc = r" Value of the field"]
pub struct GFSK_MLDR {
    bits: bool,
}
impl GFSK_MLDR {
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
pub struct GFSK_FLDR {
    bits: bool,
}
impl GFSK_FLDR {
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
#[doc = "Possible values of the field `GFSK_MOD_INDEX_SCALING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFSK_MOD_INDEX_SCALINGR {
    #[doc = "1"]
    _000,
    #[doc = "1 + 1/32"]
    _001,
    #[doc = "1 + 1/16"]
    _010,
    #[doc = "1 + 1/8"]
    _011,
    #[doc = "1 - 1/32"]
    _100,
    #[doc = "1 - 1/16"]
    _101,
    #[doc = "1 - 1/8"]
    _110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GFSK_MOD_INDEX_SCALINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GFSK_MOD_INDEX_SCALINGR::_000 => 0,
            GFSK_MOD_INDEX_SCALINGR::_001 => 1,
            GFSK_MOD_INDEX_SCALINGR::_010 => 2,
            GFSK_MOD_INDEX_SCALINGR::_011 => 3,
            GFSK_MOD_INDEX_SCALINGR::_100 => 4,
            GFSK_MOD_INDEX_SCALINGR::_101 => 5,
            GFSK_MOD_INDEX_SCALINGR::_110 => 6,
            GFSK_MOD_INDEX_SCALINGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GFSK_MOD_INDEX_SCALINGR {
        match value {
            0 => GFSK_MOD_INDEX_SCALINGR::_000,
            1 => GFSK_MOD_INDEX_SCALINGR::_001,
            2 => GFSK_MOD_INDEX_SCALINGR::_010,
            3 => GFSK_MOD_INDEX_SCALINGR::_011,
            4 => GFSK_MOD_INDEX_SCALINGR::_100,
            5 => GFSK_MOD_INDEX_SCALINGR::_101,
            6 => GFSK_MOD_INDEX_SCALINGR::_110,
            i => GFSK_MOD_INDEX_SCALINGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == GFSK_MOD_INDEX_SCALINGR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == GFSK_MOD_INDEX_SCALINGR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == GFSK_MOD_INDEX_SCALINGR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == GFSK_MOD_INDEX_SCALINGR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == GFSK_MOD_INDEX_SCALINGR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == GFSK_MOD_INDEX_SCALINGR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == GFSK_MOD_INDEX_SCALINGR::_110
    }
}
#[doc = r" Value of the field"]
pub struct TX_IMAGE_FILTER_OVRD_ENR {
    bits: bool,
}
impl TX_IMAGE_FILTER_OVRD_ENR {
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
pub struct TX_IMAGE_FILTER_0_OVRDR {
    bits: bool,
}
impl TX_IMAGE_FILTER_0_OVRDR {
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
pub struct TX_IMAGE_FILTER_1_OVRDR {
    bits: bool,
}
impl TX_IMAGE_FILTER_1_OVRDR {
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
pub struct TX_IMAGE_FILTER_2_OVRDR {
    bits: bool,
}
impl TX_IMAGE_FILTER_2_OVRDR {
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
pub struct _GFSK_MULTIPLY_TABLE_MANUALW<'a> {
    w: &'a mut W,
}
impl<'a> _GFSK_MULTIPLY_TABLE_MANUALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GFSK_MI`"]
pub enum GFSK_MIW {
    #[doc = "0.32"]
    _00,
    #[doc = "0.50"]
    _01,
    #[doc = "0.70"]
    _10,
    #[doc = "1.00"]
    _11,
}
impl GFSK_MIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GFSK_MIW::_00 => 0,
            GFSK_MIW::_01 => 1,
            GFSK_MIW::_10 => 2,
            GFSK_MIW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GFSK_MIW<'a> {
    w: &'a mut W,
}
impl<'a> _GFSK_MIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GFSK_MIW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0.32"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(GFSK_MIW::_00)
    }
    #[doc = "0.50"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(GFSK_MIW::_01)
    }
    #[doc = "0.70"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(GFSK_MIW::_10)
    }
    #[doc = "1.00"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(GFSK_MIW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GFSK_MLDW<'a> {
    w: &'a mut W,
}
impl<'a> _GFSK_MLDW<'a> {
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
#[doc = r" Proxy"]
pub struct _GFSK_FLDW<'a> {
    w: &'a mut W,
}
impl<'a> _GFSK_FLDW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GFSK_MOD_INDEX_SCALING`"]
pub enum GFSK_MOD_INDEX_SCALINGW {
    #[doc = "1"]
    _000,
    #[doc = "1 + 1/32"]
    _001,
    #[doc = "1 + 1/16"]
    _010,
    #[doc = "1 + 1/8"]
    _011,
    #[doc = "1 - 1/32"]
    _100,
    #[doc = "1 - 1/16"]
    _101,
    #[doc = "1 - 1/8"]
    _110,
}
impl GFSK_MOD_INDEX_SCALINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GFSK_MOD_INDEX_SCALINGW::_000 => 0,
            GFSK_MOD_INDEX_SCALINGW::_001 => 1,
            GFSK_MOD_INDEX_SCALINGW::_010 => 2,
            GFSK_MOD_INDEX_SCALINGW::_011 => 3,
            GFSK_MOD_INDEX_SCALINGW::_100 => 4,
            GFSK_MOD_INDEX_SCALINGW::_101 => 5,
            GFSK_MOD_INDEX_SCALINGW::_110 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GFSK_MOD_INDEX_SCALINGW<'a> {
    w: &'a mut W,
}
impl<'a> _GFSK_MOD_INDEX_SCALINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GFSK_MOD_INDEX_SCALINGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(GFSK_MOD_INDEX_SCALINGW::_000)
    }
    #[doc = "1 + 1/32"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(GFSK_MOD_INDEX_SCALINGW::_001)
    }
    #[doc = "1 + 1/16"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(GFSK_MOD_INDEX_SCALINGW::_010)
    }
    #[doc = "1 + 1/8"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(GFSK_MOD_INDEX_SCALINGW::_011)
    }
    #[doc = "1 - 1/32"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(GFSK_MOD_INDEX_SCALINGW::_100)
    }
    #[doc = "1 - 1/16"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(GFSK_MOD_INDEX_SCALINGW::_101)
    }
    #[doc = "1 - 1/8"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(GFSK_MOD_INDEX_SCALINGW::_110)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX_IMAGE_FILTER_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_IMAGE_FILTER_OVRD_ENW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX_IMAGE_FILTER_0_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_IMAGE_FILTER_0_OVRDW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX_IMAGE_FILTER_1_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_IMAGE_FILTER_1_OVRDW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX_IMAGE_FILTER_2_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_IMAGE_FILTER_2_OVRDW<'a> {
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
    #[doc = "Bits 0:15 - Manual GFSK Multiply Lookup Table Value"]
    #[inline]
    pub fn gfsk_multiply_table_manual(&self) -> GFSK_MULTIPLY_TABLE_MANUALR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        GFSK_MULTIPLY_TABLE_MANUALR { bits }
    }
    #[doc = "Bits 16:17 - GFSK Modulation Index"]
    #[inline]
    pub fn gfsk_mi(&self) -> GFSK_MIR {
        GFSK_MIR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Disable GFSK Multiply Lookup Table"]
    #[inline]
    pub fn gfsk_mld(&self) -> GFSK_MLDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GFSK_MLDR { bits }
    }
    #[doc = "Bit 21 - Disable GFSK Filter Lookup Table"]
    #[inline]
    pub fn gfsk_fld(&self) -> GFSK_FLDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GFSK_FLDR { bits }
    }
    #[doc = "Bits 24:26 - GFSK Modulation Index Scaling Factor"]
    #[inline]
    pub fn gfsk_mod_index_scaling(&self) -> GFSK_MOD_INDEX_SCALINGR {
        GFSK_MOD_INDEX_SCALINGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - TX Image Filter Override Enable"]
    #[inline]
    pub fn tx_image_filter_ovrd_en(&self) -> TX_IMAGE_FILTER_OVRD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_IMAGE_FILTER_OVRD_ENR { bits }
    }
    #[doc = "Bit 29 - TX Image Filter 0 Override Control"]
    #[inline]
    pub fn tx_image_filter_0_ovrd(&self) -> TX_IMAGE_FILTER_0_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_IMAGE_FILTER_0_OVRDR { bits }
    }
    #[doc = "Bit 30 - TX Image Filter 1 Override Control"]
    #[inline]
    pub fn tx_image_filter_1_ovrd(&self) -> TX_IMAGE_FILTER_1_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_IMAGE_FILTER_1_OVRDR { bits }
    }
    #[doc = "Bit 31 - TX Image Filter 2 Override Control"]
    #[inline]
    pub fn tx_image_filter_2_ovrd(&self) -> TX_IMAGE_FILTER_2_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_IMAGE_FILTER_2_OVRDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 50413568 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Manual GFSK Multiply Lookup Table Value"]
    #[inline]
    pub fn gfsk_multiply_table_manual(&mut self) -> _GFSK_MULTIPLY_TABLE_MANUALW {
        _GFSK_MULTIPLY_TABLE_MANUALW { w: self }
    }
    #[doc = "Bits 16:17 - GFSK Modulation Index"]
    #[inline]
    pub fn gfsk_mi(&mut self) -> _GFSK_MIW {
        _GFSK_MIW { w: self }
    }
    #[doc = "Bit 20 - Disable GFSK Multiply Lookup Table"]
    #[inline]
    pub fn gfsk_mld(&mut self) -> _GFSK_MLDW {
        _GFSK_MLDW { w: self }
    }
    #[doc = "Bit 21 - Disable GFSK Filter Lookup Table"]
    #[inline]
    pub fn gfsk_fld(&mut self) -> _GFSK_FLDW {
        _GFSK_FLDW { w: self }
    }
    #[doc = "Bits 24:26 - GFSK Modulation Index Scaling Factor"]
    #[inline]
    pub fn gfsk_mod_index_scaling(&mut self) -> _GFSK_MOD_INDEX_SCALINGW {
        _GFSK_MOD_INDEX_SCALINGW { w: self }
    }
    #[doc = "Bit 28 - TX Image Filter Override Enable"]
    #[inline]
    pub fn tx_image_filter_ovrd_en(&mut self) -> _TX_IMAGE_FILTER_OVRD_ENW {
        _TX_IMAGE_FILTER_OVRD_ENW { w: self }
    }
    #[doc = "Bit 29 - TX Image Filter 0 Override Control"]
    #[inline]
    pub fn tx_image_filter_0_ovrd(&mut self) -> _TX_IMAGE_FILTER_0_OVRDW {
        _TX_IMAGE_FILTER_0_OVRDW { w: self }
    }
    #[doc = "Bit 30 - TX Image Filter 1 Override Control"]
    #[inline]
    pub fn tx_image_filter_1_ovrd(&mut self) -> _TX_IMAGE_FILTER_1_OVRDW {
        _TX_IMAGE_FILTER_1_OVRDW { w: self }
    }
    #[doc = "Bit 31 - TX Image Filter 2 Override Control"]
    #[inline]
    pub fn tx_image_filter_2_ovrd(&mut self) -> _TX_IMAGE_FILTER_2_OVRDW {
        _TX_IMAGE_FILTER_2_OVRDW { w: self }
    }
}
