#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NTW_ADR_CTRL {
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
#[doc = "Possible values of the field `NTW_ADR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NTW_ADR_ENR {
    #[doc = "Enable Network Address 0 for correlation"]
    _0001,
    #[doc = "Enable Network Address 1 for correlation"]
    _0010,
    #[doc = "Enable Network Address 2 for correlation"]
    _0100,
    #[doc = "Enable Network Address 3 for correlation"]
    _1000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NTW_ADR_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NTW_ADR_ENR::_0001 => 1,
            NTW_ADR_ENR::_0010 => 2,
            NTW_ADR_ENR::_0100 => 4,
            NTW_ADR_ENR::_1000 => 8,
            NTW_ADR_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NTW_ADR_ENR {
        match value {
            1 => NTW_ADR_ENR::_0001,
            2 => NTW_ADR_ENR::_0010,
            4 => NTW_ADR_ENR::_0100,
            8 => NTW_ADR_ENR::_1000,
            i => NTW_ADR_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == NTW_ADR_ENR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == NTW_ADR_ENR::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == NTW_ADR_ENR::_0100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == NTW_ADR_ENR::_1000
    }
}
#[doc = "Possible values of the field `NTW_ADR_MCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NTW_ADR_MCHR {
    #[doc = "Network Address 0 has matched"]
    _0001,
    #[doc = "Network Address 1 has matched"]
    _0010,
    #[doc = "Network Address 2 has matched"]
    _0100,
    #[doc = "Network Address 3 has matched"]
    _1000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NTW_ADR_MCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NTW_ADR_MCHR::_0001 => 1,
            NTW_ADR_MCHR::_0010 => 2,
            NTW_ADR_MCHR::_0100 => 4,
            NTW_ADR_MCHR::_1000 => 8,
            NTW_ADR_MCHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NTW_ADR_MCHR {
        match value {
            1 => NTW_ADR_MCHR::_0001,
            2 => NTW_ADR_MCHR::_0010,
            4 => NTW_ADR_MCHR::_0100,
            8 => NTW_ADR_MCHR::_1000,
            i => NTW_ADR_MCHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == NTW_ADR_MCHR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == NTW_ADR_MCHR::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == NTW_ADR_MCHR::_0100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == NTW_ADR_MCHR::_1000
    }
}
#[doc = "Possible values of the field `NTW_ADR0_SZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NTW_ADR0_SZR {
    #[doc = "Network Address 0 requires a 8-bit correlation"]
    _0,
    #[doc = "Network Address 0 requires a 16-bit correlation"]
    _1,
    #[doc = "Network Address 0 requires a 24-bit correlation"]
    _2,
    #[doc = "Network Address 0 requires a 32-bit correlation"]
    _3,
}
impl NTW_ADR0_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NTW_ADR0_SZR::_0 => 0,
            NTW_ADR0_SZR::_1 => 1,
            NTW_ADR0_SZR::_2 => 2,
            NTW_ADR0_SZR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NTW_ADR0_SZR {
        match value {
            0 => NTW_ADR0_SZR::_0,
            1 => NTW_ADR0_SZR::_1,
            2 => NTW_ADR0_SZR::_2,
            3 => NTW_ADR0_SZR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NTW_ADR0_SZR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NTW_ADR0_SZR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == NTW_ADR0_SZR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == NTW_ADR0_SZR::_3
    }
}
#[doc = "Possible values of the field `NTW_ADR1_SZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NTW_ADR1_SZR {
    #[doc = "Network Address 1 requires a 8-bit correlation"]
    _0,
    #[doc = "Network Address 1 requires a 16-bit correlation"]
    _1,
    #[doc = "Network Address 1 requires a 24-bit correlation"]
    _2,
    #[doc = "Network Address 1 requires a 32-bit correlation"]
    _3,
}
impl NTW_ADR1_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NTW_ADR1_SZR::_0 => 0,
            NTW_ADR1_SZR::_1 => 1,
            NTW_ADR1_SZR::_2 => 2,
            NTW_ADR1_SZR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NTW_ADR1_SZR {
        match value {
            0 => NTW_ADR1_SZR::_0,
            1 => NTW_ADR1_SZR::_1,
            2 => NTW_ADR1_SZR::_2,
            3 => NTW_ADR1_SZR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NTW_ADR1_SZR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NTW_ADR1_SZR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == NTW_ADR1_SZR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == NTW_ADR1_SZR::_3
    }
}
#[doc = "Possible values of the field `NTW_ADR2_SZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NTW_ADR2_SZR {
    #[doc = "Network Address 2 requires a 8-bit correlation"]
    _0,
    #[doc = "Network Address 2 requires a 16-bit correlation"]
    _1,
    #[doc = "Network Address 2 requires a 24-bit correlation"]
    _2,
    #[doc = "Network Address 2 requires a 32-bit correlation"]
    _3,
}
impl NTW_ADR2_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NTW_ADR2_SZR::_0 => 0,
            NTW_ADR2_SZR::_1 => 1,
            NTW_ADR2_SZR::_2 => 2,
            NTW_ADR2_SZR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NTW_ADR2_SZR {
        match value {
            0 => NTW_ADR2_SZR::_0,
            1 => NTW_ADR2_SZR::_1,
            2 => NTW_ADR2_SZR::_2,
            3 => NTW_ADR2_SZR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NTW_ADR2_SZR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NTW_ADR2_SZR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == NTW_ADR2_SZR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == NTW_ADR2_SZR::_3
    }
}
#[doc = "Possible values of the field `NTW_ADR3_SZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NTW_ADR3_SZR {
    #[doc = "Network Address 3 requires a 8-bit correlation"]
    _0,
    #[doc = "Network Address 3 requires a 16-bit correlation"]
    _1,
    #[doc = "Network Address 3 requires a 24-bit correlation"]
    _2,
    #[doc = "Network Address 3 requires a 32-bit correlation"]
    _3,
}
impl NTW_ADR3_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NTW_ADR3_SZR::_0 => 0,
            NTW_ADR3_SZR::_1 => 1,
            NTW_ADR3_SZR::_2 => 2,
            NTW_ADR3_SZR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NTW_ADR3_SZR {
        match value {
            0 => NTW_ADR3_SZR::_0,
            1 => NTW_ADR3_SZR::_1,
            2 => NTW_ADR3_SZR::_2,
            3 => NTW_ADR3_SZR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NTW_ADR3_SZR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NTW_ADR3_SZR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == NTW_ADR3_SZR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == NTW_ADR3_SZR::_3
    }
}
#[doc = r" Value of the field"]
pub struct NTW_ADR_THR0R {
    bits: u8,
}
impl NTW_ADR_THR0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NTW_ADR_THR1R {
    bits: u8,
}
impl NTW_ADR_THR1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NTW_ADR_THR2R {
    bits: u8,
}
impl NTW_ADR_THR2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NTW_ADR_THR3R {
    bits: u8,
}
impl NTW_ADR_THR3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `NTW_ADR_EN`"]
pub enum NTW_ADR_ENW {
    #[doc = "Enable Network Address 0 for correlation"]
    _0001,
    #[doc = "Enable Network Address 1 for correlation"]
    _0010,
    #[doc = "Enable Network Address 2 for correlation"]
    _0100,
    #[doc = "Enable Network Address 3 for correlation"]
    _1000,
}
impl NTW_ADR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NTW_ADR_ENW::_0001 => 1,
            NTW_ADR_ENW::_0010 => 2,
            NTW_ADR_ENW::_0100 => 4,
            NTW_ADR_ENW::_1000 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NTW_ADR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _NTW_ADR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NTW_ADR_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Enable Network Address 0 for correlation"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(NTW_ADR_ENW::_0001)
    }
    #[doc = "Enable Network Address 1 for correlation"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(NTW_ADR_ENW::_0010)
    }
    #[doc = "Enable Network Address 2 for correlation"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(NTW_ADR_ENW::_0100)
    }
    #[doc = "Enable Network Address 3 for correlation"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(NTW_ADR_ENW::_1000)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NTW_ADR0_SZ`"]
pub enum NTW_ADR0_SZW {
    #[doc = "Network Address 0 requires a 8-bit correlation"]
    _0,
    #[doc = "Network Address 0 requires a 16-bit correlation"]
    _1,
    #[doc = "Network Address 0 requires a 24-bit correlation"]
    _2,
    #[doc = "Network Address 0 requires a 32-bit correlation"]
    _3,
}
impl NTW_ADR0_SZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NTW_ADR0_SZW::_0 => 0,
            NTW_ADR0_SZW::_1 => 1,
            NTW_ADR0_SZW::_2 => 2,
            NTW_ADR0_SZW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NTW_ADR0_SZW<'a> {
    w: &'a mut W,
}
impl<'a> _NTW_ADR0_SZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NTW_ADR0_SZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Network Address 0 requires a 8-bit correlation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NTW_ADR0_SZW::_0)
    }
    #[doc = "Network Address 0 requires a 16-bit correlation"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NTW_ADR0_SZW::_1)
    }
    #[doc = "Network Address 0 requires a 24-bit correlation"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(NTW_ADR0_SZW::_2)
    }
    #[doc = "Network Address 0 requires a 32-bit correlation"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(NTW_ADR0_SZW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NTW_ADR1_SZ`"]
pub enum NTW_ADR1_SZW {
    #[doc = "Network Address 1 requires a 8-bit correlation"]
    _0,
    #[doc = "Network Address 1 requires a 16-bit correlation"]
    _1,
    #[doc = "Network Address 1 requires a 24-bit correlation"]
    _2,
    #[doc = "Network Address 1 requires a 32-bit correlation"]
    _3,
}
impl NTW_ADR1_SZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NTW_ADR1_SZW::_0 => 0,
            NTW_ADR1_SZW::_1 => 1,
            NTW_ADR1_SZW::_2 => 2,
            NTW_ADR1_SZW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NTW_ADR1_SZW<'a> {
    w: &'a mut W,
}
impl<'a> _NTW_ADR1_SZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NTW_ADR1_SZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Network Address 1 requires a 8-bit correlation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NTW_ADR1_SZW::_0)
    }
    #[doc = "Network Address 1 requires a 16-bit correlation"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NTW_ADR1_SZW::_1)
    }
    #[doc = "Network Address 1 requires a 24-bit correlation"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(NTW_ADR1_SZW::_2)
    }
    #[doc = "Network Address 1 requires a 32-bit correlation"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(NTW_ADR1_SZW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NTW_ADR2_SZ`"]
pub enum NTW_ADR2_SZW {
    #[doc = "Network Address 2 requires a 8-bit correlation"]
    _0,
    #[doc = "Network Address 2 requires a 16-bit correlation"]
    _1,
    #[doc = "Network Address 2 requires a 24-bit correlation"]
    _2,
    #[doc = "Network Address 2 requires a 32-bit correlation"]
    _3,
}
impl NTW_ADR2_SZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NTW_ADR2_SZW::_0 => 0,
            NTW_ADR2_SZW::_1 => 1,
            NTW_ADR2_SZW::_2 => 2,
            NTW_ADR2_SZW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NTW_ADR2_SZW<'a> {
    w: &'a mut W,
}
impl<'a> _NTW_ADR2_SZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NTW_ADR2_SZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Network Address 2 requires a 8-bit correlation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NTW_ADR2_SZW::_0)
    }
    #[doc = "Network Address 2 requires a 16-bit correlation"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NTW_ADR2_SZW::_1)
    }
    #[doc = "Network Address 2 requires a 24-bit correlation"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(NTW_ADR2_SZW::_2)
    }
    #[doc = "Network Address 2 requires a 32-bit correlation"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(NTW_ADR2_SZW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NTW_ADR3_SZ`"]
pub enum NTW_ADR3_SZW {
    #[doc = "Network Address 3 requires a 8-bit correlation"]
    _0,
    #[doc = "Network Address 3 requires a 16-bit correlation"]
    _1,
    #[doc = "Network Address 3 requires a 24-bit correlation"]
    _2,
    #[doc = "Network Address 3 requires a 32-bit correlation"]
    _3,
}
impl NTW_ADR3_SZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NTW_ADR3_SZW::_0 => 0,
            NTW_ADR3_SZW::_1 => 1,
            NTW_ADR3_SZW::_2 => 2,
            NTW_ADR3_SZW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NTW_ADR3_SZW<'a> {
    w: &'a mut W,
}
impl<'a> _NTW_ADR3_SZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NTW_ADR3_SZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Network Address 3 requires a 8-bit correlation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NTW_ADR3_SZW::_0)
    }
    #[doc = "Network Address 3 requires a 16-bit correlation"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NTW_ADR3_SZW::_1)
    }
    #[doc = "Network Address 3 requires a 24-bit correlation"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(NTW_ADR3_SZW::_2)
    }
    #[doc = "Network Address 3 requires a 32-bit correlation"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(NTW_ADR3_SZW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NTW_ADR_THR0W<'a> {
    w: &'a mut W,
}
impl<'a> _NTW_ADR_THR0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NTW_ADR_THR1W<'a> {
    w: &'a mut W,
}
impl<'a> _NTW_ADR_THR1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NTW_ADR_THR2W<'a> {
    w: &'a mut W,
}
impl<'a> _NTW_ADR_THR2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NTW_ADR_THR3W<'a> {
    w: &'a mut W,
}
impl<'a> _NTW_ADR_THR3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - Network Address Enable"]
    #[inline]
    pub fn ntw_adr_en(&self) -> NTW_ADR_ENR {
        NTW_ADR_ENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Network Address Match"]
    #[inline]
    pub fn ntw_adr_mch(&self) -> NTW_ADR_MCHR {
        NTW_ADR_MCHR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Network Address 0 Size"]
    #[inline]
    pub fn ntw_adr0_sz(&self) -> NTW_ADR0_SZR {
        NTW_ADR0_SZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Network Address 1 Size"]
    #[inline]
    pub fn ntw_adr1_sz(&self) -> NTW_ADR1_SZR {
        NTW_ADR1_SZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Network Address 2 Size"]
    #[inline]
    pub fn ntw_adr2_sz(&self) -> NTW_ADR2_SZR {
        NTW_ADR2_SZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Network Address 3 Size"]
    #[inline]
    pub fn ntw_adr3_sz(&self) -> NTW_ADR3_SZR {
        NTW_ADR3_SZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Network Address 0 Threshold"]
    #[inline]
    pub fn ntw_adr_thr0(&self) -> NTW_ADR_THR0R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NTW_ADR_THR0R { bits }
    }
    #[doc = "Bits 20:22 - Network Address 1 Threshold"]
    #[inline]
    pub fn ntw_adr_thr1(&self) -> NTW_ADR_THR1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NTW_ADR_THR1R { bits }
    }
    #[doc = "Bits 24:26 - Network Address 2 Threshold"]
    #[inline]
    pub fn ntw_adr_thr2(&self) -> NTW_ADR_THR2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NTW_ADR_THR2R { bits }
    }
    #[doc = "Bits 28:30 - Network Address 3 Threshold"]
    #[inline]
    pub fn ntw_adr_thr3(&self) -> NTW_ADR_THR3R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NTW_ADR_THR3R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 21760 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Network Address Enable"]
    #[inline]
    pub fn ntw_adr_en(&mut self) -> _NTW_ADR_ENW {
        _NTW_ADR_ENW { w: self }
    }
    #[doc = "Bits 8:9 - Network Address 0 Size"]
    #[inline]
    pub fn ntw_adr0_sz(&mut self) -> _NTW_ADR0_SZW {
        _NTW_ADR0_SZW { w: self }
    }
    #[doc = "Bits 10:11 - Network Address 1 Size"]
    #[inline]
    pub fn ntw_adr1_sz(&mut self) -> _NTW_ADR1_SZW {
        _NTW_ADR1_SZW { w: self }
    }
    #[doc = "Bits 12:13 - Network Address 2 Size"]
    #[inline]
    pub fn ntw_adr2_sz(&mut self) -> _NTW_ADR2_SZW {
        _NTW_ADR2_SZW { w: self }
    }
    #[doc = "Bits 14:15 - Network Address 3 Size"]
    #[inline]
    pub fn ntw_adr3_sz(&mut self) -> _NTW_ADR3_SZW {
        _NTW_ADR3_SZW { w: self }
    }
    #[doc = "Bits 16:18 - Network Address 0 Threshold"]
    #[inline]
    pub fn ntw_adr_thr0(&mut self) -> _NTW_ADR_THR0W {
        _NTW_ADR_THR0W { w: self }
    }
    #[doc = "Bits 20:22 - Network Address 1 Threshold"]
    #[inline]
    pub fn ntw_adr_thr1(&mut self) -> _NTW_ADR_THR1W {
        _NTW_ADR_THR1W { w: self }
    }
    #[doc = "Bits 24:26 - Network Address 2 Threshold"]
    #[inline]
    pub fn ntw_adr_thr2(&mut self) -> _NTW_ADR_THR2W {
        _NTW_ADR_THR2W { w: self }
    }
    #[doc = "Bits 28:30 - Network Address 3 Threshold"]
    #[inline]
    pub fn ntw_adr_thr3(&mut self) -> _NTW_ADR_THR3W {
        _NTW_ADR_THR3W { w: self }
    }
}
