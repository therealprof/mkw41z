#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPM_CAL1 {
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
pub struct HPM_COUNT_1R {
    bits: u32,
}
impl HPM_COUNT_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `CS_WT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_WTR {
    #[doc = "128 us"]
    _000,
    #[doc = "256 us"]
    _001,
    #[doc = "384 us"]
    _010,
    #[doc = "512 us"]
    _011,
    #[doc = "640 us"]
    _100,
    #[doc = "768 us"]
    _101,
    #[doc = "896 us"]
    _110,
    #[doc = "1024 us"]
    _111,
}
impl CS_WTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CS_WTR::_000 => 0,
            CS_WTR::_001 => 1,
            CS_WTR::_010 => 2,
            CS_WTR::_011 => 3,
            CS_WTR::_100 => 4,
            CS_WTR::_101 => 5,
            CS_WTR::_110 => 6,
            CS_WTR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CS_WTR {
        match value {
            0 => CS_WTR::_000,
            1 => CS_WTR::_001,
            2 => CS_WTR::_010,
            3 => CS_WTR::_011,
            4 => CS_WTR::_100,
            5 => CS_WTR::_101,
            6 => CS_WTR::_110,
            7 => CS_WTR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == CS_WTR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == CS_WTR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == CS_WTR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == CS_WTR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == CS_WTR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == CS_WTR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == CS_WTR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == CS_WTR::_111
    }
}
#[doc = "Possible values of the field `CS_FW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_FWR {
    #[doc = "8 us"]
    _000,
    #[doc = "16 us"]
    _001,
    #[doc = "24 us"]
    _010,
    #[doc = "32 us"]
    _011,
    #[doc = "64 us"]
    _100,
    #[doc = "96 us"]
    _101,
    #[doc = "128 us"]
    _110,
    #[doc = "256 us"]
    _111,
}
impl CS_FWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CS_FWR::_000 => 0,
            CS_FWR::_001 => 1,
            CS_FWR::_010 => 2,
            CS_FWR::_011 => 3,
            CS_FWR::_100 => 4,
            CS_FWR::_101 => 5,
            CS_FWR::_110 => 6,
            CS_FWR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CS_FWR {
        match value {
            0 => CS_FWR::_000,
            1 => CS_FWR::_001,
            2 => CS_FWR::_010,
            3 => CS_FWR::_011,
            4 => CS_FWR::_100,
            5 => CS_FWR::_101,
            6 => CS_FWR::_110,
            7 => CS_FWR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == CS_FWR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == CS_FWR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == CS_FWR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == CS_FWR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == CS_FWR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == CS_FWR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == CS_FWR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == CS_FWR::_111
    }
}
#[doc = r" Value of the field"]
pub struct CS_FCNTR {
    bits: u8,
}
impl CS_FCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CS_WT`"]
pub enum CS_WTW {
    #[doc = "128 us"]
    _000,
    #[doc = "256 us"]
    _001,
    #[doc = "384 us"]
    _010,
    #[doc = "512 us"]
    _011,
    #[doc = "640 us"]
    _100,
    #[doc = "768 us"]
    _101,
    #[doc = "896 us"]
    _110,
    #[doc = "1024 us"]
    _111,
}
impl CS_WTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CS_WTW::_000 => 0,
            CS_WTW::_001 => 1,
            CS_WTW::_010 => 2,
            CS_WTW::_011 => 3,
            CS_WTW::_100 => 4,
            CS_WTW::_101 => 5,
            CS_WTW::_110 => 6,
            CS_WTW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CS_WTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS_WTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CS_WTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "128 us"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(CS_WTW::_000)
    }
    #[doc = "256 us"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(CS_WTW::_001)
    }
    #[doc = "384 us"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(CS_WTW::_010)
    }
    #[doc = "512 us"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(CS_WTW::_011)
    }
    #[doc = "640 us"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(CS_WTW::_100)
    }
    #[doc = "768 us"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(CS_WTW::_101)
    }
    #[doc = "896 us"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(CS_WTW::_110)
    }
    #[doc = "1024 us"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(CS_WTW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CS_FW`"]
pub enum CS_FWW {
    #[doc = "8 us"]
    _000,
    #[doc = "16 us"]
    _001,
    #[doc = "24 us"]
    _010,
    #[doc = "32 us"]
    _011,
    #[doc = "64 us"]
    _100,
    #[doc = "96 us"]
    _101,
    #[doc = "128 us"]
    _110,
    #[doc = "256 us"]
    _111,
}
impl CS_FWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CS_FWW::_000 => 0,
            CS_FWW::_001 => 1,
            CS_FWW::_010 => 2,
            CS_FWW::_011 => 3,
            CS_FWW::_100 => 4,
            CS_FWW::_101 => 5,
            CS_FWW::_110 => 6,
            CS_FWW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CS_FWW<'a> {
    w: &'a mut W,
}
impl<'a> _CS_FWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CS_FWW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8 us"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(CS_FWW::_000)
    }
    #[doc = "16 us"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(CS_FWW::_001)
    }
    #[doc = "24 us"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(CS_FWW::_010)
    }
    #[doc = "32 us"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(CS_FWW::_011)
    }
    #[doc = "64 us"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(CS_FWW::_100)
    }
    #[doc = "96 us"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(CS_FWW::_101)
    }
    #[doc = "128 us"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(CS_FWW::_110)
    }
    #[doc = "256 us"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(CS_FWW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CS_FCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS_FCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:18 - High Port Modulation Counter Value 1"]
    #[inline]
    pub fn hpm_count_1(&self) -> HPM_COUNT_1R {
        let bits = {
            const MASK: u32 = 524287;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        HPM_COUNT_1R { bits }
    }
    #[doc = "Bits 20:22 - Cycle Slip Wait Time"]
    #[inline]
    pub fn cs_wt(&self) -> CS_WTR {
        CS_WTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Cycle Slip Flag Window"]
    #[inline]
    pub fn cs_fw(&self) -> CS_FWR {
        CS_FWR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Cycle Slip Flag Count"]
    #[inline]
    pub fn cs_fcnt(&self) -> CS_FCNTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS_FCNTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1143996416 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 20:22 - Cycle Slip Wait Time"]
    #[inline]
    pub fn cs_wt(&mut self) -> _CS_WTW {
        _CS_WTW { w: self }
    }
    #[doc = "Bits 24:26 - Cycle Slip Flag Window"]
    #[inline]
    pub fn cs_fw(&mut self) -> _CS_FWW {
        _CS_FWW { w: self }
    }
    #[doc = "Bits 28:31 - Cycle Slip Flag Count"]
    #[inline]
    pub fn cs_fcnt(&mut self) -> _CS_FCNTW {
        _CS_FCNTW { w: self }
    }
}
