use std::{ffi::CStr, os, io};

use ::libc;
use libc::{srand, rand, time, time_t, malloc, fprintf, abort, printf, sprintf, free};
use lmdb_rust::mdb::{MDB_env, MDB_dbi, MDB_val, MDB_txn, MDB_cursor, MDB_FIRST, NULL, mdb_env_create, mdb_env_set_maxreaders, MDB_stat, mdb_env_set_mapsize, mdb_size_t, mdb_env_open, mdb_mode_t, mdb_txn_begin, mdb_dbi_open, mdb_put, mdb_txn_commit, mdb_env_stat, mdb_cursor_open, MDB_NEXT, mdb_cursor_get, mdb_cursor_close, mdb_txn_abort, mdb_del, MDB_LAST, MDB_PREV, mdb_dbi_close, mdb_env_close, mdb_strerror};

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
    let mut cur2 = 0 as *mut MDB_cursor;
    let mut op = MDB_FIRST;
    let mut count: libc::c_int = 0;
    let mut values = 0 as *mut libc::c_int;
    let mut sval: [libc::c_char; 32] = [0; 32];//*::core::mem::transmute::<
    //     &[u8; 32],
    //     &mut [libc::c_char; 32],
    // >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    srand(123);
    count = rand() % 384 as libc::c_int + 64 as libc::c_int;
    values = malloc(
       ( (count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)) as usize,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        *values.offset(i as isize) = rand() % 1024 as libc::c_int;
        i += 1;
        i;
    }
    rc = mdb_env_create(&mut env);
    if rc == 0 as libc::c_int {} else {
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
    if rc == 0 as libc::c_int {} else {
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
    if rc == 0 as libc::c_int {} else {
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
    if rc == 0 as libc::c_int {} else {
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
        0 as *mut MDB_txn,
        0 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {} else {
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
        0 as *const libc::c_char,
        0 as libc::c_int as libc::c_uint,
        &mut dbi,
    );
    if rc == 0 as libc::c_int {} else {
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
            }
        {
            j += 1;
            j;
            data.mv_size = ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong;
            data.mv_data = sval.as_mut_ptr() as *mut libc::c_void;
        }
        i += 1;
        i;
    }
    if j != 0 {
        printf(b"%d duplicates skipped\n\0" as *const u8 as *const libc::c_char, j);
    }
    rc = mdb_txn_commit(txn);
    if rc == 0 as libc::c_int {} else {
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
    if rc == 0 as libc::c_int {} else {
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
        0 as *mut MDB_txn,
        0x20000 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {} else {
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
    if rc == 0 as libc::c_int {} else {
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
            0 as *mut MDB_txn,
            0 as libc::c_int as libc::c_uint,
            &mut txn,
        );
        if rc == 0 as libc::c_int {} else {
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
        rc = mdb_del(txn, dbi, &mut key, 0 as *mut MDB_val);
        if rc == -(30798 as libc::c_int)
            || {
                if rc == 0 {} else {
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
            }
        {
            j -= 1;
            j;
            mdb_txn_abort(txn);
        } else {
            rc = mdb_txn_commit(txn);
            if rc == 0 as libc::c_int {} else {
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
    printf(b"Deleted %d values\n\0" as *const u8 as *const libc::c_char, j);
    rc = mdb_env_stat(env, &mut mst);
    if rc == 0 as libc::c_int {} else {
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
        0 as *mut MDB_txn,
        0x20000 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {} else {
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
    if rc == 0 as libc::c_int {} else {
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
    if rc == 0 as libc::c_int {} else {
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
    if rc == 0 as libc::c_int {} else {
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
    if rc == 0 as libc::c_int {} else {
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
        0 as *mut MDB_txn,
        0 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {} else {
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
    if rc == 0 as libc::c_int {} else {
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
        if rc == -(30798 as libc::c_int)
            || {
                if rc == 0 {} else {
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
            }
        {
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
        rc = mdb_del(txn, dbi, &mut key, 0 as *mut MDB_val);
        if rc == 0 as libc::c_int {} else {
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
        if rc == -(30798 as libc::c_int)
            || {
                if rc == 0 {} else {
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
            }
        {
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
    if rc == 0 as libc::c_int {} else {
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
        0 as *mut MDB_txn,
        0 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc == 0 as libc::c_int {} else {
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
    if rc == 0 as libc::c_int {} else {
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
        if rc == -(30798 as libc::c_int)
            || {
                if rc == 0 {} else {
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
            }
        {
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
    return 0 as libc::c_int;
}

#[test]
pub fn main() -> io::Result<()> {
    let temp = tempfile::tempdir()?;
    let mut testdb = temp.path().to_path_buf();
    testdb.push("testdb");
    std::fs::create_dir(testdb)?;
    std::env::set_current_dir(temp.path())?;

    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());

    if unsafe { main_0(
        (args.len() - 1) as libc::c_int,
        args.as_mut_ptr() as *mut *mut libc::c_char,
    ) } != 0 {
        panic!("main_0 failed");
    }

    Ok(())
}
