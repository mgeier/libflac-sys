use libflac_sys as ffi;

#[test]
fn ogg_support() {
    unsafe {
        assert_eq!(
            ffi::FLAC_API_SUPPORTS_OGG_FLAC,
            cfg!(feature = "ogg").into()
        );
    }
}
