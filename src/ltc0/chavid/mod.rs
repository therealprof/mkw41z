#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CHAVID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct AESREVR {
    bits: u8,
}
impl AESREVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AESVIDR {
    bits: u8,
}
impl AESVIDR {
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
    #[doc = "Bits 0:3 - AES Revision Number"]
    #[inline]
    pub fn aesrev(&self) -> AESREVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AESREVR { bits }
    }
    #[doc = "Bits 4:7 - AES Version ID"]
    #[inline]
    pub fn aesvid(&self) -> AESVIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AESVIDR { bits }
    }
}
