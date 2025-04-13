use libflac_sys as ffi;

#[test]
fn cuesheet_track_bitfields() {
    let ptr: *mut _ = unsafe { ffi::FLAC__metadata_object_cuesheet_track_new() };
    let track: &mut _ = unsafe { ptr.as_mut().expect("allocation error") };
    assert_eq!(track.type_(), 0);
    assert_eq!(track.pre_emphasis(), 0);
    track.set_type(1);
    track.set_pre_emphasis(1);
    assert_eq!(track.type_(), 1);
    assert_eq!(track.pre_emphasis(), 1);
    unsafe { ffi::FLAC__metadata_object_cuesheet_track_delete(ptr) };
}
