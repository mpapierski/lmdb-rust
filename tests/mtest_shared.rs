use std::io;

const TEST_DB: &str = "testdb";

pub fn mtest_wrapper(
    main_0: unsafe fn(libc::c_int, *mut *mut libc::c_char) -> libc::c_int,
) -> io::Result<()> {
    let temp = tempfile::tempdir()?;
    let mut testdb = temp.path().to_path_buf();
    testdb.push(TEST_DB);
    std::fs::create_dir(testdb)?;
    std::env::set_current_dir(temp.path())?;

    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());

    if unsafe {
        main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        )
    } != 0
    {
        panic!("main_0 failed");
    }

    Ok(())
}
