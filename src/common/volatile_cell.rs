use core::ops::{BitOrAssign, BitAndAssign, BitOr, BitAnd, Shl, Shr, Not};

pub trait Zero {
    fn zero() -> Self; 
}

pub trait Radix : Copy + PartialEq 
+ BitOr<Output = Self> + BitAnd<Output = Self>
+ Shl<Self, Output = Self> + Shr<Self, Output = Self> + Not<Output = Self>
+ BitOrAssign<Self> + BitAndAssign<Self> + Zero {}

impl Radix for u8 {}
impl Radix for u16 {}
impl Radix for u32 {}
impl Radix for u64 {}
impl Radix for usize {}

impl Zero for u8 { fn zero() -> Self { 0 } }
impl Zero for u16 { fn zero() -> Self { 0 } }
impl Zero for u32 { fn zero() -> Self { 0 } }
impl Zero for u64 { fn zero() -> Self { 0 } }
impl Zero for usize { fn zero() -> Self { 0 } }

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VolatileCell<T: Radix> {
    value: T,
}

#[allow(dead_code)]
impl<T: Radix> VolatileCell<T> {
    pub const fn new(value: T) -> Self {
        VolatileCell { value }
    }

    #[inline(always)]
    pub fn get(&self) -> T {
        unsafe { ::core::ptr::read_volatile(&self.value) }
    }

    #[inline(always)]
    pub fn set(&self, value: T) {
        unsafe { ::core::ptr::write_volatile(&self.value as *const T as *mut T, value) };
    }

    pub fn update<F: FnOnce(T) -> T>(&self, f: F) {
        self.set(f(self.get()));
    }

    pub fn mask_set(&self, mask: T, shift: T, val: T) {
        let mut reg = self.get();
        reg = (reg & !mask) | (val << shift);
        self.set(reg);
    }

    pub fn mask_get(&self, mask: T, shift: T) -> T {
        (self.get() & !mask) >> shift
    }

    pub fn test(&self, val: T) -> bool {
        self.get() & val != T::zero()
    }

    pub fn check(&self, val: T) {
        let mut reg = self.get();
        reg |= val;
        self.set(reg);
    }

    pub fn uncheck(&self, val: T) {
        let mut reg = self.get();
        reg &= !val;
        self.set(reg);
    }
}