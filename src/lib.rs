/*!
# FFI bindings to the libFLAC C library

https://xiph.org/flac/api/

Following the `*-sys` package conventions,
the `libflac-sys` crate does not define higher-level abstractions over
the native `libFLAC` library functions.
*/
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!("bindings.rs");

pub type FILE = libc::FILE;
