use ::libc;

use crate::errno;
extern "C" {
    pub type __sFILEX;
    pub type MDB_env;
    pub type MDB_txn;
    pub type MDB_cursor;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn freopen(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut FILE,
    ) -> *mut FILE;
    fn putchar(_: libc::c_int) -> libc::c_int;
    fn __error() -> *mut libc::c_int;
    fn signal(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
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
        mode_0: mdb_mode_t,
    ) -> libc::c_int;
    fn mdb_env_info(env: *mut MDB_env, stat: *mut MDB_envinfo) -> libc::c_int;
    fn mdb_env_close(env: *mut MDB_env);
    fn mdb_env_set_maxdbs(env: *mut MDB_env, dbs: MDB_dbi) -> libc::c_int;
    fn mdb_txn_begin(
        env: *mut MDB_env,
        parent: *mut MDB_txn,
        flags: libc::c_uint,
        txn: *mut *mut MDB_txn,
    ) -> libc::c_int;
    fn mdb_txn_env(txn: *mut MDB_txn) -> *mut MDB_env;
    fn mdb_txn_abort(txn: *mut MDB_txn);
    fn mdb_dbi_open(
        txn: *mut MDB_txn,
        name: *const libc::c_char,
        flags: libc::c_uint,
        dbi: *mut MDB_dbi,
    ) -> libc::c_int;
    fn mdb_stat(txn: *mut MDB_txn, dbi: MDB_dbi, stat: *mut MDB_stat) -> libc::c_int;
    fn mdb_dbi_flags(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        flags: *mut libc::c_uint,
    ) -> libc::c_int;
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
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
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
pub type sig_atomic_t = libc::c_int;
pub type mode_t = __darwin_mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: libc::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [libc::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [libc::c_char; 8],
    pub __encoding: [libc::c_char; 32],
    pub __sgetrune: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            __darwin_size_t,
            *mut *const libc::c_char,
        ) -> __darwin_rune_t,
    >,
    pub __sputrune: Option::<
        unsafe extern "C" fn(
            __darwin_rune_t,
            *mut libc::c_char,
            __darwin_size_t,
            *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut libc::c_void,
    pub __variable_len: libc::c_int,
    pub __ncharclasses: libc::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flagbit {
    pub bit: libc::c_int,
    pub name: *mut libc::c_char,
}
pub const __DARWIN_NULL: libc::c_int = 0 as libc::c_int;
pub const EOF: libc::c_int = -(1 as libc::c_int);
pub const EINTR: libc::c_int = 4 as libc::c_int;
pub const SIGHUP: libc::c_int = 1 as libc::c_int;
pub const SIGINT: libc::c_int = 2 as libc::c_int;
pub const SIGPIPE: libc::c_int = 13 as libc::c_int;
pub const SIGTERM: libc::c_int = 15 as libc::c_int;
pub const EXIT_FAILURE: libc::c_int = 1 as libc::c_int;
pub const EXIT_SUCCESS: libc::c_int = 0 as libc::c_int;
pub const _CTYPE_R: libc::c_long = 0x40000 as libc::c_long;
#[inline]
unsafe extern "C" fn isascii(mut _c: libc::c_int) -> libc::c_int {
    return (_c & !(0x7f as libc::c_int) == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __istype(
    mut _c: __darwin_ct_rune_t,
    mut _f: libc::c_ulong,
) -> libc::c_int {
    return if isascii(_c) != 0 {
        (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0)
            as libc::c_int
    } else {
        (__maskrune(_c, _f) != 0) as libc::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isprint(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, _CTYPE_R as libc::c_ulong);
}
pub const MDB_VERSION_STRING: [libc::c_char; 33] = unsafe {
    *::core::mem::transmute::<
        &[u8; 33],
        &[libc::c_char; 33],
    >(b"LMDB 0.9.70: (December 19, 2015)\0")
};
pub const MDB_NOSUBDIR: libc::c_int = 0x4000 as libc::c_int;
pub const MDB_RDONLY: libc::c_int = 0x20000 as libc::c_int;
pub const MDB_PREVSNAPSHOT: libc::c_int = 0x2000000 as libc::c_int;
pub const MDB_REVERSEKEY: libc::c_int = 0x2 as libc::c_int;
pub const MDB_DUPSORT: libc::c_int = 0x4 as libc::c_int;
pub const MDB_INTEGERKEY: libc::c_int = 0x8 as libc::c_int;
pub const MDB_DUPFIXED: libc::c_int = 0x10 as libc::c_int;
pub const MDB_INTEGERDUP: libc::c_int = 0x20 as libc::c_int;
pub const MDB_REVERSEDUP: libc::c_int = 0x40 as libc::c_int;
pub const MDB_SUCCESS: libc::c_int = 0 as libc::c_int;
pub const MDB_NOTFOUND: libc::c_int = -(30798 as libc::c_int);
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const PRINT: libc::c_int = 1 as libc::c_int;
static mut mode: libc::c_int = 0;
#[no_mangle]
pub static mut dbflags: [flagbit; 7] = [
    {
        let mut init = flagbit {
            bit: MDB_REVERSEKEY,
            name: b"reversekey\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: MDB_DUPSORT,
            name: b"dupsort\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: MDB_INTEGERKEY,
            name: b"integerkey\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: MDB_DUPFIXED,
            name: b"dupfixed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: MDB_INTEGERDUP,
            name: b"integerdup\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: MDB_REVERSEDUP,
            name: b"reversedup\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: 0 as libc::c_int,
            name: NULL as *mut libc::c_char,
        };
        init
    },
];
static mut gotsig: sig_atomic_t = 0;
unsafe extern "C" fn dumpsig(mut sig: libc::c_int) {
    ::core::ptr::write_volatile(&mut gotsig as *mut sig_atomic_t, 1 as libc::c_int);
}
static mut hexc: [libc::c_char; 17] = unsafe {
    *::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"0123456789abcdef\0")
};
unsafe extern "C" fn hex(mut c: libc::c_uchar) {
    putchar(hexc[(c as libc::c_int >> 4 as libc::c_int) as usize] as libc::c_int);
    putchar(hexc[(c as libc::c_int & 0xf as libc::c_int) as usize] as libc::c_int);
}
unsafe extern "C" fn text(mut v: *mut MDB_val) {
    let mut c = 0 as *mut libc::c_uchar;
    let mut end = 0 as *mut libc::c_uchar;
    putchar(' ' as i32);
    c = (*v).mv_data as *mut libc::c_uchar;
    end = c.offset((*v).mv_size as isize);
    while c < end {
        if isprint(*c as libc::c_int) != 0 {
            if *c as libc::c_int == '\\' as i32 {
                putchar('\\' as i32);
            }
            putchar(*c as libc::c_int);
        } else {
            putchar('\\' as i32);
            hex(*c);
        }
        c = c.offset(1);
        c;
    }
    putchar('\n' as i32);
}
unsafe extern "C" fn byte(mut v: *mut MDB_val) {
    let mut c = 0 as *mut libc::c_uchar;
    let mut end = 0 as *mut libc::c_uchar;
    putchar(' ' as i32);
    c = (*v).mv_data as *mut libc::c_uchar;
    end = c.offset((*v).mv_size as isize);
    while c < end {
        let fresh0 = c;
        c = c.offset(1);
        hex(*fresh0);
    }
    putchar('\n' as i32);
}
unsafe extern "C" fn dumpit(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut mc = 0 as *mut MDB_cursor;
    let mut ms = MDB_stat {
        ms_psize: 0,
        ms_depth: 0,
        ms_branch_pages: 0,
        ms_leaf_pages: 0,
        ms_overflow_pages: 0,
        ms_entries: 0,
    };
    let mut key = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut info = MDB_envinfo {
        me_mapaddr: 0 as *mut libc::c_void,
        me_mapsize: 0,
        me_last_pgno: 0,
        me_last_txnid: 0,
        me_maxreaders: 0,
        me_numreaders: 0,
    };
    let mut flags: libc::c_uint = 0;
    let mut rc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    rc = mdb_dbi_flags(txn, dbi, &mut flags);
    if rc != 0 {
        return rc;
    }
    rc = mdb_stat(txn, dbi, &mut ms);
    if rc != 0 {
        return rc;
    }
    rc = mdb_env_info(mdb_txn_env(txn), &mut info);
    if rc != 0 {
        return rc;
    }
    printf(b"VERSION=3\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"format=%s\n\0" as *const u8 as *const libc::c_char,
        if mode & PRINT != 0 {
            b"print\0" as *const u8 as *const libc::c_char
        } else {
            b"bytevalue\0" as *const u8 as *const libc::c_char
        },
    );
    if !name.is_null() {
        printf(b"database=%s\n\0" as *const u8 as *const libc::c_char, name);
    }
    printf(b"type=btree\n\0" as *const u8 as *const libc::c_char);
    printf(b"mapsize=%zu\n\0" as *const u8 as *const libc::c_char, info.me_mapsize);
    if !(info.me_mapaddr).is_null() {
        printf(b"mapaddr=%p\n\0" as *const u8 as *const libc::c_char, info.me_mapaddr);
    }
    printf(b"maxreaders=%u\n\0" as *const u8 as *const libc::c_char, info.me_maxreaders);
    if flags & MDB_DUPSORT as libc::c_uint != 0 {
        printf(b"duplicates=1\n\0" as *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while dbflags[i as usize].bit != 0 {
        if flags & dbflags[i as usize].bit as libc::c_uint != 0 {
            printf(
                b"%s=1\n\0" as *const u8 as *const libc::c_char,
                dbflags[i as usize].name,
            );
        }
        i += 1;
        i;
    }
    printf(b"db_pagesize=%d\n\0" as *const u8 as *const libc::c_char, ms.ms_psize);
    printf(b"HEADER=END\n\0" as *const u8 as *const libc::c_char);
    rc = mdb_cursor_open(txn, dbi, &mut mc);
    if rc != 0 {
        return rc;
    }
    loop {
        rc = (mdb_cursor_get(mc, &mut key, &mut data, MDB_NEXT) == MDB_SUCCESS)
            as libc::c_int;
        if !(rc != 0) {
            break;
        }
        if gotsig != 0 {
            rc = EINTR;
            break;
        } else if mode & PRINT != 0 {
            text(&mut key);
            text(&mut data);
        } else {
            byte(&mut key);
            byte(&mut data);
        }
    }
    printf(b"DATA=END\n\0" as *const u8 as *const libc::c_char);
    if rc == MDB_NOTFOUND {
        rc = MDB_SUCCESS;
    }
    return rc;
}
unsafe extern "C" fn usage(mut prog: *mut libc::c_char) {
    fprintf(
        __stderrp,
        b"usage: %s [-V] [-f output] [-l] [-n] [-p] [-v] [-a|-s subdb] dbpath\n\0"
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
    let mut prog = *argv.offset(0 as libc::c_int as isize);
    let mut envname = 0 as *mut libc::c_char;
    let mut subname = NULL as *mut libc::c_char;
    let mut alldbs = 0 as libc::c_int;
    let mut envflags = 0 as libc::c_int;
    let mut list = 0 as libc::c_int;
    if argc < 2 as libc::c_int {
        usage(prog);
    }
    loop {
        i = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"af:lnps:vV\0" as *const u8 as *const libc::c_char,
        );
        if !(i != EOF) {
            break;
        }
        let mut current_block_19: u64;
        match i {
            86 => {
                printf(
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    MDB_VERSION_STRING.as_ptr(),
                );
                exit(0 as libc::c_int);
            }
            108 => {
                list = 1 as libc::c_int;
                current_block_19 = 15433940732114890699;
            }
            97 => {
                current_block_19 = 15433940732114890699;
            }
            102 => {
                if (freopen(
                    optarg,
                    b"w\0" as *const u8 as *const libc::c_char,
                    __stdoutp,
                ))
                    .is_null()
                {
                    fprintf(
                        __stderrp,
                        b"%s: %s: reopen: %s\n\0" as *const u8 as *const libc::c_char,
                        prog,
                        optarg,
                        strerror(errno()),
                    );
                    exit(EXIT_FAILURE);
                }
                current_block_19 = 15768484401365413375;
            }
            110 => {
                envflags |= MDB_NOSUBDIR;
                current_block_19 = 15768484401365413375;
            }
            118 => {
                envflags |= MDB_PREVSNAPSHOT;
                current_block_19 = 15768484401365413375;
            }
            112 => {
                mode |= PRINT;
                current_block_19 = 15768484401365413375;
            }
            115 => {
                if alldbs != 0 {
                    usage(prog);
                }
                subname = optarg;
                current_block_19 = 15768484401365413375;
            }
            _ => {
                usage(prog);
                current_block_19 = 15768484401365413375;
            }
        }
        match current_block_19 {
            15433940732114890699 => {
                if !subname.is_null() {
                    usage(prog);
                }
                alldbs += 1;
                alldbs;
            }
            _ => {}
        }
    }
    if optind != argc - 1 as libc::c_int {
        usage(prog);
    }
    signal(SIGPIPE, Some(dumpsig as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(SIGHUP, Some(dumpsig as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(SIGINT, Some(dumpsig as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(SIGTERM, Some(dumpsig as unsafe extern "C" fn(libc::c_int) -> ()));
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
        mdb_env_set_maxdbs(env, 2 as libc::c_int as MDB_dbi);
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
            rc = mdb_dbi_open(txn, subname, 0 as libc::c_int as libc::c_uint, &mut dbi);
            if rc != 0 {
                fprintf(
                    __stderrp,
                    b"mdb_open failed, error %d %s\n\0" as *const u8
                        as *const libc::c_char,
                    rc,
                    mdb_strerror(rc),
                );
            } else {
                if alldbs != 0 {
                    let mut cursor = 0 as *mut MDB_cursor;
                    let mut key = MDB_val {
                        mv_size: 0,
                        mv_data: 0 as *mut libc::c_void,
                    };
                    let mut count = 0 as libc::c_int;
                    rc = mdb_cursor_open(txn, dbi, &mut cursor);
                    if rc != 0 {
                        fprintf(
                            __stderrp,
                            b"mdb_cursor_open failed, error %d %s\n\0" as *const u8
                                as *const libc::c_char,
                            rc,
                            mdb_strerror(rc),
                        );
                        current_block = 13407604293425256640;
                    } else {
                        loop {
                            rc = mdb_cursor_get(
                                cursor,
                                &mut key,
                                NULL as *mut MDB_val,
                                MDB_NEXT_NODUP,
                            );
                            if !(rc == 0 as libc::c_int) {
                                break;
                            }
                            let mut str = 0 as *mut libc::c_char;
                            let mut db2: MDB_dbi = 0;
                            if !(memchr(key.mv_data, '\0' as i32, key.mv_size)).is_null()
                            {
                                continue;
                            }
                            count += 1;
                            count;
                            str = malloc(
                                (key.mv_size).wrapping_add(1 as libc::c_int as size_t),
                            ) as *mut libc::c_char;
                            memcpy(str as *mut libc::c_void, key.mv_data, key.mv_size);
                            *str
                                .offset(key.mv_size as isize) = '\0' as i32 as libc::c_char;
                            rc = mdb_dbi_open(
                                txn,
                                str,
                                0 as libc::c_int as libc::c_uint,
                                &mut db2,
                            );
                            if rc == MDB_SUCCESS {
                                if list != 0 {
                                    printf(b"%s\n\0" as *const u8 as *const libc::c_char, str);
                                    list += 1;
                                    list;
                                } else {
                                    rc = dumpit(txn, db2, str);
                                    if rc != 0 {
                                        break;
                                    }
                                }
                                mdb_dbi_close(env, db2);
                            }
                            free(str as *mut libc::c_void);
                            rc != 0;
                        }
                        mdb_cursor_close(cursor);
                        if count == 0 {
                            fprintf(
                                __stderrp,
                                b"%s: %s does not contain multiple databases\n\0"
                                    as *const u8 as *const libc::c_char,
                                prog,
                                envname,
                            );
                            rc = MDB_NOTFOUND;
                        } else if rc == MDB_NOTFOUND {
                            rc = MDB_SUCCESS;
                        }
                        current_block = 7158658067966855297;
                    }
                } else {
                    rc = dumpit(txn, dbi, subname);
                    current_block = 7158658067966855297;
                }
                match current_block {
                    13407604293425256640 => {}
                    _ => {
                        if rc != 0 && rc != MDB_NOTFOUND {
                            fprintf(
                                __stderrp,
                                b"%s: %s: %s\n\0" as *const u8 as *const libc::c_char,
                                prog,
                                envname,
                                mdb_strerror(rc),
                            );
                        }
                        mdb_dbi_close(env, dbi);
                    }
                }
            }
            mdb_txn_abort(txn);
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
