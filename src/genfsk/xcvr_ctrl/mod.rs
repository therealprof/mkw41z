#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XCVR_CTRL {
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
#[doc = "Possible values of the field `SEQCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQCMDR {
    #[doc = "No Action"]
    _0X0,
    #[doc = "TX Start Now"]
    _0X1,
    #[doc = "TX Start @ T1 Timer Compare Match (EVENT_TMR = T1_CMP)"]
    _0X2,
    #[doc = "TX Start @ T2 Timer Compare Match (EVENT_TMR = T2_CMP)"]
    _0X3,
    #[doc = "TX Cancel -- Cancels pending TX events but do not abort a TX-in-progress"]
    _0X4,
    #[doc = "RX Start Now"]
    _0X5,
    #[doc = "RX Start @ T1 Timer Compare Match (EVENT_TMR = T1_CMP)"]
    _0X6,
    #[doc = "RX Start @ T2 Timer Compare Match (EVENT_TMR = T2_CMP)"]
    _0X7,
    #[doc = "RX Stop @ T1 Timer Compare Match (EVENT_TMR = T1_CMP)"]
    _0X8,
    #[doc = "RX Stop @ T2 Timer Compare Match (EVENT_TMR = T2_CMP)"]
    _0X9,
    #[doc = "RX Cancel -- Cancels pending RX events but do not abort a RX-in-progress"]
    _0XA,
    #[doc = "Abort All - Cancels all pending events and abort any sequence-in-progress"]
    _0XB,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEQCMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEQCMDR::_0X0 => 0,
            SEQCMDR::_0X1 => 1,
            SEQCMDR::_0X2 => 2,
            SEQCMDR::_0X3 => 3,
            SEQCMDR::_0X4 => 4,
            SEQCMDR::_0X5 => 5,
            SEQCMDR::_0X6 => 6,
            SEQCMDR::_0X7 => 7,
            SEQCMDR::_0X8 => 8,
            SEQCMDR::_0X9 => 9,
            SEQCMDR::_0XA => 10,
            SEQCMDR::_0XB => 11,
            SEQCMDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEQCMDR {
        match value {
            0 => SEQCMDR::_0X0,
            1 => SEQCMDR::_0X1,
            2 => SEQCMDR::_0X2,
            3 => SEQCMDR::_0X3,
            4 => SEQCMDR::_0X4,
            5 => SEQCMDR::_0X5,
            6 => SEQCMDR::_0X6,
            7 => SEQCMDR::_0X7,
            8 => SEQCMDR::_0X8,
            9 => SEQCMDR::_0X9,
            10 => SEQCMDR::_0XA,
            11 => SEQCMDR::_0XB,
            i => SEQCMDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline]
    pub fn is_0x0(&self) -> bool {
        *self == SEQCMDR::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline]
    pub fn is_0x1(&self) -> bool {
        *self == SEQCMDR::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline]
    pub fn is_0x2(&self) -> bool {
        *self == SEQCMDR::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline]
    pub fn is_0x3(&self) -> bool {
        *self == SEQCMDR::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline]
    pub fn is_0x4(&self) -> bool {
        *self == SEQCMDR::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline]
    pub fn is_0x5(&self) -> bool {
        *self == SEQCMDR::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline]
    pub fn is_0x6(&self) -> bool {
        *self == SEQCMDR::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline]
    pub fn is_0x7(&self) -> bool {
        *self == SEQCMDR::_0X7
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline]
    pub fn is_0x8(&self) -> bool {
        *self == SEQCMDR::_0X8
    }
    #[doc = "Checks if the value of the field is `_0X9`"]
    #[inline]
    pub fn is_0x9(&self) -> bool {
        *self == SEQCMDR::_0X9
    }
    #[doc = "Checks if the value of the field is `_0XA`"]
    #[inline]
    pub fn is_0x_a(&self) -> bool {
        *self == SEQCMDR::_0XA
    }
    #[doc = "Checks if the value of the field is `_0XB`"]
    #[inline]
    pub fn is_0x_b(&self) -> bool {
        *self == SEQCMDR::_0XB
    }
}
#[doc = r" Value of the field"]
pub struct CMDDEC_CSR {
    bits: u8,
}
impl CMDDEC_CSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `XCVR_BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XCVR_BUSYR {
    #[doc = "IDLE"]
    _0,
    #[doc = "BUSY"]
    _1,
}
impl XCVR_BUSYR {
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
            XCVR_BUSYR::_0 => false,
            XCVR_BUSYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XCVR_BUSYR {
        match value {
            false => XCVR_BUSYR::_0,
            true => XCVR_BUSYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == XCVR_BUSYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == XCVR_BUSYR::_1
    }
}
#[doc = "Values that can be written to the field `SEQCMD`"]
pub enum SEQCMDW {
    #[doc = "No Action"]
    _0X0,
    #[doc = "TX Start Now"]
    _0X1,
    #[doc = "TX Start @ T1 Timer Compare Match (EVENT_TMR = T1_CMP)"]
    _0X2,
    #[doc = "TX Start @ T2 Timer Compare Match (EVENT_TMR = T2_CMP)"]
    _0X3,
    #[doc = "TX Cancel -- Cancels pending TX events but do not abort a TX-in-progress"]
    _0X4,
    #[doc = "RX Start Now"]
    _0X5,
    #[doc = "RX Start @ T1 Timer Compare Match (EVENT_TMR = T1_CMP)"]
    _0X6,
    #[doc = "RX Start @ T2 Timer Compare Match (EVENT_TMR = T2_CMP)"]
    _0X7,
    #[doc = "RX Stop @ T1 Timer Compare Match (EVENT_TMR = T1_CMP)"]
    _0X8,
    #[doc = "RX Stop @ T2 Timer Compare Match (EVENT_TMR = T2_CMP)"]
    _0X9,
    #[doc = "RX Cancel -- Cancels pending RX events but do not abort a RX-in-progress"]
    _0XA,
    #[doc = "Abort All - Cancels all pending events and abort any sequence-in-progress"]
    _0XB,
}
impl SEQCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SEQCMDW::_0X0 => 0,
            SEQCMDW::_0X1 => 1,
            SEQCMDW::_0X2 => 2,
            SEQCMDW::_0X3 => 3,
            SEQCMDW::_0X4 => 4,
            SEQCMDW::_0X5 => 5,
            SEQCMDW::_0X6 => 6,
            SEQCMDW::_0X7 => 7,
            SEQCMDW::_0X8 => 8,
            SEQCMDW::_0X9 => 9,
            SEQCMDW::_0XA => 10,
            SEQCMDW::_0XB => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQCMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQCMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Action"]
    #[inline]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(SEQCMDW::_0X0)
    }
    #[doc = "TX Start Now"]
    #[inline]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(SEQCMDW::_0X1)
    }
    #[doc = "TX Start @ T1 Timer Compare Match (EVENT_TMR = T1_CMP)"]
    #[inline]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(SEQCMDW::_0X2)
    }
    #[doc = "TX Start @ T2 Timer Compare Match (EVENT_TMR = T2_CMP)"]
    #[inline]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(SEQCMDW::_0X3)
    }
    #[doc = "TX Cancel -- Cancels pending TX events but do not abort a TX-in-progress"]
    #[inline]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(SEQCMDW::_0X4)
    }
    #[doc = "RX Start Now"]
    #[inline]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(SEQCMDW::_0X5)
    }
    #[doc = "RX Start @ T1 Timer Compare Match (EVENT_TMR = T1_CMP)"]
    #[inline]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(SEQCMDW::_0X6)
    }
    #[doc = "RX Start @ T2 Timer Compare Match (EVENT_TMR = T2_CMP)"]
    #[inline]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(SEQCMDW::_0X7)
    }
    #[doc = "RX Stop @ T1 Timer Compare Match (EVENT_TMR = T1_CMP)"]
    #[inline]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(SEQCMDW::_0X8)
    }
    #[doc = "RX Stop @ T2 Timer Compare Match (EVENT_TMR = T2_CMP)"]
    #[inline]
    pub fn _0x9(self) -> &'a mut W {
        self.variant(SEQCMDW::_0X9)
    }
    #[doc = "RX Cancel -- Cancels pending RX events but do not abort a RX-in-progress"]
    #[inline]
    pub fn _0x_a(self) -> &'a mut W {
        self.variant(SEQCMDW::_0XA)
    }
    #[doc = "Abort All - Cancels all pending events and abort any sequence-in-progress"]
    #[inline]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(SEQCMDW::_0XB)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Sequence Commands"]
    #[inline]
    pub fn seqcmd(&self) -> SEQCMDR {
        SEQCMDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Command Decode"]
    #[inline]
    pub fn cmddec_cs(&self) -> CMDDEC_CSR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMDDEC_CSR { bits }
    }
    #[doc = "Bit 31 - Transceiver Busy"]
    #[inline]
    pub fn xcvr_busy(&self) -> XCVR_BUSYR {
        XCVR_BUSYR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Sequence Commands"]
    #[inline]
    pub fn seqcmd(&mut self) -> _SEQCMDW {
        _SEQCMDW { w: self }
    }
}
