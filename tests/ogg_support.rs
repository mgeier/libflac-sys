#[test]
#[cfg(feature = "build-flac")]
fn ogg_support() {
    unsafe {
        assert_eq!(
            libflac_sys::FLAC_API_SUPPORTS_OGG_FLAC,
            cfg!(feature = "build-ogg").into()
        );
    }
}
