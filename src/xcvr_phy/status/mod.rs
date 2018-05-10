#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PREAMBLE_FOUNDR {
    bits: bool,
}
impl PREAMBLE_FOUNDR {
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
pub struct AA_SFD_MATCHEDR {
    bits: bool,
}
impl AA_SFD_MATCHEDR {
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
#[doc = "Possible values of the field `AA_MATCHED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AA_MATCHEDR {
    #[doc = "No Network Address has matched"]
    _0000,
    #[doc = "Network Address 0 has matched"]
    _0001,
    #[doc = "Network Address 1 has matched"]
    _0010,
    #[doc = "Network Address 2 has matched"]
    _0100,
    #[doc = "Network Address 3 has matched"]
    _1000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AA_MATCHEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AA_MATCHEDR::_0000 => 0,
            AA_MATCHEDR::_0001 => 1,
            AA_MATCHEDR::_0010 => 2,
            AA_MATCHEDR::_0100 => 4,
            AA_MATCHEDR::_1000 => 8,
            AA_MATCHEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AA_MATCHEDR {
        match value {
            0 => AA_MATCHEDR::_0000,
            1 => AA_MATCHEDR::_0001,
            2 => AA_MATCHEDR::_0010,
            4 => AA_MATCHEDR::_0100,
            8 => AA_MATCHEDR::_1000,
            i => AA_MATCHEDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == AA_MATCHEDR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == AA_MATCHEDR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == AA_MATCHEDR::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == AA_MATCHEDR::_0100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == AA_MATCHEDR::_1000
    }
}
#[doc = r" Value of the field"]
pub struct HAMMING_DISTANCER {
    bits: u8,
}
impl HAMMING_DISTANCER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA_FIFO_DEPTHR {
    bits: u8,
}
impl DATA_FIFO_DEPTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CFO_ESTIMATER {
    bits: u8,
}
impl CFO_ESTIMATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Preamble Found"]
    #[inline]
    pub fn preamble_found(&self) -> PREAMBLE_FOUNDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PREAMBLE_FOUNDR { bits }
    }
    #[doc = "Bit 1 - Access Address or SFD Found"]
    #[inline]
    pub fn aa_sfd_matched(&self) -> AA_SFD_MATCHEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AA_SFD_MATCHEDR { bits }
    }
    #[doc = "Bits 4:7 - Access Address Matched"]
    #[inline]
    pub fn aa_matched(&self) -> AA_MATCHEDR {
        AA_MATCHEDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - HAMMING DISTANCE"]
    #[inline]
    pub fn hamming_distance(&self) -> HAMMING_DISTANCER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HAMMING_DISTANCER { bits }
    }
    #[doc = "Bits 12:15 - DATA FIFO DEPTH"]
    #[inline]
    pub fn data_fifo_depth(&self) -> DATA_FIFO_DEPTHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_FIFO_DEPTHR { bits }
    }
    #[doc = "Bits 16:23 - Carrier Frequency Offset Estimate"]
    #[inline]
    pub fn cfo_estimate(&self) -> CFO_ESTIMATER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CFO_ESTIMATER { bits }
    }
}
