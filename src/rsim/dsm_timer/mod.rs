#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DSM_TIMER {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DSM_TIMERR {
    bits: u32,
}
impl DSM_TIMERR {
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
    #[doc = "Bits 0:23 - Deep Sleep Mode Timer"]
    #[inline]
    pub fn dsm_timer(&self) -> DSM_TIMERR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DSM_TIMERR { bits }
    }
}
