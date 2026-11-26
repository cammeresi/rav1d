//! C types and `errno` values.

#![allow(non_camel_case_types)]

pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub use libc::{off_t, EAGAIN, EINVAL, ENOENT, ENOMEM, ENOPROTOOPT, ERANGE};

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub use self::wasm_unknown::*;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
mod wasm_unknown {
    use std::ffi::c_int;

    pub type off_t = i64;

    // Since wasm*-unknown-unknown doesn't have a C library, these values only
    // need to be distinct, nonzero, different from `Rav1dError::Other`, and
    // small enough to fit in a `u8`.
    pub const EAGAIN: c_int = 2;
    pub const EINVAL: c_int = 3;
    pub const ENOENT: c_int = 4;
    pub const ENOMEM: c_int = 5;
    pub const ENOPROTOOPT: c_int = 6;
    pub const ERANGE: c_int = 7;
}
