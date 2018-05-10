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
#[doc = "Possible values of the field `PROTOCOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTOCOLR {
    #[doc = "BLE"]
    _0000,
    #[doc = "BLE in MBAN"]
    _0001,
    #[doc = "BLE overlap MBAN"]
    _0010,
    #[doc = "ANT"]
    _0011,
    #[doc = "Zigbee"]
    _0100,
    #[doc = "802.15.4j"]
    _0101,
    #[doc = "128 Channel FSK"]
    _0110,
    #[doc = "128 Channel GFSK"]
    _0111,
    #[doc = "Generic FSK"]
    _1000,
    #[doc = "MSK"]
    _1001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PROTOCOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PROTOCOLR::_0000 => 0,
            PROTOCOLR::_0001 => 1,
            PROTOCOLR::_0010 => 2,
            PROTOCOLR::_0011 => 3,
            PROTOCOLR::_0100 => 4,
            PROTOCOLR::_0101 => 5,
            PROTOCOLR::_0110 => 6,
            PROTOCOLR::_0111 => 7,
            PROTOCOLR::_1000 => 8,
            PROTOCOLR::_1001 => 9,
            PROTOCOLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PROTOCOLR {
        match value {
            0 => PROTOCOLR::_0000,
            1 => PROTOCOLR::_0001,
            2 => PROTOCOLR::_0010,
            3 => PROTOCOLR::_0011,
            4 => PROTOCOLR::_0100,
            5 => PROTOCOLR::_0101,
            6 => PROTOCOLR::_0110,
            7 => PROTOCOLR::_0111,
            8 => PROTOCOLR::_1000,
            9 => PROTOCOLR::_1001,
            i => PROTOCOLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == PROTOCOLR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == PROTOCOLR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == PROTOCOLR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == PROTOCOLR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == PROTOCOLR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == PROTOCOLR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == PROTOCOLR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == PROTOCOLR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == PROTOCOLR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == PROTOCOLR::_1001
    }
}
#[doc = r" Value of the field"]
pub struct TGT_PWR_SRCR {
    bits: u8,
}
impl TGT_PWR_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `REF_CLK_FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REF_CLK_FREQR {
    #[doc = "32 MHz"]
    _00,
    #[doc = "26 MHz"]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REF_CLK_FREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REF_CLK_FREQR::_00 => 0,
            REF_CLK_FREQR::_01 => 1,
            REF_CLK_FREQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REF_CLK_FREQR {
        match value {
            0 => REF_CLK_FREQR::_00,
            1 => REF_CLK_FREQR::_01,
            i => REF_CLK_FREQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == REF_CLK_FREQR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == REF_CLK_FREQR::_01
    }
}
#[doc = r" Value of the field"]
pub struct SOC_RF_OSC_CLK_GATE_ENR {
    bits: bool,
}
impl SOC_RF_OSC_CLK_GATE_ENR {
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
#[doc = "Possible values of the field `DEMOD_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEMOD_SELR {
    #[doc = "No demodulator selected"]
    _00,
    #[doc = "Use Freescale Constant Envelope demodulator"]
    _01,
    #[doc = "Use Legacy 802.15.4 demodulator"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DEMOD_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEMOD_SELR::_00 => 0,
            DEMOD_SELR::_01 => 1,
            DEMOD_SELR::_10 => 2,
            DEMOD_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEMOD_SELR {
        match value {
            0 => DEMOD_SELR::_00,
            1 => DEMOD_SELR::_01,
            2 => DEMOD_SELR::_10,
            i => DEMOD_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DEMOD_SELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DEMOD_SELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DEMOD_SELR::_10
    }
}
#[doc = "Possible values of the field `RADIO0_IRQ_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RADIO0_IRQ_SELR {
    #[doc = "Assign Radio #0 Interrupt to BLE"]
    _000,
    #[doc = "Assign Radio #0 Interrupt to 802.15.4"]
    _001,
    #[doc = "Assign Radio #0 Interrupt to ANT"]
    _010,
    #[doc = "Assign Radio #0 Interrupt to GENERIC_FSK"]
    _011,
    #[doc = "Radio #0 Interrupt unassigned"]
    _100,
    #[doc = "Radio #0 Interrupt unassigned"]
    _101,
    #[doc = "Radio #0 Interrupt unassigned"]
    _110,
    #[doc = "Radio #0 Interrupt unassigned"]
    _111,
}
impl RADIO0_IRQ_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RADIO0_IRQ_SELR::_000 => 0,
            RADIO0_IRQ_SELR::_001 => 1,
            RADIO0_IRQ_SELR::_010 => 2,
            RADIO0_IRQ_SELR::_011 => 3,
            RADIO0_IRQ_SELR::_100 => 4,
            RADIO0_IRQ_SELR::_101 => 5,
            RADIO0_IRQ_SELR::_110 => 6,
            RADIO0_IRQ_SELR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RADIO0_IRQ_SELR {
        match value {
            0 => RADIO0_IRQ_SELR::_000,
            1 => RADIO0_IRQ_SELR::_001,
            2 => RADIO0_IRQ_SELR::_010,
            3 => RADIO0_IRQ_SELR::_011,
            4 => RADIO0_IRQ_SELR::_100,
            5 => RADIO0_IRQ_SELR::_101,
            6 => RADIO0_IRQ_SELR::_110,
            7 => RADIO0_IRQ_SELR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == RADIO0_IRQ_SELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == RADIO0_IRQ_SELR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == RADIO0_IRQ_SELR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == RADIO0_IRQ_SELR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == RADIO0_IRQ_SELR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == RADIO0_IRQ_SELR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == RADIO0_IRQ_SELR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == RADIO0_IRQ_SELR::_111
    }
}
#[doc = "Possible values of the field `RADIO1_IRQ_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RADIO1_IRQ_SELR {
    #[doc = "Assign Radio #1 Interrupt to BLE"]
    _000,
    #[doc = "Assign Radio #1 Interrupt to 802.15.4"]
    _001,
    #[doc = "Assign Radio #1 Interrupt to ANT"]
    _010,
    #[doc = "Assign Radio #1 Interrupt to GENERIC_FSK"]
    _011,
    #[doc = "Radio #1 Interrupt unassigned"]
    _100,
    #[doc = "Radio #1 Interrupt unassigned"]
    _101,
    #[doc = "Radio #1 Interrupt unassigned"]
    _110,
    #[doc = "Radio #1 Interrupt unassigned"]
    _111,
}
impl RADIO1_IRQ_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RADIO1_IRQ_SELR::_000 => 0,
            RADIO1_IRQ_SELR::_001 => 1,
            RADIO1_IRQ_SELR::_010 => 2,
            RADIO1_IRQ_SELR::_011 => 3,
            RADIO1_IRQ_SELR::_100 => 4,
            RADIO1_IRQ_SELR::_101 => 5,
            RADIO1_IRQ_SELR::_110 => 6,
            RADIO1_IRQ_SELR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RADIO1_IRQ_SELR {
        match value {
            0 => RADIO1_IRQ_SELR::_000,
            1 => RADIO1_IRQ_SELR::_001,
            2 => RADIO1_IRQ_SELR::_010,
            3 => RADIO1_IRQ_SELR::_011,
            4 => RADIO1_IRQ_SELR::_100,
            5 => RADIO1_IRQ_SELR::_101,
            6 => RADIO1_IRQ_SELR::_110,
            7 => RADIO1_IRQ_SELR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == RADIO1_IRQ_SELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == RADIO1_IRQ_SELR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == RADIO1_IRQ_SELR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == RADIO1_IRQ_SELR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == RADIO1_IRQ_SELR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == RADIO1_IRQ_SELR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == RADIO1_IRQ_SELR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == RADIO1_IRQ_SELR::_111
    }
}
#[doc = "Values that can be written to the field `PROTOCOL`"]
pub enum PROTOCOLW {
    #[doc = "BLE"]
    _0000,
    #[doc = "BLE in MBAN"]
    _0001,
    #[doc = "BLE overlap MBAN"]
    _0010,
    #[doc = "ANT"]
    _0011,
    #[doc = "Zigbee"]
    _0100,
    #[doc = "802.15.4j"]
    _0101,
    #[doc = "128 Channel FSK"]
    _0110,
    #[doc = "128 Channel GFSK"]
    _0111,
    #[doc = "Generic FSK"]
    _1000,
    #[doc = "MSK"]
    _1001,
}
impl PROTOCOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PROTOCOLW::_0000 => 0,
            PROTOCOLW::_0001 => 1,
            PROTOCOLW::_0010 => 2,
            PROTOCOLW::_0011 => 3,
            PROTOCOLW::_0100 => 4,
            PROTOCOLW::_0101 => 5,
            PROTOCOLW::_0110 => 6,
            PROTOCOLW::_0111 => 7,
            PROTOCOLW::_1000 => 8,
            PROTOCOLW::_1001 => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROTOCOLW<'a> {
    w: &'a mut W,
}
impl<'a> _PROTOCOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROTOCOLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "BLE"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(PROTOCOLW::_0000)
    }
    #[doc = "BLE in MBAN"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(PROTOCOLW::_0001)
    }
    #[doc = "BLE overlap MBAN"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(PROTOCOLW::_0010)
    }
    #[doc = "ANT"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(PROTOCOLW::_0011)
    }
    #[doc = "Zigbee"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(PROTOCOLW::_0100)
    }
    #[doc = "802.15.4j"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(PROTOCOLW::_0101)
    }
    #[doc = "128 Channel FSK"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(PROTOCOLW::_0110)
    }
    #[doc = "128 Channel GFSK"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(PROTOCOLW::_0111)
    }
    #[doc = "Generic FSK"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(PROTOCOLW::_1000)
    }
    #[doc = "MSK"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(PROTOCOLW::_1001)
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
#[doc = r" Proxy"]
pub struct _TGT_PWR_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TGT_PWR_SRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REF_CLK_FREQ`"]
pub enum REF_CLK_FREQW {
    #[doc = "32 MHz"]
    _00,
    #[doc = "26 MHz"]
    _01,
}
impl REF_CLK_FREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REF_CLK_FREQW::_00 => 0,
            REF_CLK_FREQW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REF_CLK_FREQW<'a> {
    w: &'a mut W,
}
impl<'a> _REF_CLK_FREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REF_CLK_FREQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "32 MHz"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(REF_CLK_FREQW::_00)
    }
    #[doc = "26 MHz"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(REF_CLK_FREQW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_RF_OSC_CLK_GATE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_RF_OSC_CLK_GATE_ENW<'a> {
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
#[doc = "Values that can be written to the field `DEMOD_SEL`"]
pub enum DEMOD_SELW {
    #[doc = "No demodulator selected"]
    _00,
    #[doc = "Use Freescale Constant Envelope demodulator"]
    _01,
    #[doc = "Use Legacy 802.15.4 demodulator"]
    _10,
}
impl DEMOD_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DEMOD_SELW::_00 => 0,
            DEMOD_SELW::_01 => 1,
            DEMOD_SELW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEMOD_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _DEMOD_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEMOD_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No demodulator selected"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DEMOD_SELW::_00)
    }
    #[doc = "Use Freescale Constant Envelope demodulator"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DEMOD_SELW::_01)
    }
    #[doc = "Use Legacy 802.15.4 demodulator"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DEMOD_SELW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RADIO0_IRQ_SEL`"]
pub enum RADIO0_IRQ_SELW {
    #[doc = "Assign Radio #0 Interrupt to BLE"]
    _000,
    #[doc = "Assign Radio #0 Interrupt to 802.15.4"]
    _001,
    #[doc = "Assign Radio #0 Interrupt to ANT"]
    _010,
    #[doc = "Assign Radio #0 Interrupt to GENERIC_FSK"]
    _011,
    #[doc = "Radio #0 Interrupt unassigned"]
    _100,
    #[doc = "Radio #0 Interrupt unassigned"]
    _101,
    #[doc = "Radio #0 Interrupt unassigned"]
    _110,
    #[doc = "Radio #0 Interrupt unassigned"]
    _111,
}
impl RADIO0_IRQ_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RADIO0_IRQ_SELW::_000 => 0,
            RADIO0_IRQ_SELW::_001 => 1,
            RADIO0_IRQ_SELW::_010 => 2,
            RADIO0_IRQ_SELW::_011 => 3,
            RADIO0_IRQ_SELW::_100 => 4,
            RADIO0_IRQ_SELW::_101 => 5,
            RADIO0_IRQ_SELW::_110 => 6,
            RADIO0_IRQ_SELW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RADIO0_IRQ_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _RADIO0_IRQ_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RADIO0_IRQ_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Assign Radio #0 Interrupt to BLE"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(RADIO0_IRQ_SELW::_000)
    }
    #[doc = "Assign Radio #0 Interrupt to 802.15.4"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(RADIO0_IRQ_SELW::_001)
    }
    #[doc = "Assign Radio #0 Interrupt to ANT"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(RADIO0_IRQ_SELW::_010)
    }
    #[doc = "Assign Radio #0 Interrupt to GENERIC_FSK"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(RADIO0_IRQ_SELW::_011)
    }
    #[doc = "Radio #0 Interrupt unassigned"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(RADIO0_IRQ_SELW::_100)
    }
    #[doc = "Radio #0 Interrupt unassigned"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(RADIO0_IRQ_SELW::_101)
    }
    #[doc = "Radio #0 Interrupt unassigned"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(RADIO0_IRQ_SELW::_110)
    }
    #[doc = "Radio #0 Interrupt unassigned"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(RADIO0_IRQ_SELW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RADIO1_IRQ_SEL`"]
pub enum RADIO1_IRQ_SELW {
    #[doc = "Assign Radio #1 Interrupt to BLE"]
    _000,
    #[doc = "Assign Radio #1 Interrupt to 802.15.4"]
    _001,
    #[doc = "Assign Radio #1 Interrupt to ANT"]
    _010,
    #[doc = "Assign Radio #1 Interrupt to GENERIC_FSK"]
    _011,
    #[doc = "Radio #1 Interrupt unassigned"]
    _100,
    #[doc = "Radio #1 Interrupt unassigned"]
    _101,
    #[doc = "Radio #1 Interrupt unassigned"]
    _110,
    #[doc = "Radio #1 Interrupt unassigned"]
    _111,
}
impl RADIO1_IRQ_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RADIO1_IRQ_SELW::_000 => 0,
            RADIO1_IRQ_SELW::_001 => 1,
            RADIO1_IRQ_SELW::_010 => 2,
            RADIO1_IRQ_SELW::_011 => 3,
            RADIO1_IRQ_SELW::_100 => 4,
            RADIO1_IRQ_SELW::_101 => 5,
            RADIO1_IRQ_SELW::_110 => 6,
            RADIO1_IRQ_SELW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RADIO1_IRQ_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _RADIO1_IRQ_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RADIO1_IRQ_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Assign Radio #1 Interrupt to BLE"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(RADIO1_IRQ_SELW::_000)
    }
    #[doc = "Assign Radio #1 Interrupt to 802.15.4"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(RADIO1_IRQ_SELW::_001)
    }
    #[doc = "Assign Radio #1 Interrupt to ANT"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(RADIO1_IRQ_SELW::_010)
    }
    #[doc = "Assign Radio #1 Interrupt to GENERIC_FSK"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(RADIO1_IRQ_SELW::_011)
    }
    #[doc = "Radio #1 Interrupt unassigned"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(RADIO1_IRQ_SELW::_100)
    }
    #[doc = "Radio #1 Interrupt unassigned"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(RADIO1_IRQ_SELW::_101)
    }
    #[doc = "Radio #1 Interrupt unassigned"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(RADIO1_IRQ_SELW::_110)
    }
    #[doc = "Radio #1 Interrupt unassigned"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(RADIO1_IRQ_SELW::_111)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Radio Protocol Selection"]
    #[inline]
    pub fn protocol(&self) -> PROTOCOLR {
        PROTOCOLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Target Power Source"]
    #[inline]
    pub fn tgt_pwr_src(&self) -> TGT_PWR_SRCR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TGT_PWR_SRCR { bits }
    }
    #[doc = "Bits 8:9 - Radio Reference Clock Frequency"]
    #[inline]
    pub fn ref_clk_freq(&self) -> REF_CLK_FREQR {
        REF_CLK_FREQR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - SOC_RF_OSC_CLK_GATE_EN"]
    #[inline]
    pub fn soc_rf_osc_clk_gate_en(&self) -> SOC_RF_OSC_CLK_GATE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_RF_OSC_CLK_GATE_ENR { bits }
    }
    #[doc = "Bits 12:13 - Demodulator Selector"]
    #[inline]
    pub fn demod_sel(&self) -> DEMOD_SELR {
        DEMOD_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - RADIO0_IRQ_SEL"]
    #[inline]
    pub fn radio0_irq_sel(&self) -> RADIO0_IRQ_SELR {
        RADIO0_IRQ_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - RADIO1_IRQ_SEL"]
    #[inline]
    pub fn radio1_irq_sel(&self) -> RADIO1_IRQ_SELR {
        RADIO1_IRQ_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1052672 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Radio Protocol Selection"]
    #[inline]
    pub fn protocol(&mut self) -> _PROTOCOLW {
        _PROTOCOLW { w: self }
    }
    #[doc = "Bits 4:6 - Target Power Source"]
    #[inline]
    pub fn tgt_pwr_src(&mut self) -> _TGT_PWR_SRCW {
        _TGT_PWR_SRCW { w: self }
    }
    #[doc = "Bits 8:9 - Radio Reference Clock Frequency"]
    #[inline]
    pub fn ref_clk_freq(&mut self) -> _REF_CLK_FREQW {
        _REF_CLK_FREQW { w: self }
    }
    #[doc = "Bit 11 - SOC_RF_OSC_CLK_GATE_EN"]
    #[inline]
    pub fn soc_rf_osc_clk_gate_en(&mut self) -> _SOC_RF_OSC_CLK_GATE_ENW {
        _SOC_RF_OSC_CLK_GATE_ENW { w: self }
    }
    #[doc = "Bits 12:13 - Demodulator Selector"]
    #[inline]
    pub fn demod_sel(&mut self) -> _DEMOD_SELW {
        _DEMOD_SELW { w: self }
    }
    #[doc = "Bits 16:18 - RADIO0_IRQ_SEL"]
    #[inline]
    pub fn radio0_irq_sel(&mut self) -> _RADIO0_IRQ_SELW {
        _RADIO0_IRQ_SELW { w: self }
    }
    #[doc = "Bits 20:22 - RADIO1_IRQ_SEL"]
    #[inline]
    pub fn radio1_irq_sel(&mut self) -> _RADIO1_IRQ_SELW {
        _RADIO1_IRQ_SELW { w: self }
    }
}
