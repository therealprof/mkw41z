#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXDIG_DFT {
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
#[doc = "Possible values of the field `DFT_TONE_FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFT_TONE_FREQR {
    #[doc = "1/64 of the ref osc frequency (500kHz for 32MHz ref osc)"]
    _0,
    #[doc = "1/128 of the ref osc frequency (250kHz for 32MHz ref osc)"]
    _1,
    #[doc = "1/256 of the ref osc frequency (125kHz for 32MHz ref osc)"]
    _2,
    #[doc = "1/512 of the ref osc frequency (62.5kHz for 32MHz ref osc)"]
    _3,
    #[doc = "1/1024 of the ref osc frequency (31.25kHz for 32MHz ref osc)"]
    _4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DFT_TONE_FREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DFT_TONE_FREQR::_0 => 0,
            DFT_TONE_FREQR::_1 => 1,
            DFT_TONE_FREQR::_2 => 2,
            DFT_TONE_FREQR::_3 => 3,
            DFT_TONE_FREQR::_4 => 4,
            DFT_TONE_FREQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DFT_TONE_FREQR {
        match value {
            0 => DFT_TONE_FREQR::_0,
            1 => DFT_TONE_FREQR::_1,
            2 => DFT_TONE_FREQR::_2,
            3 => DFT_TONE_FREQR::_3,
            4 => DFT_TONE_FREQR::_4,
            i => DFT_TONE_FREQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFT_TONE_FREQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFT_TONE_FREQR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == DFT_TONE_FREQR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == DFT_TONE_FREQR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == DFT_TONE_FREQR::_4
    }
}
#[doc = "Possible values of the field `DFT_TONE_SCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFT_TONE_SCALER {
    #[doc = "The DFT tone generator uses 3/4 of the DC offset correction DAC range."]
    _0,
    #[doc = "The DFT tone generator uses 1/2 of the DC offset correction DAC range."]
    _1,
}
impl DFT_TONE_SCALER {
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
            DFT_TONE_SCALER::_0 => false,
            DFT_TONE_SCALER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFT_TONE_SCALER {
        match value {
            false => DFT_TONE_SCALER::_0,
            true => DFT_TONE_SCALER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFT_TONE_SCALER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFT_TONE_SCALER::_1
    }
}
#[doc = "Possible values of the field `DFT_TONE_TZA_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFT_TONE_TZA_ENR {
    #[doc = "The DCOC controls the TZA DC offset correction DACs"]
    _0,
    #[doc = "A tone is generated using the TZA DC offset correction DACs."]
    _1,
}
impl DFT_TONE_TZA_ENR {
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
            DFT_TONE_TZA_ENR::_0 => false,
            DFT_TONE_TZA_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFT_TONE_TZA_ENR {
        match value {
            false => DFT_TONE_TZA_ENR::_0,
            true => DFT_TONE_TZA_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFT_TONE_TZA_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFT_TONE_TZA_ENR::_1
    }
}
#[doc = "Possible values of the field `DFT_TONE_BBA_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFT_TONE_BBA_ENR {
    #[doc = "The DCOC controls the BBA DC offset correction DACs"]
    _0,
    #[doc = "A tone is generated using the BBA DC offset correction DACs."]
    _1,
}
impl DFT_TONE_BBA_ENR {
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
            DFT_TONE_BBA_ENR::_0 => false,
            DFT_TONE_BBA_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFT_TONE_BBA_ENR {
        match value {
            false => DFT_TONE_BBA_ENR::_0,
            true => DFT_TONE_BBA_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFT_TONE_BBA_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFT_TONE_BBA_ENR::_1
    }
}
#[doc = "Values that can be written to the field `DFT_TONE_FREQ`"]
pub enum DFT_TONE_FREQW {
    #[doc = "1/64 of the ref osc frequency (500kHz for 32MHz ref osc)"]
    _0,
    #[doc = "1/128 of the ref osc frequency (250kHz for 32MHz ref osc)"]
    _1,
    #[doc = "1/256 of the ref osc frequency (125kHz for 32MHz ref osc)"]
    _2,
    #[doc = "1/512 of the ref osc frequency (62.5kHz for 32MHz ref osc)"]
    _3,
    #[doc = "1/1024 of the ref osc frequency (31.25kHz for 32MHz ref osc)"]
    _4,
}
impl DFT_TONE_FREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DFT_TONE_FREQW::_0 => 0,
            DFT_TONE_FREQW::_1 => 1,
            DFT_TONE_FREQW::_2 => 2,
            DFT_TONE_FREQW::_3 => 3,
            DFT_TONE_FREQW::_4 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFT_TONE_FREQW<'a> {
    w: &'a mut W,
}
impl<'a> _DFT_TONE_FREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFT_TONE_FREQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1/64 of the ref osc frequency (500kHz for 32MHz ref osc)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFT_TONE_FREQW::_0)
    }
    #[doc = "1/128 of the ref osc frequency (250kHz for 32MHz ref osc)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFT_TONE_FREQW::_1)
    }
    #[doc = "1/256 of the ref osc frequency (125kHz for 32MHz ref osc)"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(DFT_TONE_FREQW::_2)
    }
    #[doc = "1/512 of the ref osc frequency (62.5kHz for 32MHz ref osc)"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(DFT_TONE_FREQW::_3)
    }
    #[doc = "1/1024 of the ref osc frequency (31.25kHz for 32MHz ref osc)"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(DFT_TONE_FREQW::_4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DFT_TONE_SCALE`"]
pub enum DFT_TONE_SCALEW {
    #[doc = "The DFT tone generator uses 3/4 of the DC offset correction DAC range."]
    _0,
    #[doc = "The DFT tone generator uses 1/2 of the DC offset correction DAC range."]
    _1,
}
impl DFT_TONE_SCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFT_TONE_SCALEW::_0 => false,
            DFT_TONE_SCALEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFT_TONE_SCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _DFT_TONE_SCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFT_TONE_SCALEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DFT tone generator uses 3/4 of the DC offset correction DAC range."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFT_TONE_SCALEW::_0)
    }
    #[doc = "The DFT tone generator uses 1/2 of the DC offset correction DAC range."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFT_TONE_SCALEW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DFT_TONE_TZA_EN`"]
pub enum DFT_TONE_TZA_ENW {
    #[doc = "The DCOC controls the TZA DC offset correction DACs"]
    _0,
    #[doc = "A tone is generated using the TZA DC offset correction DACs."]
    _1,
}
impl DFT_TONE_TZA_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFT_TONE_TZA_ENW::_0 => false,
            DFT_TONE_TZA_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFT_TONE_TZA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DFT_TONE_TZA_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFT_TONE_TZA_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DCOC controls the TZA DC offset correction DACs"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFT_TONE_TZA_ENW::_0)
    }
    #[doc = "A tone is generated using the TZA DC offset correction DACs."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFT_TONE_TZA_ENW::_1)
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
#[doc = "Values that can be written to the field `DFT_TONE_BBA_EN`"]
pub enum DFT_TONE_BBA_ENW {
    #[doc = "The DCOC controls the BBA DC offset correction DACs"]
    _0,
    #[doc = "A tone is generated using the BBA DC offset correction DACs."]
    _1,
}
impl DFT_TONE_BBA_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFT_TONE_BBA_ENW::_0 => false,
            DFT_TONE_BBA_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFT_TONE_BBA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DFT_TONE_BBA_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFT_TONE_BBA_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DCOC controls the BBA DC offset correction DACs"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFT_TONE_BBA_ENW::_0)
    }
    #[doc = "A tone is generated using the BBA DC offset correction DACs."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFT_TONE_BBA_ENW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - DFT Tone Generator Frequency"]
    #[inline]
    pub fn dft_tone_freq(&self) -> DFT_TONE_FREQR {
        DFT_TONE_FREQR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - DFT Tone Generator Scale"]
    #[inline]
    pub fn dft_tone_scale(&self) -> DFT_TONE_SCALER {
        DFT_TONE_SCALER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - DFT Tone Generator TZA Enable"]
    #[inline]
    pub fn dft_tone_tza_en(&self) -> DFT_TONE_TZA_ENR {
        DFT_TONE_TZA_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - DFT Tone Generator BBA Enable"]
    #[inline]
    pub fn dft_tone_bba_en(&self) -> DFT_TONE_BBA_ENR {
        DFT_TONE_BBA_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:2 - DFT Tone Generator Frequency"]
    #[inline]
    pub fn dft_tone_freq(&mut self) -> _DFT_TONE_FREQW {
        _DFT_TONE_FREQW { w: self }
    }
    #[doc = "Bit 3 - DFT Tone Generator Scale"]
    #[inline]
    pub fn dft_tone_scale(&mut self) -> _DFT_TONE_SCALEW {
        _DFT_TONE_SCALEW { w: self }
    }
    #[doc = "Bit 4 - DFT Tone Generator TZA Enable"]
    #[inline]
    pub fn dft_tone_tza_en(&mut self) -> _DFT_TONE_TZA_ENW {
        _DFT_TONE_TZA_ENW { w: self }
    }
    #[doc = "Bit 5 - DFT Tone Generator BBA Enable"]
    #[inline]
    pub fn dft_tone_bba_en(&mut self) -> _DFT_TONE_BBA_ENW {
        _DFT_TONE_BBA_ENW { w: self }
    }
}
