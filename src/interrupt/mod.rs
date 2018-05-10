use bare_metal::Nr;
#[cfg(feature = "rt")]
extern "C" {
    fn DEFAULT_HANDLER();
}
#[cfg(feature = "rt")]
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DH_TRAMPOLINE() {
    DEFAULT_HANDLER();
}
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak DMA0\nDMA0 = DH_TRAMPOLINE\n.weak DMA1\nDMA1 = DH_TRAMPOLINE\n.weak DMA2\nDMA2 = DH_TRAMPOLINE\n.weak DMA3\nDMA3 = DH_TRAMPOLINE\n.weak FTFA\nFTFA = DH_TRAMPOLINE\n.weak LVD_LVW_DCDC\nLVD_LVW_DCDC = DH_TRAMPOLINE\n.weak LLWU\nLLWU = DH_TRAMPOLINE\n.weak I2C0\nI2C0 = DH_TRAMPOLINE\n.weak I2C1\nI2C1 = DH_TRAMPOLINE\n.weak SPI0\nSPI0 = DH_TRAMPOLINE\n.weak TSI0\nTSI0 = DH_TRAMPOLINE\n.weak LPUART0\nLPUART0 = DH_TRAMPOLINE\n.weak TRNG0\nTRNG0 = DH_TRAMPOLINE\n.weak CMT\nCMT = DH_TRAMPOLINE\n.weak ADC0\nADC0 = DH_TRAMPOLINE\n.weak CMP0\nCMP0 = DH_TRAMPOLINE\n.weak TPM0\nTPM0 = DH_TRAMPOLINE\n.weak TPM1\nTPM1 = DH_TRAMPOLINE\n.weak TPM2\nTPM2 = DH_TRAMPOLINE\n.weak RTC\nRTC = DH_TRAMPOLINE\n.weak RTC_SECONDS\nRTC_SECONDS = DH_TRAMPOLINE\n.weak PIT\nPIT = DH_TRAMPOLINE\n.weak LTC0\nLTC0 = DH_TRAMPOLINE\n.weak DAC0\nDAC0 = DH_TRAMPOLINE\n.weak MCG\nMCG = DH_TRAMPOLINE\n.weak LPTMR0\nLPTMR0 = DH_TRAMPOLINE\n.weak SPI1\nSPI1 = DH_TRAMPOLINE\n.weak PORTA\nPORTA = DH_TRAMPOLINE\n.weak PORTB_PORTC\nPORTB_PORTC = DH_TRAMPOLINE" ) ;
#[cfg(feature = "rt")]
extern "C" {
    fn DMA0();
    fn DMA1();
    fn DMA2();
    fn DMA3();
    fn FTFA();
    fn LVD_LVW_DCDC();
    fn LLWU();
    fn I2C0();
    fn I2C1();
    fn SPI0();
    fn TSI0();
    fn LPUART0();
    fn TRNG0();
    fn CMT();
    fn ADC0();
    fn CMP0();
    fn TPM0();
    fn TPM1();
    fn TPM2();
    fn RTC();
    fn RTC_SECONDS();
    fn PIT();
    fn LTC0();
    fn DAC0();
    fn MCG();
    fn LPTMR0();
    fn SPI1();
    fn PORTA();
    fn PORTB_PORTC();
}
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 32] = [
    Some(DMA0),
    Some(DMA1),
    Some(DMA2),
    Some(DMA3),
    None,
    Some(FTFA),
    Some(LVD_LVW_DCDC),
    Some(LLWU),
    Some(I2C0),
    Some(I2C1),
    Some(SPI0),
    Some(TSI0),
    Some(LPUART0),
    Some(TRNG0),
    Some(CMT),
    Some(ADC0),
    Some(CMP0),
    Some(TPM0),
    Some(TPM1),
    Some(TPM2),
    Some(RTC),
    Some(RTC_SECONDS),
    Some(PIT),
    Some(LTC0),
    None,
    Some(DAC0),
    None,
    Some(MCG),
    Some(LPTMR0),
    Some(SPI1),
    Some(PORTA),
    Some(PORTB_PORTC),
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - DMA0"]
    DMA0,
    #[doc = "1 - DMA1"]
    DMA1,
    #[doc = "2 - DMA2"]
    DMA2,
    #[doc = "3 - DMA3"]
    DMA3,
    #[doc = "5 - FTFA"]
    FTFA,
    #[doc = "6 - LVD_LVW_DCDC"]
    LVD_LVW_DCDC,
    #[doc = "7 - LLWU"]
    LLWU,
    #[doc = "8 - I2C0"]
    I2C0,
    #[doc = "9 - I2C1"]
    I2C1,
    #[doc = "10 - SPI0"]
    SPI0,
    #[doc = "11 - TSI0"]
    TSI0,
    #[doc = "12 - LPUART0"]
    LPUART0,
    #[doc = "13 - TRNG0"]
    TRNG0,
    #[doc = "14 - CMT"]
    CMT,
    #[doc = "15 - ADC0"]
    ADC0,
    #[doc = "16 - CMP0"]
    CMP0,
    #[doc = "17 - TPM0"]
    TPM0,
    #[doc = "18 - TPM1"]
    TPM1,
    #[doc = "19 - TPM2"]
    TPM2,
    #[doc = "20 - RTC"]
    RTC,
    #[doc = "21 - RTC_Seconds"]
    RTC_SECONDS,
    #[doc = "22 - PIT"]
    PIT,
    #[doc = "23 - LTC0"]
    LTC0,
    #[doc = "25 - DAC0"]
    DAC0,
    #[doc = "27 - MCG"]
    MCG,
    #[doc = "28 - LPTMR0"]
    LPTMR0,
    #[doc = "29 - SPI1"]
    SPI1,
    #[doc = "30 - PORTA"]
    PORTA,
    #[doc = "31 - PORTB_PORTC"]
    PORTB_PORTC,
}
unsafe impl Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::DMA0 => 0,
            Interrupt::DMA1 => 1,
            Interrupt::DMA2 => 2,
            Interrupt::DMA3 => 3,
            Interrupt::FTFA => 5,
            Interrupt::LVD_LVW_DCDC => 6,
            Interrupt::LLWU => 7,
            Interrupt::I2C0 => 8,
            Interrupt::I2C1 => 9,
            Interrupt::SPI0 => 10,
            Interrupt::TSI0 => 11,
            Interrupt::LPUART0 => 12,
            Interrupt::TRNG0 => 13,
            Interrupt::CMT => 14,
            Interrupt::ADC0 => 15,
            Interrupt::CMP0 => 16,
            Interrupt::TPM0 => 17,
            Interrupt::TPM1 => 18,
            Interrupt::TPM2 => 19,
            Interrupt::RTC => 20,
            Interrupt::RTC_SECONDS => 21,
            Interrupt::PIT => 22,
            Interrupt::LTC0 => 23,
            Interrupt::DAC0 => 25,
            Interrupt::MCG => 27,
            Interrupt::LPTMR0 => 28,
            Interrupt::SPI1 => 29,
            Interrupt::PORTA => 30,
            Interrupt::PORTB_PORTC => 31,
        }
    }
}
use core::convert::TryFrom;
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl TryFrom<u8> for Interrupt {
    type Error = TryFromInterruptError;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Interrupt::DMA0),
            1 => Ok(Interrupt::DMA1),
            2 => Ok(Interrupt::DMA2),
            3 => Ok(Interrupt::DMA3),
            5 => Ok(Interrupt::FTFA),
            6 => Ok(Interrupt::LVD_LVW_DCDC),
            7 => Ok(Interrupt::LLWU),
            8 => Ok(Interrupt::I2C0),
            9 => Ok(Interrupt::I2C1),
            10 => Ok(Interrupt::SPI0),
            11 => Ok(Interrupt::TSI0),
            12 => Ok(Interrupt::LPUART0),
            13 => Ok(Interrupt::TRNG0),
            14 => Ok(Interrupt::CMT),
            15 => Ok(Interrupt::ADC0),
            16 => Ok(Interrupt::CMP0),
            17 => Ok(Interrupt::TPM0),
            18 => Ok(Interrupt::TPM1),
            19 => Ok(Interrupt::TPM2),
            20 => Ok(Interrupt::RTC),
            21 => Ok(Interrupt::RTC_SECONDS),
            22 => Ok(Interrupt::PIT),
            23 => Ok(Interrupt::LTC0),
            25 => Ok(Interrupt::DAC0),
            27 => Ok(Interrupt::MCG),
            28 => Ok(Interrupt::LPTMR0),
            29 => Ok(Interrupt::SPI1),
            30 => Ok(Interrupt::PORTA),
            31 => Ok(Interrupt::PORTB_PORTC),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
