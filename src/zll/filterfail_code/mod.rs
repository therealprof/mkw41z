#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FILTERFAIL_CODE {
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
#[doc = r" Value of the field"]
pub struct FILTERFAIL_CODER {
    bits: u16,
}
impl FILTERFAIL_CODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `FILTERFAIL_PAN_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTERFAIL_PAN_SELR {
    #[doc = "FILTERFAIL_CODE[9:0] will report the FILTERFAIL status of PAN0"]
    _0,
    #[doc = "FILTERFAIL_CODE[9:0] will report the FILTERFAIL status of PAN1"]
    _1,
}
impl FILTERFAIL_PAN_SELR {
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
            FILTERFAIL_PAN_SELR::_0 => false,
            FILTERFAIL_PAN_SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FILTERFAIL_PAN_SELR {
        match value {
            false => FILTERFAIL_PAN_SELR::_0,
            true => FILTERFAIL_PAN_SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FILTERFAIL_PAN_SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FILTERFAIL_PAN_SELR::_1
    }
}
#[doc = "Values that can be written to the field `FILTERFAIL_PAN_SEL`"]
pub enum FILTERFAIL_PAN_SELW {
    #[doc = "FILTERFAIL_CODE[9:0] will report the FILTERFAIL status of PAN0"]
    _0,
    #[doc = "FILTERFAIL_CODE[9:0] will report the FILTERFAIL status of PAN1"]
    _1,
}
impl FILTERFAIL_PAN_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FILTERFAIL_PAN_SELW::_0 => false,
            FILTERFAIL_PAN_SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTERFAIL_PAN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERFAIL_PAN_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTERFAIL_PAN_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FILTERFAIL_CODE[9:0] will report the FILTERFAIL status of PAN0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILTERFAIL_PAN_SELW::_0)
    }
    #[doc = "FILTERFAIL_CODE[9:0] will report the FILTERFAIL status of PAN1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILTERFAIL_PAN_SELW::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - Filter Fail Code"]
    #[inline]
    pub fn filterfail_code(&self) -> FILTERFAIL_CODER {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FILTERFAIL_CODER { bits }
    }
    #[doc = "Bit 15 - PAN Selector for Filter Fail Code"]
    #[inline]
    pub fn filterfail_pan_sel(&self) -> FILTERFAIL_PAN_SELR {
        FILTERFAIL_PAN_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 15 - PAN Selector for Filter Fail Code"]
    #[inline]
    pub fn filterfail_pan_sel(&mut self) -> _FILTERFAIL_PAN_SELW {
        _FILTERFAIL_PAN_SELW { w: self }
    }
}
