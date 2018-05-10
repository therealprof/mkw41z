#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
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
#[doc = "Possible values of the field `IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMR {
    #[doc = "Interrupt not masked."]
    _0,
    #[doc = "Interrupt masked"]
    _1,
}
impl IMR {
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
            IMR::_0 => false,
            IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMR {
        match value {
            false => IMR::_0,
            true => IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IMR::_1
    }
}
#[doc = "Possible values of the field `IFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFER {
    #[doc = "DMA Request and Done signals disabled for the Input FIFO."]
    _0,
    #[doc = "DMA Request and Done signals enabled for the Input FIFO."]
    _1,
}
impl IFER {
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
            IFER::_0 => false,
            IFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IFER {
        match value {
            false => IFER::_0,
            true => IFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IFER::_1
    }
}
#[doc = "Possible values of the field `IFR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFRR {
    #[doc = "DMA request size is 1 entry."]
    _0,
    #[doc = "DMA request size is 4 entries."]
    _1,
}
impl IFRR {
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
            IFRR::_0 => false,
            IFRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IFRR {
        match value {
            false => IFRR::_0,
            true => IFRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IFRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IFRR::_1
    }
}
#[doc = "Possible values of the field `OFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFER {
    #[doc = "DMA Request and Done signals disabled for the Output FIFO."]
    _0,
    #[doc = "DMA Request and Done signals enabled for the Output FIFO."]
    _1,
}
impl OFER {
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
            OFER::_0 => false,
            OFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OFER {
        match value {
            false => OFER::_0,
            true => OFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OFER::_1
    }
}
#[doc = "Possible values of the field `OFR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFRR {
    #[doc = "DMA request size is 1 entry."]
    _0,
    #[doc = "DMA request size is 4 entries."]
    _1,
}
impl OFRR {
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
            OFRR::_0 => false,
            OFRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OFRR {
        match value {
            false => OFRR::_0,
            true => OFRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OFRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OFRR::_1
    }
}
#[doc = "Possible values of the field `IFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFSR {
    #[doc = "Do Not Byte Swap Data."]
    _0,
    #[doc = "Byte Swap Data."]
    _1,
}
impl IFSR {
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
            IFSR::_0 => false,
            IFSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IFSR {
        match value {
            false => IFSR::_0,
            true => IFSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IFSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IFSR::_1
    }
}
#[doc = "Possible values of the field `OFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFSR {
    #[doc = "Do Not Byte Swap Data."]
    _0,
    #[doc = "Byte Swap Data."]
    _1,
}
impl OFSR {
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
            OFSR::_0 => false,
            OFSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OFSR {
        match value {
            false => OFSR::_0,
            true => OFSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OFSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OFSR::_1
    }
}
#[doc = "Possible values of the field `KIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KISR {
    #[doc = "Do Not Byte Swap Data."]
    _0,
    #[doc = "Byte Swap Data."]
    _1,
}
impl KISR {
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
            KISR::_0 => false,
            KISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KISR {
        match value {
            false => KISR::_0,
            true => KISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == KISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == KISR::_1
    }
}
#[doc = "Possible values of the field `KOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KOSR {
    #[doc = "Do Not Byte Swap Data."]
    _0,
    #[doc = "Byte Swap Data."]
    _1,
}
impl KOSR {
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
            KOSR::_0 => false,
            KOSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KOSR {
        match value {
            false => KOSR::_0,
            true => KOSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == KOSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == KOSR::_1
    }
}
#[doc = "Possible values of the field `CIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CISR {
    #[doc = "Do Not Byte Swap Data."]
    _0,
    #[doc = "Byte Swap Data."]
    _1,
}
impl CISR {
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
            CISR::_0 => false,
            CISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CISR {
        match value {
            false => CISR::_0,
            true => CISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CISR::_1
    }
}
#[doc = "Possible values of the field `COS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSR {
    #[doc = "Do Not Byte Swap Data."]
    _0,
    #[doc = "Byte Swap Data."]
    _1,
}
impl COSR {
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
            COSR::_0 => false,
            COSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COSR {
        match value {
            false => COSR::_0,
            true => COSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COSR::_1
    }
}
#[doc = "Possible values of the field `KAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KALR {
    #[doc = "Key Register is readable."]
    _0,
    #[doc = "Key Register is not readable."]
    _1,
}
impl KALR {
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
            KALR::_0 => false,
            KALR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KALR {
        match value {
            false => KALR::_0,
            true => KALR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == KALR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == KALR::_1
    }
}
#[doc = "Values that can be written to the field `IM`"]
pub enum IMW {
    #[doc = "Interrupt not masked."]
    _0,
    #[doc = "Interrupt masked"]
    _1,
}
impl IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IMW::_0 => false,
            IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IMW<'a> {
    w: &'a mut W,
}
impl<'a> _IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt not masked."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IMW::_0)
    }
    #[doc = "Interrupt masked"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IMW::_1)
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
#[doc = "Values that can be written to the field `IFE`"]
pub enum IFEW {
    #[doc = "DMA Request and Done signals disabled for the Input FIFO."]
    _0,
    #[doc = "DMA Request and Done signals enabled for the Input FIFO."]
    _1,
}
impl IFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IFEW::_0 => false,
            IFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IFEW<'a> {
    w: &'a mut W,
}
impl<'a> _IFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA Request and Done signals disabled for the Input FIFO."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFEW::_0)
    }
    #[doc = "DMA Request and Done signals enabled for the Input FIFO."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFEW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IFR`"]
pub enum IFRW {
    #[doc = "DMA request size is 1 entry."]
    _0,
    #[doc = "DMA request size is 4 entries."]
    _1,
}
impl IFRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IFRW::_0 => false,
            IFRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IFRW<'a> {
    w: &'a mut W,
}
impl<'a> _IFRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IFRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request size is 1 entry."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFRW::_0)
    }
    #[doc = "DMA request size is 4 entries."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFRW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OFE`"]
pub enum OFEW {
    #[doc = "DMA Request and Done signals disabled for the Output FIFO."]
    _0,
    #[doc = "DMA Request and Done signals enabled for the Output FIFO."]
    _1,
}
impl OFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFEW::_0 => false,
            OFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFEW<'a> {
    w: &'a mut W,
}
impl<'a> _OFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA Request and Done signals disabled for the Output FIFO."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OFEW::_0)
    }
    #[doc = "DMA Request and Done signals enabled for the Output FIFO."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OFEW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OFR`"]
pub enum OFRW {
    #[doc = "DMA request size is 1 entry."]
    _0,
    #[doc = "DMA request size is 4 entries."]
    _1,
}
impl OFRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFRW::_0 => false,
            OFRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFRW<'a> {
    w: &'a mut W,
}
impl<'a> _OFRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request size is 1 entry."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OFRW::_0)
    }
    #[doc = "DMA request size is 4 entries."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OFRW::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IFS`"]
pub enum IFSW {
    #[doc = "Do Not Byte Swap Data."]
    _0,
    #[doc = "Byte Swap Data."]
    _1,
}
impl IFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IFSW::_0 => false,
            IFSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IFSW<'a> {
    w: &'a mut W,
}
impl<'a> _IFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do Not Byte Swap Data."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFSW::_0)
    }
    #[doc = "Byte Swap Data."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFSW::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OFS`"]
pub enum OFSW {
    #[doc = "Do Not Byte Swap Data."]
    _0,
    #[doc = "Byte Swap Data."]
    _1,
}
impl OFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFSW::_0 => false,
            OFSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFSW<'a> {
    w: &'a mut W,
}
impl<'a> _OFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do Not Byte Swap Data."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OFSW::_0)
    }
    #[doc = "Byte Swap Data."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OFSW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KIS`"]
pub enum KISW {
    #[doc = "Do Not Byte Swap Data."]
    _0,
    #[doc = "Byte Swap Data."]
    _1,
}
impl KISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KISW::_0 => false,
            KISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KISW<'a> {
    w: &'a mut W,
}
impl<'a> _KISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do Not Byte Swap Data."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(KISW::_0)
    }
    #[doc = "Byte Swap Data."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(KISW::_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KOS`"]
pub enum KOSW {
    #[doc = "Do Not Byte Swap Data."]
    _0,
    #[doc = "Byte Swap Data."]
    _1,
}
impl KOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KOSW::_0 => false,
            KOSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KOSW<'a> {
    w: &'a mut W,
}
impl<'a> _KOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KOSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do Not Byte Swap Data."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(KOSW::_0)
    }
    #[doc = "Byte Swap Data."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(KOSW::_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CIS`"]
pub enum CISW {
    #[doc = "Do Not Byte Swap Data."]
    _0,
    #[doc = "Byte Swap Data."]
    _1,
}
impl CISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CISW::_0 => false,
            CISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CISW<'a> {
    w: &'a mut W,
}
impl<'a> _CISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do Not Byte Swap Data."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CISW::_0)
    }
    #[doc = "Byte Swap Data."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CISW::_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COS`"]
pub enum COSW {
    #[doc = "Do Not Byte Swap Data."]
    _0,
    #[doc = "Byte Swap Data."]
    _1,
}
impl COSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COSW::_0 => false,
            COSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COSW<'a> {
    w: &'a mut W,
}
impl<'a> _COSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do Not Byte Swap Data."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COSW::_0)
    }
    #[doc = "Byte Swap Data."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COSW::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KAL`"]
pub enum KALW {
    #[doc = "Key Register is readable."]
    _0,
    #[doc = "Key Register is not readable."]
    _1,
}
impl KALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KALW::_0 => false,
            KALW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KALW<'a> {
    w: &'a mut W,
}
impl<'a> _KALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Key Register is readable."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(KALW::_0)
    }
    #[doc = "Key Register is not readable."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(KALW::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Interrupt Mask"]
    #[inline]
    pub fn im(&self) -> IMR {
        IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Input FIFO DMA Enable"]
    #[inline]
    pub fn ife(&self) -> IFER {
        IFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Input FIFO DMA Request Size"]
    #[inline]
    pub fn ifr(&self) -> IFRR {
        IFRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Output FIFO DMA Enable"]
    #[inline]
    pub fn ofe(&self) -> OFER {
        OFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Output FIFO DMA Request Size"]
    #[inline]
    pub fn ofr(&self) -> OFRR {
        OFRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Input FIFO Byte Swap"]
    #[inline]
    pub fn ifs(&self) -> IFSR {
        IFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Output FIFO Byte Swap"]
    #[inline]
    pub fn ofs(&self) -> OFSR {
        OFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Key Register Input Byte Swap"]
    #[inline]
    pub fn kis(&self) -> KISR {
        KISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Key Register Output Byte Swap"]
    #[inline]
    pub fn kos(&self) -> KOSR {
        KOSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Context Register Input Byte Swap"]
    #[inline]
    pub fn cis(&self) -> CISR {
        CISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Context Register Output Byte Swap"]
    #[inline]
    pub fn cos(&self) -> COSR {
        COSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Key Register Access Lock"]
    #[inline]
    pub fn kal(&self) -> KALR {
        KALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Interrupt Mask"]
    #[inline]
    pub fn im(&mut self) -> _IMW {
        _IMW { w: self }
    }
    #[doc = "Bit 8 - Input FIFO DMA Enable"]
    #[inline]
    pub fn ife(&mut self) -> _IFEW {
        _IFEW { w: self }
    }
    #[doc = "Bit 9 - Input FIFO DMA Request Size"]
    #[inline]
    pub fn ifr(&mut self) -> _IFRW {
        _IFRW { w: self }
    }
    #[doc = "Bit 12 - Output FIFO DMA Enable"]
    #[inline]
    pub fn ofe(&mut self) -> _OFEW {
        _OFEW { w: self }
    }
    #[doc = "Bit 13 - Output FIFO DMA Request Size"]
    #[inline]
    pub fn ofr(&mut self) -> _OFRW {
        _OFRW { w: self }
    }
    #[doc = "Bit 16 - Input FIFO Byte Swap"]
    #[inline]
    pub fn ifs(&mut self) -> _IFSW {
        _IFSW { w: self }
    }
    #[doc = "Bit 17 - Output FIFO Byte Swap"]
    #[inline]
    pub fn ofs(&mut self) -> _OFSW {
        _OFSW { w: self }
    }
    #[doc = "Bit 20 - Key Register Input Byte Swap"]
    #[inline]
    pub fn kis(&mut self) -> _KISW {
        _KISW { w: self }
    }
    #[doc = "Bit 21 - Key Register Output Byte Swap"]
    #[inline]
    pub fn kos(&mut self) -> _KOSW {
        _KOSW { w: self }
    }
    #[doc = "Bit 22 - Context Register Input Byte Swap"]
    #[inline]
    pub fn cis(&mut self) -> _CISW {
        _CISW { w: self }
    }
    #[doc = "Bit 23 - Context Register Output Byte Swap"]
    #[inline]
    pub fn cos(&mut self) -> _COSW {
        _COSW { w: self }
    }
    #[doc = "Bit 31 - Key Register Access Lock"]
    #[inline]
    pub fn kal(&mut self) -> _KALW {
        _KALW { w: self }
    }
}
