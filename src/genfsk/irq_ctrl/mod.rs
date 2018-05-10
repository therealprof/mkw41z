#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRQ_CTRL {
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
#[doc = "Possible values of the field `SEQ_END_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQ_END_IRQR {
    #[doc = "Sequence End Interrupt is not asserted."]
    _0,
    #[doc = "Sequence End Interrupt is asserted."]
    _1,
}
impl SEQ_END_IRQR {
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
            SEQ_END_IRQR::_0 => false,
            SEQ_END_IRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEQ_END_IRQR {
        match value {
            false => SEQ_END_IRQR::_0,
            true => SEQ_END_IRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SEQ_END_IRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SEQ_END_IRQR::_1
    }
}
#[doc = "Possible values of the field `TX_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_IRQR {
    #[doc = "TX Interrupt is not asserted."]
    _0,
    #[doc = "TX Interrupt is asserted."]
    _1,
}
impl TX_IRQR {
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
            TX_IRQR::_0 => false,
            TX_IRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_IRQR {
        match value {
            false => TX_IRQR::_0,
            true => TX_IRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_IRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_IRQR::_1
    }
}
#[doc = "Possible values of the field `RX_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_IRQR {
    #[doc = "RX Interrupt is not asserted."]
    _0,
    #[doc = "RX Interrupt is asserted."]
    _1,
}
impl RX_IRQR {
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
            RX_IRQR::_0 => false,
            RX_IRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_IRQR {
        match value {
            false => RX_IRQR::_0,
            true => RX_IRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_IRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_IRQR::_1
    }
}
#[doc = "Possible values of the field `NTW_ADR_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NTW_ADR_IRQR {
    #[doc = "Network Address Match Interrupt is not asserted."]
    _0,
    #[doc = "Network Address Match Interrupt is asserted."]
    _1,
}
impl NTW_ADR_IRQR {
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
            NTW_ADR_IRQR::_0 => false,
            NTW_ADR_IRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NTW_ADR_IRQR {
        match value {
            false => NTW_ADR_IRQR::_0,
            true => NTW_ADR_IRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NTW_ADR_IRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NTW_ADR_IRQR::_1
    }
}
#[doc = "Possible values of the field `T1_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T1_IRQR {
    #[doc = "Timer1 (T1) Compare Interrupt is not asserted."]
    _0,
    #[doc = "Timer1 (T1) Compare Interrupt is asserted."]
    _1,
}
impl T1_IRQR {
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
            T1_IRQR::_0 => false,
            T1_IRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> T1_IRQR {
        match value {
            false => T1_IRQR::_0,
            true => T1_IRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == T1_IRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == T1_IRQR::_1
    }
}
#[doc = "Possible values of the field `T2_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T2_IRQR {
    #[doc = "Timer2 (T2) Compare Interrupt is not asserted."]
    _0,
    #[doc = "Timer2 (T2) Compare Interrupt is asserted."]
    _1,
}
impl T2_IRQR {
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
            T2_IRQR::_0 => false,
            T2_IRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> T2_IRQR {
        match value {
            false => T2_IRQR::_0,
            true => T2_IRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == T2_IRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == T2_IRQR::_1
    }
}
#[doc = "Possible values of the field `PLL_UNLOCK_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_UNLOCK_IRQR {
    #[doc = "PLL Unlock Interrupt is not asserted."]
    _0,
    #[doc = "PLL Unlock Interrupt is asserted."]
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
#[doc = "Possible values of the field `WAKE_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_IRQR {
    #[doc = "Wake Interrupt is not asserted."]
    _0,
    #[doc = "Wake Interrupt is asserted."]
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
#[doc = "Possible values of the field `RX_WATERMARK_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_WATERMARK_IRQR {
    #[doc = "RX Watermark Interrupt is not asserted."]
    _0,
    #[doc = "RX Watermark Interrupt is asserted."]
    _1,
}
impl RX_WATERMARK_IRQR {
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
            RX_WATERMARK_IRQR::_0 => false,
            RX_WATERMARK_IRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_WATERMARK_IRQR {
        match value {
            false => RX_WATERMARK_IRQR::_0,
            true => RX_WATERMARK_IRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_WATERMARK_IRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_WATERMARK_IRQR::_1
    }
}
#[doc = "Possible values of the field `TSM_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_IRQR {
    #[doc = "TSM0_IRQ and TSM1_IRQ are both clear."]
    _0,
    #[doc = "Indicates TSM0_IRQ or TSM1_IRQ is set in XCVR_STATUS."]
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
#[doc = "Possible values of the field `SEQ_END_IRQ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQ_END_IRQ_ENR {
    #[doc = "Sequence End Interrupt is not enabled."]
    _0,
    #[doc = "Sequence End Interrupt is enabled."]
    _1,
}
impl SEQ_END_IRQ_ENR {
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
            SEQ_END_IRQ_ENR::_0 => false,
            SEQ_END_IRQ_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEQ_END_IRQ_ENR {
        match value {
            false => SEQ_END_IRQ_ENR::_0,
            true => SEQ_END_IRQ_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SEQ_END_IRQ_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SEQ_END_IRQ_ENR::_1
    }
}
#[doc = "Possible values of the field `TX_IRQ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_IRQ_ENR {
    #[doc = "TX Interrupt is not enabled."]
    _0,
    #[doc = "TX Interrupt is enabled."]
    _1,
}
impl TX_IRQ_ENR {
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
            TX_IRQ_ENR::_0 => false,
            TX_IRQ_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_IRQ_ENR {
        match value {
            false => TX_IRQ_ENR::_0,
            true => TX_IRQ_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_IRQ_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_IRQ_ENR::_1
    }
}
#[doc = "Possible values of the field `RX_IRQ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_IRQ_ENR {
    #[doc = "RX Interrupt is not enabled."]
    _0,
    #[doc = "RX Interrupt is enabled."]
    _1,
}
impl RX_IRQ_ENR {
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
            RX_IRQ_ENR::_0 => false,
            RX_IRQ_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_IRQ_ENR {
        match value {
            false => RX_IRQ_ENR::_0,
            true => RX_IRQ_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_IRQ_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_IRQ_ENR::_1
    }
}
#[doc = "Possible values of the field `NTW_ADR_IRQ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NTW_ADR_IRQ_ENR {
    #[doc = "Network Address Match Interrupt is not enabled."]
    _0,
    #[doc = "Network Address Match Interrupt is enabled."]
    _1,
}
impl NTW_ADR_IRQ_ENR {
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
            NTW_ADR_IRQ_ENR::_0 => false,
            NTW_ADR_IRQ_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NTW_ADR_IRQ_ENR {
        match value {
            false => NTW_ADR_IRQ_ENR::_0,
            true => NTW_ADR_IRQ_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NTW_ADR_IRQ_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NTW_ADR_IRQ_ENR::_1
    }
}
#[doc = "Possible values of the field `T1_IRQ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T1_IRQ_ENR {
    #[doc = "Timer1 (T1) Compare Interrupt is not enabled."]
    _0,
    #[doc = "Timer1 (T1) Compare Interrupt is enabled."]
    _1,
}
impl T1_IRQ_ENR {
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
            T1_IRQ_ENR::_0 => false,
            T1_IRQ_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> T1_IRQ_ENR {
        match value {
            false => T1_IRQ_ENR::_0,
            true => T1_IRQ_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == T1_IRQ_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == T1_IRQ_ENR::_1
    }
}
#[doc = "Possible values of the field `T2_IRQ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T2_IRQ_ENR {
    #[doc = "Timer1 (T2) Compare Interrupt is not enabled."]
    _0,
    #[doc = "Timer1 (T2) Compare Interrupt is enabled."]
    _1,
}
impl T2_IRQ_ENR {
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
            T2_IRQ_ENR::_0 => false,
            T2_IRQ_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> T2_IRQ_ENR {
        match value {
            false => T2_IRQ_ENR::_0,
            true => T2_IRQ_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == T2_IRQ_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == T2_IRQ_ENR::_1
    }
}
#[doc = "Possible values of the field `PLL_UNLOCK_IRQ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_UNLOCK_IRQ_ENR {
    #[doc = "PLL Unlock Interrupt is not enabled."]
    _0,
    #[doc = "PLL Unlock Interrupt is enabled."]
    _1,
}
impl PLL_UNLOCK_IRQ_ENR {
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
            PLL_UNLOCK_IRQ_ENR::_0 => false,
            PLL_UNLOCK_IRQ_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLL_UNLOCK_IRQ_ENR {
        match value {
            false => PLL_UNLOCK_IRQ_ENR::_0,
            true => PLL_UNLOCK_IRQ_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLL_UNLOCK_IRQ_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLL_UNLOCK_IRQ_ENR::_1
    }
}
#[doc = "Possible values of the field `WAKE_IRQ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_IRQ_ENR {
    #[doc = "Wake Interrupt is not enabled."]
    _0,
    #[doc = "Wake Interrupt is enabled."]
    _1,
}
impl WAKE_IRQ_ENR {
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
            WAKE_IRQ_ENR::_0 => false,
            WAKE_IRQ_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKE_IRQ_ENR {
        match value {
            false => WAKE_IRQ_ENR::_0,
            true => WAKE_IRQ_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WAKE_IRQ_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WAKE_IRQ_ENR::_1
    }
}
#[doc = "Possible values of the field `RX_WATERMARK_IRQ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_WATERMARK_IRQ_ENR {
    #[doc = "RX Watermark Interrupt is not enabled."]
    _0,
    #[doc = "RX Watermark Interrupt is enabled."]
    _1,
}
impl RX_WATERMARK_IRQ_ENR {
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
            RX_WATERMARK_IRQ_ENR::_0 => false,
            RX_WATERMARK_IRQ_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_WATERMARK_IRQ_ENR {
        match value {
            false => RX_WATERMARK_IRQ_ENR::_0,
            true => RX_WATERMARK_IRQ_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_WATERMARK_IRQ_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_WATERMARK_IRQ_ENR::_1
    }
}
#[doc = "Possible values of the field `TSM_IRQ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_IRQ_ENR {
    #[doc = "TSM Interrupt is not enabled."]
    _0,
    #[doc = "TSM Interrupt is enabled."]
    _1,
}
impl TSM_IRQ_ENR {
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
            TSM_IRQ_ENR::_0 => false,
            TSM_IRQ_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSM_IRQ_ENR {
        match value {
            false => TSM_IRQ_ENR::_0,
            true => TSM_IRQ_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSM_IRQ_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSM_IRQ_ENR::_1
    }
}
#[doc = "Possible values of the field `GENERIC_FSK_IRQ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENERIC_FSK_IRQ_ENR {
    #[doc = "All GENERIC_FSK Interrupts are disabled."]
    _0,
    #[doc = "All GENERIC_FSK Interrupts can be enabled."]
    _1,
}
impl GENERIC_FSK_IRQ_ENR {
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
            GENERIC_FSK_IRQ_ENR::_0 => false,
            GENERIC_FSK_IRQ_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GENERIC_FSK_IRQ_ENR {
        match value {
            false => GENERIC_FSK_IRQ_ENR::_0,
            true => GENERIC_FSK_IRQ_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GENERIC_FSK_IRQ_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GENERIC_FSK_IRQ_ENR::_1
    }
}
#[doc = "Possible values of the field `CRC_IGNORE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_IGNORER {
    #[doc = "RX_IRQ will not be asserted for a received packet which fails CRC verification."]
    _0,
    #[doc = "RX_IRQ will be asserted even for a received packet which fails CRC verification."]
    _1,
}
impl CRC_IGNORER {
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
            CRC_IGNORER::_0 => false,
            CRC_IGNORER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC_IGNORER {
        match value {
            false => CRC_IGNORER::_0,
            true => CRC_IGNORER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRC_IGNORER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRC_IGNORER::_1
    }
}
#[doc = "Possible values of the field `CRC_VALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_VALIDR {
    #[doc = "CRC of RX packet is not valid."]
    _0,
    #[doc = "CRC of RX packet is valid."]
    _1,
}
impl CRC_VALIDR {
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
            CRC_VALIDR::_0 => false,
            CRC_VALIDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC_VALIDR {
        match value {
            false => CRC_VALIDR::_0,
            true => CRC_VALIDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRC_VALIDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRC_VALIDR::_1
    }
}
#[doc = "Values that can be written to the field `SEQ_END_IRQ`"]
pub enum SEQ_END_IRQW {
    #[doc = "Sequence End Interrupt is not asserted."]
    _0,
    #[doc = "Sequence End Interrupt is asserted."]
    _1,
}
impl SEQ_END_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQ_END_IRQW::_0 => false,
            SEQ_END_IRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQ_END_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQ_END_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQ_END_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sequence End Interrupt is not asserted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEQ_END_IRQW::_0)
    }
    #[doc = "Sequence End Interrupt is asserted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEQ_END_IRQW::_1)
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
#[doc = "Values that can be written to the field `TX_IRQ`"]
pub enum TX_IRQW {
    #[doc = "TX Interrupt is not asserted."]
    _0,
    #[doc = "TX Interrupt is asserted."]
    _1,
}
impl TX_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_IRQW::_0 => false,
            TX_IRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TX Interrupt is not asserted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_IRQW::_0)
    }
    #[doc = "TX Interrupt is asserted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_IRQW::_1)
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
#[doc = "Values that can be written to the field `RX_IRQ`"]
pub enum RX_IRQW {
    #[doc = "RX Interrupt is not asserted."]
    _0,
    #[doc = "RX Interrupt is asserted."]
    _1,
}
impl RX_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_IRQW::_0 => false,
            RX_IRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX Interrupt is not asserted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_IRQW::_0)
    }
    #[doc = "RX Interrupt is asserted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_IRQW::_1)
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
#[doc = "Values that can be written to the field `NTW_ADR_IRQ`"]
pub enum NTW_ADR_IRQW {
    #[doc = "Network Address Match Interrupt is not asserted."]
    _0,
    #[doc = "Network Address Match Interrupt is asserted."]
    _1,
}
impl NTW_ADR_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NTW_ADR_IRQW::_0 => false,
            NTW_ADR_IRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NTW_ADR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _NTW_ADR_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NTW_ADR_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Network Address Match Interrupt is not asserted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NTW_ADR_IRQW::_0)
    }
    #[doc = "Network Address Match Interrupt is asserted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NTW_ADR_IRQW::_1)
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
#[doc = "Values that can be written to the field `T1_IRQ`"]
pub enum T1_IRQW {
    #[doc = "Timer1 (T1) Compare Interrupt is not asserted."]
    _0,
    #[doc = "Timer1 (T1) Compare Interrupt is asserted."]
    _1,
}
impl T1_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            T1_IRQW::_0 => false,
            T1_IRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _T1_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _T1_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: T1_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer1 (T1) Compare Interrupt is not asserted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(T1_IRQW::_0)
    }
    #[doc = "Timer1 (T1) Compare Interrupt is asserted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(T1_IRQW::_1)
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
#[doc = "Values that can be written to the field `T2_IRQ`"]
pub enum T2_IRQW {
    #[doc = "Timer2 (T2) Compare Interrupt is not asserted."]
    _0,
    #[doc = "Timer2 (T2) Compare Interrupt is asserted."]
    _1,
}
impl T2_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            T2_IRQW::_0 => false,
            T2_IRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _T2_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _T2_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: T2_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer2 (T2) Compare Interrupt is not asserted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(T2_IRQW::_0)
    }
    #[doc = "Timer2 (T2) Compare Interrupt is asserted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(T2_IRQW::_1)
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
    #[doc = "PLL Unlock Interrupt is not asserted."]
    _0,
    #[doc = "PLL Unlock Interrupt is asserted."]
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
    #[doc = "PLL Unlock Interrupt is not asserted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLL_UNLOCK_IRQW::_0)
    }
    #[doc = "PLL Unlock Interrupt is asserted."]
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
    #[doc = "Wake Interrupt is not asserted."]
    _0,
    #[doc = "Wake Interrupt is asserted."]
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
    #[doc = "Wake Interrupt is not asserted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKE_IRQW::_0)
    }
    #[doc = "Wake Interrupt is asserted."]
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_WATERMARK_IRQ`"]
pub enum RX_WATERMARK_IRQW {
    #[doc = "RX Watermark Interrupt is not asserted."]
    _0,
    #[doc = "RX Watermark Interrupt is asserted."]
    _1,
}
impl RX_WATERMARK_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_WATERMARK_IRQW::_0 => false,
            RX_WATERMARK_IRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_WATERMARK_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_WATERMARK_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_WATERMARK_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX Watermark Interrupt is not asserted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_WATERMARK_IRQW::_0)
    }
    #[doc = "RX Watermark Interrupt is asserted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_WATERMARK_IRQW::_1)
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
#[doc = "Values that can be written to the field `SEQ_END_IRQ_EN`"]
pub enum SEQ_END_IRQ_ENW {
    #[doc = "Sequence End Interrupt is not enabled."]
    _0,
    #[doc = "Sequence End Interrupt is enabled."]
    _1,
}
impl SEQ_END_IRQ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQ_END_IRQ_ENW::_0 => false,
            SEQ_END_IRQ_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQ_END_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQ_END_IRQ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQ_END_IRQ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sequence End Interrupt is not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEQ_END_IRQ_ENW::_0)
    }
    #[doc = "Sequence End Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEQ_END_IRQ_ENW::_1)
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
#[doc = "Values that can be written to the field `TX_IRQ_EN`"]
pub enum TX_IRQ_ENW {
    #[doc = "TX Interrupt is not enabled."]
    _0,
    #[doc = "TX Interrupt is enabled."]
    _1,
}
impl TX_IRQ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_IRQ_ENW::_0 => false,
            TX_IRQ_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_IRQ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_IRQ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TX Interrupt is not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_IRQ_ENW::_0)
    }
    #[doc = "TX Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_IRQ_ENW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_IRQ_EN`"]
pub enum RX_IRQ_ENW {
    #[doc = "RX Interrupt is not enabled."]
    _0,
    #[doc = "RX Interrupt is enabled."]
    _1,
}
impl RX_IRQ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_IRQ_ENW::_0 => false,
            RX_IRQ_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_IRQ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_IRQ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX Interrupt is not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_IRQ_ENW::_0)
    }
    #[doc = "RX Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_IRQ_ENW::_1)
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
#[doc = "Values that can be written to the field `NTW_ADR_IRQ_EN`"]
pub enum NTW_ADR_IRQ_ENW {
    #[doc = "Network Address Match Interrupt is not enabled."]
    _0,
    #[doc = "Network Address Match Interrupt is enabled."]
    _1,
}
impl NTW_ADR_IRQ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NTW_ADR_IRQ_ENW::_0 => false,
            NTW_ADR_IRQ_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NTW_ADR_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _NTW_ADR_IRQ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NTW_ADR_IRQ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Network Address Match Interrupt is not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NTW_ADR_IRQ_ENW::_0)
    }
    #[doc = "Network Address Match Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NTW_ADR_IRQ_ENW::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `T1_IRQ_EN`"]
pub enum T1_IRQ_ENW {
    #[doc = "Timer1 (T1) Compare Interrupt is not enabled."]
    _0,
    #[doc = "Timer1 (T1) Compare Interrupt is enabled."]
    _1,
}
impl T1_IRQ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            T1_IRQ_ENW::_0 => false,
            T1_IRQ_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _T1_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _T1_IRQ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: T1_IRQ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer1 (T1) Compare Interrupt is not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(T1_IRQ_ENW::_0)
    }
    #[doc = "Timer1 (T1) Compare Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(T1_IRQ_ENW::_1)
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
#[doc = "Values that can be written to the field `T2_IRQ_EN`"]
pub enum T2_IRQ_ENW {
    #[doc = "Timer1 (T2) Compare Interrupt is not enabled."]
    _0,
    #[doc = "Timer1 (T2) Compare Interrupt is enabled."]
    _1,
}
impl T2_IRQ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            T2_IRQ_ENW::_0 => false,
            T2_IRQ_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _T2_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _T2_IRQ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: T2_IRQ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer1 (T2) Compare Interrupt is not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(T2_IRQ_ENW::_0)
    }
    #[doc = "Timer1 (T2) Compare Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(T2_IRQ_ENW::_1)
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
#[doc = "Values that can be written to the field `PLL_UNLOCK_IRQ_EN`"]
pub enum PLL_UNLOCK_IRQ_ENW {
    #[doc = "PLL Unlock Interrupt is not enabled."]
    _0,
    #[doc = "PLL Unlock Interrupt is enabled."]
    _1,
}
impl PLL_UNLOCK_IRQ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLL_UNLOCK_IRQ_ENW::_0 => false,
            PLL_UNLOCK_IRQ_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL_UNLOCK_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_UNLOCK_IRQ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL_UNLOCK_IRQ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL Unlock Interrupt is not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLL_UNLOCK_IRQ_ENW::_0)
    }
    #[doc = "PLL Unlock Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLL_UNLOCK_IRQ_ENW::_1)
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
#[doc = "Values that can be written to the field `WAKE_IRQ_EN`"]
pub enum WAKE_IRQ_ENW {
    #[doc = "Wake Interrupt is not enabled."]
    _0,
    #[doc = "Wake Interrupt is enabled."]
    _1,
}
impl WAKE_IRQ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKE_IRQ_ENW::_0 => false,
            WAKE_IRQ_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKE_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKE_IRQ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKE_IRQ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake Interrupt is not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKE_IRQ_ENW::_0)
    }
    #[doc = "Wake Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKE_IRQ_ENW::_1)
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
#[doc = "Values that can be written to the field `RX_WATERMARK_IRQ_EN`"]
pub enum RX_WATERMARK_IRQ_ENW {
    #[doc = "RX Watermark Interrupt is not enabled."]
    _0,
    #[doc = "RX Watermark Interrupt is enabled."]
    _1,
}
impl RX_WATERMARK_IRQ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_WATERMARK_IRQ_ENW::_0 => false,
            RX_WATERMARK_IRQ_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_WATERMARK_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_WATERMARK_IRQ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_WATERMARK_IRQ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX Watermark Interrupt is not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_WATERMARK_IRQ_ENW::_0)
    }
    #[doc = "RX Watermark Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_WATERMARK_IRQ_ENW::_1)
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
#[doc = "Values that can be written to the field `TSM_IRQ_EN`"]
pub enum TSM_IRQ_ENW {
    #[doc = "TSM Interrupt is not enabled."]
    _0,
    #[doc = "TSM Interrupt is enabled."]
    _1,
}
impl TSM_IRQ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSM_IRQ_ENW::_0 => false,
            TSM_IRQ_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSM_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_IRQ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSM_IRQ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TSM Interrupt is not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSM_IRQ_ENW::_0)
    }
    #[doc = "TSM Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSM_IRQ_ENW::_1)
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
#[doc = "Values that can be written to the field `GENERIC_FSK_IRQ_EN`"]
pub enum GENERIC_FSK_IRQ_ENW {
    #[doc = "All GENERIC_FSK Interrupts are disabled."]
    _0,
    #[doc = "All GENERIC_FSK Interrupts can be enabled."]
    _1,
}
impl GENERIC_FSK_IRQ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GENERIC_FSK_IRQ_ENW::_0 => false,
            GENERIC_FSK_IRQ_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GENERIC_FSK_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _GENERIC_FSK_IRQ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GENERIC_FSK_IRQ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "All GENERIC_FSK Interrupts are disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GENERIC_FSK_IRQ_ENW::_0)
    }
    #[doc = "All GENERIC_FSK Interrupts can be enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GENERIC_FSK_IRQ_ENW::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRC_IGNORE`"]
pub enum CRC_IGNOREW {
    #[doc = "RX_IRQ will not be asserted for a received packet which fails CRC verification."]
    _0,
    #[doc = "RX_IRQ will be asserted even for a received packet which fails CRC verification."]
    _1,
}
impl CRC_IGNOREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRC_IGNOREW::_0 => false,
            CRC_IGNOREW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC_IGNOREW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_IGNOREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC_IGNOREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX_IRQ will not be asserted for a received packet which fails CRC verification."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_IGNOREW::_0)
    }
    #[doc = "RX_IRQ will be asserted even for a received packet which fails CRC verification."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_IGNOREW::_1)
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bit 0 - Sequence End Interrupt"]
    #[inline]
    pub fn seq_end_irq(&self) -> SEQ_END_IRQR {
        SEQ_END_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TX Interrupt"]
    #[inline]
    pub fn tx_irq(&self) -> TX_IRQR {
        TX_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RX Interrupt"]
    #[inline]
    pub fn rx_irq(&self) -> RX_IRQR {
        RX_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Network Address Match Interrupt"]
    #[inline]
    pub fn ntw_adr_irq(&self) -> NTW_ADR_IRQR {
        NTW_ADR_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Timer1 (T1) Compare Interrupt"]
    #[inline]
    pub fn t1_irq(&self) -> T1_IRQR {
        T1_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Timer2 (T2) Compare Interrupt"]
    #[inline]
    pub fn t2_irq(&self) -> T2_IRQR {
        T2_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PLL Unlock Interrupt"]
    #[inline]
    pub fn pll_unlock_irq(&self) -> PLL_UNLOCK_IRQR {
        PLL_UNLOCK_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Wake Interrrupt"]
    #[inline]
    pub fn wake_irq(&self) -> WAKE_IRQR {
        WAKE_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - RX Watermark Interrupt"]
    #[inline]
    pub fn rx_watermark_irq(&self) -> RX_WATERMARK_IRQR {
        RX_WATERMARK_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - TSM Interrupt"]
    #[inline]
    pub fn tsm_irq(&self) -> TSM_IRQR {
        TSM_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - SEQ_END_IRQ Enable"]
    #[inline]
    pub fn seq_end_irq_en(&self) -> SEQ_END_IRQ_ENR {
        SEQ_END_IRQ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - TX_IRQ Enable"]
    #[inline]
    pub fn tx_irq_en(&self) -> TX_IRQ_ENR {
        TX_IRQ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - RX_IRQ Enable"]
    #[inline]
    pub fn rx_irq_en(&self) -> RX_IRQ_ENR {
        RX_IRQ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - NTW_ADR_IRQ Enable"]
    #[inline]
    pub fn ntw_adr_irq_en(&self) -> NTW_ADR_IRQ_ENR {
        NTW_ADR_IRQ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - T1_IRQ Enable"]
    #[inline]
    pub fn t1_irq_en(&self) -> T1_IRQ_ENR {
        T1_IRQ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - T2_IRQ Enable"]
    #[inline]
    pub fn t2_irq_en(&self) -> T2_IRQ_ENR {
        T2_IRQ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - PLL_UNLOCK_IRQ Enable"]
    #[inline]
    pub fn pll_unlock_irq_en(&self) -> PLL_UNLOCK_IRQ_ENR {
        PLL_UNLOCK_IRQ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - WAKE_IRQ Enable"]
    #[inline]
    pub fn wake_irq_en(&self) -> WAKE_IRQ_ENR {
        WAKE_IRQ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - RX_WATERMARK_IRQ Enable"]
    #[inline]
    pub fn rx_watermark_irq_en(&self) -> RX_WATERMARK_IRQ_ENR {
        RX_WATERMARK_IRQ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - TSM_IRQ Enable"]
    #[inline]
    pub fn tsm_irq_en(&self) -> TSM_IRQ_ENR {
        TSM_IRQ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - GENERIC_FSK_IRQ Master Enable"]
    #[inline]
    pub fn generic_fsk_irq_en(&self) -> GENERIC_FSK_IRQ_ENR {
        GENERIC_FSK_IRQ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - CRC Ignore"]
    #[inline]
    pub fn crc_ignore(&self) -> CRC_IGNORER {
        CRC_IGNORER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - CRC Valid"]
    #[inline]
    pub fn crc_valid(&self) -> CRC_VALIDR {
        CRC_VALIDR::_from({
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
    #[doc = "Bit 0 - Sequence End Interrupt"]
    #[inline]
    pub fn seq_end_irq(&mut self) -> _SEQ_END_IRQW {
        _SEQ_END_IRQW { w: self }
    }
    #[doc = "Bit 1 - TX Interrupt"]
    #[inline]
    pub fn tx_irq(&mut self) -> _TX_IRQW {
        _TX_IRQW { w: self }
    }
    #[doc = "Bit 2 - RX Interrupt"]
    #[inline]
    pub fn rx_irq(&mut self) -> _RX_IRQW {
        _RX_IRQW { w: self }
    }
    #[doc = "Bit 3 - Network Address Match Interrupt"]
    #[inline]
    pub fn ntw_adr_irq(&mut self) -> _NTW_ADR_IRQW {
        _NTW_ADR_IRQW { w: self }
    }
    #[doc = "Bit 4 - Timer1 (T1) Compare Interrupt"]
    #[inline]
    pub fn t1_irq(&mut self) -> _T1_IRQW {
        _T1_IRQW { w: self }
    }
    #[doc = "Bit 5 - Timer2 (T2) Compare Interrupt"]
    #[inline]
    pub fn t2_irq(&mut self) -> _T2_IRQW {
        _T2_IRQW { w: self }
    }
    #[doc = "Bit 6 - PLL Unlock Interrupt"]
    #[inline]
    pub fn pll_unlock_irq(&mut self) -> _PLL_UNLOCK_IRQW {
        _PLL_UNLOCK_IRQW { w: self }
    }
    #[doc = "Bit 7 - Wake Interrrupt"]
    #[inline]
    pub fn wake_irq(&mut self) -> _WAKE_IRQW {
        _WAKE_IRQW { w: self }
    }
    #[doc = "Bit 8 - RX Watermark Interrupt"]
    #[inline]
    pub fn rx_watermark_irq(&mut self) -> _RX_WATERMARK_IRQW {
        _RX_WATERMARK_IRQW { w: self }
    }
    #[doc = "Bit 16 - SEQ_END_IRQ Enable"]
    #[inline]
    pub fn seq_end_irq_en(&mut self) -> _SEQ_END_IRQ_ENW {
        _SEQ_END_IRQ_ENW { w: self }
    }
    #[doc = "Bit 17 - TX_IRQ Enable"]
    #[inline]
    pub fn tx_irq_en(&mut self) -> _TX_IRQ_ENW {
        _TX_IRQ_ENW { w: self }
    }
    #[doc = "Bit 18 - RX_IRQ Enable"]
    #[inline]
    pub fn rx_irq_en(&mut self) -> _RX_IRQ_ENW {
        _RX_IRQ_ENW { w: self }
    }
    #[doc = "Bit 19 - NTW_ADR_IRQ Enable"]
    #[inline]
    pub fn ntw_adr_irq_en(&mut self) -> _NTW_ADR_IRQ_ENW {
        _NTW_ADR_IRQ_ENW { w: self }
    }
    #[doc = "Bit 20 - T1_IRQ Enable"]
    #[inline]
    pub fn t1_irq_en(&mut self) -> _T1_IRQ_ENW {
        _T1_IRQ_ENW { w: self }
    }
    #[doc = "Bit 21 - T2_IRQ Enable"]
    #[inline]
    pub fn t2_irq_en(&mut self) -> _T2_IRQ_ENW {
        _T2_IRQ_ENW { w: self }
    }
    #[doc = "Bit 22 - PLL_UNLOCK_IRQ Enable"]
    #[inline]
    pub fn pll_unlock_irq_en(&mut self) -> _PLL_UNLOCK_IRQ_ENW {
        _PLL_UNLOCK_IRQ_ENW { w: self }
    }
    #[doc = "Bit 23 - WAKE_IRQ Enable"]
    #[inline]
    pub fn wake_irq_en(&mut self) -> _WAKE_IRQ_ENW {
        _WAKE_IRQ_ENW { w: self }
    }
    #[doc = "Bit 24 - RX_WATERMARK_IRQ Enable"]
    #[inline]
    pub fn rx_watermark_irq_en(&mut self) -> _RX_WATERMARK_IRQ_ENW {
        _RX_WATERMARK_IRQ_ENW { w: self }
    }
    #[doc = "Bit 25 - TSM_IRQ Enable"]
    #[inline]
    pub fn tsm_irq_en(&mut self) -> _TSM_IRQ_ENW {
        _TSM_IRQ_ENW { w: self }
    }
    #[doc = "Bit 26 - GENERIC_FSK_IRQ Master Enable"]
    #[inline]
    pub fn generic_fsk_irq_en(&mut self) -> _GENERIC_FSK_IRQ_ENW {
        _GENERIC_FSK_IRQ_ENW { w: self }
    }
    #[doc = "Bit 27 - CRC Ignore"]
    #[inline]
    pub fn crc_ignore(&mut self) -> _CRC_IGNOREW {
        _CRC_IGNOREW { w: self }
    }
}
