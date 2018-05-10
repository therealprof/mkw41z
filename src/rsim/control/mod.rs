#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONTROL {
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
pub struct BLE_RF_OSC_REQ_ENR {
    bits: bool,
}
impl BLE_RF_OSC_REQ_ENR {
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
pub struct BLE_RF_OSC_REQ_STATR {
    bits: bool,
}
impl BLE_RF_OSC_REQ_STATR {
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
pub struct BLE_RF_OSC_REQ_INT_ENR {
    bits: bool,
}
impl BLE_RF_OSC_REQ_INT_ENR {
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
pub struct BLE_RF_OSC_REQ_INTR {
    bits: bool,
}
impl BLE_RF_OSC_REQ_INTR {
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
#[doc = "Possible values of the field `RF_OSC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_OSC_ENR {
    #[doc = "RF Ref Osc will be controlled by the SoC, external pin, or a link layer"]
    _0000,
    #[doc = "RF Ref Osc on in Run/Wait"]
    _0001,
    #[doc = "RF Ref Osc on in Stop"]
    _0011,
    #[doc = "RF Ref Osc on in VLPR/VLPW"]
    _0111,
    #[doc = "RF Ref Osc on in VLPS"]
    _1111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RF_OSC_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RF_OSC_ENR::_0000 => 0,
            RF_OSC_ENR::_0001 => 1,
            RF_OSC_ENR::_0011 => 3,
            RF_OSC_ENR::_0111 => 7,
            RF_OSC_ENR::_1111 => 15,
            RF_OSC_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RF_OSC_ENR {
        match value {
            0 => RF_OSC_ENR::_0000,
            1 => RF_OSC_ENR::_0001,
            3 => RF_OSC_ENR::_0011,
            7 => RF_OSC_ENR::_0111,
            15 => RF_OSC_ENR::_1111,
            i => RF_OSC_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == RF_OSC_ENR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == RF_OSC_ENR::_0001
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == RF_OSC_ENR::_0011
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == RF_OSC_ENR::_0111
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == RF_OSC_ENR::_1111
    }
}
#[doc = r" Value of the field"]
pub struct RADIO_GASKET_BYPASS_OVRD_ENR {
    bits: bool,
}
impl RADIO_GASKET_BYPASS_OVRD_ENR {
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
pub struct RADIO_GASKET_BYPASS_OVRDR {
    bits: bool,
}
impl RADIO_GASKET_BYPASS_OVRDR {
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
pub struct IPP_OBE_3V_BLE_ACTIVE_1R {
    bits: bool,
}
impl IPP_OBE_3V_BLE_ACTIVE_1R {
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
pub struct IPP_OBE_3V_BLE_ACTIVE_2R {
    bits: bool,
}
impl IPP_OBE_3V_BLE_ACTIVE_2R {
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
pub struct RADIO_RAM_ACCESS_OVRD_ENR {
    bits: bool,
}
impl RADIO_RAM_ACCESS_OVRD_ENR {
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
pub struct RADIO_RAM_ACCESS_OVRDR {
    bits: bool,
}
impl RADIO_RAM_ACCESS_OVRDR {
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
pub struct RSIM_DSM_EXITR {
    bits: bool,
}
impl RSIM_DSM_EXITR {
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
pub struct RSIM_STOP_ACK_OVRD_ENR {
    bits: bool,
}
impl RSIM_STOP_ACK_OVRD_ENR {
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
pub struct RSIM_STOP_ACK_OVRDR {
    bits: bool,
}
impl RSIM_STOP_ACK_OVRDR {
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
pub struct RF_OSC_READYR {
    bits: bool,
}
impl RF_OSC_READYR {
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
pub struct RF_OSC_READY_OVRD_ENR {
    bits: bool,
}
impl RF_OSC_READY_OVRD_ENR {
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
pub struct RF_OSC_READY_OVRDR {
    bits: bool,
}
impl RF_OSC_READY_OVRDR {
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
pub struct BLOCK_SOC_RESETSR {
    bits: bool,
}
impl BLOCK_SOC_RESETSR {
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
pub struct BLOCK_RADIO_OUTPUTSR {
    bits: bool,
}
impl BLOCK_RADIO_OUTPUTSR {
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
pub struct ALLOW_DFT_RESETSR {
    bits: bool,
}
impl ALLOW_DFT_RESETSR {
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
pub struct RADIO_RESET_BITR {
    bits: bool,
}
impl RADIO_RESET_BITR {
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
pub struct _BLE_RF_OSC_REQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BLE_RF_OSC_REQ_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _BLE_RF_OSC_REQ_INT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BLE_RF_OSC_REQ_INT_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _BLE_RF_OSC_REQ_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _BLE_RF_OSC_REQ_INTW<'a> {
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
#[doc = "Values that can be written to the field `RF_OSC_EN`"]
pub enum RF_OSC_ENW {
    #[doc = "RF Ref Osc will be controlled by the SoC, external pin, or a link layer"]
    _0000,
    #[doc = "RF Ref Osc on in Run/Wait"]
    _0001,
    #[doc = "RF Ref Osc on in Stop"]
    _0011,
    #[doc = "RF Ref Osc on in VLPR/VLPW"]
    _0111,
    #[doc = "RF Ref Osc on in VLPS"]
    _1111,
}
impl RF_OSC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RF_OSC_ENW::_0000 => 0,
            RF_OSC_ENW::_0001 => 1,
            RF_OSC_ENW::_0011 => 3,
            RF_OSC_ENW::_0111 => 7,
            RF_OSC_ENW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RF_OSC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RF_OSC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RF_OSC_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "RF Ref Osc will be controlled by the SoC, external pin, or a link layer"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(RF_OSC_ENW::_0000)
    }
    #[doc = "RF Ref Osc on in Run/Wait"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(RF_OSC_ENW::_0001)
    }
    #[doc = "RF Ref Osc on in Stop"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(RF_OSC_ENW::_0011)
    }
    #[doc = "RF Ref Osc on in VLPR/VLPW"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(RF_OSC_ENW::_0111)
    }
    #[doc = "RF Ref Osc on in VLPS"]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(RF_OSC_ENW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RADIO_GASKET_BYPASS_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RADIO_GASKET_BYPASS_OVRD_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _RADIO_GASKET_BYPASS_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RADIO_GASKET_BYPASS_OVRDW<'a> {
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
#[doc = r" Proxy"]
pub struct _IPP_OBE_3V_BLE_ACTIVE_1W<'a> {
    w: &'a mut W,
}
impl<'a> _IPP_OBE_3V_BLE_ACTIVE_1W<'a> {
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
pub struct _IPP_OBE_3V_BLE_ACTIVE_2W<'a> {
    w: &'a mut W,
}
impl<'a> _IPP_OBE_3V_BLE_ACTIVE_2W<'a> {
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
pub struct _RADIO_RAM_ACCESS_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RADIO_RAM_ACCESS_OVRD_ENW<'a> {
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
pub struct _RADIO_RAM_ACCESS_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RADIO_RAM_ACCESS_OVRDW<'a> {
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
pub struct _RSIM_DSM_EXITW<'a> {
    w: &'a mut W,
}
impl<'a> _RSIM_DSM_EXITW<'a> {
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
pub struct _RSIM_STOP_ACK_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RSIM_STOP_ACK_OVRD_ENW<'a> {
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
pub struct _RSIM_STOP_ACK_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RSIM_STOP_ACK_OVRDW<'a> {
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
#[doc = r" Proxy"]
pub struct _RF_OSC_READY_OVRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RF_OSC_READY_OVRD_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _RF_OSC_READY_OVRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RF_OSC_READY_OVRDW<'a> {
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
#[doc = r" Proxy"]
pub struct _BLOCK_SOC_RESETSW<'a> {
    w: &'a mut W,
}
impl<'a> _BLOCK_SOC_RESETSW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BLOCK_RADIO_OUTPUTSW<'a> {
    w: &'a mut W,
}
impl<'a> _BLOCK_RADIO_OUTPUTSW<'a> {
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
#[doc = r" Proxy"]
pub struct _ALLOW_DFT_RESETSW<'a> {
    w: &'a mut W,
}
impl<'a> _ALLOW_DFT_RESETSW<'a> {
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
#[doc = r" Proxy"]
pub struct _RADIO_RESET_BITW<'a> {
    w: &'a mut W,
}
impl<'a> _RADIO_RESET_BITW<'a> {
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
    #[doc = "Bit 0 - BLE Ref Osc (Sysclk) Request Enable"]
    #[inline]
    pub fn ble_rf_osc_req_en(&self) -> BLE_RF_OSC_REQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLE_RF_OSC_REQ_ENR { bits }
    }
    #[doc = "Bit 1 - BLE Ref Osc (Sysclk) Request Status"]
    #[inline]
    pub fn ble_rf_osc_req_stat(&self) -> BLE_RF_OSC_REQ_STATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLE_RF_OSC_REQ_STATR { bits }
    }
    #[doc = "Bit 4 - BLE Ref Osc (Sysclk) Request Interrupt Enable"]
    #[inline]
    pub fn ble_rf_osc_req_int_en(&self) -> BLE_RF_OSC_REQ_INT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLE_RF_OSC_REQ_INT_ENR { bits }
    }
    #[doc = "Bit 5 - BLE Ref Osc (Sysclk) Request Interrupt Flag"]
    #[inline]
    pub fn ble_rf_osc_req_int(&self) -> BLE_RF_OSC_REQ_INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLE_RF_OSC_REQ_INTR { bits }
    }
    #[doc = "Bits 8:11 - RF Ref Osc Enable Select"]
    #[inline]
    pub fn rf_osc_en(&self) -> RF_OSC_ENR {
        RF_OSC_ENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Radio Gasket Bypass Override Enable"]
    #[inline]
    pub fn radio_gasket_bypass_ovrd_en(&self) -> RADIO_GASKET_BYPASS_OVRD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RADIO_GASKET_BYPASS_OVRD_ENR { bits }
    }
    #[doc = "Bit 13 - Radio Gasket Bypass Override"]
    #[inline]
    pub fn radio_gasket_bypass_ovrd(&self) -> RADIO_GASKET_BYPASS_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RADIO_GASKET_BYPASS_OVRDR { bits }
    }
    #[doc = "Bit 16 - IPP_OBE_3V_BLE_ACTIVE_1"]
    #[inline]
    pub fn ipp_obe_3v_ble_active_1(&self) -> IPP_OBE_3V_BLE_ACTIVE_1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IPP_OBE_3V_BLE_ACTIVE_1R { bits }
    }
    #[doc = "Bit 17 - IPP_OBE_3V_BLE_ACTIVE_2"]
    #[inline]
    pub fn ipp_obe_3v_ble_active_2(&self) -> IPP_OBE_3V_BLE_ACTIVE_2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IPP_OBE_3V_BLE_ACTIVE_2R { bits }
    }
    #[doc = "Bit 18 - Radio RAM Access Override Enable"]
    #[inline]
    pub fn radio_ram_access_ovrd_en(&self) -> RADIO_RAM_ACCESS_OVRD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RADIO_RAM_ACCESS_OVRD_ENR { bits }
    }
    #[doc = "Bit 19 - Radio RAM Access Override"]
    #[inline]
    pub fn radio_ram_access_ovrd(&self) -> RADIO_RAM_ACCESS_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RADIO_RAM_ACCESS_OVRDR { bits }
    }
    #[doc = "Bit 20 - BLE Force Deep Sleep Mode Exit"]
    #[inline]
    pub fn rsim_dsm_exit(&self) -> RSIM_DSM_EXITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSIM_DSM_EXITR { bits }
    }
    #[doc = "Bit 22 - Stop Acknowledge Override Enable"]
    #[inline]
    pub fn rsim_stop_ack_ovrd_en(&self) -> RSIM_STOP_ACK_OVRD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSIM_STOP_ACK_OVRD_ENR { bits }
    }
    #[doc = "Bit 23 - Stop Acknowledge Override"]
    #[inline]
    pub fn rsim_stop_ack_ovrd(&self) -> RSIM_STOP_ACK_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSIM_STOP_ACK_OVRDR { bits }
    }
    #[doc = "Bit 24 - RF Ref Osc Ready"]
    #[inline]
    pub fn rf_osc_ready(&self) -> RF_OSC_READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RF_OSC_READYR { bits }
    }
    #[doc = "Bit 25 - RF Ref Osc Ready Override Enable"]
    #[inline]
    pub fn rf_osc_ready_ovrd_en(&self) -> RF_OSC_READY_OVRD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RF_OSC_READY_OVRD_ENR { bits }
    }
    #[doc = "Bit 26 - RF Ref Osc Ready Override"]
    #[inline]
    pub fn rf_osc_ready_ovrd(&self) -> RF_OSC_READY_OVRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RF_OSC_READY_OVRDR { bits }
    }
    #[doc = "Bit 28 - Block SoC Resets of the Radio"]
    #[inline]
    pub fn block_soc_resets(&self) -> BLOCK_SOC_RESETSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLOCK_SOC_RESETSR { bits }
    }
    #[doc = "Bit 29 - Block Radio Outputs"]
    #[inline]
    pub fn block_radio_outputs(&self) -> BLOCK_RADIO_OUTPUTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLOCK_RADIO_OUTPUTSR { bits }
    }
    #[doc = "Bit 30 - Allow the DFT Reset Pin to Reset the Radio"]
    #[inline]
    pub fn allow_dft_resets(&self) -> ALLOW_DFT_RESETSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALLOW_DFT_RESETSR { bits }
    }
    #[doc = "Bit 31 - Software Reset for the Radio"]
    #[inline]
    pub fn radio_reset_bit(&self) -> RADIO_RESET_BITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RADIO_RESET_BITR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 12582914 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - BLE Ref Osc (Sysclk) Request Enable"]
    #[inline]
    pub fn ble_rf_osc_req_en(&mut self) -> _BLE_RF_OSC_REQ_ENW {
        _BLE_RF_OSC_REQ_ENW { w: self }
    }
    #[doc = "Bit 4 - BLE Ref Osc (Sysclk) Request Interrupt Enable"]
    #[inline]
    pub fn ble_rf_osc_req_int_en(&mut self) -> _BLE_RF_OSC_REQ_INT_ENW {
        _BLE_RF_OSC_REQ_INT_ENW { w: self }
    }
    #[doc = "Bit 5 - BLE Ref Osc (Sysclk) Request Interrupt Flag"]
    #[inline]
    pub fn ble_rf_osc_req_int(&mut self) -> _BLE_RF_OSC_REQ_INTW {
        _BLE_RF_OSC_REQ_INTW { w: self }
    }
    #[doc = "Bits 8:11 - RF Ref Osc Enable Select"]
    #[inline]
    pub fn rf_osc_en(&mut self) -> _RF_OSC_ENW {
        _RF_OSC_ENW { w: self }
    }
    #[doc = "Bit 12 - Radio Gasket Bypass Override Enable"]
    #[inline]
    pub fn radio_gasket_bypass_ovrd_en(&mut self) -> _RADIO_GASKET_BYPASS_OVRD_ENW {
        _RADIO_GASKET_BYPASS_OVRD_ENW { w: self }
    }
    #[doc = "Bit 13 - Radio Gasket Bypass Override"]
    #[inline]
    pub fn radio_gasket_bypass_ovrd(&mut self) -> _RADIO_GASKET_BYPASS_OVRDW {
        _RADIO_GASKET_BYPASS_OVRDW { w: self }
    }
    #[doc = "Bit 16 - IPP_OBE_3V_BLE_ACTIVE_1"]
    #[inline]
    pub fn ipp_obe_3v_ble_active_1(&mut self) -> _IPP_OBE_3V_BLE_ACTIVE_1W {
        _IPP_OBE_3V_BLE_ACTIVE_1W { w: self }
    }
    #[doc = "Bit 17 - IPP_OBE_3V_BLE_ACTIVE_2"]
    #[inline]
    pub fn ipp_obe_3v_ble_active_2(&mut self) -> _IPP_OBE_3V_BLE_ACTIVE_2W {
        _IPP_OBE_3V_BLE_ACTIVE_2W { w: self }
    }
    #[doc = "Bit 18 - Radio RAM Access Override Enable"]
    #[inline]
    pub fn radio_ram_access_ovrd_en(&mut self) -> _RADIO_RAM_ACCESS_OVRD_ENW {
        _RADIO_RAM_ACCESS_OVRD_ENW { w: self }
    }
    #[doc = "Bit 19 - Radio RAM Access Override"]
    #[inline]
    pub fn radio_ram_access_ovrd(&mut self) -> _RADIO_RAM_ACCESS_OVRDW {
        _RADIO_RAM_ACCESS_OVRDW { w: self }
    }
    #[doc = "Bit 20 - BLE Force Deep Sleep Mode Exit"]
    #[inline]
    pub fn rsim_dsm_exit(&mut self) -> _RSIM_DSM_EXITW {
        _RSIM_DSM_EXITW { w: self }
    }
    #[doc = "Bit 22 - Stop Acknowledge Override Enable"]
    #[inline]
    pub fn rsim_stop_ack_ovrd_en(&mut self) -> _RSIM_STOP_ACK_OVRD_ENW {
        _RSIM_STOP_ACK_OVRD_ENW { w: self }
    }
    #[doc = "Bit 23 - Stop Acknowledge Override"]
    #[inline]
    pub fn rsim_stop_ack_ovrd(&mut self) -> _RSIM_STOP_ACK_OVRDW {
        _RSIM_STOP_ACK_OVRDW { w: self }
    }
    #[doc = "Bit 25 - RF Ref Osc Ready Override Enable"]
    #[inline]
    pub fn rf_osc_ready_ovrd_en(&mut self) -> _RF_OSC_READY_OVRD_ENW {
        _RF_OSC_READY_OVRD_ENW { w: self }
    }
    #[doc = "Bit 26 - RF Ref Osc Ready Override"]
    #[inline]
    pub fn rf_osc_ready_ovrd(&mut self) -> _RF_OSC_READY_OVRDW {
        _RF_OSC_READY_OVRDW { w: self }
    }
    #[doc = "Bit 28 - Block SoC Resets of the Radio"]
    #[inline]
    pub fn block_soc_resets(&mut self) -> _BLOCK_SOC_RESETSW {
        _BLOCK_SOC_RESETSW { w: self }
    }
    #[doc = "Bit 29 - Block Radio Outputs"]
    #[inline]
    pub fn block_radio_outputs(&mut self) -> _BLOCK_RADIO_OUTPUTSW {
        _BLOCK_RADIO_OUTPUTSW { w: self }
    }
    #[doc = "Bit 30 - Allow the DFT Reset Pin to Reset the Radio"]
    #[inline]
    pub fn allow_dft_resets(&mut self) -> _ALLOW_DFT_RESETSW {
        _ALLOW_DFT_RESETSW { w: self }
    }
    #[doc = "Bit 31 - Software Reset for the Radio"]
    #[inline]
    pub fn radio_reset_bit(&mut self) -> _RADIO_RESET_BITW {
        _RADIO_RESET_BITW { w: self }
    }
}
