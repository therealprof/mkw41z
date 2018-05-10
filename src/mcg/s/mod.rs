#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::S {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `IRCST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCSTR {
    #[doc = "Source of internal reference clock is the slow clock (32 kHz IRC)."]
    _0,
    #[doc = "Source of internal reference clock is the fast clock (4 MHz IRC)."]
    _1,
}
impl IRCSTR {
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
            IRCSTR::_0 => false,
            IRCSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRCSTR {
        match value {
            false => IRCSTR::_0,
            true => IRCSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IRCSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IRCSTR::_1
    }
}
#[doc = r" Value of the field"]
pub struct OSCINIT0R {
    bits: bool,
}
impl OSCINIT0R {
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
#[doc = "Possible values of the field `CLKST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSTR {
    #[doc = "Encoding 0 - Output of the FLL is selected (reset default)."]
    _00,
    #[doc = "Encoding 1 - Internal reference clock is selected."]
    _01,
    #[doc = "Encoding 2 - External reference clock is selected."]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSTR::_00 => 0,
            CLKSTR::_01 => 1,
            CLKSTR::_10 => 2,
            CLKSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSTR {
        match value {
            0 => CLKSTR::_00,
            1 => CLKSTR::_01,
            2 => CLKSTR::_10,
            i => CLKSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == CLKSTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == CLKSTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == CLKSTR::_10
    }
}
#[doc = "Possible values of the field `IREFST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREFSTR {
    #[doc = "Source of FLL reference clock is the external reference clock."]
    _0,
    #[doc = "Source of FLL reference clock is the internal reference clock."]
    _1,
}
impl IREFSTR {
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
            IREFSTR::_0 => false,
            IREFSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IREFSTR {
        match value {
            false => IREFSTR::_0,
            true => IREFSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IREFSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IREFSTR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Internal Reference Clock Status"]
    #[inline]
    pub fn ircst(&self) -> IRCSTR {
        IRCSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - OSC Initialization"]
    #[inline]
    pub fn oscinit0(&self) -> OSCINIT0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        OSCINIT0R { bits }
    }
    #[doc = "Bits 2:3 - Clock Mode Status"]
    #[inline]
    pub fn clkst(&self) -> CLKSTR {
        CLKSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 4 - Internal Reference Status"]
    #[inline]
    pub fn irefst(&self) -> IREFSTR {
        IREFSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
