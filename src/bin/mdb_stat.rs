use ::libc;
extern "C" {
    pub type __sFILEX;
    pub type MDB_env;
    pub type MDB_txn;
    pub type MDB_cursor;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(_: *const libc::c_char, _: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn getopt(
        _: libc::c_int,
        _: *const *mut libc::c_char,
        _: *const libc::c_char,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn mdb_strerror(err: libc::c_int) -> *mut libc::c_char;
    fn mdb_env_create(env: *mut *mut MDB_env) -> libc::c_int;
    fn mdb_env_open(
        env: *mut MDB_env,
        path: *const libc::c_char,
        flags: libc::c_uint,
        mode: mdb_mode_t,
    ) -> libc::c_int;
    fn mdb_env_stat(env: *mut MDB_env, stat: *mut MDB_stat) -> libc::c_int;
    fn mdb_env_info(env: *mut MDB_env, stat: *mut MDB_envinfo) -> libc::c_int;
    fn mdb_env_close(env: *mut MDB_env);
    fn mdb_env_set_maxdbs(env: *mut MDB_env, dbs: MDB_dbi) -> libc::c_int;
    fn mdb_txn_begin(
        env: *mut MDB_env,
        parent: *mut MDB_txn,
        flags: libc::c_uint,
        txn: *mut *mut MDB_txn,
    ) -> libc::c_int;
    fn mdb_txn_abort(txn: *mut MDB_txn);
    fn mdb_dbi_open(
        txn: *mut MDB_txn,
        name: *const libc::c_char,
        flags: libc::c_uint,
        dbi: *mut MDB_dbi,
    ) -> libc::c_int;
    fn mdb_stat(txn: *mut MDB_txn, dbi: MDB_dbi, stat: *mut MDB_stat) -> libc::c_int;
    fn mdb_dbi_close(env: *mut MDB_env, dbi: MDB_dbi);
    fn mdb_cursor_open(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        cursor: *mut *mut MDB_cursor,
    ) -> libc::c_int;
    fn mdb_cursor_close(cursor: *mut MDB_cursor);
    fn mdb_cursor_get(
        cursor: *mut MDB_cursor,
        key: *mut MDB_val,
        data: *mut MDB_val,
        op: MDB_cursor_op,
    ) -> libc::c_int;
    fn mdb_reader_list(
        env: *mut MDB_env,
        func: Option::<MDB_msg_func>,
        ctx: *mut libc::c_void,
    ) -> libc::c_int;
    fn mdb_reader_check(env: *mut MDB_env, dead: *mut libc::c_int) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
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
pub type ssize_t = __darwin_ssize_t;
pub type mode_t = __darwin_mode_t;
pub type mdb_mode_t = mode_t;
pub type mdb_size_t = size_t;
pub type MDB_dbi = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_val {
    pub mv_size: size_t,
    pub mv_data: *mut libc::c_void,
}
pub type MDB_cursor_op = libc::c_uint;
pub const MDB_PREV_MULTIPLE: MDB_cursor_op = 18;
pub const MDB_SET_RANGE: MDB_cursor_op = 17;
pub const MDB_SET_KEY: MDB_cursor_op = 16;
pub const MDB_SET: MDB_cursor_op = 15;
pub const MDB_PREV_NODUP: MDB_cursor_op = 14;
pub const MDB_PREV_DUP: MDB_cursor_op = 13;
pub const MDB_PREV: MDB_cursor_op = 12;
pub const MDB_NEXT_NODUP: MDB_cursor_op = 11;
pub const MDB_NEXT_MULTIPLE: MDB_cursor_op = 10;
pub const MDB_NEXT_DUP: MDB_cursor_op = 9;
pub const MDB_NEXT: MDB_cursor_op = 8;
pub const MDB_LAST_DUP: MDB_cursor_op = 7;
pub const MDB_LAST: MDB_cursor_op = 6;
pub const MDB_GET_MULTIPLE: MDB_cursor_op = 5;
pub const MDB_GET_CURRENT: MDB_cursor_op = 4;
pub const MDB_GET_BOTH_RANGE: MDB_cursor_op = 3;
pub const MDB_GET_BOTH: MDB_cursor_op = 2;
pub const MDB_FIRST_DUP: MDB_cursor_op = 1;
pub const MDB_FIRST: MDB_cursor_op = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_stat {
    pub ms_psize: libc::c_uint,
    pub ms_depth: libc::c_uint,
    pub ms_branch_pages: mdb_size_t,
    pub ms_leaf_pages: mdb_size_t,
    pub ms_overflow_pages: mdb_size_t,
    pub ms_entries: mdb_size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_envinfo {
    pub me_mapaddr: *mut libc::c_void,
    pub me_mapsize: mdb_size_t,
    pub me_last_pgno: mdb_size_t,
    pub me_last_txnid: mdb_size_t,
    pub me_maxreaders: libc::c_uint,
    pub me_numreaders: libc::c_uint,
}
pub type MDB_msg_func = unsafe extern "C" fn(
    *const libc::c_char,
    *mut libc::c_void,
) -> libc::c_int;
pub const __DARWIN_NULL: libc::c_int = 0 as libc::c_int;
pub const EOF: libc::c_int = -(1 as libc::c_int);
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
pub const MDB_SUCCESS: libc::c_int = 0 as libc::c_int;
pub const MDB_NOTFOUND: libc::c_int = MDB_NOTFOUND;
pub const NULL: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn prstat(mut ms: *mut MDB_stat) {
    printf(b"  Tree depth: %u\n\0" as *const u8 as *const libc::c_char, (*ms).ms_depth);
    printf(
        b"  Branch pages: %zu\n\0" as *const u8 as *const libc::c_char,
        (*ms).ms_branch_pages,
    );
    printf(
        b"  Leaf pages: %zu\n\0" as *const u8 as *const libc::c_char,
        (*ms).ms_leaf_pages,
    );
    printf(
        b"  Overflow pages: %zu\n\0" as *const u8 as *const libc::c_char,
        (*ms).ms_overflow_pages,
    );
    printf(b"  Entries: %zu\n\0" as *const u8 as *const libc::c_char, (*ms).ms_entries);
}
unsafe extern "C" fn usage(mut prog: *mut libc::c_char) {
    fprintf(
        __stderrp,
        b"usage: %s [-V] [-n] [-e] [-r[r]] [-f[f[f]]] [-v] [-a|-s subdb] dbpath\n\0"
            as *const u8 as *const libc::c_char,
        prog,
    );
    exit(EXIT_FAILURE);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut env = 0 as *mut MDB_env;
    let mut txn = 0 as *mut MDB_txn;
    let mut dbi: MDB_dbi = 0;
    let mut mst = MDB_stat {
        ms_psize: 0,
        ms_depth: 0,
        ms_branch_pages: 0,
        ms_leaf_pages: 0,
        ms_overflow_pages: 0,
        ms_entries: 0,
    };
    let mut mei = MDB_envinfo {
        me_mapaddr: 0 as *mut libc::c_void,
        me_mapsize: 0,
        me_last_pgno: 0,
        me_last_txnid: 0,
        me_maxreaders: 0,
        me_numreaders: 0,
    };
    let mut prog = *argv.offset(0 as libc::c_int as isize);
    let mut envname = 0 as *mut libc::c_char;
    let mut subname = NULL as *mut libc::c_char;
    let mut alldbs = 0 as libc::c_int;
    let mut envinfo = 0 as libc::c_int;
    let mut envflags = 0 as libc::c_int;
    let mut freinfo = 0 as libc::c_int;
    let mut rdrinfo = 0 as libc::c_int;
    if argc < 2 as libc::c_int {
        usage(prog);
    }
    loop {
        i = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"Vaefnrs:v\0" as *const u8 as *const libc::c_char,
        );
        if !(i != EOF) {
            break;
        }
        match i {
            86 => {
                printf(
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    MDB_VERSION_STRING.as_ptr(),
                );
                exit(0 as libc::c_int);
            }
            97 => {
                if !subname.is_null() {
                    usage(prog);
                }
                alldbs += 1;
                alldbs;
            }
            101 => {
                envinfo += 1;
                envinfo;
            }
            102 => {
                freinfo += 1;
                freinfo;
            }
            110 => {
                envflags |= MDB_NOSUBDIR;
            }
            118 => {
                envflags |= MDB_PREVSNAPSHOT;
            }
            114 => {
                rdrinfo += 1;
                rdrinfo;
            }
            115 => {
                if alldbs != 0 {
                    usage(prog);
                }
                subname = optarg;
            }
            _ => {
                usage(prog);
            }
        }
    }
    if optind != argc - 1 as libc::c_int {
        usage(prog);
    }
    envname = *argv.offset(optind as isize);
    rc = mdb_env_create(&mut env);
    if rc != 0 {
        fprintf(
            __stderrp,
            b"mdb_env_create failed, error %d %s\n\0" as *const u8
                as *const libc::c_char,
            rc,
            mdb_strerror(rc),
        );
        return EXIT_FAILURE;
    }
    if alldbs != 0 || !subname.is_null() {
        mdb_env_set_maxdbs(env, 4 as libc::c_int as MDB_dbi);
    }
    rc = mdb_env_open(
        env,
        envname,
        (envflags | MDB_RDONLY) as libc::c_uint,
        0o664 as libc::c_int as mdb_mode_t,
    );
    if rc != 0 {
        fprintf(
            __stderrp,
            b"mdb_env_open failed, error %d %s\n\0" as *const u8 as *const libc::c_char,
            rc,
            mdb_strerror(rc),
        );
    } else {
        if envinfo != 0 {
            mdb_env_stat(env, &mut mst);
            mdb_env_info(env, &mut mei);
            printf(b"Environment Info\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"  Map address: %p\n\0" as *const u8 as *const libc::c_char,
                mei.me_mapaddr,
            );
            printf(
                b"  Map size: %zu\n\0" as *const u8 as *const libc::c_char,
                mei.me_mapsize,
            );
            printf(
                b"  Page size: %u\n\0" as *const u8 as *const libc::c_char,
                mst.ms_psize,
            );
            printf(
                b"  Max pages: %zu\n\0" as *const u8 as *const libc::c_char,
                mei.me_mapsize / mst.ms_psize as mdb_size_t,
            );
            printf(
                b"  Number of pages used: %zu\n\0" as *const u8 as *const libc::c_char,
                (mei.me_last_pgno).wrapping_add(1 as libc::c_int as mdb_size_t),
            );
            printf(
                b"  Last transaction ID: %zu\n\0" as *const u8 as *const libc::c_char,
                mei.me_last_txnid,
            );
            printf(
                b"  Max readers: %u\n\0" as *const u8 as *const libc::c_char,
                mei.me_maxreaders,
            );
            printf(
                b"  Number of readers used: %u\n\0" as *const u8 as *const libc::c_char,
                mei.me_numreaders,
            );
        }
        if rdrinfo != 0 {
            printf(b"Reader Table Status\n\0" as *const u8 as *const libc::c_char);
            rc = mdb_reader_list(
                env,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_char,
                            *mut FILE,
                        ) -> libc::c_int,
                    >,
                    Option::<MDB_msg_func>,
                >(
                    Some(
                        fputs
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                *mut FILE,
                            ) -> libc::c_int,
                    ),
                ),
                __stdoutp as *mut libc::c_void,
            );
            if rdrinfo > 1 as libc::c_int {
                let mut dead: libc::c_int = 0;
                mdb_reader_check(env, &mut dead);
                printf(
                    b"  %d stale readers cleared.\n\0" as *const u8
                        as *const libc::c_char,
                    dead,
                );
                rc = mdb_reader_list(
                    env,
                    ::core::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *const libc::c_char,
                                *mut FILE,
                            ) -> libc::c_int,
                        >,
                        Option::<MDB_msg_func>,
                    >(
                        Some(
                            fputs
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    *mut FILE,
                                ) -> libc::c_int,
                        ),
                    ),
                    __stdoutp as *mut libc::c_void,
                );
            }
            if !(!subname.is_null() || alldbs != 0 || freinfo != 0) {
                current_block = 5726057570200118372;
            } else {
                current_block = 7420279277351916581;
            }
        } else {
            current_block = 7420279277351916581;
        }
        match current_block {
            5726057570200118372 => {}
            _ => {
                rc = mdb_txn_begin(
                    env,
                    NULL as *mut MDB_txn,
                    MDB_RDONLY as libc::c_uint,
                    &mut txn,
                );
                if rc != 0 {
                    fprintf(
                        __stderrp,
                        b"mdb_txn_begin failed, error %d %s\n\0" as *const u8
                            as *const libc::c_char,
                        rc,
                        mdb_strerror(rc),
                    );
                } else {
                    if freinfo != 0 {
                        let mut cursor = 0 as *mut MDB_cursor;
                        let mut key = MDB_val {
                            mv_size: 0,
                            mv_data: 0 as *mut libc::c_void,
                        };
                        let mut data = MDB_val {
                            mv_size: 0,
                            mv_data: 0 as *mut libc::c_void,
                        };
                        let mut pages = 0 as libc::c_int as mdb_size_t;
                        let mut iptr = 0 as *mut mdb_size_t;
                        printf(
                            b"Freelist Status\n\0" as *const u8 as *const libc::c_char,
                        );
                        dbi = 0 as libc::c_int as MDB_dbi;
                        rc = mdb_cursor_open(txn, dbi, &mut cursor);
                        if rc != 0 {
                            fprintf(
                                __stderrp,
                                b"mdb_cursor_open failed, error %d %s\n\0" as *const u8
                                    as *const libc::c_char,
                                rc,
                                mdb_strerror(rc),
                            );
                            current_block = 9589668473123540832;
                        } else {
                            rc = mdb_stat(txn, dbi, &mut mst);
                            if rc != 0 {
                                fprintf(
                                    __stderrp,
                                    b"mdb_stat failed, error %d %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    rc,
                                    mdb_strerror(rc),
                                );
                                current_block = 9589668473123540832;
                            } else {
                                prstat(&mut mst);
                                loop {
                                    rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_NEXT);
                                    if !(rc == 0 as libc::c_int) {
                                        break;
                                    }
                                    iptr = data.mv_data as *mut mdb_size_t;
                                    pages = pages.wrapping_add(*iptr);
                                    if freinfo > 1 as libc::c_int {
                                        let mut bad = b"\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char;
                                        let mut pg: mdb_size_t = 0;
                                        let mut prev: mdb_size_t = 0;
                                        let mut i_0: ssize_t = 0;
                                        let mut j: ssize_t = 0;
                                        let mut span = 0 as libc::c_int as ssize_t;
                                        let fresh0 = iptr;
                                        iptr = iptr.offset(1);
                                        j = *fresh0 as ssize_t;
                                        i_0 = j;
                                        prev = 1 as libc::c_int as mdb_size_t;
                                        loop {
                                            i_0 -= 1;
                                            if !(i_0 >= 0 as libc::c_int as ssize_t) {
                                                break;
                                            }
                                            pg = *iptr.offset(i_0 as isize);
                                            if pg <= prev {
                                                bad = b" [bad sequence]\0" as *const u8
                                                    as *const libc::c_char as *mut libc::c_char;
                                            }
                                            prev = pg;
                                            pg = pg.wrapping_add(span as mdb_size_t);
                                            while i_0 >= span
                                                && *iptr.offset((i_0 - span) as isize) == pg
                                            {
                                                span += 1;
                                                span;
                                                pg = pg.wrapping_add(1);
                                                pg;
                                            }
                                        }
                                        printf(
                                            b"    Transaction %zu, %zd pages, maxspan %zd%s\n\0"
                                                as *const u8 as *const libc::c_char,
                                            *(key.mv_data as *mut mdb_size_t),
                                            j,
                                            span,
                                            bad,
                                        );
                                        if freinfo > 2 as libc::c_int {
                                            j -= 1;
                                            j;
                                            while j >= 0 as libc::c_int as ssize_t {
                                                pg = *iptr.offset(j as isize);
                                                span = 1 as libc::c_int as ssize_t;
                                                loop {
                                                    j -= 1;
                                                    if !(j >= 0 as libc::c_int as ssize_t
                                                        && *iptr.offset(j as isize)
                                                            == pg.wrapping_add(span as mdb_size_t))
                                                    {
                                                        break;
                                                    }
                                                    span += 1;
                                                    span;
                                                }
                                                printf(
                                                    if span > 1 as libc::c_int as ssize_t {
                                                        b"     %9zu[%zd]\n\0" as *const u8 as *const libc::c_char
                                                    } else {
                                                        b"     %9zu\n\0" as *const u8 as *const libc::c_char
                                                    },
                                                    pg,
                                                    span,
                                                );
                                            }
                                        }
                                    }
                                }
                                mdb_cursor_close(cursor);
                                printf(
                                    b"  Free pages: %zu\n\0" as *const u8
                                        as *const libc::c_char,
                                    pages,
                                );
                                current_block = 14541395414537699361;
                            }
                        }
                    } else {
                        current_block = 14541395414537699361;
                    }
                    match current_block {
                        14541395414537699361 => {
                            rc = mdb_dbi_open(
                                txn,
                                subname,
                                0 as libc::c_int as libc::c_uint,
                                &mut dbi,
                            );
                            if rc != 0 {
                                fprintf(
                                    __stderrp,
                                    b"mdb_open failed, error %d %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    rc,
                                    mdb_strerror(rc),
                                );
                            } else {
                                rc = mdb_stat(txn, dbi, &mut mst);
                                if rc != 0 {
                                    fprintf(
                                        __stderrp,
                                        b"mdb_stat failed, error %d %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        rc,
                                        mdb_strerror(rc),
                                    );
                                } else {
                                    printf(
                                        b"Status of %s\n\0" as *const u8 as *const libc::c_char,
                                        if !subname.is_null() {
                                            subname as *const libc::c_char
                                        } else {
                                            b"Main DB\0" as *const u8 as *const libc::c_char
                                        },
                                    );
                                    prstat(&mut mst);
                                    if alldbs != 0 {
                                        let mut cursor_0 = 0 as *mut MDB_cursor;
                                        let mut key_0 = MDB_val {
                                            mv_size: 0,
                                            mv_data: 0 as *mut libc::c_void,
                                        };
                                        rc = mdb_cursor_open(txn, dbi, &mut cursor_0);
                                        if rc != 0 {
                                            fprintf(
                                                __stderrp,
                                                b"mdb_cursor_open failed, error %d %s\n\0" as *const u8
                                                    as *const libc::c_char,
                                                rc,
                                                mdb_strerror(rc),
                                            );
                                            current_block = 9589668473123540832;
                                        } else {
                                            loop {
                                                rc = mdb_cursor_get(
                                                    cursor_0,
                                                    &mut key_0,
                                                    NULL as *mut MDB_val,
                                                    MDB_NEXT_NODUP,
                                                );
                                                if !(rc == 0 as libc::c_int) {
                                                    current_block = 17702298541784679949;
                                                    break;
                                                }
                                                let mut str = 0 as *mut libc::c_char;
                                                let mut db2: MDB_dbi = 0;
                                                if !(memchr(key_0.mv_data, '\0' as i32, key_0.mv_size))
                                                    .is_null()
                                                {
                                                    continue;
                                                }
                                                str = malloc(
                                                    (key_0.mv_size).wrapping_add(1 as libc::c_int as size_t),
                                                ) as *mut libc::c_char;
                                                memcpy(
                                                    str as *mut libc::c_void,
                                                    key_0.mv_data,
                                                    key_0.mv_size,
                                                );
                                                *str
                                                    .offset(
                                                        key_0.mv_size as isize,
                                                    ) = '\0' as i32 as libc::c_char;
                                                rc = mdb_dbi_open(
                                                    txn,
                                                    str,
                                                    0 as libc::c_int as libc::c_uint,
                                                    &mut db2,
                                                );
                                                if rc == MDB_SUCCESS {
                                                    printf(
                                                        b"Status of %s\n\0" as *const u8 as *const libc::c_char,
                                                        str,
                                                    );
                                                }
                                                free(str as *mut libc::c_void);
                                                if rc != 0 {
                                                    continue;
                                                }
                                                rc = mdb_stat(txn, db2, &mut mst);
                                                if rc != 0 {
                                                    fprintf(
                                                        __stderrp,
                                                        b"mdb_stat failed, error %d %s\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        rc,
                                                        mdb_strerror(rc),
                                                    );
                                                    current_block = 9589668473123540832;
                                                    break;
                                                } else {
                                                    prstat(&mut mst);
                                                    mdb_dbi_close(env, db2);
                                                }
                                            }
                                            match current_block {
                                                9589668473123540832 => {}
                                                _ => {
                                                    mdb_cursor_close(cursor_0);
                                                    current_block = 16778110326724371720;
                                                }
                                            }
                                        }
                                    } else {
                                        current_block = 16778110326724371720;
                                    }
                                    match current_block {
                                        9589668473123540832 => {}
                                        _ => {
                                            if rc == MDB_NOTFOUND {
                                                rc = MDB_SUCCESS;
                                            }
                                            mdb_dbi_close(env, dbi);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    mdb_txn_abort(txn);
                }
            }
        }
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
