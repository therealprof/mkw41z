#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SDID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PINID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINIDR {
    #[doc = "48-pin"]
    _0100,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PINIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINIDR::_0100 => 4,
            PINIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINIDR {
        match value {
            4 => PINIDR::_0100,
            i => PINIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == PINIDR::_0100
    }
}
#[doc = r" Value of the field"]
pub struct DIEIDR {
    bits: u8,
}
impl DIEIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REVIDR {
    bits: u8,
}
impl REVIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SRAMSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMSIZER {
    #[doc = "128 KB"]
    _1001,
    #[doc = "64 KB"]
    _0111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRAMSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRAMSIZER::_1001 => 9,
            SRAMSIZER::_0111 => 7,
            SRAMSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRAMSIZER {
        match value {
            9 => SRAMSIZER::_1001,
            7 => SRAMSIZER::_0111,
            i => SRAMSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == SRAMSIZER::_1001
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == SRAMSIZER::_0111
    }
}
#[doc = "Possible values of the field `SERIESID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SERIESIDR {
    #[doc = "KW family"]
    _0101,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SERIESIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SERIESIDR::_0101 => 5,
            SERIESIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SERIESIDR {
        match value {
            5 => SERIESIDR::_0101,
            i => SERIESIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == SERIESIDR::_0101
    }
}
#[doc = "Possible values of the field `SUBFAMID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBFAMIDR {
    #[doc = "KWx0 Subfamily"]
    _00,
    #[doc = "KWx1 Subfamily"]
    _01,
    #[doc = "KWx2 Subfamily"]
    _10,
    #[doc = "KWx3 Subfamily"]
    _11,
}
impl SUBFAMIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SUBFAMIDR::_00 => 0,
            SUBFAMIDR::_01 => 1,
            SUBFAMIDR::_10 => 2,
            SUBFAMIDR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SUBFAMIDR {
        match value {
            0 => SUBFAMIDR::_00,
            1 => SUBFAMIDR::_01,
            2 => SUBFAMIDR::_10,
            3 => SUBFAMIDR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == SUBFAMIDR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == SUBFAMIDR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SUBFAMIDR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == SUBFAMIDR::_11
    }
}
#[doc = "Possible values of the field `FAMID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAMIDR {
    #[doc = "KW2x Family (802.15.4)"]
    _0010,
    #[doc = "KW3x Family (BTLE)"]
    _0011,
    #[doc = "KW4x Family (802.15.4, BTLE, GFSK , ANT)"]
    _0100,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FAMIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FAMIDR::_0010 => 2,
            FAMIDR::_0011 => 3,
            FAMIDR::_0100 => 4,
            FAMIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FAMIDR {
        match value {
            2 => FAMIDR::_0010,
            3 => FAMIDR::_0011,
            4 => FAMIDR::_0100,
            i => FAMIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == FAMIDR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == FAMIDR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == FAMIDR::_0100
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Pin count Identification"]
    #[inline]
    pub fn pinid(&self) -> PINIDR {
        PINIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:11 - Device Die Number"]
    #[inline]
    pub fn dieid(&self) -> DIEIDR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIEIDR { bits }
    }
    #[doc = "Bits 12:15 - Device Revision Number"]
    #[inline]
    pub fn revid(&self) -> REVIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REVIDR { bits }
    }
    #[doc = "Bits 16:19 - System SRAM Size"]
    #[inline]
    pub fn sramsize(&self) -> SRAMSIZER {
        SRAMSIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Kinetis Series ID"]
    #[inline]
    pub fn seriesid(&self) -> SERIESIDR {
        SERIESIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Kinetis Sub-Family ID."]
    #[inline]
    pub fn subfamid(&self) -> SUBFAMIDR {
        SUBFAMIDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Kinetis family ID"]
    #[inline]
    pub fn famid(&self) -> FAMIDR {
        FAMIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
