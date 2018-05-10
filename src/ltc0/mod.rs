#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub md: MD,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Key Size Register"]
    pub ks: KS,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - Data Size Register"]
    pub ds: DS,
    _reserved2: [u8; 4usize],
    #[doc = "0x18 - ICV Size Register"]
    pub icvs: ICVS,
    _reserved3: [u8; 20usize],
    #[doc = "0x30 - Command Register"]
    pub com: COM,
    #[doc = "0x34 - Control Register"]
    pub ctl: CTL,
    _reserved4: [u8; 8usize],
    #[doc = "0x40 - Clear Written Register"]
    pub cw: CW,
    _reserved5: [u8; 4usize],
    #[doc = "0x48 - Status Register"]
    pub sta: STA,
    #[doc = "0x4c - Error Status Register"]
    pub esta: ESTA,
    _reserved6: [u8; 8usize],
    #[doc = "0x58 - AAD Size Register"]
    pub aadsz: AADSZ,
    _reserved7: [u8; 164usize],
    #[doc = "0x100 - Context Register"]
    pub ctx_0: CTX_0,
    #[doc = "0x104 - Context Register"]
    pub ctx_1: CTX_1,
    #[doc = "0x108 - Context Register"]
    pub ctx_2: CTX_2,
    #[doc = "0x10c - Context Register"]
    pub ctx_3: CTX_3,
    #[doc = "0x110 - Context Register"]
    pub ctx_4: CTX_4,
    #[doc = "0x114 - Context Register"]
    pub ctx_5: CTX_5,
    #[doc = "0x118 - Context Register"]
    pub ctx_6: CTX_6,
    #[doc = "0x11c - Context Register"]
    pub ctx_7: CTX_7,
    #[doc = "0x120 - Context Register"]
    pub ctx_8: CTX_8,
    #[doc = "0x124 - Context Register"]
    pub ctx_9: CTX_9,
    #[doc = "0x128 - Context Register"]
    pub ctx_10: CTX_10,
    #[doc = "0x12c - Context Register"]
    pub ctx_11: CTX_11,
    #[doc = "0x130 - Context Register"]
    pub ctx_12: CTX_12,
    #[doc = "0x134 - Context Register"]
    pub ctx_13: CTX_13,
    _reserved8: [u8; 200usize],
    #[doc = "0x200 - Key Registers"]
    pub key_0: KEY_0,
    #[doc = "0x204 - Key Registers"]
    pub key_1: KEY_1,
    #[doc = "0x208 - Key Registers"]
    pub key_2: KEY_2,
    #[doc = "0x20c - Key Registers"]
    pub key_3: KEY_3,
    _reserved9: [u8; 736usize],
    #[doc = "0x4f0 - Version ID Register"]
    pub vid1: VID1,
    #[doc = "0x4f4 - Version ID 2 Register"]
    pub vid2: VID2,
    #[doc = "0x4f8 - CHA Version ID Register"]
    pub chavid: CHAVID,
    _reserved10: [u8; 708usize],
    #[doc = "0x7c0 - FIFO Status Register"]
    pub fifosta: FIFOSTA,
    _reserved11: [u8; 28usize],
    #[doc = "0x7e0 - Input Data FIFO"]
    pub ififo: IFIFO,
    _reserved12: [u8; 12usize],
    #[doc = "0x7f0 - Output Data FIFO"]
    pub ofifo: OFIFO,
}
#[doc = "Mode Register"]
pub struct MD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod md;
#[doc = "Key Size Register"]
pub struct KS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Size Register"]
pub mod ks;
#[doc = "Data Size Register"]
pub struct DS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Size Register"]
pub mod ds;
#[doc = "ICV Size Register"]
pub struct ICVS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ICV Size Register"]
pub mod icvs;
#[doc = "Command Register"]
pub struct COM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod com;
#[doc = "Control Register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctl;
#[doc = "Clear Written Register"]
pub struct CW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Written Register"]
pub mod cw;
#[doc = "Status Register"]
pub struct STA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod sta;
#[doc = "Error Status Register"]
pub struct ESTA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Status Register"]
pub mod esta;
#[doc = "AAD Size Register"]
pub struct AADSZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AAD Size Register"]
pub mod aadsz;
#[doc = "Context Register"]
pub struct CTX_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_0;
#[doc = "Context Register"]
pub struct CTX_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_1;
#[doc = "Context Register"]
pub struct CTX_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_2;
#[doc = "Context Register"]
pub struct CTX_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_3;
#[doc = "Context Register"]
pub struct CTX_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_4;
#[doc = "Context Register"]
pub struct CTX_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_5;
#[doc = "Context Register"]
pub struct CTX_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_6;
#[doc = "Context Register"]
pub struct CTX_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_7;
#[doc = "Context Register"]
pub struct CTX_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_8;
#[doc = "Context Register"]
pub struct CTX_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_9;
#[doc = "Context Register"]
pub struct CTX_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_10;
#[doc = "Context Register"]
pub struct CTX_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_11;
#[doc = "Context Register"]
pub struct CTX_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_12;
#[doc = "Context Register"]
pub struct CTX_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Context Register"]
pub mod ctx_13;
#[doc = "Key Registers"]
pub struct KEY_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Registers"]
pub mod key_0;
#[doc = "Key Registers"]
pub struct KEY_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Registers"]
pub mod key_1;
#[doc = "Key Registers"]
pub struct KEY_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Registers"]
pub mod key_2;
#[doc = "Key Registers"]
pub struct KEY_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Registers"]
pub mod key_3;
#[doc = "Version ID Register"]
pub struct VID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version ID Register"]
pub mod vid1;
#[doc = "Version ID 2 Register"]
pub struct VID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version ID 2 Register"]
pub mod vid2;
#[doc = "CHA Version ID Register"]
pub struct CHAVID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHA Version ID Register"]
pub mod chavid;
#[doc = "FIFO Status Register"]
pub struct FIFOSTA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Status Register"]
pub mod fifosta;
#[doc = "Input Data FIFO"]
pub struct IFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Data FIFO"]
pub mod ififo;
#[doc = "Output Data FIFO"]
pub struct OFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Data FIFO"]
pub mod ofifo;
