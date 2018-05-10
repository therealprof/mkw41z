#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RF_NOT_ALLOWED_CTRL {
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
#[doc = "Possible values of the field `RF_NOT_ALLOWED_NO_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_NOT_ALLOWED_NO_TXR {
    #[doc = "Assertion on RF_NOT_ALLOWED has no effect on TX"]
    _0,
    #[doc = "Assertion on RF_NOT_ALLOWED can abort TX"]
    _1,
}
impl RF_NOT_ALLOWED_NO_TXR {
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
            RF_NOT_ALLOWED_NO_TXR::_0 => false,
            RF_NOT_ALLOWED_NO_TXR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RF_NOT_ALLOWED_NO_TXR {
        match value {
            false => RF_NOT_ALLOWED_NO_TXR::_0,
            true => RF_NOT_ALLOWED_NO_TXR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RF_NOT_ALLOWED_NO_TXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RF_NOT_ALLOWED_NO_TXR::_1
    }
}
#[doc = "Possible values of the field `RF_NOT_ALLOWED_NO_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_NOT_ALLOWED_NO_RXR {
    #[doc = "Assertion on RF_NOT_ALLOWED has no effect on RX"]
    _0,
    #[doc = "Assertion on RF_NOT_ALLOWED can abort RX"]
    _1,
}
impl RF_NOT_ALLOWED_NO_RXR {
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
            RF_NOT_ALLOWED_NO_RXR::_0 => false,
            RF_NOT_ALLOWED_NO_RXR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RF_NOT_ALLOWED_NO_RXR {
        match value {
            false => RF_NOT_ALLOWED_NO_RXR::_0,
            true => RF_NOT_ALLOWED_NO_RXR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RF_NOT_ALLOWED_NO_RXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RF_NOT_ALLOWED_NO_RXR::_1
    }
}
#[doc = "Possible values of the field `RF_NOT_ALLOWED_ASSERTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_NOT_ALLOWED_ASSERTEDR {
    #[doc = "Assertion on RF_NOT_ALLOWED has not occurred"]
    _0,
    #[doc = "Assertion on RF_NOT_ALLOWED has occurred since the last time this bit was cleared"]
    _1,
}
impl RF_NOT_ALLOWED_ASSERTEDR {
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
            RF_NOT_ALLOWED_ASSERTEDR::_0 => false,
            RF_NOT_ALLOWED_ASSERTEDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RF_NOT_ALLOWED_ASSERTEDR {
        match value {
            false => RF_NOT_ALLOWED_ASSERTEDR::_0,
            true => RF_NOT_ALLOWED_ASSERTEDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RF_NOT_ALLOWED_ASSERTEDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RF_NOT_ALLOWED_ASSERTEDR::_1
    }
}
#[doc = "Possible values of the field `RF_NOT_ALLOWED_TX_ABORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_NOT_ALLOWED_TX_ABORTR {
    #[doc = "A TX abort due to assertion on RF_NOT_ALLOWED has not occurred"]
    _0,
    #[doc = "A TX abort due to assertion on RF_NOT_ALLOWED has occurred since the last time this bit was cleared"]
    _1,
}
impl RF_NOT_ALLOWED_TX_ABORTR {
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
            RF_NOT_ALLOWED_TX_ABORTR::_0 => false,
            RF_NOT_ALLOWED_TX_ABORTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RF_NOT_ALLOWED_TX_ABORTR {
        match value {
            false => RF_NOT_ALLOWED_TX_ABORTR::_0,
            true => RF_NOT_ALLOWED_TX_ABORTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RF_NOT_ALLOWED_TX_ABORTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RF_NOT_ALLOWED_TX_ABORTR::_1
    }
}
#[doc = "Possible values of the field `RF_NOT_ALLOWED_RX_ABORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_NOT_ALLOWED_RX_ABORTR {
    #[doc = "A RX abort due to assertion on RF_NOT_ALLOWED has not occurred"]
    _0,
    #[doc = "A RX abort due to assertion on RF_NOT_ALLOWED has occurred since the last time this bit was cleared"]
    _1,
}
impl RF_NOT_ALLOWED_RX_ABORTR {
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
            RF_NOT_ALLOWED_RX_ABORTR::_0 => false,
            RF_NOT_ALLOWED_RX_ABORTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RF_NOT_ALLOWED_RX_ABORTR {
        match value {
            false => RF_NOT_ALLOWED_RX_ABORTR::_0,
            true => RF_NOT_ALLOWED_RX_ABORTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RF_NOT_ALLOWED_RX_ABORTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RF_NOT_ALLOWED_RX_ABORTR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RF_NOT_ALLOWEDR {
    bits: bool,
}
impl RF_NOT_ALLOWEDR {
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
#[doc = "Values that can be written to the field `RF_NOT_ALLOWED_NO_TX`"]
pub enum RF_NOT_ALLOWED_NO_TXW {
    #[doc = "Assertion on RF_NOT_ALLOWED has no effect on TX"]
    _0,
    #[doc = "Assertion on RF_NOT_ALLOWED can abort TX"]
    _1,
}
impl RF_NOT_ALLOWED_NO_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RF_NOT_ALLOWED_NO_TXW::_0 => false,
            RF_NOT_ALLOWED_NO_TXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RF_NOT_ALLOWED_NO_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _RF_NOT_ALLOWED_NO_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RF_NOT_ALLOWED_NO_TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assertion on RF_NOT_ALLOWED has no effect on TX"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RF_NOT_ALLOWED_NO_TXW::_0)
    }
    #[doc = "Assertion on RF_NOT_ALLOWED can abort TX"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RF_NOT_ALLOWED_NO_TXW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RF_NOT_ALLOWED_NO_RX`"]
pub enum RF_NOT_ALLOWED_NO_RXW {
    #[doc = "Assertion on RF_NOT_ALLOWED has no effect on RX"]
    _0,
    #[doc = "Assertion on RF_NOT_ALLOWED can abort RX"]
    _1,
}
impl RF_NOT_ALLOWED_NO_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RF_NOT_ALLOWED_NO_RXW::_0 => false,
            RF_NOT_ALLOWED_NO_RXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RF_NOT_ALLOWED_NO_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _RF_NOT_ALLOWED_NO_RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RF_NOT_ALLOWED_NO_RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assertion on RF_NOT_ALLOWED has no effect on RX"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RF_NOT_ALLOWED_NO_RXW::_0)
    }
    #[doc = "Assertion on RF_NOT_ALLOWED can abort RX"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RF_NOT_ALLOWED_NO_RXW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RF_NOT_ALLOWED_ASSERTED`"]
pub enum RF_NOT_ALLOWED_ASSERTEDW {
    #[doc = "Assertion on RF_NOT_ALLOWED has not occurred"]
    _0,
    #[doc = "Assertion on RF_NOT_ALLOWED has occurred since the last time this bit was cleared"]
    _1,
}
impl RF_NOT_ALLOWED_ASSERTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RF_NOT_ALLOWED_ASSERTEDW::_0 => false,
            RF_NOT_ALLOWED_ASSERTEDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RF_NOT_ALLOWED_ASSERTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _RF_NOT_ALLOWED_ASSERTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RF_NOT_ALLOWED_ASSERTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assertion on RF_NOT_ALLOWED has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RF_NOT_ALLOWED_ASSERTEDW::_0)
    }
    #[doc = "Assertion on RF_NOT_ALLOWED has occurred since the last time this bit was cleared"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RF_NOT_ALLOWED_ASSERTEDW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RF_NOT_ALLOWED_TX_ABORT`"]
pub enum RF_NOT_ALLOWED_TX_ABORTW {
    #[doc = "A TX abort due to assertion on RF_NOT_ALLOWED has not occurred"]
    _0,
    #[doc = "A TX abort due to assertion on RF_NOT_ALLOWED has occurred since the last time this bit was cleared"]
    _1,
}
impl RF_NOT_ALLOWED_TX_ABORTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RF_NOT_ALLOWED_TX_ABORTW::_0 => false,
            RF_NOT_ALLOWED_TX_ABORTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RF_NOT_ALLOWED_TX_ABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _RF_NOT_ALLOWED_TX_ABORTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RF_NOT_ALLOWED_TX_ABORTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A TX abort due to assertion on RF_NOT_ALLOWED has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RF_NOT_ALLOWED_TX_ABORTW::_0)
    }
    #[doc = "A TX abort due to assertion on RF_NOT_ALLOWED has occurred since the last time this bit was cleared"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RF_NOT_ALLOWED_TX_ABORTW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RF_NOT_ALLOWED_RX_ABORT`"]
pub enum RF_NOT_ALLOWED_RX_ABORTW {
    #[doc = "A RX abort due to assertion on RF_NOT_ALLOWED has not occurred"]
    _0,
    #[doc = "A RX abort due to assertion on RF_NOT_ALLOWED has occurred since the last time this bit was cleared"]
    _1,
}
impl RF_NOT_ALLOWED_RX_ABORTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RF_NOT_ALLOWED_RX_ABORTW::_0 => false,
            RF_NOT_ALLOWED_RX_ABORTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RF_NOT_ALLOWED_RX_ABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _RF_NOT_ALLOWED_RX_ABORTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RF_NOT_ALLOWED_RX_ABORTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A RX abort due to assertion on RF_NOT_ALLOWED has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RF_NOT_ALLOWED_RX_ABORTW::_0)
    }
    #[doc = "A RX abort due to assertion on RF_NOT_ALLOWED has occurred since the last time this bit was cleared"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RF_NOT_ALLOWED_RX_ABORTW::_1)
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - RF_NOT_ALLOWED_NO_TX"]
    #[inline]
    pub fn rf_not_allowed_no_tx(&self) -> RF_NOT_ALLOWED_NO_TXR {
        RF_NOT_ALLOWED_NO_TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RF_NOT_ALLOWED_NO_RX"]
    #[inline]
    pub fn rf_not_allowed_no_rx(&self) -> RF_NOT_ALLOWED_NO_RXR {
        RF_NOT_ALLOWED_NO_RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RF_NOT_ALLOWED_ASSERTED"]
    #[inline]
    pub fn rf_not_allowed_asserted(&self) -> RF_NOT_ALLOWED_ASSERTEDR {
        RF_NOT_ALLOWED_ASSERTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - RF_NOT_ALLOWED_TX_ABORT"]
    #[inline]
    pub fn rf_not_allowed_tx_abort(&self) -> RF_NOT_ALLOWED_TX_ABORTR {
        RF_NOT_ALLOWED_TX_ABORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RF_NOT_ALLOWED_RX_ABORT"]
    #[inline]
    pub fn rf_not_allowed_rx_abort(&self) -> RF_NOT_ALLOWED_RX_ABORTR {
        RF_NOT_ALLOWED_RX_ABORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RF_NOT_ALLOWED"]
    #[inline]
    pub fn rf_not_allowed(&self) -> RF_NOT_ALLOWEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RF_NOT_ALLOWEDR { bits }
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
    #[doc = "Bit 0 - RF_NOT_ALLOWED_NO_TX"]
    #[inline]
    pub fn rf_not_allowed_no_tx(&mut self) -> _RF_NOT_ALLOWED_NO_TXW {
        _RF_NOT_ALLOWED_NO_TXW { w: self }
    }
    #[doc = "Bit 1 - RF_NOT_ALLOWED_NO_RX"]
    #[inline]
    pub fn rf_not_allowed_no_rx(&mut self) -> _RF_NOT_ALLOWED_NO_RXW {
        _RF_NOT_ALLOWED_NO_RXW { w: self }
    }
    #[doc = "Bit 2 - RF_NOT_ALLOWED_ASSERTED"]
    #[inline]
    pub fn rf_not_allowed_asserted(&mut self) -> _RF_NOT_ALLOWED_ASSERTEDW {
        _RF_NOT_ALLOWED_ASSERTEDW { w: self }
    }
    #[doc = "Bit 3 - RF_NOT_ALLOWED_TX_ABORT"]
    #[inline]
    pub fn rf_not_allowed_tx_abort(&mut self) -> _RF_NOT_ALLOWED_TX_ABORTW {
        _RF_NOT_ALLOWED_TX_ABORTW { w: self }
    }
    #[doc = "Bit 4 - RF_NOT_ALLOWED_RX_ABORT"]
    #[inline]
    pub fn rf_not_allowed_rx_abort(&mut self) -> _RF_NOT_ALLOWED_RX_ABORTW {
        _RF_NOT_ALLOWED_RX_ABORTW { w: self }
    }
}
