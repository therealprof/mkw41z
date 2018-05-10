#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PACKET_RAM_CTRL {
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
#[doc = "Possible values of the field `DBG_PAGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_PAGER {
    #[doc = "Packet RAM Debug Mode Idle"]
    _0000,
    #[doc = "RX_DIG I and Q"]
    _0001,
    #[doc = "RAW ADC I and Q"]
    _0100,
    #[doc = "DC Estimator I and Q"]
    _0111,
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
impl DBG_PAGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DBG_PAGER::_0000 => 0,
            DBG_PAGER::_0001 => 1,
            DBG_PAGER::_0100 => 4,
            DBG_PAGER::_0111 => 7,
            DBG_PAGER::_1010 => 10,
            DBG_PAGER::_1011 => 11,
            DBG_PAGER::_1100 => 12,
            DBG_PAGER::_1101 => 13,
            DBG_PAGER::_1110 => 14,
            DBG_PAGER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DBG_PAGER {
        match value {
            0 => DBG_PAGER::_0000,
            1 => DBG_PAGER::_0001,
            4 => DBG_PAGER::_0100,
            7 => DBG_PAGER::_0111,
            10 => DBG_PAGER::_1010,
            11 => DBG_PAGER::_1011,
            12 => DBG_PAGER::_1100,
            13 => DBG_PAGER::_1101,
            14 => DBG_PAGER::_1110,
            i => DBG_PAGER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == DBG_PAGER::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == DBG_PAGER::_0001
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == DBG_PAGER::_0100
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == DBG_PAGER::_0111
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == DBG_PAGER::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == DBG_PAGER::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == DBG_PAGER::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == DBG_PAGER::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == DBG_PAGER::_1110
    }
}
#[doc = "Possible values of the field `PB_PROTECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PB_PROTECTR {
    #[doc = "Incoming received packets overwrite Packet Buffer RX contents (default)"]
    _0,
    #[doc = "Incoming received packets are blocked from overwriting Packet Buffer RX contents"]
    _1,
}
impl PB_PROTECTR {
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
            PB_PROTECTR::_0 => false,
            PB_PROTECTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PB_PROTECTR {
        match value {
            false => PB_PROTECTR::_0,
            true => PB_PROTECTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PB_PROTECTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PB_PROTECTR::_1
    }
}
#[doc = "Possible values of the field `XCVR_RAM_ALLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XCVR_RAM_ALLOWR {
    #[doc = "Protocol Engines, and associated IPS busses, have exclusive access to Packet RAM (mission mode)"]
    _0,
    #[doc = "Transceiver-space access to Packet RAM, including Packet RAM debug mode, are allowed"]
    _1,
}
impl XCVR_RAM_ALLOWR {
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
            XCVR_RAM_ALLOWR::_0 => false,
            XCVR_RAM_ALLOWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XCVR_RAM_ALLOWR {
        match value {
            false => XCVR_RAM_ALLOWR::_0,
            true => XCVR_RAM_ALLOWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == XCVR_RAM_ALLOWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == XCVR_RAM_ALLOWR::_1
    }
}
#[doc = "Possible values of the field `ALL_PROTOCOLS_ALLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALL_PROTOCOLS_ALLOWR {
    #[doc = "IPS bus access to Packet RAM is restricted to the protocol engine currently selected by XCVR_CTRL[PROTOCOL]."]
    _0,
    #[doc = "All IPS bus access to Packet RAM permitted, regardless of XCVR_CTRL[PROTOCOL] setting"]
    _1,
}
impl ALL_PROTOCOLS_ALLOWR {
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
            ALL_PROTOCOLS_ALLOWR::_0 => false,
            ALL_PROTOCOLS_ALLOWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALL_PROTOCOLS_ALLOWR {
        match value {
            false => ALL_PROTOCOLS_ALLOWR::_0,
            true => ALL_PROTOCOLS_ALLOWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ALL_PROTOCOLS_ALLOWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ALL_PROTOCOLS_ALLOWR::_1
    }
}
#[doc = r" Value of the field"]
pub struct DBG_TRIGGERREDR {
    bits: bool,
}
impl DBG_TRIGGERREDR {
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
#[doc = "Possible values of the field `DBG_RAM_FULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_RAM_FULLR {
    #[doc = "Neither Packet RAM0 nor RAM1 is full"]
    _00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DBG_RAM_FULLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DBG_RAM_FULLR::_00 => 0,
            DBG_RAM_FULLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DBG_RAM_FULLR {
        match value {
            0 => DBG_RAM_FULLR::_00,
            i => DBG_RAM_FULLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DBG_RAM_FULLR::_00
    }
}
#[doc = "Possible values of the field `RAM0_CLK_ON_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM0_CLK_ON_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RAM0_CLK_ON_OVRD to override the RAM0 Clock Gate Enable."]
    _1,
}
impl RAM0_CLK_ON_OVRD_ENR {
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
            RAM0_CLK_ON_OVRD_ENR::_0 => false,
            RAM0_CLK_ON_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM0_CLK_ON_OVRD_ENR {
        match value {
            false => RAM0_CLK_ON_OVRD_ENR::_0,
            true => RAM0_CLK_ON_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RAM0_CLK_ON_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RAM0_CLK_ON_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RAM0_CLK_ON_OVRDR {
    bits: bool,
}
impl RAM0_CLK_ON_OVRDR {
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
#[doc = "Possible values of the field `RAM1_CLK_ON_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM1_CLK_ON_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RAM1_CLK_ON_OVRD to override the RAM1 Clock Gate Enable."]
    _1,
}
impl RAM1_CLK_ON_OVRD_ENR {
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
            RAM1_CLK_ON_OVRD_ENR::_0 => false,
            RAM1_CLK_ON_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM1_CLK_ON_OVRD_ENR {
        match value {
            false => RAM1_CLK_ON_OVRD_ENR::_0,
            true => RAM1_CLK_ON_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RAM1_CLK_ON_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RAM1_CLK_ON_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RAM1_CLK_ON_OVRDR {
    bits: bool,
}
impl RAM1_CLK_ON_OVRDR {
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
#[doc = "Possible values of the field `RAM0_CE_ON_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM0_CE_ON_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RAM0_CE_ON_OVRD to override the RAM0 CE."]
    _1,
}
impl RAM0_CE_ON_OVRD_ENR {
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
            RAM0_CE_ON_OVRD_ENR::_0 => false,
            RAM0_CE_ON_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM0_CE_ON_OVRD_ENR {
        match value {
            false => RAM0_CE_ON_OVRD_ENR::_0,
            true => RAM0_CE_ON_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RAM0_CE_ON_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RAM0_CE_ON_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RAM0_CE_ON_OVRDR {
    bits: bool,
}
impl RAM0_CE_ON_OVRDR {
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
#[doc = "Possible values of the field `RAM1_CE_ON_OVRD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM1_CE_ON_OVRD_ENR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RAM1_CE_ON_OVRD to override the RAM1 CE."]
    _1,
}
impl RAM1_CE_ON_OVRD_ENR {
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
            RAM1_CE_ON_OVRD_ENR::_0 => false,
            RAM1_CE_ON_OVRD_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM1_CE_ON_OVRD_ENR {
        match value {
            false => RAM1_CE_ON_OVRD_ENR::_0,
            true => RAM1_CE_ON_OVRD_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RAM1_CE_ON_OVRD_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RAM1_CE_ON_OVRD_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RAM1_CE_ON_OVRDR {
    bits: bool,
}
impl RAM1_CE_ON_OVRDR {
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
#[doc = "Values that can be written to the field `DBG_PAGE`"]
pub enum DBG_PAGEW {
    #[doc = "Packet RAM Debug Mode Idle"]
    _0000,
    #[doc = "RX_DIG I and Q"]
    _0001,
    #[doc = "RAW ADC I and Q"]
    _0100,
    #[doc = "DC Estimator I and Q"]
    _0111,
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
impl DBG_PAGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DBG_PAGEW::_0000 => 0,
            DBG_PAGEW::_0001 => 1,
            DBG_PAGEW::_0100 => 4,
            DBG_PAGEW::_0111 => 7,
            DBG_PAGEW::_1010 => 10,
            DBG_PAGEW::_1011 => 11,
            DBG_PAGEW::_1100 => 12,
            DBG_PAGEW::_1101 => 13,
            DBG_PAGEW::_1110 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBG_PAGEW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_PAGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBG_PAGEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Packet RAM Debug Mode Idle"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DBG_PAGEW::_0000)
    }
    #[doc = "RX_DIG I and Q"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DBG_PAGEW::_0001)
    }
    #[doc = "RAW ADC I and Q"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DBG_PAGEW::_0100)
    }
    #[doc = "DC Estimator I and Q"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DBG_PAGEW::_0111)
    }
    #[doc = "RX_DIG Phase Output"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(DBG_PAGEW::_1010)
    }
    #[doc = "Demodulator Hard Decision"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(DBG_PAGEW::_1011)
    }
    #[doc = "Demodulator Soft Decision"]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(DBG_PAGEW::_1100)
    }
    #[doc = "Demodulator Data Output"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(DBG_PAGEW::_1101)
    }
    #[doc = "Demodulator CFO Phase Output"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DBG_PAGEW::_1110)
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
#[doc = "Values that can be written to the field `PB_PROTECT`"]
pub enum PB_PROTECTW {
    #[doc = "Incoming received packets overwrite Packet Buffer RX contents (default)"]
    _0,
    #[doc = "Incoming received packets are blocked from overwriting Packet Buffer RX contents"]
    _1,
}
impl PB_PROTECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PB_PROTECTW::_0 => false,
            PB_PROTECTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PB_PROTECTW<'a> {
    w: &'a mut W,
}
impl<'a> _PB_PROTECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PB_PROTECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Incoming received packets overwrite Packet Buffer RX contents (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PB_PROTECTW::_0)
    }
    #[doc = "Incoming received packets are blocked from overwriting Packet Buffer RX contents"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PB_PROTECTW::_1)
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
#[doc = "Values that can be written to the field `XCVR_RAM_ALLOW`"]
pub enum XCVR_RAM_ALLOWW {
    #[doc = "Protocol Engines, and associated IPS busses, have exclusive access to Packet RAM (mission mode)"]
    _0,
    #[doc = "Transceiver-space access to Packet RAM, including Packet RAM debug mode, are allowed"]
    _1,
}
impl XCVR_RAM_ALLOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XCVR_RAM_ALLOWW::_0 => false,
            XCVR_RAM_ALLOWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XCVR_RAM_ALLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _XCVR_RAM_ALLOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XCVR_RAM_ALLOWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protocol Engines, and associated IPS busses, have exclusive access to Packet RAM (mission mode)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(XCVR_RAM_ALLOWW::_0)
    }
    #[doc = "Transceiver-space access to Packet RAM, including Packet RAM debug mode, are allowed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(XCVR_RAM_ALLOWW::_1)
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
#[doc = "Values that can be written to the field `ALL_PROTOCOLS_ALLOW`"]
pub enum ALL_PROTOCOLS_ALLOWW {
    #[doc = "IPS bus access to Packet RAM is restricted to the protocol engine currently selected by XCVR_CTRL[PROTOCOL]."]
    _0,
    #[doc = "All IPS bus access to Packet RAM permitted, regardless of XCVR_CTRL[PROTOCOL] setting"]
    _1,
}
impl ALL_PROTOCOLS_ALLOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALL_PROTOCOLS_ALLOWW::_0 => false,
            ALL_PROTOCOLS_ALLOWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALL_PROTOCOLS_ALLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _ALL_PROTOCOLS_ALLOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALL_PROTOCOLS_ALLOWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IPS bus access to Packet RAM is restricted to the protocol engine currently selected by XCVR_CTRL[PROTOCOL]."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALL_PROTOCOLS_ALLOWW::_0)
    }
    #[doc = "All IPS bus access to Packet RAM permitted, regardless of XCVR_CTRL[PROTOCOL] setting"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALL_PROTOCOLS_ALLOWW::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAM0_CLK_ON_OVRD_EN`"]
pub enum RAM0_CLK_ON_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RAM0_CLK_ON_OVRD to override the RAM0 Clock Gate Enable."]
    _1,
}
impl RAM0_CLK_ON_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM0_CLK_ON_OVRD_ENW::_0 => false,
            RAM0_CLK_ON_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM0_CLK_ON_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM0_CLK_ON_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM0_CLK_ON_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RAM0_CLK_ON_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RAM0_CLK_ON_OVRD to override the RAM0 Clock Gate Enable."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RAM0_CLK_ON_OVRD_ENW::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RAM0_CLK_ON_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM0_CLK_ON_OVRDW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAM1_CLK_ON_OVRD_EN`"]
pub enum RAM1_CLK_ON_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RAM1_CLK_ON_OVRD to override the RAM1 Clock Gate Enable."]
    _1,
}
impl RAM1_CLK_ON_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM1_CLK_ON_OVRD_ENW::_0 => false,
            RAM1_CLK_ON_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM1_CLK_ON_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM1_CLK_ON_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM1_CLK_ON_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RAM1_CLK_ON_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RAM1_CLK_ON_OVRD to override the RAM1 Clock Gate Enable."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RAM1_CLK_ON_OVRD_ENW::_1)
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
#[doc = r" Proxy"]
pub struct _RAM1_CLK_ON_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM1_CLK_ON_OVRDW<'a> {
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
#[doc = "Values that can be written to the field `RAM0_CE_ON_OVRD_EN`"]
pub enum RAM0_CE_ON_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RAM0_CE_ON_OVRD to override the RAM0 CE."]
    _1,
}
impl RAM0_CE_ON_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM0_CE_ON_OVRD_ENW::_0 => false,
            RAM0_CE_ON_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM0_CE_ON_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM0_CE_ON_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM0_CE_ON_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RAM0_CE_ON_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RAM0_CE_ON_OVRD to override the RAM0 CE."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RAM0_CE_ON_OVRD_ENW::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RAM0_CE_ON_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM0_CE_ON_OVRDW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAM1_CE_ON_OVRD_EN`"]
pub enum RAM1_CE_ON_OVRD_ENW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the state of RAM1_CE_ON_OVRD to override the RAM1 CE."]
    _1,
}
impl RAM1_CE_ON_OVRD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM1_CE_ON_OVRD_ENW::_0 => false,
            RAM1_CE_ON_OVRD_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM1_CE_ON_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM1_CE_ON_OVRD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM1_CE_ON_OVRD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RAM1_CE_ON_OVRD_ENW::_0)
    }
    #[doc = "Use the state of RAM1_CE_ON_OVRD to override the RAM1 CE."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RAM1_CE_ON_OVRD_ENW::_1)
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
#[doc = r" Proxy"]
pub struct _RAM1_CE_ON_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM1_CE_ON_OVRDW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Packet RAM Debug Page Selector"]
    #[inline]
    pub fn dbg_page(&self) -> DBG_PAGER {
        DBG_PAGER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Packet Buffer Protect"]
    #[inline]
    pub fn pb_protect(&self) -> PB_PROTECTR {
        PB_PROTECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Allow Packet RAM Transceiver Access"]
    #[inline]
    pub fn xcvr_ram_allow(&self) -> XCVR_RAM_ALLOWR {
        XCVR_RAM_ALLOWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Allow IPS bus access to Packet RAM for any protocol at any time."]
    #[inline]
    pub fn all_protocols_allow(&self) -> ALL_PROTOCOLS_ALLOWR {
        ALL_PROTOCOLS_ALLOWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - DBG_TRIGGERRED"]
    #[inline]
    pub fn dbg_triggerred(&self) -> DBG_TRIGGERREDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_TRIGGERREDR { bits }
    }
    #[doc = "Bits 8:9 - DBG_RAM_FULL[1:0]"]
    #[inline]
    pub fn dbg_ram_full(&self) -> DBG_RAM_FULLR {
        DBG_RAM_FULLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Override control for RAM0 Clock Gate Enable"]
    #[inline]
    pub fn ram0_clk_on_ovrd_en(&self) -> RAM0_CLK_ON_OVRD_ENR {
        RAM0_CLK_ON_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Override value for RAM0 Clock Gate Enable"]
    #[inline]
    pub fn ram0_clk_on_ovrd(&self) -> RAM0_CLK_ON_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RAM0_CLK_ON_OVRDR { bits }
    }
    #[doc = "Bit 12 - Override control for RAM1 Clock Gate Enable"]
    #[inline]
    pub fn ram1_clk_on_ovrd_en(&self) -> RAM1_CLK_ON_OVRD_ENR {
        RAM1_CLK_ON_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Override value for RAM1 Clock Gate Enable"]
    #[inline]
    pub fn ram1_clk_on_ovrd(&self) -> RAM1_CLK_ON_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RAM1_CLK_ON_OVRDR { bits }
    }
    #[doc = "Bit 14 - Override control for RAM0 CE (Chip Enable)"]
    #[inline]
    pub fn ram0_ce_on_ovrd_en(&self) -> RAM0_CE_ON_OVRD_ENR {
        RAM0_CE_ON_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Override value for RAM0 CE (Chip Enable)"]
    #[inline]
    pub fn ram0_ce_on_ovrd(&self) -> RAM0_CE_ON_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RAM0_CE_ON_OVRDR { bits }
    }
    #[doc = "Bit 16 - Override control for RAM1 CE (Chip Enable)"]
    #[inline]
    pub fn ram1_ce_on_ovrd_en(&self) -> RAM1_CE_ON_OVRD_ENR {
        RAM1_CE_ON_OVRD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Override value for RAM1 CE (Chip Enable)"]
    #[inline]
    pub fn ram1_ce_on_ovrd(&self) -> RAM1_CE_ON_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RAM1_CE_ON_OVRDR { bits }
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
    #[doc = "Bits 0:3 - Packet RAM Debug Page Selector"]
    #[inline]
    pub fn dbg_page(&mut self) -> _DBG_PAGEW {
        _DBG_PAGEW { w: self }
    }
    #[doc = "Bit 4 - Packet Buffer Protect"]
    #[inline]
    pub fn pb_protect(&mut self) -> _PB_PROTECTW {
        _PB_PROTECTW { w: self }
    }
    #[doc = "Bit 5 - Allow Packet RAM Transceiver Access"]
    #[inline]
    pub fn xcvr_ram_allow(&mut self) -> _XCVR_RAM_ALLOWW {
        _XCVR_RAM_ALLOWW { w: self }
    }
    #[doc = "Bit 6 - Allow IPS bus access to Packet RAM for any protocol at any time."]
    #[inline]
    pub fn all_protocols_allow(&mut self) -> _ALL_PROTOCOLS_ALLOWW {
        _ALL_PROTOCOLS_ALLOWW { w: self }
    }
    #[doc = "Bit 10 - Override control for RAM0 Clock Gate Enable"]
    #[inline]
    pub fn ram0_clk_on_ovrd_en(&mut self) -> _RAM0_CLK_ON_OVRD_ENW {
        _RAM0_CLK_ON_OVRD_ENW { w: self }
    }
    #[doc = "Bit 11 - Override value for RAM0 Clock Gate Enable"]
    #[inline]
    pub fn ram0_clk_on_ovrd(&mut self) -> _RAM0_CLK_ON_OVRDW {
        _RAM0_CLK_ON_OVRDW { w: self }
    }
    #[doc = "Bit 12 - Override control for RAM1 Clock Gate Enable"]
    #[inline]
    pub fn ram1_clk_on_ovrd_en(&mut self) -> _RAM1_CLK_ON_OVRD_ENW {
        _RAM1_CLK_ON_OVRD_ENW { w: self }
    }
    #[doc = "Bit 13 - Override value for RAM1 Clock Gate Enable"]
    #[inline]
    pub fn ram1_clk_on_ovrd(&mut self) -> _RAM1_CLK_ON_OVRDW {
        _RAM1_CLK_ON_OVRDW { w: self }
    }
    #[doc = "Bit 14 - Override control for RAM0 CE (Chip Enable)"]
    #[inline]
    pub fn ram0_ce_on_ovrd_en(&mut self) -> _RAM0_CE_ON_OVRD_ENW {
        _RAM0_CE_ON_OVRD_ENW { w: self }
    }
    #[doc = "Bit 15 - Override value for RAM0 CE (Chip Enable)"]
    #[inline]
    pub fn ram0_ce_on_ovrd(&mut self) -> _RAM0_CE_ON_OVRDW {
        _RAM0_CE_ON_OVRDW { w: self }
    }
    #[doc = "Bit 16 - Override control for RAM1 CE (Chip Enable)"]
    #[inline]
    pub fn ram1_ce_on_ovrd_en(&mut self) -> _RAM1_CE_ON_OVRD_ENW {
        _RAM1_CE_ON_OVRD_ENW { w: self }
    }
    #[doc = "Bit 17 - Override value for RAM1 CE (Chip Enable)"]
    #[inline]
    pub fn ram1_ce_on_ovrd(&mut self) -> _RAM1_CE_ON_OVRDW {
        _RAM1_CE_ON_OVRDW { w: self }
    }
}
