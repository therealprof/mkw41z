#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPM_CTRL {
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
pub struct HPM_SDM_IN_MANUALR {
    bits: u16,
}
impl HPM_SDM_IN_MANUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPFFR {
    bits: bool,
}
impl HPFFR {
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
pub struct HPM_SDM_OUT_INVERTR {
    bits: bool,
}
impl HPM_SDM_OUT_INVERTR {
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
pub struct HPM_SDM_IN_DISABLER {
    bits: bool,
}
impl HPM_SDM_IN_DISABLER {
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
#[doc = "Possible values of the field `HPM_LFSR_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPM_LFSR_SIZER {
    #[doc = "LFSR 9, tap mask 100010000"]
    _000,
    #[doc = "LFSR 10, tap mask 1001000000"]
    _001,
    #[doc = "LFSR 11, tap mask 11101000000"]
    _010,
    #[doc = "LFSR 13, tap mask 1101100000000"]
    _011,
    #[doc = "LFSR 15, tap mask 111010000000000"]
    _100,
    #[doc = "LFSR 17, tap mask 11110000000000000"]
    _101,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HPM_LFSR_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HPM_LFSR_SIZER::_000 => 0,
            HPM_LFSR_SIZER::_001 => 1,
            HPM_LFSR_SIZER::_010 => 2,
            HPM_LFSR_SIZER::_011 => 3,
            HPM_LFSR_SIZER::_100 => 4,
            HPM_LFSR_SIZER::_101 => 5,
            HPM_LFSR_SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HPM_LFSR_SIZER {
        match value {
            0 => HPM_LFSR_SIZER::_000,
            1 => HPM_LFSR_SIZER::_001,
            2 => HPM_LFSR_SIZER::_010,
            3 => HPM_LFSR_SIZER::_011,
            4 => HPM_LFSR_SIZER::_100,
            5 => HPM_LFSR_SIZER::_101,
            i => HPM_LFSR_SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == HPM_LFSR_SIZER::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == HPM_LFSR_SIZER::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == HPM_LFSR_SIZER::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == HPM_LFSR_SIZER::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == HPM_LFSR_SIZER::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == HPM_LFSR_SIZER::_101
    }
}
#[doc = r" Value of the field"]
pub struct HPM_DTH_SCLR {
    bits: bool,
}
impl HPM_DTH_SCLR {
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
pub struct HPM_DTH_ENR {
    bits: bool,
}
impl HPM_DTH_ENR {
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
#[doc = "Possible values of the field `HPM_INTEGER_SCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPM_INTEGER_SCALER {
    #[doc = "No Scaling"]
    _00,
    #[doc = "Multiply by 2"]
    _01,
    #[doc = "Divide by 2"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HPM_INTEGER_SCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HPM_INTEGER_SCALER::_00 => 0,
            HPM_INTEGER_SCALER::_01 => 1,
            HPM_INTEGER_SCALER::_10 => 2,
            HPM_INTEGER_SCALER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HPM_INTEGER_SCALER {
        match value {
            0 => HPM_INTEGER_SCALER::_00,
            1 => HPM_INTEGER_SCALER::_01,
            2 => HPM_INTEGER_SCALER::_10,
            i => HPM_INTEGER_SCALER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == HPM_INTEGER_SCALER::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == HPM_INTEGER_SCALER::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == HPM_INTEGER_SCALER::_10
    }
}
#[doc = r" Value of the field"]
pub struct HPM_INTEGER_INVERTR {
    bits: bool,
}
impl HPM_INTEGER_INVERTR {
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
pub struct HPM_CAL_INVERTR {
    bits: bool,
}
impl HPM_CAL_INVERTR {
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
pub struct HPM_MOD_IN_INVERTR {
    bits: bool,
}
impl HPM_MOD_IN_INVERTR {
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
pub struct _HPM_SDM_IN_MANUALW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_SDM_IN_MANUALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPFFW<'a> {
    w: &'a mut W,
}
impl<'a> _HPFFW<'a> {
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
pub struct _HPM_SDM_OUT_INVERTW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_SDM_OUT_INVERTW<'a> {
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
pub struct _HPM_SDM_IN_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_SDM_IN_DISABLEW<'a> {
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
#[doc = "Values that can be written to the field `HPM_LFSR_SIZE`"]
pub enum HPM_LFSR_SIZEW {
    #[doc = "LFSR 9, tap mask 100010000"]
    _000,
    #[doc = "LFSR 10, tap mask 1001000000"]
    _001,
    #[doc = "LFSR 11, tap mask 11101000000"]
    _010,
    #[doc = "LFSR 13, tap mask 1101100000000"]
    _011,
    #[doc = "LFSR 15, tap mask 111010000000000"]
    _100,
    #[doc = "LFSR 17, tap mask 11110000000000000"]
    _101,
}
impl HPM_LFSR_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HPM_LFSR_SIZEW::_000 => 0,
            HPM_LFSR_SIZEW::_001 => 1,
            HPM_LFSR_SIZEW::_010 => 2,
            HPM_LFSR_SIZEW::_011 => 3,
            HPM_LFSR_SIZEW::_100 => 4,
            HPM_LFSR_SIZEW::_101 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPM_LFSR_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_LFSR_SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPM_LFSR_SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LFSR 9, tap mask 100010000"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(HPM_LFSR_SIZEW::_000)
    }
    #[doc = "LFSR 10, tap mask 1001000000"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(HPM_LFSR_SIZEW::_001)
    }
    #[doc = "LFSR 11, tap mask 11101000000"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(HPM_LFSR_SIZEW::_010)
    }
    #[doc = "LFSR 13, tap mask 1101100000000"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(HPM_LFSR_SIZEW::_011)
    }
    #[doc = "LFSR 15, tap mask 111010000000000"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(HPM_LFSR_SIZEW::_100)
    }
    #[doc = "LFSR 17, tap mask 11110000000000000"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(HPM_LFSR_SIZEW::_101)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_DTH_SCLW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_DTH_SCLW<'a> {
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
pub struct _HPM_DTH_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_DTH_ENW<'a> {
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
#[doc = "Values that can be written to the field `HPM_INTEGER_SCALE`"]
pub enum HPM_INTEGER_SCALEW {
    #[doc = "No Scaling"]
    _00,
    #[doc = "Multiply by 2"]
    _01,
    #[doc = "Divide by 2"]
    _10,
}
impl HPM_INTEGER_SCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HPM_INTEGER_SCALEW::_00 => 0,
            HPM_INTEGER_SCALEW::_01 => 1,
            HPM_INTEGER_SCALEW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPM_INTEGER_SCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_INTEGER_SCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPM_INTEGER_SCALEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Scaling"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(HPM_INTEGER_SCALEW::_00)
    }
    #[doc = "Multiply by 2"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(HPM_INTEGER_SCALEW::_01)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(HPM_INTEGER_SCALEW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_INTEGER_INVERTW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_INTEGER_INVERTW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_CAL_INVERTW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_CAL_INVERTW<'a> {
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
pub struct _HPM_MOD_IN_INVERTW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_MOD_IN_INVERTW<'a> {
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
    #[doc = "Bits 0:9 - Manual High Port SDM Fractional value"]
    #[inline]
    pub fn hpm_sdm_in_manual(&self) -> HPM_SDM_IN_MANUALR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HPM_SDM_IN_MANUALR { bits }
    }
    #[doc = "Bit 13 - HPM SDM Invalid Flag"]
    #[inline]
    pub fn hpff(&self) -> HPFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPFFR { bits }
    }
    #[doc = "Bit 14 - Invert HPM SDM Output"]
    #[inline]
    pub fn hpm_sdm_out_invert(&self) -> HPM_SDM_OUT_INVERTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_SDM_OUT_INVERTR { bits }
    }
    #[doc = "Bit 15 - Disable HPM SDM Input"]
    #[inline]
    pub fn hpm_sdm_in_disable(&self) -> HPM_SDM_IN_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_SDM_IN_DISABLER { bits }
    }
    #[doc = "Bits 16:18 - HPM LFSR Length"]
    #[inline]
    pub fn hpm_lfsr_size(&self) -> HPM_LFSR_SIZER {
        HPM_LFSR_SIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - HPM Dither Scale"]
    #[inline]
    pub fn hpm_dth_scl(&self) -> HPM_DTH_SCLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_DTH_SCLR { bits }
    }
    #[doc = "Bit 23 - Dither Enable for HPM LFSR"]
    #[inline]
    pub fn hpm_dth_en(&self) -> HPM_DTH_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_DTH_ENR { bits }
    }
    #[doc = "Bits 24:25 - High Port Modulation Integer Scale"]
    #[inline]
    pub fn hpm_integer_scale(&self) -> HPM_INTEGER_SCALER {
        HPM_INTEGER_SCALER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - Invert High Port Modulation Integer"]
    #[inline]
    pub fn hpm_integer_invert(&self) -> HPM_INTEGER_INVERTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_INTEGER_INVERTR { bits }
    }
    #[doc = "Bit 28 - Invert High Port Modulator Calibration"]
    #[inline]
    pub fn hpm_cal_invert(&self) -> HPM_CAL_INVERTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_CAL_INVERTR { bits }
    }
    #[doc = "Bit 31 - Invert High Port Modulation"]
    #[inline]
    pub fn hpm_mod_in_invert(&self) -> HPM_MOD_IN_INVERTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_MOD_IN_INVERTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2424569856 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - Manual High Port SDM Fractional value"]
    #[inline]
    pub fn hpm_sdm_in_manual(&mut self) -> _HPM_SDM_IN_MANUALW {
        _HPM_SDM_IN_MANUALW { w: self }
    }
    #[doc = "Bit 13 - HPM SDM Invalid Flag"]
    #[inline]
    pub fn hpff(&mut self) -> _HPFFW {
        _HPFFW { w: self }
    }
    #[doc = "Bit 14 - Invert HPM SDM Output"]
    #[inline]
    pub fn hpm_sdm_out_invert(&mut self) -> _HPM_SDM_OUT_INVERTW {
        _HPM_SDM_OUT_INVERTW { w: self }
    }
    #[doc = "Bit 15 - Disable HPM SDM Input"]
    #[inline]
    pub fn hpm_sdm_in_disable(&mut self) -> _HPM_SDM_IN_DISABLEW {
        _HPM_SDM_IN_DISABLEW { w: self }
    }
    #[doc = "Bits 16:18 - HPM LFSR Length"]
    #[inline]
    pub fn hpm_lfsr_size(&mut self) -> _HPM_LFSR_SIZEW {
        _HPM_LFSR_SIZEW { w: self }
    }
    #[doc = "Bit 20 - HPM Dither Scale"]
    #[inline]
    pub fn hpm_dth_scl(&mut self) -> _HPM_DTH_SCLW {
        _HPM_DTH_SCLW { w: self }
    }
    #[doc = "Bit 23 - Dither Enable for HPM LFSR"]
    #[inline]
    pub fn hpm_dth_en(&mut self) -> _HPM_DTH_ENW {
        _HPM_DTH_ENW { w: self }
    }
    #[doc = "Bits 24:25 - High Port Modulation Integer Scale"]
    #[inline]
    pub fn hpm_integer_scale(&mut self) -> _HPM_INTEGER_SCALEW {
        _HPM_INTEGER_SCALEW { w: self }
    }
    #[doc = "Bit 27 - Invert High Port Modulation Integer"]
    #[inline]
    pub fn hpm_integer_invert(&mut self) -> _HPM_INTEGER_INVERTW {
        _HPM_INTEGER_INVERTW { w: self }
    }
    #[doc = "Bit 28 - Invert High Port Modulator Calibration"]
    #[inline]
    pub fn hpm_cal_invert(&mut self) -> _HPM_CAL_INVERTW {
        _HPM_CAL_INVERTW { w: self }
    }
    #[doc = "Bit 31 - Invert High Port Modulation"]
    #[inline]
    pub fn hpm_mod_in_invert(&mut self) -> _HPM_MOD_IN_INVERTW {
        _HPM_MOD_IN_INVERTW { w: self }
    }
}
