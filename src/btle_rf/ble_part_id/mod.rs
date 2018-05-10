#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::BLE_PART_ID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `BLE_PART_ID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLE_PART_IDR {
    #[doc = "Pre-production"]
    _0,
    #[doc = "Pre-production"]
    _1,
    #[doc = "KW40Z"]
    _2,
    #[doc = "KW41Z"]
    _3,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl BLE_PART_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            BLE_PART_IDR::_0 => 0,
            BLE_PART_IDR::_1 => 1,
            BLE_PART_IDR::_2 => 2,
            BLE_PART_IDR::_3 => 3,
            BLE_PART_IDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> BLE_PART_IDR {
        match value {
            0 => BLE_PART_IDR::_0,
            1 => BLE_PART_IDR::_1,
            2 => BLE_PART_IDR::_2,
            3 => BLE_PART_IDR::_3,
            i => BLE_PART_IDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BLE_PART_IDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BLE_PART_IDR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == BLE_PART_IDR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == BLE_PART_IDR::_3
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:15 - BLE Part ID"]
    #[inline]
    pub fn ble_part_id(&self) -> BLE_PART_IDR {
        BLE_PART_IDR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
        })
    }
}
