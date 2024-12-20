use std::{ffi::CStr, io};

use ::libc;
use libc::{free, malloc, printf, rand, sprintf, srand};
use lmdb_rust::mdb::{
    mdb_cursor_close, mdb_cursor_get, mdb_cursor_open, mdb_dbi_close, mdb_dbi_open, mdb_del,
    mdb_env_close, mdb_env_create, mdb_env_open, mdb_env_set_mapsize, mdb_env_set_maxreaders,
    mdb_env_stat, mdb_mode_t, mdb_put, mdb_size_t, mdb_strerror, mdb_txn_abort, mdb_txn_begin,
    mdb_txn_commit, MDB_cursor, MDB_dbi, MDB_env, MDB_stat, MDB_txn, MDB_val, MDB_FIRST, MDB_LAST,
    MDB_NEXT, MDB_NOTFOUND, MDB_PREV, NULL,
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
    let mut cur2 = std::ptr::null_mut::<MDB_cursor>();
    let mut op = MDB_FIRST;
    let mut count: libc::c_int = 0;
    let mut values = std::ptr::null_mut::<libc::c_int>();
    let mut sval: [libc::c_char; 32] = [0; 32]; //*::core::mem::transmute::<
                                                //     &[u8; 32],
                                                //     &mut [libc::c_char; 32],
                                                // >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    srand(123);
    count = rand() % 384 as libc::c_int + 64 as libc::c_int;
    values = malloc(
        ((count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)) as usize,
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
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     47 as libc::c_int,
        //     b"mdb_env_create(&env)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
    };
    rc = mdb_env_set_maxreaders(env, 1 as libc::c_int as libc::c_uint);
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     48 as libc::c_int,
        //     b"mdb_env_set_maxreaders(env, 1)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
    };
    rc = mdb_env_set_mapsize(env, 10485760 as libc::c_int as mdb_size_t);
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     49 as libc::c_int,
        //     b"mdb_env_set_mapsize(env, 10485760)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
    };
    rc = mdb_env_open(
        env,
        b"./testdb\0" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int as libc::c_uint,
        0o664 as libc::c_int as mdb_mode_t,
    );
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     50 as libc::c_int,
        //     b"mdb_env_open(env, \"./testdb\", MDB_FIXEDMAP , 0664)\0" as *const u8
        //         as *const libc::c_char,
        //     mdb_strerror(rc),
        // );
        let s = CStr::from_ptr(mdb_strerror(rc));
        panic!("mdb_env_open failed, error {} {}", rc, s.to_str().unwrap());
    };
    rc = mdb_txn_begin(
        env,
        std::ptr::null_mut::<MDB_txn>(),
        0 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     52 as libc::c_int,
        //     b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
    };
    rc = mdb_dbi_open(
        txn,
        std::ptr::null::<libc::c_char>(),
        0 as libc::c_int as libc::c_uint,
        &mut dbi,
    );
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     53 as libc::c_int,
        //     b"mdb_dbi_open(txn, NULL, 0, &dbi)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
    };
    key.mv_size = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong;
    key.mv_data = sval.as_mut_ptr() as *mut libc::c_void;
    printf(
        b"Adding %d values\n\0" as *const u8 as *const libc::c_char,
        count,
    );
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
        rc = mdb_put(
            txn,
            dbi,
            &mut key,
            &mut data,
            0x10 as libc::c_int as libc::c_uint,
        );
        if rc == -(30799 as libc::c_int) || {
            if rc == 0 {
            } else {
                // fprintf(
                //     __stderrp,
                //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                //     b"mtest.c\0" as *const u8 as *const libc::c_char,
                //     64 as libc::c_int,
                //     b"mdb_put(txn, dbi, &key, &data, MDB_NOOVERWRITE)\0" as *const u8
                //         as *const libc::c_char,
                //     mdb_strerror(rc),
                // );
                panic!();
            };
            0 as libc::c_int != 0
        } {
            j += 1;
            j;
            data.mv_size = ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong;
            data.mv_data = sval.as_mut_ptr() as *mut libc::c_void;
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
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     71 as libc::c_int,
        //     b"mdb_txn_commit(txn)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
    };
    rc = mdb_env_stat(env, &mut mst);
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     72 as libc::c_int,
        //     b"mdb_env_stat(env, &mst)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
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
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     74 as libc::c_int,
        //     b"mdb_txn_begin(env, NULL, MDB_RDONLY, &txn)\0" as *const u8
        //         as *const libc::c_char,
        //     mdb_strerror(rc),
        // );
        panic!();
    };
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     75 as libc::c_int,
        //     b"mdb_cursor_open(txn, dbi, &cursor)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
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
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     81 as libc::c_int,
        //     b"mdb_cursor_get\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
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
            std::ptr::null_mut::<MDB_txn>(),
            0 as libc::c_int as libc::c_uint,
            &mut txn,
        );
        if rc == 0 as libc::c_int {
        } else {
            // fprintf(
            //     __stderrp,
            //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            //     b"mtest.c\0" as *const u8 as *const libc::c_char,
            //     90 as libc::c_int,
            //     b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8
            //         as *const libc::c_char,
            //     mdb_strerror(rc),
            // );
            panic!();
        };
        sprintf(
            sval.as_mut_ptr(),
            b"%03x \0" as *const u8 as *const libc::c_char,
            *values.offset(i as isize),
        );
        rc = mdb_del(txn, dbi, &mut key, std::ptr::null_mut::<MDB_val>());
        if rc == MDB_NOTFOUND || {
            if rc == 0 {
            } else {
                // fprintf(
                //     __stderrp,
                //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                //     b"mtest.c\0" as *const u8 as *const libc::c_char,
                //     92 as libc::c_int,
                //     b"mdb_del(txn, dbi, &key, NULL)\0" as *const u8
                //         as *const libc::c_char,
                //     mdb_strerror(rc),
                // );
                panic!();
            };
            0 as libc::c_int != 0
        } {
            j -= 1;
            j;
            mdb_txn_abort(txn);
        } else {
            rc = mdb_txn_commit(txn);
            if rc == 0 as libc::c_int {
            } else {
                // fprintf(
                //     __stderrp,
                //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                //     b"mtest.c\0" as *const u8 as *const libc::c_char,
                //     96 as libc::c_int,
                //     b"mdb_txn_commit(txn)\0" as *const u8 as *const libc::c_char,
                //     mdb_strerror(rc),
                // );
                panic!();
            };
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
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     102 as libc::c_int,
        //     b"mdb_env_stat(env, &mst)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
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
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     103 as libc::c_int,
        //     b"mdb_txn_begin(env, NULL, MDB_RDONLY, &txn)\0" as *const u8
        //         as *const libc::c_char,
        //     mdb_strerror(rc),
        // );
        panic!();
    };
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     104 as libc::c_int,
        //     b"mdb_cursor_open(txn, dbi, &cursor)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
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
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     111 as libc::c_int,
        //     b"mdb_cursor_get\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
    };
    printf(b"Cursor last\n\0" as *const u8 as *const libc::c_char);
    rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_LAST);
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     113 as libc::c_int,
        //     b"mdb_cursor_get(cursor, &key, &data, MDB_LAST)\0" as *const u8
        //         as *const libc::c_char,
        //     mdb_strerror(rc),
        // );
        panic!();
    };
    printf(
        b"key: %.*s, data: %.*s\n\0" as *const u8 as *const libc::c_char,
        key.mv_size as libc::c_int,
        key.mv_data as *mut libc::c_char,
        data.mv_size as libc::c_int,
        data.mv_data as *mut libc::c_char,
    );
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
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     123 as libc::c_int,
        //     b"mdb_cursor_get\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
    };
    printf(b"Cursor last/prev\n\0" as *const u8 as *const libc::c_char);
    rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_LAST);
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     125 as libc::c_int,
        //     b"mdb_cursor_get(cursor, &key, &data, MDB_LAST)\0" as *const u8
        //         as *const libc::c_char,
        //     mdb_strerror(rc),
        // );
        panic!();
    };
    printf(
        b"key: %.*s, data: %.*s\n\0" as *const u8 as *const libc::c_char,
        key.mv_size as libc::c_int,
        key.mv_data as *mut libc::c_char,
        data.mv_size as libc::c_int,
        data.mv_data as *mut libc::c_char,
    );
    rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_PREV);
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     129 as libc::c_int,
        //     b"mdb_cursor_get(cursor, &key, &data, MDB_PREV)\0" as *const u8
        //         as *const libc::c_char,
        //     mdb_strerror(rc),
        // );
        panic!();
    };
    printf(
        b"key: %.*s, data: %.*s\n\0" as *const u8 as *const libc::c_char,
        key.mv_size as libc::c_int,
        key.mv_data as *mut libc::c_char,
        data.mv_size as libc::c_int,
        data.mv_data as *mut libc::c_char,
    );
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    printf(b"Deleting with cursor\n\0" as *const u8 as *const libc::c_char);
    rc = mdb_txn_begin(
        env,
        std::ptr::null_mut::<MDB_txn>(),
        0 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     138 as libc::c_int,
        //     b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
    };
    rc = mdb_cursor_open(txn, dbi, &mut cur2);
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     139 as libc::c_int,
        //     b"mdb_cursor_open(txn, dbi, &cur2)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
    };
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        rc = mdb_cursor_get(cur2, &mut key, &mut data, MDB_NEXT);
        if rc == MDB_NOTFOUND || {
            if rc == 0 {
            } else {
                // fprintf(
                //     __stderrp,
                //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                //     b"mtest.c\0" as *const u8 as *const libc::c_char,
                //     141 as libc::c_int,
                //     b"mdb_cursor_get(cur2, &key, &data, MDB_NEXT)\0" as *const u8
                //         as *const libc::c_char,
                //     mdb_strerror(rc),
                // );
                panic!();
            };
            0 as libc::c_int != 0
        } {
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
        rc = mdb_del(txn, dbi, &mut key, std::ptr::null_mut::<MDB_val>());
        if rc == 0 as libc::c_int {
        } else {
            // fprintf(
            //     __stderrp,
            //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
            //     b"mtest.c\0" as *const u8 as *const libc::c_char,
            //     146 as libc::c_int,
            //     b"mdb_del(txn, dbi, &key, NULL)\0" as *const u8 as *const libc::c_char,
            //     mdb_strerror(rc),
            // );
            panic!();
        };
        i += 1;
        i;
    }
    printf(b"Restarting cursor in txn\n\0" as *const u8 as *const libc::c_char);
    op = MDB_FIRST;
    i = 0 as libc::c_int;
    while i <= 32 as libc::c_int {
        rc = mdb_cursor_get(cur2, &mut key, &mut data, op);
        if rc == MDB_NOTFOUND || {
            if rc == 0 {
            } else {
                // fprintf(
                //     __stderrp,
                //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                //     b"mtest.c\0" as *const u8 as *const libc::c_char,
                //     151 as libc::c_int,
                //     b"mdb_cursor_get(cur2, &key, &data, op)\0" as *const u8
                //         as *const libc::c_char,
                //     mdb_strerror(rc),
                // );
                panic!();
            };
            0 as libc::c_int != 0
        } {
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
        op = MDB_NEXT;
        i += 1;
        i;
    }
    mdb_cursor_close(cur2);
    rc = mdb_txn_commit(txn);
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     158 as libc::c_int,
        //     b"mdb_txn_commit(txn)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
    };
    printf(b"Restarting cursor outside txn\n\0" as *const u8 as *const libc::c_char);
    rc = mdb_txn_begin(
        env,
        std::ptr::null_mut::<MDB_txn>(),
        0 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     161 as libc::c_int,
        //     b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
    };
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if rc == 0 as libc::c_int {
    } else {
        // fprintf(
        //     __stderrp,
        //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
        //     b"mtest.c\0" as *const u8 as *const libc::c_char,
        //     162 as libc::c_int,
        //     b"mdb_cursor_open(txn, dbi, &cursor)\0" as *const u8 as *const libc::c_char,
        //     mdb_strerror(rc),
        // // );
        panic!();
    };
    op = MDB_FIRST;
    i = 0 as libc::c_int;
    while i <= 32 as libc::c_int {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, op);
        if rc == MDB_NOTFOUND || {
            if rc == 0 {
            } else {
                // fprintf(
                //     __stderrp,
                //     b"%s:%d: %s: %s\n\0" as *const u8 as *const libc::c_char,
                //     b"mtest.c\0" as *const u8 as *const libc::c_char,
                //     164 as libc::c_int,
                //     b"mdb_cursor_get(cursor, &key, &data, op)\0" as *const u8
                //         as *const libc::c_char,
                //     mdb_strerror(rc),
                // );
                panic!();
            };
            0 as libc::c_int != 0
        } {
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
        op = MDB_NEXT;
        i += 1;
        i;
    }
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    mdb_dbi_close(env, dbi);
    mdb_env_close(env);
    0 as libc::c_int
}

#[test]
pub fn main() -> io::Result<()> {
    mtest_shared::mtest_wrapper(main_0)
}
