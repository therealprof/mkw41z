#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1536usize],
    #[doc = "0x600 - BLUETOOTH LOW ENERGY PART ID"]
    pub ble_part_id: BLE_PART_ID,
    _reserved1: [u8; 2usize],
    #[doc = "0x604 - BLE DSM STATUS"]
    pub dsm_status: DSM_STATUS,
    _reserved2: [u8; 2usize],
    #[doc = "0x608 - BLUETOOTH LOW ENERGY MISCELLANEOUS CONTROL"]
    pub misc_ctrl: MISC_CTRL,
}
#[doc = "BLUETOOTH LOW ENERGY PART ID"]
pub struct BLE_PART_ID {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "BLUETOOTH LOW ENERGY PART ID"]
pub mod ble_part_id;
#[doc = "BLE DSM STATUS"]
pub struct DSM_STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "BLE DSM STATUS"]
pub mod dsm_status;
#[doc = "BLUETOOTH LOW ENERGY MISCELLANEOUS CONTROL"]
pub struct MISC_CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "BLUETOOTH LOW ENERGY MISCELLANEOUS CONTROL"]
pub mod misc_ctrl;
