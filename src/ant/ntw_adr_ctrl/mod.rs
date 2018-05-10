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
#[doc = r" Value of the field"]
pub struct NTW_ADR_ENR {
    bits: u8,
}
impl NTW_ADR_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NTW_ADR_MCHR {
    bits: u8,
}
impl NTW_ADR_MCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `NTW_ADR0_SZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NTW_ADR0_SZR {
    #[doc = "1 octet"]
    _0,
    #[doc = "2 octets"]
    _1,
    #[doc = "3 octets"]
    _2,
    #[doc = "4 octets"]
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
    #[doc = "1 octet"]
    _0,
    #[doc = "2 octets"]
    _1,
    #[doc = "3 octets"]
    _2,
    #[doc = "4 octets"]
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
    #[doc = "1 octet"]
    _0,
    #[doc = "2 octets"]
    _1,
    #[doc = "3 octets"]
    _2,
    #[doc = "4 octets"]
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
    #[doc = "1 octet"]
    _0,
    #[doc = "2 octets"]
    _1,
    #[doc = "3 octets"]
    _2,
    #[doc = "4 octets"]
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
#[doc = r" Proxy"]
pub struct _NTW_ADR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _NTW_ADR_ENW<'a> {
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
    #[doc = "1 octet"]
    _0,
    #[doc = "2 octets"]
    _1,
    #[doc = "3 octets"]
    _2,
    #[doc = "4 octets"]
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
    #[doc = "1 octet"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NTW_ADR0_SZW::_0)
    }
    #[doc = "2 octets"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NTW_ADR0_SZW::_1)
    }
    #[doc = "3 octets"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(NTW_ADR0_SZW::_2)
    }
    #[doc = "4 octets"]
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
    #[doc = "1 octet"]
    _0,
    #[doc = "2 octets"]
    _1,
    #[doc = "3 octets"]
    _2,
    #[doc = "4 octets"]
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
    #[doc = "1 octet"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NTW_ADR1_SZW::_0)
    }
    #[doc = "2 octets"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NTW_ADR1_SZW::_1)
    }
    #[doc = "3 octets"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(NTW_ADR1_SZW::_2)
    }
    #[doc = "4 octets"]
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
    #[doc = "1 octet"]
    _0,
    #[doc = "2 octets"]
    _1,
    #[doc = "3 octets"]
    _2,
    #[doc = "4 octets"]
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
    #[doc = "1 octet"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NTW_ADR2_SZW::_0)
    }
    #[doc = "2 octets"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NTW_ADR2_SZW::_1)
    }
    #[doc = "3 octets"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(NTW_ADR2_SZW::_2)
    }
    #[doc = "4 octets"]
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
    #[doc = "1 octet"]
    _0,
    #[doc = "2 octets"]
    _1,
    #[doc = "3 octets"]
    _2,
    #[doc = "4 octets"]
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
    #[doc = "1 octet"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NTW_ADR3_SZW::_0)
    }
    #[doc = "2 octets"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NTW_ADR3_SZW::_1)
    }
    #[doc = "3 octets"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(NTW_ADR3_SZW::_2)
    }
    #[doc = "4 octets"]
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
    #[doc = "Bits 0:3 - Network Address Match Enable"]
    #[inline]
    pub fn ntw_adr_en(&self) -> NTW_ADR_ENR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NTW_ADR_ENR { bits }
    }
    #[doc = "Bits 4:7 - Network Address Match Status"]
    #[inline]
    pub fn ntw_adr_mch(&self) -> NTW_ADR_MCHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NTW_ADR_MCHR { bits }
    }
    #[doc = "Bits 8:9 - Network Address Match Size"]
    #[inline]
    pub fn ntw_adr0_sz(&self) -> NTW_ADR0_SZR {
        NTW_ADR0_SZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Network Address Match Size"]
    #[inline]
    pub fn ntw_adr1_sz(&self) -> NTW_ADR1_SZR {
        NTW_ADR1_SZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Network Address Match Size"]
    #[inline]
    pub fn ntw_adr2_sz(&self) -> NTW_ADR2_SZR {
        NTW_ADR2_SZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Network Address Match Size"]
    #[inline]
    pub fn ntw_adr3_sz(&self) -> NTW_ADR3_SZR {
        NTW_ADR3_SZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Network Address Match Bit Error Threshold 0"]
    #[inline]
    pub fn ntw_adr_thr0(&self) -> NTW_ADR_THR0R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NTW_ADR_THR0R { bits }
    }
    #[doc = "Bits 20:22 - Network Address Match Bit Error Threshold 1"]
    #[inline]
    pub fn ntw_adr_thr1(&self) -> NTW_ADR_THR1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NTW_ADR_THR1R { bits }
    }
    #[doc = "Bits 24:26 - Network Address Match Bit Error Threshold 2"]
    #[inline]
    pub fn ntw_adr_thr2(&self) -> NTW_ADR_THR2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NTW_ADR_THR2R { bits }
    }
    #[doc = "Bits 28:30 - Network Address Match Bit Error Threshold 3"]
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
    #[doc = "Bits 0:3 - Network Address Match Enable"]
    #[inline]
    pub fn ntw_adr_en(&mut self) -> _NTW_ADR_ENW {
        _NTW_ADR_ENW { w: self }
    }
    #[doc = "Bits 8:9 - Network Address Match Size"]
    #[inline]
    pub fn ntw_adr0_sz(&mut self) -> _NTW_ADR0_SZW {
        _NTW_ADR0_SZW { w: self }
    }
    #[doc = "Bits 10:11 - Network Address Match Size"]
    #[inline]
    pub fn ntw_adr1_sz(&mut self) -> _NTW_ADR1_SZW {
        _NTW_ADR1_SZW { w: self }
    }
    #[doc = "Bits 12:13 - Network Address Match Size"]
    #[inline]
    pub fn ntw_adr2_sz(&mut self) -> _NTW_ADR2_SZW {
        _NTW_ADR2_SZW { w: self }
    }
    #[doc = "Bits 14:15 - Network Address Match Size"]
    #[inline]
    pub fn ntw_adr3_sz(&mut self) -> _NTW_ADR3_SZW {
        _NTW_ADR3_SZW { w: self }
    }
    #[doc = "Bits 16:18 - Network Address Match Bit Error Threshold 0"]
    #[inline]
    pub fn ntw_adr_thr0(&mut self) -> _NTW_ADR_THR0W {
        _NTW_ADR_THR0W { w: self }
    }
    #[doc = "Bits 20:22 - Network Address Match Bit Error Threshold 1"]
    #[inline]
    pub fn ntw_adr_thr1(&mut self) -> _NTW_ADR_THR1W {
        _NTW_ADR_THR1W { w: self }
    }
    #[doc = "Bits 24:26 - Network Address Match Bit Error Threshold 2"]
    #[inline]
    pub fn ntw_adr_thr2(&mut self) -> _NTW_ADR_THR2W {
        _NTW_ADR_THR2W { w: self }
    }
    #[doc = "Bits 28:30 - Network Address Match Bit Error Threshold 3"]
    #[inline]
    pub fn ntw_adr_thr3(&mut self) -> _NTW_ADR_THR3W {
        _NTW_ADR_THR3W { w: self }
    }
}
