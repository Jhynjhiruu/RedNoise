use std::ffi::{c_char, c_int};
use std::ptr::null;

extern crate sdl2_sys;

extern "C" {
    fn __main(argc: c_int, argv: *const *const c_char) -> c_int;
}

fn main() {
    unsafe { __main(0, null()) };
}
