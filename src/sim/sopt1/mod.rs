#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT1 {
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
#[doc = "Possible values of the field `OSC32KOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC32KOUTR {
    #[doc = "ERCLK32K is not output."]
    _00,
    #[doc = "ERCLK32K is output on PTB3."]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSC32KOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSC32KOUTR::_00 => 0,
            OSC32KOUTR::_01 => 1,
            OSC32KOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSC32KOUTR {
        match value {
            0 => OSC32KOUTR::_00,
            1 => OSC32KOUTR::_01,
            i => OSC32KOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == OSC32KOUTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == OSC32KOUTR::_01
    }
}
#[doc = "Possible values of the field `OSC32KSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC32KSELR {
    #[doc = "32kHz oscillator (OSC32KCLK)"]
    _00,
    #[doc = "RTC_CLKIN"]
    _10,
    #[doc = "LPO 1kHz"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSC32KSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSC32KSELR::_00 => 0,
            OSC32KSELR::_10 => 2,
            OSC32KSELR::_11 => 3,
            OSC32KSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSC32KSELR {
        match value {
            0 => OSC32KSELR::_00,
            2 => OSC32KSELR::_10,
            3 => OSC32KSELR::_11,
            i => OSC32KSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == OSC32KSELR::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == OSC32KSELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == OSC32KSELR::_11
    }
}
#[doc = "Values that can be written to the field `OSC32KOUT`"]
pub enum OSC32KOUTW {
    #[doc = "ERCLK32K is not output."]
    _00,
    #[doc = "ERCLK32K is output on PTB3."]
    _01,
}
impl OSC32KOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSC32KOUTW::_00 => 0,
            OSC32KOUTW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSC32KOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _OSC32KOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSC32KOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ERCLK32K is not output."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSC32KOUTW::_00)
    }
    #[doc = "ERCLK32K is output on PTB3."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(OSC32KOUTW::_01)
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
#[doc = "Values that can be written to the field `OSC32KSEL`"]
pub enum OSC32KSELW {
    #[doc = "32kHz oscillator (OSC32KCLK)"]
    _00,
    #[doc = "RTC_CLKIN"]
    _10,
    #[doc = "LPO 1kHz"]
    _11,
}
impl OSC32KSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSC32KSELW::_00 => 0,
            OSC32KSELW::_10 => 2,
            OSC32KSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSC32KSELW<'a> {
    w: &'a mut W,
}
impl<'a> _OSC32KSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSC32KSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "32kHz oscillator (OSC32KCLK)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSC32KSELW::_00)
    }
    #[doc = "RTC_CLKIN"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(OSC32KSELW::_10)
    }
    #[doc = "LPO 1kHz"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(OSC32KSELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
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
    #[doc = "Bits 16:17 - 32K oscillator clock output"]
    #[inline]
    pub fn osc32kout(&self) -> OSC32KOUTR {
        OSC32KOUTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - 32K Oscillator Clock Select"]
    #[inline]
    pub fn osc32ksel(&self) -> OSC32KSELR {
        OSC32KSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 36864 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 16:17 - 32K oscillator clock output"]
    #[inline]
    pub fn osc32kout(&mut self) -> _OSC32KOUTW {
        _OSC32KOUTW { w: self }
    }
    #[doc = "Bits 18:19 - 32K Oscillator Clock Select"]
    #[inline]
    pub fn osc32ksel(&mut self) -> _OSC32KSELW {
        _OSC32KSELW { w: self }
    }
}
