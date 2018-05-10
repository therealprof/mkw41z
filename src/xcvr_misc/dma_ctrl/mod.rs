#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_CTRL {
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
#[doc = "Possible values of the field `DMA_PAGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_PAGER {
    #[doc = "DMA Idle"]
    _0000,
    #[doc = "RX_DIG I and Q"]
    _0001,
    #[doc = "RX_DIG I Only"]
    _0010,
    #[doc = "RX_DIG Q Only"]
    _0011,
    #[doc = "RAW ADC I and Q"]
    _0100,
    #[doc = "RAW ADC I Only"]
    _0101,
    #[doc = "RAW ADC Q only"]
    _0110,
    #[doc = "DC Estimator I and Q"]
    _0111,
    #[doc = "DC Estimator I Only"]
    _1000,
    #[doc = "DC Estimator Q only"]
    _1001,
    #[doc = "RX_DIG Phase Output"]
    _1010,
    #[doc = "Demodulator Hard Decision"]
    _1011,
    #[doc = "Demodulator Soft Decision"]
    _1100,
    #[doc = "Demodulator Data Output"]
    _1101,
    #[doc = "Demodulator CFO Phase Output"]
    _1110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DMA_PAGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMA_PAGER::_0000 => 0,
            DMA_PAGER::_0001 => 1,
            DMA_PAGER::_0010 => 2,
            DMA_PAGER::_0011 => 3,
            DMA_PAGER::_0100 => 4,
            DMA_PAGER::_0101 => 5,
            DMA_PAGER::_0110 => 6,
            DMA_PAGER::_0111 => 7,
            DMA_PAGER::_1000 => 8,
            DMA_PAGER::_1001 => 9,
            DMA_PAGER::_1010 => 10,
            DMA_PAGER::_1011 => 11,
            DMA_PAGER::_1100 => 12,
            DMA_PAGER::_1101 => 13,
            DMA_PAGER::_1110 => 14,
            DMA_PAGER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMA_PAGER {
        match value {
            0 => DMA_PAGER::_0000,
            1 => DMA_PAGER::_0001,
            2 => DMA_PAGER::_0010,
            3 => DMA_PAGER::_0011,
            4 => DMA_PAGER::_0100,
            5 => DMA_PAGER::_0101,
            6 => DMA_PAGER::_0110,
            7 => DMA_PAGER::_0111,
            8 => DMA_PAGER::_1000,
            9 => DMA_PAGER::_1001,
            10 => DMA_PAGER::_1010,
            11 => DMA_PAGER::_1011,
            12 => DMA_PAGER::_1100,
            13 => DMA_PAGER::_1101,
            14 => DMA_PAGER::_1110,
            i => DMA_PAGER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == DMA_PAGER::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == DMA_PAGER::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == DMA_PAGER::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == DMA_PAGER::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == DMA_PAGER::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == DMA_PAGER::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == DMA_PAGER::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == DMA_PAGER::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == DMA_PAGER::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == DMA_PAGER::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == DMA_PAGER::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == DMA_PAGER::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == DMA_PAGER::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == DMA_PAGER::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == DMA_PAGER::_1110
    }
}
#[doc = "Possible values of the field `SINGLE_REQ_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SINGLE_REQ_MODER {
    #[doc = "Disable Single Request Mode. The transceiver will assert ipd_req_radio_rx whenever it has a new sample ready for transfer."]
    _0,
    #[doc = "Enable Single Request Mode. A single initial request by the transceiver will transfer the entire DMA block of data"]
    _1,
}
impl SINGLE_REQ_MODER {
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
            SINGLE_REQ_MODER::_0 => false,
            SINGLE_REQ_MODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SINGLE_REQ_MODER {
        match value {
            false => SINGLE_REQ_MODER::_0,
            true => SINGLE_REQ_MODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SINGLE_REQ_MODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SINGLE_REQ_MODER::_1
    }
}
#[doc = "Possible values of the field `BYPASS_DMA_SYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_DMA_SYNCR {
    #[doc = "Don't Bypass External Synchronization. Use this setting if SINGLE_REQ_MODE=1."]
    _0,
    #[doc = "Bypass External Synchronization. This setting is mandatory if SINGLE_REQ_MODE=0."]
    _1,
}
impl BYPASS_DMA_SYNCR {
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
            BYPASS_DMA_SYNCR::_0 => false,
            BYPASS_DMA_SYNCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASS_DMA_SYNCR {
        match value {
            false => BYPASS_DMA_SYNCR::_0,
            true => BYPASS_DMA_SYNCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BYPASS_DMA_SYNCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BYPASS_DMA_SYNCR::_1
    }
}
#[doc = r" Value of the field"]
pub struct DMA_TRIGGERREDR {
    bits: bool,
}
impl DMA_TRIGGERREDR {
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
#[doc = "Possible values of the field `DMA_TIMED_OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_TIMED_OUTR {
    #[doc = "A DMA timeout has not occurred"]
    _0,
    #[doc = "A DMA timeout has occurred in Single Request Mode since the last time this bit was cleared"]
    _1,
}
impl DMA_TIMED_OUTR {
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
            DMA_TIMED_OUTR::_0 => false,
            DMA_TIMED_OUTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_TIMED_OUTR {
        match value {
            false => DMA_TIMED_OUTR::_0,
            true => DMA_TIMED_OUTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMA_TIMED_OUTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMA_TIMED_OUTR::_1
    }
}
#[doc = r" Value of the field"]
pub struct DMA_TIMEOUTR {
    bits: u8,
}
impl DMA_TIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `DMA_PAGE`"]
pub enum DMA_PAGEW {
    #[doc = "DMA Idle"]
    _0000,
    #[doc = "RX_DIG I and Q"]
    _0001,
    #[doc = "RX_DIG I Only"]
    _0010,
    #[doc = "RX_DIG Q Only"]
    _0011,
    #[doc = "RAW ADC I and Q"]
    _0100,
    #[doc = "RAW ADC I Only"]
    _0101,
    #[doc = "RAW ADC Q only"]
    _0110,
    #[doc = "DC Estimator I and Q"]
    _0111,
    #[doc = "DC Estimator I Only"]
    _1000,
    #[doc = "DC Estimator Q only"]
    _1001,
    #[doc = "RX_DIG Phase Output"]
    _1010,
    #[doc = "Demodulator Hard Decision"]
    _1011,
    #[doc = "Demodulator Soft Decision"]
    _1100,
    #[doc = "Demodulator Data Output"]
    _1101,
    #[doc = "Demodulator CFO Phase Output"]
    _1110,
}
impl DMA_PAGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMA_PAGEW::_0000 => 0,
            DMA_PAGEW::_0001 => 1,
            DMA_PAGEW::_0010 => 2,
            DMA_PAGEW::_0011 => 3,
            DMA_PAGEW::_0100 => 4,
            DMA_PAGEW::_0101 => 5,
            DMA_PAGEW::_0110 => 6,
            DMA_PAGEW::_0111 => 7,
            DMA_PAGEW::_1000 => 8,
            DMA_PAGEW::_1001 => 9,
            DMA_PAGEW::_1010 => 10,
            DMA_PAGEW::_1011 => 11,
            DMA_PAGEW::_1100 => 12,
            DMA_PAGEW::_1101 => 13,
            DMA_PAGEW::_1110 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_PAGEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_PAGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_PAGEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DMA Idle"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_0000)
    }
    #[doc = "RX_DIG I and Q"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_0001)
    }
    #[doc = "RX_DIG I Only"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_0010)
    }
    #[doc = "RX_DIG Q Only"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_0011)
    }
    #[doc = "RAW ADC I and Q"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_0100)
    }
    #[doc = "RAW ADC I Only"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_0101)
    }
    #[doc = "RAW ADC Q only"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_0110)
    }
    #[doc = "DC Estimator I and Q"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_0111)
    }
    #[doc = "DC Estimator I Only"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_1000)
    }
    #[doc = "DC Estimator Q only"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_1001)
    }
    #[doc = "RX_DIG Phase Output"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_1010)
    }
    #[doc = "Demodulator Hard Decision"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_1011)
    }
    #[doc = "Demodulator Soft Decision"]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_1100)
    }
    #[doc = "Demodulator Data Output"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_1101)
    }
    #[doc = "Demodulator CFO Phase Output"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DMA_PAGEW::_1110)
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
#[doc = "Values that can be written to the field `SINGLE_REQ_MODE`"]
pub enum SINGLE_REQ_MODEW {
    #[doc = "Disable Single Request Mode. The transceiver will assert ipd_req_radio_rx whenever it has a new sample ready for transfer."]
    _0,
    #[doc = "Enable Single Request Mode. A single initial request by the transceiver will transfer the entire DMA block of data"]
    _1,
}
impl SINGLE_REQ_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SINGLE_REQ_MODEW::_0 => false,
            SINGLE_REQ_MODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SINGLE_REQ_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SINGLE_REQ_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SINGLE_REQ_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Single Request Mode. The transceiver will assert ipd_req_radio_rx whenever it has a new sample ready for transfer."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SINGLE_REQ_MODEW::_0)
    }
    #[doc = "Enable Single Request Mode. A single initial request by the transceiver will transfer the entire DMA block of data"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SINGLE_REQ_MODEW::_1)
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
#[doc = "Values that can be written to the field `BYPASS_DMA_SYNC`"]
pub enum BYPASS_DMA_SYNCW {
    #[doc = "Don't Bypass External Synchronization. Use this setting if SINGLE_REQ_MODE=1."]
    _0,
    #[doc = "Bypass External Synchronization. This setting is mandatory if SINGLE_REQ_MODE=0."]
    _1,
}
impl BYPASS_DMA_SYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASS_DMA_SYNCW::_0 => false,
            BYPASS_DMA_SYNCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPASS_DMA_SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASS_DMA_SYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASS_DMA_SYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Don't Bypass External Synchronization. Use this setting if SINGLE_REQ_MODE=1."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BYPASS_DMA_SYNCW::_0)
    }
    #[doc = "Bypass External Synchronization. This setting is mandatory if SINGLE_REQ_MODE=0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BYPASS_DMA_SYNCW::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA_TIMED_OUT`"]
pub enum DMA_TIMED_OUTW {
    #[doc = "A DMA timeout has not occurred"]
    _0,
    #[doc = "A DMA timeout has occurred in Single Request Mode since the last time this bit was cleared"]
    _1,
}
impl DMA_TIMED_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_TIMED_OUTW::_0 => false,
            DMA_TIMED_OUTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_TIMED_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_TIMED_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_TIMED_OUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A DMA timeout has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMA_TIMED_OUTW::_0)
    }
    #[doc = "A DMA timeout has occurred in Single Request Mode since the last time this bit was cleared"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMA_TIMED_OUTW::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA_TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_TIMEOUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:3 - Transceiver DMA Page Selector"]
    #[inline]
    pub fn dma_page(&self) -> DMA_PAGER {
        DMA_PAGER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - DMA Single Request Mode"]
    #[inline]
    pub fn single_req_mode(&self) -> SINGLE_REQ_MODER {
        SINGLE_REQ_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Bypass External DMA Synchronization"]
    #[inline]
    pub fn bypass_dma_sync(&self) -> BYPASS_DMA_SYNCR {
        BYPASS_DMA_SYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - DMA TRIGGERRED"]
    #[inline]
    pub fn dma_triggerred(&self) -> DMA_TRIGGERREDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA_TRIGGERREDR { bits }
    }
    #[doc = "Bit 7 - DMA Transfer Timed Out"]
    #[inline]
    pub fn dma_timed_out(&self) -> DMA_TIMED_OUTR {
        DMA_TIMED_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - DMA Timeout"]
    #[inline]
    pub fn dma_timeout(&self) -> DMA_TIMEOUTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMA_TIMEOUTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 768 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Transceiver DMA Page Selector"]
    #[inline]
    pub fn dma_page(&mut self) -> _DMA_PAGEW {
        _DMA_PAGEW { w: self }
    }
    #[doc = "Bit 4 - DMA Single Request Mode"]
    #[inline]
    pub fn single_req_mode(&mut self) -> _SINGLE_REQ_MODEW {
        _SINGLE_REQ_MODEW { w: self }
    }
    #[doc = "Bit 5 - Bypass External DMA Synchronization"]
    #[inline]
    pub fn bypass_dma_sync(&mut self) -> _BYPASS_DMA_SYNCW {
        _BYPASS_DMA_SYNCW { w: self }
    }
    #[doc = "Bit 7 - DMA Transfer Timed Out"]
    #[inline]
    pub fn dma_timed_out(&mut self) -> _DMA_TIMED_OUTW {
        _DMA_TIMED_OUTW { w: self }
    }
    #[doc = "Bits 8:11 - DMA Timeout"]
    #[inline]
    pub fn dma_timeout(&mut self) -> _DMA_TIMEOUTW {
        _DMA_TIMEOUTW { w: self }
    }
}
