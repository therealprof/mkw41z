#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HRS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `HRS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS0R {
    #[doc = "A hardware service request for channel 0 is not present"]
    _0,
    #[doc = "A hardware service request for channel 0 is present"]
    _1,
}
impl HRS0R {
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
            HRS0R::_0 => false,
            HRS0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS0R {
        match value {
            false => HRS0R::_0,
            true => HRS0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS0R::_1
    }
}
#[doc = "Possible values of the field `HRS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS1R {
    #[doc = "A hardware service request for channel 1 is not present"]
    _0,
    #[doc = "A hardware service request for channel 1 is present"]
    _1,
}
impl HRS1R {
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
            HRS1R::_0 => false,
            HRS1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS1R {
        match value {
            false => HRS1R::_0,
            true => HRS1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS1R::_1
    }
}
#[doc = "Possible values of the field `HRS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS2R {
    #[doc = "A hardware service request for channel 2 is not present"]
    _0,
    #[doc = "A hardware service request for channel 2 is present"]
    _1,
}
impl HRS2R {
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
            HRS2R::_0 => false,
            HRS2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS2R {
        match value {
            false => HRS2R::_0,
            true => HRS2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS2R::_1
    }
}
#[doc = "Possible values of the field `HRS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS3R {
    #[doc = "A hardware service request for channel 3 is not present"]
    _0,
    #[doc = "A hardware service request for channel 3 is present"]
    _1,
}
impl HRS3R {
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
            HRS3R::_0 => false,
            HRS3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRS3R {
        match value {
            false => HRS3R::_0,
            true => HRS3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRS3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRS3R::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline]
    pub fn hrs0(&self) -> HRS0R {
        HRS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline]
    pub fn hrs1(&self) -> HRS1R {
        HRS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline]
    pub fn hrs2(&self) -> HRS2R {
        HRS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline]
    pub fn hrs3(&self) -> HRS3R {
        HRS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
