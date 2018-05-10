#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PREAMBLE REFERENCE WAVEFORM 0"]
    pub phy_pre_ref0: PHY_PRE_REF0,
    #[doc = "0x04 - PREAMBLE REFERENCE WAVEFORM 1"]
    pub pre_ref1: PRE_REF1,
    #[doc = "0x08 - PREAMBLE REFERENCE WAVEFORM 2"]
    pub pre_ref2: PRE_REF2,
    _reserved0: [u8; 20usize],
    #[doc = "0x20 - PHY CONFIGURATION REGISTER 1"]
    pub cfg1: CFG1,
    #[doc = "0x24 - PHY CONFIGURATION REGISTER 2"]
    pub cfg2: CFG2,
    #[doc = "0x28 - PHY EARLY/LATE CONFIGURATION REGISTER"]
    pub el_cfg: EL_CFG,
    #[doc = "0x2c - PHY NETWORK ADDRESS FOR BSM"]
    pub ntw_adr_bsm: NTW_ADR_BSM,
    #[doc = "0x30 - PHY STATUS REGISTER"]
    pub status: STATUS,
}
#[doc = "PREAMBLE REFERENCE WAVEFORM 0"]
pub struct PHY_PRE_REF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PREAMBLE REFERENCE WAVEFORM 0"]
pub mod phy_pre_ref0;
#[doc = "PREAMBLE REFERENCE WAVEFORM 1"]
pub struct PRE_REF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PREAMBLE REFERENCE WAVEFORM 1"]
pub mod pre_ref1;
#[doc = "PREAMBLE REFERENCE WAVEFORM 2"]
pub struct PRE_REF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PREAMBLE REFERENCE WAVEFORM 2"]
pub mod pre_ref2;
#[doc = "PHY CONFIGURATION REGISTER 1"]
pub struct CFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PHY CONFIGURATION REGISTER 1"]
pub mod cfg1;
#[doc = "PHY CONFIGURATION REGISTER 2"]
pub struct CFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PHY CONFIGURATION REGISTER 2"]
pub mod cfg2;
#[doc = "PHY EARLY/LATE CONFIGURATION REGISTER"]
pub struct EL_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PHY EARLY/LATE CONFIGURATION REGISTER"]
pub mod el_cfg;
#[doc = "PHY NETWORK ADDRESS FOR BSM"]
pub struct NTW_ADR_BSM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PHY NETWORK ADDRESS FOR BSM"]
pub mod ntw_adr_bsm;
#[doc = "PHY STATUS REGISTER"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PHY STATUS REGISTER"]
pub mod status;
