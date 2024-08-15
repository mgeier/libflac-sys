use std::ffi::CStr;

use libflac_sys as ffi;

#[test]
fn strings() {
    let idx = ffi::FLAC__STREAM_METADATA_PICTURE_TYPE_FISH as usize;
    let ptr = unsafe {
        *ffi::FLAC__StreamMetadata_Picture_TypeString
            .as_ptr()
            .add(idx)
    };
    let c_string = unsafe { CStr::from_ptr(ptr) };
    assert_eq!(c_string.to_str().unwrap(), "A bright coloured fish");
}
