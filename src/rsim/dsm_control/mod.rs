#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DSM_CONTROL {
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
pub struct DSM_ANT_READYR {
    bits: bool,
}
impl DSM_ANT_READYR {
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
pub struct ANT_DEEP_SLEEP_STATUSR {
    bits: bool,
}
impl ANT_DEEP_SLEEP_STATUSR {
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
pub struct DSM_ANT_FINISHEDR {
    bits: bool,
}
impl DSM_ANT_FINISHEDR {
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
pub struct ANT_SYSCLK_REQUEST_ENR {
    bits: bool,
}
impl ANT_SYSCLK_REQUEST_ENR {
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
pub struct ANT_SLEEP_REQUESTR {
    bits: bool,
}
impl ANT_SLEEP_REQUESTR {
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
pub struct ANT_SYSCLK_REQR {
    bits: bool,
}
impl ANT_SYSCLK_REQR {
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
pub struct ANT_SYSCLK_INTERRUPT_ENR {
    bits: bool,
}
impl ANT_SYSCLK_INTERRUPT_ENR {
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
pub struct ANT_SYSCLK_REQ_INTR {
    bits: bool,
}
impl ANT_SYSCLK_REQ_INTR {
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
pub struct DSM_GEN_READYR {
    bits: bool,
}
impl DSM_GEN_READYR {
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
pub struct GEN_DEEP_SLEEP_STATUSR {
    bits: bool,
}
impl GEN_DEEP_SLEEP_STATUSR {
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
pub struct DSM_GEN_FINISHEDR {
    bits: bool,
}
impl DSM_GEN_FINISHEDR {
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
pub struct GEN_SYSCLK_REQUEST_ENR {
    bits: bool,
}
impl GEN_SYSCLK_REQUEST_ENR {
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
pub struct GEN_SLEEP_REQUESTR {
    bits: bool,
}
impl GEN_SLEEP_REQUESTR {
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
pub struct GEN_SYSCLK_REQR {
    bits: bool,
}
impl GEN_SYSCLK_REQR {
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
pub struct GEN_SYSCLK_INTERRUPT_ENR {
    bits: bool,
}
impl GEN_SYSCLK_INTERRUPT_ENR {
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
pub struct GEN_SYSCLK_REQ_INTR {
    bits: bool,
}
impl GEN_SYSCLK_REQ_INTR {
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
pub struct DSM_ZIG_READYR {
    bits: bool,
}
impl DSM_ZIG_READYR {
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
pub struct ZIG_DEEP_SLEEP_STATUSR {
    bits: bool,
}
impl ZIG_DEEP_SLEEP_STATUSR {
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
pub struct DSM_ZIG_FINISHEDR {
    bits: bool,
}
impl DSM_ZIG_FINISHEDR {
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
pub struct ZIG_SYSCLK_REQUEST_ENR {
    bits: bool,
}
impl ZIG_SYSCLK_REQUEST_ENR {
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
pub struct ZIG_SLEEP_REQUESTR {
    bits: bool,
}
impl ZIG_SLEEP_REQUESTR {
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
pub struct ZIG_SYSCLK_REQR {
    bits: bool,
}
impl ZIG_SYSCLK_REQR {
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
pub struct ZIG_SYSCLK_INTERRUPT_ENR {
    bits: bool,
}
impl ZIG_SYSCLK_INTERRUPT_ENR {
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
pub struct ZIG_SYSCLK_REQ_INTR {
    bits: bool,
}
impl ZIG_SYSCLK_REQ_INTR {
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
pub struct DSM_TIMER_CLRR {
    bits: bool,
}
impl DSM_TIMER_CLRR {
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
pub struct DSM_TIMER_ENR {
    bits: bool,
}
impl DSM_TIMER_ENR {
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
#[doc = r" Proxy"]
pub struct _ANT_SYSCLK_REQUEST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ANT_SYSCLK_REQUEST_ENW<'a> {
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
pub struct _ANT_SYSCLK_INTERRUPT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ANT_SYSCLK_INTERRUPT_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _ANT_SYSCLK_REQ_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _ANT_SYSCLK_REQ_INTW<'a> {
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
pub struct _GEN_SYSCLK_REQUEST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _GEN_SYSCLK_REQUEST_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _GEN_SYSCLK_INTERRUPT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _GEN_SYSCLK_INTERRUPT_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _ZIG_SYSCLK_REQUEST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ZIG_SYSCLK_REQUEST_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _ZIG_SYSCLK_INTERRUPT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ZIG_SYSCLK_INTERRUPT_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _DSM_TIMER_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DSM_TIMER_CLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _DSM_TIMER_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DSM_TIMER_ENW<'a> {
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
    #[doc = "Bit 0 - ANT Ready for Deep Sleep Mode"]
    #[inline]
    pub fn dsm_ant_ready(&self) -> DSM_ANT_READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSM_ANT_READYR { bits }
    }
    #[doc = "Bit 1 - ANT Link Layer Deep Sleep Mode Status"]
    #[inline]
    pub fn ant_deep_sleep_status(&self) -> ANT_DEEP_SLEEP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ANT_DEEP_SLEEP_STATUSR { bits }
    }
    #[doc = "Bit 2 - ANT Deep Sleep Time Finished"]
    #[inline]
    pub fn dsm_ant_finished(&self) -> DSM_ANT_FINISHEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSM_ANT_FINISHEDR { bits }
    }
    #[doc = "Bit 3 - Enable ANT Link Layer to Request RF OSC"]
    #[inline]
    pub fn ant_sysclk_request_en(&self) -> ANT_SYSCLK_REQUEST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ANT_SYSCLK_REQUEST_ENR { bits }
    }
    #[doc = "Bit 4 - ANT Link Layer Deep Sleep Requested"]
    #[inline]
    pub fn ant_sleep_request(&self) -> ANT_SLEEP_REQUESTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ANT_SLEEP_REQUESTR { bits }
    }
    #[doc = "Bit 5 - ANT Link Layer RF OSC Request Status"]
    #[inline]
    pub fn ant_sysclk_req(&self) -> ANT_SYSCLK_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ANT_SYSCLK_REQR { bits }
    }
    #[doc = "Bit 6 - ANT Link Layer RF OSC Request Interrupt Enable"]
    #[inline]
    pub fn ant_sysclk_interrupt_en(&self) -> ANT_SYSCLK_INTERRUPT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ANT_SYSCLK_INTERRUPT_ENR { bits }
    }
    #[doc = "Bit 7 - Interrupt Flag from an ANT Link Layer RF OSC Request"]
    #[inline]
    pub fn ant_sysclk_req_int(&self) -> ANT_SYSCLK_REQ_INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ANT_SYSCLK_REQ_INTR { bits }
    }
    #[doc = "Bit 8 - Generic FSK Ready for Deep Sleep Mode"]
    #[inline]
    pub fn dsm_gen_ready(&self) -> DSM_GEN_READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSM_GEN_READYR { bits }
    }
    #[doc = "Bit 9 - Generic FSK Link Layer Deep Sleep Mode Status"]
    #[inline]
    pub fn gen_deep_sleep_status(&self) -> GEN_DEEP_SLEEP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GEN_DEEP_SLEEP_STATUSR { bits }
    }
    #[doc = "Bit 10 - Generic FSK Deep Sleep Time Finished"]
    #[inline]
    pub fn dsm_gen_finished(&self) -> DSM_GEN_FINISHEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSM_GEN_FINISHEDR { bits }
    }
    #[doc = "Bit 11 - Enable Generic FSK Link Layer to Request RF OSC"]
    #[inline]
    pub fn gen_sysclk_request_en(&self) -> GEN_SYSCLK_REQUEST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GEN_SYSCLK_REQUEST_ENR { bits }
    }
    #[doc = "Bit 12 - Generic FSK Link Layer Deep Sleep Requested"]
    #[inline]
    pub fn gen_sleep_request(&self) -> GEN_SLEEP_REQUESTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GEN_SLEEP_REQUESTR { bits }
    }
    #[doc = "Bit 13 - Generic FSK Link Layer RF OSC Request Status"]
    #[inline]
    pub fn gen_sysclk_req(&self) -> GEN_SYSCLK_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GEN_SYSCLK_REQR { bits }
    }
    #[doc = "Bit 14 - Generic FSK Link Layer RF OSC Request Interrupt Enable"]
    #[inline]
    pub fn gen_sysclk_interrupt_en(&self) -> GEN_SYSCLK_INTERRUPT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GEN_SYSCLK_INTERRUPT_ENR { bits }
    }
    #[doc = "Bit 15 - Interrupt Flag from an Generic FSK Link Layer RF OSC Request"]
    #[inline]
    pub fn gen_sysclk_req_int(&self) -> GEN_SYSCLK_REQ_INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GEN_SYSCLK_REQ_INTR { bits }
    }
    #[doc = "Bit 16 - 802.15.4 Ready for Deep Sleep Mode"]
    #[inline]
    pub fn dsm_zig_ready(&self) -> DSM_ZIG_READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSM_ZIG_READYR { bits }
    }
    #[doc = "Bit 17 - 802.15.4 Link Layer Deep Sleep Mode Status"]
    #[inline]
    pub fn zig_deep_sleep_status(&self) -> ZIG_DEEP_SLEEP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ZIG_DEEP_SLEEP_STATUSR { bits }
    }
    #[doc = "Bit 18 - 802.15.4 Deep Sleep Time Finished"]
    #[inline]
    pub fn dsm_zig_finished(&self) -> DSM_ZIG_FINISHEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSM_ZIG_FINISHEDR { bits }
    }
    #[doc = "Bit 19 - Enable 802.15.4 Link Layer to Request RF OSC"]
    #[inline]
    pub fn zig_sysclk_request_en(&self) -> ZIG_SYSCLK_REQUEST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ZIG_SYSCLK_REQUEST_ENR { bits }
    }
    #[doc = "Bit 20 - 802.15.4 Link Layer Deep Sleep Requested"]
    #[inline]
    pub fn zig_sleep_request(&self) -> ZIG_SLEEP_REQUESTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ZIG_SLEEP_REQUESTR { bits }
    }
    #[doc = "Bit 21 - 802.15.4 Link Layer RF OSC Request Status"]
    #[inline]
    pub fn zig_sysclk_req(&self) -> ZIG_SYSCLK_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ZIG_SYSCLK_REQR { bits }
    }
    #[doc = "Bit 22 - 802.15.4 Link Layer RF OSC Request Interrupt Enable"]
    #[inline]
    pub fn zig_sysclk_interrupt_en(&self) -> ZIG_SYSCLK_INTERRUPT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ZIG_SYSCLK_INTERRUPT_ENR { bits }
    }
    #[doc = "Bit 23 - Interrupt Flag from an 802.15.4 Link Layer RF OSC Request"]
    #[inline]
    pub fn zig_sysclk_req_int(&self) -> ZIG_SYSCLK_REQ_INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ZIG_SYSCLK_REQ_INTR { bits }
    }
    #[doc = "Bit 27 - Deep Sleep Mode Timer Clear"]
    #[inline]
    pub fn dsm_timer_clr(&self) -> DSM_TIMER_CLRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSM_TIMER_CLRR { bits }
    }
    #[doc = "Bit 31 - Deep Sleep Mode Timer Enable"]
    #[inline]
    pub fn dsm_timer_en(&self) -> DSM_TIMER_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSM_TIMER_ENR { bits }
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
    #[doc = "Bit 3 - Enable ANT Link Layer to Request RF OSC"]
    #[inline]
    pub fn ant_sysclk_request_en(&mut self) -> _ANT_SYSCLK_REQUEST_ENW {
        _ANT_SYSCLK_REQUEST_ENW { w: self }
    }
    #[doc = "Bit 6 - ANT Link Layer RF OSC Request Interrupt Enable"]
    #[inline]
    pub fn ant_sysclk_interrupt_en(&mut self) -> _ANT_SYSCLK_INTERRUPT_ENW {
        _ANT_SYSCLK_INTERRUPT_ENW { w: self }
    }
    #[doc = "Bit 7 - Interrupt Flag from an ANT Link Layer RF OSC Request"]
    #[inline]
    pub fn ant_sysclk_req_int(&mut self) -> _ANT_SYSCLK_REQ_INTW {
        _ANT_SYSCLK_REQ_INTW { w: self }
    }
    #[doc = "Bit 11 - Enable Generic FSK Link Layer to Request RF OSC"]
    #[inline]
    pub fn gen_sysclk_request_en(&mut self) -> _GEN_SYSCLK_REQUEST_ENW {
        _GEN_SYSCLK_REQUEST_ENW { w: self }
    }
    #[doc = "Bit 14 - Generic FSK Link Layer RF OSC Request Interrupt Enable"]
    #[inline]
    pub fn gen_sysclk_interrupt_en(&mut self) -> _GEN_SYSCLK_INTERRUPT_ENW {
        _GEN_SYSCLK_INTERRUPT_ENW { w: self }
    }
    #[doc = "Bit 19 - Enable 802.15.4 Link Layer to Request RF OSC"]
    #[inline]
    pub fn zig_sysclk_request_en(&mut self) -> _ZIG_SYSCLK_REQUEST_ENW {
        _ZIG_SYSCLK_REQUEST_ENW { w: self }
    }
    #[doc = "Bit 22 - 802.15.4 Link Layer RF OSC Request Interrupt Enable"]
    #[inline]
    pub fn zig_sysclk_interrupt_en(&mut self) -> _ZIG_SYSCLK_INTERRUPT_ENW {
        _ZIG_SYSCLK_INTERRUPT_ENW { w: self }
    }
    #[doc = "Bit 27 - Deep Sleep Mode Timer Clear"]
    #[inline]
    pub fn dsm_timer_clr(&mut self) -> _DSM_TIMER_CLRW {
        _DSM_TIMER_CLRW { w: self }
    }
    #[doc = "Bit 31 - Deep Sleep Mode Timer Enable"]
    #[inline]
    pub fn dsm_timer_en(&mut self) -> _DSM_TIMER_ENW {
        _DSM_TIMER_ENW { w: self }
    }
}
