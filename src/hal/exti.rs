use core::ops::Deref;
use common::VolatileCell;

const EXTI: Exti = Exti { addr: 0x4001_3C00 as *const Registers };

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Exti {
    addr: *const Registers
}

impl Exti {
    const fn new(addr: usize) -> Exti {
        Exti {
            addr: addr as *const Registers
        }
    }
}

impl Deref for Exti {
    type Target = Registers;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.addr }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TriggerMode {
    RisingEdge  = 1,
    FallingEdge = 2,
    EitherEdge  = 3
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EdgeType {
    Rising,
    Falling
}

#[repr(C)]
pub struct Registers {
    pub imr:   VolatileCell<u32>,
    pub emr:   VolatileCell<u32>,
    pub rtsr:  VolatileCell<u32>,
    pub ftsr:  VolatileCell<u32>,
    pub swier: VolatileCell<u32>,
    pub pr:    VolatileCell<u32>,
}

pub fn enable(mask: u32) {
    EXTI.imr.check(mask);
    EXTI.emr.check(mask);
}

pub fn disable(mask: u32) {
    EXTI.imr.uncheck(mask);
    EXTI.emr.uncheck(mask);
}

pub fn trigger_on_rising(mask: u32) {
    EXTI.ftsr.uncheck(mask);
    EXTI.rtsr.check(mask);
}

pub fn trigger_on_falling(mask: u32) {
    EXTI.ftsr.check(mask);
    EXTI.rtsr.uncheck(mask);
}

pub fn trigger_on_both(mask: u32) {
    EXTI.ftsr.check(mask);
    EXTI.rtsr.check(mask);
}

pub fn set_trigger(mask: u32, mode: TriggerMode) {
    match mode {
        TriggerMode::RisingEdge => trigger_on_rising(mask),
        TriggerMode::FallingEdge => trigger_on_falling(mask),
        TriggerMode::EitherEdge => trigger_on_both(mask)
    }
}

pub fn is_enabled(mask: u32) -> bool {
    EXTI.imr.test(mask) && EXTI.emr.test(mask)
}

pub fn get_flag_status(mask: u32) -> bool {
    EXTI.pr.test(mask)
}

pub fn reset_flag(mask: u32) {
    EXTI.pr.check(mask);
}
