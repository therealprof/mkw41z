#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - INTERRUPT REQUEST STATUS"]
    pub irqsts: IRQSTS,
    #[doc = "0x04 - PHY CONTROL"]
    pub phy_ctrl: PHY_CTRL,
    #[doc = "0x08 - EVENT TIMER"]
    pub event_tmr: EVENT_TMR,
    #[doc = "0x0c - TIMESTAMP"]
    pub timestamp: TIMESTAMP,
    #[doc = "0x10 - T1 COMPARE"]
    pub t1cmp: T1CMP,
    #[doc = "0x14 - T2 COMPARE"]
    pub t2cmp: T2CMP,
    #[doc = "0x18 - T2 PRIME COMPARE"]
    pub t2primecmp: T2PRIMECMP,
    #[doc = "0x1c - T3 COMPARE"]
    pub t3cmp: T3CMP,
    #[doc = "0x20 - T4 COMPARE"]
    pub t4cmp: T4CMP,
    #[doc = "0x24 - PA POWER"]
    pub pa_pwr: PA_PWR,
    #[doc = "0x28 - CHANNEL NUMBER 0"]
    pub channel_num0: CHANNEL_NUM0,
    #[doc = "0x2c - LQI AND RSSI"]
    pub lqi_and_rssi: LQI_AND_RSSI,
    #[doc = "0x30 - MAC SHORT ADDRESS 0"]
    pub macshortaddrs0: MACSHORTADDRS0,
    #[doc = "0x34 - MAC LONG ADDRESS 0 LSB"]
    pub maclongaddrs0_lsb: MACLONGADDRS0_LSB,
    #[doc = "0x38 - MAC LONG ADDRESS 0 MSB"]
    pub maclongaddrs0_msb: MACLONGADDRS0_MSB,
    #[doc = "0x3c - RECEIVE FRAME FILTER"]
    pub rx_frame_filter: RX_FRAME_FILTER,
    #[doc = "0x40 - CCA AND LQI CONTROL"]
    pub cca_lqi_ctrl: CCA_LQI_CTRL,
    #[doc = "0x44 - CCA2 CONTROL"]
    pub cca2_ctrl: CCA2_CTRL,
    _reserved0: [u8; 4usize],
    #[doc = "0x4c - DSM CONTROL"]
    pub dsm_ctrl: DSM_CTRL,
    #[doc = "0x50 - BSM CONTROL"]
    pub bsm_ctrl: BSM_CTRL,
    #[doc = "0x54 - MAC SHORT ADDRESS FOR PAN1"]
    pub macshortaddrs1: MACSHORTADDRS1,
    #[doc = "0x58 - MAC LONG ADDRESS 1 LSB"]
    pub maclongaddrs1_lsb: MACLONGADDRS1_LSB,
    #[doc = "0x5c - MAC LONG ADDRESS 1 MSB"]
    pub maclongaddrs1_msb: MACLONGADDRS1_MSB,
    #[doc = "0x60 - DUAL PAN CONTROL"]
    pub dual_pan_ctrl: DUAL_PAN_CTRL,
    #[doc = "0x64 - CHANNEL NUMBER 1"]
    pub channel_num1: CHANNEL_NUM1,
    #[doc = "0x68 - SAM CONTROL"]
    pub sam_ctrl: SAM_CTRL,
    #[doc = "0x6c - SOURCE ADDRESS MANAGEMENT TABLE"]
    pub sam_table: SAM_TABLE,
    #[doc = "0x70 - SOURCE ADDRESS MANAGEMENT MATCH"]
    pub sam_match: SAM_MATCH,
    #[doc = "0x74 - SAM FREE INDEX"]
    pub sam_free_idx: SAM_FREE_IDX,
    #[doc = "0x78 - SEQUENCE CONTROL AND STATUS"]
    pub seq_ctrl_sts: SEQ_CTRL_STS,
    #[doc = "0x7c - ACK DELAY"]
    pub ackdelay: ACKDELAY,
    #[doc = "0x80 - FILTER FAIL CODE"]
    pub filterfail_code: FILTERFAIL_CODE,
    #[doc = "0x84 - RECEIVE WATER MARK"]
    pub rx_wtr_mark: RX_WTR_MARK,
    _reserved1: [u8; 4usize],
    #[doc = "0x8c - SLOT PRELOAD"]
    pub slot_preload: SLOT_PRELOAD,
    #[doc = "0x90 - 802.15.4 SEQUENCE STATE"]
    pub seq_state: SEQ_STATE,
    #[doc = "0x94 - TIMER PRESCALER"]
    pub tmr_prescale: TMR_PRESCALE,
    #[doc = "0x98 - LENIENCY LSB"]
    pub leniency_lsb: LENIENCY_LSB,
    #[doc = "0x9c - LENIENCY MSB"]
    pub leniency_msb: LENIENCY_MSB,
    #[doc = "0xa0 - PART ID"]
    pub part_id: PART_ID,
    _reserved2: [u8; 92usize],
    #[doc = "0x100 - Packet Buffer TX"]
    pub pkt_buffer_tx0: PKT_BUFFER_TX0,
    #[doc = "0x102 - Packet Buffer TX"]
    pub pkt_buffer_tx1: PKT_BUFFER_TX1,
    #[doc = "0x104 - Packet Buffer TX"]
    pub pkt_buffer_tx2: PKT_BUFFER_TX2,
    #[doc = "0x106 - Packet Buffer TX"]
    pub pkt_buffer_tx3: PKT_BUFFER_TX3,
    #[doc = "0x108 - Packet Buffer TX"]
    pub pkt_buffer_tx4: PKT_BUFFER_TX4,
    #[doc = "0x10a - Packet Buffer TX"]
    pub pkt_buffer_tx5: PKT_BUFFER_TX5,
    #[doc = "0x10c - Packet Buffer TX"]
    pub pkt_buffer_tx6: PKT_BUFFER_TX6,
    #[doc = "0x10e - Packet Buffer TX"]
    pub pkt_buffer_tx7: PKT_BUFFER_TX7,
    #[doc = "0x110 - Packet Buffer TX"]
    pub pkt_buffer_tx8: PKT_BUFFER_TX8,
    #[doc = "0x112 - Packet Buffer TX"]
    pub pkt_buffer_tx9: PKT_BUFFER_TX9,
    #[doc = "0x114 - Packet Buffer TX"]
    pub pkt_buffer_tx10: PKT_BUFFER_TX10,
    #[doc = "0x116 - Packet Buffer TX"]
    pub pkt_buffer_tx11: PKT_BUFFER_TX11,
    #[doc = "0x118 - Packet Buffer TX"]
    pub pkt_buffer_tx12: PKT_BUFFER_TX12,
    #[doc = "0x11a - Packet Buffer TX"]
    pub pkt_buffer_tx13: PKT_BUFFER_TX13,
    #[doc = "0x11c - Packet Buffer TX"]
    pub pkt_buffer_tx14: PKT_BUFFER_TX14,
    #[doc = "0x11e - Packet Buffer TX"]
    pub pkt_buffer_tx15: PKT_BUFFER_TX15,
    #[doc = "0x120 - Packet Buffer TX"]
    pub pkt_buffer_tx16: PKT_BUFFER_TX16,
    #[doc = "0x122 - Packet Buffer TX"]
    pub pkt_buffer_tx17: PKT_BUFFER_TX17,
    #[doc = "0x124 - Packet Buffer TX"]
    pub pkt_buffer_tx18: PKT_BUFFER_TX18,
    #[doc = "0x126 - Packet Buffer TX"]
    pub pkt_buffer_tx19: PKT_BUFFER_TX19,
    #[doc = "0x128 - Packet Buffer TX"]
    pub pkt_buffer_tx20: PKT_BUFFER_TX20,
    #[doc = "0x12a - Packet Buffer TX"]
    pub pkt_buffer_tx21: PKT_BUFFER_TX21,
    #[doc = "0x12c - Packet Buffer TX"]
    pub pkt_buffer_tx22: PKT_BUFFER_TX22,
    #[doc = "0x12e - Packet Buffer TX"]
    pub pkt_buffer_tx23: PKT_BUFFER_TX23,
    #[doc = "0x130 - Packet Buffer TX"]
    pub pkt_buffer_tx24: PKT_BUFFER_TX24,
    #[doc = "0x132 - Packet Buffer TX"]
    pub pkt_buffer_tx25: PKT_BUFFER_TX25,
    #[doc = "0x134 - Packet Buffer TX"]
    pub pkt_buffer_tx26: PKT_BUFFER_TX26,
    #[doc = "0x136 - Packet Buffer TX"]
    pub pkt_buffer_tx27: PKT_BUFFER_TX27,
    #[doc = "0x138 - Packet Buffer TX"]
    pub pkt_buffer_tx28: PKT_BUFFER_TX28,
    #[doc = "0x13a - Packet Buffer TX"]
    pub pkt_buffer_tx29: PKT_BUFFER_TX29,
    #[doc = "0x13c - Packet Buffer TX"]
    pub pkt_buffer_tx30: PKT_BUFFER_TX30,
    #[doc = "0x13e - Packet Buffer TX"]
    pub pkt_buffer_tx31: PKT_BUFFER_TX31,
    #[doc = "0x140 - Packet Buffer TX"]
    pub pkt_buffer_tx32: PKT_BUFFER_TX32,
    #[doc = "0x142 - Packet Buffer TX"]
    pub pkt_buffer_tx33: PKT_BUFFER_TX33,
    #[doc = "0x144 - Packet Buffer TX"]
    pub pkt_buffer_tx34: PKT_BUFFER_TX34,
    #[doc = "0x146 - Packet Buffer TX"]
    pub pkt_buffer_tx35: PKT_BUFFER_TX35,
    #[doc = "0x148 - Packet Buffer TX"]
    pub pkt_buffer_tx36: PKT_BUFFER_TX36,
    #[doc = "0x14a - Packet Buffer TX"]
    pub pkt_buffer_tx37: PKT_BUFFER_TX37,
    #[doc = "0x14c - Packet Buffer TX"]
    pub pkt_buffer_tx38: PKT_BUFFER_TX38,
    #[doc = "0x14e - Packet Buffer TX"]
    pub pkt_buffer_tx39: PKT_BUFFER_TX39,
    #[doc = "0x150 - Packet Buffer TX"]
    pub pkt_buffer_tx40: PKT_BUFFER_TX40,
    #[doc = "0x152 - Packet Buffer TX"]
    pub pkt_buffer_tx41: PKT_BUFFER_TX41,
    #[doc = "0x154 - Packet Buffer TX"]
    pub pkt_buffer_tx42: PKT_BUFFER_TX42,
    #[doc = "0x156 - Packet Buffer TX"]
    pub pkt_buffer_tx43: PKT_BUFFER_TX43,
    #[doc = "0x158 - Packet Buffer TX"]
    pub pkt_buffer_tx44: PKT_BUFFER_TX44,
    #[doc = "0x15a - Packet Buffer TX"]
    pub pkt_buffer_tx45: PKT_BUFFER_TX45,
    #[doc = "0x15c - Packet Buffer TX"]
    pub pkt_buffer_tx46: PKT_BUFFER_TX46,
    #[doc = "0x15e - Packet Buffer TX"]
    pub pkt_buffer_tx47: PKT_BUFFER_TX47,
    #[doc = "0x160 - Packet Buffer TX"]
    pub pkt_buffer_tx48: PKT_BUFFER_TX48,
    #[doc = "0x162 - Packet Buffer TX"]
    pub pkt_buffer_tx49: PKT_BUFFER_TX49,
    #[doc = "0x164 - Packet Buffer TX"]
    pub pkt_buffer_tx50: PKT_BUFFER_TX50,
    #[doc = "0x166 - Packet Buffer TX"]
    pub pkt_buffer_tx51: PKT_BUFFER_TX51,
    #[doc = "0x168 - Packet Buffer TX"]
    pub pkt_buffer_tx52: PKT_BUFFER_TX52,
    #[doc = "0x16a - Packet Buffer TX"]
    pub pkt_buffer_tx53: PKT_BUFFER_TX53,
    #[doc = "0x16c - Packet Buffer TX"]
    pub pkt_buffer_tx54: PKT_BUFFER_TX54,
    #[doc = "0x16e - Packet Buffer TX"]
    pub pkt_buffer_tx55: PKT_BUFFER_TX55,
    #[doc = "0x170 - Packet Buffer TX"]
    pub pkt_buffer_tx56: PKT_BUFFER_TX56,
    #[doc = "0x172 - Packet Buffer TX"]
    pub pkt_buffer_tx57: PKT_BUFFER_TX57,
    #[doc = "0x174 - Packet Buffer TX"]
    pub pkt_buffer_tx58: PKT_BUFFER_TX58,
    #[doc = "0x176 - Packet Buffer TX"]
    pub pkt_buffer_tx59: PKT_BUFFER_TX59,
    #[doc = "0x178 - Packet Buffer TX"]
    pub pkt_buffer_tx60: PKT_BUFFER_TX60,
    #[doc = "0x17a - Packet Buffer TX"]
    pub pkt_buffer_tx61: PKT_BUFFER_TX61,
    #[doc = "0x17c - Packet Buffer TX"]
    pub pkt_buffer_tx62: PKT_BUFFER_TX62,
    #[doc = "0x17e - Packet Buffer TX"]
    pub pkt_buffer_tx63: PKT_BUFFER_TX63,
    #[doc = "0x180 - Packet Buffer RX"]
    pub pkt_buffer_rx0: PKT_BUFFER_RX0,
    #[doc = "0x182 - Packet Buffer RX"]
    pub pkt_buffer_rx1: PKT_BUFFER_RX1,
    #[doc = "0x184 - Packet Buffer RX"]
    pub pkt_buffer_rx2: PKT_BUFFER_RX2,
    #[doc = "0x186 - Packet Buffer RX"]
    pub pkt_buffer_rx3: PKT_BUFFER_RX3,
    #[doc = "0x188 - Packet Buffer RX"]
    pub pkt_buffer_rx4: PKT_BUFFER_RX4,
    #[doc = "0x18a - Packet Buffer RX"]
    pub pkt_buffer_rx5: PKT_BUFFER_RX5,
    #[doc = "0x18c - Packet Buffer RX"]
    pub pkt_buffer_rx6: PKT_BUFFER_RX6,
    #[doc = "0x18e - Packet Buffer RX"]
    pub pkt_buffer_rx7: PKT_BUFFER_RX7,
    #[doc = "0x190 - Packet Buffer RX"]
    pub pkt_buffer_rx8: PKT_BUFFER_RX8,
    #[doc = "0x192 - Packet Buffer RX"]
    pub pkt_buffer_rx9: PKT_BUFFER_RX9,
    #[doc = "0x194 - Packet Buffer RX"]
    pub pkt_buffer_rx10: PKT_BUFFER_RX10,
    #[doc = "0x196 - Packet Buffer RX"]
    pub pkt_buffer_rx11: PKT_BUFFER_RX11,
    #[doc = "0x198 - Packet Buffer RX"]
    pub pkt_buffer_rx12: PKT_BUFFER_RX12,
    #[doc = "0x19a - Packet Buffer RX"]
    pub pkt_buffer_rx13: PKT_BUFFER_RX13,
    #[doc = "0x19c - Packet Buffer RX"]
    pub pkt_buffer_rx14: PKT_BUFFER_RX14,
    #[doc = "0x19e - Packet Buffer RX"]
    pub pkt_buffer_rx15: PKT_BUFFER_RX15,
    #[doc = "0x1a0 - Packet Buffer RX"]
    pub pkt_buffer_rx16: PKT_BUFFER_RX16,
    #[doc = "0x1a2 - Packet Buffer RX"]
    pub pkt_buffer_rx17: PKT_BUFFER_RX17,
    #[doc = "0x1a4 - Packet Buffer RX"]
    pub pkt_buffer_rx18: PKT_BUFFER_RX18,
    #[doc = "0x1a6 - Packet Buffer RX"]
    pub pkt_buffer_rx19: PKT_BUFFER_RX19,
    #[doc = "0x1a8 - Packet Buffer RX"]
    pub pkt_buffer_rx20: PKT_BUFFER_RX20,
    #[doc = "0x1aa - Packet Buffer RX"]
    pub pkt_buffer_rx21: PKT_BUFFER_RX21,
    #[doc = "0x1ac - Packet Buffer RX"]
    pub pkt_buffer_rx22: PKT_BUFFER_RX22,
    #[doc = "0x1ae - Packet Buffer RX"]
    pub pkt_buffer_rx23: PKT_BUFFER_RX23,
    #[doc = "0x1b0 - Packet Buffer RX"]
    pub pkt_buffer_rx24: PKT_BUFFER_RX24,
    #[doc = "0x1b2 - Packet Buffer RX"]
    pub pkt_buffer_rx25: PKT_BUFFER_RX25,
    #[doc = "0x1b4 - Packet Buffer RX"]
    pub pkt_buffer_rx26: PKT_BUFFER_RX26,
    #[doc = "0x1b6 - Packet Buffer RX"]
    pub pkt_buffer_rx27: PKT_BUFFER_RX27,
    #[doc = "0x1b8 - Packet Buffer RX"]
    pub pkt_buffer_rx28: PKT_BUFFER_RX28,
    #[doc = "0x1ba - Packet Buffer RX"]
    pub pkt_buffer_rx29: PKT_BUFFER_RX29,
    #[doc = "0x1bc - Packet Buffer RX"]
    pub pkt_buffer_rx30: PKT_BUFFER_RX30,
    #[doc = "0x1be - Packet Buffer RX"]
    pub pkt_buffer_rx31: PKT_BUFFER_RX31,
    #[doc = "0x1c0 - Packet Buffer RX"]
    pub pkt_buffer_rx32: PKT_BUFFER_RX32,
    #[doc = "0x1c2 - Packet Buffer RX"]
    pub pkt_buffer_rx33: PKT_BUFFER_RX33,
    #[doc = "0x1c4 - Packet Buffer RX"]
    pub pkt_buffer_rx34: PKT_BUFFER_RX34,
    #[doc = "0x1c6 - Packet Buffer RX"]
    pub pkt_buffer_rx35: PKT_BUFFER_RX35,
    #[doc = "0x1c8 - Packet Buffer RX"]
    pub pkt_buffer_rx36: PKT_BUFFER_RX36,
    #[doc = "0x1ca - Packet Buffer RX"]
    pub pkt_buffer_rx37: PKT_BUFFER_RX37,
    #[doc = "0x1cc - Packet Buffer RX"]
    pub pkt_buffer_rx38: PKT_BUFFER_RX38,
    #[doc = "0x1ce - Packet Buffer RX"]
    pub pkt_buffer_rx39: PKT_BUFFER_RX39,
    #[doc = "0x1d0 - Packet Buffer RX"]
    pub pkt_buffer_rx40: PKT_BUFFER_RX40,
    #[doc = "0x1d2 - Packet Buffer RX"]
    pub pkt_buffer_rx41: PKT_BUFFER_RX41,
    #[doc = "0x1d4 - Packet Buffer RX"]
    pub pkt_buffer_rx42: PKT_BUFFER_RX42,
    #[doc = "0x1d6 - Packet Buffer RX"]
    pub pkt_buffer_rx43: PKT_BUFFER_RX43,
    #[doc = "0x1d8 - Packet Buffer RX"]
    pub pkt_buffer_rx44: PKT_BUFFER_RX44,
    #[doc = "0x1da - Packet Buffer RX"]
    pub pkt_buffer_rx45: PKT_BUFFER_RX45,
    #[doc = "0x1dc - Packet Buffer RX"]
    pub pkt_buffer_rx46: PKT_BUFFER_RX46,
    #[doc = "0x1de - Packet Buffer RX"]
    pub pkt_buffer_rx47: PKT_BUFFER_RX47,
    #[doc = "0x1e0 - Packet Buffer RX"]
    pub pkt_buffer_rx48: PKT_BUFFER_RX48,
    #[doc = "0x1e2 - Packet Buffer RX"]
    pub pkt_buffer_rx49: PKT_BUFFER_RX49,
    #[doc = "0x1e4 - Packet Buffer RX"]
    pub pkt_buffer_rx50: PKT_BUFFER_RX50,
    #[doc = "0x1e6 - Packet Buffer RX"]
    pub pkt_buffer_rx51: PKT_BUFFER_RX51,
    #[doc = "0x1e8 - Packet Buffer RX"]
    pub pkt_buffer_rx52: PKT_BUFFER_RX52,
    #[doc = "0x1ea - Packet Buffer RX"]
    pub pkt_buffer_rx53: PKT_BUFFER_RX53,
    #[doc = "0x1ec - Packet Buffer RX"]
    pub pkt_buffer_rx54: PKT_BUFFER_RX54,
    #[doc = "0x1ee - Packet Buffer RX"]
    pub pkt_buffer_rx55: PKT_BUFFER_RX55,
    #[doc = "0x1f0 - Packet Buffer RX"]
    pub pkt_buffer_rx56: PKT_BUFFER_RX56,
    #[doc = "0x1f2 - Packet Buffer RX"]
    pub pkt_buffer_rx57: PKT_BUFFER_RX57,
    #[doc = "0x1f4 - Packet Buffer RX"]
    pub pkt_buffer_rx58: PKT_BUFFER_RX58,
    #[doc = "0x1f6 - Packet Buffer RX"]
    pub pkt_buffer_rx59: PKT_BUFFER_RX59,
    #[doc = "0x1f8 - Packet Buffer RX"]
    pub pkt_buffer_rx60: PKT_BUFFER_RX60,
    #[doc = "0x1fa - Packet Buffer RX"]
    pub pkt_buffer_rx61: PKT_BUFFER_RX61,
    #[doc = "0x1fc - Packet Buffer RX"]
    pub pkt_buffer_rx62: PKT_BUFFER_RX62,
    #[doc = "0x1fe - Packet Buffer RX"]
    pub pkt_buffer_rx63: PKT_BUFFER_RX63,
}
#[doc = "INTERRUPT REQUEST STATUS"]
pub struct IRQSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTERRUPT REQUEST STATUS"]
pub mod irqsts;
#[doc = "PHY CONTROL"]
pub struct PHY_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PHY CONTROL"]
pub mod phy_ctrl;
#[doc = "EVENT TIMER"]
pub struct EVENT_TMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EVENT TIMER"]
pub mod event_tmr;
#[doc = "TIMESTAMP"]
pub struct TIMESTAMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMESTAMP"]
pub mod timestamp;
#[doc = "T1 COMPARE"]
pub struct T1CMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T1 COMPARE"]
pub mod t1cmp;
#[doc = "T2 COMPARE"]
pub struct T2CMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T2 COMPARE"]
pub mod t2cmp;
#[doc = "T2 PRIME COMPARE"]
pub struct T2PRIMECMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T2 PRIME COMPARE"]
pub mod t2primecmp;
#[doc = "T3 COMPARE"]
pub struct T3CMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T3 COMPARE"]
pub mod t3cmp;
#[doc = "T4 COMPARE"]
pub struct T4CMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T4 COMPARE"]
pub mod t4cmp;
#[doc = "PA POWER"]
pub struct PA_PWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PA POWER"]
pub mod pa_pwr;
#[doc = "CHANNEL NUMBER 0"]
pub struct CHANNEL_NUM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHANNEL NUMBER 0"]
pub mod channel_num0;
#[doc = "LQI AND RSSI"]
pub struct LQI_AND_RSSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LQI AND RSSI"]
pub mod lqi_and_rssi;
#[doc = "MAC SHORT ADDRESS 0"]
pub struct MACSHORTADDRS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC SHORT ADDRESS 0"]
pub mod macshortaddrs0;
#[doc = "MAC LONG ADDRESS 0 LSB"]
pub struct MACLONGADDRS0_LSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC LONG ADDRESS 0 LSB"]
pub mod maclongaddrs0_lsb;
#[doc = "MAC LONG ADDRESS 0 MSB"]
pub struct MACLONGADDRS0_MSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC LONG ADDRESS 0 MSB"]
pub mod maclongaddrs0_msb;
#[doc = "RECEIVE FRAME FILTER"]
pub struct RX_FRAME_FILTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RECEIVE FRAME FILTER"]
pub mod rx_frame_filter;
#[doc = "CCA AND LQI CONTROL"]
pub struct CCA_LQI_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCA AND LQI CONTROL"]
pub mod cca_lqi_ctrl;
#[doc = "CCA2 CONTROL"]
pub struct CCA2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCA2 CONTROL"]
pub mod cca2_ctrl;
#[doc = "DSM CONTROL"]
pub struct DSM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSM CONTROL"]
pub mod dsm_ctrl;
#[doc = "BSM CONTROL"]
pub struct BSM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BSM CONTROL"]
pub mod bsm_ctrl;
#[doc = "MAC SHORT ADDRESS FOR PAN1"]
pub struct MACSHORTADDRS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC SHORT ADDRESS FOR PAN1"]
pub mod macshortaddrs1;
#[doc = "MAC LONG ADDRESS 1 LSB"]
pub struct MACLONGADDRS1_LSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC LONG ADDRESS 1 LSB"]
pub mod maclongaddrs1_lsb;
#[doc = "MAC LONG ADDRESS 1 MSB"]
pub struct MACLONGADDRS1_MSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC LONG ADDRESS 1 MSB"]
pub mod maclongaddrs1_msb;
#[doc = "DUAL PAN CONTROL"]
pub struct DUAL_PAN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DUAL PAN CONTROL"]
pub mod dual_pan_ctrl;
#[doc = "CHANNEL NUMBER 1"]
pub struct CHANNEL_NUM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHANNEL NUMBER 1"]
pub mod channel_num1;
#[doc = "SAM CONTROL"]
pub struct SAM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAM CONTROL"]
pub mod sam_ctrl;
#[doc = "SOURCE ADDRESS MANAGEMENT TABLE"]
pub struct SAM_TABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SOURCE ADDRESS MANAGEMENT TABLE"]
pub mod sam_table;
#[doc = "SOURCE ADDRESS MANAGEMENT MATCH"]
pub struct SAM_MATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SOURCE ADDRESS MANAGEMENT MATCH"]
pub mod sam_match;
#[doc = "SAM FREE INDEX"]
pub struct SAM_FREE_IDX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAM FREE INDEX"]
pub mod sam_free_idx;
#[doc = "SEQUENCE CONTROL AND STATUS"]
pub struct SEQ_CTRL_STS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SEQUENCE CONTROL AND STATUS"]
pub mod seq_ctrl_sts;
#[doc = "ACK DELAY"]
pub struct ACKDELAY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ACK DELAY"]
pub mod ackdelay;
#[doc = "FILTER FAIL CODE"]
pub struct FILTERFAIL_CODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FILTER FAIL CODE"]
pub mod filterfail_code;
#[doc = "RECEIVE WATER MARK"]
pub struct RX_WTR_MARK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RECEIVE WATER MARK"]
pub mod rx_wtr_mark;
#[doc = "SLOT PRELOAD"]
pub struct SLOT_PRELOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SLOT PRELOAD"]
pub mod slot_preload;
#[doc = "802.15.4 SEQUENCE STATE"]
pub struct SEQ_STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "802.15.4 SEQUENCE STATE"]
pub mod seq_state;
#[doc = "TIMER PRESCALER"]
pub struct TMR_PRESCALE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMER PRESCALER"]
pub mod tmr_prescale;
#[doc = "LENIENCY LSB"]
pub struct LENIENCY_LSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LENIENCY LSB"]
pub mod leniency_lsb;
#[doc = "LENIENCY MSB"]
pub struct LENIENCY_MSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LENIENCY MSB"]
pub mod leniency_msb;
#[doc = "PART ID"]
pub struct PART_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PART ID"]
pub mod part_id;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx0;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx1;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx2;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx3;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx4;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx5;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX6 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx6;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX7 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx7;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX8 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx8;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX9 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx9;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX10 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx10;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX11 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx11;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX12 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx12;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX13 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx13;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX14 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx14;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX15 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx15;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX16 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx16;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX17 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx17;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX18 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx18;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX19 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx19;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX20 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx20;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX21 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx21;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX22 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx22;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX23 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx23;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX24 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx24;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX25 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx25;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX26 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx26;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX27 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx27;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX28 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx28;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX29 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx29;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX30 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx30;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX31 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx31;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX32 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx32;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX33 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx33;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX34 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx34;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX35 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx35;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX36 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx36;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX37 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx37;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX38 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx38;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX39 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx39;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX40 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx40;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX41 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx41;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX42 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx42;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX43 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx43;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX44 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx44;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX45 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx45;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX46 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx46;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX47 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx47;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX48 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx48;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX49 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx49;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX50 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx50;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX51 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx51;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX52 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx52;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX53 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx53;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX54 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx54;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX55 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx55;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX56 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx56;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX57 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx57;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX58 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx58;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX59 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx59;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX60 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx60;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX61 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx61;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX62 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx62;
#[doc = "Packet Buffer TX"]
pub struct PKT_BUFFER_TX63 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer TX"]
pub mod pkt_buffer_tx63;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx0;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx1;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx2;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx3;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx4;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx5;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX6 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx6;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX7 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx7;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX8 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx8;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX9 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx9;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX10 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx10;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX11 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx11;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX12 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx12;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX13 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx13;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX14 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx14;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX15 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx15;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX16 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx16;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX17 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx17;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX18 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx18;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX19 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx19;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX20 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx20;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX21 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx21;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX22 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx22;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX23 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx23;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX24 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx24;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX25 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx25;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX26 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx26;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX27 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx27;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX28 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx28;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX29 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx29;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX30 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx30;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX31 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx31;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX32 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx32;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX33 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx33;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX34 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx34;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX35 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx35;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX36 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx36;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX37 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx37;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX38 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx38;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX39 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx39;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX40 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx40;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX41 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx41;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX42 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx42;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX43 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx43;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX44 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx44;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX45 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx45;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX46 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx46;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX47 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx47;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX48 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx48;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX49 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx49;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX50 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx50;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX51 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx51;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX52 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx52;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX53 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx53;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX54 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx54;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX55 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx55;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX56 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx56;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX57 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx57;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX58 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx58;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX59 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx59;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX60 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx60;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX61 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx61;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX62 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx62;
#[doc = "Packet Buffer RX"]
pub struct PKT_BUFFER_RX63 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Packet Buffer RX"]
pub mod pkt_buffer_rx63;
