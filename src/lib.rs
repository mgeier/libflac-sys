//! Raw FFI bindings to the `libFLAC` library.
//!
//! Original C API documentation: <https://xiph.org/flac/api/>

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]
// https://github.com/rust-lang/rust-bindgen/issues/3053
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::useless_transmute)]

include!("bindings.rs");

#[allow(clippy::upper_case_acronyms)]
pub type FILE = libc::FILE;
