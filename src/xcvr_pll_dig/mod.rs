#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PLL HPM Analog Bump Control"]
    pub hpm_bump: HPM_BUMP,
    #[doc = "0x04 - PLL Modulation Control"]
    pub mod_ctrl: MOD_CTRL,
    #[doc = "0x08 - PLL Channel Mapping"]
    pub chan_map: CHAN_MAP,
    #[doc = "0x0c - PLL Lock Detect Control"]
    pub lock_detect: LOCK_DETECT,
    #[doc = "0x10 - PLL High Port Modulator Control"]
    pub hpm_ctrl: HPM_CTRL,
    #[doc = "0x14 - PLL High Port Calibration Control"]
    pub hpmcal_ctrl: HPMCAL_CTRL,
    #[doc = "0x18 - PLL High Port Calibration Result 1"]
    pub hpm_cal1: HPM_CAL1,
    #[doc = "0x1c - PLL High Port Calibration Result 2"]
    pub hpm_cal2: HPM_CAL2,
    #[doc = "0x20 - PLL High Port Sigma Delta Results"]
    pub hpm_sdm_res: HPM_SDM_RES,
    #[doc = "0x24 - PLL Low Port Modulator Control"]
    pub lpm_ctrl: LPM_CTRL,
    #[doc = "0x28 - PLL Low Port Sigma Delta Control 1"]
    pub lpm_sdm_ctrl1: LPM_SDM_CTRL1,
    #[doc = "0x2c - PLL Low Port Sigma Delta Control 2"]
    pub lpm_sdm_ctrl2: LPM_SDM_CTRL2,
    #[doc = "0x30 - PLL Low Port Sigma Delta Control 3"]
    pub lpm_sdm_ctrl3: LPM_SDM_CTRL3,
    #[doc = "0x34 - PLL Low Port Sigma Delta Result 1"]
    pub lpm_sdm_res1: LPM_SDM_RES1,
    #[doc = "0x38 - PLL Low Port Sigma Delta Result 2"]
    pub lpm_sdm_res2: LPM_SDM_RES2,
    #[doc = "0x3c - PLL Delay Matching"]
    pub delay_match: DELAY_MATCH,
    #[doc = "0x40 - PLL Coarse Tune Control"]
    pub ctune_ctrl: CTUNE_CTRL,
    #[doc = "0x44 - PLL Coarse Tune Count 6"]
    pub ctune_cnt6: CTUNE_CNT6,
    #[doc = "0x48 - PLL Coarse Tune Counts 5 and 4"]
    pub ctune_cnt5_4: CTUNE_CNT5_4,
    #[doc = "0x4c - PLL Coarse Tune Counts 3 and 2"]
    pub ctune_cnt3_2: CTUNE_CNT3_2,
    #[doc = "0x50 - PLL Coarse Tune Counts 1 and 0"]
    pub ctune_cnt1_0: CTUNE_CNT1_0,
    #[doc = "0x54 - PLL Coarse Tune Results"]
    pub ctune_res: CTUNE_RES,
}
#[doc = "PLL HPM Analog Bump Control"]
pub struct HPM_BUMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL HPM Analog Bump Control"]
pub mod hpm_bump;
#[doc = "PLL Modulation Control"]
pub struct MOD_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Modulation Control"]
pub mod mod_ctrl;
#[doc = "PLL Channel Mapping"]
pub struct CHAN_MAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Channel Mapping"]
pub mod chan_map;
#[doc = "PLL Lock Detect Control"]
pub struct LOCK_DETECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Lock Detect Control"]
pub mod lock_detect;
#[doc = "PLL High Port Modulator Control"]
pub struct HPM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL High Port Modulator Control"]
pub mod hpm_ctrl;
#[doc = "PLL High Port Calibration Control"]
pub struct HPMCAL_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL High Port Calibration Control"]
pub mod hpmcal_ctrl;
#[doc = "PLL High Port Calibration Result 1"]
pub struct HPM_CAL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL High Port Calibration Result 1"]
pub mod hpm_cal1;
#[doc = "PLL High Port Calibration Result 2"]
pub struct HPM_CAL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL High Port Calibration Result 2"]
pub mod hpm_cal2;
#[doc = "PLL High Port Sigma Delta Results"]
pub struct HPM_SDM_RES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL High Port Sigma Delta Results"]
pub mod hpm_sdm_res;
#[doc = "PLL Low Port Modulator Control"]
pub struct LPM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Low Port Modulator Control"]
pub mod lpm_ctrl;
#[doc = "PLL Low Port Sigma Delta Control 1"]
pub struct LPM_SDM_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Low Port Sigma Delta Control 1"]
pub mod lpm_sdm_ctrl1;
#[doc = "PLL Low Port Sigma Delta Control 2"]
pub struct LPM_SDM_CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Low Port Sigma Delta Control 2"]
pub mod lpm_sdm_ctrl2;
#[doc = "PLL Low Port Sigma Delta Control 3"]
pub struct LPM_SDM_CTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Low Port Sigma Delta Control 3"]
pub mod lpm_sdm_ctrl3;
#[doc = "PLL Low Port Sigma Delta Result 1"]
pub struct LPM_SDM_RES1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Low Port Sigma Delta Result 1"]
pub mod lpm_sdm_res1;
#[doc = "PLL Low Port Sigma Delta Result 2"]
pub struct LPM_SDM_RES2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Low Port Sigma Delta Result 2"]
pub mod lpm_sdm_res2;
#[doc = "PLL Delay Matching"]
pub struct DELAY_MATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Delay Matching"]
pub mod delay_match;
#[doc = "PLL Coarse Tune Control"]
pub struct CTUNE_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Coarse Tune Control"]
pub mod ctune_ctrl;
#[doc = "PLL Coarse Tune Count 6"]
pub struct CTUNE_CNT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Coarse Tune Count 6"]
pub mod ctune_cnt6;
#[doc = "PLL Coarse Tune Counts 5 and 4"]
pub struct CTUNE_CNT5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Coarse Tune Counts 5 and 4"]
pub mod ctune_cnt5_4;
#[doc = "PLL Coarse Tune Counts 3 and 2"]
pub struct CTUNE_CNT3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Coarse Tune Counts 3 and 2"]
pub mod ctune_cnt3_2;
#[doc = "PLL Coarse Tune Counts 1 and 0"]
pub struct CTUNE_CNT1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Coarse Tune Counts 1 and 0"]
pub mod ctune_cnt1_0;
#[doc = "PLL Coarse Tune Results"]
pub struct CTUNE_RES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Coarse Tune Results"]
pub mod ctune_res;
