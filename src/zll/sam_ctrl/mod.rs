#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SAM_CTRL {
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
#[doc = "Possible values of the field `SAP0_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAP0_ENR {
    #[doc = "Disables SAP0 Partition"]
    _0,
    #[doc = "Enables SAP0 Partition"]
    _1,
}
impl SAP0_ENR {
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
            SAP0_ENR::_0 => false,
            SAP0_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAP0_ENR {
        match value {
            false => SAP0_ENR::_0,
            true => SAP0_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SAP0_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SAP0_ENR::_1
    }
}
#[doc = "Possible values of the field `SAA0_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAA0_ENR {
    #[doc = "Disables SAA0 Partition"]
    _0,
    #[doc = "Enables SAA0 Partition"]
    _1,
}
impl SAA0_ENR {
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
            SAA0_ENR::_0 => false,
            SAA0_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAA0_ENR {
        match value {
            false => SAA0_ENR::_0,
            true => SAA0_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SAA0_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SAA0_ENR::_1
    }
}
#[doc = "Possible values of the field `SAP1_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAP1_ENR {
    #[doc = "Disables SAP1 Partition"]
    _0,
    #[doc = "Enables SAP1 Partition"]
    _1,
}
impl SAP1_ENR {
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
            SAP1_ENR::_0 => false,
            SAP1_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAP1_ENR {
        match value {
            false => SAP1_ENR::_0,
            true => SAP1_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SAP1_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SAP1_ENR::_1
    }
}
#[doc = "Possible values of the field `SAA1_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAA1_ENR {
    #[doc = "Disables SAA1 Partition"]
    _0,
    #[doc = "Enables SAA1 Partition"]
    _1,
}
impl SAA1_ENR {
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
            SAA1_ENR::_0 => false,
            SAA1_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAA1_ENR {
        match value {
            false => SAA1_ENR::_0,
            true => SAA1_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SAA1_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SAA1_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SAA0_STARTR {
    bits: u8,
}
impl SAA0_STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAP1_STARTR {
    bits: u8,
}
impl SAP1_STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAA1_STARTR {
    bits: u8,
}
impl SAA1_STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SAP0_EN`"]
pub enum SAP0_ENW {
    #[doc = "Disables SAP0 Partition"]
    _0,
    #[doc = "Enables SAP0 Partition"]
    _1,
}
impl SAP0_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAP0_ENW::_0 => false,
            SAP0_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAP0_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SAP0_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAP0_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables SAP0 Partition"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAP0_ENW::_0)
    }
    #[doc = "Enables SAP0 Partition"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAP0_ENW::_1)
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
#[doc = "Values that can be written to the field `SAA0_EN`"]
pub enum SAA0_ENW {
    #[doc = "Disables SAA0 Partition"]
    _0,
    #[doc = "Enables SAA0 Partition"]
    _1,
}
impl SAA0_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAA0_ENW::_0 => false,
            SAA0_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAA0_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SAA0_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAA0_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables SAA0 Partition"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAA0_ENW::_0)
    }
    #[doc = "Enables SAA0 Partition"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAA0_ENW::_1)
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
#[doc = "Values that can be written to the field `SAP1_EN`"]
pub enum SAP1_ENW {
    #[doc = "Disables SAP1 Partition"]
    _0,
    #[doc = "Enables SAP1 Partition"]
    _1,
}
impl SAP1_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAP1_ENW::_0 => false,
            SAP1_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAP1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SAP1_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAP1_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables SAP1 Partition"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAP1_ENW::_0)
    }
    #[doc = "Enables SAP1 Partition"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAP1_ENW::_1)
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
#[doc = "Values that can be written to the field `SAA1_EN`"]
pub enum SAA1_ENW {
    #[doc = "Disables SAA1 Partition"]
    _0,
    #[doc = "Enables SAA1 Partition"]
    _1,
}
impl SAA1_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAA1_ENW::_0 => false,
            SAA1_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAA1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SAA1_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAA1_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables SAA1 Partition"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAA1_ENW::_0)
    }
    #[doc = "Enables SAA1 Partition"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAA1_ENW::_1)
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
#[doc = r" Proxy"]
pub struct _SAA0_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _SAA0_STARTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAP1_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _SAP1_STARTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAA1_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _SAA1_STARTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Enables SAP0 Partition of the SAM Table"]
    #[inline]
    pub fn sap0_en(&self) -> SAP0_ENR {
        SAP0_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enables SAA0 Partition of the SAM Table"]
    #[inline]
    pub fn saa0_en(&self) -> SAA0_ENR {
        SAA0_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enables SAP1 Partition of the SAM Table"]
    #[inline]
    pub fn sap1_en(&self) -> SAP1_ENR {
        SAP1_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enables SAA1 Partition of the SAM Table"]
    #[inline]
    pub fn saa1_en(&self) -> SAA1_ENR {
        SAA1_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:15 - First Index of SAA0 partition"]
    #[inline]
    pub fn saa0_start(&self) -> SAA0_STARTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAA0_STARTR { bits }
    }
    #[doc = "Bits 16:23 - First Index of SAP1 partition"]
    #[inline]
    pub fn sap1_start(&self) -> SAP1_STARTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAP1_STARTR { bits }
    }
    #[doc = "Bits 24:31 - First Index of SAA1 partition"]
    #[inline]
    pub fn saa1_start(&self) -> SAA1_STARTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAA1_STARTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2155888640 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enables SAP0 Partition of the SAM Table"]
    #[inline]
    pub fn sap0_en(&mut self) -> _SAP0_ENW {
        _SAP0_ENW { w: self }
    }
    #[doc = "Bit 1 - Enables SAA0 Partition of the SAM Table"]
    #[inline]
    pub fn saa0_en(&mut self) -> _SAA0_ENW {
        _SAA0_ENW { w: self }
    }
    #[doc = "Bit 2 - Enables SAP1 Partition of the SAM Table"]
    #[inline]
    pub fn sap1_en(&mut self) -> _SAP1_ENW {
        _SAP1_ENW { w: self }
    }
    #[doc = "Bit 3 - Enables SAA1 Partition of the SAM Table"]
    #[inline]
    pub fn saa1_en(&mut self) -> _SAA1_ENW {
        _SAA1_ENW { w: self }
    }
    #[doc = "Bits 8:15 - First Index of SAA0 partition"]
    #[inline]
    pub fn saa0_start(&mut self) -> _SAA0_STARTW {
        _SAA0_STARTW { w: self }
    }
    #[doc = "Bits 16:23 - First Index of SAP1 partition"]
    #[inline]
    pub fn sap1_start(&mut self) -> _SAP1_STARTW {
        _SAP1_STARTW { w: self }
    }
    #[doc = "Bits 24:31 - First Index of SAA1 partition"]
    #[inline]
    pub fn saa1_start(&mut self) -> _SAA1_STARTW {
        _SAA1_STARTW { w: self }
    }
}
