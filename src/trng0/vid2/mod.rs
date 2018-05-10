#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::VID2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `CONFIG_OPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONFIG_OPTR {
    #[doc = "TRNG_CONFIG_OPT for TRNG."]
    _0X00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CONFIG_OPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CONFIG_OPTR::_0X00 => 0,
            CONFIG_OPTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CONFIG_OPTR {
        match value {
            0 => CONFIG_OPTR::_0X00,
            i => CONFIG_OPTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline]
    pub fn is_0x00(&self) -> bool {
        *self == CONFIG_OPTR::_0X00
    }
}
#[doc = "Possible values of the field `ECO_REV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECO_REVR {
    #[doc = "TRNG_ECO_REV for TRNG."]
    _0X00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ECO_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ECO_REVR::_0X00 => 0,
            ECO_REVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ECO_REVR {
        match value {
            0 => ECO_REVR::_0X00,
            i => ECO_REVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline]
    pub fn is_0x00(&self) -> bool {
        *self == ECO_REVR::_0X00
    }
}
#[doc = "Possible values of the field `INTG_OPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTG_OPTR {
    #[doc = "INTG_OPT for TRNG."]
    _0X00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INTG_OPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INTG_OPTR::_0X00 => 0,
            INTG_OPTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INTG_OPTR {
        match value {
            0 => INTG_OPTR::_0X00,
            i => INTG_OPTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline]
    pub fn is_0x00(&self) -> bool {
        *self == INTG_OPTR::_0X00
    }
}
#[doc = "Possible values of the field `ERA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERAR {
    #[doc = "COMPILE_OPT for TRNG."]
    _0X00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ERAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ERAR::_0X00 => 0,
            ERAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ERAR {
        match value {
            0 => ERAR::_0X00,
            i => ERAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline]
    pub fn is_0x00(&self) -> bool {
        *self == ERAR::_0X00
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Shows the Freescale IP's Configuaration options for the TRNG."]
    #[inline]
    pub fn config_opt(&self) -> CONFIG_OPTR {
        CONFIG_OPTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Shows the Freescale IP's ECO revision of the TRNG."]
    #[inline]
    pub fn eco_rev(&self) -> ECO_REVR {
        ECO_REVR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Shows the Freescale integration options for the TRNG."]
    #[inline]
    pub fn intg_opt(&self) -> INTG_OPTR {
        INTG_OPTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - Shows the Freescale compile options for the TRNG."]
    #[inline]
    pub fn era(&self) -> ERAR {
        ERAR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
