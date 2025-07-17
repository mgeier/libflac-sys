#[test]
fn ogg_support() {
    let supports_ogg = unsafe { libflac_sys::FLAC_API_SUPPORTS_OGG_FLAC == 1 };
    if cfg!(feature = "build-flac") {
        assert_eq!(supports_ogg, cfg!(feature = "build-ogg"));
    } else {
        println!("external libFLAC supports OGG: {supports_ogg}");
    }
}
