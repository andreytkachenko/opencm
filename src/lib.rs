#![feature(const_fn, asm)]
#![no_std]
#![allow(dead_code)]
pub mod common;
pub mod hal;
pub use self::hal::*;