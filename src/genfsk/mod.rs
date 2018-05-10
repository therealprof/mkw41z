#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IRQ CONTROL"]
    pub irq_ctrl: IRQ_CTRL,
    #[doc = "0x04 - EVENT TIMER"]
    pub event_tmr: EVENT_TMR,
    #[doc = "0x08 - T1 COMPARE"]
    pub t1_cmp: T1_CMP,
    #[doc = "0x0c - T2 COMPARE"]
    pub t2_cmp: T2_CMP,
    #[doc = "0x10 - TIMESTAMP"]
    pub timestamp: TIMESTAMP,
    #[doc = "0x14 - TRANSCEIVER CONTROL"]
    pub xcvr_ctrl: XCVR_CTRL,
    #[doc = "0x18 - TRANSCEIVER STATUS"]
    pub xcvr_sts: XCVR_STS,
    #[doc = "0x1c - TRANSCEIVER CONFIGURATION"]
    pub xcvr_cfg: XCVR_CFG,
    #[doc = "0x20 - CHANNEL NUMBER"]
    pub channel_num: CHANNEL_NUM,
    #[doc = "0x24 - TRANSMIT POWER"]
    pub tx_power: TX_POWER,
    #[doc = "0x28 - NETWORK ADDRESS CONTROL"]
    pub ntw_adr_ctrl: NTW_ADR_CTRL,
    #[doc = "0x2c - NETWORK ADDRESS 0"]
    pub ntw_adr_0: NTW_ADR_0,
    #[doc = "0x30 - NETWORK ADDRESS 1"]
    pub ntw_adr_1: NTW_ADR_1,
    #[doc = "0x34 - NETWORK ADDRESS 2"]
    pub ntw_adr_2: NTW_ADR_2,
    #[doc = "0x38 - NETWORK ADDRESS 3"]
    pub ntw_adr_3: NTW_ADR_3,
    #[doc = "0x3c - RECEIVE WATERMARK"]
    pub rx_watermark: RX_WATERMARK,
    #[doc = "0x40 - DSM CONTROL"]
    pub dsm_ctrl: DSM_CTRL,
    #[doc = "0x44 - PART ID"]
    pub part_id: PART_ID,
    _reserved0: [u8; 24usize],
    #[doc = "0x60 - PACKET CONFIGURATION"]
    pub packet_cfg: PACKET_CFG,
    #[doc = "0x64 - H0 CONFIGURATION"]
    pub h0_cfg: H0_CFG,
    #[doc = "0x68 - H1 CONFIGURATION"]
    pub h1_cfg: H1_CFG,
    #[doc = "0x6c - CRC CONFIGURATION"]
    pub crc_cfg: CRC_CFG,
    #[doc = "0x70 - CRC INITIALIZATION"]
    pub crc_init: CRC_INIT,
    #[doc = "0x74 - CRC POLYNOMIAL"]
    pub crc_poly: CRC_POLY,
    #[doc = "0x78 - CRC XOR OUT"]
    pub crc_xor_out: CRC_XOR_OUT,
    #[doc = "0x7c - WHITENER CONFIGURATION"]
    pub whiten_cfg: WHITEN_CFG,
    #[doc = "0x80 - WHITENER POLYNOMIAL"]
    pub whiten_poly: WHITEN_POLY,
    #[doc = "0x84 - WHITENER SIZE THRESHOLD"]
    pub whiten_sz_thr: WHITEN_SZ_THR,
    #[doc = "0x88 - BIT RATE"]
    pub bitrate: BITRATE,
    #[doc = "0x8c - PACKET BUFFER PARTITION POINT"]
    pub pb_partition: PB_PARTITION,
}
#[doc = "IRQ CONTROL"]
pub struct IRQ_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ CONTROL"]
pub mod irq_ctrl;
#[doc = "EVENT TIMER"]
pub struct EVENT_TMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EVENT TIMER"]
pub mod event_tmr;
#[doc = "T1 COMPARE"]
pub struct T1_CMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T1 COMPARE"]
pub mod t1_cmp;
#[doc = "T2 COMPARE"]
pub struct T2_CMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T2 COMPARE"]
pub mod t2_cmp;
#[doc = "TIMESTAMP"]
pub struct TIMESTAMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMESTAMP"]
pub mod timestamp;
#[doc = "TRANSCEIVER CONTROL"]
pub struct XCVR_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRANSCEIVER CONTROL"]
pub mod xcvr_ctrl;
#[doc = "TRANSCEIVER STATUS"]
pub struct XCVR_STS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRANSCEIVER STATUS"]
pub mod xcvr_sts;
#[doc = "TRANSCEIVER CONFIGURATION"]
pub struct XCVR_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRANSCEIVER CONFIGURATION"]
pub mod xcvr_cfg;
#[doc = "CHANNEL NUMBER"]
pub struct CHANNEL_NUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHANNEL NUMBER"]
pub mod channel_num;
#[doc = "TRANSMIT POWER"]
pub struct TX_POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRANSMIT POWER"]
pub mod tx_power;
#[doc = "NETWORK ADDRESS CONTROL"]
pub struct NTW_ADR_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NETWORK ADDRESS CONTROL"]
pub mod ntw_adr_ctrl;
#[doc = "NETWORK ADDRESS 0"]
pub struct NTW_ADR_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NETWORK ADDRESS 0"]
pub mod ntw_adr_0;
#[doc = "NETWORK ADDRESS 1"]
pub struct NTW_ADR_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NETWORK ADDRESS 1"]
pub mod ntw_adr_1;
#[doc = "NETWORK ADDRESS 2"]
pub struct NTW_ADR_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NETWORK ADDRESS 2"]
pub mod ntw_adr_2;
#[doc = "NETWORK ADDRESS 3"]
pub struct NTW_ADR_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NETWORK ADDRESS 3"]
pub mod ntw_adr_3;
#[doc = "RECEIVE WATERMARK"]
pub struct RX_WATERMARK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RECEIVE WATERMARK"]
pub mod rx_watermark;
#[doc = "DSM CONTROL"]
pub struct DSM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSM CONTROL"]
pub mod dsm_ctrl;
#[doc = "PART ID"]
pub struct PART_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PART ID"]
pub mod part_id;
#[doc = "PACKET CONFIGURATION"]
pub struct PACKET_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PACKET CONFIGURATION"]
pub mod packet_cfg;
#[doc = "H0 CONFIGURATION"]
pub struct H0_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "H0 CONFIGURATION"]
pub mod h0_cfg;
#[doc = "H1 CONFIGURATION"]
pub struct H1_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "H1 CONFIGURATION"]
pub mod h1_cfg;
#[doc = "CRC CONFIGURATION"]
pub struct CRC_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC CONFIGURATION"]
pub mod crc_cfg;
#[doc = "CRC INITIALIZATION"]
pub struct CRC_INIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC INITIALIZATION"]
pub mod crc_init;
#[doc = "CRC POLYNOMIAL"]
pub struct CRC_POLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC POLYNOMIAL"]
pub mod crc_poly;
#[doc = "CRC XOR OUT"]
pub struct CRC_XOR_OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC XOR OUT"]
pub mod crc_xor_out;
#[doc = "WHITENER CONFIGURATION"]
pub struct WHITEN_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WHITENER CONFIGURATION"]
pub mod whiten_cfg;
#[doc = "WHITENER POLYNOMIAL"]
pub struct WHITEN_POLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WHITENER POLYNOMIAL"]
pub mod whiten_poly;
#[doc = "WHITENER SIZE THRESHOLD"]
pub struct WHITEN_SZ_THR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WHITENER SIZE THRESHOLD"]
pub mod whiten_sz_thr;
#[doc = "BIT RATE"]
pub struct BITRATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BIT RATE"]
pub mod bitrate;
#[doc = "PACKET BUFFER PARTITION POINT"]
pub struct PB_PARTITION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PACKET BUFFER PARTITION POINT"]
pub mod pb_partition;
