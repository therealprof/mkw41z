#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TMR_PRESCALE {
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
#[doc = "Possible values of the field `TMR_PRESCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR_PRESCALER {
    #[doc = "500kHz (33.55 S)"]
    _010,
    #[doc = "250kHz (67.11 S)"]
    _011,
    #[doc = "125kHz (134.22 S)"]
    _100,
    #[doc = "62.5kHz (268.44 S) -- default"]
    _101,
    #[doc = "31.25kHz (536.87 S)"]
    _110,
    #[doc = "15.625kHz (1073.74 S)"]
    _111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TMR_PRESCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TMR_PRESCALER::_010 => 2,
            TMR_PRESCALER::_011 => 3,
            TMR_PRESCALER::_100 => 4,
            TMR_PRESCALER::_101 => 5,
            TMR_PRESCALER::_110 => 6,
            TMR_PRESCALER::_111 => 7,
            TMR_PRESCALER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TMR_PRESCALER {
        match value {
            2 => TMR_PRESCALER::_010,
            3 => TMR_PRESCALER::_011,
            4 => TMR_PRESCALER::_100,
            5 => TMR_PRESCALER::_101,
            6 => TMR_PRESCALER::_110,
            7 => TMR_PRESCALER::_111,
            i => TMR_PRESCALER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == TMR_PRESCALER::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == TMR_PRESCALER::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == TMR_PRESCALER::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == TMR_PRESCALER::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == TMR_PRESCALER::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == TMR_PRESCALER::_111
    }
}
#[doc = "Values that can be written to the field `TMR_PRESCALE`"]
pub enum TMR_PRESCALEW {
    #[doc = "500kHz (33.55 S)"]
    _010,
    #[doc = "250kHz (67.11 S)"]
    _011,
    #[doc = "125kHz (134.22 S)"]
    _100,
    #[doc = "62.5kHz (268.44 S) -- default"]
    _101,
    #[doc = "31.25kHz (536.87 S)"]
    _110,
    #[doc = "15.625kHz (1073.74 S)"]
    _111,
}
impl TMR_PRESCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TMR_PRESCALEW::_010 => 2,
            TMR_PRESCALEW::_011 => 3,
            TMR_PRESCALEW::_100 => 4,
            TMR_PRESCALEW::_101 => 5,
            TMR_PRESCALEW::_110 => 6,
            TMR_PRESCALEW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMR_PRESCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_PRESCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMR_PRESCALEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "500kHz (33.55 S)"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(TMR_PRESCALEW::_010)
    }
    #[doc = "250kHz (67.11 S)"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(TMR_PRESCALEW::_011)
    }
    #[doc = "125kHz (134.22 S)"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(TMR_PRESCALEW::_100)
    }
    #[doc = "62.5kHz (268.44 S) -- default"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(TMR_PRESCALEW::_101)
    }
    #[doc = "31.25kHz (536.87 S)"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(TMR_PRESCALEW::_110)
    }
    #[doc = "15.625kHz (1073.74 S)"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(TMR_PRESCALEW::_111)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Timer Prescaler"]
    #[inline]
    pub fn tmr_prescale(&self) -> TMR_PRESCALER {
        TMR_PRESCALER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 5 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Timer Prescaler"]
    #[inline]
    pub fn tmr_prescale(&mut self) -> _TMR_PRESCALEW {
        _TMR_PRESCALEW { w: self }
    }
}
