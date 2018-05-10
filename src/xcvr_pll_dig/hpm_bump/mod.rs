#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPM_BUMP {
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
#[doc = "Possible values of the field `HPM_VCM_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPM_VCM_TXR {
    #[doc = "432 mV"]
    _000,
    #[doc = "328 mV"]
    _001,
    #[doc = "456 mV"]
    _010,
    #[doc = "473 mV"]
    _011,
    #[doc = "488 mV"]
    _100,
    #[doc = "408 mV"]
    _101,
    #[doc = "392 mV"]
    _110,
    #[doc = "376 mV"]
    _111,
}
impl HPM_VCM_TXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HPM_VCM_TXR::_000 => 0,
            HPM_VCM_TXR::_001 => 1,
            HPM_VCM_TXR::_010 => 2,
            HPM_VCM_TXR::_011 => 3,
            HPM_VCM_TXR::_100 => 4,
            HPM_VCM_TXR::_101 => 5,
            HPM_VCM_TXR::_110 => 6,
            HPM_VCM_TXR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HPM_VCM_TXR {
        match value {
            0 => HPM_VCM_TXR::_000,
            1 => HPM_VCM_TXR::_001,
            2 => HPM_VCM_TXR::_010,
            3 => HPM_VCM_TXR::_011,
            4 => HPM_VCM_TXR::_100,
            5 => HPM_VCM_TXR::_101,
            6 => HPM_VCM_TXR::_110,
            7 => HPM_VCM_TXR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == HPM_VCM_TXR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == HPM_VCM_TXR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == HPM_VCM_TXR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == HPM_VCM_TXR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == HPM_VCM_TXR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == HPM_VCM_TXR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == HPM_VCM_TXR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == HPM_VCM_TXR::_111
    }
}
#[doc = "Possible values of the field `HPM_VCM_CAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPM_VCM_CALR {
    #[doc = "432 mV"]
    _000,
    #[doc = "328 mV"]
    _001,
    #[doc = "456 mV"]
    _010,
    #[doc = "473 mV"]
    _011,
    #[doc = "488 mV"]
    _100,
    #[doc = "408 mV"]
    _101,
    #[doc = "392 mV"]
    _110,
    #[doc = "376 mV"]
    _111,
}
impl HPM_VCM_CALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HPM_VCM_CALR::_000 => 0,
            HPM_VCM_CALR::_001 => 1,
            HPM_VCM_CALR::_010 => 2,
            HPM_VCM_CALR::_011 => 3,
            HPM_VCM_CALR::_100 => 4,
            HPM_VCM_CALR::_101 => 5,
            HPM_VCM_CALR::_110 => 6,
            HPM_VCM_CALR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HPM_VCM_CALR {
        match value {
            0 => HPM_VCM_CALR::_000,
            1 => HPM_VCM_CALR::_001,
            2 => HPM_VCM_CALR::_010,
            3 => HPM_VCM_CALR::_011,
            4 => HPM_VCM_CALR::_100,
            5 => HPM_VCM_CALR::_101,
            6 => HPM_VCM_CALR::_110,
            7 => HPM_VCM_CALR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == HPM_VCM_CALR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == HPM_VCM_CALR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == HPM_VCM_CALR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == HPM_VCM_CALR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == HPM_VCM_CALR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == HPM_VCM_CALR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == HPM_VCM_CALR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == HPM_VCM_CALR::_111
    }
}
#[doc = "Possible values of the field `HPM_FDB_RES_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPM_FDB_RES_TXR {
    #[doc = "29 kohms"]
    _00,
    #[doc = "58 kohms(gain of 2)"]
    _01,
    #[doc = "13 kohms"]
    _10,
    #[doc = "23.7 kohms"]
    _11,
}
impl HPM_FDB_RES_TXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HPM_FDB_RES_TXR::_00 => 0,
            HPM_FDB_RES_TXR::_01 => 1,
            HPM_FDB_RES_TXR::_10 => 2,
            HPM_FDB_RES_TXR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HPM_FDB_RES_TXR {
        match value {
            0 => HPM_FDB_RES_TXR::_00,
            1 => HPM_FDB_RES_TXR::_01,
            2 => HPM_FDB_RES_TXR::_10,
            3 => HPM_FDB_RES_TXR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == HPM_FDB_RES_TXR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == HPM_FDB_RES_TXR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == HPM_FDB_RES_TXR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == HPM_FDB_RES_TXR::_11
    }
}
#[doc = "Possible values of the field `HPM_FDB_RES_CAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPM_FDB_RES_CALR {
    #[doc = "29 kohms"]
    _00,
    #[doc = "58 kohms(gain of 2)"]
    _01,
    #[doc = "13 kohms"]
    _10,
    #[doc = "23.7 kohms"]
    _11,
}
impl HPM_FDB_RES_CALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HPM_FDB_RES_CALR::_00 => 0,
            HPM_FDB_RES_CALR::_01 => 1,
            HPM_FDB_RES_CALR::_10 => 2,
            HPM_FDB_RES_CALR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HPM_FDB_RES_CALR {
        match value {
            0 => HPM_FDB_RES_CALR::_00,
            1 => HPM_FDB_RES_CALR::_01,
            2 => HPM_FDB_RES_CALR::_10,
            3 => HPM_FDB_RES_CALR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == HPM_FDB_RES_CALR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == HPM_FDB_RES_CALR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == HPM_FDB_RES_CALR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == HPM_FDB_RES_CALR::_11
    }
}
#[doc = "Values that can be written to the field `HPM_VCM_TX`"]
pub enum HPM_VCM_TXW {
    #[doc = "432 mV"]
    _000,
    #[doc = "328 mV"]
    _001,
    #[doc = "456 mV"]
    _010,
    #[doc = "473 mV"]
    _011,
    #[doc = "488 mV"]
    _100,
    #[doc = "408 mV"]
    _101,
    #[doc = "392 mV"]
    _110,
    #[doc = "376 mV"]
    _111,
}
impl HPM_VCM_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HPM_VCM_TXW::_000 => 0,
            HPM_VCM_TXW::_001 => 1,
            HPM_VCM_TXW::_010 => 2,
            HPM_VCM_TXW::_011 => 3,
            HPM_VCM_TXW::_100 => 4,
            HPM_VCM_TXW::_101 => 5,
            HPM_VCM_TXW::_110 => 6,
            HPM_VCM_TXW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPM_VCM_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_VCM_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPM_VCM_TXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "432 mV"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(HPM_VCM_TXW::_000)
    }
    #[doc = "328 mV"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(HPM_VCM_TXW::_001)
    }
    #[doc = "456 mV"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(HPM_VCM_TXW::_010)
    }
    #[doc = "473 mV"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(HPM_VCM_TXW::_011)
    }
    #[doc = "488 mV"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(HPM_VCM_TXW::_100)
    }
    #[doc = "408 mV"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(HPM_VCM_TXW::_101)
    }
    #[doc = "392 mV"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(HPM_VCM_TXW::_110)
    }
    #[doc = "376 mV"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(HPM_VCM_TXW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HPM_VCM_CAL`"]
pub enum HPM_VCM_CALW {
    #[doc = "432 mV"]
    _000,
    #[doc = "328 mV"]
    _001,
    #[doc = "456 mV"]
    _010,
    #[doc = "473 mV"]
    _011,
    #[doc = "488 mV"]
    _100,
    #[doc = "408 mV"]
    _101,
    #[doc = "392 mV"]
    _110,
    #[doc = "376 mV"]
    _111,
}
impl HPM_VCM_CALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HPM_VCM_CALW::_000 => 0,
            HPM_VCM_CALW::_001 => 1,
            HPM_VCM_CALW::_010 => 2,
            HPM_VCM_CALW::_011 => 3,
            HPM_VCM_CALW::_100 => 4,
            HPM_VCM_CALW::_101 => 5,
            HPM_VCM_CALW::_110 => 6,
            HPM_VCM_CALW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPM_VCM_CALW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_VCM_CALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPM_VCM_CALW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "432 mV"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(HPM_VCM_CALW::_000)
    }
    #[doc = "328 mV"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(HPM_VCM_CALW::_001)
    }
    #[doc = "456 mV"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(HPM_VCM_CALW::_010)
    }
    #[doc = "473 mV"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(HPM_VCM_CALW::_011)
    }
    #[doc = "488 mV"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(HPM_VCM_CALW::_100)
    }
    #[doc = "408 mV"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(HPM_VCM_CALW::_101)
    }
    #[doc = "392 mV"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(HPM_VCM_CALW::_110)
    }
    #[doc = "376 mV"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(HPM_VCM_CALW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HPM_FDB_RES_TX`"]
pub enum HPM_FDB_RES_TXW {
    #[doc = "29 kohms"]
    _00,
    #[doc = "58 kohms(gain of 2)"]
    _01,
    #[doc = "13 kohms"]
    _10,
    #[doc = "23.7 kohms"]
    _11,
}
impl HPM_FDB_RES_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HPM_FDB_RES_TXW::_00 => 0,
            HPM_FDB_RES_TXW::_01 => 1,
            HPM_FDB_RES_TXW::_10 => 2,
            HPM_FDB_RES_TXW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPM_FDB_RES_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_FDB_RES_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPM_FDB_RES_TXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "29 kohms"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(HPM_FDB_RES_TXW::_00)
    }
    #[doc = "58 kohms(gain of 2)"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(HPM_FDB_RES_TXW::_01)
    }
    #[doc = "13 kohms"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(HPM_FDB_RES_TXW::_10)
    }
    #[doc = "23.7 kohms"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(HPM_FDB_RES_TXW::_11)
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
#[doc = "Values that can be written to the field `HPM_FDB_RES_CAL`"]
pub enum HPM_FDB_RES_CALW {
    #[doc = "29 kohms"]
    _00,
    #[doc = "58 kohms(gain of 2)"]
    _01,
    #[doc = "13 kohms"]
    _10,
    #[doc = "23.7 kohms"]
    _11,
}
impl HPM_FDB_RES_CALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HPM_FDB_RES_CALW::_00 => 0,
            HPM_FDB_RES_CALW::_01 => 1,
            HPM_FDB_RES_CALW::_10 => 2,
            HPM_FDB_RES_CALW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPM_FDB_RES_CALW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_FDB_RES_CALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPM_FDB_RES_CALW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "29 kohms"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(HPM_FDB_RES_CALW::_00)
    }
    #[doc = "58 kohms(gain of 2)"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(HPM_FDB_RES_CALW::_01)
    }
    #[doc = "13 kohms"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(HPM_FDB_RES_CALW::_10)
    }
    #[doc = "23.7 kohms"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(HPM_FDB_RES_CALW::_11)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - rfctrl_tx_dac_bump_vcm[2:0] during Transmission"]
    #[inline]
    pub fn hpm_vcm_tx(&self) -> HPM_VCM_TXR {
        HPM_VCM_TXR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - rfctrl_tx_dac_bump_vcm[2:0] during Calibration"]
    #[inline]
    pub fn hpm_vcm_cal(&self) -> HPM_VCM_CALR {
        HPM_VCM_CALR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - rfctrl_tx_dac_bump_fdb_res[1:0] during Transmission"]
    #[inline]
    pub fn hpm_fdb_res_tx(&self) -> HPM_FDB_RES_TXR {
        HPM_FDB_RES_TXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - rfctrl_tx_dac_bump_fdb_res[1:0] during Calibration"]
    #[inline]
    pub fn hpm_fdb_res_cal(&self) -> HPM_FDB_RES_CALR {
        HPM_FDB_RES_CALR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4112 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - rfctrl_tx_dac_bump_vcm[2:0] during Transmission"]
    #[inline]
    pub fn hpm_vcm_tx(&mut self) -> _HPM_VCM_TXW {
        _HPM_VCM_TXW { w: self }
    }
    #[doc = "Bits 4:6 - rfctrl_tx_dac_bump_vcm[2:0] during Calibration"]
    #[inline]
    pub fn hpm_vcm_cal(&mut self) -> _HPM_VCM_CALW {
        _HPM_VCM_CALW { w: self }
    }
    #[doc = "Bits 8:9 - rfctrl_tx_dac_bump_fdb_res[1:0] during Transmission"]
    #[inline]
    pub fn hpm_fdb_res_tx(&mut self) -> _HPM_FDB_RES_TXW {
        _HPM_FDB_RES_TXW { w: self }
    }
    #[doc = "Bits 12:13 - rfctrl_tx_dac_bump_fdb_res[1:0] during Calibration"]
    #[inline]
    pub fn hpm_fdb_res_cal(&mut self) -> _HPM_FDB_RES_CALW {
        _HPM_FDB_RES_CALW { w: self }
    }
}
