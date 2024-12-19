use ::libc;
extern "C" {
    pub type __sFILEX;
    pub type MDB_env;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn signal(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    fn exit(_: libc::c_int) -> !;
    fn mdb_strerror(err: libc::c_int) -> *mut libc::c_char;
    fn mdb_env_create(env: *mut *mut MDB_env) -> libc::c_int;
    fn mdb_env_open(
        env: *mut MDB_env,
        path: *const libc::c_char,
        flags: libc::c_uint,
        mode: mdb_mode_t,
    ) -> libc::c_int;
    fn mdb_env_copy2(
        env: *mut MDB_env,
        path: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn mdb_env_copyfd2(
        env: *mut MDB_env,
        fd: mdb_filehandle_t,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn mdb_env_close(env: *mut MDB_env);
}
pub type __uint16_t = libc::c_ushort;
pub type __int64_t = libc::c_longlong;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub _read: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub _seek: Option::<
        unsafe extern "C" fn(*mut libc::c_void, fpos_t, libc::c_int) -> fpos_t,
    >,
    pub _write: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type mode_t = __darwin_mode_t;
pub type mdb_mode_t = mode_t;
pub type mdb_filehandle_t = libc::c_int;
pub const MDB_STDOUT: libc::c_int = 1 as libc::c_int;
pub const SIGHUP: libc::c_int = 1 as libc::c_int;
pub const SIGINT: libc::c_int = 2 as libc::c_int;
pub const SIGPIPE: libc::c_int = 13 as libc::c_int;
pub const SIGTERM: libc::c_int = 15 as libc::c_int;
pub const EXIT_FAILURE: libc::c_int = 1 as libc::c_int;
pub const EXIT_SUCCESS: libc::c_int = 0 as libc::c_int;
pub const MDB_VERSION_STRING: [libc::c_char; 33] = unsafe {
    *::core::mem::transmute::<
        &[u8; 33],
        &[libc::c_char; 33],
    >(b"LMDB 0.9.70: (December 19, 2015)\0")
};
pub const MDB_NOSUBDIR: libc::c_int = 0x4000 as libc::c_int;
pub const MDB_RDONLY: libc::c_int = 0x20000 as libc::c_int;
pub const MDB_PREVSNAPSHOT: libc::c_int = 0x2000000 as libc::c_int;
pub const MDB_CP_COMPACT: libc::c_int = 0x1 as libc::c_int;
pub const MDB_SUCCESS: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn sighandle(mut sig: libc::c_int) {}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut env = 0 as *mut MDB_env;
    let mut progname: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut act = 0 as *const libc::c_char;
    let mut flags = MDB_RDONLY as libc::c_uint;
    let mut cpflags = 0 as libc::c_int as libc::c_uint;
    while argc > 1 as libc::c_int
        && *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
    {
        if *(*argv.offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int == 'n' as i32
            && *(*argv.offset(1 as libc::c_int as isize))
                .offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            flags |= MDB_NOSUBDIR as libc::c_uint;
        } else if *(*argv.offset(1 as libc::c_int as isize))
            .offset(1 as libc::c_int as isize) as libc::c_int == 'v' as i32
            && *(*argv.offset(1 as libc::c_int as isize))
                .offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            flags |= MDB_PREVSNAPSHOT as libc::c_uint;
        } else if *(*argv.offset(1 as libc::c_int as isize))
            .offset(1 as libc::c_int as isize) as libc::c_int == 'c' as i32
            && *(*argv.offset(1 as libc::c_int as isize))
                .offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            cpflags |= MDB_CP_COMPACT as libc::c_uint;
        } else if *(*argv.offset(1 as libc::c_int as isize))
            .offset(1 as libc::c_int as isize) as libc::c_int == 'V' as i32
            && *(*argv.offset(1 as libc::c_int as isize))
                .offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                MDB_VERSION_STRING.as_ptr(),
            );
            exit(0 as libc::c_int);
        } else {
            argc = 0 as libc::c_int;
        }
        argc -= 1;
        argc;
        argv = argv.offset(1);
        argv;
    }
    if argc < 2 as libc::c_int || argc > 3 as libc::c_int {
        fprintf(
            __stderrp,
            b"usage: %s [-V] [-c] [-n] [-v] srcpath [dstpath]\n\0" as *const u8
                as *const libc::c_char,
            progname,
        );
        exit(EXIT_FAILURE);
    }
    signal(SIGPIPE, Some(sighandle as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(SIGHUP, Some(sighandle as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(SIGINT, Some(sighandle as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(SIGTERM, Some(sighandle as unsafe extern "C" fn(libc::c_int) -> ()));
    act = b"opening environment\0" as *const u8 as *const libc::c_char;
    rc = mdb_env_create(&mut env);
    if rc == MDB_SUCCESS {
        rc = mdb_env_open(
            env,
            *argv.offset(1 as libc::c_int as isize),
            flags,
            0o600 as libc::c_int as mdb_mode_t,
        );
    }
    if rc == MDB_SUCCESS {
        act = b"copying\0" as *const u8 as *const libc::c_char;
        if argc == 2 as libc::c_int {
            rc = mdb_env_copyfd2(env, MDB_STDOUT, cpflags);
        } else {
            rc = mdb_env_copy2(env, *argv.offset(2 as libc::c_int as isize), cpflags);
        }
    }
    if rc != 0 {
        fprintf(
            __stderrp,
            b"%s: %s failed, error %d (%s)\n\0" as *const u8 as *const libc::c_char,
            progname,
            act,
            rc,
            mdb_strerror(rc),
        );
    }
    mdb_env_close(env);
    return if rc != 0 { EXIT_FAILURE } else { EXIT_SUCCESS };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
