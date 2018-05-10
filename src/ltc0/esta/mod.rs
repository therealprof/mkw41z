#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ESTA {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `ERRID1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRID1R {
    #[doc = "Mode Error"]
    _0001,
    #[doc = "Data Size Error"]
    _0010,
    #[doc = "Key Size Error"]
    _0011,
    #[doc = "Data Arrived out of Sequence Error"]
    _0110,
    #[doc = "ICV Check Failed"]
    _1010,
    #[doc = "Internal Hardware Failure"]
    _1011,
    #[doc = "CCM AAD Size Error (either 1. AAD flag in B0 =1 and no AAD type provided, 2. AAD flag in B0 = 0 and AAD povided, or 3. AAD flag in B0 =1 and not enough AAD provided - expecting more based on AAD size.)"]
    _1100,
    #[doc = "Invalid Crypto Engine Selected"]
    _1111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ERRID1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ERRID1R::_0001 => 1,
            ERRID1R::_0010 => 2,
            ERRID1R::_0011 => 3,
            ERRID1R::_0110 => 6,
            ERRID1R::_1010 => 10,
            ERRID1R::_1011 => 11,
            ERRID1R::_1100 => 12,
            ERRID1R::_1111 => 15,
            ERRID1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ERRID1R {
        match value {
            1 => ERRID1R::_0001,
            2 => ERRID1R::_0010,
            3 => ERRID1R::_0011,
            6 => ERRID1R::_0110,
            10 => ERRID1R::_1010,
            11 => ERRID1R::_1011,
            12 => ERRID1R::_1100,
            15 => ERRID1R::_1111,
            i => ERRID1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == ERRID1R::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == ERRID1R::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == ERRID1R::_0011
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == ERRID1R::_0110
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == ERRID1R::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == ERRID1R::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == ERRID1R::_1100
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == ERRID1R::_1111
    }
}
#[doc = "Possible values of the field `CL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CL1R {
    #[doc = "General Error"]
    _0000,
    #[doc = "AES"]
    _0001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CL1R::_0000 => 0,
            CL1R::_0001 => 1,
            CL1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CL1R {
        match value {
            0 => CL1R::_0000,
            1 => CL1R::_0001,
            i => CL1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == CL1R::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == CL1R::_0001
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Error ID 1"]
    #[inline]
    pub fn errid1(&self) -> ERRID1R {
        ERRID1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - algorithms"]
    #[inline]
    pub fn cl1(&self) -> CL1R {
        CL1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
