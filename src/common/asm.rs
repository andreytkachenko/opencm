pub fn nop() {
    unsafe {asm!("nop")}
}

pub fn wfi() {
    unsafe {asm!("wfi")}
}