#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRQSTS {
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
#[doc = "Possible values of the field `SEQIRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQIRQR {
    #[doc = "A Sequencer Interrupt has not occurred"]
    _0,
    #[doc = "A Sequencer Interrupt has occurred"]
    _1,
}
impl SEQIRQR {
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
            SEQIRQR::_0 => false,
            SEQIRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEQIRQR {
        match value {
            false => SEQIRQR::_0,
            true => SEQIRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SEQIRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SEQIRQR::_1
    }
}
#[doc = "Possible values of the field `TXIRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIRQR {
    #[doc = "A TX Interrupt has not occurred"]
    _0,
    #[doc = "A TX Interrupt has occurred"]
    _1,
}
impl TXIRQR {
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
            TXIRQR::_0 => false,
            TXIRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXIRQR {
        match value {
            false => TXIRQR::_0,
            true => TXIRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXIRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXIRQR::_1
    }
}
#[doc = "Possible values of the field `RXIRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIRQR {
    #[doc = "A RX Interrupt has not occurred"]
    _0,
    #[doc = "A RX Interrupt has occurred"]
    _1,
}
impl RXIRQR {
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
            RXIRQR::_0 => false,
            RXIRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXIRQR {
        match value {
            false => RXIRQR::_0,
            true => RXIRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXIRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXIRQR::_1
    }
}
#[doc = "Possible values of the field `CCAIRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCAIRQR {
    #[doc = "A CCA Interrupt has not occurred"]
    _0,
    #[doc = "A CCA Interrupt has occurred"]
    _1,
}
impl CCAIRQR {
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
            CCAIRQR::_0 => false,
            CCAIRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCAIRQR {
        match value {
            false => CCAIRQR::_0,
            true => CCAIRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCAIRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCAIRQR::_1
    }
}
#[doc = "Possible values of the field `RXWTRMRKIRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXWTRMRKIRQR {
    #[doc = "A Receive Watermark Interrupt has not occurred"]
    _0,
    #[doc = "A Receive Watermark Interrupt has occurred"]
    _1,
}
impl RXWTRMRKIRQR {
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
            RXWTRMRKIRQR::_0 => false,
            RXWTRMRKIRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXWTRMRKIRQR {
        match value {
            false => RXWTRMRKIRQR::_0,
            true => RXWTRMRKIRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXWTRMRKIRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXWTRMRKIRQR::_1
    }
}
#[doc = "Possible values of the field `FILTERFAIL_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTERFAIL_IRQR {
    #[doc = "A Filter Fail Interrupt has not occurred"]
    _0,
    #[doc = "A Filter Fail Interrupt has occurred"]
    _1,
}
impl FILTERFAIL_IRQR {
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
            FILTERFAIL_IRQR::_0 => false,
            FILTERFAIL_IRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FILTERFAIL_IRQR {
        match value {
            false => FILTERFAIL_IRQR::_0,
            true => FILTERFAIL_IRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FILTERFAIL_IRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FILTERFAIL_IRQR::_1
    }
}
#[doc = "Possible values of the field `PLL_UNLOCK_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_UNLOCK_IRQR {
    #[doc = "A PLL Unlock Interrupt has not occurred"]
    _0,
    #[doc = "A PLL Unlock Interrupt has occurred"]
    _1,
}
impl PLL_UNLOCK_IRQR {
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
            PLL_UNLOCK_IRQR::_0 => false,
            PLL_UNLOCK_IRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLL_UNLOCK_IRQR {
        match value {
            false => PLL_UNLOCK_IRQR::_0,
            true => PLL_UNLOCK_IRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLL_UNLOCK_IRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLL_UNLOCK_IRQR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_FRM_PENDR {
    bits: bool,
}
impl RX_FRM_PENDR {
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
#[doc = "Possible values of the field `WAKE_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_IRQR {
    #[doc = "A Wake Interrupt has not occurred"]
    _0,
    #[doc = "A Wake Interrupt has occurred"]
    _1,
}
impl WAKE_IRQR {
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
            WAKE_IRQR::_0 => false,
            WAKE_IRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKE_IRQR {
        match value {
            false => WAKE_IRQR::_0,
            true => WAKE_IRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WAKE_IRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WAKE_IRQR::_1
    }
}
#[doc = "Possible values of the field `TSM_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_IRQR {
    #[doc = "A TSM Interrupt has not occurred"]
    _0,
    #[doc = "A TSM Interrupt has occurred"]
    _1,
}
impl TSM_IRQR {
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
            TSM_IRQR::_0 => false,
            TSM_IRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSM_IRQR {
        match value {
            false => TSM_IRQR::_0,
            true => TSM_IRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSM_IRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSM_IRQR::_1
    }
}
#[doc = "Possible values of the field `ENH_PKT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENH_PKT_STATUSR {
    #[doc = "The last packet received was neither 4e- nor 2015-compliant"]
    _0,
    #[doc = "The last packet received was 4e- or 2015-compliant (RX_FRAME_FILTER register should be queried for additional status bits)"]
    _1,
}
impl ENH_PKT_STATUSR {
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
            ENH_PKT_STATUSR::_0 => false,
            ENH_PKT_STATUSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENH_PKT_STATUSR {
        match value {
            false => ENH_PKT_STATUSR::_0,
            true => ENH_PKT_STATUSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENH_PKT_STATUSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENH_PKT_STATUSR::_1
    }
}
#[doc = "Possible values of the field `PI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIR {
    #[doc = "the received packet was not a data request"]
    _0,
    #[doc = "the received packet was a data request, regardless of whether a Source Address table match occurred, or whether Source Address Management is enabled or not"]
    _1,
}
impl PIR {
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
            PIR::_0 => false,
            PIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIR {
        match value {
            false => PIR::_0,
            true => PIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PIR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SRCADDRR {
    bits: bool,
}
impl SRCADDRR {
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
#[doc = "Possible values of the field `CCA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCAR {
    #[doc = "IDLE"]
    _0,
    #[doc = "BUSY"]
    _1,
}
impl CCAR {
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
            CCAR::_0 => false,
            CCAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCAR {
        match value {
            false => CCAR::_0,
            true => CCAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCAR::_1
    }
}
#[doc = "Possible values of the field `CRCVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCVALIDR {
    #[doc = "Rx FCS != calculated CRC (incorrect)"]
    _0,
    #[doc = "Rx FCS = calculated CRC (correct)"]
    _1,
}
impl CRCVALIDR {
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
            CRCVALIDR::_0 => false,
            CRCVALIDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCVALIDR {
        match value {
            false => CRCVALIDR::_0,
            true => CRCVALIDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRCVALIDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRCVALIDR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TMR1IRQR {
    bits: bool,
}
impl TMR1IRQR {
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
#[doc = r" Value of the field"]
pub struct TMR2IRQR {
    bits: bool,
}
impl TMR2IRQR {
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
#[doc = r" Value of the field"]
pub struct TMR3IRQR {
    bits: bool,
}
impl TMR3IRQR {
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
#[doc = r" Value of the field"]
pub struct TMR4IRQR {
    bits: bool,
}
impl TMR4IRQR {
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
#[doc = "Possible values of the field `TMR1MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR1MSKR {
    #[doc = "allows interrupt when comparator matches event timer count"]
    _0,
    #[doc = "Interrupt generation is disabled, but a TMR1IRQ flag can be set"]
    _1,
}
impl TMR1MSKR {
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
            TMR1MSKR::_0 => false,
            TMR1MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMR1MSKR {
        match value {
            false => TMR1MSKR::_0,
            true => TMR1MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TMR1MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TMR1MSKR::_1
    }
}
#[doc = "Possible values of the field `TMR2MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR2MSKR {
    #[doc = "allows interrupt when comparator matches event timer count"]
    _0,
    #[doc = "Interrupt generation is disabled, but a TMR2IRQ flag can be set"]
    _1,
}
impl TMR2MSKR {
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
            TMR2MSKR::_0 => false,
            TMR2MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMR2MSKR {
        match value {
            false => TMR2MSKR::_0,
            true => TMR2MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TMR2MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TMR2MSKR::_1
    }
}
#[doc = "Possible values of the field `TMR3MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR3MSKR {
    #[doc = "allows interrupt when comparator matches event timer count"]
    _0,
    #[doc = "Interrupt generation is disabled, but a TMR3IRQ flag can be set"]
    _1,
}
impl TMR3MSKR {
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
            TMR3MSKR::_0 => false,
            TMR3MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMR3MSKR {
        match value {
            false => TMR3MSKR::_0,
            true => TMR3MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TMR3MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TMR3MSKR::_1
    }
}
#[doc = "Possible values of the field `TMR4MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR4MSKR {
    #[doc = "allows interrupt when comparator matches event timer count"]
    _0,
    #[doc = "Interrupt generation is disabled, but a TMR4IRQ flag can be set"]
    _1,
}
impl TMR4MSKR {
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
            TMR4MSKR::_0 => false,
            TMR4MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMR4MSKR {
        match value {
            false => TMR4MSKR::_0,
            true => TMR4MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TMR4MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TMR4MSKR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_FRAME_LENGTHR {
    bits: u8,
}
impl RX_FRAME_LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SEQIRQ`"]
pub enum SEQIRQW {
    #[doc = "A Sequencer Interrupt has not occurred"]
    _0,
    #[doc = "A Sequencer Interrupt has occurred"]
    _1,
}
impl SEQIRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQIRQW::_0 => false,
            SEQIRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQIRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQIRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A Sequencer Interrupt has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEQIRQW::_0)
    }
    #[doc = "A Sequencer Interrupt has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEQIRQW::_1)
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
#[doc = "Values that can be written to the field `TXIRQ`"]
pub enum TXIRQW {
    #[doc = "A TX Interrupt has not occurred"]
    _0,
    #[doc = "A TX Interrupt has occurred"]
    _1,
}
impl TXIRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXIRQW::_0 => false,
            TXIRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXIRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A TX Interrupt has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXIRQW::_0)
    }
    #[doc = "A TX Interrupt has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXIRQW::_1)
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
#[doc = "Values that can be written to the field `RXIRQ`"]
pub enum RXIRQW {
    #[doc = "A RX Interrupt has not occurred"]
    _0,
    #[doc = "A RX Interrupt has occurred"]
    _1,
}
impl RXIRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXIRQW::_0 => false,
            RXIRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A RX Interrupt has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXIRQW::_0)
    }
    #[doc = "A RX Interrupt has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXIRQW::_1)
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
#[doc = "Values that can be written to the field `CCAIRQ`"]
pub enum CCAIRQW {
    #[doc = "A CCA Interrupt has not occurred"]
    _0,
    #[doc = "A CCA Interrupt has occurred"]
    _1,
}
impl CCAIRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCAIRQW::_0 => false,
            CCAIRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCAIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CCAIRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCAIRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A CCA Interrupt has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCAIRQW::_0)
    }
    #[doc = "A CCA Interrupt has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCAIRQW::_1)
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
#[doc = "Values that can be written to the field `RXWTRMRKIRQ`"]
pub enum RXWTRMRKIRQW {
    #[doc = "A Receive Watermark Interrupt has not occurred"]
    _0,
    #[doc = "A Receive Watermark Interrupt has occurred"]
    _1,
}
impl RXWTRMRKIRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXWTRMRKIRQW::_0 => false,
            RXWTRMRKIRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXWTRMRKIRQW<'a> {
    w: &'a mut W,
}
impl<'a> _RXWTRMRKIRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXWTRMRKIRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A Receive Watermark Interrupt has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXWTRMRKIRQW::_0)
    }
    #[doc = "A Receive Watermark Interrupt has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXWTRMRKIRQW::_1)
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
#[doc = "Values that can be written to the field `FILTERFAIL_IRQ`"]
pub enum FILTERFAIL_IRQW {
    #[doc = "A Filter Fail Interrupt has not occurred"]
    _0,
    #[doc = "A Filter Fail Interrupt has occurred"]
    _1,
}
impl FILTERFAIL_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FILTERFAIL_IRQW::_0 => false,
            FILTERFAIL_IRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTERFAIL_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERFAIL_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTERFAIL_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A Filter Fail Interrupt has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILTERFAIL_IRQW::_0)
    }
    #[doc = "A Filter Fail Interrupt has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILTERFAIL_IRQW::_1)
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
#[doc = "Values that can be written to the field `PLL_UNLOCK_IRQ`"]
pub enum PLL_UNLOCK_IRQW {
    #[doc = "A PLL Unlock Interrupt has not occurred"]
    _0,
    #[doc = "A PLL Unlock Interrupt has occurred"]
    _1,
}
impl PLL_UNLOCK_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLL_UNLOCK_IRQW::_0 => false,
            PLL_UNLOCK_IRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL_UNLOCK_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_UNLOCK_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL_UNLOCK_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A PLL Unlock Interrupt has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLL_UNLOCK_IRQW::_0)
    }
    #[doc = "A PLL Unlock Interrupt has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLL_UNLOCK_IRQW::_1)
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
#[doc = "Values that can be written to the field `WAKE_IRQ`"]
pub enum WAKE_IRQW {
    #[doc = "A Wake Interrupt has not occurred"]
    _0,
    #[doc = "A Wake Interrupt has occurred"]
    _1,
}
impl WAKE_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKE_IRQW::_0 => false,
            WAKE_IRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKE_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKE_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKE_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A Wake Interrupt has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKE_IRQW::_0)
    }
    #[doc = "A Wake Interrupt has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKE_IRQW::_1)
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
#[doc = r" Proxy"]
pub struct _TMR1IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR1IRQW<'a> {
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
pub struct _TMR2IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR2IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _TMR3IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR3IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _TMR4IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR4IRQW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TMR1MSK`"]
pub enum TMR1MSKW {
    #[doc = "allows interrupt when comparator matches event timer count"]
    _0,
    #[doc = "Interrupt generation is disabled, but a TMR1IRQ flag can be set"]
    _1,
}
impl TMR1MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMR1MSKW::_0 => false,
            TMR1MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMR1MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR1MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMR1MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "allows interrupt when comparator matches event timer count"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR1MSKW::_0)
    }
    #[doc = "Interrupt generation is disabled, but a TMR1IRQ flag can be set"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR1MSKW::_1)
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
#[doc = "Values that can be written to the field `TMR2MSK`"]
pub enum TMR2MSKW {
    #[doc = "allows interrupt when comparator matches event timer count"]
    _0,
    #[doc = "Interrupt generation is disabled, but a TMR2IRQ flag can be set"]
    _1,
}
impl TMR2MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMR2MSKW::_0 => false,
            TMR2MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMR2MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR2MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMR2MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "allows interrupt when comparator matches event timer count"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR2MSKW::_0)
    }
    #[doc = "Interrupt generation is disabled, but a TMR2IRQ flag can be set"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR2MSKW::_1)
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
#[doc = "Values that can be written to the field `TMR3MSK`"]
pub enum TMR3MSKW {
    #[doc = "allows interrupt when comparator matches event timer count"]
    _0,
    #[doc = "Interrupt generation is disabled, but a TMR3IRQ flag can be set"]
    _1,
}
impl TMR3MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMR3MSKW::_0 => false,
            TMR3MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMR3MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR3MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMR3MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "allows interrupt when comparator matches event timer count"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR3MSKW::_0)
    }
    #[doc = "Interrupt generation is disabled, but a TMR3IRQ flag can be set"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR3MSKW::_1)
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
#[doc = "Values that can be written to the field `TMR4MSK`"]
pub enum TMR4MSKW {
    #[doc = "allows interrupt when comparator matches event timer count"]
    _0,
    #[doc = "Interrupt generation is disabled, but a TMR4IRQ flag can be set"]
    _1,
}
impl TMR4MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMR4MSKW::_0 => false,
            TMR4MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMR4MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR4MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMR4MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "allows interrupt when comparator matches event timer count"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR4MSKW::_0)
    }
    #[doc = "Interrupt generation is disabled, but a TMR4IRQ flag can be set"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR4MSKW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Sequencer IRQ"]
    #[inline]
    pub fn seqirq(&self) -> SEQIRQR {
        SEQIRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TX IRQ"]
    #[inline]
    pub fn txirq(&self) -> TXIRQR {
        TXIRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RX IRQ"]
    #[inline]
    pub fn rxirq(&self) -> RXIRQR {
        RXIRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - CCA IRQ"]
    #[inline]
    pub fn ccairq(&self) -> CCAIRQR {
        CCAIRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Receive Watermark IRQ"]
    #[inline]
    pub fn rxwtrmrkirq(&self) -> RXWTRMRKIRQR {
        RXWTRMRKIRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Filter Fail IRQ"]
    #[inline]
    pub fn filterfail_irq(&self) -> FILTERFAIL_IRQR {
        FILTERFAIL_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PLL Unlock IRQ"]
    #[inline]
    pub fn pll_unlock_irq(&self) -> PLL_UNLOCK_IRQR {
        PLL_UNLOCK_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - RX Frame Pending"]
    #[inline]
    pub fn rx_frm_pend(&self) -> RX_FRM_PENDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_FRM_PENDR { bits }
    }
    #[doc = "Bit 8 - WAKE Interrupt Request"]
    #[inline]
    pub fn wake_irq(&self) -> WAKE_IRQR {
        WAKE_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - TSM IRQ"]
    #[inline]
    pub fn tsm_irq(&self) -> TSM_IRQR {
        TSM_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enhanced Packet Status"]
    #[inline]
    pub fn enh_pkt_status(&self) -> ENH_PKT_STATUSR {
        ENH_PKT_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Poll Indication"]
    #[inline]
    pub fn pi(&self) -> PIR {
        PIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Source Address Match Status"]
    #[inline]
    pub fn srcaddr(&self) -> SRCADDRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRCADDRR { bits }
    }
    #[doc = "Bit 14 - CCA Status"]
    #[inline]
    pub fn cca(&self) -> CCAR {
        CCAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - CRC Valid Status"]
    #[inline]
    pub fn crcvalid(&self) -> CRCVALIDR {
        CRCVALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Timer 1 IRQ"]
    #[inline]
    pub fn tmr1irq(&self) -> TMR1IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TMR1IRQR { bits }
    }
    #[doc = "Bit 17 - Timer 2 IRQ"]
    #[inline]
    pub fn tmr2irq(&self) -> TMR2IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TMR2IRQR { bits }
    }
    #[doc = "Bit 18 - Timer 3 IRQ"]
    #[inline]
    pub fn tmr3irq(&self) -> TMR3IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TMR3IRQR { bits }
    }
    #[doc = "Bit 19 - Timer 4 IRQ"]
    #[inline]
    pub fn tmr4irq(&self) -> TMR4IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TMR4IRQR { bits }
    }
    #[doc = "Bit 20 - Timer Comperator 1 Interrupt Mask bit"]
    #[inline]
    pub fn tmr1msk(&self) -> TMR1MSKR {
        TMR1MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Timer Comperator 2 Interrupt Mask bit"]
    #[inline]
    pub fn tmr2msk(&self) -> TMR2MSKR {
        TMR2MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Timer Comperator 3 Interrupt Mask bit"]
    #[inline]
    pub fn tmr3msk(&self) -> TMR3MSKR {
        TMR3MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Timer Comperator 4 Interrupt Mask bit"]
    #[inline]
    pub fn tmr4msk(&self) -> TMR4MSKR {
        TMR4MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:30 - Receive Frame Length"]
    #[inline]
    pub fn rx_frame_length(&self) -> RX_FRAME_LENGTHR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_FRAME_LENGTHR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 15728640 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Sequencer IRQ"]
    #[inline]
    pub fn seqirq(&mut self) -> _SEQIRQW {
        _SEQIRQW { w: self }
    }
    #[doc = "Bit 1 - TX IRQ"]
    #[inline]
    pub fn txirq(&mut self) -> _TXIRQW {
        _TXIRQW { w: self }
    }
    #[doc = "Bit 2 - RX IRQ"]
    #[inline]
    pub fn rxirq(&mut self) -> _RXIRQW {
        _RXIRQW { w: self }
    }
    #[doc = "Bit 3 - CCA IRQ"]
    #[inline]
    pub fn ccairq(&mut self) -> _CCAIRQW {
        _CCAIRQW { w: self }
    }
    #[doc = "Bit 4 - Receive Watermark IRQ"]
    #[inline]
    pub fn rxwtrmrkirq(&mut self) -> _RXWTRMRKIRQW {
        _RXWTRMRKIRQW { w: self }
    }
    #[doc = "Bit 5 - Filter Fail IRQ"]
    #[inline]
    pub fn filterfail_irq(&mut self) -> _FILTERFAIL_IRQW {
        _FILTERFAIL_IRQW { w: self }
    }
    #[doc = "Bit 6 - PLL Unlock IRQ"]
    #[inline]
    pub fn pll_unlock_irq(&mut self) -> _PLL_UNLOCK_IRQW {
        _PLL_UNLOCK_IRQW { w: self }
    }
    #[doc = "Bit 8 - WAKE Interrupt Request"]
    #[inline]
    pub fn wake_irq(&mut self) -> _WAKE_IRQW {
        _WAKE_IRQW { w: self }
    }
    #[doc = "Bit 16 - Timer 1 IRQ"]
    #[inline]
    pub fn tmr1irq(&mut self) -> _TMR1IRQW {
        _TMR1IRQW { w: self }
    }
    #[doc = "Bit 17 - Timer 2 IRQ"]
    #[inline]
    pub fn tmr2irq(&mut self) -> _TMR2IRQW {
        _TMR2IRQW { w: self }
    }
    #[doc = "Bit 18 - Timer 3 IRQ"]
    #[inline]
    pub fn tmr3irq(&mut self) -> _TMR3IRQW {
        _TMR3IRQW { w: self }
    }
    #[doc = "Bit 19 - Timer 4 IRQ"]
    #[inline]
    pub fn tmr4irq(&mut self) -> _TMR4IRQW {
        _TMR4IRQW { w: self }
    }
    #[doc = "Bit 20 - Timer Comperator 1 Interrupt Mask bit"]
    #[inline]
    pub fn tmr1msk(&mut self) -> _TMR1MSKW {
        _TMR1MSKW { w: self }
    }
    #[doc = "Bit 21 - Timer Comperator 2 Interrupt Mask bit"]
    #[inline]
    pub fn tmr2msk(&mut self) -> _TMR2MSKW {
        _TMR2MSKW { w: self }
    }
    #[doc = "Bit 22 - Timer Comperator 3 Interrupt Mask bit"]
    #[inline]
    pub fn tmr3msk(&mut self) -> _TMR3MSKW {
        _TMR3MSKW { w: self }
    }
    #[doc = "Bit 23 - Timer Comperator 4 Interrupt Mask bit"]
    #[inline]
    pub fn tmr4msk(&mut self) -> _TMR4MSKW {
        _TMR4MSKW { w: self }
    }
}
