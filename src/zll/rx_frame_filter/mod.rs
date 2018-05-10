#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_FRAME_FILTER {
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
#[doc = "Possible values of the field `BEACON_FT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEACON_FTR {
    #[doc = "reject all Beacon frames"]
    _0,
    #[doc = "Beacon frame type enabled."]
    _1,
}
impl BEACON_FTR {
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
            BEACON_FTR::_0 => false,
            BEACON_FTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEACON_FTR {
        match value {
            false => BEACON_FTR::_0,
            true => BEACON_FTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BEACON_FTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BEACON_FTR::_1
    }
}
#[doc = "Possible values of the field `DATA_FT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_FTR {
    #[doc = "reject all Beacon frames"]
    _0,
    #[doc = "Data frame type enabled."]
    _1,
}
impl DATA_FTR {
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
            DATA_FTR::_0 => false,
            DATA_FTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATA_FTR {
        match value {
            false => DATA_FTR::_0,
            true => DATA_FTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DATA_FTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DATA_FTR::_1
    }
}
#[doc = "Possible values of the field `ACK_FT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACK_FTR {
    #[doc = "reject all Acknowledge frames"]
    _0,
    #[doc = "Acknowledge frame type enabled."]
    _1,
}
impl ACK_FTR {
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
            ACK_FTR::_0 => false,
            ACK_FTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACK_FTR {
        match value {
            false => ACK_FTR::_0,
            true => ACK_FTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ACK_FTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ACK_FTR::_1
    }
}
#[doc = "Possible values of the field `CMD_FT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_FTR {
    #[doc = "reject all MAC Command frames"]
    _0,
    #[doc = "MAC Command frame type enabled."]
    _1,
}
impl CMD_FTR {
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
            CMD_FTR::_0 => false,
            CMD_FTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_FTR {
        match value {
            false => CMD_FTR::_0,
            true => CMD_FTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CMD_FTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CMD_FTR::_1
    }
}
#[doc = "Possible values of the field `LLDN_FT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLDN_FTR {
    #[doc = "reject all LLDN frames"]
    _0,
    #[doc = "LLDN frame type enabled (Frame Type 4)."]
    _1,
}
impl LLDN_FTR {
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
            LLDN_FTR::_0 => false,
            LLDN_FTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LLDN_FTR {
        match value {
            false => LLDN_FTR::_0,
            true => LLDN_FTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LLDN_FTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LLDN_FTR::_1
    }
}
#[doc = "Possible values of the field `MULTIPURPOSE_FT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTIPURPOSE_FTR {
    #[doc = "reject all Multipurpose frames"]
    _0,
    #[doc = "Multipurpose frame type enabled (Frame Type 5)."]
    _1,
}
impl MULTIPURPOSE_FTR {
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
            MULTIPURPOSE_FTR::_0 => false,
            MULTIPURPOSE_FTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MULTIPURPOSE_FTR {
        match value {
            false => MULTIPURPOSE_FTR::_0,
            true => MULTIPURPOSE_FTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MULTIPURPOSE_FTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MULTIPURPOSE_FTR::_1
    }
}
#[doc = "Possible values of the field `NS_FT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NS_FTR {
    #[doc = "reject all \"Not Specified\" frames"]
    _0,
    #[doc = "Not-specified (reserved) frame type enabled. Applies to Frame Type 6. No packet filtering is performed, except for frame length checking (FrameLength>=5 and FrameLength<=127). No AUTOACK is transmitted for this Frame Type"]
    _1,
}
impl NS_FTR {
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
            NS_FTR::_0 => false,
            NS_FTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NS_FTR {
        match value {
            false => NS_FTR::_0,
            true => NS_FTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NS_FTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NS_FTR::_1
    }
}
#[doc = "Possible values of the field `EXTENDED_FT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTENDED_FTR {
    #[doc = "reject all Extended frames"]
    _0,
    #[doc = "Extended frame type enabled (Frame Type 7)."]
    _1,
}
impl EXTENDED_FTR {
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
            EXTENDED_FTR::_0 => false,
            EXTENDED_FTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTENDED_FTR {
        match value {
            false => EXTENDED_FTR::_0,
            true => EXTENDED_FTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EXTENDED_FTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EXTENDED_FTR::_1
    }
}
#[doc = r" Value of the field"]
pub struct FRM_VER_FILTERR {
    bits: u8,
}
impl FRM_VER_FILTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ACTIVE_PROMISCUOUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVE_PROMISCUOUSR {
    #[doc = "normal operation"]
    _0,
    #[doc = "Provide Data Indication on all received packets under the same rules which apply in PROMISCUOUS mode, however acknowledge those packets under rules which apply in non-PROMISCUOUS mode"]
    _1,
}
impl ACTIVE_PROMISCUOUSR {
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
            ACTIVE_PROMISCUOUSR::_0 => false,
            ACTIVE_PROMISCUOUSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACTIVE_PROMISCUOUSR {
        match value {
            false => ACTIVE_PROMISCUOUSR::_0,
            true => ACTIVE_PROMISCUOUSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ACTIVE_PROMISCUOUSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ACTIVE_PROMISCUOUSR::_1
    }
}
#[doc = "Possible values of the field `EXTENDED_FCS_CHK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTENDED_FCS_CHKR {
    #[doc = "Packet Processor will not check FCS for Frame Type EXTENDED (default)"]
    _0,
    #[doc = "Packet Processor will check FCS at end-of-packet based on packet length derived from PHR, for Frame Type EXTENDED"]
    _1,
}
impl EXTENDED_FCS_CHKR {
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
            EXTENDED_FCS_CHKR::_0 => false,
            EXTENDED_FCS_CHKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTENDED_FCS_CHKR {
        match value {
            false => EXTENDED_FCS_CHKR::_0,
            true => EXTENDED_FCS_CHKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EXTENDED_FCS_CHKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EXTENDED_FCS_CHKR::_1
    }
}
#[doc = "Possible values of the field `FV2_BEACON_RECD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FV2_BEACON_RECDR {
    #[doc = "The last packet received was not Frame Type Beacon with Frame Version 2"]
    _0,
    #[doc = "The last packet received was Frame Type Beacon with Frame Version 2, and FRM_VER_FILTER[2]=1 to allow such packets"]
    _1,
}
impl FV2_BEACON_RECDR {
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
            FV2_BEACON_RECDR::_0 => false,
            FV2_BEACON_RECDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FV2_BEACON_RECDR {
        match value {
            false => FV2_BEACON_RECDR::_0,
            true => FV2_BEACON_RECDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FV2_BEACON_RECDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FV2_BEACON_RECDR::_1
    }
}
#[doc = "Possible values of the field `FV2_DATA_RECD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FV2_DATA_RECDR {
    #[doc = "The last packet received was not Frame Type Data with Frame Version 2"]
    _0,
    #[doc = "The last packet received was Frame Type Data with Frame Version 2, and FRM_VER_FILTER[2]=1 to allow such packets"]
    _1,
}
impl FV2_DATA_RECDR {
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
            FV2_DATA_RECDR::_0 => false,
            FV2_DATA_RECDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FV2_DATA_RECDR {
        match value {
            false => FV2_DATA_RECDR::_0,
            true => FV2_DATA_RECDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FV2_DATA_RECDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FV2_DATA_RECDR::_1
    }
}
#[doc = "Possible values of the field `FV2_ACK_RECD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FV2_ACK_RECDR {
    #[doc = "The last packet received was not Frame Type Ack with Frame Version 2"]
    _0,
    #[doc = "The last packet received was Frame Type Ack with Frame Version 2, and FRM_VER_FILTER[2]=1 to allow such packets"]
    _1,
}
impl FV2_ACK_RECDR {
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
            FV2_ACK_RECDR::_0 => false,
            FV2_ACK_RECDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FV2_ACK_RECDR {
        match value {
            false => FV2_ACK_RECDR::_0,
            true => FV2_ACK_RECDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FV2_ACK_RECDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FV2_ACK_RECDR::_1
    }
}
#[doc = "Possible values of the field `FV2_CMD_RECD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FV2_CMD_RECDR {
    #[doc = "The last packet received was not Frame Type MAC Command with Frame Version 2"]
    _0,
    #[doc = "The last packet received was Frame Type MAC Command with Frame Version 2, and FRM_VER_FILTER[2]=1 to allow such packets"]
    _1,
}
impl FV2_CMD_RECDR {
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
            FV2_CMD_RECDR::_0 => false,
            FV2_CMD_RECDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FV2_CMD_RECDR {
        match value {
            false => FV2_CMD_RECDR::_0,
            true => FV2_CMD_RECDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FV2_CMD_RECDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FV2_CMD_RECDR::_1
    }
}
#[doc = "Possible values of the field `LLDN_RECD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLDN_RECDR {
    #[doc = "The last packet received was not Frame Type LLDN"]
    _0,
    #[doc = "The last packet received was Frame Type LLDN, and LLDN_FT=1 to allow such packets."]
    _1,
}
impl LLDN_RECDR {
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
            LLDN_RECDR::_0 => false,
            LLDN_RECDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LLDN_RECDR {
        match value {
            false => LLDN_RECDR::_0,
            true => LLDN_RECDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LLDN_RECDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LLDN_RECDR::_1
    }
}
#[doc = "Possible values of the field `MULTIPURPOSE_RECD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTIPURPOSE_RECDR {
    #[doc = "last packet received was not Frame Type MULTIPURPOSE"]
    _0,
    #[doc = "The last packet received was Frame Type MULTIPURPOSE, and MULTIPURPOSE_FT=1 to allow such packets."]
    _1,
}
impl MULTIPURPOSE_RECDR {
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
            MULTIPURPOSE_RECDR::_0 => false,
            MULTIPURPOSE_RECDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MULTIPURPOSE_RECDR {
        match value {
            false => MULTIPURPOSE_RECDR::_0,
            true => MULTIPURPOSE_RECDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MULTIPURPOSE_RECDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MULTIPURPOSE_RECDR::_1
    }
}
#[doc = "Possible values of the field `EXTENDED_RECD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTENDED_RECDR {
    #[doc = "The last packet received was not Frame Type EXTENDED"]
    _0,
    #[doc = "The last packet received was Frame Type EXTENDED, and EXTENDED_FT=1 to allow such packets."]
    _1,
}
impl EXTENDED_RECDR {
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
            EXTENDED_RECDR::_0 => false,
            EXTENDED_RECDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTENDED_RECDR {
        match value {
            false => EXTENDED_RECDR::_0,
            true => EXTENDED_RECDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EXTENDED_RECDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EXTENDED_RECDR::_1
    }
}
#[doc = "Values that can be written to the field `BEACON_FT`"]
pub enum BEACON_FTW {
    #[doc = "reject all Beacon frames"]
    _0,
    #[doc = "Beacon frame type enabled."]
    _1,
}
impl BEACON_FTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BEACON_FTW::_0 => false,
            BEACON_FTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BEACON_FTW<'a> {
    w: &'a mut W,
}
impl<'a> _BEACON_FTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BEACON_FTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "reject all Beacon frames"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BEACON_FTW::_0)
    }
    #[doc = "Beacon frame type enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BEACON_FTW::_1)
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
#[doc = "Values that can be written to the field `DATA_FT`"]
pub enum DATA_FTW {
    #[doc = "reject all Beacon frames"]
    _0,
    #[doc = "Data frame type enabled."]
    _1,
}
impl DATA_FTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATA_FTW::_0 => false,
            DATA_FTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_FTW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_FTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_FTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "reject all Beacon frames"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA_FTW::_0)
    }
    #[doc = "Data frame type enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA_FTW::_1)
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
#[doc = "Values that can be written to the field `ACK_FT`"]
pub enum ACK_FTW {
    #[doc = "reject all Acknowledge frames"]
    _0,
    #[doc = "Acknowledge frame type enabled."]
    _1,
}
impl ACK_FTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACK_FTW::_0 => false,
            ACK_FTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACK_FTW<'a> {
    w: &'a mut W,
}
impl<'a> _ACK_FTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACK_FTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "reject all Acknowledge frames"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACK_FTW::_0)
    }
    #[doc = "Acknowledge frame type enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACK_FTW::_1)
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
#[doc = "Values that can be written to the field `CMD_FT`"]
pub enum CMD_FTW {
    #[doc = "reject all MAC Command frames"]
    _0,
    #[doc = "MAC Command frame type enabled."]
    _1,
}
impl CMD_FTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_FTW::_0 => false,
            CMD_FTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_FTW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_FTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_FTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "reject all MAC Command frames"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMD_FTW::_0)
    }
    #[doc = "MAC Command frame type enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMD_FTW::_1)
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
#[doc = "Values that can be written to the field `LLDN_FT`"]
pub enum LLDN_FTW {
    #[doc = "reject all LLDN frames"]
    _0,
    #[doc = "LLDN frame type enabled (Frame Type 4)."]
    _1,
}
impl LLDN_FTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LLDN_FTW::_0 => false,
            LLDN_FTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LLDN_FTW<'a> {
    w: &'a mut W,
}
impl<'a> _LLDN_FTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LLDN_FTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "reject all LLDN frames"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LLDN_FTW::_0)
    }
    #[doc = "LLDN frame type enabled (Frame Type 4)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LLDN_FTW::_1)
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
#[doc = "Values that can be written to the field `MULTIPURPOSE_FT`"]
pub enum MULTIPURPOSE_FTW {
    #[doc = "reject all Multipurpose frames"]
    _0,
    #[doc = "Multipurpose frame type enabled (Frame Type 5)."]
    _1,
}
impl MULTIPURPOSE_FTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MULTIPURPOSE_FTW::_0 => false,
            MULTIPURPOSE_FTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MULTIPURPOSE_FTW<'a> {
    w: &'a mut W,
}
impl<'a> _MULTIPURPOSE_FTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MULTIPURPOSE_FTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "reject all Multipurpose frames"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MULTIPURPOSE_FTW::_0)
    }
    #[doc = "Multipurpose frame type enabled (Frame Type 5)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MULTIPURPOSE_FTW::_1)
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
#[doc = "Values that can be written to the field `NS_FT`"]
pub enum NS_FTW {
    #[doc = "reject all \"Not Specified\" frames"]
    _0,
    #[doc = "Not-specified (reserved) frame type enabled. Applies to Frame Type 6. No packet filtering is performed, except for frame length checking (FrameLength>=5 and FrameLength<=127). No AUTOACK is transmitted for this Frame Type"]
    _1,
}
impl NS_FTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NS_FTW::_0 => false,
            NS_FTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NS_FTW<'a> {
    w: &'a mut W,
}
impl<'a> _NS_FTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NS_FTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "reject all \"Not Specified\" frames"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NS_FTW::_0)
    }
    #[doc = "Not-specified (reserved) frame type enabled. Applies to Frame Type 6. No packet filtering is performed, except for frame length checking (FrameLength>=5 and FrameLength<=127). No AUTOACK is transmitted for this Frame Type"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NS_FTW::_1)
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
#[doc = "Values that can be written to the field `EXTENDED_FT`"]
pub enum EXTENDED_FTW {
    #[doc = "reject all Extended frames"]
    _0,
    #[doc = "Extended frame type enabled (Frame Type 7)."]
    _1,
}
impl EXTENDED_FTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTENDED_FTW::_0 => false,
            EXTENDED_FTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTENDED_FTW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTENDED_FTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTENDED_FTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "reject all Extended frames"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTENDED_FTW::_0)
    }
    #[doc = "Extended frame type enabled (Frame Type 7)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTENDED_FTW::_1)
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
pub struct _FRM_VER_FILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _FRM_VER_FILTERW<'a> {
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
#[doc = "Values that can be written to the field `ACTIVE_PROMISCUOUS`"]
pub enum ACTIVE_PROMISCUOUSW {
    #[doc = "normal operation"]
    _0,
    #[doc = "Provide Data Indication on all received packets under the same rules which apply in PROMISCUOUS mode, however acknowledge those packets under rules which apply in non-PROMISCUOUS mode"]
    _1,
}
impl ACTIVE_PROMISCUOUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACTIVE_PROMISCUOUSW::_0 => false,
            ACTIVE_PROMISCUOUSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE_PROMISCUOUSW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE_PROMISCUOUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACTIVE_PROMISCUOUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "normal operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACTIVE_PROMISCUOUSW::_0)
    }
    #[doc = "Provide Data Indication on all received packets under the same rules which apply in PROMISCUOUS mode, however acknowledge those packets under rules which apply in non-PROMISCUOUS mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACTIVE_PROMISCUOUSW::_1)
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
#[doc = "Values that can be written to the field `EXTENDED_FCS_CHK`"]
pub enum EXTENDED_FCS_CHKW {
    #[doc = "Packet Processor will not check FCS for Frame Type EXTENDED (default)"]
    _0,
    #[doc = "Packet Processor will check FCS at end-of-packet based on packet length derived from PHR, for Frame Type EXTENDED"]
    _1,
}
impl EXTENDED_FCS_CHKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTENDED_FCS_CHKW::_0 => false,
            EXTENDED_FCS_CHKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTENDED_FCS_CHKW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTENDED_FCS_CHKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTENDED_FCS_CHKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Packet Processor will not check FCS for Frame Type EXTENDED (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTENDED_FCS_CHKW::_0)
    }
    #[doc = "Packet Processor will check FCS at end-of-packet based on packet length derived from PHR, for Frame Type EXTENDED"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTENDED_FCS_CHKW::_1)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Beacon Frame Type Enable"]
    #[inline]
    pub fn beacon_ft(&self) -> BEACON_FTR {
        BEACON_FTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Data Frame Type Enable"]
    #[inline]
    pub fn data_ft(&self) -> DATA_FTR {
        DATA_FTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Ack Frame Type Enable"]
    #[inline]
    pub fn ack_ft(&self) -> ACK_FTR {
        ACK_FTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - MAC Command Frame Type Enable"]
    #[inline]
    pub fn cmd_ft(&self) -> CMD_FTR {
        CMD_FTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - LLDN Frame Type Enable"]
    #[inline]
    pub fn lldn_ft(&self) -> LLDN_FTR {
        LLDN_FTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Multipurpose Frame Type Enable"]
    #[inline]
    pub fn multipurpose_ft(&self) -> MULTIPURPOSE_FTR {
        MULTIPURPOSE_FTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - \"Not Specified\" Frame Type Enable"]
    #[inline]
    pub fn ns_ft(&self) -> NS_FTR {
        NS_FTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Extended Frame Type Enable"]
    #[inline]
    pub fn extended_ft(&self) -> EXTENDED_FTR {
        EXTENDED_FTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Frame Version selector."]
    #[inline]
    pub fn frm_ver_filter(&self) -> FRM_VER_FILTERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FRM_VER_FILTERR { bits }
    }
    #[doc = "Bit 14 - Active Promiscuous"]
    #[inline]
    pub fn active_promiscuous(&self) -> ACTIVE_PROMISCUOUSR {
        ACTIVE_PROMISCUOUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Verify FCS on Frame Type Extended"]
    #[inline]
    pub fn extended_fcs_chk(&self) -> EXTENDED_FCS_CHKR {
        EXTENDED_FCS_CHKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Frame Version 2 Beacon Packet Received"]
    #[inline]
    pub fn fv2_beacon_recd(&self) -> FV2_BEACON_RECDR {
        FV2_BEACON_RECDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Frame Version 2 Data Packet Received"]
    #[inline]
    pub fn fv2_data_recd(&self) -> FV2_DATA_RECDR {
        FV2_DATA_RECDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Frame Version 2 Acknowledge Packet Received"]
    #[inline]
    pub fn fv2_ack_recd(&self) -> FV2_ACK_RECDR {
        FV2_ACK_RECDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Frame Version 2 MAC Command Packet Received"]
    #[inline]
    pub fn fv2_cmd_recd(&self) -> FV2_CMD_RECDR {
        FV2_CMD_RECDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - LLDN Packet Received"]
    #[inline]
    pub fn lldn_recd(&self) -> LLDN_RECDR {
        LLDN_RECDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Multipurpose Packet Received"]
    #[inline]
    pub fn multipurpose_recd(&self) -> MULTIPURPOSE_RECDR {
        MULTIPURPOSE_RECDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Extended Packet Received"]
    #[inline]
    pub fn extended_recd(&self) -> EXTENDED_RECDR {
        EXTENDED_RECDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 783 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Beacon Frame Type Enable"]
    #[inline]
    pub fn beacon_ft(&mut self) -> _BEACON_FTW {
        _BEACON_FTW { w: self }
    }
    #[doc = "Bit 1 - Data Frame Type Enable"]
    #[inline]
    pub fn data_ft(&mut self) -> _DATA_FTW {
        _DATA_FTW { w: self }
    }
    #[doc = "Bit 2 - Ack Frame Type Enable"]
    #[inline]
    pub fn ack_ft(&mut self) -> _ACK_FTW {
        _ACK_FTW { w: self }
    }
    #[doc = "Bit 3 - MAC Command Frame Type Enable"]
    #[inline]
    pub fn cmd_ft(&mut self) -> _CMD_FTW {
        _CMD_FTW { w: self }
    }
    #[doc = "Bit 4 - LLDN Frame Type Enable"]
    #[inline]
    pub fn lldn_ft(&mut self) -> _LLDN_FTW {
        _LLDN_FTW { w: self }
    }
    #[doc = "Bit 5 - Multipurpose Frame Type Enable"]
    #[inline]
    pub fn multipurpose_ft(&mut self) -> _MULTIPURPOSE_FTW {
        _MULTIPURPOSE_FTW { w: self }
    }
    #[doc = "Bit 6 - \"Not Specified\" Frame Type Enable"]
    #[inline]
    pub fn ns_ft(&mut self) -> _NS_FTW {
        _NS_FTW { w: self }
    }
    #[doc = "Bit 7 - Extended Frame Type Enable"]
    #[inline]
    pub fn extended_ft(&mut self) -> _EXTENDED_FTW {
        _EXTENDED_FTW { w: self }
    }
    #[doc = "Bits 8:11 - Frame Version selector."]
    #[inline]
    pub fn frm_ver_filter(&mut self) -> _FRM_VER_FILTERW {
        _FRM_VER_FILTERW { w: self }
    }
    #[doc = "Bit 14 - Active Promiscuous"]
    #[inline]
    pub fn active_promiscuous(&mut self) -> _ACTIVE_PROMISCUOUSW {
        _ACTIVE_PROMISCUOUSW { w: self }
    }
    #[doc = "Bit 15 - Verify FCS on Frame Type Extended"]
    #[inline]
    pub fn extended_fcs_chk(&mut self) -> _EXTENDED_FCS_CHKW {
        _EXTENDED_FCS_CHKW { w: self }
    }
}
