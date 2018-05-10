#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TX Digital Control"]
    pub ctrl: CTRL,
    #[doc = "0x04 - TX Data Padding"]
    pub data_padding: DATA_PADDING,
    #[doc = "0x08 - TX GFSK Modulator Control"]
    pub gfsk_ctrl: GFSK_CTRL,
    #[doc = "0x0c - TX GFSK Filter Coefficients 2"]
    pub gfsk_coeff2: GFSK_COEFF2,
    #[doc = "0x10 - TX GFSK Filter Coefficients 1"]
    pub gfsk_coeff1: GFSK_COEFF1,
    #[doc = "0x14 - TX FSK Modulation Levels"]
    pub fsk_scale: FSK_SCALE,
    #[doc = "0x18 - TX DFT Modulation Pattern"]
    pub dft_pattern: DFT_PATTERN,
    #[doc = "0x1c - TX DFT Control 1"]
    pub rf_dft_bist_1: RF_DFT_BIST_1,
    #[doc = "0x20 - TX DFT Control 2"]
    pub rf_dft_bist_2: RF_DFT_BIST_2,
}
#[doc = "TX Digital Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Digital Control"]
pub mod ctrl;
#[doc = "TX Data Padding"]
pub struct DATA_PADDING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Data Padding"]
pub mod data_padding;
#[doc = "TX GFSK Modulator Control"]
pub struct GFSK_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX GFSK Modulator Control"]
pub mod gfsk_ctrl;
#[doc = "TX GFSK Filter Coefficients 2"]
pub struct GFSK_COEFF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX GFSK Filter Coefficients 2"]
pub mod gfsk_coeff2;
#[doc = "TX GFSK Filter Coefficients 1"]
pub struct GFSK_COEFF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX GFSK Filter Coefficients 1"]
pub mod gfsk_coeff1;
#[doc = "TX FSK Modulation Levels"]
pub struct FSK_SCALE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX FSK Modulation Levels"]
pub mod fsk_scale;
#[doc = "TX DFT Modulation Pattern"]
pub struct DFT_PATTERN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX DFT Modulation Pattern"]
pub mod dft_pattern;
#[doc = "TX DFT Control 1"]
pub struct RF_DFT_BIST_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX DFT Control 1"]
pub mod rf_dft_bist_1;
#[doc = "TX DFT Control 2"]
pub struct RF_DFT_BIST_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX DFT Control 2"]
pub mod rf_dft_bist_2;
