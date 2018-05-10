#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT2 {
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
#[doc = "Possible values of the field `CLKOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSELR {
    #[doc = "OSCERCLK DIV2"]
    _000,
    #[doc = "OSCERCLK DIV4"]
    _001,
    #[doc = "Bus clock"]
    _010,
    #[doc = "LPO clock 1 kHz"]
    _011,
    #[doc = "MCGIRCLK"]
    _100,
    #[doc = "OSCERCLK DIV8"]
    _101,
    #[doc = "OSCERCLK"]
    _110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKOUTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKOUTSELR::_000 => 0,
            CLKOUTSELR::_001 => 1,
            CLKOUTSELR::_010 => 2,
            CLKOUTSELR::_011 => 3,
            CLKOUTSELR::_100 => 4,
            CLKOUTSELR::_101 => 5,
            CLKOUTSELR::_110 => 6,
            CLKOUTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKOUTSELR {
        match value {
            0 => CLKOUTSELR::_000,
            1 => CLKOUTSELR::_001,
            2 => CLKOUTSELR::_010,
            3 => CLKOUTSELR::_011,
            4 => CLKOUTSELR::_100,
            5 => CLKOUTSELR::_101,
            6 => CLKOUTSELR::_110,
            i => CLKOUTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == CLKOUTSELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == CLKOUTSELR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == CLKOUTSELR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == CLKOUTSELR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTSELR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == CLKOUTSELR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == CLKOUTSELR::_110
    }
}
#[doc = "Possible values of the field `TPMSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPMSRCR {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK clock"]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl TPMSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TPMSRCR::_00 => 0,
            TPMSRCR::_01 => 1,
            TPMSRCR::_10 => 2,
            TPMSRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TPMSRCR {
        match value {
            0 => TPMSRCR::_00,
            1 => TPMSRCR::_01,
            2 => TPMSRCR::_10,
            3 => TPMSRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TPMSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TPMSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TPMSRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TPMSRCR::_11
    }
}
#[doc = "Possible values of the field `LPUART0SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART0SRCR {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK clock"]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl LPUART0SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPUART0SRCR::_00 => 0,
            LPUART0SRCR::_01 => 1,
            LPUART0SRCR::_10 => 2,
            LPUART0SRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPUART0SRCR {
        match value {
            0 => LPUART0SRCR::_00,
            1 => LPUART0SRCR::_01,
            2 => LPUART0SRCR::_10,
            3 => LPUART0SRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LPUART0SRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LPUART0SRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LPUART0SRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == LPUART0SRCR::_11
    }
}
#[doc = "Values that can be written to the field `CLKOUTSEL`"]
pub enum CLKOUTSELW {
    #[doc = "OSCERCLK DIV2"]
    _000,
    #[doc = "OSCERCLK DIV4"]
    _001,
    #[doc = "Bus clock"]
    _010,
    #[doc = "LPO clock 1 kHz"]
    _011,
    #[doc = "MCGIRCLK"]
    _100,
    #[doc = "OSCERCLK DIV8"]
    _101,
    #[doc = "OSCERCLK"]
    _110,
}
impl CLKOUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSELW::_000 => 0,
            CLKOUTSELW::_001 => 1,
            CLKOUTSELW::_010 => 2,
            CLKOUTSELW::_011 => 3,
            CLKOUTSELW::_100 => 4,
            CLKOUTSELW::_101 => 5,
            CLKOUTSELW::_110 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "OSCERCLK DIV2"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_000)
    }
    #[doc = "OSCERCLK DIV4"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_001)
    }
    #[doc = "Bus clock"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_010)
    }
    #[doc = "LPO clock 1 kHz"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_011)
    }
    #[doc = "MCGIRCLK"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_100)
    }
    #[doc = "OSCERCLK DIV8"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_101)
    }
    #[doc = "OSCERCLK"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_110)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPMSRC`"]
pub enum TPMSRCW {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK clock"]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl TPMSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TPMSRCW::_00 => 0,
            TPMSRCW::_01 => 1,
            TPMSRCW::_10 => 2,
            TPMSRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPMSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TPMSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPMSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPMSRCW::_00)
    }
    #[doc = "MCGFLLCLK clock"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPMSRCW::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPMSRCW::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TPMSRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART0SRC`"]
pub enum LPUART0SRCW {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK clock"]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl LPUART0SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPUART0SRCW::_00 => 0,
            LPUART0SRCW::_01 => 1,
            LPUART0SRCW::_10 => 2,
            LPUART0SRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART0SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART0SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART0SRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART0SRCW::_00)
    }
    #[doc = "MCGFLLCLK clock"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART0SRCW::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUART0SRCW::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(LPUART0SRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline]
    pub fn clkoutsel(&self) -> CLKOUTSELR {
        CLKOUTSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - TPM Clock Source Select"]
    #[inline]
    pub fn tpmsrc(&self) -> TPMSRCR {
        TPMSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - LPUART0 Clock Source Select"]
    #[inline]
    pub fn lpuart0src(&self) -> LPUART0SRCR {
        LPUART0SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline]
    pub fn clkoutsel(&mut self) -> _CLKOUTSELW {
        _CLKOUTSELW { w: self }
    }
    #[doc = "Bits 24:25 - TPM Clock Source Select"]
    #[inline]
    pub fn tpmsrc(&mut self) -> _TPMSRCW {
        _TPMSRCW { w: self }
    }
    #[doc = "Bits 26:27 - LPUART0 Clock Source Select"]
    #[inline]
    pub fn lpuart0src(&mut self) -> _LPUART0SRCW {
        _LPUART0SRCW { w: self }
    }
}
