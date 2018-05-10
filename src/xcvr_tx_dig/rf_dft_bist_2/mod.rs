#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RF_DFT_BIST_2 {
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
pub struct SYN_BIST_GOR {
    bits: bool,
}
impl SYN_BIST_GOR {
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
pub struct SYN_BIST_FINISHEDR {
    bits: bool,
}
impl SYN_BIST_FINISHEDR {
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
pub struct SYN_BIST_RESULTR {
    bits: bool,
}
impl SYN_BIST_RESULTR {
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
pub struct SYN_BIST_ALL_CHANNELSR {
    bits: bool,
}
impl SYN_BIST_ALL_CHANNELSR {
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
pub struct FREQ_COUNT_THRESHOLDR {
    bits: u8,
}
impl FREQ_COUNT_THRESHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPM_INL_BIST_GOR {
    bits: bool,
}
impl HPM_INL_BIST_GOR {
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
pub struct HPM_INL_BIST_FINISHEDR {
    bits: bool,
}
impl HPM_INL_BIST_FINISHEDR {
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
pub struct HPM_INL_BIST_RESULTR {
    bits: bool,
}
impl HPM_INL_BIST_RESULTR {
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
pub struct HPM_DNL_BIST_GOR {
    bits: bool,
}
impl HPM_DNL_BIST_GOR {
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
pub struct HPM_DNL_BIST_FINISHEDR {
    bits: bool,
}
impl HPM_DNL_BIST_FINISHEDR {
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
pub struct HPM_DNL_BIST_RESULTR {
    bits: bool,
}
impl HPM_DNL_BIST_RESULTR {
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
pub struct DFT_MAX_RAM_SIZER {
    bits: u16,
}
impl DFT_MAX_RAM_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SYN_BIST_GOW<'a> {
    w: &'a mut W,
}
impl<'a> _SYN_BIST_GOW<'a> {
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
pub struct _SYN_BIST_ALL_CHANNELSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYN_BIST_ALL_CHANNELSW<'a> {
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
pub struct _FREQ_COUNT_THRESHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQ_COUNT_THRESHOLDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_INL_BIST_GOW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_INL_BIST_GOW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_DNL_BIST_GOW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_DNL_BIST_GOW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DFT_MAX_RAM_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DFT_MAX_RAM_SIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
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
    #[doc = "Bit 0 - Start the PLL Frequency Synthesizer BIST"]
    #[inline]
    pub fn syn_bist_go(&self) -> SYN_BIST_GOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYN_BIST_GOR { bits }
    }
    #[doc = "Bit 1 - PLL Frequency Synthesizer BIST has finished trying to lock to Radio Channels"]
    #[inline]
    pub fn syn_bist_finished(&self) -> SYN_BIST_FINISHEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYN_BIST_FINISHEDR { bits }
    }
    #[doc = "Bit 2 - PLL Frequency Synthesizer BIST Result"]
    #[inline]
    pub fn syn_bist_result(&self) -> SYN_BIST_RESULTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYN_BIST_RESULTR { bits }
    }
    #[doc = "Bit 3 - PLL Frequency Synthesizer BIST All Channels"]
    #[inline]
    pub fn syn_bist_all_channels(&self) -> SYN_BIST_ALL_CHANNELSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYN_BIST_ALL_CHANNELSR { bits }
    }
    #[doc = "Bits 4:11 - Frequency Meter Count Difference Threshold"]
    #[inline]
    pub fn freq_count_threshold(&self) -> FREQ_COUNT_THRESHOLDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FREQ_COUNT_THRESHOLDR { bits }
    }
    #[doc = "Bit 12 - Start the High Port Modulator DAC INL BIST"]
    #[inline]
    pub fn hpm_inl_bist_go(&self) -> HPM_INL_BIST_GOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_INL_BIST_GOR { bits }
    }
    #[doc = "Bit 13 - High Port Modulator DAC INL BIST has finished measuring the INL of the HPM DAC"]
    #[inline]
    pub fn hpm_inl_bist_finished(&self) -> HPM_INL_BIST_FINISHEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_INL_BIST_FINISHEDR { bits }
    }
    #[doc = "Bit 14 - High Port Modulator DAC INL BIST Result"]
    #[inline]
    pub fn hpm_inl_bist_result(&self) -> HPM_INL_BIST_RESULTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_INL_BIST_RESULTR { bits }
    }
    #[doc = "Bit 16 - Start the High Port Modulator DAC DNL BIST"]
    #[inline]
    pub fn hpm_dnl_bist_go(&self) -> HPM_DNL_BIST_GOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_DNL_BIST_GOR { bits }
    }
    #[doc = "Bit 17 - High Port Modulator DAC DNL BIST has finished measuring the DNL of the HPM DAC"]
    #[inline]
    pub fn hpm_dnl_bist_finished(&self) -> HPM_DNL_BIST_FINISHEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_DNL_BIST_FINISHEDR { bits }
    }
    #[doc = "Bit 18 - High Port Modulator DAC DNL BIST Result"]
    #[inline]
    pub fn hpm_dnl_bist_result(&self) -> HPM_DNL_BIST_RESULTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_DNL_BIST_RESULTR { bits }
    }
    #[doc = "Bits 20:28 - Maximum RAM Address to use as Modulation"]
    #[inline]
    pub fn dft_max_ram_size(&self) -> DFT_MAX_RAM_SIZER {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DFT_MAX_RAM_SIZER { bits }
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
    #[doc = "Bit 0 - Start the PLL Frequency Synthesizer BIST"]
    #[inline]
    pub fn syn_bist_go(&mut self) -> _SYN_BIST_GOW {
        _SYN_BIST_GOW { w: self }
    }
    #[doc = "Bit 3 - PLL Frequency Synthesizer BIST All Channels"]
    #[inline]
    pub fn syn_bist_all_channels(&mut self) -> _SYN_BIST_ALL_CHANNELSW {
        _SYN_BIST_ALL_CHANNELSW { w: self }
    }
    #[doc = "Bits 4:11 - Frequency Meter Count Difference Threshold"]
    #[inline]
    pub fn freq_count_threshold(&mut self) -> _FREQ_COUNT_THRESHOLDW {
        _FREQ_COUNT_THRESHOLDW { w: self }
    }
    #[doc = "Bit 12 - Start the High Port Modulator DAC INL BIST"]
    #[inline]
    pub fn hpm_inl_bist_go(&mut self) -> _HPM_INL_BIST_GOW {
        _HPM_INL_BIST_GOW { w: self }
    }
    #[doc = "Bit 16 - Start the High Port Modulator DAC DNL BIST"]
    #[inline]
    pub fn hpm_dnl_bist_go(&mut self) -> _HPM_DNL_BIST_GOW {
        _HPM_DNL_BIST_GOW { w: self }
    }
    #[doc = "Bits 20:28 - Maximum RAM Address to use as Modulation"]
    #[inline]
    pub fn dft_max_ram_size(&mut self) -> _DFT_MAX_RAM_SIZEW {
        _DFT_MAX_RAM_SIZEW { w: self }
    }
}
