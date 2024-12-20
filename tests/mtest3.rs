use std::io;

use ::libc;
use libc::{free, malloc, printf, rand, sprintf, srand, time, time_t};
use lmdb_rust::mdb::{
    mdb_cursor_close, mdb_cursor_get, mdb_cursor_open, mdb_dbi_close, mdb_dbi_open, mdb_del,
    mdb_env_close, mdb_env_create, mdb_env_open, mdb_env_set_mapsize, mdb_env_set_maxdbs, mdb_env_stat, mdb_mode_t, mdb_put, mdb_size_t,
    mdb_txn_abort, mdb_txn_begin, mdb_txn_commit, MDB_cursor, MDB_dbi, MDB_env, MDB_stat, MDB_txn,
    MDB_val, MDB_NEXT, MDB_NOTFOUND, MDB_PREV, NULL,
};

#[path = "mtest_shared.rs"]
mod mtest_shared;

unsafe fn main_0(_argc: libc::c_int, _argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut i = 0 as libc::c_int;
    let mut j = 0 as libc::c_int;
    let mut rc: libc::c_int = 0;
    let mut env = std::ptr::null_mut::<MDB_env>();
    let mut dbi: MDB_dbi = 0;
    let mut key = MDB_val {
        mv_size: 0,
        mv_data: std::ptr::null_mut::<libc::c_void>(),
    };
    let mut data = MDB_val {
        mv_size: 0,
        mv_data: std::ptr::null_mut::<libc::c_void>(),
    };
    let mut txn = std::ptr::null_mut::<MDB_txn>();
    let mut mst = MDB_stat {
        ms_psize: 0,
        ms_depth: 0,
        ms_branch_pages: 0,
        ms_leaf_pages: 0,
        ms_overflow_pages: 0,
        ms_entries: 0,
    };
    let mut cursor = std::ptr::null_mut::<MDB_cursor>();
    let mut count: libc::c_int = 0;
    let mut values = std::ptr::null_mut::<libc::c_int>();
    let mut sval: [libc::c_char; 32] = [0; 32];
    let mut kval: [libc::c_char; 4] = [0; 4];
    srand(time(NULL as *mut time_t) as libc::c_uint);

    count = rand() % 384 as libc::c_int + 64 as libc::c_int;
    values = malloc(
        (count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong) as usize,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        *values.offset(i as isize) = rand() % 1024 as libc::c_int;
        i += 1;
        i;
    }
    rc = mdb_env_create(&mut env);
    if rc == 0 as libc::c_int {
    } else {
        panic!();
    };
    rc = mdb_env_set_mapsize(env, 10485760 as libc::c_int as mdb_size_t);
    if rc == 0 as libc::c_int {
    } else {
        panic!();
    };
    rc = mdb_env_set_maxdbs(env, 4 as libc::c_int as MDB_dbi);
    if rc == 0 as libc::c_int {
    } else {
        panic!();
    };
    rc = mdb_env_open(
        env,
        b"./testdb\0" as *const u8 as *const libc::c_char,
        (0x1 as libc::c_int | 0x10000 as libc::c_int) as libc::c_uint,
        0o664 as libc::c_int as mdb_mode_t,
    );
    if rc == 0 as libc::c_int {
    } else {
        panic!();
    };
    rc = mdb_txn_begin(
        env,
        std::ptr::null_mut::<MDB_txn>(),
        0 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {
    } else {
        panic!();
    };
    rc = mdb_dbi_open(
        txn,
        b"id2\0" as *const u8 as *const libc::c_char,
        (0x40000 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint,
        &mut dbi,
    );
    if rc == 0 as libc::c_int {
    } else {
        panic!();
    };
    key.mv_size = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong;
    key.mv_data = kval.as_mut_ptr() as *mut libc::c_void;
    data.mv_size = ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong;
    data.mv_data = sval.as_mut_ptr() as *mut libc::c_void;
    printf(
        b"Adding %d values\n\0" as *const u8 as *const libc::c_char,
        count,
    );
    i = 0 as libc::c_int;
    while i < count {
        if i & 0xf as libc::c_int == 0 {
            sprintf(
                kval.as_mut_ptr(),
                b"%03x\0" as *const u8 as *const libc::c_char,
                *values.offset(i as isize),
            );
        }
        sprintf(
            sval.as_mut_ptr(),
            b"%03x %d foo bar\0" as *const u8 as *const libc::c_char,
            *values.offset(i as isize),
            *values.offset(i as isize),
        );
        rc = mdb_put(
            txn,
            dbi,
            &mut key,
            &mut data,
            0x20 as libc::c_int as libc::c_uint,
        );
        if rc == -(30799 as libc::c_int) || {
            if rc == 0 {
            } else {
                panic!();
            };
            0 as libc::c_int != 0
        } {
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    if j != 0 {
        printf(
            b"%d duplicates skipped\n\0" as *const u8 as *const libc::c_char,
            j,
        );
    }
    rc = mdb_txn_commit(txn);
    if rc == 0 as libc::c_int {
    } else {
        panic!();
    };
    rc = mdb_env_stat(env, &mut mst);
    if rc == 0 as libc::c_int {
    } else {
        panic!();
    };
    rc = mdb_txn_begin(
        env,
        std::ptr::null_mut::<MDB_txn>(),
        0x20000 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {
    } else {
        panic!();
    };
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if rc == 0 as libc::c_int {
    } else {
        panic!();
    };
    loop {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_NEXT);
        if rc != 0 as libc::c_int {
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
    if rc == MDB_NOTFOUND {
    } else {
        panic!();
    };
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    j = 0 as libc::c_int;
    i = count - 1 as libc::c_int;
    while i > -(1 as libc::c_int) {
        j += 1;
        j;
        txn = NULL as *mut MDB_txn;
        rc = mdb_txn_begin(
            env,
            std::ptr::null_mut::<MDB_txn>(),
            0 as libc::c_int as libc::c_uint,
            &mut txn,
        );
        if rc == 0 as libc::c_int {
        } else {
            panic!();
        };
        sprintf(
            kval.as_mut_ptr(),
            b"%03x\0" as *const u8 as *const libc::c_char,
            *values.offset((i & !(0xf as libc::c_int)) as isize),
        );
        sprintf(
            sval.as_mut_ptr(),
            b"%03x %d foo bar\0" as *const u8 as *const libc::c_char,
            *values.offset(i as isize),
            *values.offset(i as isize),
        );
        key.mv_size = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong;
        key.mv_data = kval.as_mut_ptr() as *mut libc::c_void;
        data.mv_size = ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong;
        data.mv_data = sval.as_mut_ptr() as *mut libc::c_void;
        rc = mdb_del(txn, dbi, &mut key, &mut data);
        if rc == MDB_NOTFOUND || {
            if rc == 0 {
            } else {
                panic!();
            }

            0 as libc::c_int != 0
        } {
            j -= 1;
            j;
            mdb_txn_abort(txn);
        } else {
            rc = mdb_txn_commit(txn);
            if rc == 0 as libc::c_int {
            } else {
                panic!();
            }
        }
        i -= rand() % 5 as libc::c_int;
    }
    free(values as *mut libc::c_void);
    printf(
        b"Deleted %d values\n\0" as *const u8 as *const libc::c_char,
        j,
    );
    rc = mdb_env_stat(env, &mut mst);
    if rc == 0 as libc::c_int {
    } else {
        panic!();
    };
    rc = mdb_txn_begin(
        env,
        std::ptr::null_mut::<MDB_txn>(),
        0x20000 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {
    } else {
        panic!();
    };
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if rc == 0 as libc::c_int {
    } else {
        panic!();
    };
    printf(b"Cursor next\n\0" as *const u8 as *const libc::c_char);
    loop {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_NEXT);
        if rc != 0 as libc::c_int {
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
    if rc == MDB_NOTFOUND {
    } else {
        panic!();
    };
    printf(b"Cursor prev\n\0" as *const u8 as *const libc::c_char);
    loop {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_PREV);
        if rc != 0 as libc::c_int {
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
    if rc == MDB_NOTFOUND {
    } else {
        panic!();
    };
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    mdb_dbi_close(env, dbi);
    mdb_env_close(env);
    0 as libc::c_int
}

#[test]
fn main() -> io::Result<()> {
    mtest_shared::mtest_wrapper(main_0)
}
