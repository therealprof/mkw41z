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
#[doc = r" Value of the field"]
pub struct MIN_REVR {
    bits: u8,
}
impl MIN_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAJ_REVR {
    bits: u8,
}
impl MAJ_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IP_IDR {
    bits: u16,
}
impl IP_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Minor revision number."]
    #[inline]
    pub fn min_rev(&self) -> MIN_REVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MIN_REVR { bits }
    }
    #[doc = "Bits 8:15 - Major revision number."]
    #[inline]
    pub fn maj_rev(&self) -> MAJ_REVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAJ_REVR { bits }
    }
    #[doc = "Bits 16:31 - ID(0x0034)."]
    #[inline]
    pub fn ip_id(&self) -> IP_IDR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        IP_IDR { bits }
    }
}
