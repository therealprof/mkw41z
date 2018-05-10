#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 802.15.4 DEMOD CORRELLATOR CONTROL"]
    pub corr_ctrl: CORR_CTRL,
    #[doc = "0x04 - 802.15.4 DEMOD PN TYPE"]
    pub pn_type: PN_TYPE,
    #[doc = "0x08 - 802.15.4 DEMOD PN CODE"]
    pub pn_code: PN_CODE,
    #[doc = "0x0c - 802.15.4 DEMOD SYMBOL SYNC CONTROL"]
    pub sync_ctrl: SYNC_CTRL,
    #[doc = "0x10 - 802.15.4 CCA/LQI SOURCE"]
    pub cca_lqi_src: CCA_LQI_SRC,
    #[doc = "0x14 - FAD CORRELATOR THRESHOLD"]
    pub fad_thr: FAD_THR,
    #[doc = "0x18 - 802.15.4 AFC STATUS"]
    pub zbdem_afc: ZBDEM_AFC,
}
#[doc = "802.15.4 DEMOD CORRELLATOR CONTROL"]
pub struct CORR_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "802.15.4 DEMOD CORRELLATOR CONTROL"]
pub mod corr_ctrl;
#[doc = "802.15.4 DEMOD PN TYPE"]
pub struct PN_TYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "802.15.4 DEMOD PN TYPE"]
pub mod pn_type;
#[doc = "802.15.4 DEMOD PN CODE"]
pub struct PN_CODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "802.15.4 DEMOD PN CODE"]
pub mod pn_code;
#[doc = "802.15.4 DEMOD SYMBOL SYNC CONTROL"]
pub struct SYNC_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "802.15.4 DEMOD SYMBOL SYNC CONTROL"]
pub mod sync_ctrl;
#[doc = "802.15.4 CCA/LQI SOURCE"]
pub struct CCA_LQI_SRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "802.15.4 CCA/LQI SOURCE"]
pub mod cca_lqi_src;
#[doc = "FAD CORRELATOR THRESHOLD"]
pub struct FAD_THR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FAD CORRELATOR THRESHOLD"]
pub mod fad_thr;
#[doc = "802.15.4 AFC STATUS"]
pub struct ZBDEM_AFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "802.15.4 AFC STATUS"]
pub mod zbdem_afc;
