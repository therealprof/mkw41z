#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCOC_CAL_IIR {
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
#[doc = "Possible values of the field `DCOC_CAL_IIR1A_IDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_CAL_IIR1A_IDXR {
    #[doc = "1/1"]
    _0,
    #[doc = "1/4"]
    _1,
    #[doc = "1/8"]
    _2,
    #[doc = "1/16"]
    _3,
}
impl DCOC_CAL_IIR1A_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCOC_CAL_IIR1A_IDXR::_0 => 0,
            DCOC_CAL_IIR1A_IDXR::_1 => 1,
            DCOC_CAL_IIR1A_IDXR::_2 => 2,
            DCOC_CAL_IIR1A_IDXR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCOC_CAL_IIR1A_IDXR {
        match value {
            0 => DCOC_CAL_IIR1A_IDXR::_0,
            1 => DCOC_CAL_IIR1A_IDXR::_1,
            2 => DCOC_CAL_IIR1A_IDXR::_2,
            3 => DCOC_CAL_IIR1A_IDXR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCOC_CAL_IIR1A_IDXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCOC_CAL_IIR1A_IDXR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == DCOC_CAL_IIR1A_IDXR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == DCOC_CAL_IIR1A_IDXR::_3
    }
}
#[doc = "Possible values of the field `DCOC_CAL_IIR2A_IDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_CAL_IIR2A_IDXR {
    #[doc = "1/1"]
    _0,
    #[doc = "1/4"]
    _1,
    #[doc = "1/8"]
    _2,
    #[doc = "1/16"]
    _3,
}
impl DCOC_CAL_IIR2A_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCOC_CAL_IIR2A_IDXR::_0 => 0,
            DCOC_CAL_IIR2A_IDXR::_1 => 1,
            DCOC_CAL_IIR2A_IDXR::_2 => 2,
            DCOC_CAL_IIR2A_IDXR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCOC_CAL_IIR2A_IDXR {
        match value {
            0 => DCOC_CAL_IIR2A_IDXR::_0,
            1 => DCOC_CAL_IIR2A_IDXR::_1,
            2 => DCOC_CAL_IIR2A_IDXR::_2,
            3 => DCOC_CAL_IIR2A_IDXR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCOC_CAL_IIR2A_IDXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCOC_CAL_IIR2A_IDXR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == DCOC_CAL_IIR2A_IDXR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == DCOC_CAL_IIR2A_IDXR::_3
    }
}
#[doc = "Possible values of the field `DCOC_CAL_IIR3A_IDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOC_CAL_IIR3A_IDXR {
    #[doc = "1/4"]
    _0,
    #[doc = "1/8"]
    _1,
    #[doc = "1/16"]
    _2,
    #[doc = "1/32"]
    _3,
}
impl DCOC_CAL_IIR3A_IDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCOC_CAL_IIR3A_IDXR::_0 => 0,
            DCOC_CAL_IIR3A_IDXR::_1 => 1,
            DCOC_CAL_IIR3A_IDXR::_2 => 2,
            DCOC_CAL_IIR3A_IDXR::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCOC_CAL_IIR3A_IDXR {
        match value {
            0 => DCOC_CAL_IIR3A_IDXR::_0,
            1 => DCOC_CAL_IIR3A_IDXR::_1,
            2 => DCOC_CAL_IIR3A_IDXR::_2,
            3 => DCOC_CAL_IIR3A_IDXR::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCOC_CAL_IIR3A_IDXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCOC_CAL_IIR3A_IDXR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == DCOC_CAL_IIR3A_IDXR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == DCOC_CAL_IIR3A_IDXR::_3
    }
}
#[doc = "Values that can be written to the field `DCOC_CAL_IIR1A_IDX`"]
pub enum DCOC_CAL_IIR1A_IDXW {
    #[doc = "1/1"]
    _0,
    #[doc = "1/4"]
    _1,
    #[doc = "1/8"]
    _2,
    #[doc = "1/16"]
    _3,
}
impl DCOC_CAL_IIR1A_IDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCOC_CAL_IIR1A_IDXW::_0 => 0,
            DCOC_CAL_IIR1A_IDXW::_1 => 1,
            DCOC_CAL_IIR1A_IDXW::_2 => 2,
            DCOC_CAL_IIR1A_IDXW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_CAL_IIR1A_IDXW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_CAL_IIR1A_IDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_CAL_IIR1A_IDXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1/1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCOC_CAL_IIR1A_IDXW::_0)
    }
    #[doc = "1/4"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCOC_CAL_IIR1A_IDXW::_1)
    }
    #[doc = "1/8"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(DCOC_CAL_IIR1A_IDXW::_2)
    }
    #[doc = "1/16"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(DCOC_CAL_IIR1A_IDXW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCOC_CAL_IIR2A_IDX`"]
pub enum DCOC_CAL_IIR2A_IDXW {
    #[doc = "1/1"]
    _0,
    #[doc = "1/4"]
    _1,
    #[doc = "1/8"]
    _2,
    #[doc = "1/16"]
    _3,
}
impl DCOC_CAL_IIR2A_IDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCOC_CAL_IIR2A_IDXW::_0 => 0,
            DCOC_CAL_IIR2A_IDXW::_1 => 1,
            DCOC_CAL_IIR2A_IDXW::_2 => 2,
            DCOC_CAL_IIR2A_IDXW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_CAL_IIR2A_IDXW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_CAL_IIR2A_IDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_CAL_IIR2A_IDXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1/1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCOC_CAL_IIR2A_IDXW::_0)
    }
    #[doc = "1/4"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCOC_CAL_IIR2A_IDXW::_1)
    }
    #[doc = "1/8"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(DCOC_CAL_IIR2A_IDXW::_2)
    }
    #[doc = "1/16"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(DCOC_CAL_IIR2A_IDXW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCOC_CAL_IIR3A_IDX`"]
pub enum DCOC_CAL_IIR3A_IDXW {
    #[doc = "1/4"]
    _0,
    #[doc = "1/8"]
    _1,
    #[doc = "1/16"]
    _2,
    #[doc = "1/32"]
    _3,
}
impl DCOC_CAL_IIR3A_IDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCOC_CAL_IIR3A_IDXW::_0 => 0,
            DCOC_CAL_IIR3A_IDXW::_1 => 1,
            DCOC_CAL_IIR3A_IDXW::_2 => 2,
            DCOC_CAL_IIR3A_IDXW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOC_CAL_IIR3A_IDXW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOC_CAL_IIR3A_IDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOC_CAL_IIR3A_IDXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1/4"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCOC_CAL_IIR3A_IDXW::_0)
    }
    #[doc = "1/8"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCOC_CAL_IIR3A_IDXW::_1)
    }
    #[doc = "1/16"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(DCOC_CAL_IIR3A_IDXW::_2)
    }
    #[doc = "1/32"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(DCOC_CAL_IIR3A_IDXW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - DCOC Calibration IIR 1A Index"]
    #[inline]
    pub fn dcoc_cal_iir1a_idx(&self) -> DCOC_CAL_IIR1A_IDXR {
        DCOC_CAL_IIR1A_IDXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - DCOC Calibration IIR 2A Index"]
    #[inline]
    pub fn dcoc_cal_iir2a_idx(&self) -> DCOC_CAL_IIR2A_IDXR {
        DCOC_CAL_IIR2A_IDXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - DCOC Calibration IIR 3A Index"]
    #[inline]
    pub fn dcoc_cal_iir3a_idx(&self) -> DCOC_CAL_IIR3A_IDXR {
        DCOC_CAL_IIR3A_IDXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:1 - DCOC Calibration IIR 1A Index"]
    #[inline]
    pub fn dcoc_cal_iir1a_idx(&mut self) -> _DCOC_CAL_IIR1A_IDXW {
        _DCOC_CAL_IIR1A_IDXW { w: self }
    }
    #[doc = "Bits 2:3 - DCOC Calibration IIR 2A Index"]
    #[inline]
    pub fn dcoc_cal_iir2a_idx(&mut self) -> _DCOC_CAL_IIR2A_IDXW {
        _DCOC_CAL_IIR2A_IDXW { w: self }
    }
    #[doc = "Bits 4:5 - DCOC Calibration IIR 3A Index"]
    #[inline]
    pub fn dcoc_cal_iir3a_idx(&mut self) -> _DCOC_CAL_IIR3A_IDXW {
        _DCOC_CAL_IIR3A_IDXW { w: self }
    }
}
