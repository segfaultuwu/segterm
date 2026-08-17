#![no_std]

extern crate alloc;

pub mod _alloc;
pub mod ansi;
pub mod fb;
pub mod graphics;
pub mod term;

pub use fb::Color;
pub use term::*;

pub fn init() {
    _alloc::heap::init_heap();
}
