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
#[doc = "Possible values of the field `FORCE_TX_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_TX_ENR {
    #[doc = "TSM Idle"]
    _0,
    #[doc = "TSM executes a TX sequence"]
    _1,
}
impl FORCE_TX_ENR {
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
            FORCE_TX_ENR::_0 => false,
            FORCE_TX_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCE_TX_ENR {
        match value {
            false => FORCE_TX_ENR::_0,
            true => FORCE_TX_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FORCE_TX_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FORCE_TX_ENR::_1
    }
}
#[doc = "Possible values of the field `FORCE_RX_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_RX_ENR {
    #[doc = "TSM Idle"]
    _0,
    #[doc = "TSM executes a RX sequence"]
    _1,
}
impl FORCE_RX_ENR {
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
            FORCE_RX_ENR::_0 => false,
            FORCE_RX_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCE_RX_ENR {
        match value {
            false => FORCE_RX_ENR::_0,
            true => FORCE_RX_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FORCE_RX_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FORCE_RX_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct PA_RAMP_SELR {
    bits: u8,
}
impl PA_RAMP_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DATA_PADDING_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_PADDING_ENR {
    #[doc = "Disable TX Data Padding"]
    _00,
    #[doc = "Enable TX Data Padding"]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATA_PADDING_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATA_PADDING_ENR::_00 => 0,
            DATA_PADDING_ENR::_01 => 1,
            DATA_PADDING_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATA_PADDING_ENR {
        match value {
            0 => DATA_PADDING_ENR::_00,
            1 => DATA_PADDING_ENR::_01,
            i => DATA_PADDING_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DATA_PADDING_ENR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DATA_PADDING_ENR::_01
    }
}
#[doc = "Possible values of the field `TSM_IRQ0_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_IRQ0_ENR {
    #[doc = "TSM_IRQ0 is disabled"]
    _0,
    #[doc = "TSM_IRQ0 is enabled"]
    _1,
}
impl TSM_IRQ0_ENR {
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
            TSM_IRQ0_ENR::_0 => false,
            TSM_IRQ0_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSM_IRQ0_ENR {
        match value {
            false => TSM_IRQ0_ENR::_0,
            true => TSM_IRQ0_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSM_IRQ0_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSM_IRQ0_ENR::_1
    }
}
#[doc = "Possible values of the field `TSM_IRQ1_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSM_IRQ1_ENR {
    #[doc = "TSM_IRQ1 is disabled"]
    _0,
    #[doc = "TSM_IRQ1 is enabled"]
    _1,
}
impl TSM_IRQ1_ENR {
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
            TSM_IRQ1_ENR::_0 => false,
            TSM_IRQ1_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSM_IRQ1_ENR {
        match value {
            false => TSM_IRQ1_ENR::_0,
            true => TSM_IRQ1_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSM_IRQ1_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSM_IRQ1_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RAMP_DN_DELAYR {
    bits: u8,
}
impl RAMP_DN_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TX_ABORT_DISR {
    bits: bool,
}
impl TX_ABORT_DISR {
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
pub struct RX_ABORT_DISR {
    bits: bool,
}
impl RX_ABORT_DISR {
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
#[doc = "Possible values of the field `ABORT_ON_CTUNE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_ON_CTUNER {
    #[doc = "don't allow TSM abort on Coarse Tune Unlock Detect"]
    _0,
    #[doc = "allow TSM abort on Coarse Tune Unlock Detect"]
    _1,
}
impl ABORT_ON_CTUNER {
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
            ABORT_ON_CTUNER::_0 => false,
            ABORT_ON_CTUNER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABORT_ON_CTUNER {
        match value {
            false => ABORT_ON_CTUNER::_0,
            true => ABORT_ON_CTUNER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ABORT_ON_CTUNER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ABORT_ON_CTUNER::_1
    }
}
#[doc = "Possible values of the field `ABORT_ON_CYCLE_SLIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_ON_CYCLE_SLIPR {
    #[doc = "don't allow TSM abort on Cycle Slip Unlock Detect"]
    _0,
    #[doc = "allow TSM abort on Cycle Slip Unlock Detect"]
    _1,
}
impl ABORT_ON_CYCLE_SLIPR {
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
            ABORT_ON_CYCLE_SLIPR::_0 => false,
            ABORT_ON_CYCLE_SLIPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABORT_ON_CYCLE_SLIPR {
        match value {
            false => ABORT_ON_CYCLE_SLIPR::_0,
            true => ABORT_ON_CYCLE_SLIPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ABORT_ON_CYCLE_SLIPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ABORT_ON_CYCLE_SLIPR::_1
    }
}
#[doc = "Possible values of the field `ABORT_ON_FREQ_TARG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_ON_FREQ_TARGR {
    #[doc = "don't allow TSM abort on Frequency Target Unlock Detect"]
    _0,
    #[doc = "allow TSM abort on Frequency Target Unlock Detect"]
    _1,
}
impl ABORT_ON_FREQ_TARGR {
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
            ABORT_ON_FREQ_TARGR::_0 => false,
            ABORT_ON_FREQ_TARGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABORT_ON_FREQ_TARGR {
        match value {
            false => ABORT_ON_FREQ_TARGR::_0,
            true => ABORT_ON_FREQ_TARGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ABORT_ON_FREQ_TARGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ABORT_ON_FREQ_TARGR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BKPTR {
    bits: u8,
}
impl BKPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `FORCE_TX_EN`"]
pub enum FORCE_TX_ENW {
    #[doc = "TSM Idle"]
    _0,
    #[doc = "TSM executes a TX sequence"]
    _1,
}
impl FORCE_TX_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FORCE_TX_ENW::_0 => false,
            FORCE_TX_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORCE_TX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCE_TX_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCE_TX_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TSM Idle"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FORCE_TX_ENW::_0)
    }
    #[doc = "TSM executes a TX sequence"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FORCE_TX_ENW::_1)
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
#[doc = "Values that can be written to the field `FORCE_RX_EN`"]
pub enum FORCE_RX_ENW {
    #[doc = "TSM Idle"]
    _0,
    #[doc = "TSM executes a RX sequence"]
    _1,
}
impl FORCE_RX_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FORCE_RX_ENW::_0 => false,
            FORCE_RX_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORCE_RX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCE_RX_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCE_RX_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TSM Idle"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FORCE_RX_ENW::_0)
    }
    #[doc = "TSM executes a RX sequence"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FORCE_RX_ENW::_1)
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
#[doc = r" Proxy"]
pub struct _PA_RAMP_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PA_RAMP_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATA_PADDING_EN`"]
pub enum DATA_PADDING_ENW {
    #[doc = "Disable TX Data Padding"]
    _00,
    #[doc = "Enable TX Data Padding"]
    _01,
}
impl DATA_PADDING_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATA_PADDING_ENW::_00 => 0,
            DATA_PADDING_ENW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_PADDING_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_PADDING_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_PADDING_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable TX Data Padding"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DATA_PADDING_ENW::_00)
    }
    #[doc = "Enable TX Data Padding"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DATA_PADDING_ENW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSM_IRQ0_EN`"]
pub enum TSM_IRQ0_ENW {
    #[doc = "TSM_IRQ0 is disabled"]
    _0,
    #[doc = "TSM_IRQ0 is enabled"]
    _1,
}
impl TSM_IRQ0_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSM_IRQ0_ENW::_0 => false,
            TSM_IRQ0_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSM_IRQ0_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_IRQ0_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSM_IRQ0_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TSM_IRQ0 is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSM_IRQ0_ENW::_0)
    }
    #[doc = "TSM_IRQ0 is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSM_IRQ0_ENW::_1)
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
#[doc = "Values that can be written to the field `TSM_IRQ1_EN`"]
pub enum TSM_IRQ1_ENW {
    #[doc = "TSM_IRQ1 is disabled"]
    _0,
    #[doc = "TSM_IRQ1 is enabled"]
    _1,
}
impl TSM_IRQ1_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSM_IRQ1_ENW::_0 => false,
            TSM_IRQ1_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSM_IRQ1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSM_IRQ1_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSM_IRQ1_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TSM_IRQ1 is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSM_IRQ1_ENW::_0)
    }
    #[doc = "TSM_IRQ1 is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSM_IRQ1_ENW::_1)
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
#[doc = r" Proxy"]
pub struct _RAMP_DN_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMP_DN_DELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX_ABORT_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_ABORT_DISW<'a> {
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
pub struct _RX_ABORT_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ABORT_DISW<'a> {
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
#[doc = "Values that can be written to the field `ABORT_ON_CTUNE`"]
pub enum ABORT_ON_CTUNEW {
    #[doc = "don't allow TSM abort on Coarse Tune Unlock Detect"]
    _0,
    #[doc = "allow TSM abort on Coarse Tune Unlock Detect"]
    _1,
}
impl ABORT_ON_CTUNEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABORT_ON_CTUNEW::_0 => false,
            ABORT_ON_CTUNEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABORT_ON_CTUNEW<'a> {
    w: &'a mut W,
}
impl<'a> _ABORT_ON_CTUNEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABORT_ON_CTUNEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't allow TSM abort on Coarse Tune Unlock Detect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABORT_ON_CTUNEW::_0)
    }
    #[doc = "allow TSM abort on Coarse Tune Unlock Detect"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABORT_ON_CTUNEW::_1)
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
#[doc = "Values that can be written to the field `ABORT_ON_CYCLE_SLIP`"]
pub enum ABORT_ON_CYCLE_SLIPW {
    #[doc = "don't allow TSM abort on Cycle Slip Unlock Detect"]
    _0,
    #[doc = "allow TSM abort on Cycle Slip Unlock Detect"]
    _1,
}
impl ABORT_ON_CYCLE_SLIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABORT_ON_CYCLE_SLIPW::_0 => false,
            ABORT_ON_CYCLE_SLIPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABORT_ON_CYCLE_SLIPW<'a> {
    w: &'a mut W,
}
impl<'a> _ABORT_ON_CYCLE_SLIPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABORT_ON_CYCLE_SLIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't allow TSM abort on Cycle Slip Unlock Detect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABORT_ON_CYCLE_SLIPW::_0)
    }
    #[doc = "allow TSM abort on Cycle Slip Unlock Detect"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABORT_ON_CYCLE_SLIPW::_1)
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
#[doc = "Values that can be written to the field `ABORT_ON_FREQ_TARG`"]
pub enum ABORT_ON_FREQ_TARGW {
    #[doc = "don't allow TSM abort on Frequency Target Unlock Detect"]
    _0,
    #[doc = "allow TSM abort on Frequency Target Unlock Detect"]
    _1,
}
impl ABORT_ON_FREQ_TARGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABORT_ON_FREQ_TARGW::_0 => false,
            ABORT_ON_FREQ_TARGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABORT_ON_FREQ_TARGW<'a> {
    w: &'a mut W,
}
impl<'a> _ABORT_ON_FREQ_TARGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABORT_ON_FREQ_TARGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "don't allow TSM abort on Frequency Target Unlock Detect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABORT_ON_FREQ_TARGW::_0)
    }
    #[doc = "allow TSM abort on Frequency Target Unlock Detect"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABORT_ON_FREQ_TARGW::_1)
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
#[doc = r" Proxy"]
pub struct _BKPTW<'a> {
    w: &'a mut W,
}
impl<'a> _BKPTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 2 - Force Transmit Enable"]
    #[inline]
    pub fn force_tx_en(&self) -> FORCE_TX_ENR {
        FORCE_TX_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Force Receive Enable"]
    #[inline]
    pub fn force_rx_en(&self) -> FORCE_RX_ENR {
        FORCE_RX_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - PA Ramp Selection"]
    #[inline]
    pub fn pa_ramp_sel(&self) -> PA_RAMP_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PA_RAMP_SELR { bits }
    }
    #[doc = "Bits 6:7 - Data Padding Enable"]
    #[inline]
    pub fn data_padding_en(&self) -> DATA_PADDING_ENR {
        DATA_PADDING_ENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - TSM_IRQ0 Enable/Disable bit"]
    #[inline]
    pub fn tsm_irq0_en(&self) -> TSM_IRQ0_ENR {
        TSM_IRQ0_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - TSM_IRQ1 Enable/Disable bit"]
    #[inline]
    pub fn tsm_irq1_en(&self) -> TSM_IRQ1_ENR {
        TSM_IRQ1_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:15 - PA Ramp Down Delay"]
    #[inline]
    pub fn ramp_dn_delay(&self) -> RAMP_DN_DELAYR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RAMP_DN_DELAYR { bits }
    }
    #[doc = "Bit 16 - Transmit Abort Disable"]
    #[inline]
    pub fn tx_abort_dis(&self) -> TX_ABORT_DISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_ABORT_DISR { bits }
    }
    #[doc = "Bit 17 - Receive Abort Disable"]
    #[inline]
    pub fn rx_abort_dis(&self) -> RX_ABORT_DISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_ABORT_DISR { bits }
    }
    #[doc = "Bit 18 - Abort On Coarse Tune Lock Detect Failure"]
    #[inline]
    pub fn abort_on_ctune(&self) -> ABORT_ON_CTUNER {
        ABORT_ON_CTUNER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Abort On Cycle Slip Lock Detect Failure"]
    #[inline]
    pub fn abort_on_cycle_slip(&self) -> ABORT_ON_CYCLE_SLIPR {
        ABORT_ON_CYCLE_SLIPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Abort On Frequency Target Lock Detect Failure"]
    #[inline]
    pub fn abort_on_freq_targ(&self) -> ABORT_ON_FREQ_TARGR {
        ABORT_ON_FREQ_TARGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:31 - TSM Breakpoint"]
    #[inline]
    pub fn bkpt(&self) -> BKPTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BKPTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4278206464 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Force Transmit Enable"]
    #[inline]
    pub fn force_tx_en(&mut self) -> _FORCE_TX_ENW {
        _FORCE_TX_ENW { w: self }
    }
    #[doc = "Bit 3 - Force Receive Enable"]
    #[inline]
    pub fn force_rx_en(&mut self) -> _FORCE_RX_ENW {
        _FORCE_RX_ENW { w: self }
    }
    #[doc = "Bits 4:5 - PA Ramp Selection"]
    #[inline]
    pub fn pa_ramp_sel(&mut self) -> _PA_RAMP_SELW {
        _PA_RAMP_SELW { w: self }
    }
    #[doc = "Bits 6:7 - Data Padding Enable"]
    #[inline]
    pub fn data_padding_en(&mut self) -> _DATA_PADDING_ENW {
        _DATA_PADDING_ENW { w: self }
    }
    #[doc = "Bit 8 - TSM_IRQ0 Enable/Disable bit"]
    #[inline]
    pub fn tsm_irq0_en(&mut self) -> _TSM_IRQ0_ENW {
        _TSM_IRQ0_ENW { w: self }
    }
    #[doc = "Bit 9 - TSM_IRQ1 Enable/Disable bit"]
    #[inline]
    pub fn tsm_irq1_en(&mut self) -> _TSM_IRQ1_ENW {
        _TSM_IRQ1_ENW { w: self }
    }
    #[doc = "Bits 12:15 - PA Ramp Down Delay"]
    #[inline]
    pub fn ramp_dn_delay(&mut self) -> _RAMP_DN_DELAYW {
        _RAMP_DN_DELAYW { w: self }
    }
    #[doc = "Bit 16 - Transmit Abort Disable"]
    #[inline]
    pub fn tx_abort_dis(&mut self) -> _TX_ABORT_DISW {
        _TX_ABORT_DISW { w: self }
    }
    #[doc = "Bit 17 - Receive Abort Disable"]
    #[inline]
    pub fn rx_abort_dis(&mut self) -> _RX_ABORT_DISW {
        _RX_ABORT_DISW { w: self }
    }
    #[doc = "Bit 18 - Abort On Coarse Tune Lock Detect Failure"]
    #[inline]
    pub fn abort_on_ctune(&mut self) -> _ABORT_ON_CTUNEW {
        _ABORT_ON_CTUNEW { w: self }
    }
    #[doc = "Bit 19 - Abort On Cycle Slip Lock Detect Failure"]
    #[inline]
    pub fn abort_on_cycle_slip(&mut self) -> _ABORT_ON_CYCLE_SLIPW {
        _ABORT_ON_CYCLE_SLIPW { w: self }
    }
    #[doc = "Bit 20 - Abort On Frequency Target Lock Detect Failure"]
    #[inline]
    pub fn abort_on_freq_targ(&mut self) -> _ABORT_ON_FREQ_TARGW {
        _ABORT_ON_FREQ_TARGW { w: self }
    }
    #[doc = "Bits 24:31 - TSM Breakpoint"]
    #[inline]
    pub fn bkpt(&mut self) -> _BKPTW {
        _BKPTW { w: self }
    }
}
