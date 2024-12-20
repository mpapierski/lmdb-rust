use std::{ffi::CStr, ptr::NonNull};

use lmdb_rust::{self, mdb};

#[test]
fn test_version() {
    let mut major = i32::MIN;
    let mut minor = i32::MIN;
    let mut patch = i32::MIN;

    let result_ptr = unsafe {
        mdb::mdb_version(
            NonNull::from(&mut major).as_mut(),
            NonNull::from(&mut minor).as_mut(),
            NonNull::from(&mut patch).as_mut(),
        )
    };
    let safe_ptr = unsafe { CStr::from_ptr(result_ptr) };
    assert_eq!(
        safe_ptr.to_str().unwrap(),
        "LMDB 0.9.70: (December 19, 2015)"
    );
    assert_eq!(major, 0);
    assert_eq!(minor, 9);
    assert_eq!(patch, 70);
}
