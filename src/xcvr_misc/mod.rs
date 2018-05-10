#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TRANSCEIVER CONTROL"]
    pub xcvr_ctrl: XCVR_CTRL,
    #[doc = "0x04 - TRANSCEIVER STATUS"]
    pub xcvr_status: XCVR_STATUS,
    #[doc = "0x08 - BLE ARBITRATION CONTROL"]
    pub ble_arb_ctrl: BLE_ARB_CTRL,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - OVERWRITE VERSION"]
    pub overwrite_ver: OVERWRITE_VER,
    #[doc = "0x14 - TRANSCEIVER DMA CONTROL"]
    pub dma_ctrl: DMA_CTRL,
    #[doc = "0x18 - TRANSCEIVER DMA DATA"]
    pub dma_data: DMA_DATA,
    #[doc = "0x1c - DIGITAL TEST MUX CONTROL"]
    pub dtest_ctrl: DTEST_CTRL,
    #[doc = "0x20 - PACKET RAM CONTROL"]
    pub packet_ram_ctrl: PACKET_RAM_CTRL,
    #[doc = "0x24 - FAD CONTROL"]
    pub fad_ctrl: FAD_CTRL,
    #[doc = "0x28 - LOW POWER PREAMBLE SEARCH CONTROL"]
    pub lpps_ctrl: LPPS_CTRL,
    #[doc = "0x2c - WIFI COEXISTENCE CONTROL"]
    pub rf_not_allowed_ctrl: RF_NOT_ALLOWED_CTRL,
    #[doc = "0x30 - CRC/WHITENER CONTROL"]
    pub crcw_cfg: CRCW_CFG,
    #[doc = "0x34 - CRC ERROR CORRECTION MASK"]
    pub crc_ec_mask: CRC_EC_MASK,
    #[doc = "0x38 - CRC RESULT"]
    pub crc_res_out: CRC_RES_OUT,
}
#[doc = "TRANSCEIVER CONTROL"]
pub struct XCVR_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRANSCEIVER CONTROL"]
pub mod xcvr_ctrl;
#[doc = "TRANSCEIVER STATUS"]
pub struct XCVR_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRANSCEIVER STATUS"]
pub mod xcvr_status;
#[doc = "BLE ARBITRATION CONTROL"]
pub struct BLE_ARB_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BLE ARBITRATION CONTROL"]
pub mod ble_arb_ctrl;
#[doc = "OVERWRITE VERSION"]
pub struct OVERWRITE_VER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OVERWRITE VERSION"]
pub mod overwrite_ver;
#[doc = "TRANSCEIVER DMA CONTROL"]
pub struct DMA_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRANSCEIVER DMA CONTROL"]
pub mod dma_ctrl;
#[doc = "TRANSCEIVER DMA DATA"]
pub struct DMA_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRANSCEIVER DMA DATA"]
pub mod dma_data;
#[doc = "DIGITAL TEST MUX CONTROL"]
pub struct DTEST_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DIGITAL TEST MUX CONTROL"]
pub mod dtest_ctrl;
#[doc = "PACKET RAM CONTROL"]
pub struct PACKET_RAM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PACKET RAM CONTROL"]
pub mod packet_ram_ctrl;
#[doc = "FAD CONTROL"]
pub struct FAD_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FAD CONTROL"]
pub mod fad_ctrl;
#[doc = "LOW POWER PREAMBLE SEARCH CONTROL"]
pub struct LPPS_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LOW POWER PREAMBLE SEARCH CONTROL"]
pub mod lpps_ctrl;
#[doc = "WIFI COEXISTENCE CONTROL"]
pub struct RF_NOT_ALLOWED_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WIFI COEXISTENCE CONTROL"]
pub mod rf_not_allowed_ctrl;
#[doc = "CRC/WHITENER CONTROL"]
pub struct CRCW_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC/WHITENER CONTROL"]
pub mod crcw_cfg;
#[doc = "CRC ERROR CORRECTION MASK"]
pub struct CRC_EC_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC ERROR CORRECTION MASK"]
pub mod crc_ec_mask;
#[doc = "CRC RESULT"]
pub struct CRC_RES_OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC RESULT"]
pub mod crc_res_out;
