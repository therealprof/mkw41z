#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_RCCAL_CTRL1 {
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
pub struct ADC_RCCAL_OFFSETR {
    bits: u8,
}
impl ADC_RCCAL_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADC_RCCAL_MANUALR {
    bits: u8,
}
impl ADC_RCCAL_MANUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ADC_RCCAL_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_RCCAL_DISR {
    #[doc = "ADC RC Calibration is enabled"]
    _0,
    #[doc = "ADC RC Calibration is disabled"]
    _1,
}
impl ADC_RCCAL_DISR {
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
            ADC_RCCAL_DISR::_0 => false,
            ADC_RCCAL_DISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_RCCAL_DISR {
        match value {
            false => ADC_RCCAL_DISR::_0,
            true => ADC_RCCAL_DISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADC_RCCAL_DISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADC_RCCAL_DISR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BBA2_RCCAL_OFFSETR {
    bits: u8,
}
impl BBA2_RCCAL_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BBA2_RCCAL_MANUALR {
    bits: u8,
}
impl BBA2_RCCAL_MANUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BBA2_RCCAL_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBA2_RCCAL_DISR {
    #[doc = "BBA2 RC Calibration is enabled"]
    _0,
    #[doc = "BBA2 RC Calibration is disabled"]
    _1,
}
impl BBA2_RCCAL_DISR {
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
            BBA2_RCCAL_DISR::_0 => false,
            BBA2_RCCAL_DISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BBA2_RCCAL_DISR {
        match value {
            false => BBA2_RCCAL_DISR::_0,
            true => BBA2_RCCAL_DISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BBA2_RCCAL_DISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BBA2_RCCAL_DISR::_1
    }
}
#[doc = r" Proxy"]
pub struct _ADC_RCCAL_OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RCCAL_OFFSETW<'a> {
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
pub struct _ADC_RCCAL_MANUALW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RCCAL_MANUALW<'a> {
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
#[doc = "Values that can be written to the field `ADC_RCCAL_DIS`"]
pub enum ADC_RCCAL_DISW {
    #[doc = "ADC RC Calibration is enabled"]
    _0,
    #[doc = "ADC RC Calibration is disabled"]
    _1,
}
impl ADC_RCCAL_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_RCCAL_DISW::_0 => false,
            ADC_RCCAL_DISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_RCCAL_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RCCAL_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_RCCAL_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC RC Calibration is enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC_RCCAL_DISW::_0)
    }
    #[doc = "ADC RC Calibration is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC_RCCAL_DISW::_1)
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
#[doc = r" Proxy"]
pub struct _BBA2_RCCAL_OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA2_RCCAL_OFFSETW<'a> {
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
pub struct _BBA2_RCCAL_MANUALW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA2_RCCAL_MANUALW<'a> {
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
#[doc = "Values that can be written to the field `BBA2_RCCAL_DIS`"]
pub enum BBA2_RCCAL_DISW {
    #[doc = "BBA2 RC Calibration is enabled"]
    _0,
    #[doc = "BBA2 RC Calibration is disabled"]
    _1,
}
impl BBA2_RCCAL_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BBA2_RCCAL_DISW::_0 => false,
            BBA2_RCCAL_DISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BBA2_RCCAL_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _BBA2_RCCAL_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BBA2_RCCAL_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BBA2 RC Calibration is enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BBA2_RCCAL_DISW::_0)
    }
    #[doc = "BBA2 RC Calibration is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BBA2_RCCAL_DISW::_1)
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
    #[doc = "Bits 0:3 - ADC RC Calibration value offset"]
    #[inline]
    pub fn adc_rccal_offset(&self) -> ADC_RCCAL_OFFSETR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADC_RCCAL_OFFSETR { bits }
    }
    #[doc = "Bits 4:8 - ADC RC Calibration manual value"]
    #[inline]
    pub fn adc_rccal_manual(&self) -> ADC_RCCAL_MANUALR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADC_RCCAL_MANUALR { bits }
    }
    #[doc = "Bit 9 - ADC RC Calibration Disable"]
    #[inline]
    pub fn adc_rccal_dis(&self) -> ADC_RCCAL_DISR {
        ADC_RCCAL_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - BBA2 RC Calibration value offset"]
    #[inline]
    pub fn bba2_rccal_offset(&self) -> BBA2_RCCAL_OFFSETR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA2_RCCAL_OFFSETR { bits }
    }
    #[doc = "Bits 20:24 - BBA2 RC Calibration manual value"]
    #[inline]
    pub fn bba2_rccal_manual(&self) -> BBA2_RCCAL_MANUALR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBA2_RCCAL_MANUALR { bits }
    }
    #[doc = "Bit 25 - BBA2 RC Calibration Disable"]
    #[inline]
    pub fn bba2_rccal_dis(&self) -> BBA2_RCCAL_DISR {
        BBA2_RCCAL_DISR::_from({
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
    #[doc = "Bits 0:3 - ADC RC Calibration value offset"]
    #[inline]
    pub fn adc_rccal_offset(&mut self) -> _ADC_RCCAL_OFFSETW {
        _ADC_RCCAL_OFFSETW { w: self }
    }
    #[doc = "Bits 4:8 - ADC RC Calibration manual value"]
    #[inline]
    pub fn adc_rccal_manual(&mut self) -> _ADC_RCCAL_MANUALW {
        _ADC_RCCAL_MANUALW { w: self }
    }
    #[doc = "Bit 9 - ADC RC Calibration Disable"]
    #[inline]
    pub fn adc_rccal_dis(&mut self) -> _ADC_RCCAL_DISW {
        _ADC_RCCAL_DISW { w: self }
    }
    #[doc = "Bits 16:19 - BBA2 RC Calibration value offset"]
    #[inline]
    pub fn bba2_rccal_offset(&mut self) -> _BBA2_RCCAL_OFFSETW {
        _BBA2_RCCAL_OFFSETW { w: self }
    }
    #[doc = "Bits 20:24 - BBA2 RC Calibration manual value"]
    #[inline]
    pub fn bba2_rccal_manual(&mut self) -> _BBA2_RCCAL_MANUALW {
        _BBA2_RCCAL_MANUALW { w: self }
    }
    #[doc = "Bit 25 - BBA2 RC Calibration Disable"]
    #[inline]
    pub fn bba2_rccal_dis(&mut self) -> _BBA2_RCCAL_DISW {
        _BBA2_RCCAL_DISW { w: self }
    }
}
