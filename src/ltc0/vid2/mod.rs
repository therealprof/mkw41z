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
#[doc = r" Value of the field"]
pub struct ECO_REVR {
    bits: u8,
}
impl ECO_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ARCH_ERAR {
    bits: u8,
}
impl ARCH_ERAR {
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
    #[doc = "Bits 0:7 - ECO revision number."]
    #[inline]
    pub fn eco_rev(&self) -> ECO_REVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ECO_REVR { bits }
    }
    #[doc = "Bits 8:15 - Architectural ERA."]
    #[inline]
    pub fn arch_era(&self) -> ARCH_ERAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ARCH_ERAR { bits }
    }
}
