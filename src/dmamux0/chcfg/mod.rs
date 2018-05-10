#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CHCFG {
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
#[doc = "Possible values of the field `SOURCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCER {
    #[doc = "Disable_Signal"]
    _0,
    #[doc = "LPUART0_Rx_Signal"]
    _2,
    #[doc = "LPUART0_Tx_Signal"]
    _3,
    #[doc = "SPI0_Rx_Signal"]
    _16,
    #[doc = "SPI0_Tx_Signal"]
    _17,
    #[doc = "SPI1_Rx_Signal"]
    _18,
    #[doc = "SPI1_Tx_Signal"]
    _19,
    #[doc = "LTC0_Input_FIFO_Signal"]
    _20,
    #[doc = "LTC0_Output_FIFO_Signal"]
    _21,
    #[doc = "I2C0_Signal"]
    _22,
    #[doc = "I2C1_Signal"]
    _23,
    #[doc = "TPM0_Channel0_Signal"]
    _24,
    #[doc = "TPM0_Channel1_Signal"]
    _25,
    #[doc = "TPM0_Channel2_Signal"]
    _26,
    #[doc = "TPM0_Channel3_Signal"]
    _27,
    #[doc = "TPM1_Channel0_Signal"]
    _32,
    #[doc = "TPM1_Channel1_Signal"]
    _33,
    #[doc = "TPM2_Channel0_Signal"]
    _34,
    #[doc = "TPM2_Channel1_Signal"]
    _35,
    #[doc = "ADC0_Signal"]
    _40,
    #[doc = "CMP0_Signal"]
    _42,
    #[doc = "DAC0_Signal"]
    _45,
    #[doc = "CMT_Signal"]
    _47,
    #[doc = "PortA_Signal"]
    _49,
    #[doc = "PortB_Signal"]
    _50,
    #[doc = "PortC_Signal"]
    _51,
    #[doc = "TPM0_Overflow_Signal"]
    _54,
    #[doc = "TPM1_Overflow_Signal"]
    _55,
    #[doc = "TPM2_Overflow_Signal"]
    _56,
    #[doc = "TSI0_Signal"]
    _57,
    #[doc = "AlwaysOn60_Signal"]
    _60,
    #[doc = "AlwaysOn61_Signal"]
    _61,
    #[doc = "AlwaysOn62_Signal"]
    _62,
    #[doc = "AlwaysOn63_Signal"]
    _63,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SOURCER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SOURCER::_0 => 0,
            SOURCER::_2 => 2,
            SOURCER::_3 => 3,
            SOURCER::_16 => 16,
            SOURCER::_17 => 17,
            SOURCER::_18 => 18,
            SOURCER::_19 => 19,
            SOURCER::_20 => 20,
            SOURCER::_21 => 21,
            SOURCER::_22 => 22,
            SOURCER::_23 => 23,
            SOURCER::_24 => 24,
            SOURCER::_25 => 25,
            SOURCER::_26 => 26,
            SOURCER::_27 => 27,
            SOURCER::_32 => 32,
            SOURCER::_33 => 33,
            SOURCER::_34 => 34,
            SOURCER::_35 => 35,
            SOURCER::_40 => 40,
            SOURCER::_42 => 42,
            SOURCER::_45 => 45,
            SOURCER::_47 => 47,
            SOURCER::_49 => 49,
            SOURCER::_50 => 50,
            SOURCER::_51 => 51,
            SOURCER::_54 => 54,
            SOURCER::_55 => 55,
            SOURCER::_56 => 56,
            SOURCER::_57 => 57,
            SOURCER::_60 => 60,
            SOURCER::_61 => 61,
            SOURCER::_62 => 62,
            SOURCER::_63 => 63,
            SOURCER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SOURCER {
        match value {
            0 => SOURCER::_0,
            2 => SOURCER::_2,
            3 => SOURCER::_3,
            16 => SOURCER::_16,
            17 => SOURCER::_17,
            18 => SOURCER::_18,
            19 => SOURCER::_19,
            20 => SOURCER::_20,
            21 => SOURCER::_21,
            22 => SOURCER::_22,
            23 => SOURCER::_23,
            24 => SOURCER::_24,
            25 => SOURCER::_25,
            26 => SOURCER::_26,
            27 => SOURCER::_27,
            32 => SOURCER::_32,
            33 => SOURCER::_33,
            34 => SOURCER::_34,
            35 => SOURCER::_35,
            40 => SOURCER::_40,
            42 => SOURCER::_42,
            45 => SOURCER::_45,
            47 => SOURCER::_47,
            49 => SOURCER::_49,
            50 => SOURCER::_50,
            51 => SOURCER::_51,
            54 => SOURCER::_54,
            55 => SOURCER::_55,
            56 => SOURCER::_56,
            57 => SOURCER::_57,
            60 => SOURCER::_60,
            61 => SOURCER::_61,
            62 => SOURCER::_62,
            63 => SOURCER::_63,
            i => SOURCER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SOURCER::_0
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == SOURCER::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == SOURCER::_3
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == SOURCER::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline]
    pub fn is_17(&self) -> bool {
        *self == SOURCER::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline]
    pub fn is_18(&self) -> bool {
        *self == SOURCER::_18
    }
    #[doc = "Checks if the value of the field is `_19`"]
    #[inline]
    pub fn is_19(&self) -> bool {
        *self == SOURCER::_19
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline]
    pub fn is_20(&self) -> bool {
        *self == SOURCER::_20
    }
    #[doc = "Checks if the value of the field is `_21`"]
    #[inline]
    pub fn is_21(&self) -> bool {
        *self == SOURCER::_21
    }
    #[doc = "Checks if the value of the field is `_22`"]
    #[inline]
    pub fn is_22(&self) -> bool {
        *self == SOURCER::_22
    }
    #[doc = "Checks if the value of the field is `_23`"]
    #[inline]
    pub fn is_23(&self) -> bool {
        *self == SOURCER::_23
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline]
    pub fn is_24(&self) -> bool {
        *self == SOURCER::_24
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline]
    pub fn is_25(&self) -> bool {
        *self == SOURCER::_25
    }
    #[doc = "Checks if the value of the field is `_26`"]
    #[inline]
    pub fn is_26(&self) -> bool {
        *self == SOURCER::_26
    }
    #[doc = "Checks if the value of the field is `_27`"]
    #[inline]
    pub fn is_27(&self) -> bool {
        *self == SOURCER::_27
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == SOURCER::_32
    }
    #[doc = "Checks if the value of the field is `_33`"]
    #[inline]
    pub fn is_33(&self) -> bool {
        *self == SOURCER::_33
    }
    #[doc = "Checks if the value of the field is `_34`"]
    #[inline]
    pub fn is_34(&self) -> bool {
        *self == SOURCER::_34
    }
    #[doc = "Checks if the value of the field is `_35`"]
    #[inline]
    pub fn is_35(&self) -> bool {
        *self == SOURCER::_35
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline]
    pub fn is_40(&self) -> bool {
        *self == SOURCER::_40
    }
    #[doc = "Checks if the value of the field is `_42`"]
    #[inline]
    pub fn is_42(&self) -> bool {
        *self == SOURCER::_42
    }
    #[doc = "Checks if the value of the field is `_45`"]
    #[inline]
    pub fn is_45(&self) -> bool {
        *self == SOURCER::_45
    }
    #[doc = "Checks if the value of the field is `_47`"]
    #[inline]
    pub fn is_47(&self) -> bool {
        *self == SOURCER::_47
    }
    #[doc = "Checks if the value of the field is `_49`"]
    #[inline]
    pub fn is_49(&self) -> bool {
        *self == SOURCER::_49
    }
    #[doc = "Checks if the value of the field is `_50`"]
    #[inline]
    pub fn is_50(&self) -> bool {
        *self == SOURCER::_50
    }
    #[doc = "Checks if the value of the field is `_51`"]
    #[inline]
    pub fn is_51(&self) -> bool {
        *self == SOURCER::_51
    }
    #[doc = "Checks if the value of the field is `_54`"]
    #[inline]
    pub fn is_54(&self) -> bool {
        *self == SOURCER::_54
    }
    #[doc = "Checks if the value of the field is `_55`"]
    #[inline]
    pub fn is_55(&self) -> bool {
        *self == SOURCER::_55
    }
    #[doc = "Checks if the value of the field is `_56`"]
    #[inline]
    pub fn is_56(&self) -> bool {
        *self == SOURCER::_56
    }
    #[doc = "Checks if the value of the field is `_57`"]
    #[inline]
    pub fn is_57(&self) -> bool {
        *self == SOURCER::_57
    }
    #[doc = "Checks if the value of the field is `_60`"]
    #[inline]
    pub fn is_60(&self) -> bool {
        *self == SOURCER::_60
    }
    #[doc = "Checks if the value of the field is `_61`"]
    #[inline]
    pub fn is_61(&self) -> bool {
        *self == SOURCER::_61
    }
    #[doc = "Checks if the value of the field is `_62`"]
    #[inline]
    pub fn is_62(&self) -> bool {
        *self == SOURCER::_62
    }
    #[doc = "Checks if the value of the field is `_63`"]
    #[inline]
    pub fn is_63(&self) -> bool {
        *self == SOURCER::_63
    }
}
#[doc = "Possible values of the field `TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGR {
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    _0,
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    _1,
}
impl TRIGR {
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
            TRIGR::_0 => false,
            TRIGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGR {
        match value {
            false => TRIGR::_0,
            true => TRIGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRIGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRIGR::_1
    }
}
#[doc = "Possible values of the field `ENBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENBLR {
    #[doc = "DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    _0,
    #[doc = "DMA channel is enabled"]
    _1,
}
impl ENBLR {
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
            ENBLR::_0 => false,
            ENBLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENBLR {
        match value {
            false => ENBLR::_0,
            true => ENBLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENBLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENBLR::_1
    }
}
#[doc = "Values that can be written to the field `SOURCE`"]
pub enum SOURCEW {
    #[doc = "Disable_Signal"]
    _0,
    #[doc = "LPUART0_Rx_Signal"]
    _2,
    #[doc = "LPUART0_Tx_Signal"]
    _3,
    #[doc = "SPI0_Rx_Signal"]
    _16,
    #[doc = "SPI0_Tx_Signal"]
    _17,
    #[doc = "SPI1_Rx_Signal"]
    _18,
    #[doc = "SPI1_Tx_Signal"]
    _19,
    #[doc = "LTC0_Input_FIFO_Signal"]
    _20,
    #[doc = "LTC0_Output_FIFO_Signal"]
    _21,
    #[doc = "I2C0_Signal"]
    _22,
    #[doc = "I2C1_Signal"]
    _23,
    #[doc = "TPM0_Channel0_Signal"]
    _24,
    #[doc = "TPM0_Channel1_Signal"]
    _25,
    #[doc = "TPM0_Channel2_Signal"]
    _26,
    #[doc = "TPM0_Channel3_Signal"]
    _27,
    #[doc = "TPM1_Channel0_Signal"]
    _32,
    #[doc = "TPM1_Channel1_Signal"]
    _33,
    #[doc = "TPM2_Channel0_Signal"]
    _34,
    #[doc = "TPM2_Channel1_Signal"]
    _35,
    #[doc = "ADC0_Signal"]
    _40,
    #[doc = "CMP0_Signal"]
    _42,
    #[doc = "DAC0_Signal"]
    _45,
    #[doc = "CMT_Signal"]
    _47,
    #[doc = "PortA_Signal"]
    _49,
    #[doc = "PortB_Signal"]
    _50,
    #[doc = "PortC_Signal"]
    _51,
    #[doc = "TPM0_Overflow_Signal"]
    _54,
    #[doc = "TPM1_Overflow_Signal"]
    _55,
    #[doc = "TPM2_Overflow_Signal"]
    _56,
    #[doc = "TSI0_Signal"]
    _57,
    #[doc = "AlwaysOn60_Signal"]
    _60,
    #[doc = "AlwaysOn61_Signal"]
    _61,
    #[doc = "AlwaysOn62_Signal"]
    _62,
    #[doc = "AlwaysOn63_Signal"]
    _63,
}
impl SOURCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SOURCEW::_0 => 0,
            SOURCEW::_2 => 2,
            SOURCEW::_3 => 3,
            SOURCEW::_16 => 16,
            SOURCEW::_17 => 17,
            SOURCEW::_18 => 18,
            SOURCEW::_19 => 19,
            SOURCEW::_20 => 20,
            SOURCEW::_21 => 21,
            SOURCEW::_22 => 22,
            SOURCEW::_23 => 23,
            SOURCEW::_24 => 24,
            SOURCEW::_25 => 25,
            SOURCEW::_26 => 26,
            SOURCEW::_27 => 27,
            SOURCEW::_32 => 32,
            SOURCEW::_33 => 33,
            SOURCEW::_34 => 34,
            SOURCEW::_35 => 35,
            SOURCEW::_40 => 40,
            SOURCEW::_42 => 42,
            SOURCEW::_45 => 45,
            SOURCEW::_47 => 47,
            SOURCEW::_49 => 49,
            SOURCEW::_50 => 50,
            SOURCEW::_51 => 51,
            SOURCEW::_54 => 54,
            SOURCEW::_55 => 55,
            SOURCEW::_56 => 56,
            SOURCEW::_57 => 57,
            SOURCEW::_60 => 60,
            SOURCEW::_61 => 61,
            SOURCEW::_62 => 62,
            SOURCEW::_63 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOURCEW<'a> {
    w: &'a mut W,
}
impl<'a> _SOURCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOURCEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable_Signal"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOURCEW::_0)
    }
    #[doc = "LPUART0_Rx_Signal"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(SOURCEW::_2)
    }
    #[doc = "LPUART0_Tx_Signal"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(SOURCEW::_3)
    }
    #[doc = "SPI0_Rx_Signal"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(SOURCEW::_16)
    }
    #[doc = "SPI0_Tx_Signal"]
    #[inline]
    pub fn _17(self) -> &'a mut W {
        self.variant(SOURCEW::_17)
    }
    #[doc = "SPI1_Rx_Signal"]
    #[inline]
    pub fn _18(self) -> &'a mut W {
        self.variant(SOURCEW::_18)
    }
    #[doc = "SPI1_Tx_Signal"]
    #[inline]
    pub fn _19(self) -> &'a mut W {
        self.variant(SOURCEW::_19)
    }
    #[doc = "LTC0_Input_FIFO_Signal"]
    #[inline]
    pub fn _20(self) -> &'a mut W {
        self.variant(SOURCEW::_20)
    }
    #[doc = "LTC0_Output_FIFO_Signal"]
    #[inline]
    pub fn _21(self) -> &'a mut W {
        self.variant(SOURCEW::_21)
    }
    #[doc = "I2C0_Signal"]
    #[inline]
    pub fn _22(self) -> &'a mut W {
        self.variant(SOURCEW::_22)
    }
    #[doc = "I2C1_Signal"]
    #[inline]
    pub fn _23(self) -> &'a mut W {
        self.variant(SOURCEW::_23)
    }
    #[doc = "TPM0_Channel0_Signal"]
    #[inline]
    pub fn _24(self) -> &'a mut W {
        self.variant(SOURCEW::_24)
    }
    #[doc = "TPM0_Channel1_Signal"]
    #[inline]
    pub fn _25(self) -> &'a mut W {
        self.variant(SOURCEW::_25)
    }
    #[doc = "TPM0_Channel2_Signal"]
    #[inline]
    pub fn _26(self) -> &'a mut W {
        self.variant(SOURCEW::_26)
    }
    #[doc = "TPM0_Channel3_Signal"]
    #[inline]
    pub fn _27(self) -> &'a mut W {
        self.variant(SOURCEW::_27)
    }
    #[doc = "TPM1_Channel0_Signal"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(SOURCEW::_32)
    }
    #[doc = "TPM1_Channel1_Signal"]
    #[inline]
    pub fn _33(self) -> &'a mut W {
        self.variant(SOURCEW::_33)
    }
    #[doc = "TPM2_Channel0_Signal"]
    #[inline]
    pub fn _34(self) -> &'a mut W {
        self.variant(SOURCEW::_34)
    }
    #[doc = "TPM2_Channel1_Signal"]
    #[inline]
    pub fn _35(self) -> &'a mut W {
        self.variant(SOURCEW::_35)
    }
    #[doc = "ADC0_Signal"]
    #[inline]
    pub fn _40(self) -> &'a mut W {
        self.variant(SOURCEW::_40)
    }
    #[doc = "CMP0_Signal"]
    #[inline]
    pub fn _42(self) -> &'a mut W {
        self.variant(SOURCEW::_42)
    }
    #[doc = "DAC0_Signal"]
    #[inline]
    pub fn _45(self) -> &'a mut W {
        self.variant(SOURCEW::_45)
    }
    #[doc = "CMT_Signal"]
    #[inline]
    pub fn _47(self) -> &'a mut W {
        self.variant(SOURCEW::_47)
    }
    #[doc = "PortA_Signal"]
    #[inline]
    pub fn _49(self) -> &'a mut W {
        self.variant(SOURCEW::_49)
    }
    #[doc = "PortB_Signal"]
    #[inline]
    pub fn _50(self) -> &'a mut W {
        self.variant(SOURCEW::_50)
    }
    #[doc = "PortC_Signal"]
    #[inline]
    pub fn _51(self) -> &'a mut W {
        self.variant(SOURCEW::_51)
    }
    #[doc = "TPM0_Overflow_Signal"]
    #[inline]
    pub fn _54(self) -> &'a mut W {
        self.variant(SOURCEW::_54)
    }
    #[doc = "TPM1_Overflow_Signal"]
    #[inline]
    pub fn _55(self) -> &'a mut W {
        self.variant(SOURCEW::_55)
    }
    #[doc = "TPM2_Overflow_Signal"]
    #[inline]
    pub fn _56(self) -> &'a mut W {
        self.variant(SOURCEW::_56)
    }
    #[doc = "TSI0_Signal"]
    #[inline]
    pub fn _57(self) -> &'a mut W {
        self.variant(SOURCEW::_57)
    }
    #[doc = "AlwaysOn60_Signal"]
    #[inline]
    pub fn _60(self) -> &'a mut W {
        self.variant(SOURCEW::_60)
    }
    #[doc = "AlwaysOn61_Signal"]
    #[inline]
    pub fn _61(self) -> &'a mut W {
        self.variant(SOURCEW::_61)
    }
    #[doc = "AlwaysOn62_Signal"]
    #[inline]
    pub fn _62(self) -> &'a mut W {
        self.variant(SOURCEW::_62)
    }
    #[doc = "AlwaysOn63_Signal"]
    #[inline]
    pub fn _63(self) -> &'a mut W {
        self.variant(SOURCEW::_63)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIG`"]
pub enum TRIGW {
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    _0,
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    _1,
}
impl TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGW::_0 => false,
            TRIGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIGW::_0)
    }
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIGW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENBL`"]
pub enum ENBLW {
    #[doc = "DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    _0,
    #[doc = "DMA channel is enabled"]
    _1,
}
impl ENBLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENBLW::_0 => false,
            ENBLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENBLW<'a> {
    w: &'a mut W,
}
impl<'a> _ENBLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENBLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENBLW::_0)
    }
    #[doc = "DMA channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENBLW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:5 - DMA Channel Source (Slot)"]
    #[inline]
    pub fn source(&self) -> SOURCER {
        SOURCER::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline]
    pub fn trig(&self) -> TRIGR {
        TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline]
    pub fn enbl(&self) -> ENBLR {
        ENBLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - DMA Channel Source (Slot)"]
    #[inline]
    pub fn source(&mut self) -> _SOURCEW {
        _SOURCEW { w: self }
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline]
    pub fn trig(&mut self) -> _TRIGW {
        _TRIGW { w: self }
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline]
    pub fn enbl(&mut self) -> _ENBLW {
        _ENBLW { w: self }
    }
}
