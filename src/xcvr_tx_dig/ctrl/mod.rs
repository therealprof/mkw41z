#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `RADIO_DFT_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RADIO_DFT_MODER {
    #[doc = "Normal Radio Operation, DFT not engaged."]
    _0000,
    #[doc = "Carrier Frequency Only"]
    _0001,
    #[doc = "Pattern Register GFSK"]
    _0010,
    #[doc = "LFSR GFSK"]
    _0011,
    #[doc = "Pattern Register FSK"]
    _0100,
    #[doc = "LFSR FSK"]
    _0101,
    #[doc = "Pattern Register O-QPSK"]
    _0110,
    #[doc = "LFSR O-QPSK"]
    _0111,
    #[doc = "LFSR 802.15.4 Symbols"]
    _1000,
    #[doc = "PLL Modulation from RAM"]
    _1001,
    #[doc = "PLL Coarse Tune BIST"]
    _1010,
    #[doc = "PLL Frequency Synthesizer BIST"]
    _1011,
    #[doc = "High Port DAC BIST"]
    _1100,
    #[doc = "VCO Frequency Meter"]
    _1101,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RADIO_DFT_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RADIO_DFT_MODER::_0000 => 0,
            RADIO_DFT_MODER::_0001 => 1,
            RADIO_DFT_MODER::_0010 => 2,
            RADIO_DFT_MODER::_0011 => 3,
            RADIO_DFT_MODER::_0100 => 4,
            RADIO_DFT_MODER::_0101 => 5,
            RADIO_DFT_MODER::_0110 => 6,
            RADIO_DFT_MODER::_0111 => 7,
            RADIO_DFT_MODER::_1000 => 8,
            RADIO_DFT_MODER::_1001 => 9,
            RADIO_DFT_MODER::_1010 => 10,
            RADIO_DFT_MODER::_1011 => 11,
            RADIO_DFT_MODER::_1100 => 12,
            RADIO_DFT_MODER::_1101 => 13,
            RADIO_DFT_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RADIO_DFT_MODER {
        match value {
            0 => RADIO_DFT_MODER::_0000,
            1 => RADIO_DFT_MODER::_0001,
            2 => RADIO_DFT_MODER::_0010,
            3 => RADIO_DFT_MODER::_0011,
            4 => RADIO_DFT_MODER::_0100,
            5 => RADIO_DFT_MODER::_0101,
            6 => RADIO_DFT_MODER::_0110,
            7 => RADIO_DFT_MODER::_0111,
            8 => RADIO_DFT_MODER::_1000,
            9 => RADIO_DFT_MODER::_1001,
            10 => RADIO_DFT_MODER::_1010,
            11 => RADIO_DFT_MODER::_1011,
            12 => RADIO_DFT_MODER::_1100,
            13 => RADIO_DFT_MODER::_1101,
            i => RADIO_DFT_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == RADIO_DFT_MODER::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == RADIO_DFT_MODER::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == RADIO_DFT_MODER::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == RADIO_DFT_MODER::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == RADIO_DFT_MODER::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == RADIO_DFT_MODER::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == RADIO_DFT_MODER::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == RADIO_DFT_MODER::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == RADIO_DFT_MODER::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == RADIO_DFT_MODER::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == RADIO_DFT_MODER::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == RADIO_DFT_MODER::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == RADIO_DFT_MODER::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == RADIO_DFT_MODER::_1101
    }
}
#[doc = "Possible values of the field `LFSR_LENGTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFSR_LENGTHR {
    #[doc = "LFSR 9, tap mask 100010000"]
    _000,
    #[doc = "LFSR 10, tap mask 1001000000"]
    _001,
    #[doc = "LFSR 11, tap mask 11101000000"]
    _010,
    #[doc = "LFSR 13, tap mask 1101100000000"]
    _011,
    #[doc = "LFSR 15, tap mask 111010000000000"]
    _100,
    #[doc = "LFSR 17, tap mask 11110000000000000"]
    _101,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LFSR_LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LFSR_LENGTHR::_000 => 0,
            LFSR_LENGTHR::_001 => 1,
            LFSR_LENGTHR::_010 => 2,
            LFSR_LENGTHR::_011 => 3,
            LFSR_LENGTHR::_100 => 4,
            LFSR_LENGTHR::_101 => 5,
            LFSR_LENGTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LFSR_LENGTHR {
        match value {
            0 => LFSR_LENGTHR::_000,
            1 => LFSR_LENGTHR::_001,
            2 => LFSR_LENGTHR::_010,
            3 => LFSR_LENGTHR::_011,
            4 => LFSR_LENGTHR::_100,
            5 => LFSR_LENGTHR::_101,
            i => LFSR_LENGTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == LFSR_LENGTHR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == LFSR_LENGTHR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == LFSR_LENGTHR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == LFSR_LENGTHR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == LFSR_LENGTHR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == LFSR_LENGTHR::_101
    }
}
#[doc = r" Value of the field"]
pub struct LFSR_ENR {
    bits: bool,
}
impl LFSR_ENR {
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
#[doc = "Possible values of the field `DFT_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFT_CLK_SELR {
    #[doc = "62.5 kHz"]
    _000,
    #[doc = "125 kHz"]
    _001,
    #[doc = "250 kHz"]
    _010,
    #[doc = "500 kHz"]
    _011,
    #[doc = "1 MHz"]
    _100,
    #[doc = "2 MHz"]
    _101,
    #[doc = "4 MHz"]
    _110,
    #[doc = "RF OSC Clock"]
    _111,
}
impl DFT_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DFT_CLK_SELR::_000 => 0,
            DFT_CLK_SELR::_001 => 1,
            DFT_CLK_SELR::_010 => 2,
            DFT_CLK_SELR::_011 => 3,
            DFT_CLK_SELR::_100 => 4,
            DFT_CLK_SELR::_101 => 5,
            DFT_CLK_SELR::_110 => 6,
            DFT_CLK_SELR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DFT_CLK_SELR {
        match value {
            0 => DFT_CLK_SELR::_000,
            1 => DFT_CLK_SELR::_001,
            2 => DFT_CLK_SELR::_010,
            3 => DFT_CLK_SELR::_011,
            4 => DFT_CLK_SELR::_100,
            5 => DFT_CLK_SELR::_101,
            6 => DFT_CLK_SELR::_110,
            7 => DFT_CLK_SELR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == DFT_CLK_SELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == DFT_CLK_SELR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == DFT_CLK_SELR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == DFT_CLK_SELR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == DFT_CLK_SELR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == DFT_CLK_SELR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == DFT_CLK_SELR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == DFT_CLK_SELR::_111
    }
}
#[doc = r" Value of the field"]
pub struct TX_DFT_ENR {
    bits: bool,
}
impl TX_DFT_ENR {
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
#[doc = "Possible values of the field `SOC_TEST_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOC_TEST_SELR {
    #[doc = "No Clock Selected"]
    _00,
    #[doc = "PLL Sigma Delta Clock, divided by 2"]
    _01,
    #[doc = "Auxiliary PLL Clock, divided by 2"]
    _10,
    #[doc = "RF Ref Osc clock, divided by 2"]
    _11,
}
impl SOC_TEST_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SOC_TEST_SELR::_00 => 0,
            SOC_TEST_SELR::_01 => 1,
            SOC_TEST_SELR::_10 => 2,
            SOC_TEST_SELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SOC_TEST_SELR {
        match value {
            0 => SOC_TEST_SELR::_00,
            1 => SOC_TEST_SELR::_01,
            2 => SOC_TEST_SELR::_10,
            3 => SOC_TEST_SELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == SOC_TEST_SELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == SOC_TEST_SELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SOC_TEST_SELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == SOC_TEST_SELR::_11
    }
}
#[doc = r" Value of the field"]
pub struct TX_CAPTURE_POLR {
    bits: bool,
}
impl TX_CAPTURE_POLR {
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
pub struct FREQ_WORD_ADJR {
    bits: u16,
}
impl FREQ_WORD_ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `RADIO_DFT_MODE`"]
pub enum RADIO_DFT_MODEW {
    #[doc = "Normal Radio Operation, DFT not engaged."]
    _0000,
    #[doc = "Carrier Frequency Only"]
    _0001,
    #[doc = "Pattern Register GFSK"]
    _0010,
    #[doc = "LFSR GFSK"]
    _0011,
    #[doc = "Pattern Register FSK"]
    _0100,
    #[doc = "LFSR FSK"]
    _0101,
    #[doc = "Pattern Register O-QPSK"]
    _0110,
    #[doc = "LFSR O-QPSK"]
    _0111,
    #[doc = "LFSR 802.15.4 Symbols"]
    _1000,
    #[doc = "PLL Modulation from RAM"]
    _1001,
    #[doc = "PLL Coarse Tune BIST"]
    _1010,
    #[doc = "PLL Frequency Synthesizer BIST"]
    _1011,
    #[doc = "High Port DAC BIST"]
    _1100,
    #[doc = "VCO Frequency Meter"]
    _1101,
}
impl RADIO_DFT_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RADIO_DFT_MODEW::_0000 => 0,
            RADIO_DFT_MODEW::_0001 => 1,
            RADIO_DFT_MODEW::_0010 => 2,
            RADIO_DFT_MODEW::_0011 => 3,
            RADIO_DFT_MODEW::_0100 => 4,
            RADIO_DFT_MODEW::_0101 => 5,
            RADIO_DFT_MODEW::_0110 => 6,
            RADIO_DFT_MODEW::_0111 => 7,
            RADIO_DFT_MODEW::_1000 => 8,
            RADIO_DFT_MODEW::_1001 => 9,
            RADIO_DFT_MODEW::_1010 => 10,
            RADIO_DFT_MODEW::_1011 => 11,
            RADIO_DFT_MODEW::_1100 => 12,
            RADIO_DFT_MODEW::_1101 => 13,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RADIO_DFT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _RADIO_DFT_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RADIO_DFT_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal Radio Operation, DFT not engaged."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_0000)
    }
    #[doc = "Carrier Frequency Only"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_0001)
    }
    #[doc = "Pattern Register GFSK"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_0010)
    }
    #[doc = "LFSR GFSK"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_0011)
    }
    #[doc = "Pattern Register FSK"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_0100)
    }
    #[doc = "LFSR FSK"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_0101)
    }
    #[doc = "Pattern Register O-QPSK"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_0110)
    }
    #[doc = "LFSR O-QPSK"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_0111)
    }
    #[doc = "LFSR 802.15.4 Symbols"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_1000)
    }
    #[doc = "PLL Modulation from RAM"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_1001)
    }
    #[doc = "PLL Coarse Tune BIST"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_1010)
    }
    #[doc = "PLL Frequency Synthesizer BIST"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_1011)
    }
    #[doc = "High Port DAC BIST"]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_1100)
    }
    #[doc = "VCO Frequency Meter"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(RADIO_DFT_MODEW::_1101)
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
#[doc = "Values that can be written to the field `LFSR_LENGTH`"]
pub enum LFSR_LENGTHW {
    #[doc = "LFSR 9, tap mask 100010000"]
    _000,
    #[doc = "LFSR 10, tap mask 1001000000"]
    _001,
    #[doc = "LFSR 11, tap mask 11101000000"]
    _010,
    #[doc = "LFSR 13, tap mask 1101100000000"]
    _011,
    #[doc = "LFSR 15, tap mask 111010000000000"]
    _100,
    #[doc = "LFSR 17, tap mask 11110000000000000"]
    _101,
}
impl LFSR_LENGTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LFSR_LENGTHW::_000 => 0,
            LFSR_LENGTHW::_001 => 1,
            LFSR_LENGTHW::_010 => 2,
            LFSR_LENGTHW::_011 => 3,
            LFSR_LENGTHW::_100 => 4,
            LFSR_LENGTHW::_101 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFSR_LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _LFSR_LENGTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFSR_LENGTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LFSR 9, tap mask 100010000"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(LFSR_LENGTHW::_000)
    }
    #[doc = "LFSR 10, tap mask 1001000000"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(LFSR_LENGTHW::_001)
    }
    #[doc = "LFSR 11, tap mask 11101000000"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(LFSR_LENGTHW::_010)
    }
    #[doc = "LFSR 13, tap mask 1101100000000"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(LFSR_LENGTHW::_011)
    }
    #[doc = "LFSR 15, tap mask 111010000000000"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(LFSR_LENGTHW::_100)
    }
    #[doc = "LFSR 17, tap mask 11110000000000000"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(LFSR_LENGTHW::_101)
    }
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
#[doc = r" Proxy"]
pub struct _LFSR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LFSR_ENW<'a> {
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
#[doc = "Values that can be written to the field `DFT_CLK_SEL`"]
pub enum DFT_CLK_SELW {
    #[doc = "62.5 kHz"]
    _000,
    #[doc = "125 kHz"]
    _001,
    #[doc = "250 kHz"]
    _010,
    #[doc = "500 kHz"]
    _011,
    #[doc = "1 MHz"]
    _100,
    #[doc = "2 MHz"]
    _101,
    #[doc = "4 MHz"]
    _110,
    #[doc = "RF OSC Clock"]
    _111,
}
impl DFT_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DFT_CLK_SELW::_000 => 0,
            DFT_CLK_SELW::_001 => 1,
            DFT_CLK_SELW::_010 => 2,
            DFT_CLK_SELW::_011 => 3,
            DFT_CLK_SELW::_100 => 4,
            DFT_CLK_SELW::_101 => 5,
            DFT_CLK_SELW::_110 => 6,
            DFT_CLK_SELW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFT_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _DFT_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFT_CLK_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "62.5 kHz"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(DFT_CLK_SELW::_000)
    }
    #[doc = "125 kHz"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(DFT_CLK_SELW::_001)
    }
    #[doc = "250 kHz"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(DFT_CLK_SELW::_010)
    }
    #[doc = "500 kHz"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(DFT_CLK_SELW::_011)
    }
    #[doc = "1 MHz"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(DFT_CLK_SELW::_100)
    }
    #[doc = "2 MHz"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(DFT_CLK_SELW::_101)
    }
    #[doc = "4 MHz"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(DFT_CLK_SELW::_110)
    }
    #[doc = "RF OSC Clock"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(DFT_CLK_SELW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX_DFT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DFT_ENW<'a> {
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
#[doc = "Values that can be written to the field `SOC_TEST_SEL`"]
pub enum SOC_TEST_SELW {
    #[doc = "No Clock Selected"]
    _00,
    #[doc = "PLL Sigma Delta Clock, divided by 2"]
    _01,
    #[doc = "Auxiliary PLL Clock, divided by 2"]
    _10,
    #[doc = "RF Ref Osc clock, divided by 2"]
    _11,
}
impl SOC_TEST_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SOC_TEST_SELW::_00 => 0,
            SOC_TEST_SELW::_01 => 1,
            SOC_TEST_SELW::_10 => 2,
            SOC_TEST_SELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOC_TEST_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_TEST_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOC_TEST_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No Clock Selected"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(SOC_TEST_SELW::_00)
    }
    #[doc = "PLL Sigma Delta Clock, divided by 2"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(SOC_TEST_SELW::_01)
    }
    #[doc = "Auxiliary PLL Clock, divided by 2"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(SOC_TEST_SELW::_10)
    }
    #[doc = "RF Ref Osc clock, divided by 2"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(SOC_TEST_SELW::_11)
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
#[doc = r" Proxy"]
pub struct _TX_CAPTURE_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_CAPTURE_POLW<'a> {
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
pub struct _FREQ_WORD_ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQ_WORD_ADJW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:3 - Radio DFT Modes"]
    #[inline]
    pub fn radio_dft_mode(&self) -> RADIO_DFT_MODER {
        RADIO_DFT_MODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - LFSR Length"]
    #[inline]
    pub fn lfsr_length(&self) -> LFSR_LENGTHR {
        LFSR_LENGTHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - LFSR Enable"]
    #[inline]
    pub fn lfsr_en(&self) -> LFSR_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LFSR_ENR { bits }
    }
    #[doc = "Bits 8:10 - DFT Clock Selection"]
    #[inline]
    pub fn dft_clk_sel(&self) -> DFT_CLK_SELR {
        DFT_CLK_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - DFT Modulation Enable"]
    #[inline]
    pub fn tx_dft_en(&self) -> TX_DFT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_DFT_ENR { bits }
    }
    #[doc = "Bits 12:13 - Radio Clock Selector for SoC RF Clock Tests"]
    #[inline]
    pub fn soc_test_sel(&self) -> SOC_TEST_SELR {
        SOC_TEST_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Polarity of the Input Data for the Transmitter"]
    #[inline]
    pub fn tx_capture_pol(&self) -> TX_CAPTURE_POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_CAPTURE_POLR { bits }
    }
    #[doc = "Bits 22:31 - Frequency Word Adjustment"]
    #[inline]
    pub fn freq_word_adj(&self) -> FREQ_WORD_ADJR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FREQ_WORD_ADJR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 320 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Radio DFT Modes"]
    #[inline]
    pub fn radio_dft_mode(&mut self) -> _RADIO_DFT_MODEW {
        _RADIO_DFT_MODEW { w: self }
    }
    #[doc = "Bits 4:6 - LFSR Length"]
    #[inline]
    pub fn lfsr_length(&mut self) -> _LFSR_LENGTHW {
        _LFSR_LENGTHW { w: self }
    }
    #[doc = "Bit 7 - LFSR Enable"]
    #[inline]
    pub fn lfsr_en(&mut self) -> _LFSR_ENW {
        _LFSR_ENW { w: self }
    }
    #[doc = "Bits 8:10 - DFT Clock Selection"]
    #[inline]
    pub fn dft_clk_sel(&mut self) -> _DFT_CLK_SELW {
        _DFT_CLK_SELW { w: self }
    }
    #[doc = "Bit 11 - DFT Modulation Enable"]
    #[inline]
    pub fn tx_dft_en(&mut self) -> _TX_DFT_ENW {
        _TX_DFT_ENW { w: self }
    }
    #[doc = "Bits 12:13 - Radio Clock Selector for SoC RF Clock Tests"]
    #[inline]
    pub fn soc_test_sel(&mut self) -> _SOC_TEST_SELW {
        _SOC_TEST_SELW { w: self }
    }
    #[doc = "Bit 16 - Polarity of the Input Data for the Transmitter"]
    #[inline]
    pub fn tx_capture_pol(&mut self) -> _TX_CAPTURE_POLW {
        _TX_CAPTURE_POLW { w: self }
    }
    #[doc = "Bits 22:31 - Frequency Word Adjustment"]
    #[inline]
    pub fn freq_word_adj(&mut self) -> _FREQ_WORD_ADJW {
        _FREQ_WORD_ADJW { w: self }
    }
}
