#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FCFG2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct MAXADDR1R {
    bits: u8,
}
impl MAXADDR1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAXADDR0R {
    bits: u8,
}
impl MAXADDR0R {
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
    #[doc = "Bits 16:22 - This field concatenated with leading zeros plus the value of the MAXADDR1 field indicates the first invalid address of the second program flash block (flash block 1)"]
    #[inline]
    pub fn maxaddr1(&self) -> MAXADDR1R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAXADDR1R { bits }
    }
    #[doc = "Bits 24:30 - Max Address lock"]
    #[inline]
    pub fn maxaddr0(&self) -> MAXADDR0R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAXADDR0R { bits }
    }
}
