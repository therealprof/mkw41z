#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SAM_MATCH {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SAP0_MATCHR {
    bits: u8,
}
impl SAP0_MATCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAP0_ADDR_PRESENTR {
    bits: bool,
}
impl SAP0_ADDR_PRESENTR {
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
pub struct SAA0_MATCHR {
    bits: u8,
}
impl SAA0_MATCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAA0_ADDR_ABSENTR {
    bits: bool,
}
impl SAA0_ADDR_ABSENTR {
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
pub struct SAP1_MATCHR {
    bits: u8,
}
impl SAP1_MATCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAP1_ADDR_PRESENTR {
    bits: bool,
}
impl SAP1_ADDR_PRESENTR {
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
pub struct SAA1_MATCHR {
    bits: u8,
}
impl SAA1_MATCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAA1_ADDR_ABSENTR {
    bits: bool,
}
impl SAA1_ADDR_ABSENTR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Index in the SAP0 Partition of the SAM Table corresponding to the first checksum match"]
    #[inline]
    pub fn sap0_match(&self) -> SAP0_MATCHR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAP0_MATCHR { bits }
    }
    #[doc = "Bit 7 - A Checksum Match is Present in the SAP0 Partition of the SAM Table"]
    #[inline]
    pub fn sap0_addr_present(&self) -> SAP0_ADDR_PRESENTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SAP0_ADDR_PRESENTR { bits }
    }
    #[doc = "Bits 8:14 - Index in the SAA0 Partition of the SAM Table corresponding to the first checksum match"]
    #[inline]
    pub fn saa0_match(&self) -> SAA0_MATCHR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAA0_MATCHR { bits }
    }
    #[doc = "Bit 15 - A Checksum Match is Absent in the SAA0 Partition of the SAM Table"]
    #[inline]
    pub fn saa0_addr_absent(&self) -> SAA0_ADDR_ABSENTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SAA0_ADDR_ABSENTR { bits }
    }
    #[doc = "Bits 16:22 - Index in the SAP1 Partition of the SAM Table corresponding to the first checksum match"]
    #[inline]
    pub fn sap1_match(&self) -> SAP1_MATCHR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAP1_MATCHR { bits }
    }
    #[doc = "Bit 23 - A Checksum Match is Present in the SAP1 Partition of the SAM Table"]
    #[inline]
    pub fn sap1_addr_present(&self) -> SAP1_ADDR_PRESENTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SAP1_ADDR_PRESENTR { bits }
    }
    #[doc = "Bits 24:30 - Index in the SAA1 Partition of the SAM Table corresponding to the first checksum match"]
    #[inline]
    pub fn saa1_match(&self) -> SAA1_MATCHR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAA1_MATCHR { bits }
    }
    #[doc = "Bit 31 - A Checksum Match is Absent in the SAP1 Partition of the SAM Table"]
    #[inline]
    pub fn saa1_addr_absent(&self) -> SAA1_ADDR_ABSENTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SAA1_ADDR_ABSENTR { bits }
    }
}
