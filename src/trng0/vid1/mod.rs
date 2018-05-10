#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::VID1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `MIN_REV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIN_REVR {
    #[doc = "Minor revision number for TRNG."]
    _0X00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MIN_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MIN_REVR::_0X00 => 0,
            MIN_REVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MIN_REVR {
        match value {
            0 => MIN_REVR::_0X00,
            i => MIN_REVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline]
    pub fn is_0x00(&self) -> bool {
        *self == MIN_REVR::_0X00
    }
}
#[doc = "Possible values of the field `MAJ_REV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAJ_REVR {
    #[doc = "Major revision number for TRNG."]
    _0X01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MAJ_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MAJ_REVR::_0X01 => 1,
            MAJ_REVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MAJ_REVR {
        match value {
            1 => MAJ_REVR::_0X01,
            i => MAJ_REVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline]
    pub fn is_0x01(&self) -> bool {
        *self == MAJ_REVR::_0X01
    }
}
#[doc = "Possible values of the field `IP_ID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IP_IDR {
    #[doc = "ID for TRNG."]
    _0X0030,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl IP_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            IP_IDR::_0X0030 => 48,
            IP_IDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> IP_IDR {
        match value {
            48 => IP_IDR::_0X0030,
            i => IP_IDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X0030`"]
    #[inline]
    pub fn is_0x0030(&self) -> bool {
        *self == IP_IDR::_0X0030
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Shows the Freescale IP's Minor revision of the TRNG."]
    #[inline]
    pub fn min_rev(&self) -> MIN_REVR {
        MIN_REVR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Shows the Freescale IP's Major revision of the TRNG."]
    #[inline]
    pub fn maj_rev(&self) -> MAJ_REVR {
        MAJ_REVR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:31 - Shows the Freescale IP ID."]
    #[inline]
    pub fn ip_id(&self) -> IP_IDR {
        IP_IDR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
