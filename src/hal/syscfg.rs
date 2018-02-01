use common::VolatileCell;

const SYSCFG_BASE: usize = 0x4001_3800;

#[repr(C)]
struct Registers {
    pub memrm:  VolatileCell<u32>,
    pub pmc:    VolatileCell<u32>,
    pub exticr: [VolatileCell<u32>; 4],
        _reserved0: [u8; 8usize],
    pub cmpcr:  VolatileCell<u32>
}

pub fn select_exti_source(exti: u16, port: u16) {
    let regs: &Registers = unsafe { &*(SYSCFG_BASE as *const Registers) };
    let shift = ((exti as u32) % 4) * 4;
    let reg = &regs.exticr[(exti as usize) / 4];
    let mut val = reg.get() as u32;

    val = val & !(0x0Fu32 << shift as u32);
    val = val | ((port as u32) << (shift as u32));

    reg.set(val);
}

pub fn get_exti_source(exti: u16) -> u16 {
    let regs: &Registers = unsafe { &*(SYSCFG_BASE as *const Registers) };

    let reg = (exti / 4) as usize;
    let shift = (exti % 4) as u32;

    regs.exticr[reg].mask_get(0b1111 << shift, shift as u32) as u16
}