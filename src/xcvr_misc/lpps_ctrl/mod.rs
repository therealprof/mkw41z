#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPPS_CTRL {
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
pub struct LPPS_ENABLER {
    bits: bool,
}
impl LPPS_ENABLER {
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
pub struct LPPS_TZA_ALLOWR {
    bits: bool,
}
impl LPPS_TZA_ALLOWR {
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
pub struct LPPS_BBA_ALLOWR {
    bits: bool,
}
impl LPPS_BBA_ALLOWR {
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
pub struct LPPS_ADC_ALLOWR {
    bits: bool,
}
impl LPPS_ADC_ALLOWR {
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
pub struct LPPS_DCOC_ALLOWR {
    bits: bool,
}
impl LPPS_DCOC_ALLOWR {
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
pub struct LPPS_PDET_ALLOWR {
    bits: bool,
}
impl LPPS_PDET_ALLOWR {
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
pub struct LPPS_SY_LO_ALLOWR {
    bits: bool,
}
impl LPPS_SY_LO_ALLOWR {
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
pub struct LPPS_SY_LO_BUF_ALLOWR {
    bits: bool,
}
impl LPPS_SY_LO_BUF_ALLOWR {
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
pub struct LPPS_RX_DIG_ALLOWR {
    bits: bool,
}
impl LPPS_RX_DIG_ALLOWR {
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
pub struct LPPS_DCOC_DIG_ALLOWR {
    bits: bool,
}
impl LPPS_DCOC_DIG_ALLOWR {
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
pub struct LPPS_START_RXR {
    bits: u8,
}
impl LPPS_START_RXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPPS_DEST_RXR {
    bits: u8,
}
impl LPPS_DEST_RXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _LPPS_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPPS_ENABLEW<'a> {
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
pub struct _LPPS_TZA_ALLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _LPPS_TZA_ALLOWW<'a> {
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
pub struct _LPPS_BBA_ALLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _LPPS_BBA_ALLOWW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPPS_ADC_ALLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _LPPS_ADC_ALLOWW<'a> {
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
pub struct _LPPS_DCOC_ALLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _LPPS_DCOC_ALLOWW<'a> {
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
pub struct _LPPS_PDET_ALLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _LPPS_PDET_ALLOWW<'a> {
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
pub struct _LPPS_SY_LO_ALLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _LPPS_SY_LO_ALLOWW<'a> {
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
pub struct _LPPS_SY_LO_BUF_ALLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _LPPS_SY_LO_BUF_ALLOWW<'a> {
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
pub struct _LPPS_RX_DIG_ALLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _LPPS_RX_DIG_ALLOWW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPPS_DCOC_DIG_ALLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _LPPS_DCOC_DIG_ALLOWW<'a> {
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
#[doc = r" Proxy"]
pub struct _LPPS_START_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _LPPS_START_RXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPPS_DEST_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _LPPS_DEST_RXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - LPPS_ENABLE"]
    #[inline]
    pub fn lpps_enable(&self) -> LPPS_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPPS_ENABLER { bits }
    }
    #[doc = "Bit 1 - LPPS_TZA_ALLOW"]
    #[inline]
    pub fn lpps_tza_allow(&self) -> LPPS_TZA_ALLOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPPS_TZA_ALLOWR { bits }
    }
    #[doc = "Bit 2 - LPPS_BBA_ALLOW"]
    #[inline]
    pub fn lpps_bba_allow(&self) -> LPPS_BBA_ALLOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPPS_BBA_ALLOWR { bits }
    }
    #[doc = "Bit 3 - LPPS_ADC_ALLOW"]
    #[inline]
    pub fn lpps_adc_allow(&self) -> LPPS_ADC_ALLOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPPS_ADC_ALLOWR { bits }
    }
    #[doc = "Bit 4 - LPPS_DCOC_ALLOW"]
    #[inline]
    pub fn lpps_dcoc_allow(&self) -> LPPS_DCOC_ALLOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPPS_DCOC_ALLOWR { bits }
    }
    #[doc = "Bit 5 - LPPS_PDET_ALLOW"]
    #[inline]
    pub fn lpps_pdet_allow(&self) -> LPPS_PDET_ALLOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPPS_PDET_ALLOWR { bits }
    }
    #[doc = "Bit 6 - LPPS_SY_LO_ALLOW"]
    #[inline]
    pub fn lpps_sy_lo_allow(&self) -> LPPS_SY_LO_ALLOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPPS_SY_LO_ALLOWR { bits }
    }
    #[doc = "Bit 7 - LPPS_SY_LO_BUF_ALLOW"]
    #[inline]
    pub fn lpps_sy_lo_buf_allow(&self) -> LPPS_SY_LO_BUF_ALLOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPPS_SY_LO_BUF_ALLOWR { bits }
    }
    #[doc = "Bit 8 - LPPS_RX_DIG_ALLOW"]
    #[inline]
    pub fn lpps_rx_dig_allow(&self) -> LPPS_RX_DIG_ALLOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPPS_RX_DIG_ALLOWR { bits }
    }
    #[doc = "Bit 9 - LPPS_DCOC_DIG_ALLOW"]
    #[inline]
    pub fn lpps_dcoc_dig_allow(&self) -> LPPS_DCOC_DIG_ALLOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPPS_DCOC_DIG_ALLOWR { bits }
    }
    #[doc = "Bits 16:23 - LPPS Fast TSM RX Warmup \"Jump-from\" Point"]
    #[inline]
    pub fn lpps_start_rx(&self) -> LPPS_START_RXR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPPS_START_RXR { bits }
    }
    #[doc = "Bits 24:31 - LPPS Fast TSM RX Warmup \"Jump-to\" Point"]
    #[inline]
    pub fn lpps_dest_rx(&self) -> LPPS_DEST_RXR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPPS_DEST_RXR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1680211968 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - LPPS_ENABLE"]
    #[inline]
    pub fn lpps_enable(&mut self) -> _LPPS_ENABLEW {
        _LPPS_ENABLEW { w: self }
    }
    #[doc = "Bit 1 - LPPS_TZA_ALLOW"]
    #[inline]
    pub fn lpps_tza_allow(&mut self) -> _LPPS_TZA_ALLOWW {
        _LPPS_TZA_ALLOWW { w: self }
    }
    #[doc = "Bit 2 - LPPS_BBA_ALLOW"]
    #[inline]
    pub fn lpps_bba_allow(&mut self) -> _LPPS_BBA_ALLOWW {
        _LPPS_BBA_ALLOWW { w: self }
    }
    #[doc = "Bit 3 - LPPS_ADC_ALLOW"]
    #[inline]
    pub fn lpps_adc_allow(&mut self) -> _LPPS_ADC_ALLOWW {
        _LPPS_ADC_ALLOWW { w: self }
    }
    #[doc = "Bit 4 - LPPS_DCOC_ALLOW"]
    #[inline]
    pub fn lpps_dcoc_allow(&mut self) -> _LPPS_DCOC_ALLOWW {
        _LPPS_DCOC_ALLOWW { w: self }
    }
    #[doc = "Bit 5 - LPPS_PDET_ALLOW"]
    #[inline]
    pub fn lpps_pdet_allow(&mut self) -> _LPPS_PDET_ALLOWW {
        _LPPS_PDET_ALLOWW { w: self }
    }
    #[doc = "Bit 6 - LPPS_SY_LO_ALLOW"]
    #[inline]
    pub fn lpps_sy_lo_allow(&mut self) -> _LPPS_SY_LO_ALLOWW {
        _LPPS_SY_LO_ALLOWW { w: self }
    }
    #[doc = "Bit 7 - LPPS_SY_LO_BUF_ALLOW"]
    #[inline]
    pub fn lpps_sy_lo_buf_allow(&mut self) -> _LPPS_SY_LO_BUF_ALLOWW {
        _LPPS_SY_LO_BUF_ALLOWW { w: self }
    }
    #[doc = "Bit 8 - LPPS_RX_DIG_ALLOW"]
    #[inline]
    pub fn lpps_rx_dig_allow(&mut self) -> _LPPS_RX_DIG_ALLOWW {
        _LPPS_RX_DIG_ALLOWW { w: self }
    }
    #[doc = "Bit 9 - LPPS_DCOC_DIG_ALLOW"]
    #[inline]
    pub fn lpps_dcoc_dig_allow(&mut self) -> _LPPS_DCOC_DIG_ALLOWW {
        _LPPS_DCOC_DIG_ALLOWW { w: self }
    }
    #[doc = "Bits 16:23 - LPPS Fast TSM RX Warmup \"Jump-from\" Point"]
    #[inline]
    pub fn lpps_start_rx(&mut self) -> _LPPS_START_RXW {
        _LPPS_START_RXW { w: self }
    }
    #[doc = "Bits 24:31 - LPPS Fast TSM RX Warmup \"Jump-to\" Point"]
    #[inline]
    pub fn lpps_dest_rx(&mut self) -> _LPPS_DEST_RXW {
        _LPPS_DEST_RXW { w: self }
    }
}
