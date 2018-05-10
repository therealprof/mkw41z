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
    #[doc = "0x3c - RX WATERMARK"]
    pub rx_watermark: RX_WATERMARK,
    #[doc = "0x40 - DSM CONTROL"]
    pub dsm_ctrl: DSM_CTRL,
    #[doc = "0x44 - PART ID"]
    pub part_id: PART_ID,
    _reserved0: [u8; 184usize],
    #[doc = "0x100 - PACKET BUFFER"]
    pub packet_buffer_0: PACKET_BUFFER_0,
    #[doc = "0x102 - PACKET BUFFER"]
    pub packet_buffer_1: PACKET_BUFFER_1,
    #[doc = "0x104 - PACKET BUFFER"]
    pub packet_buffer_2: PACKET_BUFFER_2,
    #[doc = "0x106 - PACKET BUFFER"]
    pub packet_buffer_3: PACKET_BUFFER_3,
    #[doc = "0x108 - PACKET BUFFER"]
    pub packet_buffer_4: PACKET_BUFFER_4,
    #[doc = "0x10a - PACKET BUFFER"]
    pub packet_buffer_5: PACKET_BUFFER_5,
    #[doc = "0x10c - PACKET BUFFER"]
    pub packet_buffer_6: PACKET_BUFFER_6,
    #[doc = "0x10e - PACKET BUFFER"]
    pub packet_buffer_7: PACKET_BUFFER_7,
    #[doc = "0x110 - PACKET BUFFER"]
    pub packet_buffer_8: PACKET_BUFFER_8,
    #[doc = "0x112 - PACKET BUFFER"]
    pub packet_buffer_9: PACKET_BUFFER_9,
    #[doc = "0x114 - PACKET BUFFER"]
    pub packet_buffer_10: PACKET_BUFFER_10,
    #[doc = "0x116 - PACKET BUFFER"]
    pub packet_buffer_11: PACKET_BUFFER_11,
    #[doc = "0x118 - PACKET BUFFER"]
    pub packet_buffer_12: PACKET_BUFFER_12,
    #[doc = "0x11a - PACKET BUFFER"]
    pub packet_buffer_13: PACKET_BUFFER_13,
    #[doc = "0x11c - PACKET BUFFER"]
    pub packet_buffer_14: PACKET_BUFFER_14,
    #[doc = "0x11e - PACKET BUFFER"]
    pub packet_buffer_15: PACKET_BUFFER_15,
    #[doc = "0x120 - PACKET BUFFER"]
    pub packet_buffer_16: PACKET_BUFFER_16,
    #[doc = "0x122 - PACKET BUFFER"]
    pub packet_buffer_17: PACKET_BUFFER_17,
    #[doc = "0x124 - PACKET BUFFER"]
    pub packet_buffer_18: PACKET_BUFFER_18,
    #[doc = "0x126 - PACKET BUFFER"]
    pub packet_buffer_19: PACKET_BUFFER_19,
    #[doc = "0x128 - PACKET BUFFER"]
    pub packet_buffer_20: PACKET_BUFFER_20,
    #[doc = "0x12a - PACKET BUFFER"]
    pub packet_buffer_21: PACKET_BUFFER_21,
    #[doc = "0x12c - PACKET BUFFER"]
    pub packet_buffer_22: PACKET_BUFFER_22,
    #[doc = "0x12e - PACKET BUFFER"]
    pub packet_buffer_23: PACKET_BUFFER_23,
    #[doc = "0x130 - PACKET BUFFER"]
    pub packet_buffer_24: PACKET_BUFFER_24,
    #[doc = "0x132 - PACKET BUFFER"]
    pub packet_buffer_25: PACKET_BUFFER_25,
    #[doc = "0x134 - PACKET BUFFER"]
    pub packet_buffer_26: PACKET_BUFFER_26,
    #[doc = "0x136 - PACKET BUFFER"]
    pub packet_buffer_27: PACKET_BUFFER_27,
    #[doc = "0x138 - PACKET BUFFER"]
    pub packet_buffer_28: PACKET_BUFFER_28,
    #[doc = "0x13a - PACKET BUFFER"]
    pub packet_buffer_29: PACKET_BUFFER_29,
    #[doc = "0x13c - PACKET BUFFER"]
    pub packet_buffer_30: PACKET_BUFFER_30,
    #[doc = "0x13e - PACKET BUFFER"]
    pub packet_buffer_31: PACKET_BUFFER_31,
    #[doc = "0x140 - PACKET BUFFER"]
    pub packet_buffer_32: PACKET_BUFFER_32,
    #[doc = "0x142 - PACKET BUFFER"]
    pub packet_buffer_33: PACKET_BUFFER_33,
    #[doc = "0x144 - PACKET BUFFER"]
    pub packet_buffer_34: PACKET_BUFFER_34,
    #[doc = "0x146 - PACKET BUFFER"]
    pub packet_buffer_35: PACKET_BUFFER_35,
    #[doc = "0x148 - PACKET BUFFER"]
    pub packet_buffer_36: PACKET_BUFFER_36,
    #[doc = "0x14a - PACKET BUFFER"]
    pub packet_buffer_37: PACKET_BUFFER_37,
    #[doc = "0x14c - PACKET BUFFER"]
    pub packet_buffer_38: PACKET_BUFFER_38,
    #[doc = "0x14e - PACKET BUFFER"]
    pub packet_buffer_39: PACKET_BUFFER_39,
    #[doc = "0x150 - PACKET BUFFER"]
    pub packet_buffer_40: PACKET_BUFFER_40,
    #[doc = "0x152 - PACKET BUFFER"]
    pub packet_buffer_41: PACKET_BUFFER_41,
    #[doc = "0x154 - PACKET BUFFER"]
    pub packet_buffer_42: PACKET_BUFFER_42,
    #[doc = "0x156 - PACKET BUFFER"]
    pub packet_buffer_43: PACKET_BUFFER_43,
    #[doc = "0x158 - PACKET BUFFER"]
    pub packet_buffer_44: PACKET_BUFFER_44,
    #[doc = "0x15a - PACKET BUFFER"]
    pub packet_buffer_45: PACKET_BUFFER_45,
    #[doc = "0x15c - PACKET BUFFER"]
    pub packet_buffer_46: PACKET_BUFFER_46,
    #[doc = "0x15e - PACKET BUFFER"]
    pub packet_buffer_47: PACKET_BUFFER_47,
    #[doc = "0x160 - PACKET BUFFER"]
    pub packet_buffer_48: PACKET_BUFFER_48,
    #[doc = "0x162 - PACKET BUFFER"]
    pub packet_buffer_49: PACKET_BUFFER_49,
    #[doc = "0x164 - PACKET BUFFER"]
    pub packet_buffer_50: PACKET_BUFFER_50,
    #[doc = "0x166 - PACKET BUFFER"]
    pub packet_buffer_51: PACKET_BUFFER_51,
    #[doc = "0x168 - PACKET BUFFER"]
    pub packet_buffer_52: PACKET_BUFFER_52,
    #[doc = "0x16a - PACKET BUFFER"]
    pub packet_buffer_53: PACKET_BUFFER_53,
    #[doc = "0x16c - PACKET BUFFER"]
    pub packet_buffer_54: PACKET_BUFFER_54,
    #[doc = "0x16e - PACKET BUFFER"]
    pub packet_buffer_55: PACKET_BUFFER_55,
    #[doc = "0x170 - PACKET BUFFER"]
    pub packet_buffer_56: PACKET_BUFFER_56,
    #[doc = "0x172 - PACKET BUFFER"]
    pub packet_buffer_57: PACKET_BUFFER_57,
    #[doc = "0x174 - PACKET BUFFER"]
    pub packet_buffer_58: PACKET_BUFFER_58,
    #[doc = "0x176 - PACKET BUFFER"]
    pub packet_buffer_59: PACKET_BUFFER_59,
    #[doc = "0x178 - PACKET BUFFER"]
    pub packet_buffer_60: PACKET_BUFFER_60,
    #[doc = "0x17a - PACKET BUFFER"]
    pub packet_buffer_61: PACKET_BUFFER_61,
    #[doc = "0x17c - PACKET BUFFER"]
    pub packet_buffer_62: PACKET_BUFFER_62,
    #[doc = "0x17e - PACKET BUFFER"]
    pub packet_buffer_63: PACKET_BUFFER_63,
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
#[doc = "RX WATERMARK"]
pub struct RX_WATERMARK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX WATERMARK"]
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
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_0;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_1;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_2;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_3;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_4;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_5;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_6 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_6;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_7 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_7;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_8 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_8;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_9 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_9;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_10 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_10;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_11 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_11;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_12 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_12;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_13 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_13;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_14 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_14;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_15 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_15;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_16 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_16;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_17 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_17;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_18 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_18;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_19 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_19;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_20 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_20;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_21 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_21;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_22 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_22;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_23 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_23;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_24 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_24;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_25 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_25;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_26 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_26;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_27 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_27;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_28 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_28;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_29 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_29;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_30 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_30;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_31 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_31;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_32 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_32;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_33 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_33;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_34 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_34;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_35 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_35;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_36 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_36;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_37 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_37;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_38 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_38;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_39 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_39;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_40 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_40;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_41 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_41;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_42 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_42;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_43 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_43;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_44 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_44;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_45 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_45;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_46 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_46;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_47 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_47;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_48 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_48;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_49 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_49;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_50 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_50;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_51 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_51;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_52 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_52;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_53 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_53;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_54 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_54;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_55 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_55;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_56 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_56;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_57 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_57;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_58 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_58;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_59 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_59;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_60 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_60;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_61 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_61;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_62 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_62;
#[doc = "PACKET BUFFER"]
pub struct PACKET_BUFFER_63 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PACKET BUFFER"]
pub mod packet_buffer_63;
