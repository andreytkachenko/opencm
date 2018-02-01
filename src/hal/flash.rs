use common::VolatileCell;

const FLASH_BASE: usize = 0x40023c00;

#[repr(C)]
struct Registers {
    pub acr:    VolatileCell<u32>,
    pub keyr:   VolatileCell<u32>,
    pub optkeyr: VolatileCell<u32>,
    pub sr:     VolatileCell<u32>,
    pub cr:     VolatileCell<u32>,
    pub optcr:  VolatileCell<u32>,
}

pub mod flags {
    pub mod acr {
        pub const DCRST:  u32 = (1 << 12);
        pub const ICRST:  u32 = (1 << 11);
        pub const DCEN:   u32 = (1 << 10);
        pub const ICEN:   u32 = (1 << 9);
        pub const PRFTEN: u32 = (1 << 8);
        pub const LATENCY_MASK: u32 = 0x07;
        pub const LATENCY_0WS: u32 = 0x00;
        pub const LATENCY_2WS: u32 = 0x02;
        pub const LATENCY_1WS: u32 = 0x01;
        pub const LATENCY_3WS: u32 = 0x03;
        pub const LATENCY_4WS: u32 = 0x04;
        pub const LATENCY_5WS: u32 = 0x05;
        pub const LATENCY_6WS: u32 = 0x06;
        pub const LATENCY_7WS: u32 = 0x07;
    }

    pub mod cr {
        pub const LOCK: u32 = (1 << 31);
        pub const ERRIE: u32 = (1 << 25);
        pub const EOPIE: u32 = (1 << 24);
        pub const STRT: u32 = (1 << 16);
        pub const MER: u32 = (1 << 2);
        pub const SER: u32 = (1 << 1);
        pub const PG: u32 = (1 << 0);
        pub const SNB_SHIFT: u32 = 3;
        pub const SNB_MASK: u32 = 0x1f;
        pub const PROGRAM_MASK: u32 = 0x3;
        pub const PROGRAM_SHIFT: u32 = 8;

        pub enum Program {
            X8  = 0,
            X16 = 1,
            X32 = 2,
            X64 = 3 
        }
    }

    pub mod sr {
        pub const BSY: u32 = (1 << 16);
        pub const PGSERR: u32 = (1 << 7);
        pub const PGPERR: u32 = (1 << 6);
        pub const PGAERR: u32 = (1 << 5);
        pub const WRPERR: u32 = (1 << 4);
        pub const OPERR: u32 = (1 << 1);
        pub const EOP: u32 = (1 << 0);
    }
}

pub fn set_ws(ws: u32) {
    let flash: &mut Registers = unsafe { &mut *(FLASH_BASE as *mut Registers) };

    let mut reg32: u32 = flash.acr.get();
    reg32 &= !flags::acr::LATENCY_MASK;
    reg32 |= ws;
    flash.acr.set(reg32);
}