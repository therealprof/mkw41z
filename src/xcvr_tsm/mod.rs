#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TRANSCEIVER SEQUENCE MANAGER CONTROL"]
    pub ctrl: CTRL,
    #[doc = "0x04 - TSM END OF SEQUENCE"]
    pub end_of_seq: END_OF_SEQ,
    #[doc = "0x08 - TSM OVERRIDE REGISTER 0"]
    pub ovrd0: OVRD0,
    #[doc = "0x0c - TSM OVERRIDE REGISTER 1"]
    pub ovrd1: OVRD1,
    #[doc = "0x10 - TSM OVERRIDE REGISTER 2"]
    pub ovrd2: OVRD2,
    #[doc = "0x14 - TSM OVERRIDE REGISTER 3"]
    pub ovrd3: OVRD3,
    #[doc = "0x18 - PA POWER"]
    pub pa_power: PA_POWER,
    #[doc = "0x1c - PA RAMP TABLE 0"]
    pub pa_ramp_tbl0: PA_RAMP_TBL0,
    #[doc = "0x20 - PA RAMP TABLE 1"]
    pub pa_ramp_tbl1: PA_RAMP_TBL1,
    #[doc = "0x24 - TSM RECYCLE COUNT"]
    pub recycle_count: RECYCLE_COUNT,
    #[doc = "0x28 - TSM FAST WARMUP CONTROL REGISTER 1"]
    pub fast_ctrl1: FAST_CTRL1,
    #[doc = "0x2c - TSM FAST WARMUP CONTROL REGISTER 2"]
    pub fast_ctrl2: FAST_CTRL2,
    #[doc = "0x30 - TSM_TIMING00"]
    pub timing00: TIMING00,
    #[doc = "0x34 - TSM_TIMING01"]
    pub timing01: TIMING01,
    #[doc = "0x38 - TSM_TIMING02"]
    pub timing02: TIMING02,
    #[doc = "0x3c - TSM_TIMING03"]
    pub timing03: TIMING03,
    #[doc = "0x40 - TSM_TIMING04"]
    pub timing04: TIMING04,
    #[doc = "0x44 - TSM_TIMING05"]
    pub timing05: TIMING05,
    #[doc = "0x48 - TSM_TIMING06"]
    pub timing06: TIMING06,
    #[doc = "0x4c - TSM_TIMING07"]
    pub timing07: TIMING07,
    #[doc = "0x50 - TSM_TIMING08"]
    pub timing08: TIMING08,
    #[doc = "0x54 - TSM_TIMING09"]
    pub timing09: TIMING09,
    #[doc = "0x58 - TSM_TIMING10"]
    pub timing10: TIMING10,
    #[doc = "0x5c - TSM_TIMING11"]
    pub timing11: TIMING11,
    #[doc = "0x60 - TSM_TIMING12"]
    pub timing12: TIMING12,
    #[doc = "0x64 - TSM_TIMING13"]
    pub timing13: TIMING13,
    #[doc = "0x68 - TSM_TIMING14"]
    pub timing14: TIMING14,
    #[doc = "0x6c - TSM_TIMING15"]
    pub timing15: TIMING15,
    #[doc = "0x70 - TSM_TIMING16"]
    pub timing16: TIMING16,
    #[doc = "0x74 - TSM_TIMING17"]
    pub timing17: TIMING17,
    #[doc = "0x78 - TSM_TIMING18"]
    pub timing18: TIMING18,
    #[doc = "0x7c - TSM_TIMING19"]
    pub timing19: TIMING19,
    #[doc = "0x80 - TSM_TIMING20"]
    pub timing20: TIMING20,
    #[doc = "0x84 - TSM_TIMING21"]
    pub timing21: TIMING21,
    #[doc = "0x88 - TSM_TIMING22"]
    pub timing22: TIMING22,
    #[doc = "0x8c - TSM_TIMING23"]
    pub timing23: TIMING23,
    #[doc = "0x90 - TSM_TIMING24"]
    pub timing24: TIMING24,
    #[doc = "0x94 - TSM_TIMING25"]
    pub timing25: TIMING25,
    #[doc = "0x98 - TSM_TIMING26"]
    pub timing26: TIMING26,
    #[doc = "0x9c - TSM_TIMING27"]
    pub timing27: TIMING27,
    #[doc = "0xa0 - TSM_TIMING28"]
    pub timing28: TIMING28,
    #[doc = "0xa4 - TSM_TIMING29"]
    pub timing29: TIMING29,
    #[doc = "0xa8 - TSM_TIMING30"]
    pub timing30: TIMING30,
    #[doc = "0xac - TSM_TIMING31"]
    pub timing31: TIMING31,
    #[doc = "0xb0 - TSM_TIMING32"]
    pub timing32: TIMING32,
    #[doc = "0xb4 - TSM_TIMING33"]
    pub timing33: TIMING33,
    #[doc = "0xb8 - TSM_TIMING34"]
    pub timing34: TIMING34,
    #[doc = "0xbc - TSM_TIMING35"]
    pub timing35: TIMING35,
    #[doc = "0xc0 - TSM_TIMING36"]
    pub timing36: TIMING36,
    #[doc = "0xc4 - TSM_TIMING37"]
    pub timing37: TIMING37,
    #[doc = "0xc8 - TSM_TIMING38"]
    pub timing38: TIMING38,
    #[doc = "0xcc - TSM_TIMING39"]
    pub timing39: TIMING39,
    #[doc = "0xd0 - TSM_TIMING40"]
    pub timing40: TIMING40,
    #[doc = "0xd4 - TSM_TIMING41"]
    pub timing41: TIMING41,
    #[doc = "0xd8 - TSM_TIMING42"]
    pub timing42: TIMING42,
    #[doc = "0xdc - TSM_TIMING43"]
    pub timing43: TIMING43,
    #[doc = "0xe0 - TSM_TIMING44"]
    pub timing44: TIMING44,
    #[doc = "0xe4 - TSM_TIMING45"]
    pub timing45: TIMING45,
    #[doc = "0xe8 - TSM_TIMING46"]
    pub timing46: TIMING46,
    #[doc = "0xec - TSM_TIMING47"]
    pub timing47: TIMING47,
    #[doc = "0xf0 - TSM_TIMING48"]
    pub timing48: TIMING48,
    #[doc = "0xf4 - TSM_TIMING49"]
    pub timing49: TIMING49,
    #[doc = "0xf8 - TSM_TIMING50"]
    pub timing50: TIMING50,
    #[doc = "0xfc - TSM_TIMING51"]
    pub timing51: TIMING51,
    #[doc = "0x100 - TSM_TIMING52"]
    pub timing52: TIMING52,
    #[doc = "0x104 - TSM_TIMING53"]
    pub timing53: TIMING53,
    #[doc = "0x108 - TSM_TIMING54"]
    pub timing54: TIMING54,
    #[doc = "0x10c - TSM_TIMING55"]
    pub timing55: TIMING55,
    #[doc = "0x110 - TSM_TIMING56"]
    pub timing56: TIMING56,
    #[doc = "0x114 - TSM_TIMING57"]
    pub timing57: TIMING57,
    #[doc = "0x118 - TSM_TIMING58"]
    pub timing58: TIMING58,
}
#[doc = "TRANSCEIVER SEQUENCE MANAGER CONTROL"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRANSCEIVER SEQUENCE MANAGER CONTROL"]
pub mod ctrl;
#[doc = "TSM END OF SEQUENCE"]
pub struct END_OF_SEQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM END OF SEQUENCE"]
pub mod end_of_seq;
#[doc = "TSM OVERRIDE REGISTER 0"]
pub struct OVRD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM OVERRIDE REGISTER 0"]
pub mod ovrd0;
#[doc = "TSM OVERRIDE REGISTER 1"]
pub struct OVRD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM OVERRIDE REGISTER 1"]
pub mod ovrd1;
#[doc = "TSM OVERRIDE REGISTER 2"]
pub struct OVRD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM OVERRIDE REGISTER 2"]
pub mod ovrd2;
#[doc = "TSM OVERRIDE REGISTER 3"]
pub struct OVRD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM OVERRIDE REGISTER 3"]
pub mod ovrd3;
#[doc = "PA POWER"]
pub struct PA_POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PA POWER"]
pub mod pa_power;
#[doc = "PA RAMP TABLE 0"]
pub struct PA_RAMP_TBL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PA RAMP TABLE 0"]
pub mod pa_ramp_tbl0;
#[doc = "PA RAMP TABLE 1"]
pub struct PA_RAMP_TBL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PA RAMP TABLE 1"]
pub mod pa_ramp_tbl1;
#[doc = "TSM RECYCLE COUNT"]
pub struct RECYCLE_COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM RECYCLE COUNT"]
pub mod recycle_count;
#[doc = "TSM FAST WARMUP CONTROL REGISTER 1"]
pub struct FAST_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM FAST WARMUP CONTROL REGISTER 1"]
pub mod fast_ctrl1;
#[doc = "TSM FAST WARMUP CONTROL REGISTER 2"]
pub struct FAST_CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM FAST WARMUP CONTROL REGISTER 2"]
pub mod fast_ctrl2;
#[doc = "TSM_TIMING00"]
pub struct TIMING00 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING00"]
pub mod timing00;
#[doc = "TSM_TIMING01"]
pub struct TIMING01 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING01"]
pub mod timing01;
#[doc = "TSM_TIMING02"]
pub struct TIMING02 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING02"]
pub mod timing02;
#[doc = "TSM_TIMING03"]
pub struct TIMING03 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING03"]
pub mod timing03;
#[doc = "TSM_TIMING04"]
pub struct TIMING04 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING04"]
pub mod timing04;
#[doc = "TSM_TIMING05"]
pub struct TIMING05 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING05"]
pub mod timing05;
#[doc = "TSM_TIMING06"]
pub struct TIMING06 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING06"]
pub mod timing06;
#[doc = "TSM_TIMING07"]
pub struct TIMING07 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING07"]
pub mod timing07;
#[doc = "TSM_TIMING08"]
pub struct TIMING08 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING08"]
pub mod timing08;
#[doc = "TSM_TIMING09"]
pub struct TIMING09 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING09"]
pub mod timing09;
#[doc = "TSM_TIMING10"]
pub struct TIMING10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING10"]
pub mod timing10;
#[doc = "TSM_TIMING11"]
pub struct TIMING11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING11"]
pub mod timing11;
#[doc = "TSM_TIMING12"]
pub struct TIMING12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING12"]
pub mod timing12;
#[doc = "TSM_TIMING13"]
pub struct TIMING13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING13"]
pub mod timing13;
#[doc = "TSM_TIMING14"]
pub struct TIMING14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING14"]
pub mod timing14;
#[doc = "TSM_TIMING15"]
pub struct TIMING15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING15"]
pub mod timing15;
#[doc = "TSM_TIMING16"]
pub struct TIMING16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING16"]
pub mod timing16;
#[doc = "TSM_TIMING17"]
pub struct TIMING17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING17"]
pub mod timing17;
#[doc = "TSM_TIMING18"]
pub struct TIMING18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING18"]
pub mod timing18;
#[doc = "TSM_TIMING19"]
pub struct TIMING19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING19"]
pub mod timing19;
#[doc = "TSM_TIMING20"]
pub struct TIMING20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING20"]
pub mod timing20;
#[doc = "TSM_TIMING21"]
pub struct TIMING21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING21"]
pub mod timing21;
#[doc = "TSM_TIMING22"]
pub struct TIMING22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING22"]
pub mod timing22;
#[doc = "TSM_TIMING23"]
pub struct TIMING23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING23"]
pub mod timing23;
#[doc = "TSM_TIMING24"]
pub struct TIMING24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING24"]
pub mod timing24;
#[doc = "TSM_TIMING25"]
pub struct TIMING25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING25"]
pub mod timing25;
#[doc = "TSM_TIMING26"]
pub struct TIMING26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING26"]
pub mod timing26;
#[doc = "TSM_TIMING27"]
pub struct TIMING27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING27"]
pub mod timing27;
#[doc = "TSM_TIMING28"]
pub struct TIMING28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING28"]
pub mod timing28;
#[doc = "TSM_TIMING29"]
pub struct TIMING29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING29"]
pub mod timing29;
#[doc = "TSM_TIMING30"]
pub struct TIMING30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING30"]
pub mod timing30;
#[doc = "TSM_TIMING31"]
pub struct TIMING31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING31"]
pub mod timing31;
#[doc = "TSM_TIMING32"]
pub struct TIMING32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING32"]
pub mod timing32;
#[doc = "TSM_TIMING33"]
pub struct TIMING33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING33"]
pub mod timing33;
#[doc = "TSM_TIMING34"]
pub struct TIMING34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING34"]
pub mod timing34;
#[doc = "TSM_TIMING35"]
pub struct TIMING35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING35"]
pub mod timing35;
#[doc = "TSM_TIMING36"]
pub struct TIMING36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING36"]
pub mod timing36;
#[doc = "TSM_TIMING37"]
pub struct TIMING37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING37"]
pub mod timing37;
#[doc = "TSM_TIMING38"]
pub struct TIMING38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING38"]
pub mod timing38;
#[doc = "TSM_TIMING39"]
pub struct TIMING39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING39"]
pub mod timing39;
#[doc = "TSM_TIMING40"]
pub struct TIMING40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING40"]
pub mod timing40;
#[doc = "TSM_TIMING41"]
pub struct TIMING41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING41"]
pub mod timing41;
#[doc = "TSM_TIMING42"]
pub struct TIMING42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING42"]
pub mod timing42;
#[doc = "TSM_TIMING43"]
pub struct TIMING43 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING43"]
pub mod timing43;
#[doc = "TSM_TIMING44"]
pub struct TIMING44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING44"]
pub mod timing44;
#[doc = "TSM_TIMING45"]
pub struct TIMING45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING45"]
pub mod timing45;
#[doc = "TSM_TIMING46"]
pub struct TIMING46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING46"]
pub mod timing46;
#[doc = "TSM_TIMING47"]
pub struct TIMING47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING47"]
pub mod timing47;
#[doc = "TSM_TIMING48"]
pub struct TIMING48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING48"]
pub mod timing48;
#[doc = "TSM_TIMING49"]
pub struct TIMING49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING49"]
pub mod timing49;
#[doc = "TSM_TIMING50"]
pub struct TIMING50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING50"]
pub mod timing50;
#[doc = "TSM_TIMING51"]
pub struct TIMING51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING51"]
pub mod timing51;
#[doc = "TSM_TIMING52"]
pub struct TIMING52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING52"]
pub mod timing52;
#[doc = "TSM_TIMING53"]
pub struct TIMING53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING53"]
pub mod timing53;
#[doc = "TSM_TIMING54"]
pub struct TIMING54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING54"]
pub mod timing54;
#[doc = "TSM_TIMING55"]
pub struct TIMING55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING55"]
pub mod timing55;
#[doc = "TSM_TIMING56"]
pub struct TIMING56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING56"]
pub mod timing56;
#[doc = "TSM_TIMING57"]
pub struct TIMING57 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING57"]
pub mod timing57;
#[doc = "TSM_TIMING58"]
pub struct TIMING58 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSM_TIMING58"]
pub mod timing58;
