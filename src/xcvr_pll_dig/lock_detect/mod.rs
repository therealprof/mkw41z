#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOCK_DETECT {
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
pub struct CT_FAILR {
    bits: bool,
}
impl CT_FAILR {
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
pub struct CTFFR {
    bits: bool,
}
impl CTFFR {
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
pub struct CS_FAILR {
    bits: bool,
}
impl CS_FAILR {
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
pub struct CSFFR {
    bits: bool,
}
impl CSFFR {
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
pub struct FT_FAILR {
    bits: bool,
}
impl FT_FAILR {
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
pub struct FTFFR {
    bits: bool,
}
impl FTFFR {
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
pub struct TAFFR {
    bits: bool,
}
impl TAFFR {
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
pub struct CTUNE_LDF_LEVR {
    bits: u8,
}
impl CTUNE_LDF_LEVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FTF_RX_THRSHR {
    bits: u8,
}
impl FTF_RX_THRSHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FTW_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTW_RXR {
    #[doc = "4 us"]
    _0,
    #[doc = "8 us"]
    _1,
}
impl FTW_RXR {
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
            FTW_RXR::_0 => false,
            FTW_RXR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTW_RXR {
        match value {
            false => FTW_RXR::_0,
            true => FTW_RXR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTW_RXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTW_RXR::_1
    }
}
#[doc = r" Value of the field"]
pub struct FTF_TX_THRSHR {
    bits: u8,
}
impl FTF_TX_THRSHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FTW_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTW_TXR {
    #[doc = "4 us"]
    _0,
    #[doc = "8 us"]
    _1,
}
impl FTW_TXR {
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
            FTW_TXR::_0 => false,
            FTW_TXR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTW_TXR {
        match value {
            false => FTW_TXR::_0,
            true => FTW_TXR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTW_TXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTW_TXR::_1
    }
}
#[doc = r" Value of the field"]
pub struct FREQ_COUNT_GOR {
    bits: bool,
}
impl FREQ_COUNT_GOR {
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
pub struct FREQ_COUNT_FINISHEDR {
    bits: bool,
}
impl FREQ_COUNT_FINISHEDR {
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
#[doc = "Possible values of the field `FREQ_COUNT_TIME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQ_COUNT_TIMER {
    #[doc = "10 us"]
    _00,
    #[doc = "25 us"]
    _01,
    #[doc = "50 us"]
    _10,
    #[doc = "100 us"]
    _11,
}
impl FREQ_COUNT_TIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FREQ_COUNT_TIMER::_00 => 0,
            FREQ_COUNT_TIMER::_01 => 1,
            FREQ_COUNT_TIMER::_10 => 2,
            FREQ_COUNT_TIMER::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FREQ_COUNT_TIMER {
        match value {
            0 => FREQ_COUNT_TIMER::_00,
            1 => FREQ_COUNT_TIMER::_01,
            2 => FREQ_COUNT_TIMER::_10,
            3 => FREQ_COUNT_TIMER::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FREQ_COUNT_TIMER::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FREQ_COUNT_TIMER::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FREQ_COUNT_TIMER::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FREQ_COUNT_TIMER::_11
    }
}
#[doc = r" Proxy"]
pub struct _CTFFW<'a> {
    w: &'a mut W,
}
impl<'a> _CTFFW<'a> {
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
#[doc = r" Proxy"]
pub struct _CSFFW<'a> {
    w: &'a mut W,
}
impl<'a> _CSFFW<'a> {
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
pub struct _FTFFW<'a> {
    w: &'a mut W,
}
impl<'a> _FTFFW<'a> {
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
pub struct _TAFFW<'a> {
    w: &'a mut W,
}
impl<'a> _TAFFW<'a> {
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
pub struct _CTUNE_LDF_LEVW<'a> {
    w: &'a mut W,
}
impl<'a> _CTUNE_LDF_LEVW<'a> {
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
pub struct _FTF_RX_THRSHW<'a> {
    w: &'a mut W,
}
impl<'a> _FTF_RX_THRSHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTW_RX`"]
pub enum FTW_RXW {
    #[doc = "4 us"]
    _0,
    #[doc = "8 us"]
    _1,
}
impl FTW_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTW_RXW::_0 => false,
            FTW_RXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTW_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _FTW_RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTW_RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "4 us"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTW_RXW::_0)
    }
    #[doc = "8 us"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTW_RXW::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FTF_TX_THRSHW<'a> {
    w: &'a mut W,
}
impl<'a> _FTF_TX_THRSHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTW_TX`"]
pub enum FTW_TXW {
    #[doc = "4 us"]
    _0,
    #[doc = "8 us"]
    _1,
}
impl FTW_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTW_TXW::_0 => false,
            FTW_TXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTW_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _FTW_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTW_TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "4 us"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTW_TXW::_0)
    }
    #[doc = "8 us"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTW_TXW::_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FREQ_COUNT_GOW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQ_COUNT_GOW<'a> {
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
#[doc = "Values that can be written to the field `FREQ_COUNT_TIME`"]
pub enum FREQ_COUNT_TIMEW {
    #[doc = "10 us"]
    _00,
    #[doc = "25 us"]
    _01,
    #[doc = "50 us"]
    _10,
    #[doc = "100 us"]
    _11,
}
impl FREQ_COUNT_TIMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FREQ_COUNT_TIMEW::_00 => 0,
            FREQ_COUNT_TIMEW::_01 => 1,
            FREQ_COUNT_TIMEW::_10 => 2,
            FREQ_COUNT_TIMEW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREQ_COUNT_TIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQ_COUNT_TIMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREQ_COUNT_TIMEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "10 us"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FREQ_COUNT_TIMEW::_00)
    }
    #[doc = "25 us"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FREQ_COUNT_TIMEW::_01)
    }
    #[doc = "50 us"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FREQ_COUNT_TIMEW::_10)
    }
    #[doc = "100 us"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FREQ_COUNT_TIMEW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - Real time status of Coarse Tune Fail signal"]
    #[inline]
    pub fn ct_fail(&self) -> CT_FAILR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CT_FAILR { bits }
    }
    #[doc = "Bit 1 - CTUNE Failure Flag, held until cleared"]
    #[inline]
    pub fn ctff(&self) -> CTFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTFFR { bits }
    }
    #[doc = "Bit 2 - Real time status of Cycle Slip circuit"]
    #[inline]
    pub fn cs_fail(&self) -> CS_FAILR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CS_FAILR { bits }
    }
    #[doc = "Bit 3 - Cycle Slip Failure Flag, held until cleared"]
    #[inline]
    pub fn csff(&self) -> CSFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSFFR { bits }
    }
    #[doc = "Bit 4 - Real time status of Frequency Target Failure"]
    #[inline]
    pub fn ft_fail(&self) -> FT_FAILR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FT_FAILR { bits }
    }
    #[doc = "Bit 5 - Frequency Target Failure Flag"]
    #[inline]
    pub fn ftff(&self) -> FTFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FTFFR { bits }
    }
    #[doc = "Bit 7 - TSM Abort Failure Flag"]
    #[inline]
    pub fn taff(&self) -> TAFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAFFR { bits }
    }
    #[doc = "Bits 8:11 - CTUNE Lock Detect Fail Level"]
    #[inline]
    pub fn ctune_ldf_lev(&self) -> CTUNE_LDF_LEVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTUNE_LDF_LEVR { bits }
    }
    #[doc = "Bits 12:17 - RX Frequency Target Fail Threshold"]
    #[inline]
    pub fn ftf_rx_thrsh(&self) -> FTF_RX_THRSHR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FTF_RX_THRSHR { bits }
    }
    #[doc = "Bit 19 - RX Frequency Target Window time select"]
    #[inline]
    pub fn ftw_rx(&self) -> FTW_RXR {
        FTW_RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 20:25 - TX Frequency Target Fail Threshold"]
    #[inline]
    pub fn ftf_tx_thrsh(&self) -> FTF_TX_THRSHR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FTF_TX_THRSHR { bits }
    }
    #[doc = "Bit 27 - TX Frequency Target Window time select"]
    #[inline]
    pub fn ftw_tx(&self) -> FTW_TXR {
        FTW_TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Start the Frequency Meter"]
    #[inline]
    pub fn freq_count_go(&self) -> FREQ_COUNT_GOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FREQ_COUNT_GOR { bits }
    }
    #[doc = "Bit 29 - Frequency Meter has finished the Count Time"]
    #[inline]
    pub fn freq_count_finished(&self) -> FREQ_COUNT_FINISHEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FREQ_COUNT_FINISHEDR { bits }
    }
    #[doc = "Bits 30:31 - Frequency Meter Count Time"]
    #[inline]
    pub fn freq_count_time(&self) -> FREQ_COUNT_TIMER {
        FREQ_COUNT_TIMER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 6318080 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - CTUNE Failure Flag, held until cleared"]
    #[inline]
    pub fn ctff(&mut self) -> _CTFFW {
        _CTFFW { w: self }
    }
    #[doc = "Bit 3 - Cycle Slip Failure Flag, held until cleared"]
    #[inline]
    pub fn csff(&mut self) -> _CSFFW {
        _CSFFW { w: self }
    }
    #[doc = "Bit 5 - Frequency Target Failure Flag"]
    #[inline]
    pub fn ftff(&mut self) -> _FTFFW {
        _FTFFW { w: self }
    }
    #[doc = "Bit 7 - TSM Abort Failure Flag"]
    #[inline]
    pub fn taff(&mut self) -> _TAFFW {
        _TAFFW { w: self }
    }
    #[doc = "Bits 8:11 - CTUNE Lock Detect Fail Level"]
    #[inline]
    pub fn ctune_ldf_lev(&mut self) -> _CTUNE_LDF_LEVW {
        _CTUNE_LDF_LEVW { w: self }
    }
    #[doc = "Bits 12:17 - RX Frequency Target Fail Threshold"]
    #[inline]
    pub fn ftf_rx_thrsh(&mut self) -> _FTF_RX_THRSHW {
        _FTF_RX_THRSHW { w: self }
    }
    #[doc = "Bit 19 - RX Frequency Target Window time select"]
    #[inline]
    pub fn ftw_rx(&mut self) -> _FTW_RXW {
        _FTW_RXW { w: self }
    }
    #[doc = "Bits 20:25 - TX Frequency Target Fail Threshold"]
    #[inline]
    pub fn ftf_tx_thrsh(&mut self) -> _FTF_TX_THRSHW {
        _FTF_TX_THRSHW { w: self }
    }
    #[doc = "Bit 27 - TX Frequency Target Window time select"]
    #[inline]
    pub fn ftw_tx(&mut self) -> _FTW_TXW {
        _FTW_TXW { w: self }
    }
    #[doc = "Bit 28 - Start the Frequency Meter"]
    #[inline]
    pub fn freq_count_go(&mut self) -> _FREQ_COUNT_GOW {
        _FREQ_COUNT_GOW { w: self }
    }
    #[doc = "Bits 30:31 - Frequency Meter Count Time"]
    #[inline]
    pub fn freq_count_time(&mut self) -> _FREQ_COUNT_TIMEW {
        _FREQ_COUNT_TIMEW { w: self }
    }
}
