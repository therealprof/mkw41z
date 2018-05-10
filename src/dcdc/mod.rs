#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCDC REGISTER 0"]
    pub reg0: REG0,
    #[doc = "0x04 - DCDC REGISTER 1"]
    pub reg1: REG1,
    #[doc = "0x08 - DCDC REGISTER 2"]
    pub reg2: REG2,
    #[doc = "0x0c - DCDC REGISTER 3"]
    pub reg3: REG3,
    #[doc = "0x10 - DCDC REGISTER 4"]
    pub reg4: REG4,
    _reserved0: [u8; 4usize],
    #[doc = "0x18 - DCDC REGISTER 6"]
    pub reg6: REG6,
    #[doc = "0x1c - DCDC REGISTER 7"]
    pub reg7: REG7,
}
#[doc = "DCDC REGISTER 0"]
pub struct REG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC REGISTER 0"]
pub mod reg0;
#[doc = "DCDC REGISTER 1"]
pub struct REG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC REGISTER 1"]
pub mod reg1;
#[doc = "DCDC REGISTER 2"]
pub struct REG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC REGISTER 2"]
pub mod reg2;
#[doc = "DCDC REGISTER 3"]
pub struct REG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC REGISTER 3"]
pub mod reg3;
#[doc = "DCDC REGISTER 4"]
pub struct REG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC REGISTER 4"]
pub mod reg4;
#[doc = "DCDC REGISTER 6"]
pub struct REG6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC REGISTER 6"]
pub mod reg6;
#[doc = "DCDC REGISTER 7"]
pub struct REG7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC REGISTER 7"]
pub mod reg7;
