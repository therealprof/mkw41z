#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]
#![doc = "Peripheral access API for MKW41Z4 microcontrollers (generated using svd2rust v0.12.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.12.1/svd2rust/#peripheral-api"]
#![allow(private_no_mangle_statics)]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![feature(const_fn)]
#![feature(try_from)]
#![no_std]
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::{default_handler, exception};
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
pub use interrupt::Interrupt;
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::CPUID;
pub use cortex_m::peripheral::DCB;
pub use cortex_m::peripheral::DWT;
pub use cortex_m::peripheral::MPU;
pub use cortex_m::peripheral::NVIC;
pub use cortex_m::peripheral::SCB;
pub use cortex_m::peripheral::SYST;
#[doc = "Flash configuration field"]
pub struct FTFA_FLASHCONFIG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFA_FLASHCONFIG {}
impl FTFA_FLASHCONFIG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftfa_flash_config::RegisterBlock {
        1024 as *const _
    }
}
impl Deref for FTFA_FLASHCONFIG {
    type Target = ftfa_flash_config::RegisterBlock;
    fn deref(&self) -> &ftfa_flash_config::RegisterBlock {
        unsafe { &*FTFA_FLASHCONFIG::ptr() }
    }
}
#[doc = "Flash configuration field"]
pub mod ftfa_flash_config;
#[doc = "Enhanced direct memory access controller"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    fn deref(&self) -> &dma::RegisterBlock {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "Enhanced direct memory access controller"]
pub mod dma;
#[doc = "Flash Memory Interface"]
pub struct FTFA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFA {}
impl FTFA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftfa::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for FTFA {
    type Target = ftfa::RegisterBlock;
    fn deref(&self) -> &ftfa::RegisterBlock {
        unsafe { &*FTFA::ptr() }
    }
}
#[doc = "Flash Memory Interface"]
pub mod ftfa;
#[doc = "DMA channel multiplexor"]
pub struct DMAMUX0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX0 {}
impl DMAMUX0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dmamux0::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for DMAMUX0 {
    type Target = dmamux0::RegisterBlock;
    fn deref(&self) -> &dmamux0::RegisterBlock {
        unsafe { &*DMAMUX0::ptr() }
    }
}
#[doc = "DMA channel multiplexor"]
pub mod dmamux0;
#[doc = "TRNG0"]
pub struct TRNG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG0 {}
impl TRNG0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const trng0::RegisterBlock {
        1073909760 as *const _
    }
}
impl Deref for TRNG0 {
    type Target = trng0::RegisterBlock;
    fn deref(&self) -> &trng0::RegisterBlock {
        unsafe { &*TRNG0::ptr() }
    }
}
#[doc = "TRNG0"]
pub mod trng0;
#[doc = "Serial Peripheral Interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1073922048 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi0;
#[doc = "Serial Peripheral Interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073926144 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi1;
#[doc = "Periodic Interrupt Timer"]
pub struct PIT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIT {}
impl PIT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pit::RegisterBlock {
        1073967104 as *const _
    }
}
impl Deref for PIT {
    type Target = pit::RegisterBlock;
    fn deref(&self) -> &pit::RegisterBlock {
        unsafe { &*PIT::ptr() }
    }
}
#[doc = "Periodic Interrupt Timer"]
pub mod pit;
#[doc = "Timer/PWM Module"]
pub struct TPM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM0 {}
impl TPM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tpm0::RegisterBlock {
        1073971200 as *const _
    }
}
impl Deref for TPM0 {
    type Target = tpm0::RegisterBlock;
    fn deref(&self) -> &tpm0::RegisterBlock {
        unsafe { &*TPM0::ptr() }
    }
}
#[doc = "Timer/PWM Module"]
pub mod tpm0;
#[doc = "Timer/PWM Module"]
pub struct TPM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM1 {}
impl TPM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tpm1::RegisterBlock {
        1073975296 as *const _
    }
}
impl Deref for TPM1 {
    type Target = tpm1::RegisterBlock;
    fn deref(&self) -> &tpm1::RegisterBlock {
        unsafe { &*TPM1::ptr() }
    }
}
#[doc = "Timer/PWM Module"]
pub mod tpm1;
#[doc = "Timer/PWM Module"]
pub struct TPM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM2 {}
impl TPM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tpm2::RegisterBlock {
        1073979392 as *const _
    }
}
impl Deref for TPM2 {
    type Target = tpm2::RegisterBlock;
    fn deref(&self) -> &tpm2::RegisterBlock {
        unsafe { &*TPM2::ptr() }
    }
}
#[doc = "Timer/PWM Module"]
pub mod tpm2;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc0::RegisterBlock {
        1073983488 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &adc0::RegisterBlock {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc0;
#[doc = "Secure Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073991680 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Secure Real Time Clock"]
pub mod rtc;
#[doc = "12-Bit Digital-to-Analog Converter"]
pub struct DAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC0 {}
impl DAC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac0::RegisterBlock {
        1073999872 as *const _
    }
}
impl Deref for DAC0 {
    type Target = dac0::RegisterBlock;
    fn deref(&self) -> &dac0::RegisterBlock {
        unsafe { &*DAC0::ptr() }
    }
}
#[doc = "12-Bit Digital-to-Analog Converter"]
pub mod dac0;
#[doc = "Low Power Timer"]
pub struct LPTMR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTMR0 {}
impl LPTMR0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lptmr0::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for LPTMR0 {
    type Target = lptmr0::RegisterBlock;
    fn deref(&self) -> &lptmr0::RegisterBlock {
        unsafe { &*LPTMR0::ptr() }
    }
}
#[doc = "Low Power Timer"]
pub mod lptmr0;
#[doc = "System register file"]
pub struct RFSYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFSYS {}
impl RFSYS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rfsys::RegisterBlock {
        1074008064 as *const _
    }
}
impl Deref for RFSYS {
    type Target = rfsys::RegisterBlock;
    fn deref(&self) -> &rfsys::RegisterBlock {
        unsafe { &*RFSYS::ptr() }
    }
}
#[doc = "System register file"]
pub mod rfsys;
#[doc = "Touch sense input"]
pub struct TSI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSI0 {}
impl TSI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tsi0::RegisterBlock {
        1074024448 as *const _
    }
}
impl Deref for TSI0 {
    type Target = tsi0::RegisterBlock;
    fn deref(&self) -> &tsi0::RegisterBlock {
        unsafe { &*TSI0::ptr() }
    }
}
#[doc = "Touch sense input"]
pub mod tsi0;
#[doc = "System Integration Module"]
pub struct SIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIM {}
impl SIM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sim::RegisterBlock {
        1074032640 as *const _
    }
}
impl Deref for SIM {
    type Target = sim::RegisterBlock;
    fn deref(&self) -> &sim::RegisterBlock {
        unsafe { &*SIM::ptr() }
    }
}
#[doc = "System Integration Module"]
pub mod sim;
#[doc = "Pin Control and Interrupts"]
pub struct PORTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTA {}
impl PORTA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const porta::RegisterBlock {
        1074040832 as *const _
    }
}
impl Deref for PORTA {
    type Target = porta::RegisterBlock;
    fn deref(&self) -> &porta::RegisterBlock {
        unsafe { &*PORTA::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porta;
#[doc = "Pin Control and Interrupts"]
pub struct PORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTB {}
impl PORTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portb::RegisterBlock {
        1074044928 as *const _
    }
}
impl Deref for PORTB {
    type Target = portb::RegisterBlock;
    fn deref(&self) -> &portb::RegisterBlock {
        unsafe { &*PORTB::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portb;
#[doc = "Pin Control and Interrupts"]
pub struct PORTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTC {}
impl PORTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portc::RegisterBlock {
        1074049024 as *const _
    }
}
impl Deref for PORTC {
    type Target = portc::RegisterBlock;
    fn deref(&self) -> &portc::RegisterBlock {
        unsafe { &*PORTC::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portc;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct LPUART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART0 {}
impl LPUART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpuart0::RegisterBlock {
        1074085888 as *const _
    }
}
impl Deref for LPUART0 {
    type Target = lpuart0::RegisterBlock;
    fn deref(&self) -> &lpuart0::RegisterBlock {
        unsafe { &*LPUART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod lpuart0;
#[doc = "LTC"]
pub struct LTC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LTC0 {}
impl LTC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ltc0::RegisterBlock {
        1074102272 as *const _
    }
}
impl Deref for LTC0 {
    type Target = ltc0::RegisterBlock;
    fn deref(&self) -> &ltc0::RegisterBlock {
        unsafe { &*LTC0::ptr() }
    }
}
#[doc = "LTC"]
pub mod ltc0;
#[doc = "RSIM"]
pub struct RSIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSIM {}
impl RSIM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rsim::RegisterBlock {
        1074106368 as *const _
    }
}
impl Deref for RSIM {
    type Target = rsim::RegisterBlock;
    fn deref(&self) -> &rsim::RegisterBlock {
        unsafe { &*RSIM::ptr() }
    }
}
#[doc = "RSIM"]
pub mod rsim;
#[doc = "DC to DC Converter"]
pub struct DCDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCDC {}
impl DCDC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dcdc::RegisterBlock {
        1074110464 as *const _
    }
}
impl Deref for DCDC {
    type Target = dcdc::RegisterBlock;
    fn deref(&self) -> &dcdc::RegisterBlock {
        unsafe { &*DCDC::ptr() }
    }
}
#[doc = "DC to DC Converter"]
pub mod dcdc;
#[doc = "BLE_RF"]
pub struct BTLE_RF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BTLE_RF {}
impl BTLE_RF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const btle_rf::RegisterBlock {
        1074114560 as *const _
    }
}
impl Deref for BTLE_RF {
    type Target = btle_rf::RegisterBlock;
    fn deref(&self) -> &btle_rf::RegisterBlock {
        unsafe { &*BTLE_RF::ptr() }
    }
}
#[doc = "BLE_RF"]
pub mod btle_rf;
#[doc = "XCVR_RX_DIG"]
pub struct XCVR_RX_DIG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XCVR_RX_DIG {}
impl XCVR_RX_DIG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const xcvr_rx_dig::RegisterBlock {
        1074118656 as *const _
    }
}
impl Deref for XCVR_RX_DIG {
    type Target = xcvr_rx_dig::RegisterBlock;
    fn deref(&self) -> &xcvr_rx_dig::RegisterBlock {
        unsafe { &*XCVR_RX_DIG::ptr() }
    }
}
#[doc = "XCVR_RX_DIG"]
pub mod xcvr_rx_dig;
#[doc = "XCVR_TX_DIG"]
pub struct XCVR_TX_DIG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XCVR_TX_DIG {}
impl XCVR_TX_DIG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const xcvr_tx_dig::RegisterBlock {
        1074119168 as *const _
    }
}
impl Deref for XCVR_TX_DIG {
    type Target = xcvr_tx_dig::RegisterBlock;
    fn deref(&self) -> &xcvr_tx_dig::RegisterBlock {
        unsafe { &*XCVR_TX_DIG::ptr() }
    }
}
#[doc = "XCVR_TX_DIG"]
pub mod xcvr_tx_dig;
#[doc = "XCVR_PLL_DIG"]
pub struct XCVR_PLL_DIG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XCVR_PLL_DIG {}
impl XCVR_PLL_DIG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const xcvr_pll_dig::RegisterBlock {
        1074119204 as *const _
    }
}
impl Deref for XCVR_PLL_DIG {
    type Target = xcvr_pll_dig::RegisterBlock;
    fn deref(&self) -> &xcvr_pll_dig::RegisterBlock {
        unsafe { &*XCVR_PLL_DIG::ptr() }
    }
}
#[doc = "XCVR_PLL_DIG"]
pub mod xcvr_pll_dig;
#[doc = "XCVR_MISC"]
pub struct XCVR_MISC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XCVR_MISC {}
impl XCVR_MISC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const xcvr_misc::RegisterBlock {
        1074119296 as *const _
    }
}
impl Deref for XCVR_MISC {
    type Target = xcvr_misc::RegisterBlock;
    fn deref(&self) -> &xcvr_misc::RegisterBlock {
        unsafe { &*XCVR_MISC::ptr() }
    }
}
#[doc = "XCVR_MISC"]
pub mod xcvr_misc;
#[doc = "XCVR_TSM"]
pub struct XCVR_TSM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XCVR_TSM {}
impl XCVR_TSM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const xcvr_tsm::RegisterBlock {
        1074119360 as *const _
    }
}
impl Deref for XCVR_TSM {
    type Target = xcvr_tsm::RegisterBlock;
    fn deref(&self) -> &xcvr_tsm::RegisterBlock {
        unsafe { &*XCVR_TSM::ptr() }
    }
}
#[doc = "XCVR_TSM"]
pub mod xcvr_tsm;
#[doc = "XCVR_PHY"]
pub struct XCVR_PHY {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XCVR_PHY {}
impl XCVR_PHY {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const xcvr_phy::RegisterBlock {
        1074119680 as *const _
    }
}
impl Deref for XCVR_PHY {
    type Target = xcvr_phy::RegisterBlock;
    fn deref(&self) -> &xcvr_phy::RegisterBlock {
        unsafe { &*XCVR_PHY::ptr() }
    }
}
#[doc = "XCVR_PHY"]
pub mod xcvr_phy;
#[doc = "XCVR_ZBDEMOD"]
pub struct XCVR_ZBDEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XCVR_ZBDEM {}
impl XCVR_ZBDEM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const xcvr_zbdem::RegisterBlock {
        1074119808 as *const _
    }
}
impl Deref for XCVR_ZBDEM {
    type Target = xcvr_zbdem::RegisterBlock;
    fn deref(&self) -> &xcvr_zbdem::RegisterBlock {
        unsafe { &*XCVR_ZBDEM::ptr() }
    }
}
#[doc = "XCVR_ZBDEMOD"]
pub mod xcvr_zbdem;
#[doc = "XCVR_ANALOG"]
pub struct XCVR_ANA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XCVR_ANA {}
impl XCVR_ANA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const xcvr_ana::RegisterBlock {
        1074119936 as *const _
    }
}
impl Deref for XCVR_ANA {
    type Target = xcvr_ana::RegisterBlock;
    fn deref(&self) -> &xcvr_ana::RegisterBlock {
        unsafe { &*XCVR_ANA::ptr() }
    }
}
#[doc = "XCVR_ANALOG"]
pub mod xcvr_ana;
#[doc = "XCVR_PKT_RAM"]
pub struct XCVR_PKT_RAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XCVR_PKT_RAM {}
impl XCVR_PKT_RAM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const xcvr_pkt_ram::RegisterBlock {
        1074120448 as *const _
    }
}
impl Deref for XCVR_PKT_RAM {
    type Target = xcvr_pkt_ram::RegisterBlock;
    fn deref(&self) -> &xcvr_pkt_ram::RegisterBlock {
        unsafe { &*XCVR_PKT_RAM::ptr() }
    }
}
#[doc = "XCVR_PKT_RAM"]
pub mod xcvr_pkt_ram;
#[doc = "ZLL"]
pub struct ZLL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ZLL {}
impl ZLL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const zll::RegisterBlock {
        1074122752 as *const _
    }
}
impl Deref for ZLL {
    type Target = zll::RegisterBlock;
    fn deref(&self) -> &zll::RegisterBlock {
        unsafe { &*ZLL::ptr() }
    }
}
#[doc = "ZLL"]
pub mod zll;
#[doc = "ANT"]
pub struct ANT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ANT {}
impl ANT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ant::RegisterBlock {
        1074126848 as *const _
    }
}
impl Deref for ANT {
    type Target = ant::RegisterBlock;
    fn deref(&self) -> &ant::RegisterBlock {
        unsafe { &*ANT::ptr() }
    }
}
#[doc = "ANT"]
pub mod ant;
#[doc = "GENERIC_FSK"]
pub struct GENFSK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GENFSK {}
impl GENFSK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const genfsk::RegisterBlock {
        1074130944 as *const _
    }
}
impl Deref for GENFSK {
    type Target = genfsk::RegisterBlock;
    fn deref(&self) -> &genfsk::RegisterBlock {
        unsafe { &*GENFSK::ptr() }
    }
}
#[doc = "GENERIC_FSK"]
pub mod genfsk;
#[doc = "Carrier Modulator Transmitter"]
pub struct CMT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMT {}
impl CMT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmt::RegisterBlock {
        1074143232 as *const _
    }
}
impl Deref for CMT {
    type Target = cmt::RegisterBlock;
    fn deref(&self) -> &cmt::RegisterBlock {
        unsafe { &*CMT::ptr() }
    }
}
#[doc = "Carrier Modulator Transmitter"]
pub mod cmt;
#[doc = "Multipurpose Clock Generator module"]
pub struct MCG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCG {}
impl MCG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcg::RegisterBlock {
        1074151424 as *const _
    }
}
impl Deref for MCG {
    type Target = mcg::RegisterBlock;
    fn deref(&self) -> &mcg::RegisterBlock {
        unsafe { &*MCG::ptr() }
    }
}
#[doc = "Multipurpose Clock Generator module"]
pub mod mcg;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074159616 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c0;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1074163712 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c1;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP0 {}
impl CMP0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmp0::RegisterBlock {
        1074212864 as *const _
    }
}
impl Deref for CMP0 {
    type Target = cmp0::RegisterBlock;
    fn deref(&self) -> &cmp0::RegisterBlock {
        unsafe { &*CMP0::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp0;
#[doc = "Voltage Reference"]
pub struct VREF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREF {}
impl VREF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vref::RegisterBlock {
        1074216960 as *const _
    }
}
impl Deref for VREF {
    type Target = vref::RegisterBlock;
    fn deref(&self) -> &vref::RegisterBlock {
        unsafe { &*VREF::ptr() }
    }
}
#[doc = "Voltage Reference"]
pub mod vref;
#[doc = "Low leakage wakeup unit"]
pub struct LLWU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LLWU {}
impl LLWU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const llwu::RegisterBlock {
        1074249728 as *const _
    }
}
impl Deref for LLWU {
    type Target = llwu::RegisterBlock;
    fn deref(&self) -> &llwu::RegisterBlock {
        unsafe { &*LLWU::ptr() }
    }
}
#[doc = "Low leakage wakeup unit"]
pub mod llwu;
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmc::RegisterBlock {
        1074253824 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    fn deref(&self) -> &pmc::RegisterBlock {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "System Mode Controller"]
pub struct SMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC {}
impl SMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const smc::RegisterBlock {
        1074257920 as *const _
    }
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    fn deref(&self) -> &smc::RegisterBlock {
        unsafe { &*SMC::ptr() }
    }
}
#[doc = "System Mode Controller"]
pub mod smc;
#[doc = "Reset Control Module"]
pub struct RCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCM {}
impl RCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcm::RegisterBlock {
        1074262016 as *const _
    }
}
impl Deref for RCM {
    type Target = rcm::RegisterBlock;
    fn deref(&self) -> &rcm::RegisterBlock {
        unsafe { &*RCM::ptr() }
    }
}
#[doc = "Reset Control Module"]
pub mod rcm;
#[doc = "General Purpose Input/Output"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1074786304 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioa;
#[doc = "General Purpose Input/Output"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1074786368 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiob;
#[doc = "General Purpose Input/Output"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1074786432 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioc;
#[doc = "System Control Block"]
pub struct SYSTEMCONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEMCONTROL {}
impl SYSTEMCONTROL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const system_control::RegisterBlock {
        3758153728 as *const _
    }
}
impl Deref for SYSTEMCONTROL {
    type Target = system_control::RegisterBlock;
    fn deref(&self) -> &system_control::RegisterBlock {
        unsafe { &*SYSTEMCONTROL::ptr() }
    }
}
#[doc = "System Control Block"]
pub mod system_control;
#[doc = "System timer"]
pub struct SYSTICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTICK {}
impl SYSTICK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sys_tick::RegisterBlock {
        3758153744 as *const _
    }
}
impl Deref for SYSTICK {
    type Target = sys_tick::RegisterBlock;
    fn deref(&self) -> &sys_tick::RegisterBlock {
        unsafe { &*SYSTICK::ptr() }
    }
}
#[doc = "System timer"]
pub mod sys_tick;
#[doc = "Micro Trace Buffer"]
pub struct MTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTB {}
impl MTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mtb::RegisterBlock {
        4026531840 as *const _
    }
}
impl Deref for MTB {
    type Target = mtb::RegisterBlock;
    fn deref(&self) -> &mtb::RegisterBlock {
        unsafe { &*MTB::ptr() }
    }
}
#[doc = "Micro Trace Buffer"]
pub mod mtb;
#[doc = "MTB data watchpoint and trace"]
pub struct MTBDWT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTBDWT {}
impl MTBDWT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mtbdwt::RegisterBlock {
        4026535936 as *const _
    }
}
impl Deref for MTBDWT {
    type Target = mtbdwt::RegisterBlock;
    fn deref(&self) -> &mtbdwt::RegisterBlock {
        unsafe { &*MTBDWT::ptr() }
    }
}
#[doc = "MTB data watchpoint and trace"]
pub mod mtbdwt;
#[doc = "System ROM"]
pub struct ROM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ROM {}
impl ROM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rom::RegisterBlock {
        4026540032 as *const _
    }
}
impl Deref for ROM {
    type Target = rom::RegisterBlock;
    fn deref(&self) -> &rom::RegisterBlock {
        unsafe { &*ROM::ptr() }
    }
}
#[doc = "System ROM"]
pub mod rom;
#[doc = "Core Platform Miscellaneous Control Module"]
pub struct MCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCM {}
impl MCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcm::RegisterBlock {
        4026544128 as *const _
    }
}
impl Deref for MCM {
    type Target = mcm::RegisterBlock;
    fn deref(&self) -> &mcm::RegisterBlock {
        unsafe { &*MCM::ptr() }
    }
}
#[doc = "Core Platform Miscellaneous Control Module"]
pub mod mcm;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOA {}
impl FGPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fgpioa::RegisterBlock {
        4160749568 as *const _
    }
}
impl Deref for FGPIOA {
    type Target = fgpioa::RegisterBlock;
    fn deref(&self) -> &fgpioa::RegisterBlock {
        unsafe { &*FGPIOA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioa;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOB {}
impl FGPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fgpiob::RegisterBlock {
        4160749632 as *const _
    }
}
impl Deref for FGPIOB {
    type Target = fgpiob::RegisterBlock;
    fn deref(&self) -> &fgpiob::RegisterBlock {
        unsafe { &*FGPIOB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiob;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOC {}
impl FGPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fgpioc::RegisterBlock {
        4160749696 as *const _
    }
}
impl Deref for FGPIOC {
    type Target = fgpioc::RegisterBlock;
    fn deref(&self) -> &fgpioc::RegisterBlock {
        unsafe { &*FGPIOC::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioc;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FTFA_FLASHCONFIG"]
    pub FTFA_FLASHCONFIG: FTFA_FLASHCONFIG,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "FTFA"]
    pub FTFA: FTFA,
    #[doc = "DMAMUX0"]
    pub DMAMUX0: DMAMUX0,
    #[doc = "TRNG0"]
    pub TRNG0: TRNG0,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "PIT"]
    pub PIT: PIT,
    #[doc = "TPM0"]
    pub TPM0: TPM0,
    #[doc = "TPM1"]
    pub TPM1: TPM1,
    #[doc = "TPM2"]
    pub TPM2: TPM2,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "DAC0"]
    pub DAC0: DAC0,
    #[doc = "LPTMR0"]
    pub LPTMR0: LPTMR0,
    #[doc = "RFSYS"]
    pub RFSYS: RFSYS,
    #[doc = "TSI0"]
    pub TSI0: TSI0,
    #[doc = "SIM"]
    pub SIM: SIM,
    #[doc = "PORTA"]
    pub PORTA: PORTA,
    #[doc = "PORTB"]
    pub PORTB: PORTB,
    #[doc = "PORTC"]
    pub PORTC: PORTC,
    #[doc = "LPUART0"]
    pub LPUART0: LPUART0,
    #[doc = "LTC0"]
    pub LTC0: LTC0,
    #[doc = "RSIM"]
    pub RSIM: RSIM,
    #[doc = "DCDC"]
    pub DCDC: DCDC,
    #[doc = "BTLE_RF"]
    pub BTLE_RF: BTLE_RF,
    #[doc = "XCVR_RX_DIG"]
    pub XCVR_RX_DIG: XCVR_RX_DIG,
    #[doc = "XCVR_TX_DIG"]
    pub XCVR_TX_DIG: XCVR_TX_DIG,
    #[doc = "XCVR_PLL_DIG"]
    pub XCVR_PLL_DIG: XCVR_PLL_DIG,
    #[doc = "XCVR_MISC"]
    pub XCVR_MISC: XCVR_MISC,
    #[doc = "XCVR_TSM"]
    pub XCVR_TSM: XCVR_TSM,
    #[doc = "XCVR_PHY"]
    pub XCVR_PHY: XCVR_PHY,
    #[doc = "XCVR_ZBDEM"]
    pub XCVR_ZBDEM: XCVR_ZBDEM,
    #[doc = "XCVR_ANA"]
    pub XCVR_ANA: XCVR_ANA,
    #[doc = "XCVR_PKT_RAM"]
    pub XCVR_PKT_RAM: XCVR_PKT_RAM,
    #[doc = "ZLL"]
    pub ZLL: ZLL,
    #[doc = "ANT"]
    pub ANT: ANT,
    #[doc = "GENFSK"]
    pub GENFSK: GENFSK,
    #[doc = "CMT"]
    pub CMT: CMT,
    #[doc = "MCG"]
    pub MCG: MCG,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "CMP0"]
    pub CMP0: CMP0,
    #[doc = "VREF"]
    pub VREF: VREF,
    #[doc = "LLWU"]
    pub LLWU: LLWU,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "SMC"]
    pub SMC: SMC,
    #[doc = "RCM"]
    pub RCM: RCM,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "SYSTEMCONTROL"]
    pub SYSTEMCONTROL: SYSTEMCONTROL,
    #[doc = "SYSTICK"]
    pub SYSTICK: SYSTICK,
    #[doc = "MTB"]
    pub MTB: MTB,
    #[doc = "MTBDWT"]
    pub MTBDWT: MTBDWT,
    #[doc = "ROM"]
    pub ROM: ROM,
    #[doc = "MCM"]
    pub MCM: MCM,
    #[doc = "FGPIOA"]
    pub FGPIOA: FGPIOA,
    #[doc = "FGPIOB"]
    pub FGPIOB: FGPIOB,
    #[doc = "FGPIOC"]
    pub FGPIOC: FGPIOC,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FTFA_FLASHCONFIG: FTFA_FLASHCONFIG {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            FTFA: FTFA {
                _marker: PhantomData,
            },
            DMAMUX0: DMAMUX0 {
                _marker: PhantomData,
            },
            TRNG0: TRNG0 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            PIT: PIT {
                _marker: PhantomData,
            },
            TPM0: TPM0 {
                _marker: PhantomData,
            },
            TPM1: TPM1 {
                _marker: PhantomData,
            },
            TPM2: TPM2 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            DAC0: DAC0 {
                _marker: PhantomData,
            },
            LPTMR0: LPTMR0 {
                _marker: PhantomData,
            },
            RFSYS: RFSYS {
                _marker: PhantomData,
            },
            TSI0: TSI0 {
                _marker: PhantomData,
            },
            SIM: SIM {
                _marker: PhantomData,
            },
            PORTA: PORTA {
                _marker: PhantomData,
            },
            PORTB: PORTB {
                _marker: PhantomData,
            },
            PORTC: PORTC {
                _marker: PhantomData,
            },
            LPUART0: LPUART0 {
                _marker: PhantomData,
            },
            LTC0: LTC0 {
                _marker: PhantomData,
            },
            RSIM: RSIM {
                _marker: PhantomData,
            },
            DCDC: DCDC {
                _marker: PhantomData,
            },
            BTLE_RF: BTLE_RF {
                _marker: PhantomData,
            },
            XCVR_RX_DIG: XCVR_RX_DIG {
                _marker: PhantomData,
            },
            XCVR_TX_DIG: XCVR_TX_DIG {
                _marker: PhantomData,
            },
            XCVR_PLL_DIG: XCVR_PLL_DIG {
                _marker: PhantomData,
            },
            XCVR_MISC: XCVR_MISC {
                _marker: PhantomData,
            },
            XCVR_TSM: XCVR_TSM {
                _marker: PhantomData,
            },
            XCVR_PHY: XCVR_PHY {
                _marker: PhantomData,
            },
            XCVR_ZBDEM: XCVR_ZBDEM {
                _marker: PhantomData,
            },
            XCVR_ANA: XCVR_ANA {
                _marker: PhantomData,
            },
            XCVR_PKT_RAM: XCVR_PKT_RAM {
                _marker: PhantomData,
            },
            ZLL: ZLL {
                _marker: PhantomData,
            },
            ANT: ANT {
                _marker: PhantomData,
            },
            GENFSK: GENFSK {
                _marker: PhantomData,
            },
            CMT: CMT {
                _marker: PhantomData,
            },
            MCG: MCG {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            CMP0: CMP0 {
                _marker: PhantomData,
            },
            VREF: VREF {
                _marker: PhantomData,
            },
            LLWU: LLWU {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            SMC: SMC {
                _marker: PhantomData,
            },
            RCM: RCM {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            SYSTEMCONTROL: SYSTEMCONTROL {
                _marker: PhantomData,
            },
            SYSTICK: SYSTICK {
                _marker: PhantomData,
            },
            MTB: MTB {
                _marker: PhantomData,
            },
            MTBDWT: MTBDWT {
                _marker: PhantomData,
            },
            ROM: ROM {
                _marker: PhantomData,
            },
            MCM: MCM {
                _marker: PhantomData,
            },
            FGPIOA: FGPIOA {
                _marker: PhantomData,
            },
            FGPIOB: FGPIOB {
                _marker: PhantomData,
            },
            FGPIOC: FGPIOC {
                _marker: PhantomData,
            },
        }
    }
}
