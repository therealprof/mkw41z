#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RSSI_DFT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DFT_MAGR {
    bits: u16,
}
impl DFT_MAGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DFT_NOISER {
    bits: u16,
}
impl DFT_NOISER {
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
    #[doc = "Bits 0:12 - RSSI MAG"]
    #[inline]
    pub fn dft_mag(&self) -> DFT_MAGR {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DFT_MAGR { bits }
    }
    #[doc = "Bits 16:28 - RSSI MAG"]
    #[inline]
    pub fn dft_noise(&self) -> DFT_NOISER {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DFT_NOISER { bits }
    }
}
