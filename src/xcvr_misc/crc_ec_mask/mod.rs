#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CRC_EC_MASK {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CRC_EC_MASKR {
    bits: u32,
}
impl CRC_EC_MASKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - CRC error correction mask"]
    #[inline]
    pub fn crc_ec_mask(&self) -> CRC_EC_MASKR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        CRC_EC_MASKR { bits }
    }
}
