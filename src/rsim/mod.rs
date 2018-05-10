#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Radio System Control"]
    pub control: CONTROL,
    #[doc = "0x04 - Radio Active Early Warning"]
    pub active_delay: ACTIVE_DELAY,
    #[doc = "0x08 - Radio MAC Address"]
    pub mac_msb: MAC_MSB,
    #[doc = "0x0c - Radio MAC Address"]
    pub mac_lsb: MAC_LSB,
    #[doc = "0x10 - Radio Miscellaneous"]
    pub misc: MISC,
    _reserved0: [u8; 236usize],
    #[doc = "0x100 - Deep Sleep Timer"]
    pub dsm_timer: DSM_TIMER,
    #[doc = "0x104 - Deep Sleep Timer Control"]
    pub dsm_control: DSM_CONTROL,
    #[doc = "0x108 - Deep Sleep Wakeup Time Offset"]
    pub dsm_osc_offset: DSM_OSC_OFFSET,
    #[doc = "0x10c - ANT Link Layer Sleep Time"]
    pub ant_sleep: ANT_SLEEP,
    #[doc = "0x110 - ANT Link Layer Wake Time"]
    pub ant_wake: ANT_WAKE,
    #[doc = "0x114 - 802.15.4 Link Layer Sleep Time"]
    pub zig_sleep: ZIG_SLEEP,
    #[doc = "0x118 - 802.15.4 Link Layer Wake Time"]
    pub zig_wake: ZIG_WAKE,
    #[doc = "0x11c - Generic FSK Link Layer Sleep Time"]
    pub gen_sleep: GEN_SLEEP,
    #[doc = "0x120 - Generic FSK Link Layer Wake Time"]
    pub gen_wake: GEN_WAKE,
    #[doc = "0x124 - Radio Oscillator Control"]
    pub rf_osc_ctrl: RF_OSC_CTRL,
    #[doc = "0x128 - Radio Analog Test Registers"]
    pub ana_test: ANA_TEST,
    #[doc = "0x12c - Radio Analog Trim Registers"]
    pub ana_trim: ANA_TRIM,
}
#[doc = "Radio System Control"]
pub struct CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio System Control"]
pub mod control;
#[doc = "Radio Active Early Warning"]
pub struct ACTIVE_DELAY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio Active Early Warning"]
pub mod active_delay;
#[doc = "Radio MAC Address"]
pub struct MAC_MSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio MAC Address"]
pub mod mac_msb;
#[doc = "Radio MAC Address"]
pub struct MAC_LSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio MAC Address"]
pub mod mac_lsb;
#[doc = "Radio Miscellaneous"]
pub struct MISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio Miscellaneous"]
pub mod misc;
#[doc = "Deep Sleep Timer"]
pub struct DSM_TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deep Sleep Timer"]
pub mod dsm_timer;
#[doc = "Deep Sleep Timer Control"]
pub struct DSM_CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deep Sleep Timer Control"]
pub mod dsm_control;
#[doc = "Deep Sleep Wakeup Time Offset"]
pub struct DSM_OSC_OFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deep Sleep Wakeup Time Offset"]
pub mod dsm_osc_offset;
#[doc = "ANT Link Layer Sleep Time"]
pub struct ANT_SLEEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANT Link Layer Sleep Time"]
pub mod ant_sleep;
#[doc = "ANT Link Layer Wake Time"]
pub struct ANT_WAKE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANT Link Layer Wake Time"]
pub mod ant_wake;
#[doc = "802.15.4 Link Layer Sleep Time"]
pub struct ZIG_SLEEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "802.15.4 Link Layer Sleep Time"]
pub mod zig_sleep;
#[doc = "802.15.4 Link Layer Wake Time"]
pub struct ZIG_WAKE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "802.15.4 Link Layer Wake Time"]
pub mod zig_wake;
#[doc = "Generic FSK Link Layer Sleep Time"]
pub struct GEN_SLEEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Generic FSK Link Layer Sleep Time"]
pub mod gen_sleep;
#[doc = "Generic FSK Link Layer Wake Time"]
pub struct GEN_WAKE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Generic FSK Link Layer Wake Time"]
pub mod gen_wake;
#[doc = "Radio Oscillator Control"]
pub struct RF_OSC_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio Oscillator Control"]
pub mod rf_osc_ctrl;
#[doc = "Radio Analog Test Registers"]
pub struct ANA_TEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio Analog Test Registers"]
pub mod ana_test;
#[doc = "Radio Analog Trim Registers"]
pub struct ANA_TRIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio Analog Trim Registers"]
pub mod ana_trim;
