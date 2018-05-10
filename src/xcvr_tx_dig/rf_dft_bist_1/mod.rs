#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RF_DFT_BIST_1 {
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
pub struct CTUNE_BIST_GOR {
    bits: bool,
}
impl CTUNE_BIST_GOR {
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
pub struct CTUNE_BIST_FINISHEDR {
    bits: bool,
}
impl CTUNE_BIST_FINISHEDR {
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
pub struct CTUNE_BIST_RESULTR {
    bits: bool,
}
impl CTUNE_BIST_RESULTR {
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
pub struct CTUNE_BIST_THRSHLDR {
    bits: u8,
}
impl CTUNE_BIST_THRSHLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTUNE_MAX_DIFFR {
    bits: u8,
}
impl CTUNE_MAX_DIFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTUNE_MAX_DIFF_CHR {
    bits: u8,
}
impl CTUNE_MAX_DIFF_CHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PA_AM_MOD_FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA_AM_MOD_FREQR {
    #[doc = "4 MHz"]
    _000,
    #[doc = "2 MHz"]
    _001,
    #[doc = "1 MHz"]
    _010,
    #[doc = "500 kHz"]
    _011,
    #[doc = "250 kHz"]
    _100,
    #[doc = "125 kHz"]
    _101,
    #[doc = "62.5 kHz"]
    _110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PA_AM_MOD_FREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PA_AM_MOD_FREQR::_000 => 0,
            PA_AM_MOD_FREQR::_001 => 1,
            PA_AM_MOD_FREQR::_010 => 2,
            PA_AM_MOD_FREQR::_011 => 3,
            PA_AM_MOD_FREQR::_100 => 4,
            PA_AM_MOD_FREQR::_101 => 5,
            PA_AM_MOD_FREQR::_110 => 6,
            PA_AM_MOD_FREQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PA_AM_MOD_FREQR {
        match value {
            0 => PA_AM_MOD_FREQR::_000,
            1 => PA_AM_MOD_FREQR::_001,
            2 => PA_AM_MOD_FREQR::_010,
            3 => PA_AM_MOD_FREQR::_011,
            4 => PA_AM_MOD_FREQR::_100,
            5 => PA_AM_MOD_FREQR::_101,
            6 => PA_AM_MOD_FREQR::_110,
            i => PA_AM_MOD_FREQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PA_AM_MOD_FREQR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PA_AM_MOD_FREQR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PA_AM_MOD_FREQR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PA_AM_MOD_FREQR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PA_AM_MOD_FREQR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PA_AM_MOD_FREQR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PA_AM_MOD_FREQR::_110
    }
}
#[doc = "Possible values of the field `PA_AM_MOD_ENTRIES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA_AM_MOD_ENTRIESR {
    #[doc = "2 entries"]
    _001,
    #[doc = "3 entries"]
    _010,
    #[doc = "4 entries"]
    _011,
    #[doc = "5 entries"]
    _100,
    #[doc = "6 entries"]
    _101,
    #[doc = "7 entries"]
    _110,
    #[doc = "8 entries"]
    _111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PA_AM_MOD_ENTRIESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PA_AM_MOD_ENTRIESR::_001 => 1,
            PA_AM_MOD_ENTRIESR::_010 => 2,
            PA_AM_MOD_ENTRIESR::_011 => 3,
            PA_AM_MOD_ENTRIESR::_100 => 4,
            PA_AM_MOD_ENTRIESR::_101 => 5,
            PA_AM_MOD_ENTRIESR::_110 => 6,
            PA_AM_MOD_ENTRIESR::_111 => 7,
            PA_AM_MOD_ENTRIESR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PA_AM_MOD_ENTRIESR {
        match value {
            1 => PA_AM_MOD_ENTRIESR::_001,
            2 => PA_AM_MOD_ENTRIESR::_010,
            3 => PA_AM_MOD_ENTRIESR::_011,
            4 => PA_AM_MOD_ENTRIESR::_100,
            5 => PA_AM_MOD_ENTRIESR::_101,
            6 => PA_AM_MOD_ENTRIESR::_110,
            7 => PA_AM_MOD_ENTRIESR::_111,
            i => PA_AM_MOD_ENTRIESR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PA_AM_MOD_ENTRIESR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PA_AM_MOD_ENTRIESR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PA_AM_MOD_ENTRIESR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PA_AM_MOD_ENTRIESR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PA_AM_MOD_ENTRIESR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PA_AM_MOD_ENTRIESR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PA_AM_MOD_ENTRIESR::_111
    }
}
#[doc = r" Value of the field"]
pub struct PA_AM_MOD_ENR {
    bits: bool,
}
impl PA_AM_MOD_ENR {
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
pub struct _CTUNE_BIST_GOW<'a> {
    w: &'a mut W,
}
impl<'a> _CTUNE_BIST_GOW<'a> {
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
pub struct _CTUNE_BIST_THRSHLDW<'a> {
    w: &'a mut W,
}
impl<'a> _CTUNE_BIST_THRSHLDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PA_AM_MOD_FREQ`"]
pub enum PA_AM_MOD_FREQW {
    #[doc = "4 MHz"]
    _000,
    #[doc = "2 MHz"]
    _001,
    #[doc = "1 MHz"]
    _010,
    #[doc = "500 kHz"]
    _011,
    #[doc = "250 kHz"]
    _100,
    #[doc = "125 kHz"]
    _101,
    #[doc = "62.5 kHz"]
    _110,
}
impl PA_AM_MOD_FREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PA_AM_MOD_FREQW::_000 => 0,
            PA_AM_MOD_FREQW::_001 => 1,
            PA_AM_MOD_FREQW::_010 => 2,
            PA_AM_MOD_FREQW::_011 => 3,
            PA_AM_MOD_FREQW::_100 => 4,
            PA_AM_MOD_FREQW::_101 => 5,
            PA_AM_MOD_FREQW::_110 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PA_AM_MOD_FREQW<'a> {
    w: &'a mut W,
}
impl<'a> _PA_AM_MOD_FREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PA_AM_MOD_FREQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "4 MHz"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PA_AM_MOD_FREQW::_000)
    }
    #[doc = "2 MHz"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PA_AM_MOD_FREQW::_001)
    }
    #[doc = "1 MHz"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PA_AM_MOD_FREQW::_010)
    }
    #[doc = "500 kHz"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PA_AM_MOD_FREQW::_011)
    }
    #[doc = "250 kHz"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PA_AM_MOD_FREQW::_100)
    }
    #[doc = "125 kHz"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PA_AM_MOD_FREQW::_101)
    }
    #[doc = "62.5 kHz"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PA_AM_MOD_FREQW::_110)
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
#[doc = "Values that can be written to the field `PA_AM_MOD_ENTRIES`"]
pub enum PA_AM_MOD_ENTRIESW {
    #[doc = "2 entries"]
    _001,
    #[doc = "3 entries"]
    _010,
    #[doc = "4 entries"]
    _011,
    #[doc = "5 entries"]
    _100,
    #[doc = "6 entries"]
    _101,
    #[doc = "7 entries"]
    _110,
    #[doc = "8 entries"]
    _111,
}
impl PA_AM_MOD_ENTRIESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PA_AM_MOD_ENTRIESW::_001 => 1,
            PA_AM_MOD_ENTRIESW::_010 => 2,
            PA_AM_MOD_ENTRIESW::_011 => 3,
            PA_AM_MOD_ENTRIESW::_100 => 4,
            PA_AM_MOD_ENTRIESW::_101 => 5,
            PA_AM_MOD_ENTRIESW::_110 => 6,
            PA_AM_MOD_ENTRIESW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PA_AM_MOD_ENTRIESW<'a> {
    w: &'a mut W,
}
impl<'a> _PA_AM_MOD_ENTRIESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PA_AM_MOD_ENTRIESW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "2 entries"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PA_AM_MOD_ENTRIESW::_001)
    }
    #[doc = "3 entries"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PA_AM_MOD_ENTRIESW::_010)
    }
    #[doc = "4 entries"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PA_AM_MOD_ENTRIESW::_011)
    }
    #[doc = "5 entries"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PA_AM_MOD_ENTRIESW::_100)
    }
    #[doc = "6 entries"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PA_AM_MOD_ENTRIESW::_101)
    }
    #[doc = "7 entries"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PA_AM_MOD_ENTRIESW::_110)
    }
    #[doc = "8 entries"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PA_AM_MOD_ENTRIESW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PA_AM_MOD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PA_AM_MOD_ENW<'a> {
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
    #[doc = "Bit 0 - Start the Coarse Tune BIST"]
    #[inline]
    pub fn ctune_bist_go(&self) -> CTUNE_BIST_GOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTUNE_BIST_GOR { bits }
    }
    #[doc = "Bit 1 - Coarse Tune BIST has finished Tuning all Channels"]
    #[inline]
    pub fn ctune_bist_finished(&self) -> CTUNE_BIST_FINISHEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTUNE_BIST_FINISHEDR { bits }
    }
    #[doc = "Bit 2 - Coarse Tune BIST Result"]
    #[inline]
    pub fn ctune_bist_result(&self) -> CTUNE_BIST_RESULTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTUNE_BIST_RESULTR { bits }
    }
    #[doc = "Bits 4:7 - Maximum Difference Threshold for Coarse Tune BIST"]
    #[inline]
    pub fn ctune_bist_thrshld(&self) -> CTUNE_BIST_THRSHLDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTUNE_BIST_THRSHLDR { bits }
    }
    #[doc = "Bits 8:15 - Maximum Frequency Count Difference found by the Coarse Tune BIST"]
    #[inline]
    pub fn ctune_max_diff(&self) -> CTUNE_MAX_DIFFR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTUNE_MAX_DIFFR { bits }
    }
    #[doc = "Bits 16:22 - Maximum Frequency Count Difference Radio Channel"]
    #[inline]
    pub fn ctune_max_diff_ch(&self) -> CTUNE_MAX_DIFF_CHR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTUNE_MAX_DIFF_CHR { bits }
    }
    #[doc = "Bits 24:26 - RF Power Amplifier Amplitude Modulation Frequency"]
    #[inline]
    pub fn pa_am_mod_freq(&self) -> PA_AM_MOD_FREQR {
        PA_AM_MOD_FREQR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:30 - RF Power Amplifier Amplitude Modulation Table Entries"]
    #[inline]
    pub fn pa_am_mod_entries(&self) -> PA_AM_MOD_ENTRIESR {
        PA_AM_MOD_ENTRIESR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - RF Power Amplifier Amplitude Modulation Enable"]
    #[inline]
    pub fn pa_am_mod_en(&self) -> PA_AM_MOD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PA_AM_MOD_ENR { bits }
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
    #[doc = "Bit 0 - Start the Coarse Tune BIST"]
    #[inline]
    pub fn ctune_bist_go(&mut self) -> _CTUNE_BIST_GOW {
        _CTUNE_BIST_GOW { w: self }
    }
    #[doc = "Bits 4:7 - Maximum Difference Threshold for Coarse Tune BIST"]
    #[inline]
    pub fn ctune_bist_thrshld(&mut self) -> _CTUNE_BIST_THRSHLDW {
        _CTUNE_BIST_THRSHLDW { w: self }
    }
    #[doc = "Bits 24:26 - RF Power Amplifier Amplitude Modulation Frequency"]
    #[inline]
    pub fn pa_am_mod_freq(&mut self) -> _PA_AM_MOD_FREQW {
        _PA_AM_MOD_FREQW { w: self }
    }
    #[doc = "Bits 28:30 - RF Power Amplifier Amplitude Modulation Table Entries"]
    #[inline]
    pub fn pa_am_mod_entries(&mut self) -> _PA_AM_MOD_ENTRIESW {
        _PA_AM_MOD_ENTRIESW { w: self }
    }
    #[doc = "Bit 31 - RF Power Amplifier Amplitude Modulation Enable"]
    #[inline]
    pub fn pa_am_mod_en(&mut self) -> _PA_AM_MOD_ENW {
        _PA_AM_MOD_ENW { w: self }
    }
}
