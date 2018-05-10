#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RF Analog Baseband LDO Control 1"]
    pub bb_ldo_1: BB_LDO_1,
    #[doc = "0x04 - RF Analog Baseband LDO Control 2"]
    pub bb_ldo_2: BB_LDO_2,
    #[doc = "0x08 - RF Analog ADC Control"]
    pub rx_adc: RX_ADC,
    #[doc = "0x0c - RF Analog BBA Control"]
    pub rx_bba: RX_BBA,
    #[doc = "0x10 - RF Analog LNA Control"]
    pub rx_lna: RX_LNA,
    #[doc = "0x14 - RF Analog TZA Control"]
    pub rx_tza: RX_TZA,
    #[doc = "0x18 - RF Analog Aux PLL Control"]
    pub rx_auxpll: RX_AUXPLL,
    #[doc = "0x1c - RF Analog Synthesizer Control 1"]
    pub sy_ctrl_1: SY_CTRL_1,
    #[doc = "0x20 - RF Analog Synthesizer Control 2"]
    pub sy_ctrl_2: SY_CTRL_2,
    #[doc = "0x24 - RF Analog TX HPM DAC and PA Control"]
    pub tx_dac_pa: TX_DAC_PA,
    #[doc = "0x28 - RF Analog Balun TX Mode Control"]
    pub balun_tx: BALUN_TX,
    #[doc = "0x2c - RF Analog Balun RX Mode Control"]
    pub balun_rx: BALUN_RX,
    #[doc = "0x30 - RF Analog DFT Observation Register 1"]
    pub dft_obsv_1: DFT_OBSV_1,
    #[doc = "0x34 - RF Analog DFT Observation Register 2"]
    pub dft_obsv_2: DFT_OBSV_2,
}
#[doc = "RF Analog Baseband LDO Control 1"]
pub struct BB_LDO_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog Baseband LDO Control 1"]
pub mod bb_ldo_1;
#[doc = "RF Analog Baseband LDO Control 2"]
pub struct BB_LDO_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog Baseband LDO Control 2"]
pub mod bb_ldo_2;
#[doc = "RF Analog ADC Control"]
pub struct RX_ADC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog ADC Control"]
pub mod rx_adc;
#[doc = "RF Analog BBA Control"]
pub struct RX_BBA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog BBA Control"]
pub mod rx_bba;
#[doc = "RF Analog LNA Control"]
pub struct RX_LNA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog LNA Control"]
pub mod rx_lna;
#[doc = "RF Analog TZA Control"]
pub struct RX_TZA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog TZA Control"]
pub mod rx_tza;
#[doc = "RF Analog Aux PLL Control"]
pub struct RX_AUXPLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog Aux PLL Control"]
pub mod rx_auxpll;
#[doc = "RF Analog Synthesizer Control 1"]
pub struct SY_CTRL_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog Synthesizer Control 1"]
pub mod sy_ctrl_1;
#[doc = "RF Analog Synthesizer Control 2"]
pub struct SY_CTRL_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog Synthesizer Control 2"]
pub mod sy_ctrl_2;
#[doc = "RF Analog TX HPM DAC and PA Control"]
pub struct TX_DAC_PA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog TX HPM DAC and PA Control"]
pub mod tx_dac_pa;
#[doc = "RF Analog Balun TX Mode Control"]
pub struct BALUN_TX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog Balun TX Mode Control"]
pub mod balun_tx;
#[doc = "RF Analog Balun RX Mode Control"]
pub struct BALUN_RX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog Balun RX Mode Control"]
pub mod balun_rx;
#[doc = "RF Analog DFT Observation Register 1"]
pub struct DFT_OBSV_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog DFT Observation Register 1"]
pub mod dft_obsv_1;
#[doc = "RF Analog DFT Observation Register 2"]
pub struct DFT_OBSV_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Analog DFT Observation Register 2"]
pub mod dft_obsv_2;
