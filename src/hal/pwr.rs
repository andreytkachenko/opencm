use common::VolatileCell;

const FLASH_BASE: usize = 0x40007000;

#[repr(C)]
struct Registers {
    pub cr:    VolatileCell<u32>,
    pub csr:   VolatileCell<u32>,
}

pub mod flags {
    pub mod cr {
        pub const VOS: u32  = 1 << 14;
        pub const LPRUN: u32  = 1 << 14;
        pub const FWU: u32  = 1 << 10;
        pub const FPDS: u32 = 1 << 9;
        pub const ULP: u32  = 1 << 9;
        pub const DBP: u32  = 1 << 8;
        pub const PVDE: u32 = 1 << 4;
        pub const CSBF: u32 = 1 << 3;
        pub const CWUF: u32 = 1 << 2;
        pub const PDDS: u32 = 1 << 1;
        pub const LPDS: u32 = 1 << 0;
        pub const LPSDSR: u32 = 1 << 0;

        pub const VOS_LSB: u32    = 11;
        pub const VOS_RANGE1: u32 = 0x1 << VOS_LSB;
        pub const VOS_RANGE2: u32 = 0x2 << VOS_LSB;
        pub const VOS_RANGE3: u32 = 0x3 << VOS_LSB;
        pub const VOS_MASK: u32   = 0b11 << VOS_LSB;
    }

    pub mod csr {
        pub const VOSRDY: u32 = 1 << 14;
        pub const BRE: u32    = 1 << 9;
        pub const EWUP2: u32  = 1 << 9;
        pub const EWUP: u32   = 1 << 8;
        pub const EWUP1: u32  = EWUP;
        pub const REGLPF: u32 = 1 << 5;
        pub const VOSF: u32   = 1 << 4;
        pub const VREFINTRDYF: u32 = 1 << 3;
        pub const BRR: u32    = 1 << 3;
        pub const PVDO: u32   = 1 << 2;
        pub const SBF: u32    = 1 << 1;
        pub const WUF: u32    = 1 << 0;
    }

    pub enum VOSScale {
        Scale1 = 0,
        Scale2 = 1,
        Scale3 = 2,
    }
}

pub fn set_vos_scale(scale: flags::VOSScale) {
    let pwr: &Registers = unsafe { &*(FLASH_BASE as *mut Registers) };
    let mut cr = pwr.cr.get();

	cr &= !flags::cr::VOS_MASK;
	cr |= match scale {
        flags::VOSScale::Scale1 => flags::cr::VOS_RANGE1,
        flags::VOSScale::Scale2 => flags::cr::VOS_RANGE2,
        flags::VOSScale::Scale3 => flags::cr::VOS_RANGE3,
    };

    pwr.cr.set(cr);
}