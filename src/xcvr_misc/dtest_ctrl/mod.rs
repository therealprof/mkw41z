#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DTEST_CTRL {
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
pub struct DTEST_PAGER {
    bits: u8,
}
impl DTEST_PAGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DTEST_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEST_ENR {
    #[doc = "Disables DTEST. The DTEST pins assume their mission function."]
    _0,
    #[doc = "Enables DTEST. The contents of the selected page (DTEST_PAGE) will appear on the DTEST output pins."]
    _1,
}
impl DTEST_ENR {
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
            DTEST_ENR::_0 => false,
            DTEST_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTEST_ENR {
        match value {
            false => DTEST_ENR::_0,
            true => DTEST_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DTEST_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DTEST_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct GPIO0_OVLAY_PINR {
    bits: u8,
}
impl GPIO0_OVLAY_PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO1_OVLAY_PINR {
    bits: u8,
}
impl GPIO1_OVLAY_PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TSM_GPIO_OVLAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_GPIO_OVLAYR {
    #[doc = "there is no overlay, and the DTEST Page Table dictates the node that appears on each DTEST pin."]
    _00,
    #[doc = "the register GPIO0_OVLAY_PIN[3:0] selects the DTEST pin on which GPIO0_TRIG_EN will appear."]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSM_GPIO_OVLAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSM_GPIO_OVLAYR::_00 => 0,
            TSM_GPIO_OVLAYR::_01 => 1,
            TSM_GPIO_OVLAYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSM_GPIO_OVLAYR {
        match value {
            0 => TSM_GPIO_OVLAYR::_00,
            1 => TSM_GPIO_OVLAYR::_01,
            i => TSM_GPIO_OVLAYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TSM_GPIO_OVLAYR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TSM_GPIO_OVLAYR::_01
    }
}
#[doc = r" Value of the field"]
pub struct DTEST_SHFTR {
    bits: u8,
}
impl DTEST_SHFTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RAW_MODE_IR {
    bits: bool,
}
impl RAW_MODE_IR {
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
pub struct RAW_MODE_QR {
    bits: bool,
}
impl RAW_MODE_QR {
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
pub struct _DTEST_PAGEW<'a> {
    w: &'a mut W,
}
impl<'a> _DTEST_PAGEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTEST_EN`"]
pub enum DTEST_ENW {
    #[doc = "Disables DTEST. The DTEST pins assume their mission function."]
    _0,
    #[doc = "Enables DTEST. The contents of the selected page (DTEST_PAGE) will appear on the DTEST output pins."]
    _1,
}
impl DTEST_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTEST_ENW::_0 => false,
            DTEST_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTEST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTEST_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTEST_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables DTEST. The DTEST pins assume their mission function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEST_ENW::_0)
    }
    #[doc = "Enables DTEST. The contents of the selected page (DTEST_PAGE) will appear on the DTEST output pins."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEST_ENW::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO0_OVLAY_PINW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO0_OVLAY_PINW<'a> {
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
pub struct _GPIO1_OVLAY_PINW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO1_OVLAY_PINW<'a> {
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
#[doc = "Values that can be written to the field `TSM_GPIO_OVLAY`"]
pub enum TSM_GPIO_OVLAYW {
    #[doc = "there is no overlay, and the DTEST Page Table dictates the node that appears on each DTEST pin."]
    _00,
    #[doc = "the register GPIO0_OVLAY_PIN[3:0] selects the DTEST pin on which GPIO0_TRIG_EN will appear."]
    _01,
}
impl TSM_GPIO_OVLAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSM_GPIO_OVLAYW::_00 => 0,
            TSM_GPIO_OVLAYW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSM_GPIO_OVLAYW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_GPIO_OVLAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSM_GPIO_OVLAYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "there is no overlay, and the DTEST Page Table dictates the node that appears on each DTEST pin."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TSM_GPIO_OVLAYW::_00)
    }
    #[doc = "the register GPIO0_OVLAY_PIN[3:0] selects the DTEST pin on which GPIO0_TRIG_EN will appear."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TSM_GPIO_OVLAYW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTEST_SHFTW<'a> {
    w: &'a mut W,
}
impl<'a> _DTEST_SHFTW<'a> {
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
pub struct _RAW_MODE_IW<'a> {
    w: &'a mut W,
}
impl<'a> _RAW_MODE_IW<'a> {
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
pub struct _RAW_MODE_QW<'a> {
    w: &'a mut W,
}
impl<'a> _RAW_MODE_QW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - DTEST Page Selector"]
    #[inline]
    pub fn dtest_page(&self) -> DTEST_PAGER {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DTEST_PAGER { bits }
    }
    #[doc = "Bit 7 - DTEST Enable"]
    #[inline]
    pub fn dtest_en(&self) -> DTEST_ENR {
        DTEST_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - GPIO 0 Overlay Pin"]
    #[inline]
    pub fn gpio0_ovlay_pin(&self) -> GPIO0_OVLAY_PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO0_OVLAY_PINR { bits }
    }
    #[doc = "Bits 12:15 - GPIO 1 Overlay Pin"]
    #[inline]
    pub fn gpio1_ovlay_pin(&self) -> GPIO1_OVLAY_PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO1_OVLAY_PINR { bits }
    }
    #[doc = "Bits 16:17 - TSM GPIO Overlay Pin Control"]
    #[inline]
    pub fn tsm_gpio_ovlay(&self) -> TSM_GPIO_OVLAYR {
        TSM_GPIO_OVLAYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - DTEST Shift Control"]
    #[inline]
    pub fn dtest_shft(&self) -> DTEST_SHFTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DTEST_SHFTR { bits }
    }
    #[doc = "Bit 28 - DTEST Raw Mode Enable for I Channel"]
    #[inline]
    pub fn raw_mode_i(&self) -> RAW_MODE_IR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RAW_MODE_IR { bits }
    }
    #[doc = "Bit 29 - DTEST Raw Mode Enable for Q Channel"]
    #[inline]
    pub fn raw_mode_q(&self) -> RAW_MODE_QR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RAW_MODE_QR { bits }
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
    #[doc = "Bits 0:5 - DTEST Page Selector"]
    #[inline]
    pub fn dtest_page(&mut self) -> _DTEST_PAGEW {
        _DTEST_PAGEW { w: self }
    }
    #[doc = "Bit 7 - DTEST Enable"]
    #[inline]
    pub fn dtest_en(&mut self) -> _DTEST_ENW {
        _DTEST_ENW { w: self }
    }
    #[doc = "Bits 8:11 - GPIO 0 Overlay Pin"]
    #[inline]
    pub fn gpio0_ovlay_pin(&mut self) -> _GPIO0_OVLAY_PINW {
        _GPIO0_OVLAY_PINW { w: self }
    }
    #[doc = "Bits 12:15 - GPIO 1 Overlay Pin"]
    #[inline]
    pub fn gpio1_ovlay_pin(&mut self) -> _GPIO1_OVLAY_PINW {
        _GPIO1_OVLAY_PINW { w: self }
    }
    #[doc = "Bits 16:17 - TSM GPIO Overlay Pin Control"]
    #[inline]
    pub fn tsm_gpio_ovlay(&mut self) -> _TSM_GPIO_OVLAYW {
        _TSM_GPIO_OVLAYW { w: self }
    }
    #[doc = "Bits 24:26 - DTEST Shift Control"]
    #[inline]
    pub fn dtest_shft(&mut self) -> _DTEST_SHFTW {
        _DTEST_SHFTW { w: self }
    }
    #[doc = "Bit 28 - DTEST Raw Mode Enable for I Channel"]
    #[inline]
    pub fn raw_mode_i(&mut self) -> _RAW_MODE_IW {
        _RAW_MODE_IW { w: self }
    }
    #[doc = "Bit 29 - DTEST Raw Mode Enable for Q Channel"]
    #[inline]
    pub fn raw_mode_q(&mut self) -> _RAW_MODE_QW {
        _RAW_MODE_QW { w: self }
    }
}
