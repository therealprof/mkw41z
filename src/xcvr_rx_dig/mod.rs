#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RX Digital Control"]
    pub rx_dig_ctrl: RX_DIG_CTRL,
    #[doc = "0x04 - AGC Control 0"]
    pub agc_ctrl_0: AGC_CTRL_0,
    #[doc = "0x08 - AGC Control 1"]
    pub agc_ctrl_1: AGC_CTRL_1,
    #[doc = "0x0c - AGC Control 2"]
    pub agc_ctrl_2: AGC_CTRL_2,
    #[doc = "0x10 - AGC Control 3"]
    pub agc_ctrl_3: AGC_CTRL_3,
    #[doc = "0x14 - AGC Status"]
    pub agc_stat: AGC_STAT,
    #[doc = "0x18 - RSSI Control 0"]
    pub rssi_ctrl_0: RSSI_CTRL_0,
    #[doc = "0x1c - RSSI Control 1"]
    pub rssi_ctrl_1: RSSI_CTRL_1,
    #[doc = "0x20 - RSSI DFT"]
    pub rssi_dft: RSSI_DFT,
    #[doc = "0x24 - DCOC Control 0"]
    pub dcoc_ctrl_0: DCOC_CTRL_0,
    #[doc = "0x28 - DCOC Control 1"]
    pub dcoc_ctrl_1: DCOC_CTRL_1,
    #[doc = "0x2c - DCOC DAC Initialization"]
    pub dcoc_dac_init: DCOC_DAC_INIT,
    #[doc = "0x30 - DCOC Digital Correction Manual Override"]
    pub dcoc_dig_man: DCOC_DIG_MAN,
    #[doc = "0x34 - DCOC Calibration Gain"]
    pub dcoc_cal_gain: DCOC_CAL_GAIN,
    #[doc = "0x38 - DCOC Status"]
    pub dcoc_stat: DCOC_STAT,
    #[doc = "0x3c - DCOC DC Estimate"]
    pub dcoc_dc_est: DCOC_DC_EST,
    #[doc = "0x40 - DCOC Calibration Reciprocals"]
    pub dcoc_cal_rcp: DCOC_CAL_RCP,
    _reserved0: [u8; 4usize],
    #[doc = "0x48 - IQMC Control"]
    pub iqmc_ctrl: IQMC_CTRL,
    #[doc = "0x4c - IQMC Calibration"]
    pub iqmc_cal: IQMC_CAL,
    #[doc = "0x50 - LNA_GAIN Step Values 3..0"]
    pub lna_gain_val_3_0: LNA_GAIN_VAL_3_0,
    #[doc = "0x54 - LNA_GAIN Step Values 7..4"]
    pub lna_gain_val_7_4: LNA_GAIN_VAL_7_4,
    #[doc = "0x58 - LNA_GAIN Step Values 8"]
    pub lna_gain_val_8: LNA_GAIN_VAL_8,
    #[doc = "0x5c - BBA Resistor Tune Values 7..0"]
    pub bba_res_tune_val_7_0: BBA_RES_TUNE_VAL_7_0,
    #[doc = "0x60 - BBA Resistor Tune Values 10..8"]
    pub bba_res_tune_val_10_8: BBA_RES_TUNE_VAL_10_8,
    #[doc = "0x64 - LNA Linear Gain Values 2..0"]
    pub lna_gain_lin_val_2_0: LNA_GAIN_LIN_VAL_2_0,
    #[doc = "0x68 - LNA Linear Gain Values 5..3"]
    pub lna_gain_lin_val_5_3: LNA_GAIN_LIN_VAL_5_3,
    #[doc = "0x6c - LNA Linear Gain Values 8..6"]
    pub lna_gain_lin_val_8_6: LNA_GAIN_LIN_VAL_8_6,
    #[doc = "0x70 - LNA Linear Gain Values 9"]
    pub lna_gain_lin_val_9: LNA_GAIN_LIN_VAL_9,
    #[doc = "0x74 - BBA Resistor Tune Values 3..0"]
    pub bba_res_tune_lin_val_3_0: BBA_RES_TUNE_LIN_VAL_3_0,
    #[doc = "0x78 - BBA Resistor Tune Values 7..4"]
    pub bba_res_tune_lin_val_7_4: BBA_RES_TUNE_LIN_VAL_7_4,
    #[doc = "0x7c - BBA Resistor Tune Values 10..8"]
    pub bba_res_tune_lin_val_10_8: BBA_RES_TUNE_LIN_VAL_10_8,
    #[doc = "0x80 - AGC Gain Tables Step 03..00"]
    pub agc_gain_tbl_03_00: AGC_GAIN_TBL_03_00,
    #[doc = "0x84 - AGC Gain Tables Step 07..04"]
    pub agc_gain_tbl_07_04: AGC_GAIN_TBL_07_04,
    #[doc = "0x88 - AGC Gain Tables Step 11..08"]
    pub agc_gain_tbl_11_08: AGC_GAIN_TBL_11_08,
    #[doc = "0x8c - AGC Gain Tables Step 15..12"]
    pub agc_gain_tbl_15_12: AGC_GAIN_TBL_15_12,
    #[doc = "0x90 - AGC Gain Tables Step 19..16"]
    pub agc_gain_tbl_19_16: AGC_GAIN_TBL_19_16,
    #[doc = "0x94 - AGC Gain Tables Step 23..20"]
    pub agc_gain_tbl_23_20: AGC_GAIN_TBL_23_20,
    #[doc = "0x98 - AGC Gain Tables Step 26..24"]
    pub agc_gain_tbl_26_24: AGC_GAIN_TBL_26_24,
    _reserved1: [u8; 4usize],
    #[doc = "0xa0 - DCOC Offset"]
    pub dcoc_offset_0: DCOC_OFFSET_0,
    #[doc = "0xa4 - DCOC Offset"]
    pub dcoc_offset_1: DCOC_OFFSET_1,
    #[doc = "0xa8 - DCOC Offset"]
    pub dcoc_offset_2: DCOC_OFFSET_2,
    #[doc = "0xac - DCOC Offset"]
    pub dcoc_offset_3: DCOC_OFFSET_3,
    #[doc = "0xb0 - DCOC Offset"]
    pub dcoc_offset_4: DCOC_OFFSET_4,
    #[doc = "0xb4 - DCOC Offset"]
    pub dcoc_offset_5: DCOC_OFFSET_5,
    #[doc = "0xb8 - DCOC Offset"]
    pub dcoc_offset_6: DCOC_OFFSET_6,
    #[doc = "0xbc - DCOC Offset"]
    pub dcoc_offset_7: DCOC_OFFSET_7,
    #[doc = "0xc0 - DCOC Offset"]
    pub dcoc_offset_8: DCOC_OFFSET_8,
    #[doc = "0xc4 - DCOC Offset"]
    pub dcoc_offset_9: DCOC_OFFSET_9,
    #[doc = "0xc8 - DCOC Offset"]
    pub dcoc_offset_10: DCOC_OFFSET_10,
    #[doc = "0xcc - DCOC Offset"]
    pub dcoc_offset_11: DCOC_OFFSET_11,
    #[doc = "0xd0 - DCOC Offset"]
    pub dcoc_offset_12: DCOC_OFFSET_12,
    #[doc = "0xd4 - DCOC Offset"]
    pub dcoc_offset_13: DCOC_OFFSET_13,
    #[doc = "0xd8 - DCOC Offset"]
    pub dcoc_offset_14: DCOC_OFFSET_14,
    #[doc = "0xdc - DCOC Offset"]
    pub dcoc_offset_15: DCOC_OFFSET_15,
    #[doc = "0xe0 - DCOC Offset"]
    pub dcoc_offset_16: DCOC_OFFSET_16,
    #[doc = "0xe4 - DCOC Offset"]
    pub dcoc_offset_17: DCOC_OFFSET_17,
    #[doc = "0xe8 - DCOC Offset"]
    pub dcoc_offset_18: DCOC_OFFSET_18,
    #[doc = "0xec - DCOC Offset"]
    pub dcoc_offset_19: DCOC_OFFSET_19,
    #[doc = "0xf0 - DCOC Offset"]
    pub dcoc_offset_20: DCOC_OFFSET_20,
    #[doc = "0xf4 - DCOC Offset"]
    pub dcoc_offset_21: DCOC_OFFSET_21,
    #[doc = "0xf8 - DCOC Offset"]
    pub dcoc_offset_22: DCOC_OFFSET_22,
    #[doc = "0xfc - DCOC Offset"]
    pub dcoc_offset_23: DCOC_OFFSET_23,
    #[doc = "0x100 - DCOC Offset"]
    pub dcoc_offset_24: DCOC_OFFSET_24,
    #[doc = "0x104 - DCOC Offset"]
    pub dcoc_offset_25: DCOC_OFFSET_25,
    #[doc = "0x108 - DCOC Offset"]
    pub dcoc_offset_26: DCOC_OFFSET_26,
    #[doc = "0x10c - DCOC BBA DAC Step"]
    pub dcoc_bba_step: DCOC_BBA_STEP,
    #[doc = "0x110 - DCOC TZA DAC Step 0"]
    pub dcoc_tza_step_0: DCOC_TZA_STEP_0,
    #[doc = "0x114 - DCOC TZA DAC Step 1"]
    pub dcoc_tza_step_1: DCOC_TZA_STEP_1,
    #[doc = "0x118 - DCOC TZA DAC Step 2"]
    pub dcoc_tza_step_2: DCOC_TZA_STEP_2,
    #[doc = "0x11c - DCOC TZA DAC Step 3"]
    pub dcoc_tza_step_3: DCOC_TZA_STEP_3,
    #[doc = "0x120 - DCOC TZA DAC Step 4"]
    pub dcoc_tza_step_4: DCOC_TZA_STEP_4,
    #[doc = "0x124 - DCOC TZA DAC Step 5"]
    pub dcoc_tza_step_5: DCOC_TZA_STEP_5,
    #[doc = "0x128 - DCOC TZA DAC Step 6"]
    pub dcoc_tza_step_6: DCOC_TZA_STEP_6,
    #[doc = "0x12c - DCOC TZA DAC Step 7"]
    pub dcoc_tza_step_7: DCOC_TZA_STEP_7,
    #[doc = "0x130 - DCOC TZA DAC Step 5"]
    pub dcoc_tza_step_8: DCOC_TZA_STEP_8,
    #[doc = "0x134 - DCOC TZA DAC Step 9"]
    pub dcoc_tza_step_9: DCOC_TZA_STEP_9,
    #[doc = "0x138 - DCOC TZA DAC Step 10"]
    pub dcoc_tza_step_10: DCOC_TZA_STEP_10,
    _reserved2: [u8; 44usize],
    #[doc = "0x168 - DCOC Calibration Alpha"]
    pub dcoc_cal_alpha: DCOC_CAL_ALPHA,
    #[doc = "0x16c - DCOC Calibration Beta Q"]
    pub dcoc_cal_beta_q: DCOC_CAL_BETA_Q,
    #[doc = "0x170 - DCOC Calibration Beta I"]
    pub dcoc_cal_beta_i: DCOC_CAL_BETA_I,
    #[doc = "0x174 - DCOC Calibration Gamma"]
    pub dcoc_cal_gamma: DCOC_CAL_GAMMA,
    #[doc = "0x178 - DCOC Calibration IIR"]
    pub dcoc_cal_iir: DCOC_CAL_IIR,
    _reserved3: [u8; 4usize],
    #[doc = "0x180 - DCOC Calibration Result"]
    pub dcoc_cal1: DCOC_CAL1,
    #[doc = "0x184 - DCOC Calibration Result"]
    pub dcoc_cal2: DCOC_CAL2,
    #[doc = "0x188 - DCOC Calibration Result"]
    pub dcoc_cal3: DCOC_CAL3,
    _reserved4: [u8; 4usize],
    #[doc = "0x190 - RX_DIG CCA ED LQI Control Register 0"]
    pub cca_ed_lqi_ctrl_0: CCA_ED_LQI_CTRL_0,
    #[doc = "0x194 - RX_DIG CCA ED LQI Control Register 1"]
    pub cca_ed_lqi_ctrl_1: CCA_ED_LQI_CTRL_1,
    #[doc = "0x198 - RX_DIG CCA ED LQI Status Register 0"]
    pub cca_ed_lqi_stat_0: CCA_ED_LQI_STAT_0,
    _reserved5: [u8; 4usize],
    #[doc = "0x1a0 - Receive Channel Filter Coefficient 0"]
    pub rx_chf_coef_0: RX_CHF_COEF_0,
    #[doc = "0x1a4 - Receive Channel Filter Coefficient 1"]
    pub rx_chf_coef_1: RX_CHF_COEF_1,
    #[doc = "0x1a8 - Receive Channel Filter Coefficient 2"]
    pub rx_chf_coef_2: RX_CHF_COEF_2,
    #[doc = "0x1ac - Receive Channel Filter Coefficient 3"]
    pub rx_chf_coef_3: RX_CHF_COEF_3,
    #[doc = "0x1b0 - Receive Channel Filter Coefficient 4"]
    pub rx_chf_coef_4: RX_CHF_COEF_4,
    #[doc = "0x1b4 - Receive Channel Filter Coefficient 5"]
    pub rx_chf_coef_5: RX_CHF_COEF_5,
    #[doc = "0x1b8 - Receive Channel Filter Coefficient 6"]
    pub rx_chf_coef_6: RX_CHF_COEF_6,
    #[doc = "0x1bc - Receive Channel Filter Coefficient 7"]
    pub rx_chf_coef_7: RX_CHF_COEF_7,
    #[doc = "0x1c0 - Receive Channel Filter Coefficient 8"]
    pub rx_chf_coef_8: RX_CHF_COEF_8,
    #[doc = "0x1c4 - Receive Channel Filter Coefficient 9"]
    pub rx_chf_coef_9: RX_CHF_COEF_9,
    #[doc = "0x1c8 - Receive Channel Filter Coefficient 10"]
    pub rx_chf_coef_10: RX_CHF_COEF_10,
    #[doc = "0x1cc - Receive Channel Filter Coefficient 11"]
    pub rx_chf_coef_11: RX_CHF_COEF_11,
    #[doc = "0x1d0 - AGC Manual AGC Index"]
    pub agc_man_agc_idx: AGC_MAN_AGC_IDX,
    #[doc = "0x1d4 - DC Residual Control"]
    pub dc_resid_ctrl: DC_RESID_CTRL,
    #[doc = "0x1d8 - DC Residual Estimate"]
    pub dc_resid_est: DC_RESID_EST,
    #[doc = "0x1dc - RX RC Calibration Control0"]
    pub rx_rccal_ctrl0: RX_RCCAL_CTRL0,
    #[doc = "0x1e0 - RX RC Calibration Control1"]
    pub rx_rccal_ctrl1: RX_RCCAL_CTRL1,
    #[doc = "0x1e4 - RX RC Calibration Status"]
    pub rx_rccal_stat: RX_RCCAL_STAT,
    #[doc = "0x1e8 - Aux PLL Frequency Calibration Control"]
    pub auxpll_fcal_ctrl: AUXPLL_FCAL_CTRL,
    #[doc = "0x1ec - Aux PLL Frequency Calibration Count 6"]
    pub auxpll_fcal_cnt6: AUXPLL_FCAL_CNT6,
    #[doc = "0x1f0 - Aux PLL Frequency Calibration Count 5 and 4"]
    pub auxpll_fcal_cnt5_4: AUXPLL_FCAL_CNT5_4,
    #[doc = "0x1f4 - Aux PLL Frequency Calibration Count 3 and 2"]
    pub auxpll_fcal_cnt3_2: AUXPLL_FCAL_CNT3_2,
    #[doc = "0x1f8 - Aux PLL Frequency Calibration Count 1 and 0"]
    pub auxpll_fcal_cnt1_0: AUXPLL_FCAL_CNT1_0,
    #[doc = "0x1fc - RXDIG DFT"]
    pub rxdig_dft: RXDIG_DFT,
}
#[doc = "RX Digital Control"]
pub struct RX_DIG_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Digital Control"]
pub mod rx_dig_ctrl;
#[doc = "AGC Control 0"]
pub struct AGC_CTRL_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AGC Control 0"]
pub mod agc_ctrl_0;
#[doc = "AGC Control 1"]
pub struct AGC_CTRL_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AGC Control 1"]
pub mod agc_ctrl_1;
#[doc = "AGC Control 2"]
pub struct AGC_CTRL_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AGC Control 2"]
pub mod agc_ctrl_2;
#[doc = "AGC Control 3"]
pub struct AGC_CTRL_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AGC Control 3"]
pub mod agc_ctrl_3;
#[doc = "AGC Status"]
pub struct AGC_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AGC Status"]
pub mod agc_stat;
#[doc = "RSSI Control 0"]
pub struct RSSI_CTRL_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RSSI Control 0"]
pub mod rssi_ctrl_0;
#[doc = "RSSI Control 1"]
pub struct RSSI_CTRL_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RSSI Control 1"]
pub mod rssi_ctrl_1;
#[doc = "RSSI DFT"]
pub struct RSSI_DFT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RSSI DFT"]
pub mod rssi_dft;
#[doc = "DCOC Control 0"]
pub struct DCOC_CTRL_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Control 0"]
pub mod dcoc_ctrl_0;
#[doc = "DCOC Control 1"]
pub struct DCOC_CTRL_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Control 1"]
pub mod dcoc_ctrl_1;
#[doc = "DCOC DAC Initialization"]
pub struct DCOC_DAC_INIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC DAC Initialization"]
pub mod dcoc_dac_init;
#[doc = "DCOC Digital Correction Manual Override"]
pub struct DCOC_DIG_MAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Digital Correction Manual Override"]
pub mod dcoc_dig_man;
#[doc = "DCOC Calibration Gain"]
pub struct DCOC_CAL_GAIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Calibration Gain"]
pub mod dcoc_cal_gain;
#[doc = "DCOC Status"]
pub struct DCOC_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Status"]
pub mod dcoc_stat;
#[doc = "DCOC DC Estimate"]
pub struct DCOC_DC_EST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC DC Estimate"]
pub mod dcoc_dc_est;
#[doc = "DCOC Calibration Reciprocals"]
pub struct DCOC_CAL_RCP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Calibration Reciprocals"]
pub mod dcoc_cal_rcp;
#[doc = "IQMC Control"]
pub struct IQMC_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IQMC Control"]
pub mod iqmc_ctrl;
#[doc = "IQMC Calibration"]
pub struct IQMC_CAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IQMC Calibration"]
pub mod iqmc_cal;
#[doc = "LNA_GAIN Step Values 3..0"]
pub struct LNA_GAIN_VAL_3_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LNA_GAIN Step Values 3..0"]
pub mod lna_gain_val_3_0;
#[doc = "LNA_GAIN Step Values 7..4"]
pub struct LNA_GAIN_VAL_7_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LNA_GAIN Step Values 7..4"]
pub mod lna_gain_val_7_4;
#[doc = "LNA_GAIN Step Values 8"]
pub struct LNA_GAIN_VAL_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LNA_GAIN Step Values 8"]
pub mod lna_gain_val_8;
#[doc = "BBA Resistor Tune Values 7..0"]
pub struct BBA_RES_TUNE_VAL_7_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BBA Resistor Tune Values 7..0"]
pub mod bba_res_tune_val_7_0;
#[doc = "BBA Resistor Tune Values 10..8"]
pub struct BBA_RES_TUNE_VAL_10_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BBA Resistor Tune Values 10..8"]
pub mod bba_res_tune_val_10_8;
#[doc = "LNA Linear Gain Values 2..0"]
pub struct LNA_GAIN_LIN_VAL_2_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LNA Linear Gain Values 2..0"]
pub mod lna_gain_lin_val_2_0;
#[doc = "LNA Linear Gain Values 5..3"]
pub struct LNA_GAIN_LIN_VAL_5_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LNA Linear Gain Values 5..3"]
pub mod lna_gain_lin_val_5_3;
#[doc = "LNA Linear Gain Values 8..6"]
pub struct LNA_GAIN_LIN_VAL_8_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LNA Linear Gain Values 8..6"]
pub mod lna_gain_lin_val_8_6;
#[doc = "LNA Linear Gain Values 9"]
pub struct LNA_GAIN_LIN_VAL_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LNA Linear Gain Values 9"]
pub mod lna_gain_lin_val_9;
#[doc = "BBA Resistor Tune Values 3..0"]
pub struct BBA_RES_TUNE_LIN_VAL_3_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BBA Resistor Tune Values 3..0"]
pub mod bba_res_tune_lin_val_3_0;
#[doc = "BBA Resistor Tune Values 7..4"]
pub struct BBA_RES_TUNE_LIN_VAL_7_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BBA Resistor Tune Values 7..4"]
pub mod bba_res_tune_lin_val_7_4;
#[doc = "BBA Resistor Tune Values 10..8"]
pub struct BBA_RES_TUNE_LIN_VAL_10_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BBA Resistor Tune Values 10..8"]
pub mod bba_res_tune_lin_val_10_8;
#[doc = "AGC Gain Tables Step 03..00"]
pub struct AGC_GAIN_TBL_03_00 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AGC Gain Tables Step 03..00"]
pub mod agc_gain_tbl_03_00;
#[doc = "AGC Gain Tables Step 07..04"]
pub struct AGC_GAIN_TBL_07_04 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AGC Gain Tables Step 07..04"]
pub mod agc_gain_tbl_07_04;
#[doc = "AGC Gain Tables Step 11..08"]
pub struct AGC_GAIN_TBL_11_08 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AGC Gain Tables Step 11..08"]
pub mod agc_gain_tbl_11_08;
#[doc = "AGC Gain Tables Step 15..12"]
pub struct AGC_GAIN_TBL_15_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AGC Gain Tables Step 15..12"]
pub mod agc_gain_tbl_15_12;
#[doc = "AGC Gain Tables Step 19..16"]
pub struct AGC_GAIN_TBL_19_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AGC Gain Tables Step 19..16"]
pub mod agc_gain_tbl_19_16;
#[doc = "AGC Gain Tables Step 23..20"]
pub struct AGC_GAIN_TBL_23_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AGC Gain Tables Step 23..20"]
pub mod agc_gain_tbl_23_20;
#[doc = "AGC Gain Tables Step 26..24"]
pub struct AGC_GAIN_TBL_26_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AGC Gain Tables Step 26..24"]
pub mod agc_gain_tbl_26_24;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_0;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_1;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_2;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_3;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_4;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_5;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_6;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_7;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_8;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_9;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_10;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_11;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_12;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_13;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_14;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_15;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_16;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_17;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_18;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_19;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_20;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_21;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_22;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_23;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_24;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_25;
#[doc = "DCOC Offset"]
pub struct DCOC_OFFSET_26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Offset"]
pub mod dcoc_offset_26;
#[doc = "DCOC BBA DAC Step"]
pub struct DCOC_BBA_STEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC BBA DAC Step"]
pub mod dcoc_bba_step;
#[doc = "DCOC TZA DAC Step 0"]
pub struct DCOC_TZA_STEP_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC TZA DAC Step 0"]
pub mod dcoc_tza_step_0;
#[doc = "DCOC TZA DAC Step 1"]
pub struct DCOC_TZA_STEP_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC TZA DAC Step 1"]
pub mod dcoc_tza_step_1;
#[doc = "DCOC TZA DAC Step 2"]
pub struct DCOC_TZA_STEP_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC TZA DAC Step 2"]
pub mod dcoc_tza_step_2;
#[doc = "DCOC TZA DAC Step 3"]
pub struct DCOC_TZA_STEP_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC TZA DAC Step 3"]
pub mod dcoc_tza_step_3;
#[doc = "DCOC TZA DAC Step 4"]
pub struct DCOC_TZA_STEP_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC TZA DAC Step 4"]
pub mod dcoc_tza_step_4;
#[doc = "DCOC TZA DAC Step 5"]
pub struct DCOC_TZA_STEP_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC TZA DAC Step 5"]
pub mod dcoc_tza_step_5;
#[doc = "DCOC TZA DAC Step 6"]
pub struct DCOC_TZA_STEP_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC TZA DAC Step 6"]
pub mod dcoc_tza_step_6;
#[doc = "DCOC TZA DAC Step 7"]
pub struct DCOC_TZA_STEP_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC TZA DAC Step 7"]
pub mod dcoc_tza_step_7;
#[doc = "DCOC TZA DAC Step 5"]
pub struct DCOC_TZA_STEP_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC TZA DAC Step 5"]
pub mod dcoc_tza_step_8;
#[doc = "DCOC TZA DAC Step 9"]
pub struct DCOC_TZA_STEP_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC TZA DAC Step 9"]
pub mod dcoc_tza_step_9;
#[doc = "DCOC TZA DAC Step 10"]
pub struct DCOC_TZA_STEP_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC TZA DAC Step 10"]
pub mod dcoc_tza_step_10;
#[doc = "DCOC Calibration Alpha"]
pub struct DCOC_CAL_ALPHA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Calibration Alpha"]
pub mod dcoc_cal_alpha;
#[doc = "DCOC Calibration Beta Q"]
pub struct DCOC_CAL_BETA_Q {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Calibration Beta Q"]
pub mod dcoc_cal_beta_q;
#[doc = "DCOC Calibration Beta I"]
pub struct DCOC_CAL_BETA_I {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Calibration Beta I"]
pub mod dcoc_cal_beta_i;
#[doc = "DCOC Calibration Gamma"]
pub struct DCOC_CAL_GAMMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Calibration Gamma"]
pub mod dcoc_cal_gamma;
#[doc = "DCOC Calibration IIR"]
pub struct DCOC_CAL_IIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Calibration IIR"]
pub mod dcoc_cal_iir;
#[doc = "DCOC Calibration Result"]
pub struct DCOC_CAL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Calibration Result"]
pub mod dcoc_cal1;
#[doc = "DCOC Calibration Result"]
pub struct DCOC_CAL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Calibration Result"]
pub mod dcoc_cal2;
#[doc = "DCOC Calibration Result"]
pub struct DCOC_CAL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCOC Calibration Result"]
pub mod dcoc_cal3;
#[doc = "RX_DIG CCA ED LQI Control Register 0"]
pub struct CCA_ED_LQI_CTRL_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX_DIG CCA ED LQI Control Register 0"]
pub mod cca_ed_lqi_ctrl_0;
#[doc = "RX_DIG CCA ED LQI Control Register 1"]
pub struct CCA_ED_LQI_CTRL_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX_DIG CCA ED LQI Control Register 1"]
pub mod cca_ed_lqi_ctrl_1;
#[doc = "RX_DIG CCA ED LQI Status Register 0"]
pub struct CCA_ED_LQI_STAT_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX_DIG CCA ED LQI Status Register 0"]
pub mod cca_ed_lqi_stat_0;
#[doc = "Receive Channel Filter Coefficient 0"]
pub struct RX_CHF_COEF_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel Filter Coefficient 0"]
pub mod rx_chf_coef_0;
#[doc = "Receive Channel Filter Coefficient 1"]
pub struct RX_CHF_COEF_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel Filter Coefficient 1"]
pub mod rx_chf_coef_1;
#[doc = "Receive Channel Filter Coefficient 2"]
pub struct RX_CHF_COEF_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel Filter Coefficient 2"]
pub mod rx_chf_coef_2;
#[doc = "Receive Channel Filter Coefficient 3"]
pub struct RX_CHF_COEF_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel Filter Coefficient 3"]
pub mod rx_chf_coef_3;
#[doc = "Receive Channel Filter Coefficient 4"]
pub struct RX_CHF_COEF_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel Filter Coefficient 4"]
pub mod rx_chf_coef_4;
#[doc = "Receive Channel Filter Coefficient 5"]
pub struct RX_CHF_COEF_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel Filter Coefficient 5"]
pub mod rx_chf_coef_5;
#[doc = "Receive Channel Filter Coefficient 6"]
pub struct RX_CHF_COEF_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel Filter Coefficient 6"]
pub mod rx_chf_coef_6;
#[doc = "Receive Channel Filter Coefficient 7"]
pub struct RX_CHF_COEF_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel Filter Coefficient 7"]
pub mod rx_chf_coef_7;
#[doc = "Receive Channel Filter Coefficient 8"]
pub struct RX_CHF_COEF_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel Filter Coefficient 8"]
pub mod rx_chf_coef_8;
#[doc = "Receive Channel Filter Coefficient 9"]
pub struct RX_CHF_COEF_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel Filter Coefficient 9"]
pub mod rx_chf_coef_9;
#[doc = "Receive Channel Filter Coefficient 10"]
pub struct RX_CHF_COEF_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel Filter Coefficient 10"]
pub mod rx_chf_coef_10;
#[doc = "Receive Channel Filter Coefficient 11"]
pub struct RX_CHF_COEF_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel Filter Coefficient 11"]
pub mod rx_chf_coef_11;
#[doc = "AGC Manual AGC Index"]
pub struct AGC_MAN_AGC_IDX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AGC Manual AGC Index"]
pub mod agc_man_agc_idx;
#[doc = "DC Residual Control"]
pub struct DC_RESID_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DC Residual Control"]
pub mod dc_resid_ctrl;
#[doc = "DC Residual Estimate"]
pub struct DC_RESID_EST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DC Residual Estimate"]
pub mod dc_resid_est;
#[doc = "RX RC Calibration Control0"]
pub struct RX_RCCAL_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX RC Calibration Control0"]
pub mod rx_rccal_ctrl0;
#[doc = "RX RC Calibration Control1"]
pub struct RX_RCCAL_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX RC Calibration Control1"]
pub mod rx_rccal_ctrl1;
#[doc = "RX RC Calibration Status"]
pub struct RX_RCCAL_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX RC Calibration Status"]
pub mod rx_rccal_stat;
#[doc = "Aux PLL Frequency Calibration Control"]
pub struct AUXPLL_FCAL_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Aux PLL Frequency Calibration Control"]
pub mod auxpll_fcal_ctrl;
#[doc = "Aux PLL Frequency Calibration Count 6"]
pub struct AUXPLL_FCAL_CNT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Aux PLL Frequency Calibration Count 6"]
pub mod auxpll_fcal_cnt6;
#[doc = "Aux PLL Frequency Calibration Count 5 and 4"]
pub struct AUXPLL_FCAL_CNT5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Aux PLL Frequency Calibration Count 5 and 4"]
pub mod auxpll_fcal_cnt5_4;
#[doc = "Aux PLL Frequency Calibration Count 3 and 2"]
pub struct AUXPLL_FCAL_CNT3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Aux PLL Frequency Calibration Count 3 and 2"]
pub mod auxpll_fcal_cnt3_2;
#[doc = "Aux PLL Frequency Calibration Count 1 and 0"]
pub struct AUXPLL_FCAL_CNT1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Aux PLL Frequency Calibration Count 1 and 0"]
pub mod auxpll_fcal_cnt1_0;
#[doc = "RXDIG DFT"]
pub struct RXDIG_DFT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RXDIG DFT"]
pub mod rxdig_dft;
