use ::libc;
extern "C" {
    pub type __sFILEX;
    pub type MDB_env;
    pub type MDB_txn;
    pub type MDB_cursor;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn srand(_: libc::c_uint);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn time(_: *mut time_t) -> time_t;
    fn mdb_strerror(err: libc::c_int) -> *mut libc::c_char;
    fn mdb_env_create(env: *mut *mut MDB_env) -> libc::c_int;
    fn mdb_env_open(
        env: *mut MDB_env,
        path: *const libc::c_char,
        flags: libc::c_uint,
        mode: mdb_mode_t,
    ) -> libc::c_int;
    fn mdb_env_stat(env: *mut MDB_env, stat: *mut MDB_stat) -> libc::c_int;
    fn mdb_env_close(env: *mut MDB_env);
    fn mdb_env_set_mapsize(env: *mut MDB_env, size: mdb_size_t) -> libc::c_int;
    fn mdb_env_set_maxreaders(env: *mut MDB_env, readers: libc::c_uint) -> libc::c_int;
    fn mdb_env_set_maxdbs(env: *mut MDB_env, dbs: MDB_dbi) -> libc::c_int;
    fn mdb_txn_begin(
        env: *mut MDB_env,
        parent: *mut MDB_txn,
        flags: libc::c_uint,
        txn: *mut *mut MDB_txn,
    ) -> libc::c_int;
    fn mdb_txn_commit(txn: *mut MDB_txn) -> libc::c_int;
    fn mdb_txn_abort(txn: *mut MDB_txn);
    fn mdb_dbi_open(
        txn: *mut MDB_txn,
        name: *const libc::c_char,
        flags: libc::c_uint,
        dbi: *mut MDB_dbi,
    ) -> libc::c_int;
    fn mdb_dbi_close(env: *mut MDB_env, dbi: MDB_dbi);
    fn mdb_put(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        key: *mut MDB_val,
        data: *mut MDB_val,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn mdb_del(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        key: *mut MDB_val,
        data: *mut MDB_val,
    ) -> libc::c_int;
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
}
pub type __uint16_t = libc::c_ushort;
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_time_t = libc::c_long;
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
pub type mode_t = __darwin_mode_t;
pub type time_t = __darwin_time_t;
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
pub const __DARWIN_NULL: libc::c_int = 0 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i = 0 as libc::c_int;
    let mut j = 0 as libc::c_int;
    let mut rc: libc::c_int = 0;
    let mut env = 0 as *mut MDB_env;
    let mut dbi: MDB_dbi = 0;
    let mut key = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut txn = 0 as *mut MDB_txn;
    let mut mst = MDB_stat {
        ms_psize: 0,
        ms_depth: 0,
        ms_branch_pages: 0,
        ms_leaf_pages: 0,
        ms_overflow_pages: 0,
        ms_entries: 0,
    };
    let mut cursor = 0 as *mut MDB_cursor;
    let mut count: libc::c_int = 0;
    let mut values = 0 as *mut libc::c_int;
    let mut sval: [libc::c_char; 32] = *::core::mem::transmute::<
        &[u8; 32],
        &mut [libc::c_char; 32],
    >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    srand(time(NULL as *mut time_t) as libc::c_uint);
    count = rand() % 384 as libc::c_int + 64 as libc::c_int;
    values = malloc(
        (count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        *values.offset(i as isize) = rand() % 1024 as libc::c_int;
        i += 1;
        i;
    }
    rc = mdb_env_create(&mut env);
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            b"mdb_env_create(&env)\0" as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_set_maxreaders(env, 1 as libc::c_int as libc::c_uint);
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
            b"mdb_env_set_maxreaders(env, 1)\0" as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_set_mapsize(env, 10485760 as libc::c_int as mdb_size_t);
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            b"mdb_env_set_mapsize(env, 10485760)\0" as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_set_maxdbs(env, 4 as libc::c_int as MDB_dbi);
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            b"mdb_env_set_maxdbs(env, 4)\0" as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_open(
        env,
        b"./testdb\0" as *const u8 as *const libc::c_char,
        (0x1 as libc::c_int | 0x10000 as libc::c_int) as libc::c_uint,
        0o664 as libc::c_int as mdb_mode_t,
    );
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            b"mdb_env_open(env, \"./testdb\", MDB_FIXEDMAP|MDB_NOSYNC, 0664)\0"
                as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_txn_begin(
        env,
        0 as *mut MDB_txn,
        0 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_dbi_open(
        txn,
        b"id1\0" as *const u8 as *const libc::c_char,
        0x40000 as libc::c_int as libc::c_uint,
        &mut dbi,
    );
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            b"mdb_dbi_open(txn, \"id1\", MDB_CREATE, &dbi)\0" as *const u8
                as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    key.mv_size = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong;
    key.mv_data = sval.as_mut_ptr() as *mut libc::c_void;
    printf(b"Adding %d values\n\0" as *const u8 as *const libc::c_char, count);
    i = 0 as libc::c_int;
    while i < count {
        sprintf(
            sval.as_mut_ptr(),
            b"%03x %d foo bar\0" as *const u8 as *const libc::c_char,
            *values.offset(i as isize),
            *values.offset(i as isize),
        );
        data.mv_size = ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong;
        data.mv_data = sval.as_mut_ptr() as *mut libc::c_void;
        rc = mdb_put(txn, dbi, &mut key, &mut data, 0x10 as libc::c_int as libc::c_uint);
        if rc == -(30799 as libc::c_int)
            || {
                if rc == 0 {} else {
                    fprintf(
                        __stderrp,
                        b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                        b"mtest2.c\0" as *const u8 as *const libc::c_char,
                        66 as libc::c_int,
                        b"mdb_put(txn, dbi, &key, &data, MDB_NOOVERWRITE)\0" as *const u8
                            as *const libc::c_char,
                        mdb_strerror(rc),
                    );
                    abort();
                };
                0 as libc::c_int != 0
            }
        {
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    if j != 0 {
        printf(b"%d duplicates skipped\n\0" as *const u8 as *const libc::c_char, j);
    }
    rc = mdb_txn_commit(txn);
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
            b"mdb_txn_commit(txn)\0" as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_stat(env, &mut mst);
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            b"mdb_env_stat(env, &mst)\0" as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_txn_begin(
        env,
        0 as *mut MDB_txn,
        0x20000 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int,
            b"mdb_txn_begin(env, NULL, MDB_RDONLY, &txn)\0" as *const u8
                as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            b"mdb_cursor_open(txn, dbi, &cursor)\0" as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    loop {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_NEXT);
        if !(rc == 0 as libc::c_int) {
            break;
        }
        printf(
            b"key: %p %.*s, data: %p %.*s\n\0" as *const u8 as *const libc::c_char,
            key.mv_data,
            key.mv_size as libc::c_int,
            key.mv_data as *mut libc::c_char,
            data.mv_data,
            data.mv_size as libc::c_int,
            data.mv_data as *mut libc::c_char,
        );
    }
    if rc == -(30798 as libc::c_int) {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int,
            b"mdb_cursor_get\0" as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    j = 0 as libc::c_int;
    key.mv_data = sval.as_mut_ptr() as *mut libc::c_void;
    i = count - 1 as libc::c_int;
    while i > -(1 as libc::c_int) {
        j += 1;
        j;
        txn = NULL as *mut MDB_txn;
        rc = mdb_txn_begin(
            env,
            0 as *mut MDB_txn,
            0 as libc::c_int as libc::c_uint,
            &mut txn,
        );
        if rc == 0 as libc::c_int {} else {
            fprintf(
                __stderrp,
                b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                b"mtest2.c\0" as *const u8 as *const libc::c_char,
                89 as libc::c_int,
                b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8
                    as *const libc::c_char,
                mdb_strerror(rc),
            );
            abort();
        };
        sprintf(
            sval.as_mut_ptr(),
            b"%03x \0" as *const u8 as *const libc::c_char,
            *values.offset(i as isize),
        );
        rc = mdb_del(txn, dbi, &mut key, 0 as *mut MDB_val);
        if rc == -(30798 as libc::c_int)
            || {
                if rc == 0 {} else {
                    fprintf(
                        __stderrp,
                        b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                        b"mtest2.c\0" as *const u8 as *const libc::c_char,
                        91 as libc::c_int,
                        b"mdb_del(txn, dbi, &key, NULL)\0" as *const u8
                            as *const libc::c_char,
                        mdb_strerror(rc),
                    );
                    abort();
                };
                0 as libc::c_int != 0
            }
        {
            j -= 1;
            j;
            mdb_txn_abort(txn);
        } else {
            rc = mdb_txn_commit(txn);
            if rc == 0 as libc::c_int {} else {
                fprintf(
                    __stderrp,
                    b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                    b"mtest2.c\0" as *const u8 as *const libc::c_char,
                    95 as libc::c_int,
                    b"mdb_txn_commit(txn)\0" as *const u8 as *const libc::c_char,
                    mdb_strerror(rc),
                );
                abort();
            };
        }
        i -= rand() % 5 as libc::c_int;
    }
    free(values as *mut libc::c_void);
    printf(b"Deleted %d values\n\0" as *const u8 as *const libc::c_char, j);
    rc = mdb_env_stat(env, &mut mst);
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int,
            b"mdb_env_stat(env, &mst)\0" as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_txn_begin(
        env,
        0 as *mut MDB_txn,
        0x20000 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            b"mdb_txn_begin(env, NULL, MDB_RDONLY, &txn)\0" as *const u8
                as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if rc == 0 as libc::c_int {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            b"mdb_cursor_open(txn, dbi, &cursor)\0" as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    printf(b"Cursor next\n\0" as *const u8 as *const libc::c_char);
    loop {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_NEXT);
        if !(rc == 0 as libc::c_int) {
            break;
        }
        printf(
            b"key: %.*s, data: %.*s\n\0" as *const u8 as *const libc::c_char,
            key.mv_size as libc::c_int,
            key.mv_data as *mut libc::c_char,
            data.mv_size as libc::c_int,
            data.mv_data as *mut libc::c_char,
        );
    }
    if rc == -(30798 as libc::c_int) {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            b"mdb_cursor_get\0" as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    printf(b"Cursor prev\n\0" as *const u8 as *const libc::c_char);
    loop {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_PREV);
        if !(rc == 0 as libc::c_int) {
            break;
        }
        printf(
            b"key: %.*s, data: %.*s\n\0" as *const u8 as *const libc::c_char,
            key.mv_size as libc::c_int,
            key.mv_data as *mut libc::c_char,
            data.mv_size as libc::c_int,
            data.mv_data as *mut libc::c_char,
        );
    }
    if rc == -(30798 as libc::c_int) {} else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            b"mtest2.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            b"mdb_cursor_get\0" as *const u8 as *const libc::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    mdb_dbi_close(env, dbi);
    mdb_env_close(env);
    return 0 as libc::c_int;
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
