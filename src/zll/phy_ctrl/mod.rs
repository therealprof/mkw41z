#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PHY_CTRL {
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
#[doc = "Possible values of the field `XCVSEQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XCVSEQR {
    #[doc = "I (IDLE)"]
    _0,
    #[doc = "R (RECEIVE)"]
    _1,
    #[doc = "T (TRANSMIT)"]
    _2,
    #[doc = "C (CCA)"]
    _3,
    #[doc = "TR (TRANSMIT/RECEIVE)"]
    _4,
    #[doc = "CCCA (CONTINUOUS CCA)"]
    _5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl XCVSEQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            XCVSEQR::_0 => 0,
            XCVSEQR::_1 => 1,
            XCVSEQR::_2 => 2,
            XCVSEQR::_3 => 3,
            XCVSEQR::_4 => 4,
            XCVSEQR::_5 => 5,
            XCVSEQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> XCVSEQR {
        match value {
            0 => XCVSEQR::_0,
            1 => XCVSEQR::_1,
            2 => XCVSEQR::_2,
            3 => XCVSEQR::_3,
            4 => XCVSEQR::_4,
            5 => XCVSEQR::_5,
            i => XCVSEQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == XCVSEQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == XCVSEQR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == XCVSEQR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == XCVSEQR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == XCVSEQR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == XCVSEQR::_5
    }
}
#[doc = "Possible values of the field `AUTOACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOACKR {
    #[doc = "sequence manager will not follow a receive frame with a Tx Ack frame, under any conditions; the autosequence will terminate after the receive frame."]
    _0,
    #[doc = "sequence manager will follow a receive frame with an automatic hardware-generated Tx Ack frame, assuming other necessary conditions are met."]
    _1,
}
impl AUTOACKR {
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
            AUTOACKR::_0 => false,
            AUTOACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTOACKR {
        match value {
            false => AUTOACKR::_0,
            true => AUTOACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AUTOACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AUTOACKR::_1
    }
}
#[doc = "Possible values of the field `RXACKRQD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXACKRQDR {
    #[doc = "An ordinary receive frame (any type of frame) follows the transmit frame."]
    _0,
    #[doc = "A receive Ack frame is expected to follow the transmit frame (non-Ack frames are rejected)."]
    _1,
}
impl RXACKRQDR {
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
            RXACKRQDR::_0 => false,
            RXACKRQDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXACKRQDR {
        match value {
            false => RXACKRQDR::_0,
            true => RXACKRQDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXACKRQDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXACKRQDR::_1
    }
}
#[doc = "Possible values of the field `CCABFRTX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCABFRTXR {
    #[doc = "no CCA required, transmit operation begins immediately."]
    _0,
    #[doc = "at least one CCA measurement is required prior to the transmit operation (see also SLOTTED)."]
    _1,
}
impl CCABFRTXR {
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
            CCABFRTXR::_0 => false,
            CCABFRTXR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCABFRTXR {
        match value {
            false => CCABFRTXR::_0,
            true => CCABFRTXR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCABFRTXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCABFRTXR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SLOTTEDR {
    bits: bool,
}
impl SLOTTEDR {
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
#[doc = "Possible values of the field `TMRTRIGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRTRIGENR {
    #[doc = "programmed sequence initiates immediately upon write to XCVSEQ."]
    _0,
    #[doc = "allow timer TC2 (or TC2') to initiate a preprogrammed sequence (see XCVSEQ register)."]
    _1,
}
impl TMRTRIGENR {
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
            TMRTRIGENR::_0 => false,
            TMRTRIGENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMRTRIGENR {
        match value {
            false => TMRTRIGENR::_0,
            true => TMRTRIGENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TMRTRIGENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TMRTRIGENR::_1
    }
}
#[doc = "Possible values of the field `SEQMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQMSKR {
    #[doc = "allows completion of an autosequence to generate a zigbee interrupt"]
    _0,
    #[doc = "Completion of an autosequence will set the SEQIRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl SEQMSKR {
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
            SEQMSKR::_0 => false,
            SEQMSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEQMSKR {
        match value {
            false => SEQMSKR::_0,
            true => SEQMSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SEQMSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SEQMSKR::_1
    }
}
#[doc = "Possible values of the field `TXMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMSKR {
    #[doc = "allows completion of a TX operation to generate a zigbee interrupt"]
    _0,
    #[doc = "Completion of a TX operation will set the TXIRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl TXMSKR {
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
            TXMSKR::_0 => false,
            TXMSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXMSKR {
        match value {
            false => TXMSKR::_0,
            true => TXMSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXMSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXMSKR::_1
    }
}
#[doc = "Possible values of the field `RXMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXMSKR {
    #[doc = "allows completion of a RX operation to generate a zigbee interrupt"]
    _0,
    #[doc = "Completion of a RX operation will set the RXIRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl RXMSKR {
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
            RXMSKR::_0 => false,
            RXMSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXMSKR {
        match value {
            false => RXMSKR::_0,
            true => RXMSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXMSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXMSKR::_1
    }
}
#[doc = "Possible values of the field `CCAMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCAMSKR {
    #[doc = "allows completion of a CCA operation to generate a zigbee interrupt"]
    _0,
    #[doc = "Completion of a CCA operation will set the CCA status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl CCAMSKR {
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
            CCAMSKR::_0 => false,
            CCAMSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCAMSKR {
        match value {
            false => CCAMSKR::_0,
            true => CCAMSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCAMSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCAMSKR::_1
    }
}
#[doc = "Possible values of the field `RX_WMRK_MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_WMRK_MSKR {
    #[doc = "allows a Received Byte Count match to the RX_WTR_MARK threshold register to generate a zigbee interrupt"]
    _0,
    #[doc = "A Received Byte Count match to the RX_WTR_MARK threshold register will set the RXWTRMRKIRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl RX_WMRK_MSKR {
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
            RX_WMRK_MSKR::_0 => false,
            RX_WMRK_MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_WMRK_MSKR {
        match value {
            false => RX_WMRK_MSKR::_0,
            true => RX_WMRK_MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_WMRK_MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_WMRK_MSKR::_1
    }
}
#[doc = "Possible values of the field `FILTERFAIL_MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTERFAIL_MSKR {
    #[doc = "allows Packet Processor Filtering Failure to generate a zigbee interrupt"]
    _0,
    #[doc = "A Packet Processor Filtering Failure will set the FILTERFAIL_IRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl FILTERFAIL_MSKR {
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
            FILTERFAIL_MSKR::_0 => false,
            FILTERFAIL_MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FILTERFAIL_MSKR {
        match value {
            false => FILTERFAIL_MSKR::_0,
            true => FILTERFAIL_MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FILTERFAIL_MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FILTERFAIL_MSKR::_1
    }
}
#[doc = "Possible values of the field `PLL_UNLOCK_MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_UNLOCK_MSKR {
    #[doc = "allows PLL unlock event to generate a zigbee interrupt"]
    _0,
    #[doc = "A PLL unlock event will set the PLL_UNLOCK_IRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl PLL_UNLOCK_MSKR {
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
            PLL_UNLOCK_MSKR::_0 => false,
            PLL_UNLOCK_MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLL_UNLOCK_MSKR {
        match value {
            false => PLL_UNLOCK_MSKR::_0,
            true => PLL_UNLOCK_MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLL_UNLOCK_MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLL_UNLOCK_MSKR::_1
    }
}
#[doc = "Possible values of the field `CRC_MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_MSKR {
    #[doc = "sequence manager ignores CRCVALID and considers the receive operation complete after the last octet of the frame has been received."]
    _0,
    #[doc = "sequence manager requires CRCVALID=1 at the end of the received frame in order for the receive operation to complete successfully; if CRCVALID=0, sequence manager will return to preamble-detect mode after the last octet of the frame has been received."]
    _1,
}
impl CRC_MSKR {
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
            CRC_MSKR::_0 => false,
            CRC_MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC_MSKR {
        match value {
            false => CRC_MSKR::_0,
            true => CRC_MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRC_MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRC_MSKR::_1
    }
}
#[doc = "Possible values of the field `WAKE_MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_MSKR {
    #[doc = "Allows a wakeup from DSM to generate a zigbee interrupt"]
    _0,
    #[doc = "Wakeup from DSM will set the WAKE_IRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl WAKE_MSKR {
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
            WAKE_MSKR::_0 => false,
            WAKE_MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKE_MSKR {
        match value {
            false => WAKE_MSKR::_0,
            true => WAKE_MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WAKE_MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WAKE_MSKR::_1
    }
}
#[doc = "Possible values of the field `TSM_MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_MSKR {
    #[doc = "allows assertion of a TSM interrupt to generate a zigbee interrupt"]
    _0,
    #[doc = "Assertion of a TSM interrupt will set the TSM_IRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl TSM_MSKR {
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
            TSM_MSKR::_0 => false,
            TSM_MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSM_MSKR {
        match value {
            false => TSM_MSKR::_0,
            true => TSM_MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSM_MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSM_MSKR::_1
    }
}
#[doc = "Possible values of the field `TMR1CMP_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR1CMP_ENR {
    #[doc = "Don't allow an Event Timer Match to T1CMP to set TMR1IRQ"]
    _0,
    #[doc = "Allow an Event Timer Match to T1CMP to set TMR1IRQ"]
    _1,
}
impl TMR1CMP_ENR {
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
            TMR1CMP_ENR::_0 => false,
            TMR1CMP_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMR1CMP_ENR {
        match value {
            false => TMR1CMP_ENR::_0,
            true => TMR1CMP_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TMR1CMP_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TMR1CMP_ENR::_1
    }
}
#[doc = "Possible values of the field `TMR2CMP_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR2CMP_ENR {
    #[doc = "Don't allow an Event Timer Match to T2CMP or T2PRIMECMP to set TMR2IRQ"]
    _0,
    #[doc = "Allow an Event Timer Match to T2CMP or T2PRIMECMP to set TMR2IRQ"]
    _1,
}
impl TMR2CMP_ENR {
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
            TMR2CMP_ENR::_0 => false,
            TMR2CMP_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMR2CMP_ENR {
        match value {
            false => TMR2CMP_ENR::_0,
            true => TMR2CMP_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TMR2CMP_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TMR2CMP_ENR::_1
    }
}
#[doc = "Possible values of the field `TMR3CMP_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR3CMP_ENR {
    #[doc = "Don't allow an Event Timer Match to T3CMP to set TMR3IRQ"]
    _0,
    #[doc = "Allow an Event Timer Match to T3CMP to set TMR3IRQ"]
    _1,
}
impl TMR3CMP_ENR {
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
            TMR3CMP_ENR::_0 => false,
            TMR3CMP_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMR3CMP_ENR {
        match value {
            false => TMR3CMP_ENR::_0,
            true => TMR3CMP_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TMR3CMP_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TMR3CMP_ENR::_1
    }
}
#[doc = "Possible values of the field `TMR4CMP_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR4CMP_ENR {
    #[doc = "Don't allow an Event Timer Match to T4CMP to set TMR4IRQ"]
    _0,
    #[doc = "Allow an Event Timer Match to T4CMP to set TMR4IRQ"]
    _1,
}
impl TMR4CMP_ENR {
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
            TMR4CMP_ENR::_0 => false,
            TMR4CMP_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMR4CMP_ENR {
        match value {
            false => TMR4CMP_ENR::_0,
            true => TMR4CMP_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TMR4CMP_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TMR4CMP_ENR::_1
    }
}
#[doc = "Possible values of the field `TC2PRIME_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2PRIME_ENR {
    #[doc = "Don't allow a match of the lower 16 bits of Event Timer to T2PRIMECMP to set TMR2IRQ"]
    _0,
    #[doc = "Allow a match of the lower 16 bits of Event Timer to T2PRIMECMP to set TMR2IRQ"]
    _1,
}
impl TC2PRIME_ENR {
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
            TC2PRIME_ENR::_0 => false,
            TC2PRIME_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC2PRIME_ENR {
        match value {
            false => TC2PRIME_ENR::_0,
            true => TC2PRIME_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TC2PRIME_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TC2PRIME_ENR::_1
    }
}
#[doc = "Possible values of the field `PROMISCUOUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROMISCUOUSR {
    #[doc = "normal mode"]
    _0,
    #[doc = "all packet filtering except frame length checking (FrameLength>=5 and FrameLength<=127) is bypassed."]
    _1,
}
impl PROMISCUOUSR {
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
            PROMISCUOUSR::_0 => false,
            PROMISCUOUSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROMISCUOUSR {
        match value {
            false => PROMISCUOUSR::_0,
            true => PROMISCUOUSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PROMISCUOUSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PROMISCUOUSR::_1
    }
}
#[doc = "Possible values of the field `CCATYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCATYPER {
    #[doc = "ENERGY DETECT"]
    _0,
    #[doc = "CCA MODE 1"]
    _1,
    #[doc = "CCA MODE 2"]
    _2,
    #[doc = "CCA MODE 3"]
    _3,
}
impl CCATYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CCATYPER::_0 => 0,
            CCATYPER::_1 => 1,
            CCATYPER::_2 => 2,
            CCATYPER::_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CCATYPER {
        match value {
            0 => CCATYPER::_0,
            1 => CCATYPER::_1,
            2 => CCATYPER::_2,
            3 => CCATYPER::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCATYPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCATYPER::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == CCATYPER::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == CCATYPER::_3
    }
}
#[doc = r" Value of the field"]
pub struct PANCORDNTR0R {
    bits: bool,
}
impl PANCORDNTR0R {
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
#[doc = "Possible values of the field `TC3TMOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC3TMOUTR {
    #[doc = "TMR3 is a software timer only"]
    _0,
    #[doc = "Enable TMR3 to abort Rx or CCCA operations."]
    _1,
}
impl TC3TMOUTR {
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
            TC3TMOUTR::_0 => false,
            TC3TMOUTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC3TMOUTR {
        match value {
            false => TC3TMOUTR::_0,
            true => TC3TMOUTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TC3TMOUTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TC3TMOUTR::_1
    }
}
#[doc = "Possible values of the field `TRCV_MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRCV_MSKR {
    #[doc = "Enable any unmasked interrupt source to assert zigbee interrupt"]
    _0,
    #[doc = "Mask all interrupt sources from asserting zigbee interrupt"]
    _1,
}
impl TRCV_MSKR {
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
            TRCV_MSKR::_0 => false,
            TRCV_MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRCV_MSKR {
        match value {
            false => TRCV_MSKR::_0,
            true => TRCV_MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRCV_MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRCV_MSKR::_1
    }
}
#[doc = "Values that can be written to the field `XCVSEQ`"]
pub enum XCVSEQW {
    #[doc = "I (IDLE)"]
    _0,
    #[doc = "R (RECEIVE)"]
    _1,
    #[doc = "T (TRANSMIT)"]
    _2,
    #[doc = "C (CCA)"]
    _3,
    #[doc = "TR (TRANSMIT/RECEIVE)"]
    _4,
    #[doc = "CCCA (CONTINUOUS CCA)"]
    _5,
}
impl XCVSEQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            XCVSEQW::_0 => 0,
            XCVSEQW::_1 => 1,
            XCVSEQW::_2 => 2,
            XCVSEQW::_3 => 3,
            XCVSEQW::_4 => 4,
            XCVSEQW::_5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XCVSEQW<'a> {
    w: &'a mut W,
}
impl<'a> _XCVSEQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XCVSEQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "I (IDLE)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(XCVSEQW::_0)
    }
    #[doc = "R (RECEIVE)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(XCVSEQW::_1)
    }
    #[doc = "T (TRANSMIT)"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(XCVSEQW::_2)
    }
    #[doc = "C (CCA)"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(XCVSEQW::_3)
    }
    #[doc = "TR (TRANSMIT/RECEIVE)"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(XCVSEQW::_4)
    }
    #[doc = "CCCA (CONTINUOUS CCA)"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(XCVSEQW::_5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUTOACK`"]
pub enum AUTOACKW {
    #[doc = "sequence manager will not follow a receive frame with a Tx Ack frame, under any conditions; the autosequence will terminate after the receive frame."]
    _0,
    #[doc = "sequence manager will follow a receive frame with an automatic hardware-generated Tx Ack frame, assuming other necessary conditions are met."]
    _1,
}
impl AUTOACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOACKW::_0 => false,
            AUTOACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTOACKW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTOACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "sequence manager will not follow a receive frame with a Tx Ack frame, under any conditions; the autosequence will terminate after the receive frame."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AUTOACKW::_0)
    }
    #[doc = "sequence manager will follow a receive frame with an automatic hardware-generated Tx Ack frame, assuming other necessary conditions are met."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AUTOACKW::_1)
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
#[doc = "Values that can be written to the field `RXACKRQD`"]
pub enum RXACKRQDW {
    #[doc = "An ordinary receive frame (any type of frame) follows the transmit frame."]
    _0,
    #[doc = "A receive Ack frame is expected to follow the transmit frame (non-Ack frames are rejected)."]
    _1,
}
impl RXACKRQDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXACKRQDW::_0 => false,
            RXACKRQDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXACKRQDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXACKRQDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXACKRQDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An ordinary receive frame (any type of frame) follows the transmit frame."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXACKRQDW::_0)
    }
    #[doc = "A receive Ack frame is expected to follow the transmit frame (non-Ack frames are rejected)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXACKRQDW::_1)
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
#[doc = "Values that can be written to the field `CCABFRTX`"]
pub enum CCABFRTXW {
    #[doc = "no CCA required, transmit operation begins immediately."]
    _0,
    #[doc = "at least one CCA measurement is required prior to the transmit operation (see also SLOTTED)."]
    _1,
}
impl CCABFRTXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCABFRTXW::_0 => false,
            CCABFRTXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCABFRTXW<'a> {
    w: &'a mut W,
}
impl<'a> _CCABFRTXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCABFRTXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no CCA required, transmit operation begins immediately."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCABFRTXW::_0)
    }
    #[doc = "at least one CCA measurement is required prior to the transmit operation (see also SLOTTED)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCABFRTXW::_1)
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
#[doc = r" Proxy"]
pub struct _SLOTTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOTTEDW<'a> {
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
#[doc = "Values that can be written to the field `TMRTRIGEN`"]
pub enum TMRTRIGENW {
    #[doc = "programmed sequence initiates immediately upon write to XCVSEQ."]
    _0,
    #[doc = "allow timer TC2 (or TC2') to initiate a preprogrammed sequence (see XCVSEQ register)."]
    _1,
}
impl TMRTRIGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMRTRIGENW::_0 => false,
            TMRTRIGENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMRTRIGENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRTRIGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMRTRIGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "programmed sequence initiates immediately upon write to XCVSEQ."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMRTRIGENW::_0)
    }
    #[doc = "allow timer TC2 (or TC2') to initiate a preprogrammed sequence (see XCVSEQ register)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMRTRIGENW::_1)
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
#[doc = "Values that can be written to the field `SEQMSK`"]
pub enum SEQMSKW {
    #[doc = "allows completion of an autosequence to generate a zigbee interrupt"]
    _0,
    #[doc = "Completion of an autosequence will set the SEQIRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl SEQMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQMSKW::_0 => false,
            SEQMSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "allows completion of an autosequence to generate a zigbee interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEQMSKW::_0)
    }
    #[doc = "Completion of an autosequence will set the SEQIRQ status bit, but a zigbee interrupt is not generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEQMSKW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXMSK`"]
pub enum TXMSKW {
    #[doc = "allows completion of a TX operation to generate a zigbee interrupt"]
    _0,
    #[doc = "Completion of a TX operation will set the TXIRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl TXMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXMSKW::_0 => false,
            TXMSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _TXMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "allows completion of a TX operation to generate a zigbee interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXMSKW::_0)
    }
    #[doc = "Completion of a TX operation will set the TXIRQ status bit, but a zigbee interrupt is not generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXMSKW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXMSK`"]
pub enum RXMSKW {
    #[doc = "allows completion of a RX operation to generate a zigbee interrupt"]
    _0,
    #[doc = "Completion of a RX operation will set the RXIRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl RXMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXMSKW::_0 => false,
            RXMSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _RXMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "allows completion of a RX operation to generate a zigbee interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXMSKW::_0)
    }
    #[doc = "Completion of a RX operation will set the RXIRQ status bit, but a zigbee interrupt is not generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXMSKW::_1)
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
#[doc = "Values that can be written to the field `CCAMSK`"]
pub enum CCAMSKW {
    #[doc = "allows completion of a CCA operation to generate a zigbee interrupt"]
    _0,
    #[doc = "Completion of a CCA operation will set the CCA status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl CCAMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCAMSKW::_0 => false,
            CCAMSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCAMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _CCAMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCAMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "allows completion of a CCA operation to generate a zigbee interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCAMSKW::_0)
    }
    #[doc = "Completion of a CCA operation will set the CCA status bit, but a zigbee interrupt is not generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCAMSKW::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_WMRK_MSK`"]
pub enum RX_WMRK_MSKW {
    #[doc = "allows a Received Byte Count match to the RX_WTR_MARK threshold register to generate a zigbee interrupt"]
    _0,
    #[doc = "A Received Byte Count match to the RX_WTR_MARK threshold register will set the RXWTRMRKIRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl RX_WMRK_MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_WMRK_MSKW::_0 => false,
            RX_WMRK_MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_WMRK_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_WMRK_MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_WMRK_MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "allows a Received Byte Count match to the RX_WTR_MARK threshold register to generate a zigbee interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_WMRK_MSKW::_0)
    }
    #[doc = "A Received Byte Count match to the RX_WTR_MARK threshold register will set the RXWTRMRKIRQ status bit, but a zigbee interrupt is not generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_WMRK_MSKW::_1)
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
#[doc = "Values that can be written to the field `FILTERFAIL_MSK`"]
pub enum FILTERFAIL_MSKW {
    #[doc = "allows Packet Processor Filtering Failure to generate a zigbee interrupt"]
    _0,
    #[doc = "A Packet Processor Filtering Failure will set the FILTERFAIL_IRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl FILTERFAIL_MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FILTERFAIL_MSKW::_0 => false,
            FILTERFAIL_MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTERFAIL_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERFAIL_MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTERFAIL_MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "allows Packet Processor Filtering Failure to generate a zigbee interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILTERFAIL_MSKW::_0)
    }
    #[doc = "A Packet Processor Filtering Failure will set the FILTERFAIL_IRQ status bit, but a zigbee interrupt is not generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILTERFAIL_MSKW::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLL_UNLOCK_MSK`"]
pub enum PLL_UNLOCK_MSKW {
    #[doc = "allows PLL unlock event to generate a zigbee interrupt"]
    _0,
    #[doc = "A PLL unlock event will set the PLL_UNLOCK_IRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl PLL_UNLOCK_MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLL_UNLOCK_MSKW::_0 => false,
            PLL_UNLOCK_MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL_UNLOCK_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_UNLOCK_MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL_UNLOCK_MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "allows PLL unlock event to generate a zigbee interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLL_UNLOCK_MSKW::_0)
    }
    #[doc = "A PLL unlock event will set the PLL_UNLOCK_IRQ status bit, but a zigbee interrupt is not generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLL_UNLOCK_MSKW::_1)
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
#[doc = "Values that can be written to the field `CRC_MSK`"]
pub enum CRC_MSKW {
    #[doc = "sequence manager ignores CRCVALID and considers the receive operation complete after the last octet of the frame has been received."]
    _0,
    #[doc = "sequence manager requires CRCVALID=1 at the end of the received frame in order for the receive operation to complete successfully; if CRCVALID=0, sequence manager will return to preamble-detect mode after the last octet of the frame has been received."]
    _1,
}
impl CRC_MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRC_MSKW::_0 => false,
            CRC_MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC_MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "sequence manager ignores CRCVALID and considers the receive operation complete after the last octet of the frame has been received."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_MSKW::_0)
    }
    #[doc = "sequence manager requires CRCVALID=1 at the end of the received frame in order for the receive operation to complete successfully; if CRCVALID=0, sequence manager will return to preamble-detect mode after the last octet of the frame has been received."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_MSKW::_1)
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
#[doc = "Values that can be written to the field `WAKE_MSK`"]
pub enum WAKE_MSKW {
    #[doc = "Allows a wakeup from DSM to generate a zigbee interrupt"]
    _0,
    #[doc = "Wakeup from DSM will set the WAKE_IRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl WAKE_MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKE_MSKW::_0 => false,
            WAKE_MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKE_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKE_MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKE_MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allows a wakeup from DSM to generate a zigbee interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKE_MSKW::_0)
    }
    #[doc = "Wakeup from DSM will set the WAKE_IRQ status bit, but a zigbee interrupt is not generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKE_MSKW::_1)
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
#[doc = "Values that can be written to the field `TSM_MSK`"]
pub enum TSM_MSKW {
    #[doc = "allows assertion of a TSM interrupt to generate a zigbee interrupt"]
    _0,
    #[doc = "Assertion of a TSM interrupt will set the TSM_IRQ status bit, but a zigbee interrupt is not generated"]
    _1,
}
impl TSM_MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSM_MSKW::_0 => false,
            TSM_MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSM_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSM_MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "allows assertion of a TSM interrupt to generate a zigbee interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSM_MSKW::_0)
    }
    #[doc = "Assertion of a TSM interrupt will set the TSM_IRQ status bit, but a zigbee interrupt is not generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSM_MSKW::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMR1CMP_EN`"]
pub enum TMR1CMP_ENW {
    #[doc = "Don't allow an Event Timer Match to T1CMP to set TMR1IRQ"]
    _0,
    #[doc = "Allow an Event Timer Match to T1CMP to set TMR1IRQ"]
    _1,
}
impl TMR1CMP_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMR1CMP_ENW::_0 => false,
            TMR1CMP_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMR1CMP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR1CMP_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMR1CMP_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Don't allow an Event Timer Match to T1CMP to set TMR1IRQ"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR1CMP_ENW::_0)
    }
    #[doc = "Allow an Event Timer Match to T1CMP to set TMR1IRQ"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR1CMP_ENW::_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMR2CMP_EN`"]
pub enum TMR2CMP_ENW {
    #[doc = "Don't allow an Event Timer Match to T2CMP or T2PRIMECMP to set TMR2IRQ"]
    _0,
    #[doc = "Allow an Event Timer Match to T2CMP or T2PRIMECMP to set TMR2IRQ"]
    _1,
}
impl TMR2CMP_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMR2CMP_ENW::_0 => false,
            TMR2CMP_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMR2CMP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR2CMP_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMR2CMP_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Don't allow an Event Timer Match to T2CMP or T2PRIMECMP to set TMR2IRQ"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR2CMP_ENW::_0)
    }
    #[doc = "Allow an Event Timer Match to T2CMP or T2PRIMECMP to set TMR2IRQ"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR2CMP_ENW::_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMR3CMP_EN`"]
pub enum TMR3CMP_ENW {
    #[doc = "Don't allow an Event Timer Match to T3CMP to set TMR3IRQ"]
    _0,
    #[doc = "Allow an Event Timer Match to T3CMP to set TMR3IRQ"]
    _1,
}
impl TMR3CMP_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMR3CMP_ENW::_0 => false,
            TMR3CMP_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMR3CMP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR3CMP_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMR3CMP_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Don't allow an Event Timer Match to T3CMP to set TMR3IRQ"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR3CMP_ENW::_0)
    }
    #[doc = "Allow an Event Timer Match to T3CMP to set TMR3IRQ"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR3CMP_ENW::_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMR4CMP_EN`"]
pub enum TMR4CMP_ENW {
    #[doc = "Don't allow an Event Timer Match to T4CMP to set TMR4IRQ"]
    _0,
    #[doc = "Allow an Event Timer Match to T4CMP to set TMR4IRQ"]
    _1,
}
impl TMR4CMP_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMR4CMP_ENW::_0 => false,
            TMR4CMP_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMR4CMP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR4CMP_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMR4CMP_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Don't allow an Event Timer Match to T4CMP to set TMR4IRQ"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR4CMP_ENW::_0)
    }
    #[doc = "Allow an Event Timer Match to T4CMP to set TMR4IRQ"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR4CMP_ENW::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TC2PRIME_EN`"]
pub enum TC2PRIME_ENW {
    #[doc = "Don't allow a match of the lower 16 bits of Event Timer to T2PRIMECMP to set TMR2IRQ"]
    _0,
    #[doc = "Allow a match of the lower 16 bits of Event Timer to T2PRIMECMP to set TMR2IRQ"]
    _1,
}
impl TC2PRIME_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TC2PRIME_ENW::_0 => false,
            TC2PRIME_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TC2PRIME_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TC2PRIME_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TC2PRIME_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Don't allow a match of the lower 16 bits of Event Timer to T2PRIMECMP to set TMR2IRQ"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TC2PRIME_ENW::_0)
    }
    #[doc = "Allow a match of the lower 16 bits of Event Timer to T2PRIMECMP to set TMR2IRQ"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TC2PRIME_ENW::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PROMISCUOUS`"]
pub enum PROMISCUOUSW {
    #[doc = "normal mode"]
    _0,
    #[doc = "all packet filtering except frame length checking (FrameLength>=5 and FrameLength<=127) is bypassed."]
    _1,
}
impl PROMISCUOUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROMISCUOUSW::_0 => false,
            PROMISCUOUSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROMISCUOUSW<'a> {
    w: &'a mut W,
}
impl<'a> _PROMISCUOUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROMISCUOUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "normal mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROMISCUOUSW::_0)
    }
    #[doc = "all packet filtering except frame length checking (FrameLength>=5 and FrameLength<=127) is bypassed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROMISCUOUSW::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCATYPE`"]
pub enum CCATYPEW {
    #[doc = "ENERGY DETECT"]
    _0,
    #[doc = "CCA MODE 1"]
    _1,
    #[doc = "CCA MODE 2"]
    _2,
    #[doc = "CCA MODE 3"]
    _3,
}
impl CCATYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CCATYPEW::_0 => 0,
            CCATYPEW::_1 => 1,
            CCATYPEW::_2 => 2,
            CCATYPEW::_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCATYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCATYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCATYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ENERGY DETECT"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCATYPEW::_0)
    }
    #[doc = "CCA MODE 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCATYPEW::_1)
    }
    #[doc = "CCA MODE 2"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(CCATYPEW::_2)
    }
    #[doc = "CCA MODE 3"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(CCATYPEW::_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PANCORDNTR0W<'a> {
    w: &'a mut W,
}
impl<'a> _PANCORDNTR0W<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TC3TMOUT`"]
pub enum TC3TMOUTW {
    #[doc = "TMR3 is a software timer only"]
    _0,
    #[doc = "Enable TMR3 to abort Rx or CCCA operations."]
    _1,
}
impl TC3TMOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TC3TMOUTW::_0 => false,
            TC3TMOUTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TC3TMOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TC3TMOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TC3TMOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TMR3 is a software timer only"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TC3TMOUTW::_0)
    }
    #[doc = "Enable TMR3 to abort Rx or CCCA operations."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TC3TMOUTW::_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRCV_MSK`"]
pub enum TRCV_MSKW {
    #[doc = "Enable any unmasked interrupt source to assert zigbee interrupt"]
    _0,
    #[doc = "Mask all interrupt sources from asserting zigbee interrupt"]
    _1,
}
impl TRCV_MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRCV_MSKW::_0 => false,
            TRCV_MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRCV_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _TRCV_MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRCV_MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable any unmasked interrupt source to assert zigbee interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRCV_MSKW::_0)
    }
    #[doc = "Mask all interrupt sources from asserting zigbee interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRCV_MSKW::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:2 - Zigbee Transceiver Sequence Selector"]
    #[inline]
    pub fn xcvseq(&self) -> XCVSEQR {
        XCVSEQR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Auto Acknowledge Enable"]
    #[inline]
    pub fn autoack(&self) -> AUTOACKR {
        AUTOACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Receive Acknowledge Frame required"]
    #[inline]
    pub fn rxackrqd(&self) -> RXACKRQDR {
        RXACKRQDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - CCA Before TX"]
    #[inline]
    pub fn ccabfrtx(&self) -> CCABFRTXR {
        CCABFRTXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Slotted Mode"]
    #[inline]
    pub fn slotted(&self) -> SLOTTEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLOTTEDR { bits }
    }
    #[doc = "Bit 7 - Timer2 Trigger Enable"]
    #[inline]
    pub fn tmrtrigen(&self) -> TMRTRIGENR {
        TMRTRIGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Sequencer Interrupt Mask"]
    #[inline]
    pub fn seqmsk(&self) -> SEQMSKR {
        SEQMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - TX Interrupt Mask"]
    #[inline]
    pub fn txmsk(&self) -> TXMSKR {
        TXMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - RX Interrupt Mask"]
    #[inline]
    pub fn rxmsk(&self) -> RXMSKR {
        RXMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - CCA Interrupt Mask"]
    #[inline]
    pub fn ccamsk(&self) -> CCAMSKR {
        CCAMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - RX Watermark Interrupt Mask"]
    #[inline]
    pub fn rx_wmrk_msk(&self) -> RX_WMRK_MSKR {
        RX_WMRK_MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - FilterFail Interrupt Mask"]
    #[inline]
    pub fn filterfail_msk(&self) -> FILTERFAIL_MSKR {
        FILTERFAIL_MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - PLL Unlock Interrupt Mask"]
    #[inline]
    pub fn pll_unlock_msk(&self) -> PLL_UNLOCK_MSKR {
        PLL_UNLOCK_MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - CRC Mask"]
    #[inline]
    pub fn crc_msk(&self) -> CRC_MSKR {
        CRC_MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - no description available"]
    #[inline]
    pub fn wake_msk(&self) -> WAKE_MSKR {
        WAKE_MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - no description available"]
    #[inline]
    pub fn tsm_msk(&self) -> TSM_MSKR {
        TSM_MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Timer 1 Compare Enable"]
    #[inline]
    pub fn tmr1cmp_en(&self) -> TMR1CMP_ENR {
        TMR1CMP_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Timer 2 Compare Enable"]
    #[inline]
    pub fn tmr2cmp_en(&self) -> TMR2CMP_ENR {
        TMR2CMP_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Timer 3 Compare Enable"]
    #[inline]
    pub fn tmr3cmp_en(&self) -> TMR3CMP_ENR {
        TMR3CMP_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Timer 4 Compare Enable"]
    #[inline]
    pub fn tmr4cmp_en(&self) -> TMR4CMP_ENR {
        TMR4CMP_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Timer 2 Prime Compare Enable"]
    #[inline]
    pub fn tc2prime_en(&self) -> TC2PRIME_ENR {
        TC2PRIME_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Promiscuous Mode Enable"]
    #[inline]
    pub fn promiscuous(&self) -> PROMISCUOUSR {
        PROMISCUOUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 27:28 - Clear Channel Assessment Type"]
    #[inline]
    pub fn ccatype(&self) -> CCATYPER {
        CCATYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - Device is a PAN Coordinator on PAN0"]
    #[inline]
    pub fn pancordntr0(&self) -> PANCORDNTR0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PANCORDNTR0R { bits }
    }
    #[doc = "Bit 30 - TMR3 Timeout Enable"]
    #[inline]
    pub fn tc3tmout(&self) -> TC3TMOUTR {
        TC3TMOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Transceiver Global Interrupt Mask"]
    #[inline]
    pub fn trcv_msk(&self) -> TRCV_MSKR {
        TRCV_MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 134610688 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Zigbee Transceiver Sequence Selector"]
    #[inline]
    pub fn xcvseq(&mut self) -> _XCVSEQW {
        _XCVSEQW { w: self }
    }
    #[doc = "Bit 3 - Auto Acknowledge Enable"]
    #[inline]
    pub fn autoack(&mut self) -> _AUTOACKW {
        _AUTOACKW { w: self }
    }
    #[doc = "Bit 4 - Receive Acknowledge Frame required"]
    #[inline]
    pub fn rxackrqd(&mut self) -> _RXACKRQDW {
        _RXACKRQDW { w: self }
    }
    #[doc = "Bit 5 - CCA Before TX"]
    #[inline]
    pub fn ccabfrtx(&mut self) -> _CCABFRTXW {
        _CCABFRTXW { w: self }
    }
    #[doc = "Bit 6 - Slotted Mode"]
    #[inline]
    pub fn slotted(&mut self) -> _SLOTTEDW {
        _SLOTTEDW { w: self }
    }
    #[doc = "Bit 7 - Timer2 Trigger Enable"]
    #[inline]
    pub fn tmrtrigen(&mut self) -> _TMRTRIGENW {
        _TMRTRIGENW { w: self }
    }
    #[doc = "Bit 8 - Sequencer Interrupt Mask"]
    #[inline]
    pub fn seqmsk(&mut self) -> _SEQMSKW {
        _SEQMSKW { w: self }
    }
    #[doc = "Bit 9 - TX Interrupt Mask"]
    #[inline]
    pub fn txmsk(&mut self) -> _TXMSKW {
        _TXMSKW { w: self }
    }
    #[doc = "Bit 10 - RX Interrupt Mask"]
    #[inline]
    pub fn rxmsk(&mut self) -> _RXMSKW {
        _RXMSKW { w: self }
    }
    #[doc = "Bit 11 - CCA Interrupt Mask"]
    #[inline]
    pub fn ccamsk(&mut self) -> _CCAMSKW {
        _CCAMSKW { w: self }
    }
    #[doc = "Bit 12 - RX Watermark Interrupt Mask"]
    #[inline]
    pub fn rx_wmrk_msk(&mut self) -> _RX_WMRK_MSKW {
        _RX_WMRK_MSKW { w: self }
    }
    #[doc = "Bit 13 - FilterFail Interrupt Mask"]
    #[inline]
    pub fn filterfail_msk(&mut self) -> _FILTERFAIL_MSKW {
        _FILTERFAIL_MSKW { w: self }
    }
    #[doc = "Bit 14 - PLL Unlock Interrupt Mask"]
    #[inline]
    pub fn pll_unlock_msk(&mut self) -> _PLL_UNLOCK_MSKW {
        _PLL_UNLOCK_MSKW { w: self }
    }
    #[doc = "Bit 15 - CRC Mask"]
    #[inline]
    pub fn crc_msk(&mut self) -> _CRC_MSKW {
        _CRC_MSKW { w: self }
    }
    #[doc = "Bit 16 - no description available"]
    #[inline]
    pub fn wake_msk(&mut self) -> _WAKE_MSKW {
        _WAKE_MSKW { w: self }
    }
    #[doc = "Bit 18 - no description available"]
    #[inline]
    pub fn tsm_msk(&mut self) -> _TSM_MSKW {
        _TSM_MSKW { w: self }
    }
    #[doc = "Bit 20 - Timer 1 Compare Enable"]
    #[inline]
    pub fn tmr1cmp_en(&mut self) -> _TMR1CMP_ENW {
        _TMR1CMP_ENW { w: self }
    }
    #[doc = "Bit 21 - Timer 2 Compare Enable"]
    #[inline]
    pub fn tmr2cmp_en(&mut self) -> _TMR2CMP_ENW {
        _TMR2CMP_ENW { w: self }
    }
    #[doc = "Bit 22 - Timer 3 Compare Enable"]
    #[inline]
    pub fn tmr3cmp_en(&mut self) -> _TMR3CMP_ENW {
        _TMR3CMP_ENW { w: self }
    }
    #[doc = "Bit 23 - Timer 4 Compare Enable"]
    #[inline]
    pub fn tmr4cmp_en(&mut self) -> _TMR4CMP_ENW {
        _TMR4CMP_ENW { w: self }
    }
    #[doc = "Bit 24 - Timer 2 Prime Compare Enable"]
    #[inline]
    pub fn tc2prime_en(&mut self) -> _TC2PRIME_ENW {
        _TC2PRIME_ENW { w: self }
    }
    #[doc = "Bit 25 - Promiscuous Mode Enable"]
    #[inline]
    pub fn promiscuous(&mut self) -> _PROMISCUOUSW {
        _PROMISCUOUSW { w: self }
    }
    #[doc = "Bits 27:28 - Clear Channel Assessment Type"]
    #[inline]
    pub fn ccatype(&mut self) -> _CCATYPEW {
        _CCATYPEW { w: self }
    }
    #[doc = "Bit 29 - Device is a PAN Coordinator on PAN0"]
    #[inline]
    pub fn pancordntr0(&mut self) -> _PANCORDNTR0W {
        _PANCORDNTR0W { w: self }
    }
    #[doc = "Bit 30 - TMR3 Timeout Enable"]
    #[inline]
    pub fn tc3tmout(&mut self) -> _TC3TMOUTW {
        _TC3TMOUTW { w: self }
    }
    #[doc = "Bit 31 - Transceiver Global Interrupt Mask"]
    #[inline]
    pub fn trcv_msk(&mut self) -> _TRCV_MSKW {
        _TRCV_MSKW { w: self }
    }
}
