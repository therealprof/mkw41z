#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDOR {
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
#[doc = "Possible values of the field `PDO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDOR {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl PDOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            PDOR::_0 => 0,
            PDOR::_1 => 1,
            PDOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> PDOR {
        match value {
            0 => PDOR::_0,
            1 => PDOR::_1,
            i => PDOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDOR::_1
    }
}
#[doc = "Values that can be written to the field `PDO`"]
pub enum PDOW {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            PDOW::_0 => 0,
            PDOW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDOW<'a> {
    w: &'a mut W,
}
impl<'a> _PDOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDOW::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDOW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:31 - Port Data Output"]
    #[inline]
    pub fn pdo(&self) -> PDOR {
        PDOR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
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
    #[doc = "Bits 0:31 - Port Data Output"]
    #[inline]
    pub fn pdo(&mut self) -> _PDOW {
        _PDOW { w: self }
    }
}
