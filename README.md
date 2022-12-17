Rust FFI bindings to the `libFLAC` library
==========================================

This crate provides raw FFI bindings to the `libFLAC` library for reading and
writing losslessly compressed [FLAC] audio files.

[FLAC]: https://xiph.org/flac/

* Crate: https://crates.io/crates/libflac-sys
* Documentation: https://docs.rs/libflac-sys

Following the `*-sys` package conventions,
the `libflac-sys` crate does not define higher-level abstractions over
the native `libFLAC` library functions.


Usage
-----

Add this to your `Cargo.toml`:

```toml
[dependencies]
libflac-sys = "0.3"
```


Features
--------

* `build-flac` (enabled by default): build `libFLAC` instead of linking
   to the system library – `cmake` and a C toolchain is required
* `build-ogg` (enabled by default, implies `build-flac`):
   build support for FLAC data in OGG containers into `libFLAC`;
   if `build-flac` is not selected, support for OGG containers
   depends on the configuration of the system `libFLAC`


Auto-generating the Rust bindings
---------------------------------

The Rust bindings have already been auto-generated with [bindgen]
(using the `bindgen/run-bindgen.sh` script) and are part of this crate
(see `src/bindings.rs`).

[bindgen]: https://crates.io/crates/bindgen


Contributing
------------

If you want to report a problem or suggest an improvement, please go to
<https://github.com/mgeier/libflac-sys>.
Contributions are always welcome!


Licenses
--------

This crate uses the `BSD-3-Clause` license, in reference to
Xiph.Org's BSD-like license which is used as
[`libFLAC` license](https://github.com/xiph/flac/blob/master/COPYING.Xiph) and
[`libogg` license](https://github.com/xiph/ogg/blob/master/COPYING).
