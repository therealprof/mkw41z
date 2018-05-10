#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::MISC_CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `TSM_INTR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_INTR_ENR {
    #[doc = "a TSM Interrupt during a BLE TX or RX sequence will not set bit [8] of BLE EVENT_STATUS"]
    _0,
    #[doc = "a TSM Interrupt during a BLE TX or RX sequence will set bit [8] of BLE EVENT_STATUS"]
    _1,
}
impl TSM_INTR_ENR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TSM_INTR_ENR::_0 => false,
            TSM_INTR_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSM_INTR_ENR {
        match value {
            false => TSM_INTR_ENR::_0,
            true => TSM_INTR_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSM_INTR_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSM_INTR_ENR::_1
    }
}
#[doc = "Values that can be written to the field `TSM_INTR_EN`"]
pub enum TSM_INTR_ENW {
    #[doc = "a TSM Interrupt during a BLE TX or RX sequence will not set bit [8] of BLE EVENT_STATUS"]
    _0,
    #[doc = "a TSM Interrupt during a BLE TX or RX sequence will set bit [8] of BLE EVENT_STATUS"]
    _1,
}
impl TSM_INTR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSM_INTR_ENW::_0 => false,
            TSM_INTR_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSM_INTR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_INTR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSM_INTR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a TSM Interrupt during a BLE TX or RX sequence will not set bit [8] of BLE EVENT_STATUS"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSM_INTR_ENW::_0)
    }
    #[doc = "a TSM Interrupt during a BLE TX or RX sequence will set bit [8] of BLE EVENT_STATUS"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSM_INTR_ENW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 1 - TSM Interrupt Enable"]
    #[inline]
    pub fn tsm_intr_en(&self) -> TSM_INTR_ENR {
        TSM_INTR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - TSM Interrupt Enable"]
    #[inline]
    pub fn tsm_intr_en(&mut self) -> _TSM_INTR_ENW {
        _TSM_INTR_ENW { w: self }
    }
}
