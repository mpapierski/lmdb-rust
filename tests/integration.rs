use std::{ffi::CStr, ptr::NonNull};

use lmdb_rust::{
    self,
    mdb::{self, MDB_VERSION_STRING},
};

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
        CStr::from_bytes_with_nul(MDB_VERSION_STRING)
            .unwrap()
            .to_str()
            .unwrap()
    );
    assert_eq!(major, 0);
    assert_eq!(minor, 9);
    assert_eq!(patch, 70);
}
