#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Error Status Register"]
    pub es: ES,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - Enable Request Register"]
    pub erq: ERQ,
    _reserved1: [u8; 4usize],
    #[doc = "0x14 - Enable Error Interrupt Register"]
    pub eei: EEI,
    #[doc = "0x18 - Clear Enable Error Interrupt Register"]
    pub ceei: CEEI,
    #[doc = "0x19 - Set Enable Error Interrupt Register"]
    pub seei: SEEI,
    #[doc = "0x1a - Clear Enable Request Register"]
    pub cerq: CERQ,
    #[doc = "0x1b - Set Enable Request Register"]
    pub serq: SERQ,
    #[doc = "0x1c - Clear DONE Status Bit Register"]
    pub cdne: CDNE,
    #[doc = "0x1d - Set START Bit Register"]
    pub ssrt: SSRT,
    #[doc = "0x1e - Clear Error Register"]
    pub cerr: CERR,
    #[doc = "0x1f - Clear Interrupt Request Register"]
    pub cint: CINT,
    _reserved2: [u8; 4usize],
    #[doc = "0x24 - Interrupt Request Register"]
    pub int: INT,
    _reserved3: [u8; 4usize],
    #[doc = "0x2c - Error Register"]
    pub err: ERR,
    _reserved4: [u8; 4usize],
    #[doc = "0x34 - Hardware Request Status Register"]
    pub hrs: HRS,
    _reserved5: [u8; 12usize],
    #[doc = "0x44 - Enable Asynchronous Request in Stop Register"]
    pub ears: EARS,
    _reserved6: [u8; 184usize],
    #[doc = "0x100 - Channel n Priority Register"]
    pub dchpri3: DCHPRI,
    #[doc = "0x101 - Channel n Priority Register"]
    pub dchpri2: DCHPRI,
    #[doc = "0x102 - Channel n Priority Register"]
    pub dchpri1: DCHPRI,
    #[doc = "0x103 - Channel n Priority Register"]
    pub dchpri0: DCHPRI,
    _reserved7: [u8; 3836usize],
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: TCD_SADDR,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: TCD_SOFF,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: TCD_ATTR,
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd0_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: TCD_SLAST,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: TCD_DADDR,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: TCD_DOFF,
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd0_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: TCD_DLASTSGA,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: TCD_CSR,
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd0_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: TCD_SADDR,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: TCD_SOFF,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: TCD_ATTR,
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd1_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: TCD_SLAST,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: TCD_DADDR,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: TCD_DOFF,
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd1_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: TCD_DLASTSGA,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: TCD_CSR,
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd1_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: TCD_SADDR,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: TCD_SOFF,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: TCD_ATTR,
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd2_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: TCD_SLAST,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: TCD_DADDR,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: TCD_DOFF,
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd2_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: TCD_DLASTSGA,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: TCD_CSR,
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd2_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: TCD_SADDR,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: TCD_SOFF,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: TCD_ATTR,
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd3_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: TCD_SLAST,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: TCD_DADDR,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: TCD_DOFF,
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd3_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: TCD_DLASTSGA,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: TCD_CSR,
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd3_biter_elinkno: TCD_BITER_ELINKNO,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Error Status Register"]
pub struct ES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Status Register"]
pub mod es;
#[doc = "Enable Request Register"]
pub struct ERQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "Enable Error Interrupt Register"]
pub struct EEI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "Clear Enable Error Interrupt Register"]
pub struct CEEI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "Set Enable Error Interrupt Register"]
pub struct SEEI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "Clear Enable Request Register"]
pub struct CERQ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "Set Enable Request Register"]
pub struct SERQ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "Clear DONE Status Bit Register"]
pub struct CDNE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "Set START Bit Register"]
pub struct SSRT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "Clear Error Register"]
pub struct CERR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "Clear Interrupt Request Register"]
pub struct CINT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "Interrupt Request Register"]
pub struct INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "Error Register"]
pub struct ERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Register"]
pub mod err;
#[doc = "Hardware Request Status Register"]
pub struct HRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "Enable Asynchronous Request in Stop Register"]
pub struct EARS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Asynchronous Request in Stop Register"]
pub mod ears;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri;
#[doc = "TCD Source Address"]
pub struct TCD_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd_slast;
#[doc = "TCD Destination Address"]
pub struct TCD_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elinkyes;
