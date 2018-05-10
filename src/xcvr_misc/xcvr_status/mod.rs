#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XCVR_STATUS {
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
#[doc = r" Value of the field"]
pub struct TSM_COUNTR {
    bits: u8,
}
impl TSM_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PLL_SEQ_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_SEQ_STATER {
    #[doc = "PLL OFF"]
    _0,
    #[doc = "CTUNE"]
    _2,
    #[doc = "CTUNE_SETTLE"]
    _3,
    #[doc = "HPMCAL1"]
    _6,
    #[doc = "HPMCAL1_SETTLE"]
    _8,
    #[doc = "HPMCAL2"]
    _10,
    #[doc = "HPMCAL2_SETTLE"]
    _12,
    #[doc = "PLLREADY"]
    _15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLL_SEQ_STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLL_SEQ_STATER::_0 => 0,
            PLL_SEQ_STATER::_2 => 2,
            PLL_SEQ_STATER::_3 => 3,
            PLL_SEQ_STATER::_6 => 6,
            PLL_SEQ_STATER::_8 => 8,
            PLL_SEQ_STATER::_10 => 10,
            PLL_SEQ_STATER::_12 => 12,
            PLL_SEQ_STATER::_15 => 15,
            PLL_SEQ_STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLL_SEQ_STATER {
        match value {
            0 => PLL_SEQ_STATER::_0,
            2 => PLL_SEQ_STATER::_2,
            3 => PLL_SEQ_STATER::_3,
            6 => PLL_SEQ_STATER::_6,
            8 => PLL_SEQ_STATER::_8,
            10 => PLL_SEQ_STATER::_10,
            12 => PLL_SEQ_STATER::_12,
            15 => PLL_SEQ_STATER::_15,
            i => PLL_SEQ_STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLL_SEQ_STATER::_0
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == PLL_SEQ_STATER::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == PLL_SEQ_STATER::_3
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == PLL_SEQ_STATER::_6
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == PLL_SEQ_STATER::_8
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PLL_SEQ_STATER::_10
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline]
    pub fn is_12(&self) -> bool {
        *self == PLL_SEQ_STATER::_12
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline]
    pub fn is_15(&self) -> bool {
        *self == PLL_SEQ_STATER::_15
    }
}
#[doc = r" Value of the field"]
pub struct RX_MODER {
    bits: bool,
}
impl RX_MODER {
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
pub struct TX_MODER {
    bits: bool,
}
impl TX_MODER {
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
pub struct BTLE_SYSCLK_REQR {
    bits: bool,
}
impl BTLE_SYSCLK_REQR {
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
pub struct RIF_LL_ACTIVER {
    bits: bool,
}
impl RIF_LL_ACTIVER {
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
#[doc = "Possible values of the field `XTAL_READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_READYR {
    #[doc = "Indicates that the RF Oscillator is disabled or has not completed its warmup."]
    _0,
    #[doc = "Indicates that the RF Oscillator has completed its warmup count and is ready for use."]
    _1,
}
impl XTAL_READYR {
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
            XTAL_READYR::_0 => false,
            XTAL_READYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTAL_READYR {
        match value {
            false => XTAL_READYR::_0,
            true => XTAL_READYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == XTAL_READYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == XTAL_READYR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SOC_USING_RF_OSC_CLKR {
    bits: bool,
}
impl SOC_USING_RF_OSC_CLKR {
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
#[doc = "Possible values of the field `TSM_IRQ0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_IRQ0R {
    #[doc = "TSM Interrupt #0 is not asserted."]
    _0,
    #[doc = "TSM Interrupt #0 is asserted. Write '1' to this bit to clear it."]
    _1,
}
impl TSM_IRQ0R {
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
            TSM_IRQ0R::_0 => false,
            TSM_IRQ0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSM_IRQ0R {
        match value {
            false => TSM_IRQ0R::_0,
            true => TSM_IRQ0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSM_IRQ0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSM_IRQ0R::_1
    }
}
#[doc = "Possible values of the field `TSM_IRQ1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_IRQ1R {
    #[doc = "TSM Interrupt #1 is not asserted."]
    _0,
    #[doc = "TSM Interrupt #1 is asserted. Write '1' to this bit to clear it."]
    _1,
}
impl TSM_IRQ1R {
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
            TSM_IRQ1R::_0 => false,
            TSM_IRQ1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSM_IRQ1R {
        match value {
            false => TSM_IRQ1R::_0,
            true => TSM_IRQ1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSM_IRQ1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSM_IRQ1R::_1
    }
}
#[doc = "Values that can be written to the field `TSM_IRQ0`"]
pub enum TSM_IRQ0W {
    #[doc = "TSM Interrupt #0 is not asserted."]
    _0,
    #[doc = "TSM Interrupt #0 is asserted. Write '1' to this bit to clear it."]
    _1,
}
impl TSM_IRQ0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSM_IRQ0W::_0 => false,
            TSM_IRQ0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSM_IRQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_IRQ0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSM_IRQ0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TSM Interrupt #0 is not asserted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSM_IRQ0W::_0)
    }
    #[doc = "TSM Interrupt #0 is asserted. Write '1' to this bit to clear it."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSM_IRQ0W::_1)
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
#[doc = "Values that can be written to the field `TSM_IRQ1`"]
pub enum TSM_IRQ1W {
    #[doc = "TSM Interrupt #1 is not asserted."]
    _0,
    #[doc = "TSM Interrupt #1 is asserted. Write '1' to this bit to clear it."]
    _1,
}
impl TSM_IRQ1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSM_IRQ1W::_0 => false,
            TSM_IRQ1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSM_IRQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_IRQ1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSM_IRQ1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TSM Interrupt #1 is not asserted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSM_IRQ1W::_0)
    }
    #[doc = "TSM Interrupt #1 is asserted. Write '1' to this bit to clear it."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSM_IRQ1W::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - TSM_COUNT"]
    #[inline]
    pub fn tsm_count(&self) -> TSM_COUNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TSM_COUNTR { bits }
    }
    #[doc = "Bits 8:11 - PLL Sequence State"]
    #[inline]
    pub fn pll_seq_state(&self) -> PLL_SEQ_STATER {
        PLL_SEQ_STATER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Receive Mode"]
    #[inline]
    pub fn rx_mode(&self) -> RX_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_MODER { bits }
    }
    #[doc = "Bit 13 - Transmit Mode"]
    #[inline]
    pub fn tx_mode(&self) -> TX_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_MODER { bits }
    }
    #[doc = "Bit 16 - BTLE System Clock Request"]
    #[inline]
    pub fn btle_sysclk_req(&self) -> BTLE_SYSCLK_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BTLE_SYSCLK_REQR { bits }
    }
    #[doc = "Bit 17 - Link Layer Active Indication"]
    #[inline]
    pub fn rif_ll_active(&self) -> RIF_LL_ACTIVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RIF_LL_ACTIVER { bits }
    }
    #[doc = "Bit 18 - RF Osciallator Xtal Ready"]
    #[inline]
    pub fn xtal_ready(&self) -> XTAL_READYR {
        XTAL_READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - SOC Using RF Clock Indication"]
    #[inline]
    pub fn soc_using_rf_osc_clk(&self) -> SOC_USING_RF_OSC_CLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_USING_RF_OSC_CLKR { bits }
    }
    #[doc = "Bit 24 - TSM Interrupt #0"]
    #[inline]
    pub fn tsm_irq0(&self) -> TSM_IRQ0R {
        TSM_IRQ0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - TSM Interrupt #1"]
    #[inline]
    pub fn tsm_irq1(&self) -> TSM_IRQ1R {
        TSM_IRQ1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
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
    #[doc = "Bit 24 - TSM Interrupt #0"]
    #[inline]
    pub fn tsm_irq0(&mut self) -> _TSM_IRQ0W {
        _TSM_IRQ0W { w: self }
    }
    #[doc = "Bit 25 - TSM Interrupt #1"]
    #[inline]
    pub fn tsm_irq1(&mut self) -> _TSM_IRQ1W {
        _TSM_IRQ1W { w: self }
    }
}
