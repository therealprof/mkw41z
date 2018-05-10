#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT5 {
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
#[doc = "Possible values of the field `LPUART0TXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART0TXSRCR {
    #[doc = "LPUART0_TX pin"]
    _00,
    #[doc = "LPUART0_TX pin modulated with TPM1 channel 0 output"]
    _01,
    #[doc = "LPUART0_TX pin modulated with TPM2 channel 0 output"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPUART0TXSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPUART0TXSRCR::_00 => 0,
            LPUART0TXSRCR::_01 => 1,
            LPUART0TXSRCR::_10 => 2,
            LPUART0TXSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPUART0TXSRCR {
        match value {
            0 => LPUART0TXSRCR::_00,
            1 => LPUART0TXSRCR::_01,
            2 => LPUART0TXSRCR::_10,
            i => LPUART0TXSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LPUART0TXSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LPUART0TXSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LPUART0TXSRCR::_10
    }
}
#[doc = "Possible values of the field `LPUART0RXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART0RXSRCR {
    #[doc = "LPUART_RX pin"]
    _0,
    #[doc = "CMP0 output"]
    _1,
}
impl LPUART0RXSRCR {
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
            LPUART0RXSRCR::_0 => false,
            LPUART0RXSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART0RXSRCR {
        match value {
            false => LPUART0RXSRCR::_0,
            true => LPUART0RXSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPUART0RXSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPUART0RXSRCR::_1
    }
}
#[doc = "Possible values of the field `LPUART0ODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART0ODER {
    #[doc = "Open drain is disabled on LPUART0."]
    _0,
    #[doc = "Open drain is enabled on LPUART0."]
    _1,
}
impl LPUART0ODER {
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
            LPUART0ODER::_0 => false,
            LPUART0ODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART0ODER {
        match value {
            false => LPUART0ODER::_0,
            true => LPUART0ODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPUART0ODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPUART0ODER::_1
    }
}
#[doc = "Values that can be written to the field `LPUART0TXSRC`"]
pub enum LPUART0TXSRCW {
    #[doc = "LPUART0_TX pin"]
    _00,
    #[doc = "LPUART0_TX pin modulated with TPM1 channel 0 output"]
    _01,
    #[doc = "LPUART0_TX pin modulated with TPM2 channel 0 output"]
    _10,
}
impl LPUART0TXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPUART0TXSRCW::_00 => 0,
            LPUART0TXSRCW::_01 => 1,
            LPUART0TXSRCW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART0TXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART0TXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART0TXSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LPUART0_TX pin"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART0TXSRCW::_00)
    }
    #[doc = "LPUART0_TX pin modulated with TPM1 channel 0 output"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART0TXSRCW::_01)
    }
    #[doc = "LPUART0_TX pin modulated with TPM2 channel 0 output"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUART0TXSRCW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART0RXSRC`"]
pub enum LPUART0RXSRCW {
    #[doc = "LPUART_RX pin"]
    _0,
    #[doc = "CMP0 output"]
    _1,
}
impl LPUART0RXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART0RXSRCW::_0 => false,
            LPUART0RXSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART0RXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART0RXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART0RXSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPUART_RX pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPUART0RXSRCW::_0)
    }
    #[doc = "CMP0 output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPUART0RXSRCW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART0ODE`"]
pub enum LPUART0ODEW {
    #[doc = "Open drain is disabled on LPUART0."]
    _0,
    #[doc = "Open drain is enabled on LPUART0."]
    _1,
}
impl LPUART0ODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART0ODEW::_0 => false,
            LPUART0ODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART0ODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART0ODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART0ODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Open drain is disabled on LPUART0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPUART0ODEW::_0)
    }
    #[doc = "Open drain is enabled on LPUART0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPUART0ODEW::_1)
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:1 - LPUART0 Transmit Data Source Select"]
    #[inline]
    pub fn lpuart0txsrc(&self) -> LPUART0TXSRCR {
        LPUART0TXSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - LPUART0 Receive Data Source Select"]
    #[inline]
    pub fn lpuart0rxsrc(&self) -> LPUART0RXSRCR {
        LPUART0RXSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - LPUART0 Open Drain Enable"]
    #[inline]
    pub fn lpuart0ode(&self) -> LPUART0ODER {
        LPUART0ODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:1 - LPUART0 Transmit Data Source Select"]
    #[inline]
    pub fn lpuart0txsrc(&mut self) -> _LPUART0TXSRCW {
        _LPUART0TXSRCW { w: self }
    }
    #[doc = "Bit 2 - LPUART0 Receive Data Source Select"]
    #[inline]
    pub fn lpuart0rxsrc(&mut self) -> _LPUART0RXSRCW {
        _LPUART0RXSRCW { w: self }
    }
    #[doc = "Bit 16 - LPUART0 Open Drain Enable"]
    #[inline]
    pub fn lpuart0ode(&mut self) -> _LPUART0ODEW {
        _LPUART0ODEW { w: self }
    }
}
