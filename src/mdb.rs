#![allow(path_statements)]

use std::mem;

use ::libc;
use libc::FILE;

use crate::errno;
extern "C" {
    fn fstat(_: libc::c_int, _: *mut stat) -> libc::c_int;
    fn writev(_: libc::c_int, _: *const iovec, _: libc::c_int) -> ssize_t;
    fn mmap(
        _: *mut libc::c_void,
        _: size_t,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: off_t,
    ) -> *mut libc::c_void;
    fn msync(_: *mut libc::c_void, _: size_t, _: libc::c_int) -> libc::c_int;
    fn munmap(_: *mut libc::c_void, _: size_t) -> libc::c_int;
    fn madvise(_: *mut libc::c_void, _: size_t, _: libc::c_int) -> libc::c_int;
    fn open(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn fcntl(_: libc::c_int, _: libc::c_int, _: ...) -> libc::c_int;
    fn __error() -> *mut libc::c_int;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn posix_memalign(
        __memptr: *mut *mut libc::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> libc::c_int;
    fn abort() -> !;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn close(_: libc::c_int) -> libc::c_int;
    fn getpid() -> pid_t;
    fn lseek(_: libc::c_int, _: off_t, _: libc::c_int) -> off_t;
    fn sysconf(_: libc::c_int) -> libc::c_long;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __nbyte: size_t) -> ssize_t;
    fn pread(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbyte: size_t,
        __offset: off_t,
    ) -> ssize_t;
    fn pwrite(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __nbyte: size_t,
        __offset: off_t,
    ) -> ssize_t;
    fn fsync(_: libc::c_int) -> libc::c_int;
    fn ftruncate(_: libc::c_int, _: off_t) -> libc::c_int;
    fn pthread_cond_destroy(_: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_init(_: *mut pthread_cond_t, _: *const pthread_condattr_t) -> libc::c_int;
    fn pthread_cond_signal(_: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(_: *mut pthread_cond_t, _: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_create(
        _: *mut pthread_t,
        _: *const pthread_attr_t,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_getspecific(_: pthread_key_t) -> *mut libc::c_void;
    fn pthread_join(_: pthread_t, _: *mut *mut libc::c_void) -> libc::c_int;
    fn pthread_key_create(
        _: *mut pthread_key_t,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn pthread_key_delete(_: pthread_key_t) -> libc::c_int;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_init(_: *mut pthread_mutex_t, _: *const pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_setspecific(_: pthread_key_t, _: *const libc::c_void) -> libc::c_int;
    fn pthread_sigmask(_: libc::c_int, _: *const sigset_t, _: *mut sigset_t) -> libc::c_int;
    fn sigwait(_: *const sigset_t, _: *mut libc::c_int) -> libc::c_int;
    fn ftok(_: *const libc::c_char, _: libc::c_int) -> key_t;
    fn semctl(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: ...) -> libc::c_int;
    fn semget(_: key_t, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn semop(_: libc::c_int, _: *mut sembuf, _: size_t) -> libc::c_int;
    fn mdb_midl_search(ids: MDB_IDL, id: MDB_ID) -> libc::c_uint;
    fn mdb_midl_alloc(num: libc::c_int) -> MDB_IDL;
    fn mdb_midl_free(ids: MDB_IDL);
    fn mdb_midl_shrink(idp: *mut MDB_IDL);
    fn mdb_midl_need(idp: *mut MDB_IDL, num: libc::c_uint) -> libc::c_int;
    fn mdb_midl_append(idp: *mut MDB_IDL, id: MDB_ID) -> libc::c_int;
    fn mdb_midl_append_list(idp: *mut MDB_IDL, app: MDB_IDL) -> libc::c_int;
    fn mdb_midl_append_range(idp: *mut MDB_IDL, id: MDB_ID, n: libc::c_uint) -> libc::c_int;
    fn mdb_midl_xmerge(idl: MDB_IDL, merge: MDB_IDL);
    fn mdb_midl_sort(ids: MDB_IDL);
    fn mdb_mid2l_search(ids: MDB_ID2L, id: MDB_ID) -> libc::c_uint;
    fn mdb_mid2l_insert(ids: MDB_ID2L, id: *mut MDB_ID2) -> libc::c_int;
    fn mdb_mid2l_append(ids: MDB_ID2L, id: *mut MDB_ID2) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __uint64_t = libc::c_ulonglong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_uid_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub __arg: *mut libc::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_attr_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_cond_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: libc::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [libc::c_char; 8176],
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = libc::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type dev_t = __darwin_dev_t;
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type gid_t = __darwin_gid_t;
pub type key_t = __int32_t;
pub type mode_t = __darwin_mode_t;
pub type nlink_t = __uint16_t;
pub type pid_t = __darwin_pid_t;
pub type off_t = __darwin_off_t;
pub type uid_t = __darwin_uid_t;
pub type size_t = __darwin_size_t;
pub type ssize_t = __darwin_ssize_t;
pub type time_t = __darwin_time_t;
pub type pthread_attr_t = __darwin_pthread_attr_t;
pub type pthread_cond_t = __darwin_pthread_cond_t;
pub type pthread_condattr_t = __darwin_pthread_condattr_t;
pub type pthread_mutex_t = __darwin_pthread_mutex_t;
pub type pthread_mutexattr_t = __darwin_pthread_mutexattr_t;
pub type pthread_t = __darwin_pthread_t;
pub type pthread_key_t = __darwin_pthread_key_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_ino: __darwin_ino64_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: blksize_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
    pub st_lspare: __int32_t,
    pub st_qspare: [__int64_t; 2],
}
pub type sigset_t = __darwin_sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_start: off_t,
    pub l_len: off_t,
    pub l_pid: pid_t,
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
}
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C, packed(4))]
pub struct ipc_perm {
    pub uid: uid_t,
    pub gid: gid_t,
    pub cuid: uid_t,
    pub cgid: gid_t,
    pub mode: mode_t,
    pub _seq: libc::c_ushort,
    pub _key: key_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed(4))]
pub struct __semid_ds_new {
    pub sem_perm: ipc_perm,
    pub sem_base: __int32_t,
    pub sem_nsems: libc::c_ushort,
    pub sem_otime: time_t,
    pub sem_pad1: __int32_t,
    pub sem_ctime: time_t,
    pub sem_pad2: __int32_t,
    pub sem_pad3: [__int32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sembuf {
    pub sem_num: libc::c_ushort,
    pub sem_op: libc::c_short,
    pub sem_flg: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union semun {
    pub val: libc::c_int,
    pub buf: *mut __semid_ds_new,
    pub array: *mut libc::c_ushort,
}
pub type mdb_mode_t = mode_t;
pub type mdb_size_t = size_t;
pub type mdb_filehandle_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_env {
    pub me_fd: libc::c_int,
    pub me_lfd: libc::c_int,
    pub me_mfd: libc::c_int,
    pub me_flags: uint32_t,
    pub me_psize: libc::c_uint,
    pub me_os_psize: libc::c_uint,
    pub me_maxreaders: libc::c_uint,
    pub me_close_readers: libc::c_int,
    pub me_numdbs: MDB_dbi,
    pub me_maxdbs: MDB_dbi,
    pub me_pid: pid_t,
    pub me_path: *mut libc::c_char,
    pub me_map: *mut libc::c_char,
    pub me_txns: *mut MDB_txninfo,
    pub me_metas: [*mut MDB_meta; 2],
    pub me_pbuf: *mut libc::c_void,
    pub me_txn: *mut MDB_txn,
    pub me_txn0: *mut MDB_txn,
    pub me_mapsize: mdb_size_t,
    pub me_size: off_t,
    pub me_maxpg: pgno_t,
    pub me_dbxs: *mut MDB_dbx,
    pub me_dbflags: *mut uint16_t,
    pub me_dbiseqs: *mut libc::c_uint,
    pub me_txkey: pthread_key_t,
    pub me_pgoldest: txnid_t,
    pub me_pgstate: MDB_pgstate,
    pub me_dpages: *mut MDB_page,
    pub me_free_pgs: MDB_IDL,
    pub me_dirty_list: MDB_ID2L,
    pub me_maxfree_1pg: libc::c_int,
    pub me_nodemax: libc::c_uint,
    pub me_live_reader: libc::c_int,
    pub me_rmutex: mdb_mutex_t,
    pub me_wmutex: mdb_mutex_t,
    pub me_userctx: *mut libc::c_void,
    pub me_assert_func: Option<MDB_assert_func>,
}
pub type MDB_assert_func = unsafe extern "C" fn(*mut MDB_env, *const libc::c_char) -> ();
pub type mdb_mutex_t = [mdb_mutex; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdb_mutex {
    pub semid: libc::c_int,
    pub semnum: libc::c_int,
    pub locked: *mut libc::c_int,
}
pub type MDB_ID2L = *mut MDB_ID2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_ID2 {
    pub mid: MDB_ID,
    pub mptr: *mut libc::c_void,
}
pub type MDB_ID = mdb_size_t;
pub type MDB_IDL = *mut MDB_ID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_page {
    pub mp_p: C2RustUnnamed_1,
    pub mp_pad: uint16_t,
    pub mp_flags: uint16_t,
    pub mp_pb: C2RustUnnamed,
    pub mp_ptrs: [indx_t; 0],
}
pub type indx_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub pb: C2RustUnnamed_0,
    pub pb_pages: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub pb_lower: indx_t,
    pub pb_upper: indx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub p_pgno: pgno_t,
    pub p_next: *mut MDB_page,
}
pub type pgno_t = MDB_ID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_pgstate {
    pub mf_pghead: *mut pgno_t,
    pub mf_pglast: txnid_t,
}
pub type txnid_t = MDB_ID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_dbx {
    pub md_name: MDB_val,
    pub md_cmp: Option<MDB_cmp_func>,
    pub md_dcmp: Option<MDB_cmp_func>,
    pub md_rel: Option<MDB_rel_func>,
    pub md_relctx: *mut libc::c_void,
}
pub type MDB_rel_func = unsafe extern "C" fn(
    *mut MDB_val,
    *mut libc::c_void,
    *mut libc::c_void,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_val {
    pub mv_size: size_t,
    pub mv_data: *mut libc::c_void,
}
pub type MDB_cmp_func = unsafe extern "C" fn(*const MDB_val, *const MDB_val) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_txn {
    pub mt_parent: *mut MDB_txn,
    pub mt_child: *mut MDB_txn,
    pub mt_next_pgno: pgno_t,
    pub mt_txnid: txnid_t,
    pub mt_env: *mut MDB_env,
    pub mt_free_pgs: MDB_IDL,
    pub mt_loose_pgs: *mut MDB_page,
    pub mt_loose_count: libc::c_int,
    pub mt_spill_pgs: MDB_IDL,
    pub mt_u: C2RustUnnamed_2,
    pub mt_dbxs: *mut MDB_dbx,
    pub mt_dbs: *mut MDB_db,
    pub mt_dbiseqs: *mut libc::c_uint,
    pub mt_cursors: *mut *mut MDB_cursor,
    pub mt_dbflags: *mut libc::c_uchar,
    pub mt_numdbs: MDB_dbi,
    pub mt_flags: libc::c_uint,
    pub mt_dirty_room: libc::c_uint,
}
pub type MDB_dbi = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_cursor {
    pub mc_next: *mut MDB_cursor,
    pub mc_backup: *mut MDB_cursor,
    pub mc_xcursor: *mut MDB_xcursor,
    pub mc_txn: *mut MDB_txn,
    pub mc_dbi: MDB_dbi,
    pub mc_db: *mut MDB_db,
    pub mc_dbx: *mut MDB_dbx,
    pub mc_dbflag: *mut libc::c_uchar,
    pub mc_snum: libc::c_ushort,
    pub mc_top: libc::c_ushort,
    pub mc_flags: libc::c_uint,
    pub mc_pg: [*mut MDB_page; 32],
    pub mc_ki: [indx_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_db {
    pub md_pad: uint32_t,
    pub md_flags: uint16_t,
    pub md_depth: uint16_t,
    pub md_branch_pages: pgno_t,
    pub md_leaf_pages: pgno_t,
    pub md_overflow_pages: pgno_t,
    pub md_entries: mdb_size_t,
    pub md_root: pgno_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_xcursor {
    pub mx_cursor: MDB_cursor,
    pub mx_db: MDB_db,
    pub mx_dbx: MDB_dbx,
    pub mx_dbflag: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub dirty_list: MDB_ID2L,
    pub reader: *mut MDB_reader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_reader {
    pub mru: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub mrx: MDB_rxbody,
    pub pad: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_rxbody {
    pub mrb_txnid: txnid_t,
    pub mrb_pid: pid_t,
    pub mrb_tid: pthread_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_meta {
    pub mm_magic: uint32_t,
    pub mm_version: uint32_t,
    pub mm_address: *mut libc::c_void,
    pub mm_mapsize: mdb_size_t,
    pub mm_dbs: [MDB_db; 2],
    pub mm_last_pg: pgno_t,
    pub mm_txnid: txnid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_txninfo {
    pub mt1: C2RustUnnamed_5,
    pub mt2: C2RustUnnamed_4,
    pub mti_readers: [MDB_reader; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub mt2_wlocked: libc::c_int,
    pub pad: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub mtb: MDB_txbody,
    pub pad: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_txbody {
    pub mtb_magic: uint32_t,
    pub mtb_format: uint32_t,
    pub mtb_txnid: txnid_t,
    pub mtb_numreaders: libc::c_uint,
    pub mtb_semid: libc::c_int,
    pub mtb_rlocked: libc::c_int,
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
pub type mdb_nchar_t = libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_name {
    pub mn_len: libc::c_int,
    pub mn_alloced: libc::c_int,
    pub mn_val: *mut mdb_nchar_t,
}
pub type mdb_fopen_type = libc::c_uint;
pub const MDB_O_LOCKS: mdb_fopen_type = 16777734;
pub const MDB_O_MASK: mdb_fopen_type = 20974083;
pub const MDB_O_COPY: mdb_fopen_type = 16779777;
pub const MDB_O_META: mdb_fopen_type = 20971521;
pub const MDB_O_RDWR: mdb_fopen_type = 514;
pub const MDB_O_RDONLY: mdb_fopen_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union MDB_metabuf {
    pub mb_page: MDB_page,
    pub mb_metabuf: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub mm_pad: [libc::c_char; 16],
    pub mm_meta: MDB_meta,
}
pub const Size: C2RustUnnamed_7 = 152;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const MDB_lock_desc: C2RustUnnamed_11 = 166042;
pub const MDB_END_ABORT: C2RustUnnamed_12 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_ntxn {
    pub mnt_txn: MDB_txn,
    pub mnt_pgstate: MDB_pgstate,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_page2 {
    pub mp2_p: [uint16_t; 4],
    pub mp2_pad: uint16_t,
    pub mp2_flags: uint16_t,
    pub mp2_lower: indx_t,
    pub mp2_upper: indx_t,
    pub mp2_ptrs: [indx_t; 0],
}
pub type mdb_mutexref_t = *mut mdb_mutex;
pub const MDB_END_FAIL_BEGIN: C2RustUnnamed_12 = 5;
pub type Pidlock_op = libc::c_uint;
pub const Pidcheck: Pidlock_op = 7;
pub const Pidset: Pidlock_op = 8;
pub const MDB_END_RESET_TMP: C2RustUnnamed_12 = 4;
pub const MDB_END_FAIL_BEGINCHILD: C2RustUnnamed_12 = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdb_copy {
    pub mc_env: *mut MDB_env,
    pub mc_txn: *mut MDB_txn,
    pub mc_mutex: pthread_mutex_t,
    pub mc_cond: pthread_cond_t,
    pub mc_wbuf: [*mut libc::c_char; 2],
    pub mc_over: [*mut libc::c_char; 2],
    pub mc_wlen: [libc::c_int; 2],
    pub mc_olen: [libc::c_int; 2],
    pub mc_next_pgno: pgno_t,
    pub mc_fd: libc::c_int,
    pub mc_toggle: libc::c_int,
    pub mc_new: libc::c_int,
    pub mc_error: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_node {
    pub mn_lo: libc::c_ushort,
    pub mn_hi: libc::c_ushort,
    pub mn_flags: libc::c_ushort,
    pub mn_ksize: libc::c_ushort,
    pub mn_data: [libc::c_char; 1],
}
pub const Align: C2RustUnnamed_8 = 8;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const Paranoid: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const Max_retries: C2RustUnnamed_9 = 2147483647;
pub const MDB_END_COMMITTED: C2RustUnnamed_12 = 0;
pub const Mask: C2RustUnnamed_10 = 49232;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const MDB_END_EMPTY_COMMIT: C2RustUnnamed_12 = 1;
pub const MDB_END_RESET: C2RustUnnamed_12 = 3;
pub type MDB_msg_func = unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int;
pub type C2RustUnnamed_11 = libc::c_uint;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const __DARWIN_NULL: libc::c_int = 0 as libc::c_int;
pub const __DARWIN_NSIG: libc::c_int = 32 as libc::c_int;
pub const SIG_BLOCK: libc::c_int = 1 as libc::c_int;
pub const PROT_READ: libc::c_int = 0x1 as libc::c_int;
pub const PROT_WRITE: libc::c_int = 0x2 as libc::c_int;
pub const MAP_SHARED: libc::c_int = 0x1 as libc::c_int;
pub const MAP_FAILED: libc::c_int = -(1 as libc::c_int);
pub const MS_ASYNC: libc::c_int = 0x1 as libc::c_int;
pub const MS_SYNC: libc::c_int = 0x10 as libc::c_int;
pub const POSIX_MADV_RANDOM: libc::c_int = 1 as libc::c_int;
pub const MADV_RANDOM: libc::c_int = POSIX_MADV_RANDOM;
pub const O_CLOEXEC: libc::c_int = 0x1000000 as libc::c_int;
pub const F_GETFD: libc::c_int = 1 as libc::c_int;
pub const F_SETFD: libc::c_int = 2 as libc::c_int;
pub const F_GETLK: libc::c_int = 7 as libc::c_int;
pub const F_SETLK: libc::c_int = 8 as libc::c_int;
pub const F_SETLKW: libc::c_int = 9 as libc::c_int;
pub const F_NOCACHE: libc::c_int = 48 as libc::c_int;
pub const FD_CLOEXEC: libc::c_int = 1 as libc::c_int;
pub const F_RDLCK: libc::c_int = 1 as libc::c_int;
pub const F_UNLCK: libc::c_int = 2 as libc::c_int;
pub const F_WRLCK: libc::c_int = 3 as libc::c_int;
pub const ENOENT: libc::c_int = 2 as libc::c_int;
pub const EINTR: libc::c_int = 4 as libc::c_int;
pub const EIO: libc::c_int = 5 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EACCES: libc::c_int = 13 as libc::c_int;
pub const EBUSY: libc::c_int = 16 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const ENOSPC: libc::c_int = 28 as libc::c_int;
pub const EROFS: libc::c_int = 30 as libc::c_int;
pub const EPIPE: libc::c_int = 32 as libc::c_int;
pub const EAGAIN: libc::c_int = 35 as libc::c_int;
pub const UINTPTR_MAX: libc::c_ulong = libc::c_ulong::MAX;
pub const SIZE_MAX: libc::c_ulong = UINTPTR_MAX;
pub const _SC_PAGESIZE: libc::c_int = 29 as libc::c_int;
pub const _SC_PAGE_SIZE: libc::c_int = _SC_PAGESIZE;
pub const MDB_FDATASYNC: unsafe extern "C" fn(libc::c_int) -> libc::c_int = fsync;
#[inline(always)]
unsafe extern "C" fn __sigbits(mut __signo: libc::c_int) -> libc::c_int {
    return if __signo > __DARWIN_NSIG {
        0 as libc::c_int
    } else {
        (1 as libc::c_int) << __signo - 1 as libc::c_int
    };
}
pub const IPC_CREAT: libc::c_int = 0o1000 as libc::c_int;
pub const IPC_RMID: libc::c_int = 0 as libc::c_int;
pub const IPC_SET: libc::c_int = 1 as libc::c_int;
pub const IPC_STAT: libc::c_int = 2 as libc::c_int;
pub const SETALL: libc::c_int = 9 as libc::c_int;
pub const SEM_UNDO: libc::c_int = 0o10000 as libc::c_int;
pub const MDB_SIZE_MAX: libc::c_ulong = SIZE_MAX;
pub const MDB_VERSION_MAJOR: libc::c_int = 0 as libc::c_int;
pub const MDB_VERSION_MINOR: libc::c_int = 9 as libc::c_int;
pub const MDB_VERSION_PATCH: libc::c_int = 70 as libc::c_int;
pub const MDB_VERSION_STRING: &'static [u8] = b"LMDB 0.9.70: (December 19, 2015)\0";
pub const MDB_FIXEDMAP: libc::c_int = 0x1 as libc::c_int;
pub const MDB_NOSUBDIR: libc::c_int = 0x4000 as libc::c_int;
pub const MDB_NOSYNC: libc::c_int = 0x10000 as libc::c_int;
pub const MDB_RDONLY: libc::c_int = 0x20000 as libc::c_int;
pub const MDB_NOMETASYNC: libc::c_int = 0x40000 as libc::c_int;
pub const MDB_WRITEMAP: libc::c_int = 0x80000 as libc::c_int;
pub const MDB_MAPASYNC: libc::c_int = 0x100000 as libc::c_int;
pub const MDB_NOTLS: libc::c_int = 0x200000 as libc::c_int;
pub const MDB_NOLOCK: libc::c_int = 0x400000 as libc::c_int;
pub const MDB_NORDAHEAD: libc::c_int = 0x800000 as libc::c_int;
pub const MDB_NOMEMINIT: libc::c_int = 0x1000000 as libc::c_int;
pub const MDB_PREVSNAPSHOT: libc::c_int = 0x2000000 as libc::c_int;
pub const MDB_REVERSEKEY: libc::c_int = 0x2 as libc::c_int;
pub const MDB_DUPSORT: libc::c_int = 0x4 as libc::c_int;
pub const MDB_INTEGERKEY: libc::c_int = 0x8 as libc::c_int;
pub const MDB_DUPFIXED: libc::c_int = 0x10 as libc::c_int;
pub const MDB_INTEGERDUP: libc::c_int = 0x20 as libc::c_int;
pub const MDB_REVERSEDUP: libc::c_int = 0x40 as libc::c_int;
pub const MDB_CREATE: libc::c_int = 0x40000 as libc::c_int;
pub const MDB_NOOVERWRITE: libc::c_int = 0x10 as libc::c_int;
pub const MDB_NODUPDATA: libc::c_int = 0x20 as libc::c_int;
pub const MDB_CURRENT: libc::c_int = 0x40 as libc::c_int;
pub const MDB_RESERVE: libc::c_int = 0x10000 as libc::c_int;
pub const MDB_APPEND: libc::c_int = 0x20000 as libc::c_int;
pub const MDB_APPENDDUP: libc::c_int = 0x40000 as libc::c_int;
pub const MDB_MULTIPLE: libc::c_int = 0x80000 as libc::c_int;
pub const MDB_CP_COMPACT: libc::c_int = 0x1 as libc::c_int;
pub const MDB_SUCCESS: libc::c_int = 0 as libc::c_int;
pub const MDB_KEYEXIST: libc::c_int = -(30799 as libc::c_int);
pub const MDB_NOTFOUND: libc::c_int = -30798;
pub const MDB_PAGE_NOTFOUND: libc::c_int = -(30797 as libc::c_int);
pub const MDB_CORRUPTED: libc::c_int = -(30796 as libc::c_int);
pub const MDB_PANIC: libc::c_int = -(30795 as libc::c_int);
pub const MDB_VERSION_MISMATCH: libc::c_int = -(30794 as libc::c_int);
pub const MDB_INVALID: libc::c_int = -(30793 as libc::c_int);
pub const MDB_MAP_FULL: libc::c_int = -(30792 as libc::c_int);
pub const MDB_DBS_FULL: libc::c_int = -(30791 as libc::c_int);
pub const MDB_READERS_FULL: libc::c_int = -(30790 as libc::c_int);
pub const MDB_TXN_FULL: libc::c_int = -(30788 as libc::c_int);
pub const MDB_CURSOR_FULL: libc::c_int = -(30787 as libc::c_int);
pub const MDB_PAGE_FULL: libc::c_int = -(30786 as libc::c_int);
pub const MDB_MAP_RESIZED: libc::c_int = -(30785 as libc::c_int);
pub const MDB_INCOMPATIBLE: libc::c_int = -(30784 as libc::c_int);
pub const MDB_BAD_RSLOT: libc::c_int = -(30783 as libc::c_int);
pub const MDB_BAD_TXN: libc::c_int = -(30782 as libc::c_int);
pub const MDB_BAD_VALSIZE: libc::c_int = -(30781 as libc::c_int);
pub const MDB_BAD_DBI: libc::c_int = -(30780 as libc::c_int);
pub const MDB_PROBLEM: libc::c_int = -(30779 as libc::c_int);
pub const MDB_LAST_ERRCODE: libc::c_int = MDB_PROBLEM;
pub const MDB_IDL_LOGN: libc::c_int = 16 as libc::c_int;
pub const MDB_IDL_UM_SIZE: libc::c_int = (1 as libc::c_int) << MDB_IDL_LOGN + 1 as libc::c_int;
pub const MDB_IDL_UM_MAX: libc::c_int = MDB_IDL_UM_SIZE - 1 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const UINT_MAX: libc::c_uint = libc::c_uint::MAX;
pub const LONG_MAX: libc::c_long = libc::c_long::MAX;
pub const SSIZE_MAX: libc::c_long = LONG_MAX;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const SEEK_END: libc::c_int = 2 as libc::c_int;
pub const MDB_NO_ROOT: libc::c_int = MDB_LAST_ERRCODE + 10 as libc::c_int;
pub const MDB_OWNERDEAD: libc::c_int = MDB_LAST_ERRCODE + 11 as libc::c_int;
unsafe extern "C" fn mdb_sem_wait(mut sem: mdb_mutexref_t) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut locked = (*sem).locked;
    let mut sb = {
        let mut init = sembuf {
            sem_num: 0 as libc::c_int as libc::c_ushort,
            sem_op: -(1 as libc::c_int) as libc::c_short,
            sem_flg: SEM_UNDO as libc::c_short,
        };
        init
    };
    sb.sem_num = (*sem).semnum as libc::c_ushort;
    loop {
        if semop((*sem).semid, &mut sb, 1 as libc::c_int as size_t) == 0 {
            rc = if *locked != 0 {
                MDB_OWNERDEAD
            } else {
                MDB_SUCCESS
            };
            *locked = 1 as libc::c_int;
            break;
        } else {
            rc = errno();
            if !(rc == EINTR) {
                break;
            }
        }
    }
    return rc;
}
pub const INVALID_HANDLE_VALUE: libc::c_int = -(1 as libc::c_int);
pub const MDB_MINKEYS: libc::c_int = 2 as libc::c_int;
pub const MDB_MAGIC: libc::c_uint = 0xbeefc0de as libc::c_uint;
pub const MDB_LOCK_VERSION_BITS: libc::c_int = 12 as libc::c_int;
pub const MAXDATASIZE: libc::c_ulong = 0xffffffff as libc::c_ulong;
pub const P_INVALID: pgno_t = !(0 as libc::c_int as pgno_t);
pub const DEFAULT_MAPSIZE: libc::c_int = 1048576 as libc::c_int;
pub const DEFAULT_READERS: libc::c_int = 126 as libc::c_int;
pub const P_BRANCH: libc::c_int = 0x1 as libc::c_int;
pub const P_LEAF: libc::c_int = 0x2 as libc::c_int;
pub const P_OVERFLOW: libc::c_int = 0x4 as libc::c_int;
pub const P_META: libc::c_int = 0x8 as libc::c_int;
pub const P_DIRTY: libc::c_int = 0x10 as libc::c_int;
pub const P_LEAF2: libc::c_int = 0x20 as libc::c_int;
pub const P_SUBP: libc::c_int = 0x40 as libc::c_int;
pub const P_LOOSE: libc::c_int = 0x4000 as libc::c_int;
pub const P_KEEP: libc::c_int = 0x8000 as libc::c_int;
pub const PAGEHDRSZ: libc::c_ulong = std::mem::offset_of!(MDB_page, mp_ptrs) as libc::c_ulong;
pub const FILL_THRESHOLD: libc::c_int = 250 as libc::c_int;
pub const F_BIGDATA: libc::c_int = 0x1 as libc::c_int;
pub const F_SUBDATA: libc::c_int = 0x2 as libc::c_int;
pub const F_DUPDATA: libc::c_int = 0x4 as libc::c_int;
pub const NODE_ADD_FLAGS: libc::c_int = F_DUPDATA | F_SUBDATA | MDB_RESERVE | MDB_APPEND;
pub const NODESIZE: libc::c_ulong = mem::offset_of!(MDB_node, mn_data) as libc::c_ulong;
pub const MDB_VALID: libc::c_int = 0x8000 as libc::c_int;
pub const PERSISTENT_FLAGS: libc::c_int = 0xffff as libc::c_int & !(0x8000 as libc::c_int);
pub const VALID_FLAGS: libc::c_int = MDB_REVERSEKEY
    | MDB_DUPSORT
    | MDB_INTEGERKEY
    | MDB_DUPFIXED
    | MDB_INTEGERDUP
    | MDB_REVERSEDUP
    | MDB_CREATE;
pub const FREE_DBI: libc::c_int = 0 as libc::c_int;
pub const MAIN_DBI: libc::c_int = 1 as libc::c_int;
pub const CORE_DBS: libc::c_int = 2 as libc::c_int;
pub const NUM_METAS: libc::c_int = 2 as libc::c_int;
pub const DB_DIRTY: libc::c_int = 0x1 as libc::c_int;
pub const DB_STALE: libc::c_int = 0x2 as libc::c_int;
pub const DB_NEW: libc::c_int = 0x4 as libc::c_int;
pub const DB_VALID: libc::c_int = 0x8 as libc::c_int;
pub const DB_USRVALID: libc::c_int = 0x10 as libc::c_int;
pub const DB_DUPDATA: libc::c_int = 0x20 as libc::c_int;
pub const MDB_TXN_BEGIN_FLAGS: libc::c_int = MDB_NOMETASYNC | MDB_NOSYNC | MDB_RDONLY;
pub const MDB_TXN_RDONLY: libc::c_int = 0x20000 as libc::c_int;
pub const MDB_TXN_WRITEMAP: libc::c_int = MDB_WRITEMAP;
pub const MDB_TXN_FINISHED: libc::c_int = 0x1 as libc::c_int;
pub const MDB_TXN_ERROR: libc::c_int = 0x2 as libc::c_int;
pub const MDB_TXN_DIRTY: libc::c_int = 0x4 as libc::c_int;
pub const MDB_TXN_SPILLS: libc::c_int = 0x8 as libc::c_int;
pub const MDB_TXN_HAS_CHILD: libc::c_int = 0x10 as libc::c_int;
pub const MDB_TXN_BLOCKED: libc::c_int = MDB_TXN_FINISHED | MDB_TXN_ERROR | MDB_TXN_HAS_CHILD;
pub const CURSOR_STACK: libc::c_int = 32 as libc::c_int;
pub const C_INITIALIZED: libc::c_int = 0x1 as libc::c_int;
pub const C_EOF: libc::c_int = 0x2 as libc::c_int;
pub const C_SUB: libc::c_int = 0x4 as libc::c_int;
pub const C_DEL: libc::c_int = 0x8 as libc::c_int;
pub const C_UNTRACK: libc::c_int = 0x40 as libc::c_int;
pub const C_WRITEMAP: libc::c_int = MDB_TXN_WRITEMAP;
pub const C_ORIG_RDONLY: libc::c_int = MDB_TXN_RDONLY;
pub const MDB_FATAL_ERROR: libc::c_uint = 0x80000000 as libc::c_uint;
pub const MDB_ENV_ACTIVE: libc::c_uint = 0x20000000 as libc::c_uint;
pub const MDB_ENV_TXKEY: libc::c_uint = 0x10000000 as libc::c_uint;
pub const MDB_COMMIT_PAGES: libc::c_int = 64 as libc::c_int;
pub const MAX_WRITE: libc::c_uint = 0x40000000 as libc::c_uint
    >> (::core::mem::size_of::<ssize_t>() as libc::c_ulong == 4 as libc::c_int as libc::c_ulong)
        as libc::c_int;
pub const MDB_END_UPDATE: libc::c_int = 0x10 as libc::c_int;
pub const MDB_END_FREE: libc::c_int = 0x20 as libc::c_int;
pub const MDB_END_SLOT: libc::c_int = 0x200000 as libc::c_int;
pub const MDB_PS_MODIFY: libc::c_int = 1 as libc::c_int;
pub const MDB_PS_ROOTONLY: libc::c_int = 2 as libc::c_int;
pub const MDB_PS_FIRST: libc::c_int = 4 as libc::c_int;
pub const MDB_PS_LAST: libc::c_int = 8 as libc::c_int;
pub const MDB_SPLIT_REPLACE: libc::c_int = 0x40000 as libc::c_int;
pub const mdb_cmp_clong: unsafe extern "C" fn(*const MDB_val, *const MDB_val) -> libc::c_int =
    mdb_cmp_cint;
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_version(
    mut major: *mut libc::c_int,
    mut minor: *mut libc::c_int,
    mut patch: *mut libc::c_int,
) -> *const libc::c_char {
    if !major.is_null() {
        *major = MDB_VERSION_MAJOR;
    }
    if !minor.is_null() {
        *minor = MDB_VERSION_MINOR;
    }
    if !patch.is_null() {
        *patch = MDB_VERSION_PATCH;
    }
    return MDB_VERSION_STRING.as_ptr() as _;
}
static mut mdb_errstr: [*mut libc::c_char; 21] = [
    b"MDB_KEYEXIST: Key/data pair already exists\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_NOTFOUND: No matching key/data pair found\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_PAGE_NOTFOUND: Requested page not found\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_CORRUPTED: Located page was wrong type\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_PANIC: Update of meta page failed or environment had fatal error\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_VERSION_MISMATCH: Database environment version mismatch\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_INVALID: File is not an LMDB file\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_MAP_FULL: Environment mapsize limit reached\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_DBS_FULL: Environment maxdbs limit reached\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_READERS_FULL: Environment maxreaders limit reached\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_TLS_FULL: Thread-local storage keys full - too many environments open\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_TXN_FULL: Transaction has too many dirty pages - transaction too big\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_CURSOR_FULL: Internal error - cursor stack limit reached\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_PAGE_FULL: Internal error - page has no more space\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_MAP_RESIZED: Database contents grew beyond environment mapsize\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_INCOMPATIBLE: Operation and DB incompatible, or DB flags changed\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_BAD_RSLOT: Invalid reuse of reader locktable slot\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_BAD_TXN: Transaction must abort, has a child, or is invalid\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_BAD_VALSIZE: Unsupported size of key/DB name/data, or wrong DUPFIXED size\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_BAD_DBI: The specified DBI handle was closed/changed unexpectedly\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_PROBLEM: Unexpected problem - txn should abort\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn mdb_strerror(mut err: libc::c_int) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    if err == 0 {
        return b"Successful return: 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if err >= MDB_KEYEXIST && err <= MDB_LAST_ERRCODE {
        i = err - MDB_KEYEXIST;
        return mdb_errstr[i as usize];
    }
    if err < 0 as libc::c_int {
        return b"Invalid error code\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return strerror(err);
}
#[cold]
unsafe extern "C" fn mdb_assert_fail(
    mut env: *mut MDB_env,
    mut expr_txt: *const libc::c_char,
    mut func: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) {
    let mut buf: [libc::c_char; 400] = [0; 400];
    sprintf(
        buf.as_mut_ptr(),
        b"%.100s:%d: Assertion '%.200s' failed in %.40s()\0" as *const u8 as *const libc::c_char,
        file,
        line,
        expr_txt,
        func,
    );
    if ((*env).me_assert_func).is_some() {
        ((*env).me_assert_func).expect("non-null function pointer")(env, buf.as_mut_ptr());
    }
    fprintf(
        __stderrp,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
    );
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn mdb_cmp(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    return ((*((*txn).mt_dbxs).offset(dbi as isize)).md_cmp).expect("non-null function pointer")(
        a, b,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mdb_dcmp(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    let mut dcmp: Option<MDB_cmp_func> = (*((*txn).mt_dbxs).offset(dbi as isize)).md_dcmp;
    if (UINT_MAX as libc::c_ulong) < MDB_SIZE_MAX
        && dcmp == Some(mdb_cmp_int as MDB_cmp_func)
        && (*a).mv_size == ::core::mem::size_of::<mdb_size_t>() as libc::c_ulong
    {
        dcmp = Some(mdb_cmp_clong);
    }
    return dcmp.expect("non-null function pointer")(a, b);
}
unsafe extern "C" fn mdb_page_malloc(
    mut txn: *mut MDB_txn,
    mut num: libc::c_uint,
) -> *mut MDB_page {
    let mut env = (*txn).mt_env;
    let mut ret = (*env).me_dpages;
    let mut psize = (*env).me_psize as size_t;
    let mut sz = psize;
    let mut off: size_t = 0;
    if num == 1 as libc::c_int as libc::c_uint {
        if !ret.is_null() {
            (*env).me_dpages = (*ret).mp_p.p_next;
            return ret;
        }
        off = PAGEHDRSZ;
        psize = psize.wrapping_sub(off);
    } else {
        sz = sz * num as size_t;
        off = sz.wrapping_sub(psize);
    }
    ret = malloc(sz) as *mut MDB_page;
    if !ret.is_null() {
        if (*env).me_flags & MDB_NOMEMINIT as uint32_t == 0 {
            memset(
                (ret as *mut libc::c_char).offset(off as isize) as *mut libc::c_void,
                0 as libc::c_int,
                psize,
            );
            (*ret).mp_pad = 0 as libc::c_int as uint16_t;
        }
    } else {
        (*txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
    }
    return ret;
}
unsafe extern "C" fn mdb_page_free(mut env: *mut MDB_env, mut mp: *mut MDB_page) {
    (*mp).mp_p.p_next = (*env).me_dpages;
    (*env).me_dpages = mp;
}
unsafe extern "C" fn mdb_dpage_free(mut env: *mut MDB_env, mut dp: *mut MDB_page) {
    if !((*(dp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
        & 0x4 as libc::c_int
        == 0x4 as libc::c_int)
        || (*dp).mp_pb.pb_pages == 1 as libc::c_int as uint32_t
    {
        mdb_page_free(env, dp);
    } else {
        free(dp as *mut libc::c_void);
    };
}
unsafe extern "C" fn mdb_dlist_free(mut txn: *mut MDB_txn) {
    let mut env = (*txn).mt_env;
    let mut dl = (*txn).mt_u.dirty_list;
    let mut i: libc::c_uint = 0;
    let mut n = (*dl.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
    i = 1 as libc::c_int as libc::c_uint;
    while i <= n {
        mdb_dpage_free(env, (*dl.offset(i as isize)).mptr as *mut MDB_page);
        i = i.wrapping_add(1);
        i;
    }
    (*dl.offset(0 as libc::c_int as isize)).mid = 0 as libc::c_int as MDB_ID;
}
unsafe extern "C" fn mdb_page_loose(mut mc: *mut MDB_cursor, mut mp: *mut MDB_page) -> libc::c_int {
    let mut loose = 0 as libc::c_int;
    let mut pgno = (*mp).mp_p.p_pgno;
    let mut txn = (*mc).mc_txn;
    if (*mp).mp_flags as libc::c_int & P_DIRTY != 0 && (*mc).mc_dbi != FREE_DBI as MDB_dbi {
        if !((*txn).mt_parent).is_null() {
            let mut dl = (*txn).mt_u.dirty_list;
            if (*dl.offset(0 as libc::c_int as isize)).mid != 0 {
                let mut x = mdb_mid2l_search(dl, pgno);
                if x as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid
                    && (*dl.offset(x as isize)).mid == pgno
                {
                    if mp != (*dl.offset(x as isize)).mptr as *mut MDB_page {
                        (*mc).mc_flags &= !(C_INITIALIZED | C_EOF) as libc::c_uint;
                        (*txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
                        return MDB_PROBLEM;
                    }
                    loose = 1 as libc::c_int;
                }
            }
        } else {
            loose = 1 as libc::c_int;
        }
    }
    if loose != 0 {
        let ref mut fresh0 = *(mp.offset(2 as libc::c_int as isize) as *mut *mut MDB_page);
        *fresh0 = (*txn).mt_loose_pgs;
        (*txn).mt_loose_pgs = mp;
        (*txn).mt_loose_count += 1;
        (*txn).mt_loose_count;
        (*mp).mp_flags = ((*mp).mp_flags as libc::c_int | P_LOOSE) as uint16_t;
    } else {
        let mut rc = mdb_midl_append(&mut (*txn).mt_free_pgs, pgno);
        if rc != 0 {
            return rc;
        }
    }
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_pages_xkeep(
    mut mc: *mut MDB_cursor,
    mut pflags: libc::c_uint,
    mut all: libc::c_int,
) -> libc::c_int {
    let mut txn = (*mc).mc_txn;
    let mut m3 = 0 as *mut MDB_cursor;
    let mut m0 = mc;
    let mut mx = 0 as *mut MDB_xcursor;
    let mut dp = 0 as *mut MDB_page;
    let mut mp = 0 as *mut MDB_page;
    let mut leaf = 0 as *mut MDB_node;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut rc = MDB_SUCCESS;
    let mut level: libc::c_int = 0;
    i = (*txn).mt_numdbs;
    's_25: loop {
        if (*mc).mc_flags & C_INITIALIZED as libc::c_uint != 0 {
            m3 = mc;
            loop {
                mp = NULL as *mut MDB_page;
                j = 0 as libc::c_int as libc::c_uint;
                while j < (*m3).mc_snum as libc::c_uint {
                    mp = (*m3).mc_pg[j as usize];
                    if ((*mp).mp_flags as libc::c_int & Mask as libc::c_int) as libc::c_uint
                        == pflags
                    {
                        (*mp).mp_flags = ((*mp).mp_flags as libc::c_int ^ P_KEEP) as uint16_t;
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                mx = (*m3).mc_xcursor;
                if !(!mx.is_null() && (*mx).mx_cursor.mc_flags & C_INITIALIZED as libc::c_uint != 0)
                {
                    break;
                }
                if !(!mp.is_null() && (*mp).mp_flags as libc::c_int & P_LEAF != 0) {
                    break;
                }
                leaf = (mp as *mut libc::c_char)
                    .offset(
                        *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                            .as_mut_ptr()
                            .offset(
                                (*m3).mc_ki
                                    [j.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                                    as isize,
                            ) as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                if (*leaf).mn_flags as libc::c_int & F_SUBDATA == 0 {
                    break;
                }
                m3 = &mut (*mx).mx_cursor;
            }
        }
        mc = (*mc).mc_next;
        while mc.is_null() || mc == m0 {
            if i == 0 as libc::c_int as libc::c_uint {
                break 's_25;
            }
            i = i.wrapping_sub(1);
            mc = *((*txn).mt_cursors).offset(i as isize);
        }
    }
    if all != 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*txn).mt_numdbs {
            if *((*txn).mt_dbflags).offset(i as isize) as libc::c_int & DB_DIRTY != 0 {
                let mut pgno = (*((*txn).mt_dbs).offset(i as isize)).md_root;
                if !(pgno == P_INVALID) {
                    rc = mdb_page_get(m0, pgno, &mut dp, &mut level);
                    if rc != MDB_SUCCESS {
                        break;
                    }
                    if ((*dp).mp_flags as libc::c_int & Mask as libc::c_int) as libc::c_uint
                        == pflags
                        && level <= 1 as libc::c_int
                    {
                        (*dp).mp_flags = ((*dp).mp_flags as libc::c_int ^ P_KEEP) as uint16_t;
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return rc;
}
unsafe extern "C" fn mdb_page_spill(
    mut m0: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut current_block: u64;
    let mut txn = (*m0).mc_txn;
    let mut dp = 0 as *mut MDB_page;
    let mut dl = (*txn).mt_u.dirty_list;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut need: libc::c_uint = 0;
    let mut rc: libc::c_int = 0;
    if (*m0).mc_flags & C_SUB as libc::c_uint != 0 {
        return MDB_SUCCESS;
    }
    i = (*(*m0).mc_db).md_depth as libc::c_uint;
    if (*m0).mc_dbi >= CORE_DBS as MDB_dbi {
        i = i.wrapping_add((*((*txn).mt_dbs).offset(MAIN_DBI as isize)).md_depth as libc::c_uint);
    }
    if !key.is_null() {
        i = (i as libc::c_ulong).wrapping_add(
            NODESIZE
                .wrapping_add((*key).mv_size)
                .wrapping_add((*data).mv_size)
                .wrapping_add((*(*txn).mt_env).me_psize as libc::c_ulong)
                .wrapping_div((*(*txn).mt_env).me_psize as libc::c_ulong),
        ) as libc::c_uint as libc::c_uint;
    }
    i = i.wrapping_add(i);
    need = i;
    if (*txn).mt_dirty_room > i {
        return MDB_SUCCESS;
    }
    if ((*txn).mt_spill_pgs).is_null() {
        (*txn).mt_spill_pgs = mdb_midl_alloc(MDB_IDL_UM_MAX);
        if ((*txn).mt_spill_pgs).is_null() {
            return ENOMEM;
        }
    } else {
        let mut sl = (*txn).mt_spill_pgs;
        let mut num = *sl.offset(0 as libc::c_int as isize) as libc::c_uint;
        j = 0 as libc::c_int as libc::c_uint;
        i = 1 as libc::c_int as libc::c_uint;
        while i <= num {
            if *sl.offset(i as isize) & 1 as libc::c_int as MDB_ID == 0 {
                j = j.wrapping_add(1);
                *sl.offset(j as isize) = *sl.offset(i as isize);
            }
            i = i.wrapping_add(1);
            i;
        }
        *sl.offset(0 as libc::c_int as isize) = j as MDB_ID;
    }
    rc = mdb_pages_xkeep(m0, P_DIRTY as libc::c_uint, 1 as libc::c_int);
    if !(rc != MDB_SUCCESS) {
        if need < (MDB_IDL_UM_MAX / 8 as libc::c_int) as libc::c_uint {
            need = (MDB_IDL_UM_MAX / 8 as libc::c_int) as libc::c_uint;
        }
        i = (*dl.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
        loop {
            if !(i != 0 && need != 0) {
                current_block = 980989089337379490;
                break;
            }
            let mut pn = (*dl.offset(i as isize)).mid << 1 as libc::c_int;
            dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
            if !((*dp).mp_flags as libc::c_int & (P_LOOSE | P_KEEP) != 0) {
                if !((*txn).mt_parent).is_null() {
                    let mut tx2 = 0 as *mut MDB_txn;
                    tx2 = (*txn).mt_parent;
                    while !tx2.is_null() {
                        if !((*tx2).mt_spill_pgs).is_null() {
                            j = mdb_midl_search((*tx2).mt_spill_pgs, pn);
                            if j as MDB_ID
                                <= *((*tx2).mt_spill_pgs).offset(0 as libc::c_int as isize)
                                && *((*tx2).mt_spill_pgs).offset(j as isize) == pn
                            {
                                (*dp).mp_flags =
                                    ((*dp).mp_flags as libc::c_int | P_KEEP) as uint16_t;
                                break;
                            }
                        }
                        tx2 = (*tx2).mt_parent;
                    }
                    if !tx2.is_null() {
                        current_block = 13472856163611868459;
                    } else {
                        current_block = 8180496224585318153;
                    }
                } else {
                    current_block = 8180496224585318153;
                }
                match current_block {
                    13472856163611868459 => {}
                    _ => {
                        rc = mdb_midl_append(&mut (*txn).mt_spill_pgs, pn);
                        if rc != 0 {
                            current_block = 839901895064581446;
                            break;
                        }
                        need = need.wrapping_sub(1);
                        need;
                    }
                }
            }
            i = i.wrapping_sub(1);
            i;
        }
        match current_block {
            839901895064581446 => {}
            _ => {
                mdb_midl_sort((*txn).mt_spill_pgs);
                rc = mdb_page_flush(txn, i as libc::c_int);
                if !(rc != MDB_SUCCESS) {
                    rc = mdb_pages_xkeep(m0, (P_DIRTY | P_KEEP) as libc::c_uint, i as libc::c_int);
                }
            }
        }
    }
    (*txn).mt_flags |= (if rc != 0 {
        MDB_TXN_ERROR
    } else {
        MDB_TXN_SPILLS
    }) as libc::c_uint;
    return rc;
}
unsafe extern "C" fn mdb_find_oldest(mut txn: *mut MDB_txn) -> txnid_t {
    let mut i: libc::c_int = 0;
    let mut mr: txnid_t = 0;
    let mut oldest = ((*txn).mt_txnid).wrapping_sub(1 as libc::c_int as txnid_t);
    if !((*(*txn).mt_env).me_txns).is_null() {
        let mut r = ((*(*(*txn).mt_env).me_txns).mti_readers).as_mut_ptr();
        i = (*(*(*txn).mt_env).me_txns).mt1.mtb.mtb_numreaders as libc::c_int;
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            if (*r.offset(i as isize)).mru.mrx.mrb_pid != 0 {
                mr = (*r.offset(i as isize)).mru.mrx.mrb_txnid;
                if oldest > mr {
                    oldest = mr;
                }
            }
        }
    }
    return oldest;
}
unsafe extern "C" fn mdb_page_dirty(mut txn: *mut MDB_txn, mut mp: *mut MDB_page) {
    let mut mid = MDB_ID2 {
        mid: 0,
        mptr: 0 as *mut libc::c_void,
    };
    let mut rc: libc::c_int = 0;
    let mut insert: Option<unsafe extern "C" fn(MDB_ID2L, *mut MDB_ID2) -> libc::c_int> = None;
    if (*txn).mt_flags & MDB_TXN_WRITEMAP as libc::c_uint != 0 {
        insert =
            Some(mdb_mid2l_append as unsafe extern "C" fn(MDB_ID2L, *mut MDB_ID2) -> libc::c_int);
    } else {
        insert =
            Some(mdb_mid2l_insert as unsafe extern "C" fn(MDB_ID2L, *mut MDB_ID2) -> libc::c_int);
    }
    mid.mid = (*mp).mp_p.p_pgno;
    mid.mptr = mp as *mut libc::c_void;
    rc = insert.expect("non-null function pointer")((*txn).mt_u.dirty_list, &mut mid);
    if rc == 0 as libc::c_int {
    } else {
        mdb_assert_fail(
            (*txn).mt_env,
            b"rc == 0\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"mdb_page_dirty\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            2467 as libc::c_int,
        );
    };
    (*txn).mt_dirty_room = ((*txn).mt_dirty_room).wrapping_sub(1);
    (*txn).mt_dirty_room;
}
unsafe extern "C" fn mdb_page_alloc(
    mut mc: *mut MDB_cursor,
    mut num: libc::c_int,
    mut mp: *mut *mut MDB_page,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut retry = num * 60 as libc::c_int;
    let mut txn = (*mc).mc_txn;
    let mut env = (*txn).mt_env;
    let mut pgno: pgno_t = 0;
    let mut mop = (*env).me_pgstate.mf_pghead;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut mop_len = (if !mop.is_null() {
        *mop.offset(0 as libc::c_int as isize)
    } else {
        0 as libc::c_int as pgno_t
    }) as libc::c_uint;
    let mut n2 = (num - 1 as libc::c_int) as libc::c_uint;
    let mut np = 0 as *mut MDB_page;
    let mut oldest = 0 as libc::c_int as txnid_t;
    let mut last: txnid_t = 0;
    let mut op = MDB_FIRST;
    let mut m2 = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut found_old = 0 as libc::c_int;
    if num == 1 as libc::c_int && !((*txn).mt_loose_pgs).is_null() {
        np = (*txn).mt_loose_pgs;
        (*txn).mt_loose_pgs = *(np.offset(2 as libc::c_int as isize) as *mut *mut MDB_page);
        (*txn).mt_loose_count -= 1;
        (*txn).mt_loose_count;
        *mp = np;
        return MDB_SUCCESS;
    }
    *mp = NULL as *mut MDB_page;
    if (*txn).mt_dirty_room == 0 as libc::c_int as libc::c_uint {
        rc = MDB_TXN_FULL;
    } else {
        op = MDB_FIRST;
        's_69: loop {
            let mut key = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            let mut data = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            let mut leaf = 0 as *mut MDB_node;
            let mut idl = 0 as *mut pgno_t;
            if mop_len > n2 {
                i = mop_len;
                loop {
                    pgno = *mop.offset(i as isize);
                    if *mop.offset(i.wrapping_sub(n2) as isize) == pgno.wrapping_add(n2 as pgno_t) {
                        current_block = 5537592996104297960;
                        break 's_69;
                    }
                    i = i.wrapping_sub(1);
                    if !(i > n2) {
                        break;
                    }
                }
                retry -= 1;
                if retry < 0 as libc::c_int {
                    current_block = 6721012065216013753;
                    break;
                }
            }
            if op as libc::c_uint == MDB_FIRST as libc::c_int as libc::c_uint {
                last = (*env).me_pgstate.mf_pglast;
                oldest = (*env).me_pgoldest;
                mdb_cursor_init(&mut m2, txn, FREE_DBI as MDB_dbi, NULL as *mut MDB_xcursor);
                if last != 0 {
                    op = MDB_SET_RANGE;
                    key.mv_data = &mut last as *mut txnid_t as *mut libc::c_void;
                    key.mv_size = ::core::mem::size_of::<txnid_t>() as libc::c_ulong;
                }
                if Paranoid as libc::c_int != 0 && (*mc).mc_dbi == FREE_DBI as MDB_dbi {
                    retry = -(1 as libc::c_int);
                }
            }
            if Paranoid as libc::c_int != 0 && retry < 0 as libc::c_int && mop_len != 0 {
                current_block = 6721012065216013753;
                break;
            }
            last = last.wrapping_add(1);
            last;
            if oldest <= last {
                if found_old == 0 {
                    oldest = mdb_find_oldest(txn);
                    (*env).me_pgoldest = oldest;
                    found_old = 1 as libc::c_int;
                }
                if oldest <= last {
                    current_block = 6721012065216013753;
                    break;
                }
            }
            rc = mdb_cursor_get(&mut m2, &mut key, NULL as *mut MDB_val, op);
            if rc != 0 {
                if rc == MDB_NOTFOUND {
                    current_block = 6721012065216013753;
                    break;
                } else {
                    current_block = 815053351550855110;
                    break;
                }
            } else {
                last = *(key.mv_data as *mut txnid_t);
                if oldest <= last {
                    if found_old == 0 {
                        oldest = mdb_find_oldest(txn);
                        (*env).me_pgoldest = oldest;
                        found_old = 1 as libc::c_int;
                    }
                    if oldest <= last {
                        current_block = 6721012065216013753;
                        break;
                    }
                }
                np = m2.mc_pg[m2.mc_top as usize];
                leaf = (np as *mut libc::c_char)
                    .offset(
                        *((*(np as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                            .as_mut_ptr()
                            .offset(m2.mc_ki[m2.mc_top as usize] as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                rc = mdb_node_read(&mut m2, leaf, &mut data);
                if rc != MDB_SUCCESS {
                    current_block = 815053351550855110;
                    break;
                }
                idl = data.mv_data as *mut MDB_ID;
                i = *idl.offset(0 as libc::c_int as isize) as libc::c_uint;
                if mop.is_null() {
                    mop = mdb_midl_alloc(i as libc::c_int);
                    (*env).me_pgstate.mf_pghead = mop;
                    if ((*env).me_pgstate.mf_pghead).is_null() {
                        rc = ENOMEM;
                        current_block = 815053351550855110;
                        break;
                    }
                } else {
                    rc = mdb_midl_need(&mut (*env).me_pgstate.mf_pghead, i);
                    if rc != 0 as libc::c_int {
                        current_block = 815053351550855110;
                        break;
                    }
                    mop = (*env).me_pgstate.mf_pghead;
                }
                (*env).me_pgstate.mf_pglast = last;
                mdb_midl_xmerge(mop, idl);
                mop_len = *mop.offset(0 as libc::c_int as isize) as libc::c_uint;
                op = MDB_NEXT;
            }
        }
        match current_block {
            815053351550855110 => {}
            _ => {
                match current_block {
                    6721012065216013753 => {
                        i = 0 as libc::c_int as libc::c_uint;
                        pgno = (*txn).mt_next_pgno;
                        if pgno.wrapping_add(num as pgno_t) >= (*env).me_maxpg {
                            rc = MDB_MAP_FULL;
                            current_block = 815053351550855110;
                        } else {
                            current_block = 5537592996104297960;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    815053351550855110 => {}
                    _ => {
                        if (*env).me_flags & MDB_WRITEMAP as uint32_t != 0 {
                            np = ((*env).me_map).offset(((*env).me_psize as pgno_t * pgno) as isize)
                                as *mut MDB_page;
                            current_block = 16779030619667747692;
                        } else {
                            np = mdb_page_malloc(txn, num as libc::c_uint);
                            if np.is_null() {
                                rc = ENOMEM;
                                current_block = 815053351550855110;
                            } else {
                                current_block = 16779030619667747692;
                            }
                        }
                        match current_block {
                            815053351550855110 => {}
                            _ => {
                                if i != 0 {
                                    mop_len = mop_len.wrapping_sub(num as libc::c_uint);
                                    *mop.offset(0 as libc::c_int as isize) = mop_len as pgno_t;
                                    j = i.wrapping_sub(num as libc::c_uint);
                                    while j < mop_len {
                                        i = i.wrapping_add(1);
                                        j = j.wrapping_add(1);
                                        *mop.offset(j as isize) = *mop.offset(i as isize);
                                    }
                                } else {
                                    (*txn).mt_next_pgno = pgno.wrapping_add(num as pgno_t);
                                }
                                (*np).mp_p.p_pgno = pgno;
                                mdb_page_dirty(txn, np);
                                *mp = np;
                                return MDB_SUCCESS;
                            }
                        }
                    }
                }
            }
        }
    }
    (*txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
    return rc;
}
unsafe extern "C" fn mdb_page_copy(
    mut dst: *mut MDB_page,
    mut src: *mut MDB_page,
    mut psize: libc::c_uint,
) {
    let mut upper = (*src).mp_pb.pb.pb_upper;
    let mut lower = (*src).mp_pb.pb.pb_lower;
    let mut unused = (upper as libc::c_int - lower as libc::c_int) as indx_t;
    unused = (unused as libc::c_int & -(Align as libc::c_int)) as indx_t;
    if unused as libc::c_int != 0
        && !((*(src as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
            & 0x20 as libc::c_int
            == 0x20 as libc::c_int)
    {
        upper = ((upper as libc::c_uint).wrapping_add(if 0 as libc::c_int != 0 {
            PAGEHDRSZ as libc::c_uint
        } else {
            0 as libc::c_int as libc::c_uint
        }) & -(Align as libc::c_int) as libc::c_uint) as indx_t;
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            ((lower as libc::c_uint)
                .wrapping_add(if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                })
                .wrapping_add((Align as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                & -(Align as libc::c_int) as libc::c_uint) as libc::c_ulong,
        );
        memcpy(
            (dst as *mut libc::c_char).offset(upper as libc::c_int as isize) as *mut pgno_t
                as *mut libc::c_void,
            (src as *mut libc::c_char).offset(upper as libc::c_int as isize) as *mut pgno_t
                as *const libc::c_void,
            psize.wrapping_sub(upper as libc::c_uint) as libc::c_ulong,
        );
    } else {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            psize.wrapping_sub(unused as libc::c_uint) as libc::c_ulong,
        );
    };
}
unsafe extern "C" fn mdb_page_unspill(
    mut txn: *mut MDB_txn,
    mut mp: *mut MDB_page,
    mut ret: *mut *mut MDB_page,
) -> libc::c_int {
    let mut env = (*txn).mt_env;
    let mut tx2 = 0 as *const MDB_txn;
    let mut x: libc::c_uint = 0;
    let mut pgno = (*mp).mp_p.p_pgno;
    let mut pn = pgno << 1 as libc::c_int;
    tx2 = txn;
    while !tx2.is_null() {
        if !((*tx2).mt_spill_pgs).is_null() {
            x = mdb_midl_search((*tx2).mt_spill_pgs, pn);
            if x as MDB_ID <= *((*tx2).mt_spill_pgs).offset(0 as libc::c_int as isize)
                && *((*tx2).mt_spill_pgs).offset(x as isize) == pn
            {
                let mut np = 0 as *mut MDB_page;
                let mut num: libc::c_int = 0;
                if (*txn).mt_dirty_room == 0 as libc::c_int as libc::c_uint {
                    return MDB_TXN_FULL;
                }
                if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                    & 0x4 as libc::c_int
                    == 0x4 as libc::c_int
                {
                    num = (*mp).mp_pb.pb_pages as libc::c_int;
                } else {
                    num = 1 as libc::c_int;
                }
                if (*env).me_flags & MDB_WRITEMAP as uint32_t != 0 {
                    np = mp;
                } else {
                    np = mdb_page_malloc(txn, num as libc::c_uint);
                    if np.is_null() {
                        return ENOMEM;
                    }
                    if num > 1 as libc::c_int {
                        memcpy(
                            np as *mut libc::c_void,
                            mp as *const libc::c_void,
                            (num as libc::c_uint).wrapping_mul((*env).me_psize) as libc::c_ulong,
                        );
                    } else {
                        mdb_page_copy(np, mp, (*env).me_psize);
                    }
                }
                if tx2 == txn as *const MDB_txn {
                    if x as MDB_ID == *((*txn).mt_spill_pgs).offset(0 as libc::c_int as isize) {
                        let ref mut fresh1 =
                            *((*txn).mt_spill_pgs).offset(0 as libc::c_int as isize);
                        *fresh1 = (*fresh1).wrapping_sub(1);
                        let _ = *fresh1;
                    } else {
                        *((*txn).mt_spill_pgs).offset(x as isize) |= 1 as libc::c_int as MDB_ID;
                    }
                }
                mdb_page_dirty(txn, np);
                (*np).mp_flags = ((*np).mp_flags as libc::c_int | P_DIRTY) as uint16_t;
                *ret = np;
                break;
            }
        }
        tx2 = (*tx2).mt_parent;
    }
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_page_touch(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut current_block: u64;
    let mut mp = (*mc).mc_pg[(*mc).mc_top as usize];
    let mut np = 0 as *mut MDB_page;
    let mut txn = (*mc).mc_txn;
    let mut m2 = 0 as *mut MDB_cursor;
    let mut m3 = 0 as *mut MDB_cursor;
    let mut pgno: pgno_t = 0;
    let mut rc: libc::c_int = 0;
    if !((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
        & 0x10 as libc::c_int
        == 0x10 as libc::c_int)
    {
        if (*txn).mt_flags & MDB_TXN_SPILLS as libc::c_uint != 0 {
            np = NULL as *mut MDB_page;
            rc = mdb_page_unspill(txn, mp, &mut np);
            if rc != 0 {
                current_block = 7813835194233133412;
            } else if !np.is_null() {
                current_block = 5681221687841258195;
            } else {
                current_block = 11006700562992250127;
            }
        } else {
            current_block = 11006700562992250127;
        }
        match current_block {
            5681221687841258195 => {}
            _ => {
                match current_block {
                    11006700562992250127 => {
                        rc = mdb_midl_need(
                            &mut (*txn).mt_free_pgs,
                            1 as libc::c_int as libc::c_uint,
                        );
                        if rc != 0 || {
                            rc = mdb_page_alloc(mc, 1 as libc::c_int, &mut np);
                            rc != 0
                        } {
                            current_block = 7813835194233133412;
                        } else {
                            pgno = (*np).mp_p.p_pgno;
                            if (*mp).mp_p.p_pgno != pgno {
                            } else {
                                mdb_assert_fail(
                                    (*(*mc).mc_txn).mt_env,
                                    b"mp->mp_pgno != pgno\0" as *const u8 as *const libc::c_char,
                                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                                        b"mdb_page_touch\0",
                                    ))
                                    .as_ptr(),
                                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                                    2796 as libc::c_int,
                                );
                            };
                            let mut xidl = (*txn).mt_free_pgs;
                            let ref mut fresh2 = *xidl.offset(0 as libc::c_int as isize);
                            *fresh2 = (*fresh2).wrapping_add(1);
                            let mut xlen = *fresh2;
                            *xidl.offset(xlen as isize) = (*mp).mp_p.p_pgno;
                            if (*mc).mc_top != 0 {
                                let mut parent = (*mc).mc_pg
                                    [((*mc).mc_top as libc::c_int - 1 as libc::c_int) as usize];
                                let mut node = (parent as *mut libc::c_char)
                                    .offset(
                                        *((*(parent as *mut libc::c_void as *mut MDB_page2))
                                            .mp2_ptrs)
                                            .as_mut_ptr()
                                            .offset(
                                                (*mc).mc_ki[((*mc).mc_top as libc::c_int
                                                    - 1 as libc::c_int)
                                                    as usize]
                                                    as isize,
                                            ) as libc::c_int
                                            as isize,
                                    )
                                    .offset(
                                        (if 0 as libc::c_int != 0 {
                                            PAGEHDRSZ as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        }) as isize,
                                    )
                                    as *mut MDB_node;
                                (*node).mn_lo =
                                    (pgno & 0xffff as libc::c_int as pgno_t) as libc::c_ushort;
                                (*node).mn_hi = (pgno >> 16 as libc::c_int) as libc::c_ushort;
                                if if -(1 as libc::c_int) as pgno_t
                                    > 0xffffffff as libc::c_uint as pgno_t
                                {
                                    32 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                } != 0
                                {
                                    (*node).mn_flags = (pgno
                                        >> (if -(1 as libc::c_int) as pgno_t
                                            > 0xffffffff as libc::c_uint as pgno_t
                                        {
                                            32 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        }))
                                        as libc::c_ushort;
                                }
                            } else {
                                (*(*mc).mc_db).md_root = pgno;
                            }
                            current_block = 13131896068329595644;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    13131896068329595644 => {}
                    _ => {
                        (*txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
                        return rc;
                    }
                }
            }
        }
    } else {
        if !((*txn).mt_parent).is_null()
            && !((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                & 0x40 as libc::c_int
                == 0x40 as libc::c_int)
        {
            let mut mid = MDB_ID2 {
                mid: 0,
                mptr: 0 as *mut libc::c_void,
            };
            let mut dl = (*txn).mt_u.dirty_list;
            pgno = (*mp).mp_p.p_pgno;
            if (*dl.offset(0 as libc::c_int as isize)).mid != 0 {
                let mut x = mdb_mid2l_search(dl, pgno);
                if x as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid
                    && (*dl.offset(x as isize)).mid == pgno
                {
                    if mp != (*dl.offset(x as isize)).mptr as *mut MDB_page {
                        (*mc).mc_flags &= !(C_INITIALIZED | C_EOF) as libc::c_uint;
                        (*txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
                        return MDB_PROBLEM;
                    }
                    return 0 as libc::c_int;
                }
            }
            if (*dl.offset(0 as libc::c_int as isize)).mid
                < (((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int) - 1 as libc::c_int)
                    as MDB_ID
            {
            } else {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"dl[0].mid < MDB_IDL_UM_MAX\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"mdb_page_touch\0",
                    ))
                    .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    2823 as libc::c_int,
                );
            };
            np = mdb_page_malloc(txn, 1 as libc::c_int as libc::c_uint);
            if np.is_null() {
                return ENOMEM;
            }
            mid.mid = pgno;
            mid.mptr = np as *mut libc::c_void;
            rc = mdb_mid2l_insert(dl, &mut mid);
            if rc == 0 as libc::c_int {
            } else {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"rc == 0\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"mdb_page_touch\0",
                    ))
                    .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    2831 as libc::c_int,
                );
            };
        } else {
            return 0 as libc::c_int;
        }
        current_block = 13131896068329595644;
    }
    match current_block {
        13131896068329595644 => {
            mdb_page_copy(np, mp, (*(*txn).mt_env).me_psize);
            (*np).mp_p.p_pgno = pgno;
            (*np).mp_flags = ((*np).mp_flags as libc::c_int | P_DIRTY) as uint16_t;
        }
        _ => {}
    }
    (*mc).mc_pg[(*mc).mc_top as usize] = np;
    m2 = *((*txn).mt_cursors).offset((*mc).mc_dbi as isize);
    if (*mc).mc_flags & C_SUB as libc::c_uint != 0 {
        while !m2.is_null() {
            m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
            if !(((*m3).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int) {
                if (*m3).mc_pg[(*mc).mc_top as usize] == mp {
                    (*m3).mc_pg[(*mc).mc_top as usize] = np;
                }
            }
            m2 = (*m2).mc_next;
        }
    } else {
        while !m2.is_null() {
            if !(((*m2).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int) {
                if !(m2 == mc) {
                    if (*m2).mc_pg[(*mc).mc_top as usize] == mp {
                        (*m2).mc_pg[(*mc).mc_top as usize] = np;
                        if (*(np as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                            & 0x2 as libc::c_int
                            == 0x2 as libc::c_int
                        {
                            let mut xr_pg = np;
                            let mut xr_node = 0 as *mut MDB_node;
                            if !(!(!((*m2).mc_xcursor).is_null()
                                && (*(*m2).mc_xcursor).mx_cursor.mc_flags
                                    & C_INITIALIZED as libc::c_uint
                                    != 0)
                                || (*m2).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                    >= ((*(xr_pg as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                                        as libc::c_uint)
                                        .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                            if 0 as libc::c_int != 0 {
                                                PAGEHDRSZ as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            },
                                        ))
                                        >> 1 as libc::c_int)
                            {
                                xr_node = (xr_pg as *mut libc::c_char)
                                    .offset(
                                        *((*(xr_pg as *mut libc::c_void as *mut MDB_page2))
                                            .mp2_ptrs)
                                            .as_mut_ptr()
                                            .offset((*m2).mc_ki[(*mc).mc_top as usize] as isize)
                                            as libc::c_int
                                            as isize,
                                    )
                                    .offset(
                                        (if 0 as libc::c_int != 0 {
                                            PAGEHDRSZ as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        }) as isize,
                                    ) as *mut MDB_node;
                                if (*xr_node).mn_flags as libc::c_int & (F_DUPDATA | F_SUBDATA)
                                    == F_DUPDATA
                                {
                                    (*(*m2).mc_xcursor).mx_cursor.mc_pg
                                        [0 as libc::c_int as usize] = ((*xr_node).mn_data)
                                        .as_mut_ptr()
                                        .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                        as *mut libc::c_void
                                        as *mut MDB_page;
                                }
                            }
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_env_sync0(
    mut env: *mut MDB_env,
    mut force: libc::c_int,
    mut numpgs: pgno_t,
) -> libc::c_int {
    let mut rc = 0 as libc::c_int;
    if (*env).me_flags & MDB_RDONLY as uint32_t != 0 {
        return EACCES;
    }
    if force != 0 || (*env).me_flags & MDB_NOSYNC as uint32_t == 0 {
        if (*env).me_flags & MDB_WRITEMAP as uint32_t != 0 {
            let mut flags = if (*env).me_flags & MDB_MAPASYNC as uint32_t != 0 && force == 0 {
                MS_ASYNC
            } else {
                MS_SYNC
            };
            if msync(
                (*env).me_map as *mut libc::c_void,
                (*env).me_psize as pgno_t * numpgs,
                flags,
            ) != 0
            {
                rc = *__error();
            }
        } else if fsync((*env).me_fd) != 0 {
            rc = *__error();
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_env_sync(
    mut env: *mut MDB_env,
    mut force: libc::c_int,
) -> libc::c_int {
    let mut m = mdb_env_pick_meta(env);
    return mdb_env_sync0(
        env,
        force,
        ((*m).mm_last_pg).wrapping_add(1 as libc::c_int as pgno_t),
    );
}
unsafe extern "C" fn mdb_cursor_shadow(
    mut src: *mut MDB_txn,
    mut dst: *mut MDB_txn,
) -> libc::c_int {
    let mut mc = 0 as *mut MDB_cursor;
    let mut bk = 0 as *mut MDB_cursor;
    let mut mx = 0 as *mut MDB_xcursor;
    let mut size: size_t = 0;
    let mut i: libc::c_int = 0;
    i = (*src).mt_numdbs as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        mc = *((*src).mt_cursors).offset(i as isize);
        if !mc.is_null() {
            size = ::core::mem::size_of::<MDB_cursor>() as libc::c_ulong;
            if !((*mc).mc_xcursor).is_null() {
                size = (size as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<MDB_xcursor>() as libc::c_ulong)
                    as size_t as size_t;
            }
            while !mc.is_null() {
                bk = malloc(size) as *mut MDB_cursor;
                if bk.is_null() {
                    return ENOMEM;
                }
                *bk = *mc;
                (*mc).mc_backup = bk;
                (*mc).mc_db = &mut *((*dst).mt_dbs).offset(i as isize) as *mut MDB_db;
                (*mc).mc_txn = dst;
                (*mc).mc_dbflag =
                    &mut *((*dst).mt_dbflags).offset(i as isize) as *mut libc::c_uchar;
                mx = (*mc).mc_xcursor;
                if !mx.is_null() {
                    *(bk.offset(1 as libc::c_int as isize) as *mut MDB_xcursor) = *mx;
                    (*mx).mx_cursor.mc_txn = dst;
                }
                (*mc).mc_next = *((*dst).mt_cursors).offset(i as isize);
                let ref mut fresh3 = *((*dst).mt_cursors).offset(i as isize);
                *fresh3 = mc;
                mc = (*bk).mc_next;
            }
        }
    }
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_cursors_close(mut txn: *mut MDB_txn, mut merge: libc::c_uint) {
    let mut cursors = (*txn).mt_cursors;
    let mut mc = 0 as *mut MDB_cursor;
    let mut next = 0 as *mut MDB_cursor;
    let mut bk = 0 as *mut MDB_cursor;
    let mut mx = 0 as *mut MDB_xcursor;
    let mut i: libc::c_int = 0;
    i = (*txn).mt_numdbs as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        mc = *cursors.offset(i as isize);
        while !mc.is_null() {
            next = (*mc).mc_next;
            bk = (*mc).mc_backup;
            if !bk.is_null() {
                if merge != 0 {
                    (*mc).mc_next = (*bk).mc_next;
                    (*mc).mc_backup = (*bk).mc_backup;
                    (*mc).mc_txn = (*bk).mc_txn;
                    (*mc).mc_db = (*bk).mc_db;
                    (*mc).mc_dbflag = (*bk).mc_dbflag;
                    mx = (*mc).mc_xcursor;
                    if !mx.is_null() {
                        (*mx).mx_cursor.mc_txn = (*bk).mc_txn;
                    }
                } else {
                    *mc = *bk;
                    mx = (*mc).mc_xcursor;
                    if !mx.is_null() {
                        *mx = *(bk.offset(1 as libc::c_int as isize) as *mut MDB_xcursor);
                    }
                }
                mc = bk;
            }
            free(mc as *mut libc::c_void);
            mc = next;
        }
        let ref mut fresh4 = *cursors.offset(i as isize);
        *fresh4 = NULL as *mut MDB_cursor;
    }
}
unsafe extern "C" fn mdb_reader_pid(
    mut env: *mut MDB_env,
    mut op: Pidlock_op,
    mut pid: pid_t,
) -> libc::c_int {
    loop {
        let mut rc: libc::c_int = 0;
        let mut lock_info = flock {
            l_start: 0,
            l_len: 0,
            l_pid: 0,
            l_type: 0,
            l_whence: 0,
        };
        memset(
            &mut lock_info as *mut flock as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<flock>() as libc::c_ulong,
        );
        lock_info.l_type = F_WRLCK as libc::c_short;
        lock_info.l_whence = SEEK_SET as libc::c_short;
        lock_info.l_start = pid as off_t;
        lock_info.l_len = 1 as libc::c_int as off_t;
        rc = fcntl(
            (*env).me_lfd,
            op as libc::c_int,
            &mut lock_info as *mut flock,
        );
        if rc == 0 as libc::c_int {
            if op as libc::c_uint == F_GETLK as libc::c_uint
                && lock_info.l_type as libc::c_int != F_UNLCK
            {
                rc = -(1 as libc::c_int);
            }
        } else {
            rc = *__error();
            if rc == EINTR {
                continue;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn mdb_txn_renew0(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut env = (*txn).mt_env;
    let mut ti = (*env).me_txns;
    let mut meta = 0 as *mut MDB_meta;
    let mut i: libc::c_uint = 0;
    let mut nr: libc::c_uint = 0;
    let mut flags = (*txn).mt_flags;
    let mut x: uint16_t = 0;
    let mut rc: libc::c_int = 0;
    let mut new_notls = 0 as libc::c_int;
    flags &= MDB_TXN_RDONLY as libc::c_uint;
    if flags != 0 as libc::c_int as libc::c_uint {
        if ti.is_null() {
            meta = mdb_env_pick_meta(env);
            (*txn).mt_txnid = (*meta).mm_txnid;
            (*txn).mt_u.reader = NULL as *mut MDB_reader;
        } else {
            let mut r = (if (*env).me_flags & MDB_NOTLS as uint32_t != 0 {
                (*txn).mt_u.reader as *mut libc::c_void
            } else {
                pthread_getspecific((*env).me_txkey)
            }) as *mut MDB_reader;
            if !r.is_null() {
                if (*r).mru.mrx.mrb_pid != (*env).me_pid
                    || (*r).mru.mrx.mrb_txnid != -(1 as libc::c_int) as txnid_t
                {
                    return MDB_BAD_RSLOT;
                }
            } else {
                let mut pid = (*env).me_pid;
                let mut tid = pthread_self();
                let mut rmutex = ((*env).me_rmutex).as_mut_ptr();
                if (*env).me_live_reader == 0 {
                    rc = mdb_reader_pid(env, Pidset, pid);
                    if rc != 0 {
                        return rc;
                    }
                    (*env).me_live_reader = 1 as libc::c_int;
                }
                rc = mdb_sem_wait(rmutex);
                if rc != 0 && {
                    rc = mdb_mutex_failed(env, rmutex, rc);
                    rc != 0
                } {
                    return rc;
                }
                nr = (*ti).mt1.mtb.mtb_numreaders;
                i = 0 as libc::c_int as libc::c_uint;
                while i < nr {
                    if (*((*ti).mti_readers).as_mut_ptr().offset(i as isize))
                        .mru
                        .mrx
                        .mrb_pid
                        == 0 as libc::c_int
                    {
                        break;
                    }
                    i = i.wrapping_add(1);
                    i;
                }
                if i == (*env).me_maxreaders {
                    let mut sb = {
                        let mut init = sembuf {
                            sem_num: 0 as libc::c_int as libc::c_ushort,
                            sem_op: 1 as libc::c_int as libc::c_short,
                            sem_flg: SEM_UNDO as libc::c_short,
                        };
                        init
                    };
                    sb.sem_num = (*rmutex).semnum as libc::c_ushort;
                    *(*rmutex).locked = 0 as libc::c_int;
                    semop((*rmutex).semid, &mut sb, 1 as libc::c_int as size_t);
                    return MDB_READERS_FULL;
                }
                r = &mut *((*ti).mti_readers).as_mut_ptr().offset(i as isize) as *mut MDB_reader;
                ::core::ptr::write_volatile(
                    &mut (*r).mru.mrx.mrb_pid as *mut pid_t,
                    0 as libc::c_int,
                );
                ::core::ptr::write_volatile(
                    &mut (*r).mru.mrx.mrb_txnid as *mut txnid_t,
                    -(1 as libc::c_int) as txnid_t,
                );
                ::core::ptr::write_volatile(&mut (*r).mru.mrx.mrb_tid as *mut pthread_t, tid);
                if i == nr {
                    nr = nr.wrapping_add(1);
                    ::core::ptr::write_volatile(
                        &mut (*ti).mt1.mtb.mtb_numreaders as *mut libc::c_uint,
                        nr,
                    );
                }
                ::core::ptr::write_volatile(
                    &mut (*env).me_close_readers as *mut libc::c_int,
                    nr as libc::c_int,
                );
                ::core::ptr::write_volatile(&mut (*r).mru.mrx.mrb_pid as *mut pid_t, pid);
                let mut sb_0 = {
                    let mut init = sembuf {
                        sem_num: 0 as libc::c_int as libc::c_ushort,
                        sem_op: 1 as libc::c_int as libc::c_short,
                        sem_flg: SEM_UNDO as libc::c_short,
                    };
                    init
                };
                sb_0.sem_num = (*rmutex).semnum as libc::c_ushort;
                *(*rmutex).locked = 0 as libc::c_int;
                semop((*rmutex).semid, &mut sb_0, 1 as libc::c_int as size_t);
                new_notls = ((*env).me_flags & MDB_NOTLS as uint32_t) as libc::c_int;
                if new_notls == 0 && {
                    rc = pthread_setspecific((*env).me_txkey, r as *const libc::c_void);
                    rc != 0
                } {
                    ::core::ptr::write_volatile(
                        &mut (*r).mru.mrx.mrb_pid as *mut pid_t,
                        0 as libc::c_int,
                    );
                    return rc;
                }
            }
            loop {
                ::core::ptr::write_volatile(
                    &mut (*r).mru.mrx.mrb_txnid as *mut txnid_t,
                    (*ti).mt1.mtb.mtb_txnid,
                );
                if !((*r).mru.mrx.mrb_txnid != (*ti).mt1.mtb.mtb_txnid) {
                    break;
                }
            }
            if (*r).mru.mrx.mrb_txnid == 0 && (*env).me_flags & MDB_RDONLY as uint32_t != 0 {
                meta = mdb_env_pick_meta(env);
                ::core::ptr::write_volatile(
                    &mut (*r).mru.mrx.mrb_txnid as *mut txnid_t,
                    (*meta).mm_txnid,
                );
            } else {
                meta = (*env).me_metas
                    [((*r).mru.mrx.mrb_txnid & 1 as libc::c_int as txnid_t) as usize];
            }
            (*txn).mt_txnid = (*r).mru.mrx.mrb_txnid;
            (*txn).mt_u.reader = r;
        }
    } else {
        if !ti.is_null() {
            rc = mdb_sem_wait(((*env).me_wmutex).as_mut_ptr());
            if rc != 0 && {
                rc = mdb_mutex_failed(env, ((*env).me_wmutex).as_mut_ptr(), rc);
                rc != 0
            } {
                return rc;
            }
            (*txn).mt_txnid = (*ti).mt1.mtb.mtb_txnid;
            meta = (*env).me_metas[((*txn).mt_txnid & 1 as libc::c_int as txnid_t) as usize];
        } else {
            meta = mdb_env_pick_meta(env);
            (*txn).mt_txnid = (*meta).mm_txnid;
        }
        (*txn).mt_txnid = ((*txn).mt_txnid).wrapping_add(1);
        (*txn).mt_txnid;
        (*txn).mt_child = NULL as *mut MDB_txn;
        (*txn).mt_loose_pgs = NULL as *mut MDB_page;
        (*txn).mt_loose_count = 0 as libc::c_int;
        (*txn).mt_dirty_room = MDB_IDL_UM_MAX as libc::c_uint;
        (*txn).mt_u.dirty_list = (*env).me_dirty_list;
        (*((*txn).mt_u.dirty_list).offset(0 as libc::c_int as isize)).mid =
            0 as libc::c_int as MDB_ID;
        (*txn).mt_free_pgs = (*env).me_free_pgs;
        *((*txn).mt_free_pgs).offset(0 as libc::c_int as isize) = 0 as libc::c_int as MDB_ID;
        (*txn).mt_spill_pgs = NULL as MDB_IDL;
        (*env).me_txn = txn;
        memcpy(
            (*txn).mt_dbiseqs as *mut libc::c_void,
            (*env).me_dbiseqs as *const libc::c_void,
            ((*env).me_maxdbs as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        );
    }
    memcpy(
        (*txn).mt_dbs as *mut libc::c_void,
        ((*meta).mm_dbs).as_mut_ptr() as *const libc::c_void,
        (CORE_DBS as libc::c_ulong).wrapping_mul(::core::mem::size_of::<MDB_db>() as libc::c_ulong),
    );
    (*txn).mt_next_pgno = ((*meta).mm_last_pg).wrapping_add(1 as libc::c_int as pgno_t);
    (*txn).mt_flags = flags;
    (*txn).mt_numdbs = (*env).me_numdbs;
    i = CORE_DBS as libc::c_uint;
    while i < (*txn).mt_numdbs {
        x = *((*env).me_dbflags).offset(i as isize);
        (*((*txn).mt_dbs).offset(i as isize)).md_flags =
            (x as libc::c_int & PERSISTENT_FLAGS) as uint16_t;
        *((*txn).mt_dbflags).offset(i as isize) = (if x as libc::c_int & MDB_VALID != 0 {
            DB_VALID | DB_USRVALID | DB_STALE
        } else {
            0 as libc::c_int
        }) as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    *((*txn).mt_dbflags).offset(MAIN_DBI as isize) = (DB_VALID | DB_USRVALID) as libc::c_uchar;
    *((*txn).mt_dbflags).offset(FREE_DBI as isize) = DB_VALID as libc::c_uchar;
    if (*env).me_flags & MDB_FATAL_ERROR != 0 {
        rc = MDB_PANIC;
    } else if (*env).me_maxpg < (*txn).mt_next_pgno {
        rc = MDB_MAP_RESIZED;
    } else {
        return MDB_SUCCESS;
    }
    mdb_txn_end(
        txn,
        (new_notls | MDB_END_FAIL_BEGIN as libc::c_int) as libc::c_uint,
    );
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_txn_renew(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if txn.is_null()
        || !((*txn).mt_flags & (0x20000 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint
            == (0x20000 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint)
    {
        return EINVAL;
    }
    rc = mdb_txn_renew0(txn);
    let _ = rc == MDB_SUCCESS;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_txn_begin(
    mut env: *mut MDB_env,
    mut parent: *mut MDB_txn,
    mut flags: libc::c_uint,
    mut ret: *mut *mut MDB_txn,
) -> libc::c_int {
    let mut current_block: u64;
    let mut txn = 0 as *mut MDB_txn;
    let mut ntxn = 0 as *mut MDB_ntxn;
    let mut rc: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut tsize: libc::c_int = 0;
    flags &= MDB_TXN_BEGIN_FLAGS as libc::c_uint;
    flags |= (*env).me_flags & MDB_WRITEMAP as uint32_t;
    if (*env).me_flags & MDB_RDONLY as uint32_t & !flags != 0 {
        return EACCES;
    }
    if !parent.is_null() {
        flags |= (*parent).mt_flags;
        if flags & (MDB_RDONLY | MDB_WRITEMAP | MDB_TXN_BLOCKED) as libc::c_uint != 0 {
            return if (*parent).mt_flags & MDB_TXN_RDONLY as libc::c_uint != 0 {
                EINVAL
            } else {
                MDB_BAD_TXN
            };
        }
        size = ((*env).me_maxdbs as libc::c_ulong).wrapping_mul(
            (::core::mem::size_of::<MDB_db>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<*mut MDB_cursor>() as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as libc::c_int;
        tsize = ::core::mem::size_of::<MDB_ntxn>() as libc::c_ulong as libc::c_int;
        size += tsize;
        current_block = 12800627514080957624;
    } else if flags & MDB_RDONLY as libc::c_uint != 0 {
        size = ((*env).me_maxdbs as libc::c_ulong).wrapping_mul(
            (::core::mem::size_of::<MDB_db>() as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as libc::c_int;
        tsize = ::core::mem::size_of::<MDB_txn>() as libc::c_ulong as libc::c_int;
        size += tsize;
        current_block = 12800627514080957624;
    } else {
        txn = (*env).me_txn0;
        current_block = 6097032363541911929;
    }
    match current_block {
        12800627514080957624 => {
            txn = calloc(1 as libc::c_int as libc::c_ulong, size as libc::c_ulong) as *mut MDB_txn;
            if txn.is_null() {
                return ENOMEM;
            }
            (*txn).mt_dbxs = (*env).me_dbxs;
            (*txn).mt_dbs = (txn as *mut libc::c_char).offset(tsize as isize) as *mut MDB_db;
            (*txn).mt_dbflags = (txn as *mut libc::c_uchar)
                .offset(size as isize)
                .offset(-((*env).me_maxdbs as isize));
            (*txn).mt_flags = flags;
            (*txn).mt_env = env;
            if !parent.is_null() {
                let mut i: libc::c_uint = 0;
                (*txn).mt_cursors =
                    ((*txn).mt_dbs).offset((*env).me_maxdbs as isize) as *mut *mut MDB_cursor;
                (*txn).mt_dbiseqs = (*parent).mt_dbiseqs;
                (*txn).mt_u.dirty_list = malloc(
                    (::core::mem::size_of::<MDB_ID2>() as libc::c_ulong)
                        .wrapping_mul(MDB_IDL_UM_SIZE as libc::c_ulong),
                ) as MDB_ID2L;
                if ((*txn).mt_u.dirty_list).is_null() || {
                    (*txn).mt_free_pgs = mdb_midl_alloc(MDB_IDL_UM_MAX);
                    ((*txn).mt_free_pgs).is_null()
                } {
                    free((*txn).mt_u.dirty_list as *mut libc::c_void);
                    free(txn as *mut libc::c_void);
                    return ENOMEM;
                }
                (*txn).mt_txnid = (*parent).mt_txnid;
                (*txn).mt_dirty_room = (*parent).mt_dirty_room;
                (*((*txn).mt_u.dirty_list).offset(0 as libc::c_int as isize)).mid =
                    0 as libc::c_int as MDB_ID;
                (*txn).mt_spill_pgs = NULL as MDB_IDL;
                (*txn).mt_next_pgno = (*parent).mt_next_pgno;
                (*parent).mt_flags |= MDB_TXN_HAS_CHILD as libc::c_uint;
                (*parent).mt_child = txn;
                (*txn).mt_parent = parent;
                (*txn).mt_numdbs = (*parent).mt_numdbs;
                memcpy(
                    (*txn).mt_dbs as *mut libc::c_void,
                    (*parent).mt_dbs as *const libc::c_void,
                    ((*txn).mt_numdbs as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<MDB_db>() as libc::c_ulong),
                );
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*txn).mt_numdbs {
                    *((*txn).mt_dbflags).offset(i as isize) =
                        (*((*parent).mt_dbflags).offset(i as isize) as libc::c_int & !DB_NEW)
                            as libc::c_uchar;
                    i = i.wrapping_add(1);
                    i;
                }
                rc = 0 as libc::c_int;
                ntxn = txn as *mut MDB_ntxn;
                (*ntxn).mnt_pgstate = (*env).me_pgstate;
                if !((*env).me_pgstate.mf_pghead).is_null() {
                    size = (*((*env).me_pgstate.mf_pghead).offset(0 as libc::c_int as isize))
                        .wrapping_add(1 as libc::c_int as pgno_t)
                        .wrapping_mul(::core::mem::size_of::<MDB_ID>() as libc::c_ulong)
                        as libc::c_int;
                    (*env).me_pgstate.mf_pghead = mdb_midl_alloc(
                        *((*env).me_pgstate.mf_pghead).offset(0 as libc::c_int as isize)
                            as libc::c_int,
                    );
                    if !((*env).me_pgstate.mf_pghead).is_null() {
                        memcpy(
                            (*env).me_pgstate.mf_pghead as *mut libc::c_void,
                            (*ntxn).mnt_pgstate.mf_pghead as *const libc::c_void,
                            size as libc::c_ulong,
                        );
                    } else {
                        rc = ENOMEM;
                    }
                }
                if rc == 0 {
                    rc = mdb_cursor_shadow(parent, txn);
                }
                if rc != 0 {
                    mdb_txn_end(txn, MDB_END_FAIL_BEGINCHILD as libc::c_int as libc::c_uint);
                }
                current_block = 10150597327160359210;
            } else {
                (*txn).mt_dbiseqs = (*env).me_dbiseqs;
                current_block = 6097032363541911929;
            }
        }
        _ => {}
    }
    match current_block {
        6097032363541911929 => {
            rc = mdb_txn_renew0(txn);
        }
        _ => {}
    }
    if rc != 0 {
        if txn != (*env).me_txn0 {
            free(txn as *mut libc::c_void);
        }
    } else {
        (*txn).mt_flags |= flags;
        *ret = txn;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_txn_env(mut txn: *mut MDB_txn) -> *mut MDB_env {
    if txn.is_null() {
        return NULL as *mut MDB_env;
    }
    return (*txn).mt_env;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_txn_id(mut txn: *mut MDB_txn) -> mdb_size_t {
    if txn.is_null() {
        return 0 as libc::c_int as mdb_size_t;
    }
    return (*txn).mt_txnid;
}
unsafe extern "C" fn mdb_dbis_update(mut txn: *mut MDB_txn, mut keep: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut n = (*txn).mt_numdbs;
    let mut env = (*txn).mt_env;
    let mut tdbflags = (*txn).mt_dbflags;
    i = n as libc::c_int;
    loop {
        i -= 1;
        if !(i >= CORE_DBS) {
            break;
        }
        if *tdbflags.offset(i as isize) as libc::c_int & DB_NEW != 0 {
            if keep != 0 {
                *((*env).me_dbflags).offset(i as isize) =
                    ((*((*txn).mt_dbs).offset(i as isize)).md_flags as libc::c_int | MDB_VALID)
                        as uint16_t;
            } else {
                let mut ptr =
                    (*((*env).me_dbxs).offset(i as isize)).md_name.mv_data as *mut libc::c_char;
                if !ptr.is_null() {
                    let ref mut fresh5 = (*((*env).me_dbxs).offset(i as isize)).md_name.mv_data;
                    *fresh5 = NULL as *mut libc::c_void;
                    (*((*env).me_dbxs).offset(i as isize)).md_name.mv_size =
                        0 as libc::c_int as size_t;
                    *((*env).me_dbflags).offset(i as isize) = 0 as libc::c_int as uint16_t;
                    let ref mut fresh6 = *((*env).me_dbiseqs).offset(i as isize);
                    *fresh6 = (*fresh6).wrapping_add(1);
                    let _ = *fresh6;
                    free(ptr as *mut libc::c_void);
                }
            }
        }
    }
    if keep != 0 && (*env).me_numdbs < n {
        (*env).me_numdbs = n;
    }
}
unsafe extern "C" fn mdb_txn_end(mut txn: *mut MDB_txn, mut mode: libc::c_uint) {
    let mut env = (*txn).mt_env;
    mdb_dbis_update(txn, (mode & MDB_END_UPDATE as libc::c_uint) as libc::c_int);
    if (*txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint
        == 0x20000 as libc::c_int as libc::c_uint
    {
        if !((*txn).mt_u.reader).is_null() {
            ::core::ptr::write_volatile(
                &mut (*(*txn).mt_u.reader).mru.mrx.mrb_txnid as *mut txnid_t,
                -(1 as libc::c_int) as txnid_t,
            );
            if (*env).me_flags & MDB_NOTLS as uint32_t == 0 {
                (*txn).mt_u.reader = NULL as *mut MDB_reader;
            } else if mode & MDB_END_SLOT as libc::c_uint != 0 {
                ::core::ptr::write_volatile(
                    &mut (*(*txn).mt_u.reader).mru.mrx.mrb_pid as *mut pid_t,
                    0 as libc::c_int,
                );
                (*txn).mt_u.reader = NULL as *mut MDB_reader;
            }
        }
        (*txn).mt_numdbs = 0 as libc::c_int as MDB_dbi;
        (*txn).mt_flags |= MDB_TXN_FINISHED as libc::c_uint;
    } else if !((*txn).mt_flags & 0x1 as libc::c_int as libc::c_uint
        == 0x1 as libc::c_int as libc::c_uint)
    {
        let mut pghead = (*env).me_pgstate.mf_pghead;
        if mode & MDB_END_UPDATE as libc::c_uint == 0 {
            mdb_cursors_close(txn, 0 as libc::c_int as libc::c_uint);
        }
        if (*env).me_flags & MDB_WRITEMAP as uint32_t == 0 {
            mdb_dlist_free(txn);
        }
        (*txn).mt_numdbs = 0 as libc::c_int as MDB_dbi;
        (*txn).mt_flags = MDB_TXN_FINISHED as libc::c_uint;
        if ((*txn).mt_parent).is_null() {
            mdb_midl_shrink(&mut (*txn).mt_free_pgs);
            (*env).me_free_pgs = (*txn).mt_free_pgs;
            (*env).me_pgstate.mf_pghead = NULL as *mut pgno_t;
            (*env).me_pgstate.mf_pglast = 0 as libc::c_int as txnid_t;
            (*env).me_txn = NULL as *mut MDB_txn;
            mode = 0 as libc::c_int as libc::c_uint;
            if !((*env).me_txns).is_null() {
                let mut sb = {
                    let mut init = sembuf {
                        sem_num: 0 as libc::c_int as libc::c_ushort,
                        sem_op: 1 as libc::c_int as libc::c_short,
                        sem_flg: SEM_UNDO as libc::c_short,
                    };
                    init
                };
                sb.sem_num = (*((*env).me_wmutex).as_mut_ptr()).semnum as libc::c_ushort;
                *(*((*env).me_wmutex).as_mut_ptr()).locked = 0 as libc::c_int;
                semop(
                    (*((*env).me_wmutex).as_mut_ptr()).semid,
                    &mut sb,
                    1 as libc::c_int as size_t,
                );
            }
        } else {
            (*(*txn).mt_parent).mt_child = NULL as *mut MDB_txn;
            (*(*txn).mt_parent).mt_flags &= !MDB_TXN_HAS_CHILD as libc::c_uint;
            (*env).me_pgstate = (*(txn as *mut MDB_ntxn)).mnt_pgstate;
            mdb_midl_free((*txn).mt_free_pgs);
            free((*txn).mt_u.dirty_list as *mut libc::c_void);
        }
        mdb_midl_free((*txn).mt_spill_pgs);
        mdb_midl_free(pghead);
    }
    if mode & MDB_END_FREE as libc::c_uint != 0 {
        free(txn as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mdb_txn_reset(mut txn: *mut MDB_txn) {
    if txn.is_null() {
        return;
    }
    if (*txn).mt_flags & MDB_TXN_RDONLY as libc::c_uint == 0 {
        return;
    }
    mdb_txn_end(txn, MDB_END_RESET as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn _mdb_txn_abort(mut txn: *mut MDB_txn) {
    if txn.is_null() {
        return;
    }
    if !((*txn).mt_child).is_null() {
        _mdb_txn_abort((*txn).mt_child);
    }
    mdb_txn_end(
        txn,
        (MDB_END_ABORT as libc::c_int | MDB_END_SLOT | MDB_END_FREE) as libc::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mdb_txn_abort(mut txn: *mut MDB_txn) {
    _mdb_txn_abort(txn);
}
unsafe extern "C" fn mdb_freelist_save(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut mc = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut env = (*txn).mt_env;
    let mut rc: libc::c_int = 0;
    let mut maxfree_1pg = (*env).me_maxfree_1pg;
    let mut more = 1 as libc::c_int;
    let mut pglast = 0 as libc::c_int as txnid_t;
    let mut head_id = 0 as libc::c_int as txnid_t;
    let mut freecnt = 0 as libc::c_int as pgno_t;
    let mut free_pgs = 0 as *mut pgno_t;
    let mut mop = 0 as *mut pgno_t;
    let mut head_room = 0 as libc::c_int as ssize_t;
    let mut total_room = 0 as libc::c_int as ssize_t;
    let mut mop_len: ssize_t = 0;
    let mut clean_limit: ssize_t = 0;
    mdb_cursor_init(&mut mc, txn, FREE_DBI as MDB_dbi, NULL as *mut MDB_xcursor);
    if !((*env).me_pgstate.mf_pghead).is_null() {
        rc = mdb_page_search(&mut mc, NULL as *mut MDB_val, MDB_PS_FIRST | MDB_PS_MODIFY);
        if rc != 0 && rc != MDB_NOTFOUND {
            return rc;
        }
    }
    if ((*env).me_pgstate.mf_pghead).is_null() && !((*txn).mt_loose_pgs).is_null() {
        let mut mp = (*txn).mt_loose_pgs;
        let mut dl = (*txn).mt_u.dirty_list;
        let mut x: libc::c_uint = 0;
        rc = mdb_midl_need(
            &mut (*txn).mt_free_pgs,
            (*txn).mt_loose_count as libc::c_uint,
        );
        if rc != 0 as libc::c_int {
            return rc;
        }
        while !mp.is_null() {
            let mut xidl = (*txn).mt_free_pgs;
            let ref mut fresh7 = *xidl.offset(0 as libc::c_int as isize);
            *fresh7 = (*fresh7).wrapping_add(1);
            let mut xlen = *fresh7;
            *xidl.offset(xlen as isize) = (*mp).mp_p.p_pgno;
            if (*txn).mt_flags & MDB_TXN_WRITEMAP as libc::c_uint != 0 {
                x = 1 as libc::c_int as libc::c_uint;
                while x as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid {
                    if (*dl.offset(x as isize)).mid == (*mp).mp_p.p_pgno {
                        break;
                    }
                    x = x.wrapping_add(1);
                    x;
                }
                if x as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid {
                } else {
                    mdb_assert_fail(
                        (*txn).mt_env,
                        b"x <= dl[0].mid\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                            b"mdb_freelist_save\0",
                        ))
                        .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                        3544 as libc::c_int,
                    );
                };
            } else {
                x = mdb_mid2l_search(dl, (*mp).mp_p.p_pgno);
                if (*dl.offset(x as isize)).mid == (*mp).mp_p.p_pgno {
                } else {
                    mdb_assert_fail(
                        (*txn).mt_env,
                        b"dl[x].mid == mp->mp_pgno\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                            b"mdb_freelist_save\0",
                        ))
                        .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                        3547 as libc::c_int,
                    );
                };
                mdb_dpage_free(env, mp);
            }
            let ref mut fresh8 = (*dl.offset(x as isize)).mptr;
            *fresh8 = NULL as *mut libc::c_void;
            mp = *(mp.offset(2 as libc::c_int as isize) as *mut *mut MDB_page);
        }
        let mut y: libc::c_uint = 0;
        y = 1 as libc::c_int as libc::c_uint;
        while !((*dl.offset(y as isize)).mptr).is_null()
            && y as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid
        {
            y = y.wrapping_add(1);
            y;
        }
        if y as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid {
            x = y;
            y = y.wrapping_add(1);
            y;
            loop {
                while ((*dl.offset(y as isize)).mptr).is_null()
                    && y as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid
                {
                    y = y.wrapping_add(1);
                    y;
                }
                if y as MDB_ID > (*dl.offset(0 as libc::c_int as isize)).mid {
                    break;
                }
                let fresh9 = y;
                y = y.wrapping_add(1);
                let fresh10 = x;
                x = x.wrapping_add(1);
                *dl.offset(fresh10 as isize) = *dl.offset(fresh9 as isize);
            }
            (*dl.offset(0 as libc::c_int as isize)).mid =
                x.wrapping_sub(1 as libc::c_int as libc::c_uint) as MDB_ID;
        } else {
            (*dl.offset(0 as libc::c_int as isize)).mid = 0 as libc::c_int as MDB_ID;
        }
        (*txn).mt_loose_pgs = NULL as *mut MDB_page;
        (*txn).mt_loose_count = 0 as libc::c_int;
    }
    clean_limit = if (*env).me_flags & (MDB_NOMEMINIT | MDB_WRITEMAP) as uint32_t != 0 {
        SSIZE_MAX
    } else {
        maxfree_1pg as libc::c_long
    };
    loop {
        let mut key = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        let mut data = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        let mut pgs = 0 as *mut pgno_t;
        let mut j: ssize_t = 0;
        while pglast < (*env).me_pgstate.mf_pglast {
            rc = mdb_cursor_first(&mut mc, &mut key, NULL as *mut MDB_val);
            if rc != 0 {
                return rc;
            }
            head_id = *(key.mv_data as *mut txnid_t);
            pglast = head_id;
            head_room = 0 as libc::c_int as ssize_t;
            total_room = head_room;
            if pglast <= (*env).me_pgstate.mf_pglast {
            } else {
                mdb_assert_fail(
                    (*txn).mt_env,
                    b"pglast <= env->me_pglast\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                        b"mdb_freelist_save\0",
                    ))
                    .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    3591 as libc::c_int,
                );
            };
            rc = _mdb_cursor_del(&mut mc, 0 as libc::c_int as libc::c_uint);
            if rc != 0 {
                return rc;
            }
        }
        if freecnt < *((*txn).mt_free_pgs).offset(0 as libc::c_int as isize) {
            if freecnt == 0 {
                rc = mdb_page_search(&mut mc, NULL as *mut MDB_val, MDB_PS_LAST | MDB_PS_MODIFY);
                if rc != 0 && rc != MDB_NOTFOUND {
                    return rc;
                }
            }
            free_pgs = (*txn).mt_free_pgs;
            key.mv_size = ::core::mem::size_of::<txnid_t>() as libc::c_ulong;
            key.mv_data = &mut (*txn).mt_txnid as *mut txnid_t as *mut libc::c_void;
            loop {
                freecnt = *free_pgs.offset(0 as libc::c_int as isize);
                data.mv_size = (*free_pgs.offset(0 as libc::c_int as isize))
                    .wrapping_add(1 as libc::c_int as pgno_t)
                    .wrapping_mul(::core::mem::size_of::<MDB_ID>() as libc::c_ulong);
                rc = _mdb_cursor_put(&mut mc, &mut key, &mut data, MDB_RESERVE as libc::c_uint);
                if rc != 0 {
                    return rc;
                }
                free_pgs = (*txn).mt_free_pgs;
                if !(freecnt < *free_pgs.offset(0 as libc::c_int as isize)) {
                    break;
                }
            }
            mdb_midl_sort(free_pgs);
            memcpy(data.mv_data, free_pgs as *const libc::c_void, data.mv_size);
        } else {
            mop = (*env).me_pgstate.mf_pghead;
            mop_len = (if !mop.is_null() {
                *mop.offset(0 as libc::c_int as isize)
            } else {
                0 as libc::c_int as pgno_t
            })
            .wrapping_add((*txn).mt_loose_count as pgno_t) as ssize_t;
            if total_room >= mop_len {
                if total_room == mop_len || {
                    more -= 1;
                    more < 0 as libc::c_int
                } {
                    break;
                }
            } else if head_room >= maxfree_1pg as ssize_t && head_id > 1 as libc::c_int as txnid_t {
                head_id = head_id.wrapping_sub(1);
                head_id;
                head_room = 0 as libc::c_int as ssize_t;
            }
            total_room -= head_room;
            head_room = mop_len - total_room;
            if head_room > maxfree_1pg as ssize_t && head_id > 1 as libc::c_int as txnid_t {
                head_room = (head_room as txnid_t / head_id) as ssize_t as ssize_t;
                head_room += maxfree_1pg as ssize_t
                    - head_room % (maxfree_1pg + 1 as libc::c_int) as ssize_t;
            } else if head_room < 0 as libc::c_int as ssize_t {
                head_room = 0 as libc::c_int as ssize_t;
            }
            key.mv_size = ::core::mem::size_of::<txnid_t>() as libc::c_ulong;
            key.mv_data = &mut head_id as *mut txnid_t as *mut libc::c_void;
            data.mv_size = ((head_room + 1 as libc::c_int as ssize_t) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<pgno_t>() as libc::c_ulong);
            rc = _mdb_cursor_put(&mut mc, &mut key, &mut data, MDB_RESERVE as libc::c_uint);
            if rc != 0 {
                return rc;
            }
            pgs = data.mv_data as *mut pgno_t;
            j = if head_room > clean_limit {
                head_room
            } else {
                0 as libc::c_int as ssize_t
            };
            loop {
                *pgs.offset(j as isize) = 0 as libc::c_int as pgno_t;
                j -= 1;
                if !(j >= 0 as libc::c_int as ssize_t) {
                    break;
                }
            }
            total_room += head_room;
        }
    }
    if !((*txn).mt_loose_pgs).is_null() {
        let mut mp_0 = (*txn).mt_loose_pgs;
        let mut count = (*txn).mt_loose_count as libc::c_uint;
        let mut loose = 0 as *mut MDB_ID;
        rc = mdb_midl_need(
            &mut (*env).me_pgstate.mf_pghead,
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul(count)
                .wrapping_add(1 as libc::c_int as libc::c_uint),
        );
        if rc != 0 as libc::c_int {
            return rc;
        }
        mop = (*env).me_pgstate.mf_pghead;
        loose = mop
            .offset(*mop.offset(-(1 as libc::c_int) as isize) as isize)
            .offset(-(count as isize));
        count = 0 as libc::c_int as libc::c_uint;
        while !mp_0.is_null() {
            count = count.wrapping_add(1);
            *loose.offset(count as isize) = (*mp_0).mp_p.p_pgno;
            mp_0 = *(mp_0.offset(2 as libc::c_int as isize) as *mut *mut MDB_page);
        }
        *loose.offset(0 as libc::c_int as isize) = count as MDB_ID;
        mdb_midl_sort(loose);
        mdb_midl_xmerge(mop, loose);
        (*txn).mt_loose_pgs = NULL as *mut MDB_page;
        (*txn).mt_loose_count = 0 as libc::c_int;
        mop_len = *mop.offset(0 as libc::c_int as isize) as ssize_t;
    }
    rc = MDB_SUCCESS;
    if mop_len != 0 {
        let mut key_0 = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        let mut data_0 = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        mop = mop.offset(mop_len as isize);
        rc = mdb_cursor_first(&mut mc, &mut key_0, &mut data_0);
        while rc == 0 {
            let mut id = *(key_0.mv_data as *mut txnid_t);
            let mut len = (data_0.mv_size)
                .wrapping_div(::core::mem::size_of::<MDB_ID>() as libc::c_ulong)
                as ssize_t
                - 1 as libc::c_int as ssize_t;
            let mut save: MDB_ID = 0;
            if len >= 0 as libc::c_int as ssize_t && id <= (*env).me_pgstate.mf_pglast {
            } else {
                mdb_assert_fail(
                    (*txn).mt_env,
                    b"len >= 0 && id <= env->me_pglast\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                        b"mdb_freelist_save\0",
                    ))
                    .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    3707 as libc::c_int,
                );
            };
            key_0.mv_data = &mut id as *mut txnid_t as *mut libc::c_void;
            if len > mop_len {
                len = mop_len;
                data_0.mv_size = ((len + 1 as libc::c_int as ssize_t) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<MDB_ID>() as libc::c_ulong);
            }
            mop = mop.offset(-(len as isize));
            data_0.mv_data = mop as *mut libc::c_void;
            save = *mop.offset(0 as libc::c_int as isize);
            *mop.offset(0 as libc::c_int as isize) = len as pgno_t;
            rc = _mdb_cursor_put(
                &mut mc,
                &mut key_0,
                &mut data_0,
                MDB_CURRENT as libc::c_uint,
            );
            *mop.offset(0 as libc::c_int as isize) = save;
            if rc != 0 || {
                mop_len -= len;
                mop_len == 0
            } {
                break;
            }
            rc = mdb_cursor_next(&mut mc, &mut key_0, &mut data_0, MDB_NEXT);
        }
    }
    return rc;
}
unsafe extern "C" fn mdb_page_flush(mut txn: *mut MDB_txn, mut keep: libc::c_int) -> libc::c_int {
    let mut env = (*txn).mt_env;
    let mut dl = (*txn).mt_u.dirty_list;
    let mut psize = (*env).me_psize;
    let mut j: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut pagecount = (*dl.offset(0 as libc::c_int as isize)).mid as libc::c_int;
    let mut rc: libc::c_int = 0;
    let mut size = 0 as libc::c_int as size_t;
    let mut pos = 0 as libc::c_int as off_t;
    let mut pgno = 0 as libc::c_int as pgno_t;
    let mut dp = NULL as *mut MDB_page;
    let mut iov: [iovec; 64] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 64];
    let mut fd = (*env).me_fd;
    let mut wsize = 0 as libc::c_int as ssize_t;
    let mut wres: ssize_t = 0;
    let mut wpos = 0 as libc::c_int as off_t;
    let mut next_pos = 1 as libc::c_int as off_t;
    let mut n = 0 as libc::c_int;
    i = keep;
    j = i as libc::c_uint;
    if (*env).me_flags & MDB_WRITEMAP as uint32_t != 0 {
        loop {
            i += 1;
            if !(i <= pagecount) {
                break;
            }
            dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
            if (*dp).mp_flags as libc::c_int & (P_LOOSE | P_KEEP) != 0 {
                (*dp).mp_flags = ((*dp).mp_flags as libc::c_int & !P_KEEP) as uint16_t;
                j = j.wrapping_add(1);
                *dl.offset(j as isize) = *dl.offset(i as isize);
            } else {
                (*dp).mp_flags = ((*dp).mp_flags as libc::c_int & !P_DIRTY) as uint16_t;
            }
        }
    } else {
        loop {
            i += 1;
            if i <= pagecount {
                dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
                if (*dp).mp_flags as libc::c_int & (P_LOOSE | P_KEEP) != 0 {
                    (*dp).mp_flags = ((*dp).mp_flags as libc::c_int & !P_KEEP) as uint16_t;
                    (*dl.offset(i as isize)).mid = 0 as libc::c_int as MDB_ID;
                    continue;
                } else {
                    pgno = (*dl.offset(i as isize)).mid;
                    (*dp).mp_flags = ((*dp).mp_flags as libc::c_int & !P_DIRTY) as uint16_t;
                    pos = (pgno * psize as pgno_t) as off_t;
                    size = psize as size_t;
                    if (*(dp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                        & 0x4 as libc::c_int
                        == 0x4 as libc::c_int
                    {
                        size = size * (*dp).mp_pb.pb_pages as size_t;
                    }
                }
            }
            if pos != next_pos
                || n == MDB_COMMIT_PAGES
                || (wsize as size_t).wrapping_add(size) > MAX_WRITE as size_t
            {
                if n != 0 {
                    's_208: {
                        loop {
                            if n == 1 as libc::c_int {
                                wres = pwrite(
                                    fd,
                                    iov[0 as libc::c_int as usize].iov_base,
                                    wsize as size_t,
                                    wpos,
                                );
                            } else {
                                's_156: {
                                    loop {
                                        if lseek(fd, wpos, SEEK_SET) == -(1 as libc::c_int) as off_t
                                        {
                                            rc = *__error();
                                            if rc == EINTR {
                                                continue;
                                            }
                                            return rc;
                                        } else {
                                            break 's_156;
                                        }
                                    }
                                }
                                wres = writev(fd, iov.as_mut_ptr(), n);
                            }
                            if wres != wsize {
                                if wres < 0 as libc::c_int as ssize_t {
                                    rc = *__error();
                                    if !(rc == EINTR) {
                                        break;
                                    }
                                } else {
                                    rc = EIO;
                                    break;
                                }
                            } else {
                                n = 0 as libc::c_int;
                                break 's_208;
                            }
                        }
                        return rc;
                    }
                }
                if i > pagecount {
                    break;
                }
                wpos = pos;
                wsize = 0 as libc::c_int as ssize_t;
            }
            iov[n as usize].iov_len = size;
            iov[n as usize].iov_base = dp as *mut libc::c_char as *mut libc::c_void;
            next_pos = (pos as libc::c_ulonglong).wrapping_add(size as libc::c_ulonglong) as off_t;
            wsize = (wsize as size_t).wrapping_add(size) as ssize_t as ssize_t;
            n += 1;
            n;
        }
        if (*env).me_flags & MDB_WRITEMAP as uint32_t == 0 {
            i = keep;
            loop {
                i += 1;
                if !(i <= pagecount) {
                    break;
                }
                dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
                if (*dl.offset(i as isize)).mid == 0 {
                    j = j.wrapping_add(1);
                    *dl.offset(j as isize) = *dl.offset(i as isize);
                    (*dl.offset(j as isize)).mid = (*dp).mp_p.p_pgno;
                } else {
                    mdb_dpage_free(env, dp);
                }
            }
        }
    }
    i -= 1;
    i;
    (*txn).mt_dirty_room = ((*txn).mt_dirty_room).wrapping_add((i as libc::c_uint).wrapping_sub(j));
    (*dl.offset(0 as libc::c_int as isize)).mid = j as MDB_ID;
    return MDB_SUCCESS;
}
unsafe extern "C" fn _mdb_txn_commit(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut end_mode: libc::c_uint = 0;
    let mut env = 0 as *mut MDB_env;
    if txn.is_null() {
        return EINVAL;
    }
    end_mode = (MDB_END_EMPTY_COMMIT as libc::c_int | MDB_END_UPDATE | MDB_END_SLOT | MDB_END_FREE)
        as libc::c_uint;
    if !((*txn).mt_child).is_null() {
        rc = _mdb_txn_commit((*txn).mt_child);
        if rc != 0 {
            current_block = 16403181400158345396;
        } else {
            current_block = 11875828834189669668;
        }
    } else {
        current_block = 11875828834189669668;
    }
    match current_block {
        11875828834189669668 => {
            env = (*txn).mt_env;
            if (*txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint
                == 0x20000 as libc::c_int as libc::c_uint
            {
                current_block = 13661098578361514953;
            } else if (*txn).mt_flags & (MDB_TXN_FINISHED | MDB_TXN_ERROR) as libc::c_uint != 0 {
                if !((*txn).mt_parent).is_null() {
                    (*(*txn).mt_parent).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
                }
                rc = MDB_BAD_TXN;
                current_block = 16403181400158345396;
            } else if !((*txn).mt_parent).is_null() {
                let mut parent = (*txn).mt_parent;
                let mut lp = 0 as *mut *mut MDB_page;
                let mut dst = 0 as *mut MDB_ID2;
                let mut src = 0 as *mut MDB_ID2;
                let mut pspill = 0 as *mut MDB_ID;
                let mut x: libc::c_uint = 0;
                let mut y: libc::c_uint = 0;
                let mut len: libc::c_uint = 0;
                let mut ps_len: libc::c_uint = 0;
                rc = mdb_midl_append_list(&mut (*parent).mt_free_pgs, (*txn).mt_free_pgs);
                if rc != 0 {
                    current_block = 16403181400158345396;
                } else {
                    mdb_midl_free((*txn).mt_free_pgs);
                    (*parent).mt_next_pgno = (*txn).mt_next_pgno;
                    (*parent).mt_flags = (*txn).mt_flags;
                    mdb_cursors_close(txn, 1 as libc::c_int as libc::c_uint);
                    memcpy(
                        (*parent).mt_dbs as *mut libc::c_void,
                        (*txn).mt_dbs as *const libc::c_void,
                        ((*txn).mt_numdbs as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<MDB_db>() as libc::c_ulong),
                    );
                    (*parent).mt_numdbs = (*txn).mt_numdbs;
                    *((*parent).mt_dbflags).offset(FREE_DBI as isize) =
                        *((*txn).mt_dbflags).offset(FREE_DBI as isize);
                    *((*parent).mt_dbflags).offset(MAIN_DBI as isize) =
                        *((*txn).mt_dbflags).offset(MAIN_DBI as isize);
                    i = CORE_DBS as libc::c_uint;
                    while i < (*txn).mt_numdbs {
                        x = (*((*parent).mt_dbflags).offset(i as isize) as libc::c_int & DB_NEW)
                            as libc::c_uint;
                        *((*parent).mt_dbflags).offset(i as isize) =
                            (*((*txn).mt_dbflags).offset(i as isize) as libc::c_uint | x)
                                as libc::c_uchar;
                        i = i.wrapping_add(1);
                        i;
                    }
                    dst = (*parent).mt_u.dirty_list;
                    src = (*txn).mt_u.dirty_list;
                    pspill = (*parent).mt_spill_pgs;
                    if !pspill.is_null() && {
                        ps_len = *pspill.offset(0 as libc::c_int as isize) as libc::c_uint;
                        ps_len != 0
                    } {
                        y = ps_len;
                        x = y;
                        *pspill.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as pgno_t;
                        i = 0 as libc::c_int as libc::c_uint;
                        len = (*src.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
                        loop {
                            i = i.wrapping_add(1);
                            if !(i <= len) {
                                break;
                            }
                            let mut pn = (*src.offset(i as isize)).mid << 1 as libc::c_int;
                            while pn > *pspill.offset(x as isize) {
                                x = x.wrapping_sub(1);
                                x;
                            }
                            if pn == *pspill.offset(x as isize) {
                                *pspill.offset(x as isize) = 1 as libc::c_int as MDB_ID;
                                x = x.wrapping_sub(1);
                                y = x;
                            }
                        }
                        x = y;
                        loop {
                            x = x.wrapping_add(1);
                            if !(x <= ps_len) {
                                break;
                            }
                            if *pspill.offset(x as isize) & 1 as libc::c_int as MDB_ID == 0 {
                                y = y.wrapping_add(1);
                                *pspill.offset(y as isize) = *pspill.offset(x as isize);
                            }
                        }
                        *pspill.offset(0 as libc::c_int as isize) = y as MDB_ID;
                    }
                    if !((*txn).mt_spill_pgs).is_null()
                        && *((*txn).mt_spill_pgs).offset(0 as libc::c_int as isize) != 0
                    {
                        i = 1 as libc::c_int as libc::c_uint;
                        while i as MDB_ID
                            <= *((*txn).mt_spill_pgs).offset(0 as libc::c_int as isize)
                        {
                            let mut pn_0 = *((*txn).mt_spill_pgs).offset(i as isize);
                            if !(pn_0 & 1 as libc::c_int as MDB_ID != 0) {
                                pn_0 >>= 1 as libc::c_int;
                                y = mdb_mid2l_search(dst, pn_0);
                                if y as MDB_ID <= (*dst.offset(0 as libc::c_int as isize)).mid
                                    && (*dst.offset(y as isize)).mid == pn_0
                                {
                                    free((*dst.offset(y as isize)).mptr);
                                    while (y as MDB_ID)
                                        < (*dst.offset(0 as libc::c_int as isize)).mid
                                    {
                                        *dst.offset(y as isize) = *dst.offset(
                                            y.wrapping_add(1 as libc::c_int as libc::c_uint)
                                                as isize,
                                        );
                                        y = y.wrapping_add(1);
                                        y;
                                    }
                                    let ref mut fresh11 =
                                        (*dst.offset(0 as libc::c_int as isize)).mid;
                                    *fresh11 = (*fresh11).wrapping_sub(1);
                                    let _ = *fresh11;
                                }
                            }
                            i = i.wrapping_add(1);
                            i;
                        }
                    }
                    x = (*dst.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
                    (*dst.offset(0 as libc::c_int as isize)).mid = 0 as libc::c_int as MDB_ID;
                    if !((*parent).mt_parent).is_null() {
                        len = (x as MDB_ID)
                            .wrapping_add((*src.offset(0 as libc::c_int as isize)).mid)
                            as libc::c_uint;
                        y = (mdb_mid2l_search(
                            src,
                            ((*dst.offset(x as isize)).mid)
                                .wrapping_add(1 as libc::c_int as MDB_ID),
                        ))
                        .wrapping_sub(1 as libc::c_int as libc::c_uint);
                        i = x;
                        while y != 0 && i != 0 {
                            let mut yp = (*src.offset(y as isize)).mid;
                            while yp < (*dst.offset(i as isize)).mid {
                                i = i.wrapping_sub(1);
                                i;
                            }
                            if yp == (*dst.offset(i as isize)).mid {
                                i = i.wrapping_sub(1);
                                i;
                                len = len.wrapping_sub(1);
                                len;
                            }
                            y = y.wrapping_sub(1);
                            y;
                        }
                    } else {
                        len = (MDB_IDL_UM_MAX as libc::c_uint).wrapping_sub((*txn).mt_dirty_room);
                    }
                    y = (*src.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
                    i = len;
                    while y != 0 {
                        let mut yp_0 = (*src.offset(y as isize)).mid;
                        while yp_0 < (*dst.offset(x as isize)).mid {
                            let fresh12 = x;
                            x = x.wrapping_sub(1);
                            let fresh13 = i;
                            i = i.wrapping_sub(1);
                            *dst.offset(fresh13 as isize) = *dst.offset(fresh12 as isize);
                        }
                        if yp_0 == (*dst.offset(x as isize)).mid {
                            let fresh14 = x;
                            x = x.wrapping_sub(1);
                            free((*dst.offset(fresh14 as isize)).mptr);
                        }
                        let fresh15 = y;
                        y = y.wrapping_sub(1);
                        let fresh16 = i;
                        i = i.wrapping_sub(1);
                        *dst.offset(fresh16 as isize) = *src.offset(fresh15 as isize);
                    }
                    if i == x {
                    } else {
                        mdb_assert_fail(
                            (*txn).mt_env,
                            b"i == x\0" as *const u8 as *const libc::c_char,
                            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                                b"_mdb_txn_commit\0",
                            ))
                            .as_ptr(),
                            b"mdb.c\0" as *const u8 as *const libc::c_char,
                            4098 as libc::c_int,
                        );
                    };
                    (*dst.offset(0 as libc::c_int as isize)).mid = len as MDB_ID;
                    free((*txn).mt_u.dirty_list as *mut libc::c_void);
                    (*parent).mt_dirty_room = (*txn).mt_dirty_room;
                    if !((*txn).mt_spill_pgs).is_null() {
                        if !((*parent).mt_spill_pgs).is_null() {
                            rc = mdb_midl_append_list(
                                &mut (*parent).mt_spill_pgs,
                                (*txn).mt_spill_pgs,
                            );
                            if rc != 0 {
                                (*parent).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
                            }
                            mdb_midl_free((*txn).mt_spill_pgs);
                            mdb_midl_sort((*parent).mt_spill_pgs);
                        } else {
                            (*parent).mt_spill_pgs = (*txn).mt_spill_pgs;
                        }
                    }
                    lp = &mut (*parent).mt_loose_pgs;
                    while !(*lp).is_null() {
                        lp = (*lp).offset(2 as libc::c_int as isize) as *mut *mut MDB_page;
                    }
                    *lp = (*txn).mt_loose_pgs;
                    (*parent).mt_loose_count += (*txn).mt_loose_count;
                    (*parent).mt_child = NULL as *mut MDB_txn;
                    mdb_midl_free((*(txn as *mut MDB_ntxn)).mnt_pgstate.mf_pghead);
                    free(txn as *mut libc::c_void);
                    return rc;
                }
            } else if txn != (*env).me_txn {
                rc = EINVAL;
                current_block = 16403181400158345396;
            } else {
                mdb_cursors_close(txn, 0 as libc::c_int as libc::c_uint);
                if (*((*txn).mt_u.dirty_list).offset(0 as libc::c_int as isize)).mid == 0
                    && (*txn).mt_flags & (MDB_TXN_DIRTY | MDB_TXN_SPILLS) as libc::c_uint == 0
                {
                    current_block = 13661098578361514953;
                } else {
                    if (*txn).mt_numdbs > CORE_DBS as MDB_dbi {
                        let mut mc = MDB_cursor {
                            mc_next: 0 as *mut MDB_cursor,
                            mc_backup: 0 as *mut MDB_cursor,
                            mc_xcursor: 0 as *mut MDB_xcursor,
                            mc_txn: 0 as *mut MDB_txn,
                            mc_dbi: 0,
                            mc_db: 0 as *mut MDB_db,
                            mc_dbx: 0 as *mut MDB_dbx,
                            mc_dbflag: 0 as *mut libc::c_uchar,
                            mc_snum: 0,
                            mc_top: 0,
                            mc_flags: 0,
                            mc_pg: [0 as *mut MDB_page; 32],
                            mc_ki: [0; 32],
                        };
                        let mut i_0: MDB_dbi = 0;
                        let mut data = MDB_val {
                            mv_size: 0,
                            mv_data: 0 as *mut libc::c_void,
                        };
                        data.mv_size = ::core::mem::size_of::<MDB_db>() as libc::c_ulong;
                        mdb_cursor_init(
                            &mut mc,
                            txn,
                            MAIN_DBI as MDB_dbi,
                            NULL as *mut MDB_xcursor,
                        );
                        i_0 = CORE_DBS as MDB_dbi;
                        loop {
                            if !(i_0 < (*txn).mt_numdbs) {
                                current_block = 8656139126282042408;
                                break;
                            }
                            if *((*txn).mt_dbflags).offset(i_0 as isize) as libc::c_int & DB_DIRTY
                                != 0
                            {
                                if *((*txn).mt_dbiseqs).offset(i_0 as isize)
                                    != *((*(*txn).mt_env).me_dbiseqs).offset(i_0 as isize)
                                {
                                    rc = MDB_BAD_DBI;
                                    current_block = 16403181400158345396;
                                    break;
                                } else {
                                    data.mv_data = &mut *((*txn).mt_dbs).offset(i_0 as isize)
                                        as *mut MDB_db
                                        as *mut libc::c_void;
                                    rc = _mdb_cursor_put(
                                        &mut mc,
                                        &mut (*((*txn).mt_dbxs).offset(i_0 as isize)).md_name,
                                        &mut data,
                                        F_SUBDATA as libc::c_uint,
                                    );
                                    if rc != 0 {
                                        current_block = 16403181400158345396;
                                        break;
                                    }
                                }
                            }
                            i_0 = i_0.wrapping_add(1);
                            i_0;
                        }
                    } else {
                        current_block = 8656139126282042408;
                    }
                    match current_block {
                        16403181400158345396 => {}
                        _ => {
                            rc = mdb_freelist_save(txn);
                            if rc != 0 {
                                current_block = 16403181400158345396;
                            } else {
                                mdb_midl_free((*env).me_pgstate.mf_pghead);
                                (*env).me_pgstate.mf_pghead = NULL as *mut pgno_t;
                                mdb_midl_shrink(&mut (*txn).mt_free_pgs);
                                rc = mdb_page_flush(txn, 0 as libc::c_int);
                                if rc != 0 {
                                    current_block = 16403181400158345396;
                                } else if !((*txn).mt_flags
                                    & 0x10000 as libc::c_int as libc::c_uint
                                    == 0x10000 as libc::c_int as libc::c_uint)
                                    && {
                                        rc = mdb_env_sync0(
                                            env,
                                            0 as libc::c_int,
                                            (*txn).mt_next_pgno,
                                        );
                                        rc != 0
                                    }
                                {
                                    current_block = 16403181400158345396;
                                } else {
                                    rc = mdb_env_write_meta(txn);
                                    if rc != 0 {
                                        current_block = 16403181400158345396;
                                    } else {
                                        end_mode = (MDB_END_COMMITTED as libc::c_int
                                            | MDB_END_UPDATE)
                                            as libc::c_uint;
                                        if (*env).me_flags & MDB_PREVSNAPSHOT as uint32_t != 0 {
                                            if (*env).me_flags & MDB_NOLOCK as uint32_t == 0 {
                                                let mut excl: libc::c_int = 0;
                                                rc = mdb_env_share_locks(env, &mut excl);
                                                if rc != 0 {
                                                    current_block = 16403181400158345396;
                                                } else {
                                                    current_block = 1604201581803946138;
                                                }
                                            } else {
                                                current_block = 1604201581803946138;
                                            }
                                            match current_block {
                                                16403181400158345396 => {}
                                                _ => {
                                                    (*env).me_flags ^= MDB_PREVSNAPSHOT as uint32_t;
                                                    current_block = 13661098578361514953;
                                                }
                                            }
                                        } else {
                                            current_block = 13661098578361514953;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                16403181400158345396 => {}
                _ => {
                    mdb_txn_end(txn, end_mode);
                    return MDB_SUCCESS;
                }
            }
        }
        _ => {}
    }
    _mdb_txn_abort(txn);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_txn_commit(mut txn: *mut MDB_txn) -> libc::c_int {
    return _mdb_txn_commit(txn);
}
#[cold]
unsafe extern "C" fn mdb_env_read_header(
    mut env: *mut MDB_env,
    mut prev: libc::c_int,
    mut meta: *mut MDB_meta,
) -> libc::c_int {
    let mut pbuf = MDB_metabuf {
        mb_page: MDB_page {
            mp_p: C2RustUnnamed_1 { p_pgno: 0 },
            mp_pad: 0,
            mp_flags: 0,
            mp_pb: C2RustUnnamed {
                pb: C2RustUnnamed_0 {
                    pb_lower: 0,
                    pb_upper: 0,
                },
            },
            mp_ptrs: [0; 0],
        },
    };
    let mut p = 0 as *mut MDB_page;
    let mut m = 0 as *mut MDB_meta;
    let mut i: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    off = 0 as libc::c_int;
    i = off;
    while i < NUM_METAS {
        rc = pread(
            (*env).me_fd,
            &mut pbuf as *mut MDB_metabuf as *mut libc::c_void,
            Size as libc::c_int as size_t,
            off as off_t,
        ) as libc::c_int;
        if rc != Size as libc::c_int {
            if rc == 0 as libc::c_int && off == 0 as libc::c_int {
                return ENOENT;
            }
            rc = if rc < 0 as libc::c_int {
                *__error()
            } else {
                MDB_INVALID
            };
            return rc;
        }
        p = &mut pbuf as *mut MDB_metabuf as *mut MDB_page;
        if !((*p).mp_flags as libc::c_int & 0x8 as libc::c_int == 0x8 as libc::c_int) {
            return MDB_INVALID;
        }
        m = (p as *mut libc::c_char).offset(PAGEHDRSZ as libc::c_uint as isize) as *mut libc::c_void
            as *mut MDB_meta;
        if (*m).mm_magic != MDB_MAGIC {
            return MDB_INVALID;
        }
        if (*m).mm_version
            != (if 0 as libc::c_int != 0 {
                999 as libc::c_int
            } else {
                1 as libc::c_int
            }) as uint32_t
        {
            return MDB_VERSION_MISMATCH;
        }
        if off == 0 as libc::c_int
            || (if prev != 0 {
                ((*m).mm_txnid < (*meta).mm_txnid) as libc::c_int
            } else {
                ((*m).mm_txnid > (*meta).mm_txnid) as libc::c_int
            }) != 0
        {
            *meta = *m;
        }
        i += 1;
        i;
        off = (off as uint32_t).wrapping_add((*meta).mm_dbs[FREE_DBI as usize].md_pad)
            as libc::c_int as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[cold]
unsafe extern "C" fn mdb_env_init_meta0(mut env: *mut MDB_env, mut meta: *mut MDB_meta) {
    (*meta).mm_magic = MDB_MAGIC;
    (*meta).mm_version = (if 0 as libc::c_int != 0 {
        999 as libc::c_int
    } else {
        1 as libc::c_int
    }) as uint32_t;
    (*meta).mm_mapsize = (*env).me_mapsize;
    (*meta).mm_dbs[FREE_DBI as usize].md_pad = (*env).me_psize;
    (*meta).mm_last_pg = (NUM_METAS - 1 as libc::c_int) as pgno_t;
    (*meta).mm_dbs[FREE_DBI as usize].md_flags =
        ((*env).me_flags & 0xffff as libc::c_int as uint32_t) as uint16_t;
    (*meta).mm_dbs[FREE_DBI as usize].md_flags =
        ((*meta).mm_dbs[FREE_DBI as usize].md_flags as libc::c_int | MDB_INTEGERKEY) as uint16_t;
    (*meta).mm_dbs[FREE_DBI as usize].md_root = P_INVALID;
    (*meta).mm_dbs[MAIN_DBI as usize].md_root = P_INVALID;
}
#[cold]
unsafe extern "C" fn mdb_env_init_meta(
    mut env: *mut MDB_env,
    mut meta: *mut MDB_meta,
) -> libc::c_int {
    let mut p = 0 as *mut MDB_page;
    let mut q = 0 as *mut MDB_page;
    let mut rc: libc::c_int = 0;
    let mut psize: libc::c_uint = 0;
    let mut len: libc::c_int = 0;
    psize = (*env).me_psize;
    p = calloc(NUM_METAS as libc::c_ulong, psize as libc::c_ulong) as *mut MDB_page;
    if p.is_null() {
        return ENOMEM;
    }
    (*p).mp_p.p_pgno = 0 as libc::c_int as pgno_t;
    (*p).mp_flags = P_META as uint16_t;
    *((p as *mut libc::c_char).offset(PAGEHDRSZ as libc::c_uint as isize) as *mut libc::c_void
        as *mut MDB_meta) = *meta;
    q = (p as *mut libc::c_char).offset(psize as isize) as *mut MDB_page;
    (*q).mp_p.p_pgno = 1 as libc::c_int as pgno_t;
    (*q).mp_flags = P_META as uint16_t;
    *((q as *mut libc::c_char).offset(PAGEHDRSZ as libc::c_uint as isize) as *mut libc::c_void
        as *mut MDB_meta) = *meta;
    loop {
        len = pwrite(
            (*env).me_fd,
            p as *const libc::c_void,
            psize.wrapping_mul(2 as libc::c_int as libc::c_uint) as size_t,
            0 as libc::c_int as off_t,
        ) as libc::c_int;
        if len == -(1 as libc::c_int) && *__error() == EINTR {
            continue;
        }
        rc = (len >= 0 as libc::c_int) as libc::c_int;
        break;
    }
    if rc == 0 {
        rc = *__error();
    } else if len as libc::c_uint == psize.wrapping_mul(NUM_METAS as libc::c_uint) {
        rc = MDB_SUCCESS;
    } else {
        rc = ENOSPC;
    }
    free(p as *mut libc::c_void);
    return rc;
}
unsafe extern "C" fn mdb_env_write_meta(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut current_block: u64;
    let mut env = 0 as *mut MDB_env;
    let mut meta = MDB_meta {
        mm_magic: 0,
        mm_version: 0,
        mm_address: 0 as *mut libc::c_void,
        mm_mapsize: 0,
        mm_dbs: [MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        }; 2],
        mm_last_pg: 0,
        mm_txnid: 0,
    };
    let mut metab = MDB_meta {
        mm_magic: 0,
        mm_version: 0,
        mm_address: 0 as *mut libc::c_void,
        mm_mapsize: 0,
        mm_dbs: [MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        }; 2],
        mm_last_pg: 0,
        mm_txnid: 0,
    };
    let mut mp = 0 as *mut MDB_meta;
    let mut flags: libc::c_uint = 0;
    let mut mapsize: mdb_size_t = 0;
    let mut off: off_t = 0;
    let mut rc: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut toggle: libc::c_int = 0;
    let mut ptr = 0 as *mut libc::c_char;
    let mut mfd: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    toggle = ((*txn).mt_txnid & 1 as libc::c_int as txnid_t) as libc::c_int;
    env = (*txn).mt_env;
    flags = (*txn).mt_flags | (*env).me_flags;
    mp = (*env).me_metas[toggle as usize];
    mapsize = (*(*env).me_metas[(toggle ^ 1 as libc::c_int) as usize]).mm_mapsize;
    if mapsize < (*env).me_mapsize {
        mapsize = (*env).me_mapsize;
    }
    if flags & MDB_WRITEMAP as libc::c_uint != 0 {
        (*mp).mm_mapsize = mapsize;
        (*mp).mm_dbs[FREE_DBI as usize] = *((*txn).mt_dbs).offset(FREE_DBI as isize);
        (*mp).mm_dbs[MAIN_DBI as usize] = *((*txn).mt_dbs).offset(MAIN_DBI as isize);
        (*mp).mm_last_pg = ((*txn).mt_next_pgno).wrapping_sub(1 as libc::c_int as pgno_t);
        ::core::ptr::write_volatile(&mut (*mp).mm_txnid as *mut txnid_t, (*txn).mt_txnid);
        if flags & (MDB_NOMETASYNC | MDB_NOSYNC) as libc::c_uint == 0 {
            let mut meta_size = (*env).me_psize;
            rc = if (*env).me_flags & MDB_MAPASYNC as uint32_t != 0 {
                MS_ASYNC
            } else {
                MS_SYNC
            };
            ptr = (mp as *mut libc::c_char).offset(-(PAGEHDRSZ as libc::c_uint as isize));
            r2 = (ptr.offset_from((*env).me_map) as libc::c_long
                & ((*env).me_os_psize).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as libc::c_long) as libc::c_int;
            ptr = ptr.offset(-(r2 as isize));
            meta_size = meta_size.wrapping_add(r2 as libc::c_uint);
            if msync(ptr as *mut libc::c_void, meta_size as size_t, rc) != 0 {
                rc = *__error();
                current_block = 6372744105825038606;
            } else {
                current_block = 17040666968784172583;
            }
        } else {
            current_block = 17040666968784172583;
        }
    } else {
        ::core::ptr::write_volatile(&mut metab.mm_txnid as *mut txnid_t, (*mp).mm_txnid);
        metab.mm_last_pg = (*mp).mm_last_pg;
        meta.mm_mapsize = mapsize;
        meta.mm_dbs[FREE_DBI as usize] = *((*txn).mt_dbs).offset(FREE_DBI as isize);
        meta.mm_dbs[MAIN_DBI as usize] = *((*txn).mt_dbs).offset(MAIN_DBI as isize);
        meta.mm_last_pg = ((*txn).mt_next_pgno).wrapping_sub(1 as libc::c_int as pgno_t);
        ::core::ptr::write_volatile(&mut meta.mm_txnid as *mut txnid_t, (*txn).mt_txnid);
        off = mem::offset_of!(MDB_meta, mm_mapsize) as libc::off_t;
        ptr = (&mut meta as *mut MDB_meta as *mut libc::c_char).offset(off as isize);
        len = (::core::mem::size_of::<MDB_meta>() as libc::c_ulong as libc::c_ulonglong)
            .wrapping_sub(off as libc::c_ulonglong) as libc::c_int;
        off += (mp as *mut libc::c_char).offset_from((*env).me_map) as libc::c_long as off_t;
        mfd = if flags & (MDB_NOSYNC | MDB_NOMETASYNC) as libc::c_uint != 0 {
            (*env).me_fd
        } else {
            (*env).me_mfd
        };
        loop {
            rc = pwrite(mfd, ptr as *const libc::c_void, len as size_t, off) as libc::c_int;
            if !(rc != len) {
                current_block = 17040666968784172583;
                break;
            }
            rc = if rc < 0 as libc::c_int {
                *__error()
            } else {
                EIO
            };
            if rc == EINTR {
                continue;
            }
            meta.mm_last_pg = metab.mm_last_pg;
            ::core::ptr::write_volatile(&mut meta.mm_txnid as *mut txnid_t, metab.mm_txnid);
            r2 =
                pwrite((*env).me_fd, ptr as *const libc::c_void, len as size_t, off) as libc::c_int;
            current_block = 6372744105825038606;
            break;
        }
    }
    match current_block {
        17040666968784172583 => {
            if !((*env).me_txns).is_null() {
                ::core::ptr::write_volatile(
                    &mut (*(*env).me_txns).mt1.mtb.mtb_txnid as *mut txnid_t,
                    (*txn).mt_txnid,
                );
            }
            return MDB_SUCCESS;
        }
        _ => {
            (*env).me_flags |= MDB_FATAL_ERROR;
            return rc;
        }
    };
}
unsafe extern "C" fn mdb_env_pick_meta(mut env: *const MDB_env) -> *mut MDB_meta {
    let mut metas = ((*env).me_metas).as_ptr();
    return *metas.offset(
        (((**metas.offset(0 as libc::c_int as isize)).mm_txnid
            < (**metas.offset(1 as libc::c_int as isize)).mm_txnid) as libc::c_int
            ^ ((*env).me_flags & MDB_PREVSNAPSHOT as uint32_t != 0 as libc::c_int as uint32_t)
                as libc::c_int) as isize,
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_create(mut env: *mut *mut MDB_env) -> libc::c_int {
    let mut e = 0 as *mut MDB_env;
    e = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<MDB_env>() as libc::c_ulong,
    ) as *mut MDB_env;
    if e.is_null() {
        return ENOMEM;
    }
    (*e).me_maxreaders = DEFAULT_READERS as libc::c_uint;
    (*e).me_numdbs = CORE_DBS as MDB_dbi;
    (*e).me_maxdbs = (*e).me_numdbs;
    (*e).me_fd = INVALID_HANDLE_VALUE;
    (*e).me_lfd = INVALID_HANDLE_VALUE;
    (*e).me_mfd = INVALID_HANDLE_VALUE;
    (*((*e).me_rmutex).as_mut_ptr()).semid = -(1 as libc::c_int);
    (*((*e).me_wmutex).as_mut_ptr()).semid = -(1 as libc::c_int);
    (*e).me_pid = getpid();
    (*e).me_os_psize = sysconf(_SC_PAGE_SIZE) as libc::c_uint;
    *env = e;
    return MDB_SUCCESS;
}
#[cold]
unsafe extern "C" fn mdb_env_map(
    mut env: *mut MDB_env,
    mut addr: *mut libc::c_void,
) -> libc::c_int {
    let mut p = 0 as *mut MDB_page;
    let mut flags = (*env).me_flags;
    let mut mmap_flags = MAP_SHARED;
    let mut prot = PROT_READ;
    if flags & MDB_WRITEMAP as libc::c_uint != 0 {
        prot |= PROT_WRITE;
        if ftruncate((*env).me_fd, (*env).me_mapsize as off_t) < 0 as libc::c_int {
            return *__error();
        }
    }
    (*env).me_map = mmap(
        addr,
        (*env).me_mapsize,
        prot,
        mmap_flags,
        (*env).me_fd,
        0 as libc::c_int as off_t,
    ) as *mut libc::c_char;
    if (*env).me_map == MAP_FAILED as *mut libc::c_char {
        (*env).me_map = NULL as *mut libc::c_char;
        return *__error();
    }
    if flags & MDB_NORDAHEAD as libc::c_uint != 0 {
        madvise(
            (*env).me_map as *mut libc::c_void,
            (*env).me_mapsize,
            MADV_RANDOM,
        );
    }
    if !addr.is_null() && (*env).me_map != addr as *mut libc::c_char {
        return EBUSY;
    }
    p = (*env).me_map as *mut MDB_page;
    (*env).me_metas[0 as libc::c_int as usize] = (p as *mut libc::c_char)
        .offset(PAGEHDRSZ as libc::c_uint as isize)
        as *mut libc::c_void as *mut MDB_meta;
    (*env).me_metas[1 as libc::c_int as usize] =
        ((*env).me_metas[0 as libc::c_int as usize] as *mut libc::c_char)
            .offset((*env).me_psize as isize) as *mut MDB_meta;
    return MDB_SUCCESS;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_set_mapsize(
    mut env: *mut MDB_env,
    mut size: mdb_size_t,
) -> libc::c_int {
    if !((*env).me_map).is_null() {
        let mut meta = 0 as *mut MDB_meta;
        let mut old = 0 as *mut libc::c_void;
        let mut rc: libc::c_int = 0;
        if !((*env).me_txn).is_null() {
            return EINVAL;
        }
        meta = mdb_env_pick_meta(env);
        if size == 0 {
            size = (*meta).mm_mapsize;
        }
        let mut minsize = ((*meta).mm_last_pg).wrapping_add(1 as libc::c_int as pgno_t)
            * (*env).me_psize as pgno_t;
        if size < minsize {
            size = minsize;
        }
        munmap((*env).me_map as *mut libc::c_void, (*env).me_mapsize);
        (*env).me_mapsize = size;
        old = (if (*env).me_flags & MDB_FIXEDMAP as uint32_t != 0 {
            (*env).me_map
        } else {
            NULL as *mut libc::c_char
        }) as *mut libc::c_void;
        rc = mdb_env_map(env, old);
        if rc != 0 {
            return rc;
        }
    }
    (*env).me_mapsize = size;
    if (*env).me_psize != 0 {
        (*env).me_maxpg = (*env).me_mapsize / (*env).me_psize as mdb_size_t;
    }
    return MDB_SUCCESS;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_set_maxdbs(
    mut env: *mut MDB_env,
    mut dbs: MDB_dbi,
) -> libc::c_int {
    if !((*env).me_map).is_null() {
        return EINVAL;
    }
    (*env).me_maxdbs = dbs.wrapping_add(CORE_DBS as MDB_dbi);
    return MDB_SUCCESS;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_set_maxreaders(
    mut env: *mut MDB_env,
    mut readers: libc::c_uint,
) -> libc::c_int {
    if !((*env).me_map).is_null() || readers < 1 as libc::c_int as libc::c_uint {
        return EINVAL;
    }
    (*env).me_maxreaders = readers;
    return MDB_SUCCESS;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_get_maxreaders(
    mut env: *mut MDB_env,
    mut readers: *mut libc::c_uint,
) -> libc::c_int {
    if env.is_null() || readers.is_null() {
        return EINVAL;
    }
    *readers = (*env).me_maxreaders;
    return MDB_SUCCESS;
}
#[cold]
unsafe extern "C" fn mdb_fsize(mut fd: libc::c_int, mut size: *mut mdb_size_t) -> libc::c_int {
    let mut st = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_birthtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    if fstat(fd, &mut st) != 0 {
        return *__error();
    }
    *size = st.st_size as mdb_size_t;
    return MDB_SUCCESS;
}
pub const mdb_name_cpy: unsafe extern "C" fn(
    *mut libc::c_char,
    *const libc::c_char,
) -> *mut libc::c_char = strcpy;
static mut mdb_suffixes: [[*const mdb_nchar_t; 2]; 2] = [
    [
        b"/data.mdb\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"/lock.mdb\0" as *const u8 as *const libc::c_char,
        b"-lock\0" as *const u8 as *const libc::c_char,
    ],
];
pub const MDB_SUFFLEN: libc::c_int = 9 as libc::c_int;
#[cold]
unsafe extern "C" fn mdb_fname_init(
    mut path: *const libc::c_char,
    mut envflags: libc::c_uint,
    mut fname: *mut MDB_name,
) -> libc::c_int {
    let mut no_suffix = (envflags
        & (0x4000 as libc::c_int | 0x400000 as libc::c_int) as libc::c_uint
        == (0x4000 as libc::c_int | 0x400000 as libc::c_int) as libc::c_uint)
        as libc::c_int;
    (*fname).mn_alloced = 0 as libc::c_int;
    (*fname).mn_len = strlen(path) as libc::c_int;
    if no_suffix != 0 {
        (*fname).mn_val = path as *mut libc::c_char;
    } else {
        (*fname).mn_val =
            malloc(((*fname).mn_len + MDB_SUFFLEN + 1 as libc::c_int) as libc::c_ulong)
                as *mut mdb_nchar_t;
        if !((*fname).mn_val).is_null() {
            (*fname).mn_alloced = 1 as libc::c_int;
            strcpy((*fname).mn_val, path);
        } else {
            return ENOMEM;
        }
    }
    return MDB_SUCCESS;
}
pub const MDB_CLOEXEC: libc::c_int = O_CLOEXEC;
#[cold]
unsafe extern "C" fn mdb_fopen(
    mut env: *const MDB_env,
    mut fname: *mut MDB_name,
    mut which: mdb_fopen_type,
    mut mode: mdb_mode_t,
    mut res: *mut libc::c_int,
) -> libc::c_int {
    let mut rc = MDB_SUCCESS;
    let mut fd: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    if (*fname).mn_alloced != 0 {
        strcpy(
            ((*fname).mn_val).offset((*fname).mn_len as isize),
            mdb_suffixes[(which as libc::c_uint == MDB_O_LOCKS as libc::c_int as libc::c_uint)
                as libc::c_int as usize][((*env).me_flags
                & 0x4000 as libc::c_int as uint32_t
                == 0x4000 as libc::c_int as uint32_t)
                as libc::c_int as usize],
        );
    }
    fd = open(
        (*fname).mn_val,
        (which as libc::c_uint & MDB_O_MASK as libc::c_int as libc::c_uint) as libc::c_int,
        mode as libc::c_int,
    );
    if fd == INVALID_HANDLE_VALUE {
        rc = *__error();
    } else {
        if which as libc::c_uint != MDB_O_RDONLY as libc::c_int as libc::c_uint
            && which as libc::c_uint != MDB_O_RDWR as libc::c_int as libc::c_uint
        {
            if MDB_CLOEXEC == 0 && {
                flags = fcntl(fd, F_GETFD);
                flags != -(1 as libc::c_int)
            } {
                fcntl(fd, F_SETFD, flags | FD_CLOEXEC);
            }
        }
        if which as libc::c_uint == MDB_O_COPY as libc::c_int as libc::c_uint
            && (*env).me_psize >= (*env).me_os_psize
        {
            fcntl(fd, F_NOCACHE, 1 as libc::c_int);
        }
    }
    *res = fd;
    return rc;
}
#[cold]
unsafe extern "C" fn mdb_env_open2(mut env: *mut MDB_env, mut prev: libc::c_int) -> libc::c_int {
    let mut flags = (*env).me_flags;
    let mut i: libc::c_int = 0;
    let mut newenv = 0 as libc::c_int;
    let mut rc: libc::c_int = 0;
    let mut meta = MDB_meta {
        mm_magic: 0,
        mm_version: 0,
        mm_address: 0 as *mut libc::c_void,
        mm_mapsize: 0,
        mm_dbs: [MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        }; 2],
        mm_last_pg: 0,
        mm_txnid: 0,
    };
    i = mdb_env_read_header(env, prev, &mut meta);
    if i != 0 as libc::c_int {
        if i != ENOENT {
            return i;
        }
        newenv = 1 as libc::c_int;
        (*env).me_psize = (*env).me_os_psize;
        if (*env).me_psize
            > (if (if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) != 0
            {
                0x10000 as libc::c_int
            } else {
                0x8000 as libc::c_int
            }) as libc::c_uint
        {
            (*env).me_psize = (if if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            } != 0
            {
                0x10000 as libc::c_int
            } else {
                0x8000 as libc::c_int
            }) as libc::c_uint;
        }
        memset(
            &mut meta as *mut MDB_meta as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<MDB_meta>() as libc::c_ulong,
        );
        mdb_env_init_meta0(env, &mut meta);
        meta.mm_mapsize = DEFAULT_MAPSIZE as mdb_size_t;
    } else {
        (*env).me_psize = meta.mm_dbs[FREE_DBI as usize].md_pad;
    }
    if (*env).me_mapsize == 0 {
        (*env).me_mapsize = meta.mm_mapsize;
    }
    let mut minsize = (meta.mm_last_pg).wrapping_add(1 as libc::c_int as pgno_t)
        * meta.mm_dbs[FREE_DBI as usize].md_pad as pgno_t;
    if (*env).me_mapsize < minsize {
        (*env).me_mapsize = minsize;
    }
    meta.mm_mapsize = (*env).me_mapsize;
    if newenv != 0 && flags & MDB_FIXEDMAP as libc::c_uint == 0 {
        rc = mdb_env_init_meta(env, &mut meta);
        if rc != 0 {
            return rc;
        }
        newenv = 0 as libc::c_int;
    }
    rc = mdb_env_map(
        env,
        if flags & MDB_FIXEDMAP as libc::c_uint != 0 {
            meta.mm_address
        } else {
            NULL as *mut libc::c_void
        },
    );
    if rc != 0 {
        return rc;
    }
    if newenv != 0 {
        if flags & MDB_FIXEDMAP as libc::c_uint != 0 {
            meta.mm_address = (*env).me_map as *mut libc::c_void;
        }
        i = mdb_env_init_meta(env, &mut meta);
        if i != MDB_SUCCESS {
            return i;
        }
    }
    (*env).me_maxfree_1pg = (((*env).me_psize).wrapping_sub(PAGEHDRSZ as libc::c_uint)
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<pgno_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    (*env).me_nodemax = ((((*env).me_psize)
        .wrapping_sub(PAGEHDRSZ as libc::c_uint)
        .wrapping_div(MDB_MINKEYS as libc::c_uint)
        & -(2 as libc::c_int) as libc::c_uint) as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<indx_t>() as libc::c_ulong)
        as libc::c_uint;
    (*env).me_maxpg = (*env).me_mapsize / (*env).me_psize as mdb_size_t;
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_env_reader_dest(mut ptr: *mut libc::c_void) {
    let mut reader = ptr as *mut MDB_reader;
    if (*reader).mru.mrx.mrb_pid == getpid() {
        ::core::ptr::write_volatile(
            &mut (*reader).mru.mrx.mrb_pid as *mut pid_t,
            0 as libc::c_int,
        );
    }
}
#[cold]
unsafe extern "C" fn mdb_env_share_locks(
    mut env: *mut MDB_env,
    mut excl: *mut libc::c_int,
) -> libc::c_int {
    let mut rc = 0 as libc::c_int;
    let mut meta = mdb_env_pick_meta(env);
    ::core::ptr::write_volatile(
        &mut (*(*env).me_txns).mt1.mtb.mtb_txnid as *mut txnid_t,
        (*meta).mm_txnid,
    );
    let mut lock_info = flock {
        l_start: 0,
        l_len: 0,
        l_pid: 0,
        l_type: 0,
        l_whence: 0,
    };
    memset(
        &mut lock_info as *mut flock as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<flock>() as libc::c_ulong,
    );
    lock_info.l_type = F_RDLCK as libc::c_short;
    lock_info.l_whence = SEEK_SET as libc::c_short;
    lock_info.l_start = 0 as libc::c_int as off_t;
    lock_info.l_len = 1 as libc::c_int as off_t;
    loop {
        rc = fcntl((*env).me_lfd, F_SETLK, &mut lock_info as *mut flock);
        if !(rc != 0 && {
            rc = *__error();
            rc == EINTR
        }) {
            break;
        }
    }
    *excl = if rc != 0 {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    return rc;
}
#[cold]
unsafe extern "C" fn mdb_env_excl_lock(
    mut env: *mut MDB_env,
    mut excl: *mut libc::c_int,
) -> libc::c_int {
    let mut rc = 0 as libc::c_int;
    let mut lock_info = flock {
        l_start: 0,
        l_len: 0,
        l_pid: 0,
        l_type: 0,
        l_whence: 0,
    };
    memset(
        &mut lock_info as *mut flock as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<flock>() as libc::c_ulong,
    );
    lock_info.l_type = F_WRLCK as libc::c_short;
    lock_info.l_whence = SEEK_SET as libc::c_short;
    lock_info.l_start = 0 as libc::c_int as off_t;
    lock_info.l_len = 1 as libc::c_int as off_t;
    loop {
        rc = fcntl((*env).me_lfd, F_SETLK, &mut lock_info as *mut flock);
        if !(rc != 0 && {
            rc = *__error();
            rc == EINTR
        }) {
            break;
        }
    }
    if rc == 0 {
        *excl = 1 as libc::c_int;
    } else if *excl < 0 as libc::c_int {
        lock_info.l_type = F_RDLCK as libc::c_short;
        loop {
            rc = fcntl((*env).me_lfd, F_SETLKW, &mut lock_info as *mut flock);
            if !(rc != 0 && {
                rc = *__error();
                rc == EINTR
            }) {
                break;
            }
        }
        if rc == 0 as libc::c_int {
            *excl = 0 as libc::c_int;
        }
    }
    return rc;
}
#[cold]
unsafe extern "C" fn mdb_env_setup_locks(
    mut env: *mut MDB_env,
    mut fname: *mut MDB_name,
    mut mode: libc::c_int,
    mut excl: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut semid: libc::c_int = 0;
    let mut semu = semun { val: 0 };
    let mut rc: libc::c_int = 0;
    let mut size: off_t = 0;
    let mut rsize: off_t = 0;
    rc = mdb_fopen(
        env,
        fname,
        MDB_O_LOCKS,
        mode as mdb_mode_t,
        &mut (*env).me_lfd,
    );
    if rc != 0 {
        if rc == MDB_ERRCODE_ROFS && (*env).me_flags & MDB_RDONLY as uint32_t != 0 {
            return MDB_SUCCESS;
        }
    } else {
        if (*env).me_flags & MDB_NOTLS as uint32_t == 0 {
            rc = pthread_key_create(
                &mut (*env).me_txkey,
                Some(mdb_env_reader_dest as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            );
            if rc != 0 {
                current_block = 10166034227075777411;
            } else {
                (*env).me_flags |= MDB_ENV_TXKEY;
                current_block = 5720623009719927633;
            }
        } else {
            current_block = 5720623009719927633;
        }
        match current_block {
            10166034227075777411 => {}
            _ => {
                rc = mdb_env_excl_lock(env, excl);
                if !(rc != 0) {
                    size = lseek((*env).me_lfd, 0 as libc::c_int as off_t, SEEK_END);
                    if size == -(1 as libc::c_int) as off_t {
                        current_block = 7057229242290363491;
                    } else {
                        rsize = (((*env).me_maxreaders)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<MDB_reader>() as libc::c_ulong)
                            .wrapping_add(::core::mem::size_of::<MDB_txninfo>() as libc::c_ulong)
                            as off_t;
                        if size < rsize && *excl > 0 as libc::c_int {
                            if ftruncate((*env).me_lfd, rsize) != 0 as libc::c_int {
                                current_block = 7057229242290363491;
                            } else {
                                current_block = 17833034027772472439;
                            }
                        } else {
                            rsize = size;
                            size =
                                (rsize as libc::c_ulonglong).wrapping_sub(::core::mem::size_of::<
                                    MDB_txninfo,
                                >(
                                )
                                    as libc::c_ulong
                                    as libc::c_ulonglong) as off_t;
                            (*env).me_maxreaders = (size as libc::c_ulonglong)
                                .wrapping_div(::core::mem::size_of::<MDB_reader>() as libc::c_ulong
                                    as libc::c_ulonglong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                as libc::c_uint;
                            current_block = 17833034027772472439;
                        }
                        match current_block {
                            7057229242290363491 => {}
                            _ => {
                                let mut m = mmap(
                                    NULL as *mut libc::c_void,
                                    rsize as size_t,
                                    PROT_READ | PROT_WRITE,
                                    MAP_SHARED,
                                    (*env).me_lfd,
                                    0 as libc::c_int as off_t,
                                );
                                if m == MAP_FAILED as *mut libc::c_void {
                                    current_block = 7057229242290363491;
                                } else {
                                    (*env).me_txns = m as *mut MDB_txninfo;
                                    if *excl > 0 as libc::c_int {
                                        let mut vals: [libc::c_ushort; 2] = [
                                            1 as libc::c_int as libc::c_ushort,
                                            1 as libc::c_int as libc::c_ushort,
                                        ];
                                        let mut key = ftok((*fname).mn_val, 'M' as i32);
                                        if key == -(1 as libc::c_int) {
                                            current_block = 7057229242290363491;
                                        } else {
                                            semid = semget(
                                                key,
                                                2 as libc::c_int,
                                                mode & 0o777 as libc::c_int | IPC_CREAT,
                                            );
                                            if semid < 0 as libc::c_int {
                                                current_block = 7057229242290363491;
                                            } else {
                                                semu.array = vals.as_mut_ptr();
                                                if semctl(semid, 0 as libc::c_int, SETALL, semu)
                                                    < 0 as libc::c_int
                                                {
                                                    current_block = 7057229242290363491;
                                                } else {
                                                    (*(*env).me_txns).mt1.mtb.mtb_semid = semid;
                                                    (*(*env).me_txns).mt1.mtb.mtb_rlocked =
                                                        0 as libc::c_int;
                                                    (*(*env).me_txns).mt2.mt2_wlocked =
                                                        0 as libc::c_int;
                                                    (*(*env).me_txns).mt1.mtb.mtb_magic = MDB_MAGIC;
                                                    (*(*env).me_txns).mt1.mtb.mtb_format = ((if 0
                                                        as libc::c_int
                                                        != 0
                                                    {
                                                        999 as libc::c_int
                                                    } else {
                                                        2 as libc::c_int
                                                    })
                                                        as libc::c_uint)
                                                        .wrapping_rem(
                                                            (1 as libc::c_uint)
                                                                << MDB_LOCK_VERSION_BITS,
                                                        )
                                                        .wrapping_add(
                                                            (MDB_lock_desc as libc::c_int
                                                                as libc::c_uint)
                                                                .wrapping_mul(
                                                                    (1 as libc::c_uint)
                                                                        << MDB_LOCK_VERSION_BITS,
                                                                ),
                                                        );
                                                    ::core::ptr::write_volatile(
                                                        &mut (*(*env).me_txns).mt1.mtb.mtb_txnid
                                                            as *mut txnid_t,
                                                        0 as libc::c_int as txnid_t,
                                                    );
                                                    ::core::ptr::write_volatile(
                                                        &mut (*(*env).me_txns)
                                                            .mt1
                                                            .mtb
                                                            .mtb_numreaders
                                                            as *mut libc::c_uint,
                                                        0 as libc::c_int as libc::c_uint,
                                                    );
                                                    current_block = 5330834795799507926;
                                                }
                                            }
                                        }
                                    } else {
                                        let mut buf = __semid_ds_new {
                                            sem_perm: ipc_perm {
                                                uid: 0,
                                                gid: 0,
                                                cuid: 0,
                                                cgid: 0,
                                                mode: 0,
                                                _seq: 0,
                                                _key: 0,
                                            },
                                            sem_base: 0,
                                            sem_nsems: 0,
                                            sem_otime: 0,
                                            sem_pad1: 0,
                                            sem_ctime: 0,
                                            sem_pad2: 0,
                                            sem_pad3: [0; 4],
                                        };
                                        if (*(*env).me_txns).mt1.mtb.mtb_magic != MDB_MAGIC {
                                            rc = MDB_INVALID;
                                            current_block = 10166034227075777411;
                                        } else if (*(*env).me_txns).mt1.mtb.mtb_format
                                            != ((if 0 as libc::c_int != 0 {
                                                999 as libc::c_int
                                            } else {
                                                2 as libc::c_int
                                            })
                                                as libc::c_uint)
                                                .wrapping_rem(
                                                    (1 as libc::c_uint) << MDB_LOCK_VERSION_BITS,
                                                )
                                                .wrapping_add(
                                                    (MDB_lock_desc as libc::c_int as libc::c_uint)
                                                        .wrapping_mul(
                                                            (1 as libc::c_uint)
                                                                << MDB_LOCK_VERSION_BITS,
                                                        ),
                                                )
                                        {
                                            rc = MDB_VERSION_MISMATCH;
                                            current_block = 10166034227075777411;
                                        } else {
                                            rc = *__error();
                                            if rc != 0 && rc != EACCES && rc != EAGAIN {
                                                current_block = 10166034227075777411;
                                            } else {
                                                semid = (*(*env).me_txns).mt1.mtb.mtb_semid;
                                                semu.buf = &mut buf;
                                                if semctl(semid, 0 as libc::c_int, IPC_STAT, semu)
                                                    < 0 as libc::c_int
                                                {
                                                    current_block = 7057229242290363491;
                                                } else if semctl(
                                                    semid,
                                                    0 as libc::c_int,
                                                    IPC_SET,
                                                    semu,
                                                ) < 0 as libc::c_int
                                                {
                                                    current_block = 7057229242290363491;
                                                } else {
                                                    current_block = 5330834795799507926;
                                                }
                                            }
                                        }
                                    }
                                    match current_block {
                                        7057229242290363491 => {}
                                        10166034227075777411 => {}
                                        _ => {
                                            (*((*env).me_rmutex).as_mut_ptr()).semid = semid;
                                            (*((*env).me_wmutex).as_mut_ptr()).semid = semid;
                                            (*((*env).me_rmutex).as_mut_ptr()).semnum =
                                                0 as libc::c_int;
                                            (*((*env).me_wmutex).as_mut_ptr()).semnum =
                                                1 as libc::c_int;
                                            let ref mut fresh17 =
                                                (*((*env).me_rmutex).as_mut_ptr()).locked;
                                            *fresh17 = &mut (*(*env).me_txns).mt1.mtb.mtb_rlocked;
                                            let ref mut fresh18 =
                                                (*((*env).me_wmutex).as_mut_ptr()).locked;
                                            *fresh18 = &mut (*(*env).me_txns).mt2.mt2_wlocked;
                                            return MDB_SUCCESS;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        10166034227075777411 => {}
                        _ => {
                            rc = *__error();
                        }
                    }
                }
            }
        }
    }
    return rc;
}
pub const MDB_ERRCODE_ROFS: libc::c_int = EROFS;
pub const CHANGEABLE: libc::c_int = MDB_NOSYNC | MDB_NOMETASYNC | MDB_MAPASYNC | MDB_NOMEMINIT;
pub const CHANGELESS: libc::c_int = MDB_FIXEDMAP
    | MDB_NOSUBDIR
    | MDB_RDONLY
    | MDB_WRITEMAP
    | MDB_NOTLS
    | MDB_NOLOCK
    | MDB_NORDAHEAD
    | MDB_PREVSNAPSHOT;

#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_open(
    mut env: *mut MDB_env,
    mut path: *const libc::c_char,
    mut flags: libc::c_uint,
    mut mode: mdb_mode_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut excl = -(1 as libc::c_int);
    let mut fname = MDB_name {
        mn_len: 0,
        mn_alloced: 0,
        mn_val: 0 as *mut mdb_nchar_t,
    };
    if (*env).me_fd != INVALID_HANDLE_VALUE
        || flags & !(CHANGEABLE | CHANGELESS) as libc::c_uint != 0
    {
        return EINVAL;
    }
    flags |= (*env).me_flags;
    rc = mdb_fname_init(path, flags, &mut fname);
    if rc != 0 {
        return rc;
    }
    flags |= MDB_ENV_ACTIVE;
    if flags & MDB_RDONLY as libc::c_uint != 0 {
        flags &= !MDB_WRITEMAP as libc::c_uint;
    } else {
        (*env).me_free_pgs = mdb_midl_alloc(MDB_IDL_UM_MAX);
        if !(!((*env).me_free_pgs).is_null() && {
            (*env).me_dirty_list = calloc(
                MDB_IDL_UM_SIZE as libc::c_ulong,
                ::core::mem::size_of::<MDB_ID2>() as libc::c_ulong,
            ) as MDB_ID2L;
            !((*env).me_dirty_list).is_null()
        }) {
            rc = ENOMEM;
        }
    }
    (*env).me_flags = flags;
    if !(rc != 0) {
        (*env).me_path = strdup(path);
        (*env).me_dbxs = calloc(
            (*env).me_maxdbs as libc::c_ulong,
            ::core::mem::size_of::<MDB_dbx>() as libc::c_ulong,
        ) as *mut MDB_dbx;
        (*env).me_dbflags = calloc(
            (*env).me_maxdbs as libc::c_ulong,
            ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
        ) as *mut uint16_t;
        (*env).me_dbiseqs = calloc(
            (*env).me_maxdbs as libc::c_ulong,
            ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong,
        ) as *mut libc::c_uint;
        if !(!((*env).me_dbxs).is_null()
            && !((*env).me_path).is_null()
            && !((*env).me_dbflags).is_null()
            && !((*env).me_dbiseqs).is_null())
        {
            rc = ENOMEM;
        } else {
            let ref mut fresh19 = (*((*env).me_dbxs).offset(FREE_DBI as isize)).md_cmp;
            *fresh19 = Some(mdb_cmp_long as MDB_cmp_func);
            if flags & (MDB_RDONLY | MDB_NOLOCK) as libc::c_uint == 0 {
                rc = mdb_env_setup_locks(env, &mut fname, mode as libc::c_int, &mut excl);
                if rc != 0 {
                    current_block = 2122094917359643297;
                } else if flags & MDB_PREVSNAPSHOT as libc::c_uint != 0 && excl == 0 {
                    rc = EAGAIN;
                    current_block = 2122094917359643297;
                } else {
                    current_block = 5783071609795492627;
                }
            } else {
                current_block = 5783071609795492627;
            }
            match current_block {
                2122094917359643297 => {}
                _ => {
                    rc = mdb_fopen(
                        env,
                        &mut fname,
                        (if flags & MDB_RDONLY as libc::c_uint != 0 {
                            MDB_O_RDONLY as libc::c_int
                        } else {
                            MDB_O_RDWR as libc::c_int
                        }) as mdb_fopen_type,
                        mode,
                        &mut (*env).me_fd,
                    );
                    if !(rc != 0) {
                        if flags & (MDB_RDONLY | MDB_NOLOCK) as libc::c_uint
                            == MDB_RDONLY as libc::c_uint
                        {
                            rc = mdb_env_setup_locks(
                                env,
                                &mut fname,
                                mode as libc::c_int,
                                &mut excl,
                            );
                            if rc != 0 {
                                current_block = 2122094917359643297;
                            } else {
                                current_block = 4068382217303356765;
                            }
                        } else {
                            current_block = 4068382217303356765;
                        }
                        match current_block {
                            2122094917359643297 => {}
                            _ => {
                                rc = mdb_env_open2(
                                    env,
                                    (flags & MDB_PREVSNAPSHOT as libc::c_uint) as libc::c_int,
                                );
                                if rc == MDB_SUCCESS {
                                    if flags & (MDB_RDONLY | MDB_WRITEMAP) as libc::c_uint == 0 {
                                        rc = mdb_fopen(
                                            env,
                                            &mut fname,
                                            MDB_O_META,
                                            mode,
                                            &mut (*env).me_mfd,
                                        );
                                        if rc != 0 {
                                            current_block = 2122094917359643297;
                                        } else {
                                            current_block = 8693738493027456495;
                                        }
                                    } else {
                                        current_block = 8693738493027456495;
                                    }
                                    match current_block {
                                        2122094917359643297 => {}
                                        _ => {
                                            if excl > 0 as libc::c_int
                                                && flags & MDB_PREVSNAPSHOT as libc::c_uint == 0
                                            {
                                                rc = mdb_env_share_locks(env, &mut excl);
                                                if rc != 0 {
                                                    current_block = 2122094917359643297;
                                                } else {
                                                    current_block = 14136749492126903395;
                                                }
                                            } else {
                                                current_block = 14136749492126903395;
                                            }
                                            match current_block {
                                                2122094917359643297 => {}
                                                _ => {
                                                    if flags & MDB_RDONLY as libc::c_uint == 0 {
                                                        let mut txn = 0 as *mut MDB_txn;
                                                        let mut tsize =
                                                            ::core::mem::size_of::<MDB_txn>()
                                                                as libc::c_ulong
                                                                as libc::c_int;
                                                        let mut size = (tsize as libc::c_ulong)
                                                            .wrapping_add(
                                                            ((*env).me_maxdbs as libc::c_ulong)
                                                                .wrapping_mul(
                                                                (::core::mem::size_of::<MDB_db>()
                                                                    as libc::c_ulong)
                                                                    .wrapping_add(
                                                                        ::core::mem::size_of::<
                                                                            *mut MDB_cursor,
                                                                        >(
                                                                        )
                                                                            as libc::c_ulong,
                                                                    )
                                                                    .wrapping_add(
                                                                        ::core::mem::size_of::<
                                                                            libc::c_uint,
                                                                        >(
                                                                        )
                                                                            as libc::c_ulong,
                                                                    )
                                                                    .wrapping_add(
                                                                        1 as libc::c_int
                                                                            as libc::c_ulong,
                                                                    ),
                                                            ),
                                                        )
                                                            as libc::c_int;
                                                        (*env).me_pbuf = calloc(
                                                            1 as libc::c_int as libc::c_ulong,
                                                            (*env).me_psize as libc::c_ulong,
                                                        );
                                                        if !((*env).me_pbuf).is_null() && {
                                                            txn = calloc(
                                                                1 as libc::c_int as libc::c_ulong,
                                                                size as libc::c_ulong,
                                                            )
                                                                as *mut MDB_txn;
                                                            !txn.is_null()
                                                        } {
                                                            (*txn).mt_dbs = (txn
                                                                as *mut libc::c_char)
                                                                .offset(tsize as isize)
                                                                as *mut MDB_db;
                                                            (*txn).mt_cursors = ((*txn).mt_dbs)
                                                                .offset((*env).me_maxdbs as isize)
                                                                as *mut *mut MDB_cursor;
                                                            (*txn).mt_dbiseqs = ((*txn).mt_cursors)
                                                                .offset((*env).me_maxdbs as isize)
                                                                as *mut libc::c_uint;
                                                            (*txn).mt_dbflags = ((*txn).mt_dbiseqs)
                                                                .offset((*env).me_maxdbs as isize)
                                                                as *mut libc::c_uchar;
                                                            (*txn).mt_env = env;
                                                            (*txn).mt_dbxs = (*env).me_dbxs;
                                                            (*txn).mt_flags =
                                                                MDB_TXN_FINISHED as libc::c_uint;
                                                            (*env).me_txn0 = txn;
                                                        } else {
                                                            rc = ENOMEM;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if rc != 0 {
        mdb_env_close0(env, excl);
    }
    if fname.mn_alloced != 0 {
        free(fname.mn_val as *mut libc::c_void);
    }
    return rc;
}
#[cold]
unsafe extern "C" fn mdb_env_close0(mut env: *mut MDB_env, mut excl: libc::c_int) {
    let mut i: libc::c_int = 0;
    if (*env).me_flags & MDB_ENV_ACTIVE == 0 {
        return;
    }
    if !((*env).me_dbxs).is_null() {
        i = (*env).me_maxdbs as libc::c_int;
        loop {
            i -= 1;
            if !(i >= CORE_DBS) {
                break;
            }
            free((*((*env).me_dbxs).offset(i as isize)).md_name.mv_data);
        }
        free((*env).me_dbxs as *mut libc::c_void);
    }
    free((*env).me_pbuf);
    free((*env).me_dbiseqs as *mut libc::c_void);
    free((*env).me_dbflags as *mut libc::c_void);
    free((*env).me_path as *mut libc::c_void);
    free((*env).me_dirty_list as *mut libc::c_void);
    free((*env).me_txn0 as *mut libc::c_void);
    mdb_midl_free((*env).me_free_pgs);
    if (*env).me_flags & MDB_ENV_TXKEY != 0 {
        pthread_key_delete((*env).me_txkey);
    }
    if !((*env).me_map).is_null() {
        munmap((*env).me_map as *mut libc::c_void, (*env).me_mapsize);
    }
    if (*env).me_mfd != INVALID_HANDLE_VALUE {
        close((*env).me_mfd);
    }
    if (*env).me_fd != INVALID_HANDLE_VALUE {
        close((*env).me_fd);
    }
    if !((*env).me_txns).is_null() {
        let mut pid = getpid();
        i = (*env).me_close_readers;
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            if (*((*(*env).me_txns).mti_readers)
                .as_mut_ptr()
                .offset(i as isize))
            .mru
            .mrx
            .mrb_pid
                == pid
            {
                ::core::ptr::write_volatile(
                    &mut (*((*(*env).me_txns).mti_readers)
                        .as_mut_ptr()
                        .offset(i as isize))
                    .mru
                    .mrx
                    .mrb_pid as *mut pid_t,
                    0 as libc::c_int,
                );
            }
        }
        if (*((*env).me_rmutex).as_mut_ptr()).semid != -(1 as libc::c_int) {
            if excl == 0 as libc::c_int {
                mdb_env_excl_lock(env, &mut excl);
            }
            if excl > 0 as libc::c_int {
                semctl(
                    (*((*env).me_rmutex).as_mut_ptr()).semid,
                    0 as libc::c_int,
                    IPC_RMID,
                );
            }
        }
        munmap(
            (*env).me_txns as *mut libc::c_void,
            (((*env).me_maxreaders).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<MDB_reader>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<MDB_txninfo>() as libc::c_ulong),
        );
    }
    if (*env).me_lfd != INVALID_HANDLE_VALUE {
        close((*env).me_lfd);
    }
    (*env).me_flags &= !(MDB_ENV_ACTIVE | MDB_ENV_TXKEY);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_close(mut env: *mut MDB_env) {
    let mut dp = 0 as *mut MDB_page;
    if env.is_null() {
        return;
    }
    loop {
        dp = (*env).me_dpages;
        if dp.is_null() {
            break;
        }
        (*env).me_dpages = (*dp).mp_p.p_next;
        free(dp as *mut libc::c_void);
    }
    mdb_env_close0(env, 0 as libc::c_int);
    free(env as *mut libc::c_void);
}
unsafe extern "C" fn mdb_cmp_long(mut a: *const MDB_val, mut b: *const MDB_val) -> libc::c_int {
    return if *((*a).mv_data as *mut mdb_size_t) < *((*b).mv_data as *mut mdb_size_t) {
        -(1 as libc::c_int)
    } else {
        (*((*a).mv_data as *mut mdb_size_t) > *((*b).mv_data as *mut mdb_size_t)) as libc::c_int
    };
}
unsafe extern "C" fn mdb_cmp_int(mut a: *const MDB_val, mut b: *const MDB_val) -> libc::c_int {
    return if *((*a).mv_data as *mut libc::c_uint) < *((*b).mv_data as *mut libc::c_uint) {
        -(1 as libc::c_int)
    } else {
        (*((*a).mv_data as *mut libc::c_uint) > *((*b).mv_data as *mut libc::c_uint)) as libc::c_int
    };
}
unsafe extern "C" fn mdb_cmp_cint(mut a: *const MDB_val, mut b: *const MDB_val) -> libc::c_int {
    let mut u = 0 as *mut libc::c_ushort;
    let mut c = 0 as *mut libc::c_ushort;
    let mut x: libc::c_int = 0;
    u = ((*a).mv_data as *mut libc::c_char).offset((*a).mv_size as isize) as *mut libc::c_ushort;
    c = ((*b).mv_data as *mut libc::c_char).offset((*a).mv_size as isize) as *mut libc::c_ushort;
    loop {
        u = u.offset(-1);
        c = c.offset(-1);
        x = *u as libc::c_int - *c as libc::c_int;
        if !(x == 0 && u > (*a).mv_data as *mut libc::c_ushort) {
            break;
        }
    }
    return x;
}
unsafe extern "C" fn mdb_cmp_memn(mut a: *const MDB_val, mut b: *const MDB_val) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    let mut len_diff: ssize_t = 0;
    let mut len: libc::c_uint = 0;
    len = (*a).mv_size as libc::c_uint;
    len_diff = (*a).mv_size as ssize_t - (*b).mv_size as ssize_t;
    if len_diff > 0 as libc::c_int as ssize_t {
        len = (*b).mv_size as libc::c_uint;
        len_diff = 1 as libc::c_int as ssize_t;
    }
    diff = memcmp((*a).mv_data, (*b).mv_data, len as libc::c_ulong);
    return (if diff != 0 {
        diff as ssize_t
    } else if len_diff < 0 as libc::c_int as ssize_t {
        -(1 as libc::c_int) as ssize_t
    } else {
        len_diff
    }) as libc::c_int;
}
unsafe extern "C" fn mdb_cmp_memnr(mut a: *const MDB_val, mut b: *const MDB_val) -> libc::c_int {
    let mut p1 = 0 as *const libc::c_uchar;
    let mut p2 = 0 as *const libc::c_uchar;
    let mut p1_lim = 0 as *const libc::c_uchar;
    let mut len_diff: ssize_t = 0;
    let mut diff: libc::c_int = 0;
    p1_lim = (*a).mv_data as *const libc::c_uchar;
    p1 = ((*a).mv_data as *const libc::c_uchar).offset((*a).mv_size as isize);
    p2 = ((*b).mv_data as *const libc::c_uchar).offset((*b).mv_size as isize);
    len_diff = (*a).mv_size as ssize_t - (*b).mv_size as ssize_t;
    if len_diff > 0 as libc::c_int as ssize_t {
        p1_lim = p1_lim.offset(len_diff as isize);
        len_diff = 1 as libc::c_int as ssize_t;
    }
    while p1 > p1_lim {
        p1 = p1.offset(-1);
        p2 = p2.offset(-1);
        diff = *p1 as libc::c_int - *p2 as libc::c_int;
        if diff != 0 {
            return diff;
        }
    }
    return (if len_diff < 0 as libc::c_int as ssize_t {
        -(1 as libc::c_int) as ssize_t
    } else {
        len_diff
    }) as libc::c_int;
}
unsafe extern "C" fn mdb_node_search(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut exactp: *mut libc::c_int,
) -> *mut MDB_node {
    let mut i = 0 as libc::c_int as libc::c_uint;
    let mut nkeys: libc::c_uint = 0;
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    let mut rc = 0 as libc::c_int;
    let mut mp = (*mc).mc_pg[(*mc).mc_top as usize];
    let mut node = NULL as *mut MDB_node;
    let mut nodekey = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut cmp: Option<MDB_cmp_func> = None;
    nkeys =
        ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint).wrapping_sub(
            (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }),
        ) >> 1 as libc::c_int;
    low = if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
        & 0x2 as libc::c_int
        == 0x2 as libc::c_int
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    high = nkeys.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    cmp = (*(*mc).mc_dbx).md_cmp;
    if cmp == Some(mdb_cmp_cint as MDB_cmp_func)
        && (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
            & 0x1 as libc::c_int
            == 0x1 as libc::c_int
    {
        if (*((mp as *mut libc::c_char)
            .offset(
                *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as libc::c_int as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node))
            .mn_ksize as libc::c_ulong
            == ::core::mem::size_of::<mdb_size_t>() as libc::c_ulong
        {
            cmp = Some(mdb_cmp_long as MDB_cmp_func);
        } else {
            cmp = Some(mdb_cmp_int as MDB_cmp_func);
        }
    }
    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int & 0x20 as libc::c_int
        == 0x20 as libc::c_int
    {
        nodekey.mv_size = (*(*mc).mc_db).md_pad as size_t;
        node = (mp as *mut libc::c_char)
            .offset(
                *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as libc::c_int as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        while low <= high {
            i = (low + high >> 1 as libc::c_int) as libc::c_uint;
            nodekey.mv_data = (mp as *mut libc::c_char)
                .offset(PAGEHDRSZ as libc::c_uint as isize)
                .offset((i as size_t * nodekey.mv_size) as isize)
                as *mut libc::c_void;
            rc = cmp.expect("non-null function pointer")(key, &mut nodekey);
            if rc == 0 as libc::c_int {
                break;
            }
            if rc > 0 as libc::c_int {
                low = i.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
            } else {
                high = i.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
            }
        }
    } else {
        while low <= high {
            i = (low + high >> 1 as libc::c_int) as libc::c_uint;
            node = (mp as *mut libc::c_char)
                .offset(
                    *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                        .as_mut_ptr()
                        .offset(i as isize) as libc::c_int as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            nodekey.mv_size = (*node).mn_ksize as size_t;
            nodekey.mv_data = ((*node).mn_data).as_mut_ptr() as *mut libc::c_void;
            rc = cmp.expect("non-null function pointer")(key, &mut nodekey);
            if rc == 0 as libc::c_int {
                break;
            }
            if rc > 0 as libc::c_int {
                low = i.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
            } else {
                high = i.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
            }
        }
    }
    if rc > 0 as libc::c_int {
        i = i.wrapping_add(1);
        i;
        if !((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
            & 0x20 as libc::c_int
            == 0x20 as libc::c_int)
        {
            node = (mp as *mut libc::c_char)
                .offset(
                    *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                        .as_mut_ptr()
                        .offset(i as isize) as libc::c_int as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
        }
    }
    if !exactp.is_null() {
        *exactp =
            (rc == 0 as libc::c_int && nkeys > 0 as libc::c_int as libc::c_uint) as libc::c_int;
    }
    (*mc).mc_ki[(*mc).mc_top as usize] = i as indx_t;
    if i >= nkeys {
        return NULL as *mut MDB_node;
    }
    return node;
}
unsafe extern "C" fn mdb_cursor_pop(mut mc: *mut MDB_cursor) {
    if (*mc).mc_snum != 0 {
        (*mc).mc_snum = ((*mc).mc_snum).wrapping_sub(1);
        (*mc).mc_snum;
        if (*mc).mc_snum != 0 {
            (*mc).mc_top = ((*mc).mc_top).wrapping_sub(1);
            (*mc).mc_top;
        } else {
            (*mc).mc_flags &= !C_INITIALIZED as libc::c_uint;
        }
    }
}
unsafe extern "C" fn mdb_cursor_push(
    mut mc: *mut MDB_cursor,
    mut mp: *mut MDB_page,
) -> libc::c_int {
    if (*mc).mc_snum as libc::c_int >= CURSOR_STACK {
        (*(*mc).mc_txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
        return MDB_CURSOR_FULL;
    }
    let fresh20 = (*mc).mc_snum;
    (*mc).mc_snum = ((*mc).mc_snum).wrapping_add(1);
    (*mc).mc_top = fresh20;
    (*mc).mc_pg[(*mc).mc_top as usize] = mp;
    (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_page_get(
    mut mc: *mut MDB_cursor,
    mut pgno: pgno_t,
    mut ret: *mut *mut MDB_page,
    mut lvl: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut txn = (*mc).mc_txn;
    let mut p = NULL as *mut MDB_page;
    let mut level: libc::c_int = 0;
    if (*mc).mc_flags & (C_ORIG_RDONLY | C_WRITEMAP) as libc::c_uint == 0 {
        let mut tx2 = txn;
        level = 1 as libc::c_int;
        loop {
            let mut dl = (*tx2).mt_u.dirty_list;
            let mut x: libc::c_uint = 0;
            if !((*tx2).mt_spill_pgs).is_null() {
                let mut pn = pgno << 1 as libc::c_int;
                x = mdb_midl_search((*tx2).mt_spill_pgs, pn);
                if x as MDB_ID <= *((*tx2).mt_spill_pgs).offset(0 as libc::c_int as isize)
                    && *((*tx2).mt_spill_pgs).offset(x as isize) == pn
                {
                    current_block = 5159274329597579145;
                    break;
                }
            }
            if (*dl.offset(0 as libc::c_int as isize)).mid != 0 {
                let mut x_0 = mdb_mid2l_search(dl, pgno);
                if x_0 as MDB_ID <= (*dl.offset(0 as libc::c_int as isize)).mid
                    && (*dl.offset(x_0 as isize)).mid == pgno
                {
                    p = (*dl.offset(x_0 as isize)).mptr as *mut MDB_page;
                    current_block = 7992952532600268616;
                    break;
                }
            }
            level += 1;
            level;
            tx2 = (*tx2).mt_parent;
            if tx2.is_null() {
                current_block = 3512920355445576850;
                break;
            }
        }
    } else {
        current_block = 3512920355445576850;
    }
    match current_block {
        3512920355445576850 => {
            if pgno >= (*txn).mt_next_pgno {
                (*txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
                return MDB_PAGE_NOTFOUND;
            }
            level = 0 as libc::c_int;
            current_block = 5159274329597579145;
        }
        _ => {}
    }
    match current_block {
        5159274329597579145 => {
            let mut env = (*txn).mt_env;
            p = ((*env).me_map).offset(((*env).me_psize as pgno_t * pgno) as isize)
                as *mut MDB_page;
        }
        _ => {}
    }
    *ret = p;
    if !lvl.is_null() {
        *lvl = level;
    }
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_page_search_root(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut mp = (*mc).mc_pg[(*mc).mc_top as usize];
    let mut rc: libc::c_int = 0;
    while (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
        & 0x1 as libc::c_int
        == 0x1 as libc::c_int
    {
        let mut current_block_30: u64;
        let mut node = 0 as *mut MDB_node;
        let mut i: indx_t = 0;
        if (*mc).mc_dbi == 0
            || ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
                .wrapping_sub((16 as libc::c_ulong as libc::c_uint).wrapping_sub(
                    if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    },
                ))
                >> 1 as libc::c_int
                > 1 as libc::c_int as libc::c_uint
        {
        } else {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"!mc->mc_dbi || NUMKEYS(mp) > 1\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"mdb_page_search_root\0",
                ))
                .as_ptr(),
                b"mdb.c\0" as *const u8 as *const libc::c_char,
                6554 as libc::c_int,
            );
        };
        if flags & (MDB_PS_FIRST | MDB_PS_LAST) != 0 {
            i = 0 as libc::c_int as indx_t;
            if flags & MDB_PS_LAST != 0 {
                i = (((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
                    .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                        if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        },
                    ))
                    >> 1 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
                if (*mc).mc_flags & C_INITIALIZED as libc::c_uint != 0 {
                    if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int == i as libc::c_int {
                        let fresh21 = (*mc).mc_snum;
                        (*mc).mc_snum = ((*mc).mc_snum).wrapping_add(1);
                        (*mc).mc_top = fresh21;
                        mp = (*mc).mc_pg[(*mc).mc_top as usize];
                        current_block_30 = 9606348990739642274;
                    } else {
                        current_block_30 = 4495394744059808450;
                    }
                } else {
                    current_block_30 = 4495394744059808450;
                }
            } else {
                current_block_30 = 4495394744059808450;
            }
        } else {
            let mut exact: libc::c_int = 0;
            node = mdb_node_search(mc, key, &mut exact);
            if node.is_null() {
                i = (((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
                    .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                        if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        },
                    ))
                    >> 1 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
            } else {
                i = (*mc).mc_ki[(*mc).mc_top as usize];
                if exact == 0 {
                    if i as libc::c_int > 0 as libc::c_int {
                    } else {
                        mdb_assert_fail(
                            (*(*mc).mc_txn).mt_env,
                            b"i > 0\0" as *const u8 as *const libc::c_char,
                            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                                b"mdb_page_search_root\0",
                            ))
                            .as_ptr(),
                            b"mdb.c\0" as *const u8 as *const libc::c_char,
                            6578 as libc::c_int,
                        );
                    };
                    i = i.wrapping_sub(1);
                    i;
                }
            }
            current_block_30 = 4495394744059808450;
        }
        match current_block_30 {
            4495394744059808450 => {
                if (i as libc::c_uint)
                    < ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
                        .wrapping_sub((16 as libc::c_ulong as libc::c_uint).wrapping_sub(
                            if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            },
                        ))
                        >> 1 as libc::c_int
                {
                } else {
                    mdb_assert_fail(
                        (*(*mc).mc_txn).mt_env,
                        b"i < NUMKEYS(mp)\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                            b"mdb_page_search_root\0",
                        ))
                        .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                        6585 as libc::c_int,
                    );
                };
                node = (mp as *mut libc::c_char)
                    .offset(
                        *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                            .as_mut_ptr()
                            .offset(i as isize) as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                rc = mdb_page_get(
                    mc,
                    (*node).mn_lo as pgno_t
                        | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                        | (if (if -(1 as libc::c_int) as pgno_t
                            > 0xffffffff as libc::c_uint as pgno_t
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) != 0
                        {
                            ((*node).mn_flags as pgno_t)
                                << (if -(1 as libc::c_int) as pgno_t
                                    > 0xffffffff as libc::c_uint as pgno_t
                                {
                                    32 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                        } else {
                            0 as libc::c_int as pgno_t
                        }),
                    &mut mp,
                    NULL as *mut libc::c_int,
                );
                if rc != 0 as libc::c_int {
                    return rc;
                }
                (*mc).mc_ki[(*mc).mc_top as usize] = i;
                rc = mdb_cursor_push(mc, mp);
                if rc != 0 {
                    return rc;
                }
            }
            _ => {}
        }
        if flags & MDB_PS_MODIFY != 0 {
            rc = mdb_page_touch(mc);
            if rc != 0 as libc::c_int {
                return rc;
            }
            mp = (*mc).mc_pg[(*mc).mc_top as usize];
        }
    }
    if !((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
        & 0x2 as libc::c_int
        == 0x2 as libc::c_int)
    {
        (*(*mc).mc_txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
        return MDB_CORRUPTED;
    }
    (*mc).mc_flags |= C_INITIALIZED as libc::c_uint;
    (*mc).mc_flags &= !C_EOF as libc::c_uint;
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_page_search_lowest(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut mp = (*mc).mc_pg[(*mc).mc_top as usize];
    let mut node = (mp as *mut libc::c_char)
        .offset(
            *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as libc::c_int as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    let mut rc: libc::c_int = 0;
    rc = mdb_page_get(
        mc,
        (*node).mn_lo as pgno_t
            | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
            | (if (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                32 as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0
            {
                ((*node).mn_flags as pgno_t)
                    << (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                        32 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })
            } else {
                0 as libc::c_int as pgno_t
            }),
        &mut mp,
        NULL as *mut libc::c_int,
    );
    if rc != 0 as libc::c_int {
        return rc;
    }
    (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
    rc = mdb_cursor_push(mc, mp);
    if rc != 0 {
        return rc;
    }
    return mdb_page_search_root(mc, NULL as *mut MDB_val, MDB_PS_FIRST);
}
unsafe extern "C" fn mdb_page_search(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut root: pgno_t = 0;
    if (*(*mc).mc_txn).mt_flags & MDB_TXN_BLOCKED as libc::c_uint != 0 {
        return MDB_BAD_TXN;
    } else {
        if *(*mc).mc_dbflag as libc::c_int & DB_STALE != 0 {
            let mut mc2 = MDB_cursor {
                mc_next: 0 as *mut MDB_cursor,
                mc_backup: 0 as *mut MDB_cursor,
                mc_xcursor: 0 as *mut MDB_xcursor,
                mc_txn: 0 as *mut MDB_txn,
                mc_dbi: 0,
                mc_db: 0 as *mut MDB_db,
                mc_dbx: 0 as *mut MDB_dbx,
                mc_dbflag: 0 as *mut libc::c_uchar,
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [0 as *mut MDB_page; 32],
                mc_ki: [0; 32],
            };
            if *((*(*mc).mc_txn).mt_dbiseqs).offset((*mc).mc_dbi as isize)
                != *((*(*(*mc).mc_txn).mt_env).me_dbiseqs).offset((*mc).mc_dbi as isize)
            {
                return MDB_BAD_DBI;
            }
            mdb_cursor_init(
                &mut mc2,
                (*mc).mc_txn,
                MAIN_DBI as MDB_dbi,
                NULL as *mut MDB_xcursor,
            );
            rc = mdb_page_search(&mut mc2, &mut (*(*mc).mc_dbx).md_name, 0 as libc::c_int);
            if rc != 0 {
                return rc;
            }
            let mut data = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            let mut exact = 0 as libc::c_int;
            let mut flags_0: uint16_t = 0;
            let mut leaf = mdb_node_search(&mut mc2, &mut (*(*mc).mc_dbx).md_name, &mut exact);
            if exact == 0 {
                return MDB_BAD_DBI;
            }
            if (*leaf).mn_flags as libc::c_int & (F_DUPDATA | F_SUBDATA) != F_SUBDATA {
                return MDB_INCOMPATIBLE;
            }
            rc = mdb_node_read(&mut mc2, leaf, &mut data);
            if rc != 0 {
                return rc;
            }
            memcpy(
                &mut flags_0 as *mut uint16_t as *mut libc::c_void,
                (data.mv_data as *mut libc::c_char).offset(mem::offset_of!(MDB_db, md_flags) as _)
                    as *const libc::c_void,
                ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
            );
            if (*(*mc).mc_db).md_flags as libc::c_int & PERSISTENT_FLAGS != flags_0 as libc::c_int {
                return MDB_INCOMPATIBLE;
            }
            memcpy(
                (*mc).mc_db as *mut libc::c_void,
                data.mv_data,
                ::core::mem::size_of::<MDB_db>() as libc::c_ulong,
            );
            *(*mc).mc_dbflag = (*(*mc).mc_dbflag as libc::c_int & !DB_STALE) as libc::c_uchar;
        }
        root = (*(*mc).mc_db).md_root;
        if root == P_INVALID {
            return MDB_NOTFOUND;
        }
    }
    if root > 1 as libc::c_int as pgno_t {
    } else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"root > 1\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"mdb_page_search\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            6705 as libc::c_int,
        );
    };
    if ((*mc).mc_pg[0 as libc::c_int as usize]).is_null()
        || (*(*mc).mc_pg[0 as libc::c_int as usize]).mp_p.p_pgno != root
    {
        rc = mdb_page_get(
            mc,
            root,
            &mut *((*mc).mc_pg).as_mut_ptr().offset(0 as libc::c_int as isize),
            NULL as *mut libc::c_int,
        );
        if rc != 0 as libc::c_int {
            return rc;
        }
    }
    (*mc).mc_snum = 1 as libc::c_int as libc::c_ushort;
    (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
    if flags & MDB_PS_MODIFY != 0 {
        rc = mdb_page_touch(mc);
        if rc != 0 {
            return rc;
        }
    }
    if flags & MDB_PS_ROOTONLY != 0 {
        return MDB_SUCCESS;
    }
    return mdb_page_search_root(mc, key, flags);
}
unsafe extern "C" fn mdb_ovpage_free(
    mut mc: *mut MDB_cursor,
    mut mp: *mut MDB_page,
) -> libc::c_int {
    let mut txn = (*mc).mc_txn;
    let mut pg = (*mp).mp_p.p_pgno;
    let mut x = 0 as libc::c_int as libc::c_uint;
    let mut ovpages = (*mp).mp_pb.pb_pages;
    let mut env = (*txn).mt_env;
    let mut sl = (*txn).mt_spill_pgs;
    let mut pn = pg << 1 as libc::c_int;
    let mut rc: libc::c_int = 0;
    if !((*env).me_pgstate.mf_pghead).is_null()
        && ((*txn).mt_parent).is_null()
        && ((*mp).mp_flags as libc::c_int & P_DIRTY != 0
            || !sl.is_null()
                && {
                    x = mdb_midl_search(sl, pn);
                    x as MDB_ID <= *sl.offset(0 as libc::c_int as isize)
                }
                && *sl.offset(x as isize) == pn)
    {
        let mut i: libc::c_uint = 0;
        let mut j: libc::c_uint = 0;
        let mut mop = 0 as *mut pgno_t;
        let mut dl = 0 as *mut MDB_ID2;
        let mut ix = MDB_ID2 {
            mid: 0,
            mptr: 0 as *mut libc::c_void,
        };
        let mut iy = MDB_ID2 {
            mid: 0,
            mptr: 0 as *mut libc::c_void,
        };
        rc = mdb_midl_need(&mut (*env).me_pgstate.mf_pghead, ovpages);
        if rc != 0 {
            return rc;
        }
        if (*mp).mp_flags as libc::c_int & P_DIRTY == 0 {
            if x as MDB_ID == *sl.offset(0 as libc::c_int as isize) {
                let ref mut fresh22 = *sl.offset(0 as libc::c_int as isize);
                *fresh22 = (*fresh22).wrapping_sub(1);
                let _ = *fresh22;
            } else {
                *sl.offset(x as isize) |= 1 as libc::c_int as MDB_ID;
            }
        } else {
            dl = (*txn).mt_u.dirty_list;
            let ref mut fresh23 = (*dl.offset(0 as libc::c_int as isize)).mid;
            let fresh24 = *fresh23;
            *fresh23 = (*fresh23).wrapping_sub(1);
            x = fresh24 as libc::c_uint;
            ix = *dl.offset(x as isize);
            while ix.mptr != mp as *mut libc::c_void {
                if x > 1 as libc::c_int as libc::c_uint {
                    x = x.wrapping_sub(1);
                    x;
                    iy = *dl.offset(x as isize);
                    *dl.offset(x as isize) = ix;
                } else {
                    if x > 1 as libc::c_int as libc::c_uint {
                    } else {
                        mdb_assert_fail(
                            (*(*mc).mc_txn).mt_env,
                            b"x > 1\0" as *const u8 as *const libc::c_char,
                            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                                b"mdb_ovpage_free\0",
                            ))
                            .as_ptr(),
                            b"mdb.c\0" as *const u8 as *const libc::c_char,
                            6787 as libc::c_int,
                        );
                    };
                    let ref mut fresh25 = (*dl.offset(0 as libc::c_int as isize)).mid;
                    *fresh25 = (*fresh25).wrapping_add(1);
                    j = *fresh25 as libc::c_uint;
                    *dl.offset(j as isize) = ix;
                    (*txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
                    return MDB_PROBLEM;
                }
                ix = iy;
            }
            (*txn).mt_dirty_room = ((*txn).mt_dirty_room).wrapping_add(1);
            (*txn).mt_dirty_room;
            if (*env).me_flags & MDB_WRITEMAP as uint32_t == 0 {
                mdb_dpage_free(env, mp);
            }
        }
        mop = (*env).me_pgstate.mf_pghead;
        j = (*mop.offset(0 as libc::c_int as isize)).wrapping_add(ovpages as pgno_t)
            as libc::c_uint;
        i = *mop.offset(0 as libc::c_int as isize) as libc::c_uint;
        while i != 0 && *mop.offset(i as isize) < pg {
            let fresh26 = j;
            j = j.wrapping_sub(1);
            *mop.offset(fresh26 as isize) = *mop.offset(i as isize);
            i = i.wrapping_sub(1);
            i;
        }
        while j > i {
            let fresh27 = pg;
            pg = pg.wrapping_add(1);
            let fresh28 = j;
            j = j.wrapping_sub(1);
            *mop.offset(fresh28 as isize) = fresh27;
        }
        let ref mut fresh29 = *mop.offset(0 as libc::c_int as isize);
        *fresh29 = (*fresh29).wrapping_add(ovpages as pgno_t);
    } else {
        rc = mdb_midl_append_range(&mut (*txn).mt_free_pgs, pg, ovpages);
        if rc != 0 {
            return rc;
        }
    }
    (*(*mc).mc_db).md_overflow_pages =
        ((*(*mc).mc_db).md_overflow_pages).wrapping_sub(ovpages as pgno_t);
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_node_read(
    mut mc: *mut MDB_cursor,
    mut leaf: *mut MDB_node,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut omp = 0 as *mut MDB_page;
    let mut pgno: pgno_t = 0;
    let mut rc: libc::c_int = 0;
    let _ = !(0 as *mut MDB_page).is_null();
    if !((*leaf).mn_flags as libc::c_int & 0x1 as libc::c_int == 0x1 as libc::c_int) {
        (*data).mv_size = ((*leaf).mn_lo as libc::c_uint
            | ((*leaf).mn_hi as libc::c_uint) << 16 as libc::c_int)
            as size_t;
        (*data).mv_data = ((*leaf).mn_data)
            .as_mut_ptr()
            .offset((*leaf).mn_ksize as libc::c_int as isize)
            as *mut libc::c_void;
        return MDB_SUCCESS;
    }
    (*data).mv_size = ((*leaf).mn_lo as libc::c_uint
        | ((*leaf).mn_hi as libc::c_uint) << 16 as libc::c_int) as size_t;
    memcpy(
        &mut pgno as *mut pgno_t as *mut libc::c_void,
        ((*leaf).mn_data)
            .as_mut_ptr()
            .offset((*leaf).mn_ksize as libc::c_int as isize) as *mut libc::c_void,
        ::core::mem::size_of::<pgno_t>() as libc::c_ulong,
    );
    rc = mdb_page_get(mc, pgno, &mut omp, NULL as *mut libc::c_int);
    if rc != 0 as libc::c_int {
        return rc;
    }
    (*data).mv_data =
        (omp as *mut libc::c_char).offset(PAGEHDRSZ as libc::c_uint as isize) as *mut libc::c_void;
    return MDB_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_get(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut mc = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut mx = MDB_xcursor {
        mx_cursor: MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        },
        mx_db: MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        },
        mx_dbx: MDB_dbx {
            md_name: MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            },
            md_cmp: None,
            md_dcmp: None,
            md_rel: None,
            md_relctx: 0 as *mut libc::c_void,
        },
        mx_dbflag: 0,
    };
    let mut exact = 0 as libc::c_int;
    let mut rc: libc::c_int = 0;
    if key.is_null()
        || data.is_null()
        || !(!txn.is_null()
            && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int != 0)
    {
        return EINVAL;
    }
    if (*txn).mt_flags & MDB_TXN_BLOCKED as libc::c_uint != 0 {
        return MDB_BAD_TXN;
    }
    mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
    rc = mdb_cursor_set(&mut mc, key, data, MDB_SET, &mut exact);
    return rc;
}
unsafe extern "C" fn mdb_cursor_sibling(
    mut mc: *mut MDB_cursor,
    mut move_right: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut indx = 0 as *mut MDB_node;
    let mut mp = 0 as *mut MDB_page;
    if ((*mc).mc_snum as libc::c_int) < 2 as libc::c_int {
        return MDB_NOTFOUND;
    }
    mdb_cursor_pop(mc);
    if if move_right != 0 {
        (((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint).wrapping_add(1 as libc::c_uint)
            >= ((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                .mp2_lower as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int) as libc::c_int
    } else {
        ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int == 0 as libc::c_int) as libc::c_int
    } != 0
    {
        rc = mdb_cursor_sibling(mc, move_right);
        if rc != MDB_SUCCESS {
            (*mc).mc_top = ((*mc).mc_top).wrapping_add(1);
            (*mc).mc_top;
            (*mc).mc_snum = ((*mc).mc_snum).wrapping_add(1);
            (*mc).mc_snum;
            return rc;
        }
    } else if move_right != 0 {
        (*mc).mc_ki[(*mc).mc_top as usize] = ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_add(1);
        (*mc).mc_ki[(*mc).mc_top as usize];
    } else {
        (*mc).mc_ki[(*mc).mc_top as usize] = ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_sub(1);
        (*mc).mc_ki[(*mc).mc_top as usize];
    }
    if (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_flags
        as libc::c_int
        & 0x1 as libc::c_int
        == 0x1 as libc::c_int
    {
    } else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"IS_BRANCH(mc->mc_pg[mc->mc_top])\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"mdb_cursor_sibling\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            6929 as libc::c_int,
        );
    };
    indx = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
        .offset(
            *((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                .mp2_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    rc = mdb_page_get(
        mc,
        (*indx).mn_lo as pgno_t
            | ((*indx).mn_hi as pgno_t) << 16 as libc::c_int
            | (if (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                32 as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0
            {
                ((*indx).mn_flags as pgno_t)
                    << (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                        32 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })
            } else {
                0 as libc::c_int as pgno_t
            }),
        &mut mp,
        NULL as *mut libc::c_int,
    );
    if rc != 0 as libc::c_int {
        (*mc).mc_flags &= !(C_INITIALIZED | C_EOF) as libc::c_uint;
        return rc;
    }
    mdb_cursor_push(mc, mp);
    if move_right == 0 {
        (*mc).mc_ki[(*mc).mc_top as usize] =
            (((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
    }
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_cursor_next(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
) -> libc::c_int {
    let mut mp = 0 as *mut MDB_page;
    let mut leaf = 0 as *mut MDB_node;
    let mut rc: libc::c_int = 0;
    if (*mc).mc_flags & C_DEL as libc::c_uint != 0
        && op as libc::c_uint == MDB_NEXT_DUP as libc::c_int as libc::c_uint
    {
        return MDB_NOTFOUND;
    }
    if (*mc).mc_flags & C_INITIALIZED as libc::c_uint == 0 {
        return mdb_cursor_first(mc, key, data);
    }
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    if (*mc).mc_flags & C_EOF as libc::c_uint != 0 {
        if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint
            >= (((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            return MDB_NOTFOUND;
        }
        (*mc).mc_flags ^= C_EOF as libc::c_uint;
    }
    if (*(*mc).mc_db).md_flags as libc::c_int & MDB_DUPSORT != 0 {
        leaf = (mp as *mut libc::c_char)
            .offset(
                *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                    as libc::c_int as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
            if op as libc::c_uint == MDB_NEXT as libc::c_int as libc::c_uint
                || op as libc::c_uint == MDB_NEXT_DUP as libc::c_int as libc::c_uint
            {
                rc = mdb_cursor_next(
                    &mut (*(*mc).mc_xcursor).mx_cursor,
                    data,
                    NULL as *mut MDB_val,
                    MDB_NEXT,
                );
                if op as libc::c_uint != MDB_NEXT as libc::c_int as libc::c_uint
                    || rc != MDB_NOTFOUND
                {
                    if rc == MDB_SUCCESS {
                        if !key.is_null() {
                            (*key).mv_size = (*leaf).mn_ksize as size_t;
                            (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
                        }
                    }
                    return rc;
                }
            }
        } else {
            (*(*mc).mc_xcursor).mx_cursor.mc_flags &= !(C_INITIALIZED | C_EOF) as libc::c_uint;
            if op as libc::c_uint == MDB_NEXT_DUP as libc::c_int as libc::c_uint {
                return MDB_NOTFOUND;
            }
        }
    }
    if (*mc).mc_flags & C_DEL as libc::c_uint != 0 {
        (*mc).mc_flags ^= C_DEL as libc::c_uint;
    } else if ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint).wrapping_add(1 as libc::c_uint)
        >= ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint).wrapping_sub(
            (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }),
        ) >> 1 as libc::c_int
    {
        rc = mdb_cursor_sibling(mc, 1 as libc::c_int);
        if rc != MDB_SUCCESS {
            (*mc).mc_flags |= C_EOF as libc::c_uint;
            return rc;
        }
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
    } else {
        (*mc).mc_ki[(*mc).mc_top as usize] = ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_add(1);
        (*mc).mc_ki[(*mc).mc_top as usize];
    }
    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int & 0x20 as libc::c_int
        == 0x20 as libc::c_int
    {
        (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
        (*key).mv_data = (mp as *mut libc::c_char)
            .offset(PAGEHDRSZ as libc::c_uint as isize)
            .offset(((*mc).mc_ki[(*mc).mc_top as usize] as size_t * (*key).mv_size) as isize)
            as *mut libc::c_void;
        return MDB_SUCCESS;
    }
    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int & 0x2 as libc::c_int
        == 0x2 as libc::c_int
    {
    } else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"IS_LEAF(mp)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"mdb_cursor_next\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            7018 as libc::c_int,
        );
    };
    leaf = (mp as *mut libc::c_char)
        .offset(
            *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        rc = mdb_cursor_first(
            &mut (*(*mc).mc_xcursor).mx_cursor,
            data,
            NULL as *mut MDB_val,
        );
        if rc != MDB_SUCCESS {
            return rc;
        }
    } else if !data.is_null() {
        rc = mdb_node_read(mc, leaf, data);
        if rc != MDB_SUCCESS {
            return rc;
        }
    }
    if !key.is_null() {
        (*key).mv_size = (*leaf).mn_ksize as size_t;
        (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
    }
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_cursor_prev(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
) -> libc::c_int {
    let mut mp = 0 as *mut MDB_page;
    let mut leaf = 0 as *mut MDB_node;
    let mut rc: libc::c_int = 0;
    if (*mc).mc_flags & C_INITIALIZED as libc::c_uint == 0 {
        rc = mdb_cursor_last(mc, key, data);
        if rc != 0 {
            return rc;
        }
        (*mc).mc_ki[(*mc).mc_top as usize] = ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_add(1);
        (*mc).mc_ki[(*mc).mc_top as usize];
    }
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    if (*(*mc).mc_db).md_flags as libc::c_int & MDB_DUPSORT != 0
        && ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint)
            < ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int
    {
        leaf = (mp as *mut libc::c_char)
            .offset(
                *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                    as libc::c_int as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
            if op as libc::c_uint == MDB_PREV as libc::c_int as libc::c_uint
                || op as libc::c_uint == MDB_PREV_DUP as libc::c_int as libc::c_uint
            {
                rc = mdb_cursor_prev(
                    &mut (*(*mc).mc_xcursor).mx_cursor,
                    data,
                    NULL as *mut MDB_val,
                    MDB_PREV,
                );
                if op as libc::c_uint != MDB_PREV as libc::c_int as libc::c_uint
                    || rc != MDB_NOTFOUND
                {
                    if rc == MDB_SUCCESS {
                        if !key.is_null() {
                            (*key).mv_size = (*leaf).mn_ksize as size_t;
                            (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
                        }
                        (*mc).mc_flags &= !C_EOF as libc::c_uint;
                    }
                    return rc;
                }
            }
        } else {
            (*(*mc).mc_xcursor).mx_cursor.mc_flags &= !(C_INITIALIZED | C_EOF) as libc::c_uint;
            if op as libc::c_uint == MDB_PREV_DUP as libc::c_int as libc::c_uint {
                return MDB_NOTFOUND;
            }
        }
    }
    (*mc).mc_flags &= !(C_EOF | C_DEL) as libc::c_uint;
    if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int == 0 as libc::c_int {
        rc = mdb_cursor_sibling(mc, 0 as libc::c_int);
        if rc != MDB_SUCCESS {
            return rc;
        }
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        (*mc).mc_ki[(*mc).mc_top as usize] =
            (((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
    } else {
        (*mc).mc_ki[(*mc).mc_top as usize] = ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_sub(1);
        (*mc).mc_ki[(*mc).mc_top as usize];
    }
    if !((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
        & 0x2 as libc::c_int
        == 0x2 as libc::c_int)
    {
        return MDB_CORRUPTED;
    }
    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int & 0x20 as libc::c_int
        == 0x20 as libc::c_int
    {
        (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
        (*key).mv_data = (mp as *mut libc::c_char)
            .offset(PAGEHDRSZ as libc::c_uint as isize)
            .offset(((*mc).mc_ki[(*mc).mc_top as usize] as size_t * (*key).mv_size) as isize)
            as *mut libc::c_void;
        return MDB_SUCCESS;
    }
    leaf = (mp as *mut libc::c_char)
        .offset(
            *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        rc = mdb_cursor_last(
            &mut (*(*mc).mc_xcursor).mx_cursor,
            data,
            NULL as *mut MDB_val,
        );
        if rc != MDB_SUCCESS {
            return rc;
        }
    } else if !data.is_null() {
        rc = mdb_node_read(mc, leaf, data);
        if rc != MDB_SUCCESS {
            return rc;
        }
    }
    if !key.is_null() {
        (*key).mv_size = (*leaf).mn_ksize as size_t;
        (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
    }
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_cursor_set(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
    mut exactp: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut mp = 0 as *mut MDB_page;
    let mut leaf = NULL as *mut MDB_node;
    if (*key).mv_size == 0 as libc::c_int as size_t {
        return MDB_BAD_VALSIZE;
    }
    if !((*mc).mc_xcursor).is_null() {
        (*(*mc).mc_xcursor).mx_cursor.mc_flags &= !(C_INITIALIZED | C_EOF) as libc::c_uint;
    }
    if (*mc).mc_flags & C_INITIALIZED as libc::c_uint != 0 {
        let mut nodekey = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        if ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint).wrapping_sub(
            (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }),
        ) >> 1 as libc::c_int
            == 0
        {
            (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
            return MDB_NOTFOUND;
        }
        if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int & P_LEAF2 != 0 {
            nodekey.mv_size = (*(*mc).mc_db).md_pad as size_t;
            nodekey.mv_data = (mp as *mut libc::c_char)
                .offset(PAGEHDRSZ as libc::c_uint as isize)
                .offset((0 as libc::c_int as size_t * nodekey.mv_size) as isize)
                as *mut libc::c_void;
        } else {
            leaf = (mp as *mut libc::c_char)
                .offset(
                    *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            nodekey.mv_size = (*leaf).mn_ksize as size_t;
            nodekey.mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
        }
        rc = ((*(*mc).mc_dbx).md_cmp).expect("non-null function pointer")(key, &mut nodekey);
        if rc == 0 as libc::c_int {
            (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
            if !exactp.is_null() {
                *exactp = 1 as libc::c_int;
            }
            current_block = 7663645823963397913;
        } else {
            if rc > 0 as libc::c_int {
                let mut i: libc::c_uint = 0;
                let mut nkeys = ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                    as libc::c_uint)
                    .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                        if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        },
                    ))
                    >> 1 as libc::c_int;
                if nkeys > 1 as libc::c_int as libc::c_uint {
                    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                        & P_LEAF2
                        != 0
                    {
                        nodekey.mv_data = (mp as *mut libc::c_char)
                            .offset(PAGEHDRSZ as libc::c_uint as isize)
                            .offset(
                                (nkeys.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t
                                    * nodekey.mv_size) as isize,
                            ) as *mut libc::c_void;
                    } else {
                        leaf = (mp as *mut libc::c_char)
                            .offset(
                                *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                    .as_mut_ptr()
                                    .offset(nkeys.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        as isize) as libc::c_int
                                    as isize,
                            )
                            .offset(
                                (if 0 as libc::c_int != 0 {
                                    PAGEHDRSZ as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        nodekey.mv_size = (*leaf).mn_ksize as size_t;
                        nodekey.mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
                    }
                    rc = ((*(*mc).mc_dbx).md_cmp).expect("non-null function pointer")(
                        key,
                        &mut nodekey,
                    );
                    if rc == 0 as libc::c_int {
                        (*mc).mc_ki[(*mc).mc_top as usize] =
                            nkeys.wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
                        if !exactp.is_null() {
                            *exactp = 1 as libc::c_int;
                        }
                        current_block = 7663645823963397913;
                    } else if rc < 0 as libc::c_int {
                        if ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint)
                            < ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                                as libc::c_uint)
                                .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                    if 0 as libc::c_int != 0 {
                                        PAGEHDRSZ as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    },
                                ))
                                >> 1 as libc::c_int
                        {
                            if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags
                                as libc::c_int
                                & P_LEAF2
                                != 0
                            {
                                nodekey.mv_data = (mp as *mut libc::c_char)
                                    .offset(PAGEHDRSZ as libc::c_uint as isize)
                                    .offset(
                                        ((*mc).mc_ki[(*mc).mc_top as usize] as size_t
                                            * nodekey.mv_size)
                                            as isize,
                                    )
                                    as *mut libc::c_void;
                            } else {
                                leaf = (mp as *mut libc::c_char)
                                    .offset(
                                        *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                            .as_mut_ptr()
                                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                            as libc::c_int
                                            as isize,
                                    )
                                    .offset(
                                        (if 0 as libc::c_int != 0 {
                                            PAGEHDRSZ as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        }) as isize,
                                    ) as *mut MDB_node;
                                nodekey.mv_size = (*leaf).mn_ksize as size_t;
                                nodekey.mv_data =
                                    ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
                            }
                            rc = ((*(*mc).mc_dbx).md_cmp).expect("non-null function pointer")(
                                key,
                                &mut nodekey,
                            );
                            if rc == 0 as libc::c_int {
                                if !exactp.is_null() {
                                    *exactp = 1 as libc::c_int;
                                }
                                current_block = 7663645823963397913;
                            } else {
                                current_block = 3160140712158701372;
                            }
                        } else {
                            current_block = 3160140712158701372;
                        }
                        match current_block {
                            7663645823963397913 => {}
                            _ => {
                                rc = 0 as libc::c_int;
                                (*mc).mc_flags &= !C_EOF as libc::c_uint;
                                current_block = 4018922828020914930;
                            }
                        }
                    } else {
                        current_block = 13619784596304402172;
                    }
                } else {
                    current_block = 13619784596304402172;
                }
                match current_block {
                    4018922828020914930 => {}
                    7663645823963397913 => {}
                    _ => {
                        i = 0 as libc::c_int as libc::c_uint;
                        while i < (*mc).mc_top as libc::c_uint {
                            if ((*mc).mc_ki[i as usize] as libc::c_uint)
                                < (((*((*mc).mc_pg[i as usize] as *mut libc::c_void
                                    as *mut MDB_page2))
                                    .mp2_lower as libc::c_uint)
                                    .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                        if 0 as libc::c_int != 0 {
                                            PAGEHDRSZ as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        },
                                    ))
                                    >> 1 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            {
                                break;
                            }
                            i = i.wrapping_add(1);
                            i;
                        }
                        if i == (*mc).mc_top as libc::c_uint {
                            (*mc).mc_ki[(*mc).mc_top as usize] = nkeys as indx_t;
                            return MDB_NOTFOUND;
                        }
                        current_block = 8347882395825654554;
                    }
                }
            } else {
                current_block = 8347882395825654554;
            }
            match current_block {
                7663645823963397913 => {}
                4018922828020914930 => {}
                _ => {
                    if (*mc).mc_top == 0 {
                        (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
                        if op as libc::c_uint == MDB_SET_RANGE as libc::c_int as libc::c_uint
                            && exactp.is_null()
                        {
                            rc = 0 as libc::c_int;
                        } else {
                            return MDB_NOTFOUND;
                        }
                        current_block = 7663645823963397913;
                    } else {
                        current_block = 15594603006322722090;
                    }
                }
            }
        }
    } else {
        (*mc).mc_pg[0 as libc::c_int as usize] = 0 as *mut MDB_page;
        current_block = 15594603006322722090;
    }
    match current_block {
        15594603006322722090 => {
            rc = mdb_page_search(mc, key, 0 as libc::c_int);
            if rc != MDB_SUCCESS {
                return rc;
            }
            mp = (*mc).mc_pg[(*mc).mc_top as usize];
            if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                & 0x2 as libc::c_int
                == 0x2 as libc::c_int
            {
            } else {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"IS_LEAF(mp)\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"mdb_cursor_set\0",
                    ))
                    .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    7237 as libc::c_int,
                );
            };
            current_block = 4018922828020914930;
        }
        _ => {}
    }
    match current_block {
        4018922828020914930 => {
            leaf = mdb_node_search(mc, key, exactp);
            if !exactp.is_null() && *exactp == 0 {
                return MDB_NOTFOUND;
            }
            if leaf.is_null() {
                rc = mdb_cursor_sibling(mc, 1 as libc::c_int);
                if rc != MDB_SUCCESS {
                    (*mc).mc_flags |= C_EOF as libc::c_uint;
                    return rc;
                }
                mp = (*mc).mc_pg[(*mc).mc_top as usize];
                if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                    & 0x2 as libc::c_int
                    == 0x2 as libc::c_int
                {
                } else {
                    mdb_assert_fail(
                        (*(*mc).mc_txn).mt_env,
                        b"IS_LEAF(mp)\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                            b"mdb_cursor_set\0",
                        ))
                        .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                        7253 as libc::c_int,
                    );
                };
                leaf = (mp as *mut libc::c_char)
                    .offset(
                        *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
            }
        }
        _ => {}
    }
    (*mc).mc_flags |= C_INITIALIZED as libc::c_uint;
    (*mc).mc_flags &= !C_EOF as libc::c_uint;
    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int & 0x20 as libc::c_int
        == 0x20 as libc::c_int
    {
        if op as libc::c_uint == MDB_SET_RANGE as libc::c_int as libc::c_uint
            || op as libc::c_uint == MDB_SET_KEY as libc::c_int as libc::c_uint
        {
            (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
            (*key).mv_data = (mp as *mut libc::c_char)
                .offset(PAGEHDRSZ as libc::c_uint as isize)
                .offset(((*mc).mc_ki[(*mc).mc_top as usize] as size_t * (*key).mv_size) as isize)
                as *mut libc::c_void;
        }
        return MDB_SUCCESS;
    }
    if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        if op as libc::c_uint == MDB_SET as libc::c_int as libc::c_uint
            || op as libc::c_uint == MDB_SET_KEY as libc::c_int as libc::c_uint
            || op as libc::c_uint == MDB_SET_RANGE as libc::c_int as libc::c_uint
        {
            rc = mdb_cursor_first(
                &mut (*(*mc).mc_xcursor).mx_cursor,
                data,
                NULL as *mut MDB_val,
            );
        } else {
            let mut ex2: libc::c_int = 0;
            let mut ex2p = 0 as *mut libc::c_int;
            if op as libc::c_uint == MDB_GET_BOTH as libc::c_int as libc::c_uint {
                ex2p = &mut ex2;
                ex2 = 0 as libc::c_int;
            } else {
                ex2p = NULL as *mut libc::c_int;
            }
            rc = mdb_cursor_set(
                &mut (*(*mc).mc_xcursor).mx_cursor,
                data,
                NULL as *mut MDB_val,
                MDB_SET_RANGE,
                ex2p,
            );
            if rc != MDB_SUCCESS {
                return rc;
            }
        }
    } else if !data.is_null() {
        if op as libc::c_uint == MDB_GET_BOTH as libc::c_int as libc::c_uint
            || op as libc::c_uint == MDB_GET_BOTH_RANGE as libc::c_int as libc::c_uint
        {
            let mut olddata = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            let mut dcmp: Option<MDB_cmp_func> = None;
            rc = mdb_node_read(mc, leaf, &mut olddata);
            if rc != MDB_SUCCESS {
                return rc;
            }
            dcmp = (*(*mc).mc_dbx).md_dcmp;
            if (UINT_MAX as libc::c_ulong) < MDB_SIZE_MAX
                && dcmp == Some(mdb_cmp_int as MDB_cmp_func)
                && olddata.mv_size == ::core::mem::size_of::<mdb_size_t>() as libc::c_ulong
            {
                dcmp = Some(mdb_cmp_clong);
            }
            rc = dcmp.expect("non-null function pointer")(data, &mut olddata);
            if rc != 0 {
                if op as libc::c_uint == MDB_GET_BOTH as libc::c_int as libc::c_uint
                    || rc > 0 as libc::c_int
                {
                    return MDB_NOTFOUND;
                }
                rc = 0 as libc::c_int;
            }
            *data = olddata;
        } else {
            if !((*mc).mc_xcursor).is_null() {
                (*(*mc).mc_xcursor).mx_cursor.mc_flags &= !(C_INITIALIZED | C_EOF) as libc::c_uint;
            }
            rc = mdb_node_read(mc, leaf, data);
            if rc != MDB_SUCCESS {
                return rc;
            }
        }
    }
    if op as libc::c_uint == MDB_SET_RANGE as libc::c_int as libc::c_uint
        || op as libc::c_uint == MDB_SET_KEY as libc::c_int as libc::c_uint
    {
        if !key.is_null() {
            (*key).mv_size = (*leaf).mn_ksize as size_t;
            (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
        }
    }
    return rc;
}
unsafe extern "C" fn mdb_cursor_first(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut leaf = 0 as *mut MDB_node;
    if !((*mc).mc_xcursor).is_null() {
        (*(*mc).mc_xcursor).mx_cursor.mc_flags &= !(C_INITIALIZED | C_EOF) as libc::c_uint;
    }
    if (*mc).mc_flags & C_INITIALIZED as libc::c_uint == 0 || (*mc).mc_top as libc::c_int != 0 {
        rc = mdb_page_search(mc, NULL as *mut MDB_val, MDB_PS_FIRST);
        if rc != MDB_SUCCESS {
            return rc;
        }
    }
    if (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_flags
        as libc::c_int
        & 0x2 as libc::c_int
        == 0x2 as libc::c_int
    {
    } else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"IS_LEAF(mc->mc_pg[mc->mc_top])\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"mdb_cursor_first\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            7335 as libc::c_int,
        );
    };
    leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
        .offset(
            *((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                .mp2_ptrs)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as libc::c_int as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    (*mc).mc_flags |= C_INITIALIZED as libc::c_uint;
    (*mc).mc_flags &= !C_EOF as libc::c_uint;
    (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
    if (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_flags
        as libc::c_int
        & 0x20 as libc::c_int
        == 0x20 as libc::c_int
    {
        if !key.is_null() {
            (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
            (*key).mv_data = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                .offset(PAGEHDRSZ as libc::c_uint as isize)
                .offset((0 as libc::c_int as size_t * (*key).mv_size) as isize)
                as *mut libc::c_void;
        }
        return MDB_SUCCESS;
    }
    if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        rc = mdb_cursor_first(
            &mut (*(*mc).mc_xcursor).mx_cursor,
            data,
            NULL as *mut MDB_val,
        );
        if rc != 0 {
            return rc;
        }
    } else if !data.is_null() {
        rc = mdb_node_read(mc, leaf, data);
        if rc != MDB_SUCCESS {
            return rc;
        }
    }
    if !key.is_null() {
        (*key).mv_size = (*leaf).mn_ksize as size_t;
        (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
    }
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_cursor_last(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut leaf = 0 as *mut MDB_node;
    if !((*mc).mc_xcursor).is_null() {
        (*(*mc).mc_xcursor).mx_cursor.mc_flags &= !(C_INITIALIZED | C_EOF) as libc::c_uint;
    }
    if (*mc).mc_flags & C_INITIALIZED as libc::c_uint == 0 || (*mc).mc_top as libc::c_int != 0 {
        rc = mdb_page_search(mc, NULL as *mut MDB_val, MDB_PS_LAST);
        if rc != MDB_SUCCESS {
            return rc;
        }
    }
    if (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_flags
        as libc::c_int
        & 0x2 as libc::c_int
        == 0x2 as libc::c_int
    {
    } else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"IS_LEAF(mc->mc_pg[mc->mc_top])\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"mdb_cursor_last\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            7382 as libc::c_int,
        );
    };
    (*mc).mc_ki[(*mc).mc_top as usize] =
        (((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_lower
            as libc::c_uint)
            .wrapping_sub(
                (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }),
            )
            >> 1 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
    (*mc).mc_flags |= (C_INITIALIZED | C_EOF) as libc::c_uint;
    leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
        .offset(
            *((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                .mp2_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    if (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_flags
        as libc::c_int
        & 0x20 as libc::c_int
        == 0x20 as libc::c_int
    {
        if !key.is_null() {
            (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
            (*key).mv_data = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                .offset(PAGEHDRSZ as libc::c_uint as isize)
                .offset(((*mc).mc_ki[(*mc).mc_top as usize] as size_t * (*key).mv_size) as isize)
                as *mut libc::c_void;
        }
        return MDB_SUCCESS;
    }
    if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        rc = mdb_cursor_last(
            &mut (*(*mc).mc_xcursor).mx_cursor,
            data,
            NULL as *mut MDB_val,
        );
        if rc != 0 {
            return rc;
        }
    } else if !data.is_null() {
        rc = mdb_node_read(mc, leaf, data);
        if rc != MDB_SUCCESS {
            return rc;
        }
    }
    if !key.is_null() {
        (*key).mv_size = (*leaf).mn_ksize as size_t;
        (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
    }
    return MDB_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_cursor_get(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
) -> libc::c_int {
    let mut mx: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut exact = 0 as libc::c_int;
    let mut mfunc: Option<
        unsafe extern "C" fn(*mut MDB_cursor, *mut MDB_val, *mut MDB_val) -> libc::c_int,
    > = None;
    if mc.is_null() {
        return EINVAL;
    }
    if (*(*mc).mc_txn).mt_flags & MDB_TXN_BLOCKED as libc::c_uint != 0 {
        return MDB_BAD_TXN;
    }
    match op as libc::c_uint {
        4 => {
            if (*mc).mc_flags & C_INITIALIZED as libc::c_uint == 0 {
                rc = EINVAL;
            } else {
                let mut mp = (*mc).mc_pg[(*mc).mc_top as usize];
                let mut nkeys = (((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                    as libc::c_uint)
                    .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                        if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        },
                    ))
                    >> 1 as libc::c_int) as libc::c_int;
                if nkeys == 0 || (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int >= nkeys {
                    (*mc).mc_ki[(*mc).mc_top as usize] = nkeys as indx_t;
                    rc = MDB_NOTFOUND;
                } else {
                    rc = MDB_SUCCESS;
                    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                        & 0x20 as libc::c_int
                        == 0x20 as libc::c_int
                    {
                        (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
                        (*key).mv_data = (mp as *mut libc::c_char)
                            .offset(PAGEHDRSZ as libc::c_uint as isize)
                            .offset(
                                ((*mc).mc_ki[(*mc).mc_top as usize] as size_t * (*key).mv_size)
                                    as isize,
                            ) as *mut libc::c_void;
                    } else {
                        let mut leaf = (mp as *mut libc::c_char)
                            .offset(
                                *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                    .as_mut_ptr()
                                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                    as libc::c_int as isize,
                            )
                            .offset(
                                (if 0 as libc::c_int != 0 {
                                    PAGEHDRSZ as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        if !key.is_null() {
                            (*key).mv_size = (*leaf).mn_ksize as size_t;
                            (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
                        }
                        if !data.is_null() {
                            if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int
                                == 0x4 as libc::c_int
                            {
                                rc = mdb_cursor_get(
                                    &mut (*(*mc).mc_xcursor).mx_cursor,
                                    data,
                                    NULL as *mut MDB_val,
                                    MDB_GET_CURRENT,
                                );
                            } else {
                                rc = mdb_node_read(mc, leaf, data);
                            }
                        }
                    }
                }
            }
            current_block = 1765866445182206997;
        }
        2 | 3 => {
            if data.is_null() {
                rc = EINVAL;
                current_block = 1765866445182206997;
            } else if ((*mc).mc_xcursor).is_null() {
                rc = MDB_INCOMPATIBLE;
                current_block = 1765866445182206997;
            } else {
                current_block = 10057808065911759103;
            }
        }
        15 | 16 | 17 => {
            current_block = 10057808065911759103;
        }
        5 => {
            if data.is_null() || (*mc).mc_flags & C_INITIALIZED as libc::c_uint == 0 {
                rc = EINVAL;
                current_block = 1765866445182206997;
            } else if (*(*mc).mc_db).md_flags as libc::c_int & MDB_DUPFIXED == 0 {
                rc = MDB_INCOMPATIBLE;
                current_block = 1765866445182206997;
            } else {
                rc = MDB_SUCCESS;
                if (*(*mc).mc_xcursor).mx_cursor.mc_flags & C_INITIALIZED as libc::c_uint == 0
                    || (*(*mc).mc_xcursor).mx_cursor.mc_flags & C_EOF as libc::c_uint != 0
                {
                    current_block = 1765866445182206997;
                } else {
                    current_block = 10292515271029616820;
                }
            }
        }
        10 => {
            if data.is_null() {
                rc = EINVAL;
                current_block = 1765866445182206997;
            } else if (*(*mc).mc_db).md_flags as libc::c_int & MDB_DUPFIXED == 0 {
                rc = MDB_INCOMPATIBLE;
                current_block = 1765866445182206997;
            } else {
                rc = mdb_cursor_next(mc, key, data, MDB_NEXT_DUP);
                if rc == MDB_SUCCESS {
                    if (*(*mc).mc_xcursor).mx_cursor.mc_flags & C_INITIALIZED as libc::c_uint != 0 {
                        mx = 0 as *mut MDB_cursor;
                        current_block = 10292515271029616820;
                    } else {
                        rc = MDB_NOTFOUND;
                        current_block = 1765866445182206997;
                    }
                } else {
                    current_block = 1765866445182206997;
                }
            }
        }
        18 => {
            if data.is_null() {
                rc = EINVAL;
                current_block = 1765866445182206997;
            } else if (*(*mc).mc_db).md_flags as libc::c_int & MDB_DUPFIXED == 0 {
                rc = MDB_INCOMPATIBLE;
                current_block = 1765866445182206997;
            } else {
                if (*mc).mc_flags & C_INITIALIZED as libc::c_uint == 0 {
                    rc = mdb_cursor_last(mc, key, data);
                } else {
                    rc = MDB_SUCCESS;
                }
                if rc == MDB_SUCCESS {
                    let mut mx_0: *mut MDB_cursor = &mut (*(*mc).mc_xcursor).mx_cursor;
                    if (*mx_0).mc_flags & C_INITIALIZED as libc::c_uint != 0 {
                        rc = mdb_cursor_sibling(mx_0, 0 as libc::c_int);
                        if rc == MDB_SUCCESS {
                            current_block = 10292515271029616820;
                        } else {
                            current_block = 1765866445182206997;
                        }
                    } else {
                        rc = MDB_NOTFOUND;
                        current_block = 1765866445182206997;
                    }
                } else {
                    current_block = 1765866445182206997;
                }
            }
        }
        8 | 9 | 11 => {
            rc = mdb_cursor_next(mc, key, data, op);
            current_block = 1765866445182206997;
        }
        12 | 13 | 14 => {
            rc = mdb_cursor_prev(mc, key, data, op);
            current_block = 1765866445182206997;
        }
        0 => {
            rc = mdb_cursor_first(mc, key, data);
            current_block = 1765866445182206997;
        }
        1 => {
            mfunc = Some(
                mdb_cursor_first
                    as unsafe extern "C" fn(
                        *mut MDB_cursor,
                        *mut MDB_val,
                        *mut MDB_val,
                    ) -> libc::c_int,
            );
            current_block = 14107383661081181667;
        }
        6 => {
            rc = mdb_cursor_last(mc, key, data);
            current_block = 1765866445182206997;
        }
        7 => {
            mfunc = Some(
                mdb_cursor_last
                    as unsafe extern "C" fn(
                        *mut MDB_cursor,
                        *mut MDB_val,
                        *mut MDB_val,
                    ) -> libc::c_int,
            );
            current_block = 14107383661081181667;
        }
        _ => {
            rc = EINVAL;
            current_block = 1765866445182206997;
        }
    }
    match current_block {
        14107383661081181667 => {
            if data.is_null() || (*mc).mc_flags & C_INITIALIZED as libc::c_uint == 0 {
                rc = EINVAL;
            } else if ((*mc).mc_xcursor).is_null() {
                rc = MDB_INCOMPATIBLE;
            } else if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                >= ((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                    .mp2_lower as libc::c_uint)
                    .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                        if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        },
                    ))
                    >> 1 as libc::c_int
            {
                (*mc).mc_ki[(*mc).mc_top as usize] = (((*((*mc).mc_pg[(*mc).mc_top as usize]
                    as *mut libc::c_void
                    as *mut MDB_page2))
                    .mp2_lower
                    as libc::c_uint)
                    .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                        if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        },
                    ))
                    >> 1 as libc::c_int)
                    as indx_t;
                rc = MDB_NOTFOUND;
            } else {
                (*mc).mc_flags &= !C_EOF as libc::c_uint;
                let mut leaf_0 = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void
                            as *mut MDB_page2))
                            .mp2_ptrs)
                            .as_mut_ptr()
                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                if !((*leaf_0).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int) {
                    if !key.is_null() {
                        (*key).mv_size = (*leaf_0).mn_ksize as size_t;
                        (*key).mv_data = ((*leaf_0).mn_data).as_mut_ptr() as *mut libc::c_void;
                    }
                    rc = mdb_node_read(mc, leaf_0, data);
                } else if (*(*mc).mc_xcursor).mx_cursor.mc_flags & C_INITIALIZED as libc::c_uint
                    == 0
                {
                    rc = EINVAL;
                } else {
                    rc = mfunc.expect("non-null function pointer")(
                        &mut (*(*mc).mc_xcursor).mx_cursor,
                        data,
                        NULL as *mut MDB_val,
                    );
                }
            }
        }
        10292515271029616820 => {
            mx = &mut (*(*mc).mc_xcursor).mx_cursor;
            (*data).mv_size = (((*((*mx).mc_pg[(*mx).mc_top as usize] as *mut libc::c_void
                as *mut MDB_page2))
                .mp2_lower as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int)
                .wrapping_mul((*(*mx).mc_db).md_pad) as size_t;
            (*data).mv_data = ((*mx).mc_pg[(*mx).mc_top as usize] as *mut libc::c_char)
                .offset(PAGEHDRSZ as libc::c_uint as isize)
                as *mut libc::c_void;
            (*mx).mc_ki[(*mx).mc_top as usize] =
                (((*((*mx).mc_pg[(*mx).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                    .mp2_lower as libc::c_uint)
                    .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                        if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        },
                    ))
                    >> 1 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
        }
        10057808065911759103 => {
            if key.is_null() {
                rc = EINVAL;
            } else {
                rc = mdb_cursor_set(
                    mc,
                    key,
                    data,
                    op,
                    if op as libc::c_uint == MDB_SET_RANGE as libc::c_int as libc::c_uint {
                        NULL as *mut libc::c_int
                    } else {
                        &mut exact
                    },
                );
            }
        }
        _ => {}
    }
    if (*mc).mc_flags & C_DEL as libc::c_uint != 0 {
        (*mc).mc_flags ^= C_DEL as libc::c_uint;
    }
    return rc;
}
unsafe extern "C" fn mdb_cursor_touch(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut rc = MDB_SUCCESS;
    if (*mc).mc_dbi >= CORE_DBS as MDB_dbi
        && *(*mc).mc_dbflag as libc::c_int & (DB_DIRTY | DB_DUPDATA) == 0
    {
        let mut mc2 = MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        };
        let mut mcx = MDB_xcursor {
            mx_cursor: MDB_cursor {
                mc_next: 0 as *mut MDB_cursor,
                mc_backup: 0 as *mut MDB_cursor,
                mc_xcursor: 0 as *mut MDB_xcursor,
                mc_txn: 0 as *mut MDB_txn,
                mc_dbi: 0,
                mc_db: 0 as *mut MDB_db,
                mc_dbx: 0 as *mut MDB_dbx,
                mc_dbflag: 0 as *mut libc::c_uchar,
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [0 as *mut MDB_page; 32],
                mc_ki: [0; 32],
            },
            mx_db: MDB_db {
                md_pad: 0,
                md_flags: 0,
                md_depth: 0,
                md_branch_pages: 0,
                md_leaf_pages: 0,
                md_overflow_pages: 0,
                md_entries: 0,
                md_root: 0,
            },
            mx_dbx: MDB_dbx {
                md_name: MDB_val {
                    mv_size: 0,
                    mv_data: 0 as *mut libc::c_void,
                },
                md_cmp: None,
                md_dcmp: None,
                md_rel: None,
                md_relctx: 0 as *mut libc::c_void,
            },
            mx_dbflag: 0,
        };
        if *((*(*mc).mc_txn).mt_dbiseqs).offset((*mc).mc_dbi as isize)
            != *((*(*(*mc).mc_txn).mt_env).me_dbiseqs).offset((*mc).mc_dbi as isize)
        {
            return MDB_BAD_DBI;
        }
        mdb_cursor_init(&mut mc2, (*mc).mc_txn, MAIN_DBI as MDB_dbi, &mut mcx);
        rc = mdb_page_search(&mut mc2, &mut (*(*mc).mc_dbx).md_name, MDB_PS_MODIFY);
        if rc != 0 {
            return rc;
        }
        *(*mc).mc_dbflag = (*(*mc).mc_dbflag as libc::c_int | DB_DIRTY) as libc::c_uchar;
    }
    (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
    if (*mc).mc_snum != 0 {
        loop {
            rc = mdb_page_touch(mc);
            if !(rc == 0 && {
                (*mc).mc_top = ((*mc).mc_top).wrapping_add(1);
                ((*mc).mc_top as libc::c_int) < (*mc).mc_snum as libc::c_int
            }) {
                break;
            }
        }
        (*mc).mc_top = ((*mc).mc_snum as libc::c_int - 1 as libc::c_int) as libc::c_ushort;
    }
    return rc;
}
pub const MDB_NOSPILL: libc::c_int = 0x8000 as libc::c_int;
unsafe extern "C" fn _mdb_cursor_put(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut offset: libc::c_uint = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ksize: libc::c_uint = 0;
    let mut xflags: libc::c_int = 0;
    let mut new_dupdata: libc::c_int = 0;
    let mut ecount: mdb_size_t = 0;
    let mut current_block: u64;
    let mut env = 0 as *mut MDB_env;
    let mut leaf = NULL as *mut MDB_node;
    let mut fp = 0 as *mut MDB_page;
    let mut mp = 0 as *mut MDB_page;
    let mut sub_root = NULL as *mut MDB_page;
    let mut fp_flags: uint16_t = 0;
    let mut xdata = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut rdata = 0 as *mut MDB_val;
    let mut dkey = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut olddata = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut dummy = MDB_db {
        md_pad: 0,
        md_flags: 0,
        md_depth: 0,
        md_branch_pages: 0,
        md_leaf_pages: 0,
        md_overflow_pages: 0,
        md_entries: 0,
        md_root: 0,
    };
    let mut do_sub = 0 as libc::c_int;
    let mut insert_key: libc::c_int = 0;
    let mut insert_data: libc::c_int = 0;
    let mut mcount = 0 as libc::c_int as libc::c_uint;
    let mut dcount = 0 as libc::c_int as libc::c_uint;
    let mut nospill: libc::c_uint = 0;
    let mut nsize: size_t = 0;
    let mut rc: libc::c_int = 0;
    let mut rc2: libc::c_int = 0;
    let mut nflags: libc::c_uint = 0;
    if mc.is_null() || key.is_null() {
        return EINVAL;
    }
    env = (*(*mc).mc_txn).mt_env;
    if flags & MDB_MULTIPLE as libc::c_uint != 0 {
        dcount = (*data.offset(1 as libc::c_int as isize)).mv_size as libc::c_uint;
        (*data.offset(1 as libc::c_int as isize)).mv_size = 0 as libc::c_int as size_t;
        if !((*(*mc).mc_db).md_flags as libc::c_int & 0x10 as libc::c_int == 0x10 as libc::c_int) {
            return MDB_INCOMPATIBLE;
        }
    }
    nospill = flags & MDB_NOSPILL as libc::c_uint;
    flags &= !MDB_NOSPILL as libc::c_uint;
    if (*(*mc).mc_txn).mt_flags & (MDB_TXN_RDONLY | MDB_TXN_BLOCKED) as libc::c_uint != 0 {
        return if (*(*mc).mc_txn).mt_flags & MDB_TXN_RDONLY as libc::c_uint != 0 {
            EACCES
        } else {
            MDB_BAD_TXN
        };
    }
    if ((*key).mv_size).wrapping_sub(1 as libc::c_int as size_t)
        >= (if 0 as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            511 as libc::c_int
        }) as size_t
    {
        return MDB_BAD_VALSIZE;
    }
    if (*data).mv_size
        > (if (*(*mc).mc_db).md_flags as libc::c_int & MDB_DUPSORT != 0 {
            (if 0 as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                511 as libc::c_int
            }) as libc::c_ulong
        } else {
            MAXDATASIZE
        })
    {
        return MDB_BAD_VALSIZE;
    }
    dkey.mv_size = 0 as libc::c_int as size_t;
    if flags & MDB_CURRENT as libc::c_uint != 0 {
        if (*mc).mc_flags & C_INITIALIZED as libc::c_uint == 0 {
            return EINVAL;
        }
        rc = MDB_SUCCESS;
    } else if (*(*mc).mc_db).md_root == P_INVALID {
        (*mc).mc_snum = 0 as libc::c_int as libc::c_ushort;
        (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
        (*mc).mc_flags &= !C_INITIALIZED as libc::c_uint;
        rc = MDB_NO_ROOT;
    } else {
        let mut exact = 0 as libc::c_int;
        let mut d2 = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        if flags & MDB_APPEND as libc::c_uint != 0 {
            let mut k2 = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            rc = mdb_cursor_last(mc, &mut k2, &mut d2);
            if rc == 0 as libc::c_int {
                rc = ((*(*mc).mc_dbx).md_cmp).expect("non-null function pointer")(key, &mut k2);
                if rc > 0 as libc::c_int {
                    rc = MDB_NOTFOUND;
                    (*mc).mc_ki[(*mc).mc_top as usize] =
                        ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_add(1);
                    (*mc).mc_ki[(*mc).mc_top as usize];
                } else {
                    rc = MDB_KEYEXIST;
                }
            }
        } else {
            rc = mdb_cursor_set(mc, key, &mut d2, MDB_SET, &mut exact);
        }
        if flags & MDB_NOOVERWRITE as libc::c_uint != 0 && rc == 0 as libc::c_int {
            *data = d2;
            return MDB_KEYEXIST;
        }
        if rc != 0 && rc != MDB_NOTFOUND {
            return rc;
        }
    }
    if (*mc).mc_flags & C_DEL as libc::c_uint != 0 {
        (*mc).mc_flags ^= C_DEL as libc::c_uint;
    }
    if nospill == 0 {
        if flags & MDB_MULTIPLE as libc::c_uint != 0 {
            rdata = &mut xdata;
            xdata.mv_size = (*data).mv_size * dcount as size_t;
        } else {
            rdata = data;
        }
        rc2 = mdb_page_spill(mc, key, rdata);
        if rc2 != 0 {
            return rc2;
        }
    }
    if rc == MDB_NO_ROOT {
        let mut np = 0 as *mut MDB_page;
        rc2 = mdb_page_new(mc, P_LEAF as uint32_t, 1 as libc::c_int, &mut np);
        if rc2 != 0 {
            return rc2;
        }
        mdb_cursor_push(mc, np);
        (*(*mc).mc_db).md_root = (*np).mp_p.p_pgno;
        (*(*mc).mc_db).md_depth = ((*(*mc).mc_db).md_depth).wrapping_add(1);
        (*(*mc).mc_db).md_depth;
        *(*mc).mc_dbflag = (*(*mc).mc_dbflag as libc::c_int | DB_DIRTY) as libc::c_uchar;
        if (*(*mc).mc_db).md_flags as libc::c_int & (MDB_DUPSORT | MDB_DUPFIXED) == MDB_DUPFIXED {
            let ref mut fresh30 = (*(np as *mut libc::c_void as *mut MDB_page2)).mp2_flags;
            *fresh30 = (*fresh30 as libc::c_int | P_LEAF2) as uint16_t;
        }
        (*mc).mc_flags |= C_INITIALIZED as libc::c_uint;
    } else {
        rc2 = mdb_cursor_touch(mc);
        if rc2 != 0 {
            return rc2;
        }
    }
    insert_data = rc;
    insert_key = insert_data;
    if insert_key != 0 {
        if (*(*mc).mc_db).md_flags as libc::c_int & MDB_DUPSORT != 0
            && NODESIZE
                .wrapping_add((*key).mv_size)
                .wrapping_add((*data).mv_size)
                > (*env).me_nodemax as libc::c_ulong
        {
            fp_flags = (P_LEAF | P_DIRTY) as uint16_t;
            fp = (*env).me_pbuf as *mut MDB_page;
            (*fp).mp_pad = (*data).mv_size as uint16_t;
            let ref mut fresh31 = (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_upper;
            *fresh31 = (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as indx_t;
            (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_lower = *fresh31;
            olddata.mv_size = PAGEHDRSZ;
            current_block = 4872821565891422174;
        } else {
            current_block = 17353026871531185123;
        }
    } else if (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
        .mp2_flags as libc::c_int
        & 0x20 as libc::c_int
        == 0x20 as libc::c_int
    {
        ptr = 0 as *mut libc::c_char;
        ksize = (*(*mc).mc_db).md_pad;
        if (*key).mv_size != ksize as size_t {
            return MDB_BAD_VALSIZE;
        }
        ptr = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
            .offset(PAGEHDRSZ as libc::c_uint as isize)
            .offset(
                ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint).wrapping_mul(ksize) as isize,
            );
        memcpy(
            ptr as *mut libc::c_void,
            (*key).mv_data,
            ksize as libc::c_ulong,
        );
        current_block = 16240365560028363851;
    } else {
        current_block = 6503087309128928598;
    }
    loop {
        match current_block {
            17353026871531185123 => {
                rdata = data;
                current_block = 866732880969548764;
            }
            4872821565891422174 => {
                if (*(*mc).mc_db).md_flags as libc::c_int & MDB_DUPFIXED != 0 {
                    fp_flags = (fp_flags as libc::c_int | P_LEAF2) as uint16_t;
                    dummy.md_pad = (*fp).mp_pad as uint32_t;
                    dummy.md_flags = MDB_DUPFIXED as uint16_t;
                    if (*(*mc).mc_db).md_flags as libc::c_int & MDB_INTEGERDUP != 0 {
                        dummy.md_flags =
                            (dummy.md_flags as libc::c_int | MDB_INTEGERKEY) as uint16_t;
                    }
                } else {
                    dummy.md_pad = 0 as libc::c_int as uint32_t;
                    dummy.md_flags = 0 as libc::c_int as uint16_t;
                }
                dummy.md_depth = 1 as libc::c_int as uint16_t;
                dummy.md_branch_pages = 0 as libc::c_int as pgno_t;
                dummy.md_leaf_pages = 1 as libc::c_int as pgno_t;
                dummy.md_overflow_pages = 0 as libc::c_int as pgno_t;
                dummy.md_entries = (((*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                    as libc::c_uint)
                    .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                        if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        },
                    ))
                    >> 1 as libc::c_int) as mdb_size_t;
                xdata.mv_size = ::core::mem::size_of::<MDB_db>() as libc::c_ulong;
                xdata.mv_data = &mut dummy as *mut MDB_db as *mut libc::c_void;
                rc = mdb_page_alloc(mc, 1 as libc::c_int, &mut mp);
                if rc != 0 {
                    return rc;
                }
                offset = ((*env).me_psize as size_t).wrapping_sub(olddata.mv_size) as libc::c_uint;
                flags |= (F_DUPDATA | F_SUBDATA) as libc::c_uint;
                dummy.md_root = (*mp).mp_p.p_pgno;
                sub_root = mp;
                current_block = 10601179871800211547;
            }
            16240365560028363851 => {
                if (*mc).mc_top as libc::c_int != 0 && (*mc).mc_ki[(*mc).mc_top as usize] == 0 {
                    let mut dtop = 1 as libc::c_int as libc::c_ushort;
                    (*mc).mc_top = ((*mc).mc_top).wrapping_sub(1);
                    (*mc).mc_top;
                    while (*mc).mc_top as libc::c_int != 0
                        && (*mc).mc_ki[(*mc).mc_top as usize] == 0
                    {
                        (*mc).mc_top = ((*mc).mc_top).wrapping_sub(1);
                        (*mc).mc_top;
                        dtop = dtop.wrapping_add(1);
                        dtop;
                    }
                    if (*mc).mc_ki[(*mc).mc_top as usize] != 0 {
                        rc2 = mdb_update_key(mc, key);
                    } else {
                        rc2 = MDB_SUCCESS;
                    }
                    (*mc).mc_top =
                        ((*mc).mc_top as libc::c_int + dtop as libc::c_int) as libc::c_ushort;
                    if rc2 != 0 {
                        return rc2;
                    }
                }
                return MDB_SUCCESS;
            }
            _ => {
                leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void
                            as *mut MDB_page2))
                            .mp2_ptrs)
                            .as_mut_ptr()
                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                olddata.mv_size = ((*leaf).mn_lo as libc::c_uint
                    | ((*leaf).mn_hi as libc::c_uint) << 16 as libc::c_int)
                    as size_t;
                olddata.mv_data = ((*leaf).mn_data)
                    .as_mut_ptr()
                    .offset((*leaf).mn_ksize as libc::c_int as isize)
                    as *mut libc::c_void;
                if (*(*mc).mc_db).md_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int
                {
                    i = 0;
                    offset = 0 as libc::c_int as libc::c_uint;
                    xdata.mv_data = (*env).me_pbuf;
                    fp = xdata.mv_data as *mut MDB_page;
                    mp = fp;
                    (*mp).mp_p.p_pgno = (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_p.p_pgno;
                    if !((*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int)
                    {
                        let mut dcmp: Option<MDB_cmp_func> = None;
                        if flags == MDB_CURRENT as libc::c_uint {
                            current_block = 3799958390938788923;
                        } else {
                            dcmp = (*(*mc).mc_dbx).md_dcmp;
                            if (UINT_MAX as libc::c_ulong) < MDB_SIZE_MAX
                                && dcmp == Some(mdb_cmp_int as MDB_cmp_func)
                                && olddata.mv_size
                                    == ::core::mem::size_of::<mdb_size_t>() as libc::c_ulong
                            {
                                dcmp = Some(mdb_cmp_clong);
                            }
                            if dcmp.expect("non-null function pointer")(data, &mut olddata) == 0 {
                                if flags & (MDB_NODUPDATA | MDB_APPENDDUP) as libc::c_uint != 0 {
                                    return MDB_KEYEXIST;
                                }
                                current_block = 3799958390938788923;
                            } else {
                                dkey.mv_size = olddata.mv_size;
                                dkey.mv_data = memcpy(
                                    fp.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                    olddata.mv_data,
                                    olddata.mv_size,
                                );
                                (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_flags =
                                    (P_LEAF | P_DIRTY | P_SUBP) as uint16_t;
                                (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_lower =
                                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                        if 0 as libc::c_int != 0 {
                                            PAGEHDRSZ as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        },
                                    ) as indx_t;
                                xdata.mv_size = PAGEHDRSZ
                                    .wrapping_add(dkey.mv_size)
                                    .wrapping_add((*data).mv_size);
                                if (*(*mc).mc_db).md_flags as libc::c_int & MDB_DUPFIXED != 0 {
                                    let ref mut fresh32 =
                                        (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_flags;
                                    *fresh32 = (*fresh32 as libc::c_int | P_LEAF2) as uint16_t;
                                    (*fp).mp_pad = (*data).mv_size as uint16_t;
                                    xdata.mv_size = (xdata.mv_size)
                                        .wrapping_add(2 as libc::c_int as size_t * (*data).mv_size);
                                } else {
                                    xdata.mv_size = (xdata.mv_size as libc::c_ulong).wrapping_add(
                                        (2 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(
                                                (::core::mem::size_of::<indx_t>() as libc::c_ulong)
                                                    .wrapping_add(NODESIZE),
                                            )
                                            .wrapping_add(dkey.mv_size & 1 as libc::c_int as size_t)
                                            .wrapping_add(
                                                (*data).mv_size & 1 as libc::c_int as size_t,
                                            ),
                                    ) as size_t
                                        as size_t;
                                }
                                (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_upper =
                                    (xdata.mv_size).wrapping_sub(
                                        (if 0 as libc::c_int != 0 {
                                            PAGEHDRSZ as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        }) as size_t,
                                    ) as indx_t;
                                olddata.mv_size = xdata.mv_size;
                                current_block = 248631179418912492;
                            }
                        }
                    } else if (*leaf).mn_flags as libc::c_int & F_SUBDATA != 0 {
                        flags |= (F_DUPDATA | F_SUBDATA) as libc::c_uint;
                        current_block = 5681788516532307688;
                    } else {
                        fp = olddata.mv_data as *mut MDB_page;
                        match flags {
                            64 => {
                                current_block = 10827009522389732452;
                            }
                            _ => {
                                if (*(*mc).mc_db).md_flags as libc::c_int & MDB_DUPFIXED == 0 {
                                    offset = ((8 as libc::c_ulong)
                                        .wrapping_add(
                                            ::core::mem::size_of::<indx_t>() as libc::c_ulong
                                        )
                                        .wrapping_add((*data).mv_size)
                                        .wrapping_add(1 as libc::c_uint as libc::c_ulong)
                                        & -(2 as libc::c_int) as libc::c_ulong)
                                        as libc::c_uint;
                                    current_block = 6897179874198677617;
                                } else {
                                    offset = (*fp).mp_pad as libc::c_uint;
                                    if (((*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_upper
                                        as libc::c_int
                                        - (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                                            as libc::c_int)
                                        as indx_t
                                        as libc::c_uint)
                                        < offset
                                    {
                                        offset =
                                            offset.wrapping_mul(4 as libc::c_int as libc::c_uint);
                                        current_block = 6897179874198677617;
                                    } else {
                                        current_block = 10827009522389732452;
                                    }
                                }
                                match current_block {
                                    10827009522389732452 => {}
                                    _ => {
                                        xdata.mv_size =
                                            (olddata.mv_size).wrapping_add(offset as size_t);
                                        current_block = 248631179418912492;
                                    }
                                }
                            }
                        }
                        match current_block {
                            248631179418912492 => {}
                            _ => {
                                let ref mut fresh33 =
                                    (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_flags;
                                *fresh33 = (*fresh33 as libc::c_int | P_DIRTY) as uint16_t;
                                let mut s = 0 as *mut libc::c_ushort;
                                let mut d = 0 as *mut libc::c_ushort;
                                s = &mut (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_p
                                    as *mut [uint16_t; 4]
                                    as *mut libc::c_ushort;
                                d = &mut (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_p
                                    as *mut [uint16_t; 4]
                                    as *mut libc::c_ushort;
                                let fresh34 = s;
                                s = s.offset(1);
                                let fresh35 = d;
                                d = d.offset(1);
                                *fresh35 = *fresh34;
                                let fresh36 = s;
                                s = s.offset(1);
                                let fresh37 = d;
                                d = d.offset(1);
                                *fresh37 = *fresh36;
                                let fresh38 = s;
                                s = s.offset(1);
                                let fresh39 = d;
                                d = d.offset(1);
                                *fresh39 = *fresh38;
                                *d = *s;
                                (*(*mc).mc_xcursor).mx_cursor.mc_pg[0 as libc::c_int as usize] = fp;
                                flags |= F_DUPDATA as libc::c_uint;
                                current_block = 5681788516532307688;
                            }
                        }
                    }
                    match current_block {
                        5681788516532307688 => {}
                        3799958390938788923 => {}
                        _ => {
                            fp_flags = (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_flags;
                            if NODESIZE
                                .wrapping_add((*leaf).mn_ksize as libc::c_ulong)
                                .wrapping_add(xdata.mv_size)
                                > (*env).me_nodemax as libc::c_ulong
                            {
                                fp_flags = (fp_flags as libc::c_int & !P_SUBP) as uint16_t;
                                current_block = 4872821565891422174;
                                continue;
                            } else {
                                current_block = 10601179871800211547;
                            }
                        }
                    }
                } else {
                    current_block = 3799958390938788923;
                }
                match current_block {
                    5681788516532307688 => {}
                    10601179871800211547 => {}
                    _ => {
                        if ((*leaf).mn_flags as libc::c_uint ^ flags) & F_SUBDATA as libc::c_uint
                            != 0
                        {
                            return MDB_INCOMPATIBLE;
                        }
                        if (*leaf).mn_flags as libc::c_int & 0x1 as libc::c_int
                            == 0x1 as libc::c_int
                        {
                            let mut omp = 0 as *mut MDB_page;
                            let mut pg: pgno_t = 0;
                            let mut level: libc::c_int = 0;
                            let mut ovpages: libc::c_int = 0;
                            let mut dpages = (((PAGEHDRSZ as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as size_t)
                                .wrapping_add((*data).mv_size)
                                / (*env).me_psize as size_t)
                                .wrapping_add(1 as libc::c_int as size_t)
                                as libc::c_int;
                            memcpy(
                                &mut pg as *mut pgno_t as *mut libc::c_void,
                                olddata.mv_data,
                                ::core::mem::size_of::<pgno_t>() as libc::c_ulong,
                            );
                            rc2 = mdb_page_get(mc, pg, &mut omp, &mut level);
                            if rc2 != 0 as libc::c_int {
                                return rc2;
                            }
                            ovpages = (*omp).mp_pb.pb_pages as libc::c_int;
                            if ovpages >= dpages {
                                if (*omp).mp_flags as libc::c_int & P_DIRTY == 0
                                    && (level != 0
                                        || (*env).me_flags & MDB_WRITEMAP as uint32_t != 0)
                                {
                                    rc = mdb_page_unspill((*mc).mc_txn, omp, &mut omp);
                                    if rc != 0 {
                                        return rc;
                                    }
                                    level = 0 as libc::c_int;
                                }
                                if (*omp).mp_flags as libc::c_int & P_DIRTY != 0 {
                                    if level > 1 as libc::c_int {
                                        let mut sz = (*env).me_psize as size_t * ovpages as size_t;
                                        let mut off: size_t = 0;
                                        let mut np_0 =
                                            mdb_page_malloc((*mc).mc_txn, ovpages as libc::c_uint);
                                        let mut id2 = MDB_ID2 {
                                            mid: 0,
                                            mptr: 0 as *mut libc::c_void,
                                        };
                                        if np_0.is_null() {
                                            return ENOMEM;
                                        }
                                        id2.mid = pg;
                                        id2.mptr = np_0 as *mut libc::c_void;
                                        rc2 = mdb_mid2l_insert(
                                            (*(*mc).mc_txn).mt_u.dirty_list,
                                            &mut id2,
                                        );
                                        if rc2 == 0 as libc::c_int {
                                        } else {
                                            mdb_assert_fail(
                                                (*(*mc).mc_txn).mt_env,
                                                b"rc2 == 0\0" as *const u8 as *const libc::c_char,
                                                (*::core::mem::transmute::<
                                                    &[u8; 16],
                                                    &[libc::c_char; 16],
                                                >(
                                                    b"_mdb_cursor_put\0"
                                                ))
                                                .as_ptr(),
                                                b"mdb.c\0" as *const u8 as *const libc::c_char,
                                                7985 as libc::c_int,
                                            );
                                        };
                                        if flags & MDB_RESERVE as libc::c_uint == 0 {
                                            off = PAGEHDRSZ.wrapping_add((*data).mv_size)
                                                & -(::core::mem::size_of::<size_t>()
                                                    as libc::c_ulong
                                                    as libc::c_int)
                                                    as size_t;
                                            memcpy(
                                                (np_0 as *mut libc::c_char).offset(off as isize)
                                                    as *mut size_t
                                                    as *mut libc::c_void,
                                                (omp as *mut libc::c_char).offset(off as isize)
                                                    as *mut size_t
                                                    as *const libc::c_void,
                                                sz.wrapping_sub(off),
                                            );
                                            sz = PAGEHDRSZ;
                                        }
                                        memcpy(
                                            np_0 as *mut libc::c_void,
                                            omp as *const libc::c_void,
                                            sz,
                                        );
                                        omp = np_0;
                                    }
                                    (*leaf).mn_lo = ((*data).mv_size
                                        & 0xffff as libc::c_int as size_t)
                                        as libc::c_ushort;
                                    (*leaf).mn_hi =
                                        ((*data).mv_size >> 16 as libc::c_int) as libc::c_ushort;
                                    if flags & 0x10000 as libc::c_int as libc::c_uint
                                        == 0x10000 as libc::c_int as libc::c_uint
                                    {
                                        (*data).mv_data = (omp as *mut libc::c_char)
                                            .offset(PAGEHDRSZ as libc::c_uint as isize)
                                            as *mut libc::c_void;
                                    } else {
                                        memcpy(
                                            (omp as *mut libc::c_char)
                                                .offset(PAGEHDRSZ as libc::c_uint as isize)
                                                as *mut libc::c_void,
                                            (*data).mv_data,
                                            (*data).mv_size,
                                        );
                                    }
                                    return MDB_SUCCESS;
                                }
                            }
                            rc2 = mdb_ovpage_free(mc, omp);
                            if rc2 != MDB_SUCCESS {
                                return rc2;
                            }
                        } else if (*data).mv_size == olddata.mv_size {
                            if flags & 0x10000 as libc::c_int as libc::c_uint
                                == 0x10000 as libc::c_int as libc::c_uint
                            {
                                (*data).mv_data = olddata.mv_data;
                                current_block = 10257223768985283691;
                                break;
                            } else if (*mc).mc_flags & C_SUB as libc::c_uint == 0 {
                                memcpy(olddata.mv_data, (*data).mv_data, (*data).mv_size);
                                current_block = 10257223768985283691;
                                break;
                            } else if !((*key).mv_size != (*leaf).mn_ksize as size_t) {
                                memcpy(
                                    ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void,
                                    (*key).mv_data,
                                    (*key).mv_size,
                                );
                                current_block = 16240365560028363851;
                                continue;
                            }
                        }
                        mdb_node_del(mc, 0 as libc::c_int);
                        current_block = 17353026871531185123;
                        continue;
                    }
                }
            }
        }
        match current_block {
            10601179871800211547 => {
                if mp != fp {
                    (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags =
                        (fp_flags as libc::c_int | P_DIRTY) as uint16_t;
                    (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_pad =
                        (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_pad;
                    (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower =
                        (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_lower;
                    (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_upper =
                        ((*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_upper as libc::c_uint)
                            .wrapping_add(offset) as indx_t;
                    if fp_flags as libc::c_int & P_LEAF2 != 0 {
                        memcpy(
                            (mp as *mut libc::c_char).offset(PAGEHDRSZ as libc::c_uint as isize)
                                as *mut libc::c_void,
                            (fp as *mut libc::c_char).offset(PAGEHDRSZ as libc::c_uint as isize)
                                as *mut libc::c_void,
                            (((*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                                as libc::c_uint)
                                .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                    if 0 as libc::c_int != 0 {
                                        PAGEHDRSZ as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    },
                                ))
                                >> 1 as libc::c_int)
                                .wrapping_mul((*fp).mp_pad as libc::c_uint)
                                as libc::c_ulong,
                        );
                    } else {
                        memcpy(
                            (mp as *mut libc::c_char)
                                .offset(
                                    (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_upper
                                        as libc::c_int as isize,
                                )
                                .offset(
                                    (if 0 as libc::c_int != 0 {
                                        PAGEHDRSZ as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as isize,
                                ) as *mut libc::c_void,
                            (fp as *mut libc::c_char)
                                .offset(
                                    (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_upper
                                        as libc::c_int as isize,
                                )
                                .offset(
                                    (if 0 as libc::c_int != 0 {
                                        PAGEHDRSZ as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as isize,
                                ) as *const libc::c_void,
                            (olddata.mv_size)
                                .wrapping_sub(
                                    (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_upper
                                        as size_t,
                                )
                                .wrapping_sub(
                                    (if 0 as libc::c_int != 0 {
                                        PAGEHDRSZ as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as size_t,
                                ),
                        );
                        memcpy(
                            ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs).as_mut_ptr()
                                as *mut libc::c_char
                                as *mut libc::c_void,
                            ((*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs).as_mut_ptr()
                                as *mut libc::c_char
                                as *const libc::c_void,
                            ((((*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                                as libc::c_uint)
                                .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                    if 0 as libc::c_int != 0 {
                                        PAGEHDRSZ as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    },
                                ))
                                >> 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(::core::mem::size_of::<indx_t>() as libc::c_ulong),
                        );
                        i = 0 as libc::c_int as libc::c_uint;
                        while i
                            < ((*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                                as libc::c_uint)
                                .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                    if 0 as libc::c_int != 0 {
                                        PAGEHDRSZ as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    },
                                ))
                                >> 1 as libc::c_int
                        {
                            let ref mut fresh40 = *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize);
                            *fresh40 =
                                (*fresh40 as libc::c_uint).wrapping_add(offset) as indx_t as indx_t;
                            i = i.wrapping_add(1);
                            i;
                        }
                    }
                }
                rdata = &mut xdata;
                flags |= F_DUPDATA as libc::c_uint;
                do_sub = 1 as libc::c_int;
                if insert_key == 0 {
                    mdb_node_del(mc, 0 as libc::c_int);
                }
                current_block = 866732880969548764;
            }
            _ => {}
        }
        match current_block {
            866732880969548764 => {
                nflags = flags & NODE_ADD_FLAGS as libc::c_uint;
                nsize = if (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void
                    as *mut MDB_page2))
                    .mp2_flags as libc::c_int
                    & 0x20 as libc::c_int
                    == 0x20 as libc::c_int
                {
                    (*key).mv_size
                } else {
                    mdb_leaf_size(env, key, rdata)
                };
                if (((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                    .mp2_upper as libc::c_int
                    - (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                        .mp2_lower as libc::c_int) as indx_t as size_t)
                    < nsize
                {
                    if flags & (F_DUPDATA | F_SUBDATA) as libc::c_uint == F_DUPDATA as libc::c_uint
                    {
                        nflags &= !MDB_APPEND as libc::c_uint;
                    }
                    if insert_key == 0 {
                        nflags |= MDB_SPLIT_REPLACE as libc::c_uint;
                    }
                    rc = mdb_page_split(mc, key, rdata, P_INVALID, nflags);
                } else {
                    rc = mdb_node_add(
                        mc,
                        (*mc).mc_ki[(*mc).mc_top as usize],
                        key,
                        rdata,
                        0 as libc::c_int as pgno_t,
                        nflags,
                    );
                    if rc == 0 as libc::c_int {
                        let mut m2 = 0 as *mut MDB_cursor;
                        let mut m3 = 0 as *mut MDB_cursor;
                        let mut dbi = (*mc).mc_dbi;
                        let mut i_0 = (*mc).mc_top as libc::c_uint;
                        let mut mp_0 = (*mc).mc_pg[i_0 as usize];
                        m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
                        while !m2.is_null() {
                            if (*mc).mc_flags & C_SUB as libc::c_uint != 0 {
                                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                            } else {
                                m3 = m2;
                            }
                            if !(m3 == mc
                                || ((*m3).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int
                                || (*m3).mc_pg[i_0 as usize] != mp_0)
                            {
                                if (*m3).mc_ki[i_0 as usize] as libc::c_int
                                    >= (*mc).mc_ki[i_0 as usize] as libc::c_int
                                    && insert_key != 0
                                {
                                    (*m3).mc_ki[i_0 as usize] =
                                        ((*m3).mc_ki[i_0 as usize]).wrapping_add(1);
                                    (*m3).mc_ki[i_0 as usize];
                                }
                                let mut xr_pg = mp_0;
                                let mut xr_node = 0 as *mut MDB_node;
                                if !(!(!((*m3).mc_xcursor).is_null()
                                    && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                        & C_INITIALIZED as libc::c_uint
                                        != 0)
                                    || (*m3).mc_ki[i_0 as usize] as libc::c_uint
                                        >= ((*(xr_pg as *mut libc::c_void as *mut MDB_page2))
                                            .mp2_lower
                                            as libc::c_uint)
                                            .wrapping_sub(
                                                (PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                                    if 0 as libc::c_int != 0 {
                                                        PAGEHDRSZ as libc::c_uint
                                                    } else {
                                                        0 as libc::c_int as libc::c_uint
                                                    },
                                                ),
                                            )
                                            >> 1 as libc::c_int)
                                {
                                    xr_node = (xr_pg as *mut libc::c_char)
                                        .offset(
                                            *((*(xr_pg as *mut libc::c_void as *mut MDB_page2))
                                                .mp2_ptrs)
                                                .as_mut_ptr()
                                                .offset((*m3).mc_ki[i_0 as usize] as isize)
                                                as libc::c_int
                                                as isize,
                                        )
                                        .offset(
                                            (if 0 as libc::c_int != 0 {
                                                PAGEHDRSZ as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            }) as isize,
                                        )
                                        as *mut MDB_node;
                                    if (*xr_node).mn_flags as libc::c_int & (F_DUPDATA | F_SUBDATA)
                                        == F_DUPDATA
                                    {
                                        (*(*m3).mc_xcursor).mx_cursor.mc_pg
                                            [0 as libc::c_int as usize] = ((*xr_node).mn_data)
                                            .as_mut_ptr()
                                            .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                            as *mut libc::c_void
                                            as *mut MDB_page;
                                    }
                                }
                            }
                            m2 = (*m2).mc_next;
                        }
                    }
                }
                if !(rc == MDB_SUCCESS) {
                    current_block = 12092145755723610434;
                    break;
                }
                if do_sub != 0 {
                    xflags = 0;
                    new_dupdata = 0;
                    ecount = 0;
                    current_block = 5681788516532307688;
                } else {
                    current_block = 7301633501560609215;
                }
            }
            _ => {}
        }
        match current_block {
            5681788516532307688 => {
                xdata.mv_size = 0 as libc::c_int as size_t;
                xdata.mv_data = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_void;
                leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void
                            as *mut MDB_page2))
                            .mp2_ptrs)
                            .as_mut_ptr()
                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                if flags & (MDB_CURRENT | MDB_APPENDDUP) as libc::c_uint
                    == MDB_CURRENT as libc::c_uint
                {
                    xflags = MDB_CURRENT | MDB_NOSPILL;
                } else {
                    mdb_xcursor_init1(mc, leaf);
                    xflags = if flags & MDB_NODUPDATA as libc::c_uint != 0 {
                        MDB_NOOVERWRITE | MDB_NOSPILL
                    } else {
                        MDB_NOSPILL
                    };
                }
                if !sub_root.is_null() {
                    (*(*mc).mc_xcursor).mx_cursor.mc_pg[0 as libc::c_int as usize] = sub_root;
                }
                new_dupdata = dkey.mv_size as libc::c_int;
                if dkey.mv_size != 0 {
                    rc = _mdb_cursor_put(
                        &mut (*(*mc).mc_xcursor).mx_cursor,
                        &mut dkey,
                        &mut xdata,
                        xflags as libc::c_uint,
                    );
                    if rc != 0 {
                        current_block = 18262792344852136737;
                        break;
                    }
                    dkey.mv_size = 0 as libc::c_int as size_t;
                }
                if (*leaf).mn_flags as libc::c_int & F_SUBDATA == 0 || !sub_root.is_null() {
                    let mut m2_0 = 0 as *mut MDB_cursor;
                    let mut mx = (*mc).mc_xcursor;
                    let mut i_1 = (*mc).mc_top as libc::c_uint;
                    let mut mp_1 = (*mc).mc_pg[i_1 as usize];
                    m2_0 = *((*(*mc).mc_txn).mt_cursors).offset((*mc).mc_dbi as isize);
                    while !m2_0.is_null() {
                        if !(m2_0 == mc
                            || ((*m2_0).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int)
                        {
                            if !((*m2_0).mc_flags & C_INITIALIZED as libc::c_uint == 0) {
                                if (*m2_0).mc_pg[i_1 as usize] == mp_1 {
                                    if (*m2_0).mc_ki[i_1 as usize] as libc::c_int
                                        == (*mc).mc_ki[i_1 as usize] as libc::c_int
                                    {
                                        mdb_xcursor_init2(m2_0, mx, new_dupdata);
                                    } else if insert_key == 0 {
                                        let mut xr_pg_0 = mp_1;
                                        let mut xr_node_0 = 0 as *mut MDB_node;
                                        if !(!(!((*m2_0).mc_xcursor).is_null()
                                            && (*(*m2_0).mc_xcursor).mx_cursor.mc_flags
                                                & C_INITIALIZED as libc::c_uint
                                                != 0)
                                            || (*m2_0).mc_ki[i_1 as usize] as libc::c_uint
                                                >= ((*(xr_pg_0 as *mut libc::c_void
                                                    as *mut MDB_page2))
                                                    .mp2_lower
                                                    as libc::c_uint)
                                                    .wrapping_sub(
                                                        (PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                                            if 0 as libc::c_int != 0 {
                                                                PAGEHDRSZ as libc::c_uint
                                                            } else {
                                                                0 as libc::c_int as libc::c_uint
                                                            },
                                                        ),
                                                    )
                                                    >> 1 as libc::c_int)
                                        {
                                            xr_node_0 = (xr_pg_0 as *mut libc::c_char)
                                                .offset(
                                                    *((*(xr_pg_0 as *mut libc::c_void
                                                        as *mut MDB_page2))
                                                        .mp2_ptrs)
                                                        .as_mut_ptr()
                                                        .offset(
                                                            (*m2_0).mc_ki[i_1 as usize] as isize,
                                                        )
                                                        as libc::c_int
                                                        as isize,
                                                )
                                                .offset(
                                                    (if 0 as libc::c_int != 0 {
                                                        PAGEHDRSZ as libc::c_uint
                                                    } else {
                                                        0 as libc::c_int as libc::c_uint
                                                    })
                                                        as isize,
                                                )
                                                as *mut MDB_node;
                                            if (*xr_node_0).mn_flags as libc::c_int
                                                & (F_DUPDATA | F_SUBDATA)
                                                == F_DUPDATA
                                            {
                                                (*(*m2_0).mc_xcursor).mx_cursor.mc_pg
                                                    [0 as libc::c_int as usize] =
                                                    ((*xr_node_0).mn_data).as_mut_ptr().offset(
                                                        (*xr_node_0).mn_ksize as libc::c_int
                                                            as isize,
                                                    )
                                                        as *mut libc::c_void
                                                        as *mut MDB_page;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        m2_0 = (*m2_0).mc_next;
                    }
                }
                ecount = (*(*mc).mc_xcursor).mx_db.md_entries;
                if flags & MDB_APPENDDUP as libc::c_uint != 0 {
                    xflags |= MDB_APPEND;
                }
                rc = _mdb_cursor_put(
                    &mut (*(*mc).mc_xcursor).mx_cursor,
                    data,
                    &mut xdata,
                    xflags as libc::c_uint,
                );
                if flags & F_SUBDATA as libc::c_uint != 0 {
                    let mut db = ((*leaf).mn_data)
                        .as_mut_ptr()
                        .offset((*leaf).mn_ksize as libc::c_int as isize)
                        as *mut libc::c_void;
                    memcpy(
                        db,
                        &mut (*(*mc).mc_xcursor).mx_db as *mut MDB_db as *const libc::c_void,
                        ::core::mem::size_of::<MDB_db>() as libc::c_ulong,
                    );
                }
                insert_data =
                    ((*(*mc).mc_xcursor).mx_db.md_entries).wrapping_sub(ecount) as libc::c_int;
            }
            _ => {}
        }
        if insert_data != 0 {
            (*(*mc).mc_db).md_entries = ((*(*mc).mc_db).md_entries).wrapping_add(1);
            (*(*mc).mc_db).md_entries;
        }
        if insert_key != 0 {
            if rc != 0 {
                current_block = 18262792344852136737;
                break;
            }
            (*mc).mc_flags |= C_INITIALIZED as libc::c_uint;
        }
        if !(flags & MDB_MULTIPLE as libc::c_uint != 0) {
            current_block = 5726862278832168375;
            break;
        }
        if !(rc == 0) {
            current_block = 5726862278832168375;
            break;
        }
        mcount = mcount.wrapping_add(1);
        mcount;
        (*data.offset(1 as libc::c_int as isize)).mv_size = mcount as size_t;
        if !(mcount < dcount) {
            current_block = 5726862278832168375;
            break;
        }
        let ref mut fresh41 = (*data.offset(0 as libc::c_int as isize)).mv_data;
        *fresh41 = ((*data.offset(0 as libc::c_int as isize)).mv_data as *mut libc::c_char)
            .offset((*data.offset(0 as libc::c_int as isize)).mv_size as isize)
            as *mut libc::c_void;
        insert_data = 0 as libc::c_int;
        insert_key = insert_data;
        current_block = 6503087309128928598;
    }
    match current_block {
        5726862278832168375 => return rc,
        18262792344852136737 => {
            if rc == MDB_KEYEXIST {
                rc = MDB_PROBLEM;
            }
        }
        10257223768985283691 => return MDB_SUCCESS,
        _ => {}
    }
    (*(*mc).mc_txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_cursor_put(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut rc = _mdb_cursor_put(mc, key, data, flags);
    return rc;
}
unsafe extern "C" fn _mdb_cursor_del(
    mut mc: *mut MDB_cursor,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut current_block: u64;
    let mut leaf = 0 as *mut MDB_node;
    let mut mp = 0 as *mut MDB_page;
    let mut rc: libc::c_int = 0;
    if (*(*mc).mc_txn).mt_flags & (MDB_TXN_RDONLY | MDB_TXN_BLOCKED) as libc::c_uint != 0 {
        return if (*(*mc).mc_txn).mt_flags & MDB_TXN_RDONLY as libc::c_uint != 0 {
            EACCES
        } else {
            MDB_BAD_TXN
        };
    }
    if (*mc).mc_flags & C_INITIALIZED as libc::c_uint == 0 {
        return EINVAL;
    }
    if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint
        >= ((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_lower
            as libc::c_uint)
            .wrapping_sub(
                (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }),
            )
            >> 1 as libc::c_int
    {
        return MDB_NOTFOUND;
    }
    if flags & MDB_NOSPILL as libc::c_uint == 0 && {
        rc = mdb_page_spill(mc, NULL as *mut MDB_val, NULL as *mut MDB_val);
        rc != 0
    } {
        return rc;
    }
    rc = mdb_cursor_touch(mc);
    if rc != 0 {
        return rc;
    }
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    if !((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
        & 0x2 as libc::c_int
        == 0x2 as libc::c_int)
    {
        return MDB_CORRUPTED;
    }
    if !((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
        & 0x20 as libc::c_int
        == 0x20 as libc::c_int)
    {
        leaf = (mp as *mut libc::c_char)
            .offset(
                *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                    as libc::c_int as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
            if flags & MDB_NODUPDATA as libc::c_uint != 0 {
                (*(*mc).mc_db).md_entries = ((*(*mc).mc_db).md_entries).wrapping_sub(
                    ((*(*mc).mc_xcursor).mx_db.md_entries)
                        .wrapping_sub(1 as libc::c_int as mdb_size_t),
                );
                (*(*mc).mc_xcursor).mx_cursor.mc_flags &= !C_INITIALIZED as libc::c_uint;
            } else {
                if !((*leaf).mn_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int) {
                    (*(*mc).mc_xcursor).mx_cursor.mc_pg[0 as libc::c_int as usize] =
                        ((*leaf).mn_data)
                            .as_mut_ptr()
                            .offset((*leaf).mn_ksize as libc::c_int as isize)
                            as *mut libc::c_void as *mut MDB_page;
                }
                rc = _mdb_cursor_del(
                    &mut (*(*mc).mc_xcursor).mx_cursor,
                    MDB_NOSPILL as libc::c_uint,
                );
                if rc != 0 {
                    return rc;
                }
                if (*(*mc).mc_xcursor).mx_db.md_entries != 0 {
                    if (*leaf).mn_flags as libc::c_int & F_SUBDATA != 0 {
                        let mut db = ((*leaf).mn_data)
                            .as_mut_ptr()
                            .offset((*leaf).mn_ksize as libc::c_int as isize)
                            as *mut libc::c_void;
                        memcpy(
                            db,
                            &mut (*(*mc).mc_xcursor).mx_db as *mut MDB_db as *const libc::c_void,
                            ::core::mem::size_of::<MDB_db>() as libc::c_ulong,
                        );
                    } else {
                        let mut m2 = 0 as *mut MDB_cursor;
                        mdb_node_shrink(mp, (*mc).mc_ki[(*mc).mc_top as usize]);
                        leaf = (mp as *mut libc::c_char)
                            .offset(
                                *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                    .as_mut_ptr()
                                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                    as libc::c_int as isize,
                            )
                            .offset(
                                (if 0 as libc::c_int != 0 {
                                    PAGEHDRSZ as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        (*(*mc).mc_xcursor).mx_cursor.mc_pg[0 as libc::c_int as usize] =
                            ((*leaf).mn_data)
                                .as_mut_ptr()
                                .offset((*leaf).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void as *mut MDB_page;
                        m2 = *((*(*mc).mc_txn).mt_cursors).offset((*mc).mc_dbi as isize);
                        while !m2.is_null() {
                            if !(m2 == mc
                                || ((*m2).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int)
                            {
                                if !((*m2).mc_flags & C_INITIALIZED as libc::c_uint == 0) {
                                    if (*m2).mc_pg[(*mc).mc_top as usize] == mp {
                                        let mut xr_pg = mp;
                                        let mut xr_node = 0 as *mut MDB_node;
                                        if !(!(!((*m2).mc_xcursor).is_null()
                                            && (*(*m2).mc_xcursor).mx_cursor.mc_flags
                                                & C_INITIALIZED as libc::c_uint
                                                != 0)
                                            || (*m2).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                                >= ((*(xr_pg as *mut libc::c_void
                                                    as *mut MDB_page2))
                                                    .mp2_lower
                                                    as libc::c_uint)
                                                    .wrapping_sub(
                                                        (PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                                            if 0 as libc::c_int != 0 {
                                                                PAGEHDRSZ as libc::c_uint
                                                            } else {
                                                                0 as libc::c_int as libc::c_uint
                                                            },
                                                        ),
                                                    )
                                                    >> 1 as libc::c_int)
                                        {
                                            xr_node = (xr_pg as *mut libc::c_char)
                                                .offset(
                                                    *((*(xr_pg as *mut libc::c_void
                                                        as *mut MDB_page2))
                                                        .mp2_ptrs)
                                                        .as_mut_ptr()
                                                        .offset(
                                                            (*m2).mc_ki[(*mc).mc_top as usize]
                                                                as isize,
                                                        )
                                                        as libc::c_int
                                                        as isize,
                                                )
                                                .offset(
                                                    (if 0 as libc::c_int != 0 {
                                                        PAGEHDRSZ as libc::c_uint
                                                    } else {
                                                        0 as libc::c_int as libc::c_uint
                                                    })
                                                        as isize,
                                                )
                                                as *mut MDB_node;
                                            if (*xr_node).mn_flags as libc::c_int
                                                & (F_DUPDATA | F_SUBDATA)
                                                == F_DUPDATA
                                            {
                                                (*(*m2).mc_xcursor).mx_cursor.mc_pg
                                                    [0 as libc::c_int as usize] =
                                                    ((*xr_node).mn_data).as_mut_ptr().offset(
                                                        (*xr_node).mn_ksize as libc::c_int as isize,
                                                    )
                                                        as *mut libc::c_void
                                                        as *mut MDB_page;
                                            }
                                        }
                                    }
                                }
                            }
                            m2 = (*m2).mc_next;
                        }
                    }
                    (*(*mc).mc_db).md_entries = ((*(*mc).mc_db).md_entries).wrapping_sub(1);
                    (*(*mc).mc_db).md_entries;
                    return rc;
                } else {
                    (*(*mc).mc_xcursor).mx_cursor.mc_flags &= !C_INITIALIZED as libc::c_uint;
                }
            }
            if (*leaf).mn_flags as libc::c_int & F_SUBDATA != 0 {
                rc = mdb_drop0(&mut (*(*mc).mc_xcursor).mx_cursor, 0 as libc::c_int);
                if rc != 0 {
                    current_block = 10917811640877813153;
                } else {
                    current_block = 3689906465960840878;
                }
            } else {
                current_block = 3689906465960840878;
            }
        } else if ((*leaf).mn_flags as libc::c_uint ^ flags) & F_SUBDATA as libc::c_uint != 0 {
            rc = MDB_INCOMPATIBLE;
            current_block = 10917811640877813153;
        } else {
            current_block = 3689906465960840878;
        }
        match current_block {
            3689906465960840878 => {
                if (*leaf).mn_flags as libc::c_int & 0x1 as libc::c_int == 0x1 as libc::c_int {
                    let mut omp = 0 as *mut MDB_page;
                    let mut pg: pgno_t = 0;
                    memcpy(
                        &mut pg as *mut pgno_t as *mut libc::c_void,
                        ((*leaf).mn_data)
                            .as_mut_ptr()
                            .offset((*leaf).mn_ksize as libc::c_int as isize)
                            as *mut libc::c_void,
                        ::core::mem::size_of::<pgno_t>() as libc::c_ulong,
                    );
                    rc = mdb_page_get(mc, pg, &mut omp, NULL as *mut libc::c_int);
                    if rc != 0 || {
                        rc = mdb_ovpage_free(mc, omp);
                        rc != 0
                    } {
                        current_block = 10917811640877813153;
                    } else {
                        current_block = 2064878754691380821;
                    }
                } else {
                    current_block = 2064878754691380821;
                }
            }
            _ => {}
        }
        match current_block {
            2064878754691380821 => {}
            _ => {
                (*(*mc).mc_txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
                return rc;
            }
        }
    }
    return mdb_cursor_del0(mc);
}
#[no_mangle]
pub unsafe extern "C" fn mdb_cursor_del(
    mut mc: *mut MDB_cursor,
    mut flags: libc::c_uint,
) -> libc::c_int {
    return _mdb_cursor_del(mc, flags);
}
unsafe extern "C" fn mdb_page_new(
    mut mc: *mut MDB_cursor,
    mut flags: uint32_t,
    mut num: libc::c_int,
    mut mp: *mut *mut MDB_page,
) -> libc::c_int {
    let mut np = 0 as *mut MDB_page;
    let mut rc: libc::c_int = 0;
    rc = mdb_page_alloc(mc, num, &mut np);
    if rc != 0 {
        return rc;
    }
    (*np).mp_flags = (flags | P_DIRTY as uint32_t) as uint16_t;
    (*np).mp_pb.pb.pb_lower = (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
        PAGEHDRSZ as libc::c_uint
    } else {
        0 as libc::c_int as libc::c_uint
    }) as indx_t;
    (*np).mp_pb.pb.pb_upper =
        ((*(*(*mc).mc_txn).mt_env).me_psize).wrapping_sub(if 0 as libc::c_int != 0 {
            PAGEHDRSZ as libc::c_uint
        } else {
            0 as libc::c_int as libc::c_uint
        }) as indx_t;
    if (*(np as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int & 0x1 as libc::c_int
        == 0x1 as libc::c_int
    {
        (*(*mc).mc_db).md_branch_pages = ((*(*mc).mc_db).md_branch_pages).wrapping_add(1);
        (*(*mc).mc_db).md_branch_pages;
    } else if (*(np as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
        & 0x2 as libc::c_int
        == 0x2 as libc::c_int
    {
        (*(*mc).mc_db).md_leaf_pages = ((*(*mc).mc_db).md_leaf_pages).wrapping_add(1);
        (*(*mc).mc_db).md_leaf_pages;
    } else if (*(np as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
        & 0x4 as libc::c_int
        == 0x4 as libc::c_int
    {
        (*(*mc).mc_db).md_overflow_pages =
            ((*(*mc).mc_db).md_overflow_pages).wrapping_add(num as pgno_t);
        (*np).mp_pb.pb_pages = num as uint32_t;
    }
    *mp = np;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_leaf_size(
    mut env: *mut MDB_env,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> size_t {
    let mut sz: size_t = 0;
    sz = NODESIZE
        .wrapping_add((*key).mv_size)
        .wrapping_add((*data).mv_size);
    if sz > (*env).me_nodemax as size_t {
        sz = (sz as libc::c_ulong).wrapping_sub(
            ((*data).mv_size).wrapping_sub(::core::mem::size_of::<pgno_t>() as libc::c_ulong),
        ) as size_t as size_t;
    }
    return sz
        .wrapping_add(::core::mem::size_of::<indx_t>() as libc::c_ulong)
        .wrapping_add(1 as libc::c_uint as libc::c_ulong)
        & -(2 as libc::c_int) as libc::c_ulong;
}
unsafe extern "C" fn mdb_branch_size(mut env: *mut MDB_env, mut key: *mut MDB_val) -> size_t {
    let mut sz: size_t = 0;
    sz = NODESIZE.wrapping_add(if key.is_null() {
        0 as libc::c_int as size_t
    } else {
        (*key).mv_size
    });
    let _ = sz > (*env).me_nodemax as size_t;
    return sz.wrapping_add(::core::mem::size_of::<indx_t>() as libc::c_ulong);
}
unsafe extern "C" fn mdb_node_add(
    mut mc: *mut MDB_cursor,
    mut indx: indx_t,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut pgno: pgno_t,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_uint = 0;
    let mut node_size = NODESIZE;
    let mut room: ssize_t = 0;
    let mut ofs: indx_t = 0;
    let mut node = 0 as *mut MDB_node;
    let mut mp = (*mc).mc_pg[(*mc).mc_top as usize];
    let mut ofp = NULL as *mut MDB_page;
    let mut ndata = 0 as *mut libc::c_void;
    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_upper as libc::c_int
        >= (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_int
    {
    } else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"MP_UPPER(mp) >= MP_LOWER(mp)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"mdb_node_add\0")).as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            8402 as libc::c_int,
        );
    };
    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int & 0x20 as libc::c_int
        == 0x20 as libc::c_int
    {
        let mut ksize = (*(*mc).mc_db).md_pad as libc::c_int;
        let mut dif: libc::c_int = 0;
        let mut ptr = (mp as *mut libc::c_char)
            .offset(PAGEHDRSZ as libc::c_uint as isize)
            .offset((indx as libc::c_int * ksize) as isize);
        dif = (((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
            .wrapping_sub(
                (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }),
            )
            >> 1 as libc::c_int)
            .wrapping_sub(indx as libc::c_uint) as libc::c_int;
        if dif > 0 as libc::c_int {
            memmove(
                ptr.offset(ksize as isize) as *mut libc::c_void,
                ptr as *const libc::c_void,
                (dif * ksize) as libc::c_ulong,
            );
        }
        memcpy(
            ptr as *mut libc::c_void,
            (*key).mv_data,
            ksize as libc::c_ulong,
        );
        let ref mut fresh42 = (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower;
        *fresh42 = (*fresh42 as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<indx_t>() as libc::c_ulong)
            as indx_t as indx_t;
        let ref mut fresh43 = (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_upper;
        *fresh43 = (*fresh43 as libc::c_ulong).wrapping_sub(
            (ksize as libc::c_ulong)
                .wrapping_sub(::core::mem::size_of::<indx_t>() as libc::c_ulong),
        ) as indx_t as indx_t;
        return MDB_SUCCESS;
    }
    room = ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_upper as libc::c_int
        - (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_int)
        as indx_t as ssize_t
        - ::core::mem::size_of::<indx_t>() as libc::c_ulong as ssize_t;
    if !key.is_null() {
        node_size = node_size.wrapping_add((*key).mv_size);
    }
    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int & 0x2 as libc::c_int
        == 0x2 as libc::c_int
    {
        if !key.is_null() && !data.is_null() {
        } else {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"key && data\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"mdb_node_add\0"))
                    .as_ptr(),
                b"mdb.c\0" as *const u8 as *const libc::c_char,
                8430 as libc::c_int,
            );
        };
        if flags & 0x1 as libc::c_int as libc::c_uint == 0x1 as libc::c_int as libc::c_uint {
            node_size = (node_size as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<pgno_t>() as libc::c_ulong)
                as size_t as size_t;
            current_block = 11057878835866523405;
        } else if node_size.wrapping_add((*data).mv_size)
            > (*(*(*mc).mc_txn).mt_env).me_nodemax as size_t
        {
            let mut ovpages =
                (((PAGEHDRSZ as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as size_t)
                    .wrapping_add((*data).mv_size)
                    / (*(*(*mc).mc_txn).mt_env).me_psize as size_t)
                    .wrapping_add(1 as libc::c_int as size_t) as libc::c_int;
            let mut rc: libc::c_int = 0;
            node_size = node_size
                .wrapping_add(::core::mem::size_of::<pgno_t>() as libc::c_ulong)
                .wrapping_add(1 as libc::c_uint as libc::c_ulong)
                & -(2 as libc::c_int) as libc::c_ulong;
            if node_size as ssize_t > room {
                current_block = 15594603006322722090;
            } else {
                rc = mdb_page_new(mc, P_OVERFLOW as uint32_t, ovpages, &mut ofp);
                if rc != 0 {
                    return rc;
                }
                flags |= F_BIGDATA as libc::c_uint;
                current_block = 278520732244569559;
            }
        } else {
            node_size = node_size.wrapping_add((*data).mv_size);
            current_block = 11057878835866523405;
        }
    } else {
        current_block = 11057878835866523405;
    }
    match current_block {
        11057878835866523405 => {
            node_size =
                node_size.wrapping_add(1 as libc::c_uint as size_t) & -(2 as libc::c_int) as size_t;
            if node_size as ssize_t > room {
                current_block = 15594603006322722090;
            } else {
                current_block = 278520732244569559;
            }
        }
        _ => {}
    }
    match current_block {
        15594603006322722090 => {
            (*(*mc).mc_txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
            return MDB_PAGE_FULL;
        }
        _ => {
            i = ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int;
            while i > indx as libc::c_uint {
                *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(i as isize) = *((*(mp as *mut libc::c_void as *mut MDB_page2))
                    .mp2_ptrs)
                    .as_mut_ptr()
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
                i = i.wrapping_sub(1);
                i;
            }
            ofs = ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_upper as size_t)
                .wrapping_sub(node_size) as indx_t;
            if ofs as libc::c_ulong
                >= ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<indx_t>() as libc::c_ulong)
            {
            } else {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"ofs >= MP_LOWER(mp) + sizeof(indx_t)\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"mdb_node_add\0"))
                        .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    8463 as libc::c_int,
                );
            };
            *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                .as_mut_ptr()
                .offset(indx as isize) = ofs;
            (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_upper = ofs;
            let ref mut fresh44 = (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower;
            *fresh44 = (*fresh44 as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<indx_t>() as libc::c_ulong)
                as indx_t as indx_t;
            node = (mp as *mut libc::c_char)
                .offset(
                    *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                        .as_mut_ptr()
                        .offset(indx as isize) as libc::c_int as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            (*node).mn_ksize = (if key.is_null() {
                0 as libc::c_int as size_t
            } else {
                (*key).mv_size
            }) as libc::c_ushort;
            (*node).mn_flags = flags as libc::c_ushort;
            if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                & 0x2 as libc::c_int
                == 0x2 as libc::c_int
            {
                (*node).mn_lo =
                    ((*data).mv_size & 0xffff as libc::c_int as size_t) as libc::c_ushort;
                (*node).mn_hi = ((*data).mv_size >> 16 as libc::c_int) as libc::c_ushort;
            } else {
                (*node).mn_lo = (pgno & 0xffff as libc::c_int as pgno_t) as libc::c_ushort;
                (*node).mn_hi = (pgno >> 16 as libc::c_int) as libc::c_ushort;
                if if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                    32 as libc::c_int
                } else {
                    0 as libc::c_int
                } != 0
                {
                    (*node).mn_flags = (pgno
                        >> (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })) as libc::c_ushort;
                }
            }
            if !key.is_null() {
                memcpy(
                    ((*node).mn_data).as_mut_ptr() as *mut libc::c_void,
                    (*key).mv_data,
                    (*key).mv_size,
                );
            }
            if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                & 0x2 as libc::c_int
                == 0x2 as libc::c_int
            {
                ndata = ((*node).mn_data)
                    .as_mut_ptr()
                    .offset((*node).mn_ksize as libc::c_int as isize)
                    as *mut libc::c_void;
                if ofp.is_null() {
                    if flags & 0x1 as libc::c_int as libc::c_uint
                        == 0x1 as libc::c_int as libc::c_uint
                    {
                        memcpy(
                            ndata,
                            (*data).mv_data,
                            ::core::mem::size_of::<pgno_t>() as libc::c_ulong,
                        );
                    } else if flags & 0x10000 as libc::c_int as libc::c_uint
                        == 0x10000 as libc::c_int as libc::c_uint
                    {
                        (*data).mv_data = ndata;
                    } else {
                        memcpy(ndata, (*data).mv_data, (*data).mv_size);
                    }
                } else {
                    memcpy(
                        ndata,
                        &mut (*ofp).mp_p.p_pgno as *mut pgno_t as *const libc::c_void,
                        ::core::mem::size_of::<pgno_t>() as libc::c_ulong,
                    );
                    ndata = (ofp as *mut libc::c_char).offset(PAGEHDRSZ as libc::c_uint as isize)
                        as *mut libc::c_void;
                    if flags & 0x10000 as libc::c_int as libc::c_uint
                        == 0x10000 as libc::c_int as libc::c_uint
                    {
                        (*data).mv_data = ndata;
                    } else {
                        memcpy(ndata, (*data).mv_data, (*data).mv_size);
                    }
                }
            }
            return MDB_SUCCESS;
        }
    };
}
unsafe extern "C" fn mdb_node_del(mut mc: *mut MDB_cursor, mut ksize: libc::c_int) {
    let mut mp = (*mc).mc_pg[(*mc).mc_top as usize];
    let mut indx = (*mc).mc_ki[(*mc).mc_top as usize];
    let mut sz: libc::c_uint = 0;
    let mut i: indx_t = 0;
    let mut j: indx_t = 0;
    let mut numkeys: indx_t = 0;
    let mut ptr: indx_t = 0;
    let mut node = 0 as *mut MDB_node;
    let mut base = 0 as *mut libc::c_char;
    numkeys =
        (((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint).wrapping_sub(
            (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }),
        ) >> 1 as libc::c_int) as indx_t;
    if (indx as libc::c_int) < numkeys as libc::c_int {
    } else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"indx < numkeys\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"mdb_node_del\0")).as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            8528 as libc::c_int,
        );
    };
    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int & 0x20 as libc::c_int
        == 0x20 as libc::c_int
    {
        let mut x = numkeys as libc::c_int - 1 as libc::c_int - indx as libc::c_int;
        base = (mp as *mut libc::c_char)
            .offset(PAGEHDRSZ as libc::c_uint as isize)
            .offset((indx as libc::c_int * ksize) as isize);
        if x != 0 {
            memmove(
                base as *mut libc::c_void,
                base.offset(ksize as isize) as *const libc::c_void,
                (x * ksize) as libc::c_ulong,
            );
        }
        let ref mut fresh45 = (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower;
        *fresh45 = (*fresh45 as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<indx_t>() as libc::c_ulong)
            as indx_t as indx_t;
        let ref mut fresh46 = (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_upper;
        *fresh46 = (*fresh46 as libc::c_ulong).wrapping_add(
            (ksize as libc::c_ulong)
                .wrapping_sub(::core::mem::size_of::<indx_t>() as libc::c_ulong),
        ) as indx_t as indx_t;
        return;
    }
    node = (mp as *mut libc::c_char)
        .offset(
            *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                .as_mut_ptr()
                .offset(indx as isize) as libc::c_int as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    sz = NODESIZE.wrapping_add((*node).mn_ksize as libc::c_ulong) as libc::c_uint;
    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int & 0x2 as libc::c_int
        == 0x2 as libc::c_int
    {
        if (*node).mn_flags as libc::c_int & 0x1 as libc::c_int == 0x1 as libc::c_int {
            sz = (sz as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<pgno_t>() as libc::c_ulong)
                as libc::c_uint as libc::c_uint;
        } else {
            sz = sz.wrapping_add(
                (*node).mn_lo as libc::c_uint
                    | ((*node).mn_hi as libc::c_uint) << 16 as libc::c_int,
            );
        }
    }
    sz = sz.wrapping_add(1 as libc::c_uint) & -(2 as libc::c_int) as libc::c_uint;
    ptr = *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
        .as_mut_ptr()
        .offset(indx as isize);
    j = 0 as libc::c_int as indx_t;
    i = j;
    while (i as libc::c_int) < numkeys as libc::c_int {
        if i as libc::c_int != indx as libc::c_int {
            *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                .as_mut_ptr()
                .offset(j as isize) = *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                .as_mut_ptr()
                .offset(i as isize);
            if (*((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                .as_mut_ptr()
                .offset(i as isize) as libc::c_int)
                < ptr as libc::c_int
            {
                let ref mut fresh47 = *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(j as isize);
                *fresh47 = (*fresh47 as libc::c_uint).wrapping_add(sz) as indx_t as indx_t;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    base = (mp as *mut libc::c_char)
        .offset((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_upper as libc::c_int as isize)
        .offset(
            (if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        );
    memmove(
        base.offset(sz as isize) as *mut libc::c_void,
        base as *const libc::c_void,
        (ptr as libc::c_int
            - (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_upper as libc::c_int)
            as libc::c_ulong,
    );
    let ref mut fresh48 = (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower;
    *fresh48 = (*fresh48 as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<indx_t>() as libc::c_ulong) as indx_t
        as indx_t;
    let ref mut fresh49 = (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_upper;
    *fresh49 = (*fresh49 as libc::c_uint).wrapping_add(sz) as indx_t as indx_t;
}
unsafe extern "C" fn mdb_node_shrink(mut mp: *mut MDB_page, mut indx: indx_t) {
    let mut node = 0 as *mut MDB_node;
    let mut sp = 0 as *mut MDB_page;
    let mut xp = 0 as *mut MDB_page;
    let mut base = 0 as *mut libc::c_char;
    let mut delta: indx_t = 0;
    let mut nsize: indx_t = 0;
    let mut len: indx_t = 0;
    let mut ptr: indx_t = 0;
    let mut i: libc::c_int = 0;
    node = (mp as *mut libc::c_char)
        .offset(
            *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                .as_mut_ptr()
                .offset(indx as isize) as libc::c_int as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    sp = ((*node).mn_data)
        .as_mut_ptr()
        .offset((*node).mn_ksize as libc::c_int as isize) as *mut libc::c_void
        as *mut MDB_page;
    delta = ((*(sp as *mut libc::c_void as *mut MDB_page2)).mp2_upper as libc::c_int
        - (*(sp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_int)
        as indx_t;
    nsize = ((*node).mn_lo as libc::c_uint | ((*node).mn_hi as libc::c_uint) << 16 as libc::c_int)
        .wrapping_sub(delta as libc::c_uint) as indx_t;
    if (*(sp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int & 0x20 as libc::c_int
        == 0x20 as libc::c_int
    {
        len = nsize;
        if nsize as libc::c_int & 1 as libc::c_int != 0 {
            return;
        }
    } else {
        xp = (sp as *mut libc::c_char).offset(delta as libc::c_int as isize) as *mut MDB_page;
        i = (((*(sp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
            .wrapping_sub(
                (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }),
            )
            >> 1 as libc::c_int) as libc::c_int;
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            *((*(xp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                .as_mut_ptr()
                .offset(i as isize) = (*((*(sp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                .as_mut_ptr()
                .offset(i as isize) as libc::c_int
                - delta as libc::c_int) as indx_t;
        }
        len = PAGEHDRSZ as indx_t;
    }
    (*(sp as *mut libc::c_void as *mut MDB_page2)).mp2_upper =
        (*(sp as *mut libc::c_void as *mut MDB_page2)).mp2_lower;
    let mut s = 0 as *mut libc::c_ushort;
    let mut d = 0 as *mut libc::c_ushort;
    s = &mut (*mp).mp_p.p_pgno as *mut pgno_t as *mut libc::c_ushort;
    d = &mut (*(sp as *mut libc::c_void as *mut MDB_page2)).mp2_p as *mut [uint16_t; 4]
        as *mut libc::c_ushort;
    let fresh50 = s;
    s = s.offset(1);
    let fresh51 = d;
    d = d.offset(1);
    *fresh51 = *fresh50;
    let fresh52 = s;
    s = s.offset(1);
    let fresh53 = d;
    d = d.offset(1);
    *fresh53 = *fresh52;
    let fresh54 = s;
    s = s.offset(1);
    let fresh55 = d;
    d = d.offset(1);
    *fresh55 = *fresh54;
    *d = *s;
    (*node).mn_lo = (nsize as libc::c_int & 0xffff as libc::c_int) as libc::c_ushort;
    (*node).mn_hi = (nsize as libc::c_int >> 16 as libc::c_int) as libc::c_ushort;
    base = (mp as *mut libc::c_char)
        .offset((*mp).mp_pb.pb.pb_upper as libc::c_int as isize)
        .offset(
            (if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        );
    memmove(
        base.offset(delta as libc::c_int as isize) as *mut libc::c_void,
        base as *const libc::c_void,
        (sp as *mut libc::c_char)
            .offset(len as libc::c_int as isize)
            .offset_from(base) as libc::c_long as libc::c_ulong,
    );
    ptr = *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize);
    i = (((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint).wrapping_sub(
        (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
            PAGEHDRSZ as libc::c_uint
        } else {
            0 as libc::c_int as libc::c_uint
        }),
    ) >> 1 as libc::c_int) as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        if *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int <= ptr as libc::c_int {
            let ref mut fresh56 = *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize);
            *fresh56 = (*fresh56 as libc::c_int + delta as libc::c_int) as indx_t;
        }
    }
    (*mp).mp_pb.pb.pb_upper =
        ((*mp).mp_pb.pb.pb_upper as libc::c_int + delta as libc::c_int) as indx_t;
}
unsafe extern "C" fn mdb_xcursor_init0(mut mc: *mut MDB_cursor) {
    let mut mx = (*mc).mc_xcursor;
    (*mx).mx_cursor.mc_xcursor = NULL as *mut MDB_xcursor;
    (*mx).mx_cursor.mc_txn = (*mc).mc_txn;
    (*mx).mx_cursor.mc_db = &mut (*mx).mx_db;
    (*mx).mx_cursor.mc_dbx = &mut (*mx).mx_dbx;
    (*mx).mx_cursor.mc_dbi = (*mc).mc_dbi;
    (*mx).mx_cursor.mc_dbflag = &mut (*mx).mx_dbflag;
    (*mx).mx_cursor.mc_snum = 0 as libc::c_int as libc::c_ushort;
    (*mx).mx_cursor.mc_top = 0 as libc::c_int as libc::c_ushort;
    (*mx).mx_cursor.mc_flags =
        C_SUB as libc::c_uint | (*mc).mc_flags & (C_ORIG_RDONLY | C_WRITEMAP) as libc::c_uint;
    (*mx).mx_dbx.md_name.mv_size = 0 as libc::c_int as size_t;
    (*mx).mx_dbx.md_name.mv_data = NULL as *mut libc::c_void;
    (*mx).mx_dbx.md_cmp = (*(*mc).mc_dbx).md_dcmp;
    (*mx).mx_dbx.md_dcmp =
        ::core::mem::transmute::<libc::intptr_t, Option<MDB_cmp_func>>(NULL as libc::intptr_t);
    (*mx).mx_dbx.md_rel = (*(*mc).mc_dbx).md_rel;
}
unsafe extern "C" fn mdb_xcursor_init1(mut mc: *mut MDB_cursor, mut node: *mut MDB_node) {
    let mut mx = (*mc).mc_xcursor;
    (*mx).mx_cursor.mc_flags &= (C_SUB | C_ORIG_RDONLY | C_WRITEMAP) as libc::c_uint;
    if (*node).mn_flags as libc::c_int & F_SUBDATA != 0 {
        memcpy(
            &mut (*mx).mx_db as *mut MDB_db as *mut libc::c_void,
            ((*node).mn_data)
                .as_mut_ptr()
                .offset((*node).mn_ksize as libc::c_int as isize) as *mut libc::c_void,
            ::core::mem::size_of::<MDB_db>() as libc::c_ulong,
        );
        (*mx).mx_cursor.mc_pg[0 as libc::c_int as usize] = 0 as *mut MDB_page;
        (*mx).mx_cursor.mc_snum = 0 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_top = 0 as libc::c_int as libc::c_ushort;
    } else {
        let mut fp = ((*node).mn_data)
            .as_mut_ptr()
            .offset((*node).mn_ksize as libc::c_int as isize)
            as *mut libc::c_void as *mut MDB_page;
        (*mx).mx_db.md_pad = 0 as libc::c_int as uint32_t;
        (*mx).mx_db.md_flags = 0 as libc::c_int as uint16_t;
        (*mx).mx_db.md_depth = 1 as libc::c_int as uint16_t;
        (*mx).mx_db.md_branch_pages = 0 as libc::c_int as pgno_t;
        (*mx).mx_db.md_leaf_pages = 1 as libc::c_int as pgno_t;
        (*mx).mx_db.md_overflow_pages = 0 as libc::c_int as pgno_t;
        (*mx).mx_db.md_entries = (((*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_lower
            as libc::c_uint)
            .wrapping_sub(
                (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }),
            )
            >> 1 as libc::c_int) as mdb_size_t;
        let mut s = 0 as *mut libc::c_ushort;
        let mut d = 0 as *mut libc::c_ushort;
        s = &mut (*(fp as *mut libc::c_void as *mut MDB_page2)).mp2_p as *mut [uint16_t; 4]
            as *mut libc::c_ushort;
        d = &mut (*mx).mx_db.md_root as *mut pgno_t as *mut libc::c_ushort;
        let fresh57 = s;
        s = s.offset(1);
        let fresh58 = d;
        d = d.offset(1);
        *fresh58 = *fresh57;
        let fresh59 = s;
        s = s.offset(1);
        let fresh60 = d;
        d = d.offset(1);
        *fresh60 = *fresh59;
        let fresh61 = s;
        s = s.offset(1);
        let fresh62 = d;
        d = d.offset(1);
        *fresh62 = *fresh61;
        *d = *s;
        (*mx).mx_cursor.mc_snum = 1 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_top = 0 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_flags |= C_INITIALIZED as libc::c_uint;
        (*mx).mx_cursor.mc_pg[0 as libc::c_int as usize] = fp;
        (*mx).mx_cursor.mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
        if (*(*mc).mc_db).md_flags as libc::c_int & MDB_DUPFIXED != 0 {
            (*mx).mx_db.md_flags = MDB_DUPFIXED as uint16_t;
            (*mx).mx_db.md_pad = (*fp).mp_pad as uint32_t;
            if (*(*mc).mc_db).md_flags as libc::c_int & MDB_INTEGERDUP != 0 {
                (*mx).mx_db.md_flags =
                    ((*mx).mx_db.md_flags as libc::c_int | MDB_INTEGERKEY) as uint16_t;
            }
        }
    }
    (*mx).mx_dbflag = (DB_VALID | DB_USRVALID | DB_DUPDATA) as libc::c_uchar;
    if (UINT_MAX as libc::c_ulong) < MDB_SIZE_MAX
        && (*mx).mx_dbx.md_cmp == Some(mdb_cmp_int as MDB_cmp_func)
        && (*mx).mx_db.md_pad as libc::c_ulong
            == ::core::mem::size_of::<mdb_size_t>() as libc::c_ulong
    {
        (*mx).mx_dbx.md_cmp = Some(mdb_cmp_clong);
    }
}
unsafe extern "C" fn mdb_xcursor_init2(
    mut mc: *mut MDB_cursor,
    mut src_mx: *mut MDB_xcursor,
    mut new_dupdata: libc::c_int,
) {
    let mut mx = (*mc).mc_xcursor;
    if new_dupdata != 0 {
        (*mx).mx_cursor.mc_snum = 1 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_top = 0 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_flags |= C_INITIALIZED as libc::c_uint;
        (*mx).mx_cursor.mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
        (*mx).mx_dbflag = (DB_VALID | DB_USRVALID | DB_DUPDATA) as libc::c_uchar;
        (*mx).mx_dbx.md_cmp = (*src_mx).mx_dbx.md_cmp;
    } else if (*mx).mx_cursor.mc_flags & C_INITIALIZED as libc::c_uint == 0 {
        return;
    }
    (*mx).mx_db = (*src_mx).mx_db;
    (*mx).mx_cursor.mc_pg[0 as libc::c_int as usize] =
        (*src_mx).mx_cursor.mc_pg[0 as libc::c_int as usize];
}
unsafe extern "C" fn mdb_cursor_init(
    mut mc: *mut MDB_cursor,
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut mx: *mut MDB_xcursor,
) {
    (*mc).mc_next = NULL as *mut MDB_cursor;
    (*mc).mc_backup = NULL as *mut MDB_cursor;
    (*mc).mc_dbi = dbi;
    (*mc).mc_txn = txn;
    (*mc).mc_db = &mut *((*txn).mt_dbs).offset(dbi as isize) as *mut MDB_db;
    (*mc).mc_dbx = &mut *((*txn).mt_dbxs).offset(dbi as isize) as *mut MDB_dbx;
    (*mc).mc_dbflag = &mut *((*txn).mt_dbflags).offset(dbi as isize) as *mut libc::c_uchar;
    (*mc).mc_snum = 0 as libc::c_int as libc::c_ushort;
    (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
    (*mc).mc_pg[0 as libc::c_int as usize] = 0 as *mut MDB_page;
    (*mc).mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
    (*mc).mc_flags = (*txn).mt_flags & (C_ORIG_RDONLY | C_WRITEMAP) as libc::c_uint;
    if (*((*txn).mt_dbs).offset(dbi as isize)).md_flags as libc::c_int & MDB_DUPSORT != 0 {
        if !mx.is_null() {
        } else {
            mdb_assert_fail(
                (*txn).mt_env,
                b"mx != NULL\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"mdb_cursor_init\0"))
                    .as_ptr(),
                b"mdb.c\0" as *const u8 as *const libc::c_char,
                8739 as libc::c_int,
            );
        };
        (*mc).mc_xcursor = mx;
        mdb_xcursor_init0(mc);
    } else {
        (*mc).mc_xcursor = NULL as *mut MDB_xcursor;
    }
    if *(*mc).mc_dbflag as libc::c_int & DB_STALE != 0 {
        mdb_page_search(mc, NULL as *mut MDB_val, MDB_PS_ROOTONLY);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mdb_cursor_open(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut ret: *mut *mut MDB_cursor,
) -> libc::c_int {
    let mut mc = 0 as *mut MDB_cursor;
    let mut size = ::core::mem::size_of::<MDB_cursor>() as libc::c_ulong;
    if ret.is_null()
        || !(!txn.is_null()
            && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x8 as libc::c_int != 0)
    {
        return EINVAL;
    }
    if (*txn).mt_flags & MDB_TXN_BLOCKED as libc::c_uint != 0 {
        return MDB_BAD_TXN;
    }
    if dbi == FREE_DBI as MDB_dbi
        && !((*txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint
            == 0x20000 as libc::c_int as libc::c_uint)
    {
        return EINVAL;
    }
    if (*((*txn).mt_dbs).offset(dbi as isize)).md_flags as libc::c_int & MDB_DUPSORT != 0 {
        size = (size as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<MDB_xcursor>() as libc::c_ulong)
            as size_t as size_t;
    }
    mc = malloc(size) as *mut MDB_cursor;
    if !mc.is_null() {
        mdb_cursor_init(
            mc,
            txn,
            dbi,
            mc.offset(1 as libc::c_int as isize) as *mut MDB_xcursor,
        );
        if !((*txn).mt_cursors).is_null() {
            (*mc).mc_next = *((*txn).mt_cursors).offset(dbi as isize);
            let ref mut fresh63 = *((*txn).mt_cursors).offset(dbi as isize);
            *fresh63 = mc;
            (*mc).mc_flags |= C_UNTRACK as libc::c_uint;
        }
    } else {
        return ENOMEM;
    }
    *ret = mc;
    return MDB_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_cursor_renew(
    mut txn: *mut MDB_txn,
    mut mc: *mut MDB_cursor,
) -> libc::c_int {
    if mc.is_null()
        || !(!txn.is_null()
            && (*mc).mc_dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset((*mc).mc_dbi as isize) as libc::c_int
                & 0x8 as libc::c_int
                != 0)
    {
        return EINVAL;
    }
    if (*mc).mc_flags & C_UNTRACK as libc::c_uint != 0 || !((*txn).mt_cursors).is_null() {
        return EINVAL;
    }
    if (*txn).mt_flags & MDB_TXN_BLOCKED as libc::c_uint != 0 {
        return MDB_BAD_TXN;
    }
    mdb_cursor_init(mc, txn, (*mc).mc_dbi, (*mc).mc_xcursor);
    return MDB_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_cursor_count(
    mut mc: *mut MDB_cursor,
    mut countp: *mut mdb_size_t,
) -> libc::c_int {
    let mut leaf = 0 as *mut MDB_node;
    if mc.is_null() || countp.is_null() {
        return EINVAL;
    }
    if ((*mc).mc_xcursor).is_null() {
        return MDB_INCOMPATIBLE;
    }
    if (*(*mc).mc_txn).mt_flags & MDB_TXN_BLOCKED as libc::c_uint != 0 {
        return MDB_BAD_TXN;
    }
    if (*mc).mc_flags & C_INITIALIZED as libc::c_uint == 0 {
        return EINVAL;
    }
    if (*mc).mc_snum == 0 {
        return MDB_NOTFOUND;
    }
    if (*mc).mc_flags & C_EOF as libc::c_uint != 0 {
        if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint
            >= ((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                .mp2_lower as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int
        {
            return MDB_NOTFOUND;
        }
        (*mc).mc_flags ^= C_EOF as libc::c_uint;
    }
    leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
        .offset(
            *((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                .mp2_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    if !((*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int) {
        *countp = 1 as libc::c_int as mdb_size_t;
    } else {
        if (*(*mc).mc_xcursor).mx_cursor.mc_flags & C_INITIALIZED as libc::c_uint == 0 {
            return EINVAL;
        }
        *countp = (*(*mc).mc_xcursor).mx_db.md_entries;
    }
    return MDB_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_cursor_close(mut mc: *mut MDB_cursor) {
    let _ = !mc.is_null();
    if !mc.is_null() && ((*mc).mc_backup).is_null() {
        if (*mc).mc_flags & C_UNTRACK as libc::c_uint != 0
            && !((*(*mc).mc_txn).mt_cursors).is_null()
        {
            let mut prev: *mut *mut MDB_cursor = &mut *((*(*mc).mc_txn).mt_cursors)
                .offset((*mc).mc_dbi as isize)
                as *mut *mut MDB_cursor;
            while !(*prev).is_null() && *prev != mc {
                prev = &mut (**prev).mc_next;
            }
            if *prev == mc {
                *prev = (*mc).mc_next;
            }
        }
        free(mc as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mdb_cursor_txn(mut mc: *mut MDB_cursor) -> *mut MDB_txn {
    if mc.is_null() {
        return NULL as *mut MDB_txn;
    }
    return (*mc).mc_txn;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_cursor_dbi(mut mc: *mut MDB_cursor) -> MDB_dbi {
    return (*mc).mc_dbi;
}
unsafe extern "C" fn mdb_update_key(mut mc: *mut MDB_cursor, mut key: *mut MDB_val) -> libc::c_int {
    let mut mp = 0 as *mut MDB_page;
    let mut node = 0 as *mut MDB_node;
    let mut base = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut delta: libc::c_int = 0;
    let mut ksize: libc::c_int = 0;
    let mut oksize: libc::c_int = 0;
    let mut ptr: indx_t = 0;
    let mut i: indx_t = 0;
    let mut numkeys: indx_t = 0;
    let mut indx: indx_t = 0;
    indx = (*mc).mc_ki[(*mc).mc_top as usize];
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    node = (mp as *mut libc::c_char)
        .offset(
            *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                .as_mut_ptr()
                .offset(indx as isize) as libc::c_int as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    ptr = *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize);
    ksize = (((*key).mv_size).wrapping_add(1 as libc::c_uint as size_t)
        & -(2 as libc::c_int) as size_t) as libc::c_int;
    oksize = (((*node).mn_ksize as libc::c_uint).wrapping_add(1 as libc::c_uint)
        & -(2 as libc::c_int) as libc::c_uint) as libc::c_int;
    delta = ksize - oksize;
    if delta != 0 {
        if delta > 0 as libc::c_int
            && (((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_upper as libc::c_int
                - (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_int)
                as indx_t as libc::c_int)
                < delta
        {
            let mut pgno: pgno_t = 0;
            pgno = (*node).mn_lo as pgno_t
                | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                | (if (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                    32 as libc::c_int
                } else {
                    0 as libc::c_int
                }) != 0
                {
                    ((*node).mn_flags as pgno_t)
                        << (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })
                } else {
                    0 as libc::c_int as pgno_t
                });
            mdb_node_del(mc, 0 as libc::c_int);
            return mdb_page_split(
                mc,
                key,
                NULL as *mut MDB_val,
                pgno,
                MDB_SPLIT_REPLACE as libc::c_uint,
            );
        }
        numkeys = (((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
            .wrapping_sub(
                (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }),
            )
            >> 1 as libc::c_int) as indx_t;
        i = 0 as libc::c_int as indx_t;
        while (i as libc::c_int) < numkeys as libc::c_int {
            if *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int <= ptr as libc::c_int
            {
                let ref mut fresh64 = *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize);
                *fresh64 = (*fresh64 as libc::c_int - delta) as indx_t;
            }
            i = i.wrapping_add(1);
            i;
        }
        base = (mp as *mut libc::c_char)
            .offset((*mp).mp_pb.pb.pb_upper as libc::c_int as isize)
            .offset(
                (if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            );
        len = ((ptr as libc::c_int - (*mp).mp_pb.pb.pb_upper as libc::c_int) as libc::c_ulong)
            .wrapping_add(NODESIZE);
        memmove(
            base.offset(-(delta as isize)) as *mut libc::c_void,
            base as *const libc::c_void,
            len,
        );
        (*mp).mp_pb.pb.pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_int - delta) as indx_t;
        node = (mp as *mut libc::c_char)
            .offset(
                *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(indx as isize) as libc::c_int as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
    }
    if (*node).mn_ksize as size_t != (*key).mv_size {
        (*node).mn_ksize = (*key).mv_size as libc::c_ushort;
    }
    if (*key).mv_size != 0 {
        memcpy(
            ((*node).mn_data).as_mut_ptr() as *mut libc::c_void,
            (*key).mv_data,
            (*key).mv_size,
        );
    }
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_node_move(
    mut csrc: *mut MDB_cursor,
    mut cdst: *mut MDB_cursor,
    mut fromleft: libc::c_int,
) -> libc::c_int {
    let mut srcnode = 0 as *mut MDB_node;
    let mut key = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut srcpg: pgno_t = 0;
    let mut mn = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut rc: libc::c_int = 0;
    let mut flags: libc::c_ushort = 0;
    rc = mdb_page_touch(csrc);
    if rc != 0 || {
        rc = mdb_page_touch(cdst);
        rc != 0
    } {
        return rc;
    }
    if (*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_flags
        as libc::c_int
        & 0x20 as libc::c_int
        == 0x20 as libc::c_int
    {
        key.mv_size = (*(*csrc).mc_db).md_pad as size_t;
        key.mv_data = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
            .offset(PAGEHDRSZ as libc::c_uint as isize)
            .offset(((*csrc).mc_ki[(*csrc).mc_top as usize] as size_t * key.mv_size) as isize)
            as *mut libc::c_void;
        data.mv_size = 0 as libc::c_int as size_t;
        data.mv_data = NULL as *mut libc::c_void;
        srcpg = 0 as libc::c_int as pgno_t;
        flags = 0 as libc::c_int as libc::c_ushort;
    } else {
        srcnode = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
            .offset(
                *((*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_void
                    as *mut MDB_page2))
                    .mp2_ptrs)
                    .as_mut_ptr()
                    .offset((*csrc).mc_ki[(*csrc).mc_top as usize] as isize)
                    as libc::c_int as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if srcnode as size_t & 1 as libc::c_int as size_t == 0 {
        } else {
            mdb_assert_fail(
                (*(*csrc).mc_txn).mt_env,
                b"!((size_t)srcnode & 1)\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"mdb_node_move\0"))
                    .as_ptr(),
                b"mdb.c\0" as *const u8 as *const libc::c_char,
                8997 as libc::c_int,
            );
        };
        srcpg = (*srcnode).mn_lo as pgno_t
            | ((*srcnode).mn_hi as pgno_t) << 16 as libc::c_int
            | (if (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                32 as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0
            {
                ((*srcnode).mn_flags as pgno_t)
                    << (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                        32 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })
            } else {
                0 as libc::c_int as pgno_t
            });
        flags = (*srcnode).mn_flags;
        if (*csrc).mc_ki[(*csrc).mc_top as usize] as libc::c_int == 0 as libc::c_int
            && (*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                .mp2_flags as libc::c_int
                & 0x1 as libc::c_int
                == 0x1 as libc::c_int
        {
            let mut snum = (*csrc).mc_snum as libc::c_uint;
            let mut s2 = 0 as *mut MDB_node;
            rc = mdb_page_search_lowest(csrc);
            if rc != 0 {
                return rc;
            }
            if (*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                .mp2_flags as libc::c_int
                & 0x20 as libc::c_int
                == 0x20 as libc::c_int
            {
                key.mv_size = (*(*csrc).mc_db).md_pad as size_t;
                key.mv_data = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
                    .offset(PAGEHDRSZ as libc::c_uint as isize)
                    .offset((0 as libc::c_int as size_t * key.mv_size) as isize)
                    as *mut libc::c_void;
            } else {
                s2 = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_void
                            as *mut MDB_page2))
                            .mp2_ptrs)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                key.mv_size = (*s2).mn_ksize as size_t;
                key.mv_data = ((*s2).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
            let fresh65 = snum;
            snum = snum.wrapping_sub(1);
            (*csrc).mc_snum = fresh65 as libc::c_ushort;
            (*csrc).mc_top = snum as libc::c_ushort;
        } else {
            key.mv_size = (*srcnode).mn_ksize as size_t;
            key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
        }
        data.mv_size = ((*srcnode).mn_lo as libc::c_uint
            | ((*srcnode).mn_hi as libc::c_uint) << 16 as libc::c_int)
            as size_t;
        data.mv_data = ((*srcnode).mn_data)
            .as_mut_ptr()
            .offset((*srcnode).mn_ksize as libc::c_int as isize)
            as *mut libc::c_void;
    }
    mn.mc_xcursor = NULL as *mut MDB_xcursor;
    if (*((*cdst).mc_pg[(*cdst).mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_flags
        as libc::c_int
        & 0x1 as libc::c_int
        == 0x1 as libc::c_int
        && (*cdst).mc_ki[(*cdst).mc_top as usize] as libc::c_int == 0 as libc::c_int
    {
        let mut snum_0 = (*cdst).mc_snum as libc::c_uint;
        let mut s2_0 = 0 as *mut MDB_node;
        let mut bkey = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        mdb_cursor_copy(cdst, &mut mn);
        rc = mdb_page_search_lowest(&mut mn);
        if rc != 0 {
            return rc;
        }
        if (*(mn.mc_pg[mn.mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_flags
            as libc::c_int
            & 0x20 as libc::c_int
            == 0x20 as libc::c_int
        {
            bkey.mv_size = (*mn.mc_db).md_pad as size_t;
            bkey.mv_data = (mn.mc_pg[mn.mc_top as usize] as *mut libc::c_char)
                .offset(PAGEHDRSZ as libc::c_uint as isize)
                .offset((0 as libc::c_int as size_t * bkey.mv_size) as isize)
                as *mut libc::c_void;
        } else {
            s2_0 = (mn.mc_pg[mn.mc_top as usize] as *mut libc::c_char)
                .offset(
                    *((*(mn.mc_pg[mn.mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                        .mp2_ptrs)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            bkey.mv_size = (*s2_0).mn_ksize as size_t;
            bkey.mv_data = ((*s2_0).mn_data).as_mut_ptr() as *mut libc::c_void;
        }
        let fresh66 = snum_0;
        snum_0 = snum_0.wrapping_sub(1);
        mn.mc_snum = fresh66 as libc::c_ushort;
        mn.mc_top = snum_0 as libc::c_ushort;
        mn.mc_ki[snum_0 as usize] = 0 as libc::c_int as indx_t;
        rc = mdb_update_key(&mut mn, &mut bkey);
        if rc != 0 {
            return rc;
        }
    }
    rc = mdb_node_add(
        cdst,
        (*cdst).mc_ki[(*cdst).mc_top as usize],
        &mut key,
        &mut data,
        srcpg,
        flags as libc::c_uint,
    );
    if rc != MDB_SUCCESS {
        return rc;
    }
    mdb_node_del(csrc, key.mv_size as libc::c_int);
    let mut m2 = 0 as *mut MDB_cursor;
    let mut m3 = 0 as *mut MDB_cursor;
    let mut dbi = (*csrc).mc_dbi;
    let mut mpd = 0 as *mut MDB_page;
    let mut mps = 0 as *mut MDB_page;
    mps = (*csrc).mc_pg[(*csrc).mc_top as usize];
    if fromleft != 0 {
        mpd = (*cdst).mc_pg[(*csrc).mc_top as usize];
        m2 = *((*(*csrc).mc_txn).mt_cursors).offset(dbi as isize);
        while !m2.is_null() {
            if (*csrc).mc_flags & C_SUB as libc::c_uint != 0 {
                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
            } else {
                m3 = m2;
            }
            if !((*m3).mc_flags & C_INITIALIZED as libc::c_uint == 0
                || ((*m3).mc_top as libc::c_int) < (*csrc).mc_top as libc::c_int)
            {
                if m3 != cdst
                    && (*m3).mc_pg[(*csrc).mc_top as usize] == mpd
                    && (*m3).mc_ki[(*csrc).mc_top as usize] as libc::c_int
                        >= (*cdst).mc_ki[(*csrc).mc_top as usize] as libc::c_int
                {
                    (*m3).mc_ki[(*csrc).mc_top as usize] =
                        ((*m3).mc_ki[(*csrc).mc_top as usize]).wrapping_add(1);
                    (*m3).mc_ki[(*csrc).mc_top as usize];
                }
                if m3 != csrc
                    && (*m3).mc_pg[(*csrc).mc_top as usize] == mps
                    && (*m3).mc_ki[(*csrc).mc_top as usize] as libc::c_int
                        == (*csrc).mc_ki[(*csrc).mc_top as usize] as libc::c_int
                {
                    (*m3).mc_pg[(*csrc).mc_top as usize] = (*cdst).mc_pg[(*cdst).mc_top as usize];
                    (*m3).mc_ki[(*csrc).mc_top as usize] = (*cdst).mc_ki[(*cdst).mc_top as usize];
                    (*m3).mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int) as usize] =
                        ((*m3).mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int) as usize])
                            .wrapping_add(1);
                    (*m3).mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int) as usize];
                }
                if (*(mps as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                    & 0x2 as libc::c_int
                    == 0x2 as libc::c_int
                {
                    let mut xr_pg = (*m3).mc_pg[(*csrc).mc_top as usize];
                    let mut xr_node = 0 as *mut MDB_node;
                    if !(!(!((*m3).mc_xcursor).is_null()
                        && (*(*m3).mc_xcursor).mx_cursor.mc_flags & C_INITIALIZED as libc::c_uint
                            != 0)
                        || (*m3).mc_ki[(*csrc).mc_top as usize] as libc::c_uint
                            >= ((*(xr_pg as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                                as libc::c_uint)
                                .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                    if 0 as libc::c_int != 0 {
                                        PAGEHDRSZ as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    },
                                ))
                                >> 1 as libc::c_int)
                    {
                        xr_node = (xr_pg as *mut libc::c_char)
                            .offset(
                                *((*(xr_pg as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                    .as_mut_ptr()
                                    .offset((*m3).mc_ki[(*csrc).mc_top as usize] as isize)
                                    as libc::c_int as isize,
                            )
                            .offset(
                                (if 0 as libc::c_int != 0 {
                                    PAGEHDRSZ as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        if (*xr_node).mn_flags as libc::c_int & (F_DUPDATA | F_SUBDATA) == F_DUPDATA
                        {
                            (*(*m3).mc_xcursor).mx_cursor.mc_pg[0 as libc::c_int as usize] =
                                ((*xr_node).mn_data)
                                    .as_mut_ptr()
                                    .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                    as *mut libc::c_void
                                    as *mut MDB_page;
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
    } else {
        m2 = *((*(*csrc).mc_txn).mt_cursors).offset(dbi as isize);
        while !m2.is_null() {
            if (*csrc).mc_flags & C_SUB as libc::c_uint != 0 {
                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
            } else {
                m3 = m2;
            }
            if !(m3 == csrc) {
                if !((*m3).mc_flags & C_INITIALIZED as libc::c_uint == 0
                    || ((*m3).mc_top as libc::c_int) < (*csrc).mc_top as libc::c_int)
                {
                    if (*m3).mc_pg[(*csrc).mc_top as usize] == mps {
                        if (*m3).mc_ki[(*csrc).mc_top as usize] == 0 {
                            (*m3).mc_pg[(*csrc).mc_top as usize] =
                                (*cdst).mc_pg[(*cdst).mc_top as usize];
                            (*m3).mc_ki[(*csrc).mc_top as usize] =
                                (*cdst).mc_ki[(*cdst).mc_top as usize];
                            (*m3).mc_ki
                                [((*csrc).mc_top as libc::c_int - 1 as libc::c_int) as usize] =
                                ((*m3).mc_ki
                                    [((*csrc).mc_top as libc::c_int - 1 as libc::c_int) as usize])
                                    .wrapping_sub(1);
                            (*m3).mc_ki
                                [((*csrc).mc_top as libc::c_int - 1 as libc::c_int) as usize];
                        } else {
                            (*m3).mc_ki[(*csrc).mc_top as usize] =
                                ((*m3).mc_ki[(*csrc).mc_top as usize]).wrapping_sub(1);
                            (*m3).mc_ki[(*csrc).mc_top as usize];
                        }
                        if (*(mps as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                            & 0x2 as libc::c_int
                            == 0x2 as libc::c_int
                        {
                            let mut xr_pg_0 = (*m3).mc_pg[(*csrc).mc_top as usize];
                            let mut xr_node_0 = 0 as *mut MDB_node;
                            if !(!(!((*m3).mc_xcursor).is_null()
                                && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                    & C_INITIALIZED as libc::c_uint
                                    != 0)
                                || (*m3).mc_ki[(*csrc).mc_top as usize] as libc::c_uint
                                    >= ((*(xr_pg_0 as *mut libc::c_void as *mut MDB_page2))
                                        .mp2_lower
                                        as libc::c_uint)
                                        .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                            if 0 as libc::c_int != 0 {
                                                PAGEHDRSZ as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            },
                                        ))
                                        >> 1 as libc::c_int)
                            {
                                xr_node_0 = (xr_pg_0 as *mut libc::c_char)
                                    .offset(
                                        *((*(xr_pg_0 as *mut libc::c_void as *mut MDB_page2))
                                            .mp2_ptrs)
                                            .as_mut_ptr()
                                            .offset((*m3).mc_ki[(*csrc).mc_top as usize] as isize)
                                            as libc::c_int
                                            as isize,
                                    )
                                    .offset(
                                        (if 0 as libc::c_int != 0 {
                                            PAGEHDRSZ as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        }) as isize,
                                    ) as *mut MDB_node;
                                if (*xr_node_0).mn_flags as libc::c_int & (F_DUPDATA | F_SUBDATA)
                                    == F_DUPDATA
                                {
                                    (*(*m3).mc_xcursor).mx_cursor.mc_pg
                                        [0 as libc::c_int as usize] = ((*xr_node_0).mn_data)
                                        .as_mut_ptr()
                                        .offset((*xr_node_0).mn_ksize as libc::c_int as isize)
                                        as *mut libc::c_void
                                        as *mut MDB_page;
                                }
                            }
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
    }
    if (*csrc).mc_ki[(*csrc).mc_top as usize] as libc::c_int == 0 as libc::c_int {
        if (*csrc).mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int) as usize] as libc::c_int
            != 0 as libc::c_int
        {
            if (*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                .mp2_flags as libc::c_int
                & 0x20 as libc::c_int
                == 0x20 as libc::c_int
            {
                key.mv_data = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
                    .offset(PAGEHDRSZ as libc::c_uint as isize)
                    .offset((0 as libc::c_int as size_t * key.mv_size) as isize)
                    as *mut libc::c_void;
            } else {
                srcnode = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_void
                            as *mut MDB_page2))
                            .mp2_ptrs)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                key.mv_size = (*srcnode).mn_ksize as size_t;
                key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
            mdb_cursor_copy(csrc, &mut mn);
            mn.mc_snum = (mn.mc_snum).wrapping_sub(1);
            mn.mc_snum;
            mn.mc_top = (mn.mc_top).wrapping_sub(1);
            mn.mc_top;
            let mut dummy = MDB_cursor {
                mc_next: 0 as *mut MDB_cursor,
                mc_backup: 0 as *mut MDB_cursor,
                mc_xcursor: 0 as *mut MDB_xcursor,
                mc_txn: 0 as *mut MDB_txn,
                mc_dbi: 0,
                mc_db: 0 as *mut MDB_db,
                mc_dbx: 0 as *mut MDB_dbx,
                mc_dbflag: 0 as *mut libc::c_uchar,
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [0 as *mut MDB_page; 32],
                mc_ki: [0; 32],
            };
            let mut tracked = 0 as *mut MDB_cursor;
            let mut tp: *mut *mut MDB_cursor =
                &mut *((*mn.mc_txn).mt_cursors).offset(mn.mc_dbi as isize) as *mut *mut MDB_cursor;
            if mn.mc_flags & C_SUB as libc::c_uint != 0 {
                dummy.mc_flags = C_INITIALIZED as libc::c_uint;
                dummy.mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                tracked = &mut dummy;
            } else {
                tracked = &mut mn;
            }
            (*tracked).mc_next = *tp;
            *tp = tracked;
            rc = mdb_update_key(&mut mn, &mut key);
            *tp = (*tracked).mc_next;
            if rc != 0 {
                return rc;
            }
        }
        if (*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
            .mp2_flags as libc::c_int
            & 0x1 as libc::c_int
            == 0x1 as libc::c_int
        {
            let mut nullkey = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            let mut ix = (*csrc).mc_ki[(*csrc).mc_top as usize];
            nullkey.mv_size = 0 as libc::c_int as size_t;
            (*csrc).mc_ki[(*csrc).mc_top as usize] = 0 as libc::c_int as indx_t;
            rc = mdb_update_key(csrc, &mut nullkey);
            (*csrc).mc_ki[(*csrc).mc_top as usize] = ix;
            if rc == 0 as libc::c_int {
            } else {
                mdb_assert_fail(
                    (*(*csrc).mc_txn).mt_env,
                    b"rc == MDB_SUCCESS\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"mdb_node_move\0"))
                        .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    9154 as libc::c_int,
                );
            };
        }
    }
    if (*cdst).mc_ki[(*cdst).mc_top as usize] as libc::c_int == 0 as libc::c_int {
        if (*cdst).mc_ki[((*cdst).mc_top as libc::c_int - 1 as libc::c_int) as usize] as libc::c_int
            != 0 as libc::c_int
        {
            if (*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                .mp2_flags as libc::c_int
                & 0x20 as libc::c_int
                == 0x20 as libc::c_int
            {
                key.mv_data = ((*cdst).mc_pg[(*cdst).mc_top as usize] as *mut libc::c_char)
                    .offset(PAGEHDRSZ as libc::c_uint as isize)
                    .offset((0 as libc::c_int as size_t * key.mv_size) as isize)
                    as *mut libc::c_void;
            } else {
                srcnode = ((*cdst).mc_pg[(*cdst).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*((*cdst).mc_pg[(*cdst).mc_top as usize] as *mut libc::c_void
                            as *mut MDB_page2))
                            .mp2_ptrs)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                key.mv_size = (*srcnode).mn_ksize as size_t;
                key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
            mdb_cursor_copy(cdst, &mut mn);
            mn.mc_snum = (mn.mc_snum).wrapping_sub(1);
            mn.mc_snum;
            mn.mc_top = (mn.mc_top).wrapping_sub(1);
            mn.mc_top;
            let mut dummy_0 = MDB_cursor {
                mc_next: 0 as *mut MDB_cursor,
                mc_backup: 0 as *mut MDB_cursor,
                mc_xcursor: 0 as *mut MDB_xcursor,
                mc_txn: 0 as *mut MDB_txn,
                mc_dbi: 0,
                mc_db: 0 as *mut MDB_db,
                mc_dbx: 0 as *mut MDB_dbx,
                mc_dbflag: 0 as *mut libc::c_uchar,
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [0 as *mut MDB_page; 32],
                mc_ki: [0; 32],
            };
            let mut tracked_0 = 0 as *mut MDB_cursor;
            let mut tp_0: *mut *mut MDB_cursor =
                &mut *((*mn.mc_txn).mt_cursors).offset(mn.mc_dbi as isize) as *mut *mut MDB_cursor;
            if mn.mc_flags & C_SUB as libc::c_uint != 0 {
                dummy_0.mc_flags = C_INITIALIZED as libc::c_uint;
                dummy_0.mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                tracked_0 = &mut dummy_0;
            } else {
                tracked_0 = &mut mn;
            }
            (*tracked_0).mc_next = *tp_0;
            *tp_0 = tracked_0;
            rc = mdb_update_key(&mut mn, &mut key);
            *tp_0 = (*tracked_0).mc_next;
            if rc != 0 {
                return rc;
            }
        }
        if (*((*cdst).mc_pg[(*cdst).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
            .mp2_flags as libc::c_int
            & 0x1 as libc::c_int
            == 0x1 as libc::c_int
        {
            let mut nullkey_0 = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            let mut ix_0 = (*cdst).mc_ki[(*cdst).mc_top as usize];
            nullkey_0.mv_size = 0 as libc::c_int as size_t;
            (*cdst).mc_ki[(*cdst).mc_top as usize] = 0 as libc::c_int as indx_t;
            rc = mdb_update_key(cdst, &mut nullkey_0);
            (*cdst).mc_ki[(*cdst).mc_top as usize] = ix_0;
            if rc == 0 as libc::c_int {
            } else {
                mdb_assert_fail(
                    (*(*cdst).mc_txn).mt_env,
                    b"rc == MDB_SUCCESS\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"mdb_node_move\0"))
                        .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    9185 as libc::c_int,
                );
            };
        }
    }
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_page_merge(
    mut csrc: *mut MDB_cursor,
    mut cdst: *mut MDB_cursor,
) -> libc::c_int {
    let mut psrc = 0 as *mut MDB_page;
    let mut pdst = 0 as *mut MDB_page;
    let mut srcnode = 0 as *mut MDB_node;
    let mut key = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut nkeys: libc::c_uint = 0;
    let mut rc: libc::c_int = 0;
    let mut i: indx_t = 0;
    let mut j: indx_t = 0;
    psrc = (*csrc).mc_pg[(*csrc).mc_top as usize];
    pdst = (*cdst).mc_pg[(*cdst).mc_top as usize];
    if (*csrc).mc_snum as libc::c_int > 1 as libc::c_int {
    } else {
        mdb_assert_fail(
            (*(*csrc).mc_txn).mt_env,
            b"csrc->mc_snum > 1\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"mdb_page_merge\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            9215 as libc::c_int,
        );
    };
    if (*cdst).mc_snum as libc::c_int > 1 as libc::c_int {
    } else {
        mdb_assert_fail(
            (*(*csrc).mc_txn).mt_env,
            b"cdst->mc_snum > 1\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"mdb_page_merge\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            9216 as libc::c_int,
        );
    };
    rc = mdb_page_touch(cdst);
    if rc != 0 {
        return rc;
    }
    pdst = (*cdst).mc_pg[(*cdst).mc_top as usize];
    nkeys =
        ((*(pdst as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint).wrapping_sub(
            (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }),
        ) >> 1 as libc::c_int;
    j = nkeys as indx_t;
    if (*(psrc as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
        & 0x20 as libc::c_int
        == 0x20 as libc::c_int
    {
        key.mv_size = (*(*csrc).mc_db).md_pad as size_t;
        key.mv_data = (psrc as *mut libc::c_char).offset(PAGEHDRSZ as libc::c_uint as isize)
            as *mut libc::c_void;
        i = 0 as libc::c_int as indx_t;
        while (i as libc::c_uint)
            < ((*(psrc as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int
        {
            rc = mdb_node_add(
                cdst,
                j,
                &mut key,
                NULL as *mut MDB_val,
                0 as libc::c_int as pgno_t,
                0 as libc::c_int as libc::c_uint,
            );
            if rc != MDB_SUCCESS {
                return rc;
            }
            key.mv_data = (key.mv_data as *mut libc::c_char).offset(key.mv_size as isize)
                as *mut libc::c_void;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
        }
    } else {
        i = 0 as libc::c_int as indx_t;
        while (i as libc::c_uint)
            < ((*(psrc as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int
        {
            srcnode = (psrc as *mut libc::c_char)
                .offset(
                    *((*(psrc as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                        .as_mut_ptr()
                        .offset(i as isize) as libc::c_int as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            if i as libc::c_int == 0 as libc::c_int
                && (*(psrc as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                    & 0x1 as libc::c_int
                    == 0x1 as libc::c_int
            {
                let mut mn = MDB_cursor {
                    mc_next: 0 as *mut MDB_cursor,
                    mc_backup: 0 as *mut MDB_cursor,
                    mc_xcursor: 0 as *mut MDB_xcursor,
                    mc_txn: 0 as *mut MDB_txn,
                    mc_dbi: 0,
                    mc_db: 0 as *mut MDB_db,
                    mc_dbx: 0 as *mut MDB_dbx,
                    mc_dbflag: 0 as *mut libc::c_uchar,
                    mc_snum: 0,
                    mc_top: 0,
                    mc_flags: 0,
                    mc_pg: [0 as *mut MDB_page; 32],
                    mc_ki: [0; 32],
                };
                let mut s2 = 0 as *mut MDB_node;
                mdb_cursor_copy(csrc, &mut mn);
                mn.mc_xcursor = NULL as *mut MDB_xcursor;
                rc = mdb_page_search_lowest(&mut mn);
                if rc != 0 {
                    return rc;
                }
                if (*(mn.mc_pg[mn.mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                    .mp2_flags as libc::c_int
                    & 0x20 as libc::c_int
                    == 0x20 as libc::c_int
                {
                    key.mv_size = (*mn.mc_db).md_pad as size_t;
                    key.mv_data = (mn.mc_pg[mn.mc_top as usize] as *mut libc::c_char)
                        .offset(PAGEHDRSZ as libc::c_uint as isize)
                        .offset((0 as libc::c_int as size_t * key.mv_size) as isize)
                        as *mut libc::c_void;
                } else {
                    s2 = (mn.mc_pg[mn.mc_top as usize] as *mut libc::c_char)
                        .offset(
                            *((*(mn.mc_pg[mn.mc_top as usize] as *mut libc::c_void
                                as *mut MDB_page2))
                                .mp2_ptrs)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize)
                                as libc::c_int as isize,
                        )
                        .offset(
                            (if 0 as libc::c_int != 0 {
                                PAGEHDRSZ as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    key.mv_size = (*s2).mn_ksize as size_t;
                    key.mv_data = ((*s2).mn_data).as_mut_ptr() as *mut libc::c_void;
                }
            } else {
                key.mv_size = (*srcnode).mn_ksize as size_t;
                key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
            data.mv_size = ((*srcnode).mn_lo as libc::c_uint
                | ((*srcnode).mn_hi as libc::c_uint) << 16 as libc::c_int)
                as size_t;
            data.mv_data = ((*srcnode).mn_data)
                .as_mut_ptr()
                .offset((*srcnode).mn_ksize as libc::c_int as isize)
                as *mut libc::c_void;
            rc = mdb_node_add(
                cdst,
                j,
                &mut key,
                &mut data,
                (*srcnode).mn_lo as pgno_t
                    | ((*srcnode).mn_hi as pgno_t) << 16 as libc::c_int
                    | (if (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                        32 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) != 0
                    {
                        ((*srcnode).mn_flags as pgno_t)
                            << (if -(1 as libc::c_int) as pgno_t
                                > 0xffffffff as libc::c_uint as pgno_t
                            {
                                32 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })
                    } else {
                        0 as libc::c_int as pgno_t
                    }),
                (*srcnode).mn_flags as libc::c_uint,
            );
            if rc != MDB_SUCCESS {
                return rc;
            }
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
        }
    }
    (*csrc).mc_top = ((*csrc).mc_top).wrapping_sub(1);
    (*csrc).mc_top;
    mdb_node_del(csrc, 0 as libc::c_int);
    if (*csrc).mc_ki[(*csrc).mc_top as usize] as libc::c_int == 0 as libc::c_int {
        key.mv_size = 0 as libc::c_int as size_t;
        rc = mdb_update_key(csrc, &mut key);
        if rc != 0 {
            (*csrc).mc_top = ((*csrc).mc_top).wrapping_add(1);
            (*csrc).mc_top;
            return rc;
        }
    }
    (*csrc).mc_top = ((*csrc).mc_top).wrapping_add(1);
    (*csrc).mc_top;
    psrc = (*csrc).mc_pg[(*csrc).mc_top as usize];
    rc = mdb_page_loose(csrc, psrc);
    if rc != 0 {
        return rc;
    }
    if (*(psrc as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
        & 0x2 as libc::c_int
        == 0x2 as libc::c_int
    {
        (*(*csrc).mc_db).md_leaf_pages = ((*(*csrc).mc_db).md_leaf_pages).wrapping_sub(1);
        (*(*csrc).mc_db).md_leaf_pages;
    } else {
        (*(*csrc).mc_db).md_branch_pages = ((*(*csrc).mc_db).md_branch_pages).wrapping_sub(1);
        (*(*csrc).mc_db).md_branch_pages;
    }
    let mut m2 = 0 as *mut MDB_cursor;
    let mut m3 = 0 as *mut MDB_cursor;
    let mut dbi = (*csrc).mc_dbi;
    let mut top = (*csrc).mc_top as libc::c_uint;
    m2 = *((*(*csrc).mc_txn).mt_cursors).offset(dbi as isize);
    while !m2.is_null() {
        if (*csrc).mc_flags & C_SUB as libc::c_uint != 0 {
            m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
        } else {
            m3 = m2;
        }
        if !(m3 == csrc) {
            if !(((*m3).mc_snum as libc::c_int) < (*csrc).mc_snum as libc::c_int) {
                if (*m3).mc_pg[top as usize] == psrc {
                    (*m3).mc_pg[top as usize] = pdst;
                    (*m3).mc_ki[top as usize] = ((*m3).mc_ki[top as usize] as libc::c_uint)
                        .wrapping_add(nkeys)
                        as indx_t as indx_t;
                    (*m3).mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize] =
                        (*cdst).mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize];
                } else if (*m3).mc_pg[top.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                    == (*csrc).mc_pg[top.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                    && (*m3).mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                        as libc::c_int
                        > (*csrc).mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                            as libc::c_int
                {
                    (*m3).mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize] =
                        ((*m3).mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
                            .wrapping_sub(1);
                    (*m3).mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize];
                }
                if (*(psrc as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                    & 0x2 as libc::c_int
                    == 0x2 as libc::c_int
                {
                    let mut xr_pg = (*m3).mc_pg[top as usize];
                    let mut xr_node = 0 as *mut MDB_node;
                    if !(!(!((*m3).mc_xcursor).is_null()
                        && (*(*m3).mc_xcursor).mx_cursor.mc_flags & C_INITIALIZED as libc::c_uint
                            != 0)
                        || (*m3).mc_ki[top as usize] as libc::c_uint
                            >= ((*(xr_pg as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                                as libc::c_uint)
                                .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                    if 0 as libc::c_int != 0 {
                                        PAGEHDRSZ as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    },
                                ))
                                >> 1 as libc::c_int)
                    {
                        xr_node = (xr_pg as *mut libc::c_char)
                            .offset(
                                *((*(xr_pg as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                    .as_mut_ptr()
                                    .offset((*m3).mc_ki[top as usize] as isize)
                                    as libc::c_int as isize,
                            )
                            .offset(
                                (if 0 as libc::c_int != 0 {
                                    PAGEHDRSZ as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        if (*xr_node).mn_flags as libc::c_int & (F_DUPDATA | F_SUBDATA) == F_DUPDATA
                        {
                            (*(*m3).mc_xcursor).mx_cursor.mc_pg[0 as libc::c_int as usize] =
                                ((*xr_node).mn_data)
                                    .as_mut_ptr()
                                    .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                    as *mut libc::c_void
                                    as *mut MDB_page;
                        }
                    }
                }
            }
        }
        m2 = (*m2).mc_next;
    }
    let mut snum = (*cdst).mc_snum as libc::c_uint;
    let mut depth = (*(*cdst).mc_db).md_depth;
    mdb_cursor_pop(cdst);
    rc = mdb_rebalance(cdst);
    if depth as libc::c_int != (*(*cdst).mc_db).md_depth as libc::c_int {
        snum = snum.wrapping_add(
            ((*(*cdst).mc_db).md_depth as libc::c_int - depth as libc::c_int) as libc::c_uint,
        );
    }
    (*cdst).mc_snum = snum as libc::c_ushort;
    (*cdst).mc_top = snum.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ushort;
    return rc;
}
unsafe extern "C" fn mdb_cursor_copy(mut csrc: *const MDB_cursor, mut cdst: *mut MDB_cursor) {
    let mut i: libc::c_uint = 0;
    (*cdst).mc_txn = (*csrc).mc_txn;
    (*cdst).mc_dbi = (*csrc).mc_dbi;
    (*cdst).mc_db = (*csrc).mc_db;
    (*cdst).mc_dbx = (*csrc).mc_dbx;
    (*cdst).mc_snum = (*csrc).mc_snum;
    (*cdst).mc_top = (*csrc).mc_top;
    (*cdst).mc_flags = (*csrc).mc_flags;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*csrc).mc_snum as libc::c_uint {
        (*cdst).mc_pg[i as usize] = (*csrc).mc_pg[i as usize];
        (*cdst).mc_ki[i as usize] = (*csrc).mc_ki[i as usize];
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn mdb_rebalance(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut node = 0 as *mut MDB_node;
    let mut rc: libc::c_int = 0;
    let mut fromleft: libc::c_int = 0;
    let mut ptop: libc::c_uint = 0;
    let mut minkeys: libc::c_uint = 0;
    let mut thresh: libc::c_uint = 0;
    let mut mn = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut oldki: indx_t = 0;
    if (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_flags
        as libc::c_int
        & 0x1 as libc::c_int
        == 0x1 as libc::c_int
    {
        minkeys = 2 as libc::c_int as libc::c_uint;
        thresh = 1 as libc::c_int as libc::c_uint;
    } else {
        minkeys = 1 as libc::c_int as libc::c_uint;
        thresh = FILL_THRESHOLD as libc::c_uint;
    }
    if 1000 as libc::c_long
        * ((*(*(*mc).mc_txn).mt_env).me_psize)
            .wrapping_sub(PAGEHDRSZ as libc::c_uint)
            .wrapping_sub(
                ((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                    .mp2_upper as libc::c_int
                    - (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                        .mp2_lower as libc::c_int) as indx_t as libc::c_uint,
            ) as libc::c_long
        / ((*(*(*mc).mc_txn).mt_env).me_psize).wrapping_sub(PAGEHDRSZ as libc::c_uint)
            as libc::c_long
        >= thresh as libc::c_long
        && ((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_lower
            as libc::c_uint)
            .wrapping_sub(
                (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }),
            )
            >> 1 as libc::c_int
            >= minkeys
    {
        return MDB_SUCCESS;
    }
    if ((*mc).mc_snum as libc::c_int) < 2 as libc::c_int {
        let mut mp = (*mc).mc_pg[0 as libc::c_int as usize];
        if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
            & 0x40 as libc::c_int
            == 0x40 as libc::c_int
        {
            return MDB_SUCCESS;
        }
        if ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint).wrapping_sub(
            (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }),
        ) >> 1 as libc::c_int
            == 0 as libc::c_int as libc::c_uint
        {
            (*(*mc).mc_db).md_root = P_INVALID;
            (*(*mc).mc_db).md_depth = 0 as libc::c_int as uint16_t;
            (*(*mc).mc_db).md_leaf_pages = 0 as libc::c_int as pgno_t;
            rc = mdb_midl_append(&mut (*(*mc).mc_txn).mt_free_pgs, (*mp).mp_p.p_pgno);
            if rc != 0 {
                return rc;
            }
            (*mc).mc_snum = 0 as libc::c_int as libc::c_ushort;
            (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
            (*mc).mc_flags &= !C_INITIALIZED as libc::c_uint;
            let mut m2 = 0 as *mut MDB_cursor;
            let mut m3 = 0 as *mut MDB_cursor;
            let mut dbi = (*mc).mc_dbi;
            m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
            while !m2.is_null() {
                if (*mc).mc_flags & C_SUB as libc::c_uint != 0 {
                    m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                } else {
                    m3 = m2;
                }
                if !((*m3).mc_flags & C_INITIALIZED as libc::c_uint == 0
                    || ((*m3).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int)
                {
                    if (*m3).mc_pg[0 as libc::c_int as usize] == mp {
                        (*m3).mc_snum = 0 as libc::c_int as libc::c_ushort;
                        (*m3).mc_top = 0 as libc::c_int as libc::c_ushort;
                        (*m3).mc_flags &= !C_INITIALIZED as libc::c_uint;
                    }
                }
                m2 = (*m2).mc_next;
            }
        } else if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
            & 0x1 as libc::c_int
            == 0x1 as libc::c_int
            && ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int
                == 1 as libc::c_int as libc::c_uint
        {
            let mut i: libc::c_int = 0;
            rc = mdb_midl_append(&mut (*(*mc).mc_txn).mt_free_pgs, (*mp).mp_p.p_pgno);
            if rc != 0 {
                return rc;
            }
            (*(*mc).mc_db).md_root = (*((mp as *mut libc::c_char)
                .offset(
                    *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node))
                .mn_lo as pgno_t
                | ((*((mp as *mut libc::c_char)
                    .offset(
                        *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node))
                    .mn_hi as pgno_t)
                    << 16 as libc::c_int
                | (if (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                    32 as libc::c_int
                } else {
                    0 as libc::c_int
                }) != 0
                {
                    ((*((mp as *mut libc::c_char)
                        .offset(
                            *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize)
                                as libc::c_int as isize,
                        )
                        .offset(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as isize,
                        ) as *mut MDB_node))
                        .mn_flags as pgno_t)
                        << (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })
                } else {
                    0 as libc::c_int as pgno_t
                });
            rc = mdb_page_get(
                mc,
                (*(*mc).mc_db).md_root,
                &mut *((*mc).mc_pg).as_mut_ptr().offset(0 as libc::c_int as isize),
                NULL as *mut libc::c_int,
            );
            if rc != 0 {
                return rc;
            }
            (*(*mc).mc_db).md_depth = ((*(*mc).mc_db).md_depth).wrapping_sub(1);
            (*(*mc).mc_db).md_depth;
            (*(*mc).mc_db).md_branch_pages = ((*(*mc).mc_db).md_branch_pages).wrapping_sub(1);
            (*(*mc).mc_db).md_branch_pages;
            (*mc).mc_ki[0 as libc::c_int as usize] = (*mc).mc_ki[1 as libc::c_int as usize];
            i = 1 as libc::c_int;
            while i < (*(*mc).mc_db).md_depth as libc::c_int {
                (*mc).mc_pg[i as usize] = (*mc).mc_pg[(i + 1 as libc::c_int) as usize];
                (*mc).mc_ki[i as usize] = (*mc).mc_ki[(i + 1 as libc::c_int) as usize];
                i += 1;
                i;
            }
            let mut m2_0 = 0 as *mut MDB_cursor;
            let mut m3_0 = 0 as *mut MDB_cursor;
            let mut dbi_0 = (*mc).mc_dbi;
            m2_0 = *((*(*mc).mc_txn).mt_cursors).offset(dbi_0 as isize);
            while !m2_0.is_null() {
                if (*mc).mc_flags & C_SUB as libc::c_uint != 0 {
                    m3_0 = &mut (*(*m2_0).mc_xcursor).mx_cursor;
                } else {
                    m3_0 = m2_0;
                }
                if !(m3_0 == mc) {
                    if !((*m3_0).mc_flags & C_INITIALIZED as libc::c_uint == 0) {
                        if (*m3_0).mc_pg[0 as libc::c_int as usize] == mp {
                            i = 0 as libc::c_int;
                            while i < (*(*mc).mc_db).md_depth as libc::c_int {
                                (*m3_0).mc_pg[i as usize] =
                                    (*m3_0).mc_pg[(i + 1 as libc::c_int) as usize];
                                (*m3_0).mc_ki[i as usize] =
                                    (*m3_0).mc_ki[(i + 1 as libc::c_int) as usize];
                                i += 1;
                                i;
                            }
                            (*m3_0).mc_snum = ((*m3_0).mc_snum).wrapping_sub(1);
                            (*m3_0).mc_snum;
                            (*m3_0).mc_top = ((*m3_0).mc_top).wrapping_sub(1);
                            (*m3_0).mc_top;
                        }
                    }
                }
                m2_0 = (*m2_0).mc_next;
            }
        }
        return MDB_SUCCESS;
    }
    ptop = ((*mc).mc_top as libc::c_int - 1 as libc::c_int) as libc::c_uint;
    if ((*((*mc).mc_pg[ptop as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_lower
        as libc::c_uint)
        .wrapping_sub((16 as libc::c_ulong as libc::c_uint).wrapping_sub(
            if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            },
        ))
        >> 1 as libc::c_int
        > 1 as libc::c_int as libc::c_uint
    {
    } else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"NUMKEYS(mc->mc_pg[ptop]) > 1\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"mdb_rebalance\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            9480 as libc::c_int,
        );
    };
    mdb_cursor_copy(mc, &mut mn);
    mn.mc_xcursor = NULL as *mut MDB_xcursor;
    oldki = (*mc).mc_ki[(*mc).mc_top as usize];
    if (*mc).mc_ki[ptop as usize] as libc::c_int == 0 as libc::c_int {
        mn.mc_ki[ptop as usize] = (mn.mc_ki[ptop as usize]).wrapping_add(1);
        mn.mc_ki[ptop as usize];
        node = ((*mc).mc_pg[ptop as usize] as *mut libc::c_char)
            .offset(
                *((*((*mc).mc_pg[ptop as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(mn.mc_ki[ptop as usize] as isize) as libc::c_int
                    as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        rc = mdb_page_get(
            mc,
            (*node).mn_lo as pgno_t
                | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                | (if (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                    32 as libc::c_int
                } else {
                    0 as libc::c_int
                }) != 0
                {
                    ((*node).mn_flags as pgno_t)
                        << (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })
                } else {
                    0 as libc::c_int as pgno_t
                }),
            &mut *(mn.mc_pg).as_mut_ptr().offset(mn.mc_top as isize),
            NULL as *mut libc::c_int,
        );
        if rc != 0 {
            return rc;
        }
        mn.mc_ki[mn.mc_top as usize] = 0 as libc::c_int as indx_t;
        (*mc).mc_ki[(*mc).mc_top as usize] =
            (((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                .mp2_lower as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int) as indx_t;
        fromleft = 0 as libc::c_int;
    } else {
        mn.mc_ki[ptop as usize] = (mn.mc_ki[ptop as usize]).wrapping_sub(1);
        mn.mc_ki[ptop as usize];
        node = ((*mc).mc_pg[ptop as usize] as *mut libc::c_char)
            .offset(
                *((*((*mc).mc_pg[ptop as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(mn.mc_ki[ptop as usize] as isize) as libc::c_int
                    as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        rc = mdb_page_get(
            mc,
            (*node).mn_lo as pgno_t
                | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                | (if (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                    32 as libc::c_int
                } else {
                    0 as libc::c_int
                }) != 0
                {
                    ((*node).mn_flags as pgno_t)
                        << (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })
                } else {
                    0 as libc::c_int as pgno_t
                }),
            &mut *(mn.mc_pg).as_mut_ptr().offset(mn.mc_top as isize),
            NULL as *mut libc::c_int,
        );
        if rc != 0 {
            return rc;
        }
        mn.mc_ki[mn.mc_top as usize] =
            (((*(mn.mc_pg[mn.mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
        (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
        fromleft = 1 as libc::c_int;
    }
    if 1000 as libc::c_long
        * ((*(*(*mc).mc_txn).mt_env).me_psize)
            .wrapping_sub(PAGEHDRSZ as libc::c_uint)
            .wrapping_sub(
                ((*(mn.mc_pg[mn.mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_upper
                    as libc::c_int
                    - (*(mn.mc_pg[mn.mc_top as usize] as *mut libc::c_void as *mut MDB_page2))
                        .mp2_lower as libc::c_int) as indx_t as libc::c_uint,
            ) as libc::c_long
        / ((*(*(*mc).mc_txn).mt_env).me_psize).wrapping_sub(PAGEHDRSZ as libc::c_uint)
            as libc::c_long
        >= thresh as libc::c_long
        && ((*(mn.mc_pg[mn.mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_lower
            as libc::c_uint)
            .wrapping_sub(
                (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }),
            )
            >> 1 as libc::c_int
            > minkeys
    {
        rc = mdb_node_move(&mut mn, mc, fromleft);
        if fromleft != 0 {
            oldki = oldki.wrapping_add(1);
            oldki;
        }
    } else {
        if fromleft == 0 {
            rc = mdb_page_merge(&mut mn, mc);
        } else {
            oldki = (oldki as libc::c_uint).wrapping_add(
                ((*(mn.mc_pg[mn.mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                    as libc::c_uint)
                    .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                        if 0 as libc::c_int != 0 {
                            PAGEHDRSZ as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        },
                    ))
                    >> 1 as libc::c_int,
            ) as indx_t as indx_t;
            mn.mc_ki[mn.mc_top as usize] = (mn.mc_ki[mn.mc_top as usize] as libc::c_int
                + ((*mc).mc_ki[mn.mc_top as usize] as libc::c_int + 1 as libc::c_int))
                as indx_t;
            let mut dummy = MDB_cursor {
                mc_next: 0 as *mut MDB_cursor,
                mc_backup: 0 as *mut MDB_cursor,
                mc_xcursor: 0 as *mut MDB_xcursor,
                mc_txn: 0 as *mut MDB_txn,
                mc_dbi: 0,
                mc_db: 0 as *mut MDB_db,
                mc_dbx: 0 as *mut MDB_dbx,
                mc_dbflag: 0 as *mut libc::c_uchar,
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [0 as *mut MDB_page; 32],
                mc_ki: [0; 32],
            };
            let mut tracked = 0 as *mut MDB_cursor;
            let mut tp: *mut *mut MDB_cursor =
                &mut *((*mn.mc_txn).mt_cursors).offset(mn.mc_dbi as isize) as *mut *mut MDB_cursor;
            if mn.mc_flags & C_SUB as libc::c_uint != 0 {
                dummy.mc_flags = C_INITIALIZED as libc::c_uint;
                dummy.mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                tracked = &mut dummy;
            } else {
                tracked = &mut mn;
            }
            (*tracked).mc_next = *tp;
            *tp = tracked;
            rc = mdb_page_merge(mc, &mut mn);
            *tp = (*tracked).mc_next;
            mdb_cursor_copy(&mut mn, mc);
        }
        (*mc).mc_flags &= !C_EOF as libc::c_uint;
    }
    (*mc).mc_ki[(*mc).mc_top as usize] = oldki;
    return rc;
}
unsafe extern "C" fn mdb_cursor_del0(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut mp = 0 as *mut MDB_page;
    let mut ki: indx_t = 0;
    let mut nkeys: libc::c_uint = 0;
    let mut m2 = 0 as *mut MDB_cursor;
    let mut m3 = 0 as *mut MDB_cursor;
    let mut dbi = (*mc).mc_dbi;
    ki = (*mc).mc_ki[(*mc).mc_top as usize];
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    mdb_node_del(mc, (*(*mc).mc_db).md_pad as libc::c_int);
    (*(*mc).mc_db).md_entries = ((*(*mc).mc_db).md_entries).wrapping_sub(1);
    (*(*mc).mc_db).md_entries;
    m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
    while !m2.is_null() {
        m3 = if (*mc).mc_flags & C_SUB as libc::c_uint != 0 {
            &mut (*(*m2).mc_xcursor).mx_cursor
        } else {
            m2
        };
        if !((*m2).mc_flags & (*m3).mc_flags & C_INITIALIZED as libc::c_uint == 0) {
            if !(m3 == mc || ((*m3).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int) {
                if (*m3).mc_pg[(*mc).mc_top as usize] == mp {
                    if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_int == ki as libc::c_int {
                        (*m3).mc_flags |= C_DEL as libc::c_uint;
                        if (*(*mc).mc_db).md_flags as libc::c_int & MDB_DUPSORT != 0 {
                            (*(*m3).mc_xcursor).mx_cursor.mc_flags &=
                                !(C_INITIALIZED | C_EOF) as libc::c_uint;
                        }
                    } else {
                        if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_int > ki as libc::c_int {
                            (*m3).mc_ki[(*mc).mc_top as usize] =
                                ((*m3).mc_ki[(*mc).mc_top as usize]).wrapping_sub(1);
                            (*m3).mc_ki[(*mc).mc_top as usize];
                        }
                        let mut xr_pg = mp;
                        let mut xr_node = 0 as *mut MDB_node;
                        if !(!(!((*m3).mc_xcursor).is_null()
                            && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                & C_INITIALIZED as libc::c_uint
                                != 0)
                            || (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                >= ((*(xr_pg as *mut libc::c_void as *mut MDB_page2)).mp2_lower
                                    as libc::c_uint)
                                    .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                        if 0 as libc::c_int != 0 {
                                            PAGEHDRSZ as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        },
                                    ))
                                    >> 1 as libc::c_int)
                        {
                            xr_node = (xr_pg as *mut libc::c_char)
                                .offset(
                                    *((*(xr_pg as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                        .as_mut_ptr()
                                        .offset((*m3).mc_ki[(*mc).mc_top as usize] as isize)
                                        as libc::c_int as isize,
                                )
                                .offset(
                                    (if 0 as libc::c_int != 0 {
                                        PAGEHDRSZ as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                            if (*xr_node).mn_flags as libc::c_int & (F_DUPDATA | F_SUBDATA)
                                == F_DUPDATA
                            {
                                (*(*m3).mc_xcursor).mx_cursor.mc_pg[0 as libc::c_int as usize] =
                                    ((*xr_node).mn_data)
                                        .as_mut_ptr()
                                        .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                        as *mut libc::c_void
                                        as *mut MDB_page;
                            }
                        }
                    }
                }
            }
        }
        m2 = (*m2).mc_next;
    }
    rc = mdb_rebalance(mc);
    if !(rc != 0) {
        if (*mc).mc_snum == 0 {
            (*mc).mc_flags |= C_EOF as libc::c_uint;
            return rc;
        }
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        nkeys = ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
            .wrapping_sub(
                (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }),
            )
            >> 1 as libc::c_int;
        m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
        loop {
            if !(rc == 0 && !m2.is_null()) {
                current_block = 13281731871476506071;
                break;
            }
            m3 = if (*mc).mc_flags & C_SUB as libc::c_uint != 0 {
                &mut (*(*m2).mc_xcursor).mx_cursor
            } else {
                m2
            };
            if !((*m2).mc_flags & (*m3).mc_flags & C_INITIALIZED as libc::c_uint == 0) {
                if !(((*m3).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int) {
                    if (*m3).mc_pg[(*mc).mc_top as usize] == mp {
                        if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_int
                            >= (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int
                        {
                            if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_uint >= nkeys {
                                rc = mdb_cursor_sibling(m3, 1 as libc::c_int);
                                if rc == MDB_NOTFOUND {
                                    (*m3).mc_flags |= C_EOF as libc::c_uint;
                                    rc = MDB_SUCCESS;
                                    current_block = 2569451025026770673;
                                } else {
                                    if rc != 0 {
                                        current_block = 5716779716218003043;
                                        break;
                                    }
                                    current_block = 18435049525520518667;
                                }
                            } else {
                                current_block = 18435049525520518667;
                            }
                            match current_block {
                                2569451025026770673 => {}
                                _ => {
                                    if !((*m3).mc_xcursor).is_null()
                                        && (*m3).mc_flags & C_EOF as libc::c_uint == 0
                                    {
                                        let mut node = ((*m3).mc_pg[(*m3).mc_top as usize]
                                            as *mut libc::c_char)
                                            .offset(
                                                *((*((*m3).mc_pg[(*m3).mc_top as usize]
                                                    as *mut libc::c_void
                                                    as *mut MDB_page2))
                                                    .mp2_ptrs)
                                                    .as_mut_ptr()
                                                    .offset(
                                                        (*m3).mc_ki[(*m3).mc_top as usize] as isize,
                                                    )
                                                    as libc::c_int
                                                    as isize,
                                            )
                                            .offset(
                                                (if 0 as libc::c_int != 0 {
                                                    PAGEHDRSZ as libc::c_uint
                                                } else {
                                                    0 as libc::c_int as libc::c_uint
                                                })
                                                    as isize,
                                            )
                                            as *mut MDB_node;
                                        if (*node).mn_flags as libc::c_int & F_DUPDATA != 0 {
                                            if (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                                & C_INITIALIZED as libc::c_uint
                                                != 0
                                            {
                                                if (*node).mn_flags as libc::c_int & F_SUBDATA == 0
                                                {
                                                    (*(*m3).mc_xcursor).mx_cursor.mc_pg
                                                        [0 as libc::c_int as usize] =
                                                        ((*node).mn_data).as_mut_ptr().offset(
                                                            (*node).mn_ksize as libc::c_int
                                                                as isize,
                                                        )
                                                            as *mut libc::c_void
                                                            as *mut MDB_page;
                                                }
                                            } else {
                                                mdb_xcursor_init1(m3, node);
                                                rc = mdb_cursor_first(
                                                    &mut (*(*m3).mc_xcursor).mx_cursor,
                                                    NULL as *mut MDB_val,
                                                    NULL as *mut MDB_val,
                                                );
                                                if rc != 0 {
                                                    current_block = 5716779716218003043;
                                                    break;
                                                }
                                            }
                                        }
                                        (*(*m3).mc_xcursor).mx_cursor.mc_flags |=
                                            C_DEL as libc::c_uint;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
        match current_block {
            5716779716218003043 => {}
            _ => {
                (*mc).mc_flags |= C_DEL as libc::c_uint;
            }
        }
    }
    if rc != 0 {
        (*(*mc).mc_txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_del(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    if key.is_null()
        || !(!txn.is_null()
            && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int != 0)
    {
        return EINVAL;
    }
    if (*txn).mt_flags & (MDB_TXN_RDONLY | MDB_TXN_BLOCKED) as libc::c_uint != 0 {
        return if (*txn).mt_flags & MDB_TXN_RDONLY as libc::c_uint != 0 {
            EACCES
        } else {
            MDB_BAD_TXN
        };
    }
    if !((*((*txn).mt_dbs).offset(dbi as isize)).md_flags as libc::c_int & 0x4 as libc::c_int
        == 0x4 as libc::c_int)
    {
        data = NULL as *mut MDB_val;
    }
    return mdb_del0(txn, dbi, key, data, 0 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn mdb_del0(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut mc = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut mx = MDB_xcursor {
        mx_cursor: MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        },
        mx_db: MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        },
        mx_dbx: MDB_dbx {
            md_name: MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            },
            md_cmp: None,
            md_dcmp: None,
            md_rel: None,
            md_relctx: 0 as *mut libc::c_void,
        },
        mx_dbflag: 0,
    };
    let mut op = MDB_FIRST;
    let mut rdata = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut xdata = 0 as *mut MDB_val;
    let mut rc: libc::c_int = 0;
    let mut exact = 0 as libc::c_int;
    mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
    if !data.is_null() {
        op = MDB_GET_BOTH;
        rdata = *data;
        xdata = &mut rdata;
    } else {
        op = MDB_SET;
        xdata = NULL as *mut MDB_val;
        flags |= MDB_NODUPDATA as libc::c_uint;
    }
    rc = mdb_cursor_set(&mut mc, key, xdata, op, &mut exact);
    if rc == 0 as libc::c_int {
        mc.mc_next = *((*txn).mt_cursors).offset(dbi as isize);
        let ref mut fresh67 = *((*txn).mt_cursors).offset(dbi as isize);
        *fresh67 = &mut mc;
        rc = _mdb_cursor_del(&mut mc, flags);
        let ref mut fresh68 = *((*txn).mt_cursors).offset(dbi as isize);
        *fresh68 = mc.mc_next;
    }
    return rc;
}
unsafe extern "C" fn mdb_page_split(
    mut mc: *mut MDB_cursor,
    mut newkey: *mut MDB_val,
    mut newdata: *mut MDB_val,
    mut newpgno: pgno_t,
    mut nflags: libc::c_uint,
) -> libc::c_int {
    let mut current_block: u64;
    let mut flags: libc::c_uint = 0;
    let mut rc = MDB_SUCCESS;
    let mut new_root = 0 as libc::c_int;
    let mut did_split = 0 as libc::c_int;
    let mut newindx: indx_t = 0;
    let mut pgno = 0 as libc::c_int as pgno_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut split_indx: libc::c_int = 0;
    let mut nkeys: libc::c_int = 0;
    let mut pmax: libc::c_int = 0;
    let mut env = (*(*mc).mc_txn).mt_env;
    let mut node = 0 as *mut MDB_node;
    let mut sepkey = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut rkey = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut xdata = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut rdata: *mut MDB_val = &mut xdata;
    let mut copy = NULL as *mut MDB_page;
    let mut mp = 0 as *mut MDB_page;
    let mut rp = 0 as *mut MDB_page;
    let mut pp = 0 as *mut MDB_page;
    let mut ptop: libc::c_int = 0;
    let mut mn = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    newindx = (*mc).mc_ki[(*mc).mc_top as usize];
    nkeys =
        (((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint).wrapping_sub(
            (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                PAGEHDRSZ as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }),
        ) >> 1 as libc::c_int) as libc::c_int;
    rc = mdb_page_new(mc, (*mp).mp_flags as uint32_t, 1 as libc::c_int, &mut rp);
    if rc != 0 {
        return rc;
    }
    (*rp).mp_pad = (*mp).mp_pad;
    if ((*mc).mc_top as libc::c_int) < 1 as libc::c_int {
        rc = mdb_page_new(mc, P_BRANCH as uint32_t, 1 as libc::c_int, &mut pp);
        if rc != 0 {
            current_block = 9884865637931044769;
        } else {
            i = (*mc).mc_snum as libc::c_int;
            while i > 0 as libc::c_int {
                (*mc).mc_pg[i as usize] = (*mc).mc_pg[(i - 1 as libc::c_int) as usize];
                (*mc).mc_ki[i as usize] = (*mc).mc_ki[(i - 1 as libc::c_int) as usize];
                i -= 1;
                i;
            }
            (*mc).mc_pg[0 as libc::c_int as usize] = pp;
            (*mc).mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
            (*(*mc).mc_db).md_root = (*pp).mp_p.p_pgno;
            let fresh69 = (*(*mc).mc_db).md_depth;
            (*(*mc).mc_db).md_depth = ((*(*mc).mc_db).md_depth).wrapping_add(1);
            new_root = fresh69 as libc::c_int;
            rc = mdb_node_add(
                mc,
                0 as libc::c_int as indx_t,
                NULL as *mut MDB_val,
                NULL as *mut MDB_val,
                (*mp).mp_p.p_pgno,
                0 as libc::c_int as libc::c_uint,
            );
            if rc != MDB_SUCCESS {
                (*mc).mc_pg[0 as libc::c_int as usize] = (*mc).mc_pg[1 as libc::c_int as usize];
                (*mc).mc_ki[0 as libc::c_int as usize] = (*mc).mc_ki[1 as libc::c_int as usize];
                (*(*mc).mc_db).md_root = (*mp).mp_p.p_pgno;
                (*(*mc).mc_db).md_depth = ((*(*mc).mc_db).md_depth).wrapping_sub(1);
                (*(*mc).mc_db).md_depth;
                current_block = 9884865637931044769;
            } else {
                (*mc).mc_snum = ((*mc).mc_snum).wrapping_add(1);
                (*mc).mc_snum;
                (*mc).mc_top = ((*mc).mc_top).wrapping_add(1);
                (*mc).mc_top;
                ptop = 0 as libc::c_int;
                current_block = 15925075030174552612;
            }
        }
    } else {
        ptop = (*mc).mc_top as libc::c_int - 1 as libc::c_int;
        current_block = 15925075030174552612;
    }
    match current_block {
        15925075030174552612 => {
            mdb_cursor_copy(mc, &mut mn);
            mn.mc_xcursor = NULL as *mut MDB_xcursor;
            mn.mc_pg[mn.mc_top as usize] = rp;
            mn.mc_ki[ptop as usize] =
                ((*mc).mc_ki[ptop as usize] as libc::c_int + 1 as libc::c_int) as indx_t;
            if nflags & MDB_APPEND as libc::c_uint != 0 {
                mn.mc_ki[mn.mc_top as usize] = 0 as libc::c_int as indx_t;
                sepkey = *newkey;
                split_indx = newindx as libc::c_int;
                nkeys = 0 as libc::c_int;
                current_block = 4804377075063615140;
            } else {
                split_indx = (nkeys + 1 as libc::c_int) / 2 as libc::c_int;
                if (*(rp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                    & 0x20 as libc::c_int
                    == 0x20 as libc::c_int
                {
                    let mut split = 0 as *mut libc::c_char;
                    let mut ins = 0 as *mut libc::c_char;
                    let mut x: libc::c_int = 0;
                    let mut lsize: libc::c_uint = 0;
                    let mut rsize: libc::c_uint = 0;
                    let mut ksize: libc::c_uint = 0;
                    x = (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int - split_indx;
                    ksize = (*(*mc).mc_db).md_pad;
                    split = (mp as *mut libc::c_char)
                        .offset(PAGEHDRSZ as libc::c_uint as isize)
                        .offset((split_indx as libc::c_uint).wrapping_mul(ksize) as isize);
                    rsize = ((nkeys - split_indx) as libc::c_uint).wrapping_mul(ksize);
                    lsize = ((nkeys - split_indx) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<indx_t>() as libc::c_ulong)
                        as libc::c_uint;
                    (*mp).mp_pb.pb.pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                        .wrapping_sub(lsize) as indx_t
                        as indx_t;
                    (*rp).mp_pb.pb.pb_lower = ((*rp).mp_pb.pb.pb_lower as libc::c_uint)
                        .wrapping_add(lsize) as indx_t
                        as indx_t;
                    (*mp).mp_pb.pb.pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_uint)
                        .wrapping_add(rsize.wrapping_sub(lsize))
                        as indx_t as indx_t;
                    (*rp).mp_pb.pb.pb_upper = ((*rp).mp_pb.pb.pb_upper as libc::c_uint)
                        .wrapping_sub(rsize.wrapping_sub(lsize))
                        as indx_t as indx_t;
                    sepkey.mv_size = ksize as size_t;
                    if newindx as libc::c_int == split_indx {
                        sepkey.mv_data = (*newkey).mv_data;
                    } else {
                        sepkey.mv_data = split as *mut libc::c_void;
                    }
                    if x < 0 as libc::c_int {
                        ins = (mp as *mut libc::c_char)
                            .offset(PAGEHDRSZ as libc::c_uint as isize)
                            .offset(
                                ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint)
                                    .wrapping_mul(ksize) as isize,
                            );
                        memcpy(
                            ((*rp).mp_ptrs).as_mut_ptr() as *mut libc::c_void,
                            split as *const libc::c_void,
                            rsize as libc::c_ulong,
                        );
                        sepkey.mv_data = ((*rp).mp_ptrs).as_mut_ptr() as *mut libc::c_void;
                        memmove(
                            ins.offset(ksize as isize) as *mut libc::c_void,
                            ins as *const libc::c_void,
                            ((split_indx - (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int)
                                as libc::c_uint)
                                .wrapping_mul(ksize) as libc::c_ulong,
                        );
                        memcpy(
                            ins as *mut libc::c_void,
                            (*newkey).mv_data,
                            ksize as libc::c_ulong,
                        );
                        (*mp).mp_pb.pb.pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_ulong)
                            .wrapping_add(::core::mem::size_of::<indx_t>() as libc::c_ulong)
                            as indx_t as indx_t;
                        (*mp).mp_pb.pb.pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_ulong)
                            .wrapping_sub(
                            (ksize as libc::c_ulong)
                                .wrapping_sub(::core::mem::size_of::<indx_t>() as libc::c_ulong),
                        ) as indx_t as indx_t;
                    } else {
                        if x != 0 {
                            memcpy(
                                ((*rp).mp_ptrs).as_mut_ptr() as *mut libc::c_void,
                                split as *const libc::c_void,
                                (x as libc::c_uint).wrapping_mul(ksize) as libc::c_ulong,
                            );
                        }
                        ins = (rp as *mut libc::c_char)
                            .offset(PAGEHDRSZ as libc::c_uint as isize)
                            .offset((x as libc::c_uint).wrapping_mul(ksize) as isize);
                        memcpy(
                            ins as *mut libc::c_void,
                            (*newkey).mv_data,
                            ksize as libc::c_ulong,
                        );
                        memcpy(
                            ins.offset(ksize as isize) as *mut libc::c_void,
                            split.offset((x as libc::c_uint).wrapping_mul(ksize) as isize)
                                as *const libc::c_void,
                            rsize.wrapping_sub((x as libc::c_uint).wrapping_mul(ksize))
                                as libc::c_ulong,
                        );
                        (*rp).mp_pb.pb.pb_lower = ((*rp).mp_pb.pb.pb_lower as libc::c_ulong)
                            .wrapping_add(::core::mem::size_of::<indx_t>() as libc::c_ulong)
                            as indx_t as indx_t;
                        (*rp).mp_pb.pb.pb_upper = ((*rp).mp_pb.pb.pb_upper as libc::c_ulong)
                            .wrapping_sub(
                            (ksize as libc::c_ulong)
                                .wrapping_sub(::core::mem::size_of::<indx_t>() as libc::c_ulong),
                        ) as indx_t as indx_t;
                        (*mc).mc_ki[(*mc).mc_top as usize] = x as indx_t;
                    }
                    current_block = 4804377075063615140;
                } else {
                    let mut psize: libc::c_int = 0;
                    let mut nsize: libc::c_int = 0;
                    let mut k: libc::c_int = 0;
                    let mut keythresh: libc::c_int = 0;
                    pmax = ((*env).me_psize).wrapping_sub(PAGEHDRSZ as libc::c_uint) as libc::c_int;
                    keythresh = ((*env).me_psize >> 7 as libc::c_int) as libc::c_int;
                    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                        & 0x2 as libc::c_int
                        == 0x2 as libc::c_int
                    {
                        nsize = mdb_leaf_size(env, newkey, newdata) as libc::c_int;
                    } else {
                        nsize = mdb_branch_size(env, newkey) as libc::c_int;
                    }
                    nsize = ((nsize as libc::c_uint).wrapping_add(1 as libc::c_uint)
                        & -(2 as libc::c_int) as libc::c_uint)
                        as libc::c_int;
                    copy = mdb_page_malloc((*mc).mc_txn, 1 as libc::c_int as libc::c_uint);
                    if copy.is_null() {
                        rc = ENOMEM;
                        current_block = 9884865637931044769;
                    } else {
                        (*copy).mp_p.p_pgno = (*mp).mp_p.p_pgno;
                        (*copy).mp_flags = (*mp).mp_flags;
                        (*copy).mp_pb.pb.pb_lower =
                            (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                                PAGEHDRSZ as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as indx_t;
                        (*copy).mp_pb.pb.pb_upper =
                            ((*env).me_psize).wrapping_sub(if 0 as libc::c_int != 0 {
                                PAGEHDRSZ as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as indx_t;
                        i = 0 as libc::c_int;
                        j = 0 as libc::c_int;
                        while i < nkeys {
                            if i == newindx as libc::c_int {
                                let fresh70 = j;
                                j = j + 1;
                                *((*copy).mp_ptrs).as_mut_ptr().offset(fresh70 as isize) =
                                    0 as libc::c_int as indx_t;
                            }
                            let fresh71 = j;
                            j = j + 1;
                            *((*copy).mp_ptrs).as_mut_ptr().offset(fresh71 as isize) =
                                *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize);
                            i += 1;
                            i;
                        }
                        if nkeys < keythresh
                            || nsize > pmax / 16 as libc::c_int
                            || newindx as libc::c_int >= nkeys
                        {
                            psize = 0 as libc::c_int;
                            if newindx as libc::c_int <= split_indx
                                || newindx as libc::c_int >= nkeys
                            {
                                i = 0 as libc::c_int;
                                j = 1 as libc::c_int;
                                k = if newindx as libc::c_int >= nkeys {
                                    nkeys
                                } else {
                                    split_indx
                                        + 1 as libc::c_int
                                        + ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags
                                            as libc::c_int
                                            & 0x2 as libc::c_int
                                            == 0x2 as libc::c_int)
                                            as libc::c_int
                                };
                            } else {
                                i = nkeys;
                                j = -(1 as libc::c_int);
                                k = split_indx - 1 as libc::c_int;
                            }
                            while i != k {
                                if i == newindx as libc::c_int {
                                    psize += nsize;
                                    node = NULL as *mut MDB_node;
                                } else {
                                    node = (mp as *mut libc::c_char)
                                        .offset(*((*copy).mp_ptrs).as_mut_ptr().offset(i as isize)
                                            as libc::c_int
                                            as isize)
                                        .offset(
                                            (if 0 as libc::c_int != 0 {
                                                PAGEHDRSZ as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            }) as isize,
                                        )
                                        as *mut MDB_node;
                                    psize = (psize as libc::c_ulong).wrapping_add(
                                        NODESIZE
                                            .wrapping_add((*node).mn_ksize as libc::c_ulong)
                                            .wrapping_add(
                                                ::core::mem::size_of::<indx_t>() as libc::c_ulong
                                            ),
                                    ) as libc::c_int
                                        as libc::c_int;
                                    if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags
                                        as libc::c_int
                                        & 0x2 as libc::c_int
                                        == 0x2 as libc::c_int
                                    {
                                        if (*node).mn_flags as libc::c_int & 0x1 as libc::c_int
                                            == 0x1 as libc::c_int
                                        {
                                            psize = (psize as libc::c_ulong)
                                                .wrapping_add(::core::mem::size_of::<pgno_t>()
                                                    as libc::c_ulong)
                                                as libc::c_int
                                                as libc::c_int;
                                        } else {
                                            psize = (psize as libc::c_uint).wrapping_add(
                                                (*node).mn_lo as libc::c_uint
                                                    | ((*node).mn_hi as libc::c_uint)
                                                        << 16 as libc::c_int,
                                            )
                                                as libc::c_int
                                                as libc::c_int;
                                        }
                                    }
                                    psize = ((psize as libc::c_uint)
                                        .wrapping_add(1 as libc::c_uint)
                                        & -(2 as libc::c_int) as libc::c_uint)
                                        as libc::c_int;
                                }
                                if psize > pmax || i == k - j {
                                    split_indx = i + (j < 0 as libc::c_int) as libc::c_int;
                                    break;
                                } else {
                                    i += j;
                                }
                            }
                        }
                        if split_indx == newindx as libc::c_int {
                            sepkey.mv_size = (*newkey).mv_size;
                            sepkey.mv_data = (*newkey).mv_data;
                        } else {
                            node = (mp as *mut libc::c_char)
                                .offset(*((*copy).mp_ptrs).as_mut_ptr().offset(split_indx as isize)
                                    as libc::c_int as isize)
                                .offset(
                                    (if 0 as libc::c_int != 0 {
                                        PAGEHDRSZ as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                            sepkey.mv_size = (*node).mn_ksize as size_t;
                            sepkey.mv_data = ((*node).mn_data).as_mut_ptr() as *mut libc::c_void;
                        }
                        current_block = 4804377075063615140;
                    }
                }
            }
            match current_block {
                9884865637931044769 => {}
                _ => {
                    if (((*(mn.mc_pg[ptop as usize] as *mut libc::c_void as *mut MDB_page2))
                        .mp2_upper as libc::c_int
                        - (*(mn.mc_pg[ptop as usize] as *mut libc::c_void as *mut MDB_page2))
                            .mp2_lower as libc::c_int) as indx_t as size_t)
                        < mdb_branch_size(env, &mut sepkey)
                    {
                        let mut snum = (*mc).mc_snum as libc::c_int;
                        mn.mc_snum = (mn.mc_snum).wrapping_sub(1);
                        mn.mc_snum;
                        mn.mc_top = (mn.mc_top).wrapping_sub(1);
                        mn.mc_top;
                        did_split = 1 as libc::c_int;
                        let mut dummy = MDB_cursor {
                            mc_next: 0 as *mut MDB_cursor,
                            mc_backup: 0 as *mut MDB_cursor,
                            mc_xcursor: 0 as *mut MDB_xcursor,
                            mc_txn: 0 as *mut MDB_txn,
                            mc_dbi: 0,
                            mc_db: 0 as *mut MDB_db,
                            mc_dbx: 0 as *mut MDB_dbx,
                            mc_dbflag: 0 as *mut libc::c_uchar,
                            mc_snum: 0,
                            mc_top: 0,
                            mc_flags: 0,
                            mc_pg: [0 as *mut MDB_page; 32],
                            mc_ki: [0; 32],
                        };
                        let mut tracked = 0 as *mut MDB_cursor;
                        let mut tp: *mut *mut MDB_cursor = &mut *((*mn.mc_txn).mt_cursors)
                            .offset(mn.mc_dbi as isize)
                            as *mut *mut MDB_cursor;
                        if mn.mc_flags & C_SUB as libc::c_uint != 0 {
                            dummy.mc_flags = C_INITIALIZED as libc::c_uint;
                            dummy.mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                            tracked = &mut dummy;
                        } else {
                            tracked = &mut mn;
                        }
                        (*tracked).mc_next = *tp;
                        *tp = tracked;
                        rc = mdb_page_split(
                            &mut mn,
                            &mut sepkey,
                            0 as *mut MDB_val,
                            (*rp).mp_p.p_pgno,
                            0 as libc::c_int as libc::c_uint,
                        );
                        *tp = (*tracked).mc_next;
                        if rc != 0 {
                            current_block = 9884865637931044769;
                        } else {
                            if (*mc).mc_snum as libc::c_int > snum {
                                ptop += 1;
                                ptop;
                            }
                            if mn.mc_pg[ptop as usize] != (*mc).mc_pg[ptop as usize]
                                && (*mc).mc_ki[ptop as usize] as libc::c_uint
                                    >= ((*((*mc).mc_pg[ptop as usize] as *mut libc::c_void
                                        as *mut MDB_page2))
                                        .mp2_lower
                                        as libc::c_uint)
                                        .wrapping_sub((PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                            if 0 as libc::c_int != 0 {
                                                PAGEHDRSZ as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            },
                                        ))
                                        >> 1 as libc::c_int
                            {
                                i = 0 as libc::c_int;
                                while i < ptop {
                                    (*mc).mc_pg[i as usize] = mn.mc_pg[i as usize];
                                    (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                    i += 1;
                                    i;
                                }
                                (*mc).mc_pg[ptop as usize] = mn.mc_pg[ptop as usize];
                                if mn.mc_ki[ptop as usize] != 0 {
                                    (*mc).mc_ki[ptop as usize] =
                                        (mn.mc_ki[ptop as usize] as libc::c_int - 1 as libc::c_int)
                                            as indx_t;
                                } else {
                                    (*mc).mc_ki[ptop as usize] = mn.mc_ki[ptop as usize];
                                    rc = mdb_cursor_sibling(mc, 0 as libc::c_int);
                                }
                            }
                            current_block = 5431927413890720344;
                        }
                    } else {
                        mn.mc_top = (mn.mc_top).wrapping_sub(1);
                        mn.mc_top;
                        rc = mdb_node_add(
                            &mut mn,
                            mn.mc_ki[ptop as usize],
                            &mut sepkey,
                            NULL as *mut MDB_val,
                            (*rp).mp_p.p_pgno,
                            0 as libc::c_int as libc::c_uint,
                        );
                        mn.mc_top = (mn.mc_top).wrapping_add(1);
                        mn.mc_top;
                        current_block = 5431927413890720344;
                    }
                    match current_block {
                        9884865637931044769 => {}
                        _ => {
                            if rc != MDB_SUCCESS {
                                if rc == MDB_NOTFOUND {
                                    rc = MDB_PROBLEM;
                                }
                            } else {
                                if nflags & MDB_APPEND as libc::c_uint != 0 {
                                    (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                    (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
                                    rc = mdb_node_add(
                                        mc,
                                        0 as libc::c_int as indx_t,
                                        newkey,
                                        newdata,
                                        newpgno,
                                        nflags,
                                    );
                                    if rc != 0 {
                                        current_block = 9884865637931044769;
                                    } else {
                                        i = 0 as libc::c_int;
                                        while i < (*mc).mc_top as libc::c_int {
                                            (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                            i += 1;
                                            i;
                                        }
                                        current_block = 6091595930016798176;
                                    }
                                } else if !((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags
                                    as libc::c_int
                                    & 0x20 as libc::c_int
                                    == 0x20 as libc::c_int)
                                {
                                    (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                    i = split_indx;
                                    j = 0 as libc::c_int;
                                    loop {
                                        if i == newindx as libc::c_int {
                                            rkey.mv_data = (*newkey).mv_data;
                                            rkey.mv_size = (*newkey).mv_size;
                                            if (*(mp as *mut libc::c_void as *mut MDB_page2))
                                                .mp2_flags
                                                as libc::c_int
                                                & 0x2 as libc::c_int
                                                == 0x2 as libc::c_int
                                            {
                                                rdata = newdata;
                                            } else {
                                                pgno = newpgno;
                                            }
                                            flags = nflags;
                                            (*mc).mc_ki[(*mc).mc_top as usize] = j as indx_t;
                                        } else {
                                            node = (mp as *mut libc::c_char)
                                                .offset(
                                                    *((*copy).mp_ptrs)
                                                        .as_mut_ptr()
                                                        .offset(i as isize)
                                                        as libc::c_int
                                                        as isize,
                                                )
                                                .offset(
                                                    (if 0 as libc::c_int != 0 {
                                                        PAGEHDRSZ as libc::c_uint
                                                    } else {
                                                        0 as libc::c_int as libc::c_uint
                                                    })
                                                        as isize,
                                                )
                                                as *mut MDB_node;
                                            rkey.mv_data =
                                                ((*node).mn_data).as_mut_ptr() as *mut libc::c_void;
                                            rkey.mv_size = (*node).mn_ksize as size_t;
                                            if (*(mp as *mut libc::c_void as *mut MDB_page2))
                                                .mp2_flags
                                                as libc::c_int
                                                & 0x2 as libc::c_int
                                                == 0x2 as libc::c_int
                                            {
                                                xdata.mv_data =
                                                    ((*node).mn_data).as_mut_ptr().offset(
                                                        (*node).mn_ksize as libc::c_int as isize,
                                                    )
                                                        as *mut libc::c_void;
                                                xdata.mv_size = ((*node).mn_lo as libc::c_uint
                                                    | ((*node).mn_hi as libc::c_uint)
                                                        << 16 as libc::c_int)
                                                    as size_t;
                                                rdata = &mut xdata;
                                            } else {
                                                pgno = (*node).mn_lo as pgno_t
                                                    | ((*node).mn_hi as pgno_t)
                                                        << 16 as libc::c_int
                                                    | (if (if -(1 as libc::c_int) as pgno_t
                                                        > 0xffffffff as libc::c_uint as pgno_t
                                                    {
                                                        32 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    }) != 0
                                                    {
                                                        ((*node).mn_flags as pgno_t)
                                                            << (if -(1 as libc::c_int) as pgno_t
                                                                > 0xffffffff as libc::c_uint
                                                                    as pgno_t
                                                            {
                                                                32 as libc::c_int
                                                            } else {
                                                                0 as libc::c_int
                                                            })
                                                    } else {
                                                        0 as libc::c_int as pgno_t
                                                    });
                                            }
                                            flags = (*node).mn_flags as libc::c_uint;
                                        }
                                        if !((*(mp as *mut libc::c_void as *mut MDB_page2))
                                            .mp2_flags
                                            as libc::c_int
                                            & 0x2 as libc::c_int
                                            == 0x2 as libc::c_int)
                                            && j == 0 as libc::c_int
                                        {
                                            rkey.mv_size = 0 as libc::c_int as size_t;
                                        }
                                        rc = mdb_node_add(
                                            mc,
                                            j as indx_t,
                                            &mut rkey,
                                            rdata,
                                            pgno,
                                            flags,
                                        );
                                        if rc != 0 {
                                            current_block = 9884865637931044769;
                                            break;
                                        }
                                        if i == nkeys {
                                            i = 0 as libc::c_int;
                                            j = 0 as libc::c_int;
                                            (*mc).mc_pg[(*mc).mc_top as usize] = copy;
                                        } else {
                                            i += 1;
                                            i;
                                            j += 1;
                                            j;
                                        }
                                        if !(i != split_indx) {
                                            current_block = 13598848910332274892;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9884865637931044769 => {}
                                        _ => {
                                            nkeys = (((*(copy as *mut libc::c_void
                                                as *mut MDB_page2))
                                                .mp2_lower
                                                as libc::c_uint)
                                                .wrapping_sub(
                                                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                                        if 0 as libc::c_int != 0 {
                                                            PAGEHDRSZ as libc::c_uint
                                                        } else {
                                                            0 as libc::c_int as libc::c_uint
                                                        },
                                                    ),
                                                )
                                                >> 1 as libc::c_int)
                                                as libc::c_int;
                                            i = 0 as libc::c_int;
                                            while i < nkeys {
                                                *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) =
                                                    *((*copy).mp_ptrs)
                                                        .as_mut_ptr()
                                                        .offset(i as isize);
                                                i += 1;
                                                i;
                                            }
                                            (*mp).mp_pb.pb.pb_lower = (*copy).mp_pb.pb.pb_lower;
                                            (*mp).mp_pb.pb.pb_upper = (*copy).mp_pb.pb.pb_upper;
                                            memcpy(
                                                (mp as *mut libc::c_char)
                                                    .offset(
                                                        *((*(mp as *mut libc::c_void
                                                            as *mut MDB_page2))
                                                            .mp2_ptrs)
                                                            .as_mut_ptr()
                                                            .offset(
                                                                (nkeys - 1 as libc::c_int) as isize,
                                                            )
                                                            as libc::c_int
                                                            as isize,
                                                    )
                                                    .offset(
                                                        (if 0 as libc::c_int != 0 {
                                                            PAGEHDRSZ as libc::c_uint
                                                        } else {
                                                            0 as libc::c_int as libc::c_uint
                                                        })
                                                            as isize,
                                                    )
                                                    as *mut MDB_node
                                                    as *mut libc::c_void,
                                                (copy as *mut libc::c_char)
                                                    .offset(
                                                        *((*(copy as *mut libc::c_void
                                                            as *mut MDB_page2))
                                                            .mp2_ptrs)
                                                            .as_mut_ptr()
                                                            .offset(
                                                                (nkeys - 1 as libc::c_int) as isize,
                                                            )
                                                            as libc::c_int
                                                            as isize,
                                                    )
                                                    .offset(
                                                        (if 0 as libc::c_int != 0 {
                                                            PAGEHDRSZ as libc::c_uint
                                                        } else {
                                                            0 as libc::c_int as libc::c_uint
                                                        })
                                                            as isize,
                                                    )
                                                    as *mut MDB_node
                                                    as *const libc::c_void,
                                                ((*env).me_psize)
                                                    .wrapping_sub(
                                                        (*copy).mp_pb.pb.pb_upper as libc::c_uint,
                                                    )
                                                    .wrapping_sub(if 0 as libc::c_int != 0 {
                                                        PAGEHDRSZ as libc::c_uint
                                                    } else {
                                                        0 as libc::c_int as libc::c_uint
                                                    })
                                                    as libc::c_ulong,
                                            );
                                            if (newindx as libc::c_int) < split_indx {
                                                (*mc).mc_pg[(*mc).mc_top as usize] = mp;
                                            } else {
                                                (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                                (*mc).mc_ki[ptop as usize] =
                                                    ((*mc).mc_ki[ptop as usize]).wrapping_add(1);
                                                (*mc).mc_ki[ptop as usize];
                                                if mn.mc_pg[ptop as usize]
                                                    != (*mc).mc_pg[ptop as usize]
                                                    && (*mc).mc_ki[ptop as usize] as libc::c_uint
                                                        >= ((*((*mc).mc_pg[ptop as usize]
                                                            as *mut libc::c_void
                                                            as *mut MDB_page2))
                                                            .mp2_lower
                                                            as libc::c_uint)
                                                            .wrapping_sub(
                                                                (PAGEHDRSZ as libc::c_uint)
                                                                    .wrapping_sub(
                                                                        if 0 as libc::c_int != 0 {
                                                                            PAGEHDRSZ
                                                                                as libc::c_uint
                                                                        } else {
                                                                            0 as libc::c_int
                                                                                as libc::c_uint
                                                                        },
                                                                    ),
                                                            )
                                                            >> 1 as libc::c_int
                                                {
                                                    i = 0 as libc::c_int;
                                                    while i <= ptop {
                                                        (*mc).mc_pg[i as usize] =
                                                            mn.mc_pg[i as usize];
                                                        (*mc).mc_ki[i as usize] =
                                                            mn.mc_ki[i as usize];
                                                        i += 1;
                                                        i;
                                                    }
                                                }
                                            }
                                            if nflags & MDB_RESERVE as libc::c_uint != 0 {
                                                node = ((*mc).mc_pg[(*mc).mc_top as usize]
                                                    as *mut libc::c_char)
                                                    .offset(
                                                        *((*((*mc).mc_pg[(*mc).mc_top as usize]
                                                            as *mut libc::c_void
                                                            as *mut MDB_page2))
                                                            .mp2_ptrs)
                                                            .as_mut_ptr()
                                                            .offset(
                                                                (*mc).mc_ki[(*mc).mc_top as usize]
                                                                    as isize,
                                                            )
                                                            as libc::c_int
                                                            as isize,
                                                    )
                                                    .offset(
                                                        (if 0 as libc::c_int != 0 {
                                                            PAGEHDRSZ as libc::c_uint
                                                        } else {
                                                            0 as libc::c_int as libc::c_uint
                                                        })
                                                            as isize,
                                                    )
                                                    as *mut MDB_node;
                                                if (*node).mn_flags as libc::c_int & F_BIGDATA == 0
                                                {
                                                    (*newdata).mv_data =
                                                        ((*node).mn_data).as_mut_ptr().offset(
                                                            (*node).mn_ksize as libc::c_int
                                                                as isize,
                                                        )
                                                            as *mut libc::c_void;
                                                }
                                            }
                                            current_block = 6091595930016798176;
                                        }
                                    }
                                } else {
                                    if newindx as libc::c_int >= split_indx {
                                        (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                        (*mc).mc_ki[ptop as usize] =
                                            ((*mc).mc_ki[ptop as usize]).wrapping_add(1);
                                        (*mc).mc_ki[ptop as usize];
                                        if mn.mc_pg[ptop as usize] != (*mc).mc_pg[ptop as usize]
                                            && (*mc).mc_ki[ptop as usize] as libc::c_uint
                                                >= ((*((*mc).mc_pg[ptop as usize]
                                                    as *mut libc::c_void
                                                    as *mut MDB_page2))
                                                    .mp2_lower
                                                    as libc::c_uint)
                                                    .wrapping_sub(
                                                        (PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                                            if 0 as libc::c_int != 0 {
                                                                PAGEHDRSZ as libc::c_uint
                                                            } else {
                                                                0 as libc::c_int as libc::c_uint
                                                            },
                                                        ),
                                                    )
                                                    >> 1 as libc::c_int
                                        {
                                            i = 0 as libc::c_int;
                                            while i <= ptop {
                                                (*mc).mc_pg[i as usize] = mn.mc_pg[i as usize];
                                                (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                                i += 1;
                                                i;
                                            }
                                        }
                                    }
                                    current_block = 6091595930016798176;
                                }
                                match current_block {
                                    9884865637931044769 => {}
                                    _ => {
                                        let mut m2 = 0 as *mut MDB_cursor;
                                        let mut m3 = 0 as *mut MDB_cursor;
                                        let mut dbi = (*mc).mc_dbi;
                                        nkeys = (((*(mp as *mut libc::c_void as *mut MDB_page2))
                                            .mp2_lower
                                            as libc::c_uint)
                                            .wrapping_sub(
                                                (PAGEHDRSZ as libc::c_uint).wrapping_sub(
                                                    if 0 as libc::c_int != 0 {
                                                        PAGEHDRSZ as libc::c_uint
                                                    } else {
                                                        0 as libc::c_int as libc::c_uint
                                                    },
                                                ),
                                            )
                                            >> 1 as libc::c_int)
                                            as libc::c_int;
                                        let mut current_block_285: u64;
                                        m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
                                        while !m2.is_null() {
                                            if (*mc).mc_flags & C_SUB as libc::c_uint != 0 {
                                                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                                            } else {
                                                m3 = m2;
                                            }
                                            if !(m3 == mc) {
                                                if !((*m2).mc_flags
                                                    & (*m3).mc_flags
                                                    & C_INITIALIZED as libc::c_uint
                                                    == 0)
                                                {
                                                    if new_root != 0 {
                                                        let mut k_0: libc::c_int = 0;
                                                        if (*m3).mc_pg[0 as libc::c_int as usize]
                                                            != mp
                                                        {
                                                            current_block_285 =
                                                                14303212209785889906;
                                                        } else {
                                                            k_0 = new_root;
                                                            while k_0 >= 0 as libc::c_int {
                                                                (*m3).mc_ki[(k_0 + 1 as libc::c_int)
                                                                    as usize] =
                                                                    (*m3).mc_ki[k_0 as usize];
                                                                (*m3).mc_pg[(k_0 + 1 as libc::c_int)
                                                                    as usize] =
                                                                    (*m3).mc_pg[k_0 as usize];
                                                                k_0 -= 1;
                                                                k_0;
                                                            }
                                                            if (*m3).mc_ki
                                                                [0 as libc::c_int as usize]
                                                                as libc::c_int
                                                                >= nkeys
                                                            {
                                                                (*m3).mc_ki
                                                                    [0 as libc::c_int as usize] =
                                                                    1 as libc::c_int as indx_t;
                                                            } else {
                                                                (*m3).mc_ki
                                                                    [0 as libc::c_int as usize] =
                                                                    0 as libc::c_int as indx_t;
                                                            }
                                                            (*m3).mc_pg
                                                                [0 as libc::c_int as usize] = (*mc)
                                                                .mc_pg
                                                                [0 as libc::c_int as usize];
                                                            (*m3).mc_snum =
                                                                ((*m3).mc_snum).wrapping_add(1);
                                                            (*m3).mc_snum;
                                                            (*m3).mc_top =
                                                                ((*m3).mc_top).wrapping_add(1);
                                                            (*m3).mc_top;
                                                            current_block_285 =
                                                                14723615986260991866;
                                                        }
                                                    } else {
                                                        current_block_285 = 14723615986260991866;
                                                    }
                                                    match current_block_285 {
                                                        14303212209785889906 => {}
                                                        _ => {
                                                            if (*m3).mc_top as libc::c_int
                                                                >= (*mc).mc_top as libc::c_int
                                                                && (*m3).mc_pg
                                                                    [(*mc).mc_top as usize]
                                                                    == mp
                                                            {
                                                                if (*m3).mc_ki
                                                                    [(*mc).mc_top as usize]
                                                                    as libc::c_int
                                                                    >= newindx as libc::c_int
                                                                    && nflags
                                                                        & MDB_SPLIT_REPLACE
                                                                            as libc::c_uint
                                                                        == 0
                                                                {
                                                                    (*m3).mc_ki
                                                                        [(*mc).mc_top as usize] =
                                                                        ((*m3).mc_ki[(*mc).mc_top
                                                                            as usize])
                                                                            .wrapping_add(1);
                                                                    (*m3).mc_ki
                                                                        [(*mc).mc_top as usize];
                                                                }
                                                                if (*m3).mc_ki
                                                                    [(*mc).mc_top as usize]
                                                                    as libc::c_int
                                                                    >= nkeys
                                                                {
                                                                    (*m3).mc_pg
                                                                        [(*mc).mc_top as usize] =
                                                                        rp;
                                                                    (*m3).mc_ki
                                                                        [(*mc).mc_top as usize] =
                                                                        ((*m3).mc_ki
                                                                            [(*mc).mc_top as usize]
                                                                            as libc::c_int
                                                                            - nkeys)
                                                                            as indx_t;
                                                                    i = 0 as libc::c_int;
                                                                    while i
                                                                        < (*mc).mc_top
                                                                            as libc::c_int
                                                                    {
                                                                        (*m3).mc_ki[i as usize] =
                                                                            mn.mc_ki[i as usize];
                                                                        (*m3).mc_pg[i as usize] =
                                                                            mn.mc_pg[i as usize];
                                                                        i += 1;
                                                                        i;
                                                                    }
                                                                }
                                                            } else if did_split == 0
                                                                && (*m3).mc_top as libc::c_int
                                                                    >= ptop
                                                                && (*m3).mc_pg[ptop as usize]
                                                                    == (*mc).mc_pg[ptop as usize]
                                                                && (*m3).mc_ki[ptop as usize]
                                                                    as libc::c_int
                                                                    >= (*mc).mc_ki[ptop as usize]
                                                                        as libc::c_int
                                                            {
                                                                (*m3).mc_ki[ptop as usize] =
                                                                    ((*m3).mc_ki[ptop as usize])
                                                                        .wrapping_add(1);
                                                                (*m3).mc_ki[ptop as usize];
                                                            }
                                                            if (*(mp as *mut libc::c_void
                                                                as *mut MDB_page2))
                                                                .mp2_flags
                                                                as libc::c_int
                                                                & 0x2 as libc::c_int
                                                                == 0x2 as libc::c_int
                                                            {
                                                                let mut xr_pg = (*m3).mc_pg
                                                                    [(*mc).mc_top as usize];
                                                                let mut xr_node =
                                                                    0 as *mut MDB_node;
                                                                if !(!(!((*m3).mc_xcursor).is_null()
                                                                    && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                                                        & C_INITIALIZED as libc::c_uint != 0)
                                                                    || (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                                                        >= ((*(xr_pg as *mut libc::c_void as *mut MDB_page2))
                                                                            .mp2_lower as libc::c_uint)
                                                                            .wrapping_sub(
                                                                                (PAGEHDRSZ as libc::c_uint)
                                                                                    .wrapping_sub(
                                                                                        if 0 as libc::c_int != 0 {
                                                                                            PAGEHDRSZ as libc::c_uint
                                                                                        } else {
                                                                                            0 as libc::c_int as libc::c_uint
                                                                                        },
                                                                                    ),
                                                                            ) >> 1 as libc::c_int)
                                                                {
                                                                    xr_node = (xr_pg as *mut libc::c_char)
                                                                        .offset(
                                                                            *((*(xr_pg as *mut libc::c_void as *mut MDB_page2))
                                                                                .mp2_ptrs)
                                                                                .as_mut_ptr()
                                                                                .offset((*m3).mc_ki[(*mc).mc_top as usize] as isize)
                                                                                as libc::c_int as isize,
                                                                        )
                                                                        .offset(
                                                                            (if 0 as libc::c_int != 0 {
                                                                                PAGEHDRSZ as libc::c_uint
                                                                            } else {
                                                                                0 as libc::c_int as libc::c_uint
                                                                            }) as isize,
                                                                        ) as *mut MDB_node;
                                                                    if (*xr_node).mn_flags as libc::c_int
                                                                        & (F_DUPDATA | F_SUBDATA) == F_DUPDATA
                                                                    {
                                                                        (*(*m3).mc_xcursor)
                                                                            .mx_cursor
                                                                            .mc_pg[0 as libc::c_int
                                                                            as usize] = ((*xr_node).mn_data)
                                                                            .as_mut_ptr()
                                                                            .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                                                            as *mut libc::c_void as *mut MDB_page;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            m2 = (*m2).mc_next;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !copy.is_null() {
        mdb_page_free(env, copy);
    }
    if rc != 0 {
        (*(*mc).mc_txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_put(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut mc = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut mx = MDB_xcursor {
        mx_cursor: MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        },
        mx_db: MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        },
        mx_dbx: MDB_dbx {
            md_name: MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            },
            md_cmp: None,
            md_dcmp: None,
            md_rel: None,
            md_relctx: 0 as *mut libc::c_void,
        },
        mx_dbflag: 0,
    };
    let mut rc: libc::c_int = 0;
    if key.is_null()
        || data.is_null()
        || !(!txn.is_null()
            && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int != 0)
    {
        return EINVAL;
    }
    if flags
        & !(MDB_NOOVERWRITE | MDB_NODUPDATA | MDB_RESERVE | MDB_APPEND | MDB_APPENDDUP)
            as libc::c_uint
        != 0
    {
        return EINVAL;
    }
    if (*txn).mt_flags & (MDB_TXN_RDONLY | MDB_TXN_BLOCKED) as libc::c_uint != 0 {
        return if (*txn).mt_flags & MDB_TXN_RDONLY as libc::c_uint != 0 {
            EACCES
        } else {
            MDB_BAD_TXN
        };
    }
    mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
    mc.mc_next = *((*txn).mt_cursors).offset(dbi as isize);
    let ref mut fresh72 = *((*txn).mt_cursors).offset(dbi as isize);
    *fresh72 = &mut mc;
    rc = _mdb_cursor_put(&mut mc, key, data, flags);
    let ref mut fresh73 = *((*txn).mt_cursors).offset(dbi as isize);
    *fresh73 = mc.mc_next;
    return rc;
}
pub const MDB_WBUF: libc::c_int = 1024 as libc::c_int * 1024 as libc::c_int;
pub const MDB_EOF: libc::c_int = 0x10 as libc::c_int;
#[cold]
unsafe extern "C" fn mdb_env_copythr(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut my = arg as *mut mdb_copy;
    let mut ptr = 0 as *mut libc::c_char;
    let mut toggle = 0 as libc::c_int;
    let mut wsize: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut set: sigset_t = 0;
    set = 0 as libc::c_int as sigset_t;
    set |= __sigbits(13 as libc::c_int) as sigset_t;
    rc = pthread_sigmask(
        SIG_BLOCK,
        &mut set as *mut sigset_t as *const sigset_t,
        NULL as *mut sigset_t,
    );
    if rc != 0 as libc::c_int {
        ::core::ptr::write_volatile(&mut (*my).mc_error as *mut libc::c_int, rc);
    }
    pthread_mutex_lock(&mut (*my).mc_mutex);
    loop {
        while (*my).mc_new == 0 {
            pthread_cond_wait(&mut (*my).mc_cond, &mut (*my).mc_mutex);
        }
        if (*my).mc_new == 0 as libc::c_int + MDB_EOF {
            break;
        }
        wsize = (*my).mc_wlen[toggle as usize];
        ptr = (*my).mc_wbuf[toggle as usize];
        loop {
            rc = MDB_SUCCESS;
            while wsize > 0 as libc::c_int && (*my).mc_error == 0 {
                len =
                    write((*my).mc_fd, ptr as *const libc::c_void, wsize as size_t) as libc::c_int;
                rc = (len >= 0 as libc::c_int) as libc::c_int;
                if rc == 0 {
                    rc = *__error();
                    if rc == EPIPE {
                        let mut tmp: libc::c_int = 0;
                        sigwait(&mut set, &mut tmp);
                    }
                    break;
                } else if len > 0 as libc::c_int {
                    rc = MDB_SUCCESS;
                    ptr = ptr.offset(len as isize);
                    wsize -= len;
                } else {
                    rc = EIO;
                    break;
                }
            }
            if rc != 0 {
                ::core::ptr::write_volatile(&mut (*my).mc_error as *mut libc::c_int, rc);
            }
            if !((*my).mc_olen[toggle as usize] != 0) {
                break;
            }
            wsize = (*my).mc_olen[toggle as usize];
            ptr = (*my).mc_over[toggle as usize];
            (*my).mc_olen[toggle as usize] = 0 as libc::c_int;
        }
        (*my).mc_wlen[toggle as usize] = 0 as libc::c_int;
        toggle ^= 1 as libc::c_int;
        (*my).mc_new -= 1;
        (*my).mc_new;
        pthread_cond_signal(&mut (*my).mc_cond);
    }
    pthread_mutex_unlock(&mut (*my).mc_mutex);
    return 0 as *mut libc::c_void;
}
#[cold]
unsafe extern "C" fn mdb_env_cthr_toggle(
    mut my: *mut mdb_copy,
    mut adjust: libc::c_int,
) -> libc::c_int {
    pthread_mutex_lock(&mut (*my).mc_mutex);
    (*my).mc_new += adjust;
    pthread_cond_signal(&mut (*my).mc_cond);
    while (*my).mc_new & 2 as libc::c_int != 0 {
        pthread_cond_wait(&mut (*my).mc_cond, &mut (*my).mc_mutex);
    }
    pthread_mutex_unlock(&mut (*my).mc_mutex);
    (*my).mc_toggle ^= adjust & 1 as libc::c_int;
    (*my).mc_wlen[(*my).mc_toggle as usize] = 0 as libc::c_int;
    return (*my).mc_error;
}
#[cold]
unsafe extern "C" fn mdb_env_cwalk(
    mut my: *mut mdb_copy,
    mut pg: *mut pgno_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut mc = {
        let mut init = MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        };
        init
    };
    let mut ni = 0 as *mut MDB_node;
    let mut mo = 0 as *mut MDB_page;
    let mut mp = 0 as *mut MDB_page;
    let mut leaf = 0 as *mut MDB_page;
    let mut buf = 0 as *mut libc::c_char;
    let mut ptr = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0;
    let mut toggle: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    if *pg == P_INVALID {
        return MDB_SUCCESS;
    }
    mc.mc_snum = 1 as libc::c_int as libc::c_ushort;
    mc.mc_txn = (*my).mc_txn;
    mc.mc_flags = (*(*my).mc_txn).mt_flags & (C_ORIG_RDONLY | C_WRITEMAP) as libc::c_uint;
    rc = mdb_page_get(
        &mut mc,
        *pg,
        &mut *(mc.mc_pg).as_mut_ptr().offset(0 as libc::c_int as isize),
        NULL as *mut libc::c_int,
    );
    if rc != 0 {
        return rc;
    }
    rc = mdb_page_search_root(&mut mc, NULL as *mut MDB_val, MDB_PS_FIRST);
    if rc != 0 {
        return rc;
    }
    ptr = malloc(
        ((*(*my).mc_env).me_psize).wrapping_mul(mc.mc_snum as libc::c_uint) as libc::c_ulong,
    ) as *mut libc::c_char;
    buf = ptr;
    if buf.is_null() {
        return ENOMEM;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < mc.mc_top as libc::c_uint {
        mdb_page_copy(
            ptr as *mut MDB_page,
            mc.mc_pg[i as usize],
            (*(*my).mc_env).me_psize,
        );
        mc.mc_pg[i as usize] = ptr as *mut MDB_page;
        ptr = ptr.offset((*(*my).mc_env).me_psize as isize);
        i = i.wrapping_add(1);
        i;
    }
    leaf = ptr as *mut MDB_page;
    toggle = (*my).mc_toggle;
    's_89: while mc.mc_snum as libc::c_int > 0 as libc::c_int {
        let mut n: libc::c_uint = 0;
        mp = mc.mc_pg[mc.mc_top as usize];
        n = ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
            .wrapping_sub(
                (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }),
            )
            >> 1 as libc::c_int;
        if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
            & 0x2 as libc::c_int
            == 0x2 as libc::c_int
        {
            if !((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                & 0x20 as libc::c_int
                == 0x20 as libc::c_int)
                && flags & F_DUPDATA == 0
            {
                i = 0 as libc::c_int as libc::c_uint;
                while i < n {
                    ni = (mp as *mut libc::c_char)
                        .offset(
                            *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                .as_mut_ptr()
                                .offset(i as isize) as libc::c_int
                                as isize,
                        )
                        .offset(
                            (if 0 as libc::c_int != 0 {
                                PAGEHDRSZ as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    if (*ni).mn_flags as libc::c_int & F_BIGDATA != 0 {
                        let mut omp = 0 as *mut MDB_page;
                        let mut pg_0: pgno_t = 0;
                        if mp != leaf {
                            mc.mc_pg[mc.mc_top as usize] = leaf;
                            mdb_page_copy(leaf, mp, (*(*my).mc_env).me_psize);
                            mp = leaf;
                            ni = (mp as *mut libc::c_char)
                                .offset(
                                    *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                        .as_mut_ptr()
                                        .offset(i as isize)
                                        as libc::c_int as isize,
                                )
                                .offset(
                                    (if 0 as libc::c_int != 0 {
                                        PAGEHDRSZ as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                        }
                        memcpy(
                            &mut pg_0 as *mut pgno_t as *mut libc::c_void,
                            ((*ni).mn_data)
                                .as_mut_ptr()
                                .offset((*ni).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void,
                            ::core::mem::size_of::<pgno_t>() as libc::c_ulong,
                        );
                        memcpy(
                            ((*ni).mn_data)
                                .as_mut_ptr()
                                .offset((*ni).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void,
                            &mut (*my).mc_next_pgno as *mut pgno_t as *const libc::c_void,
                            ::core::mem::size_of::<pgno_t>() as libc::c_ulong,
                        );
                        rc = mdb_page_get(&mut mc, pg_0, &mut omp, NULL as *mut libc::c_int);
                        if rc != 0 {
                            break 's_89;
                        }
                        if (*my).mc_wlen[toggle as usize] >= MDB_WBUF {
                            rc = mdb_env_cthr_toggle(my, 1 as libc::c_int);
                            if rc != 0 {
                                break 's_89;
                            }
                            toggle = (*my).mc_toggle;
                        }
                        mo = ((*my).mc_wbuf[toggle as usize])
                            .offset((*my).mc_wlen[toggle as usize] as isize)
                            as *mut MDB_page;
                        memcpy(
                            mo as *mut libc::c_void,
                            omp as *const libc::c_void,
                            (*(*my).mc_env).me_psize as libc::c_ulong,
                        );
                        (*mo).mp_p.p_pgno = (*my).mc_next_pgno;
                        (*my).mc_next_pgno =
                            ((*my).mc_next_pgno).wrapping_add((*omp).mp_pb.pb_pages as pgno_t);
                        (*my).mc_wlen[toggle as usize] =
                            ((*my).mc_wlen[toggle as usize] as libc::c_uint)
                                .wrapping_add((*(*my).mc_env).me_psize)
                                as libc::c_int as libc::c_int;
                        if (*omp).mp_pb.pb_pages > 1 as libc::c_int as uint32_t {
                            (*my).mc_olen[toggle as usize] =
                                ((*(*my).mc_env).me_psize).wrapping_mul(
                                    ((*omp).mp_pb.pb_pages)
                                        .wrapping_sub(1 as libc::c_int as uint32_t),
                                ) as libc::c_int;
                            (*my).mc_over[toggle as usize] = (omp as *mut libc::c_char)
                                .offset((*(*my).mc_env).me_psize as isize);
                            rc = mdb_env_cthr_toggle(my, 1 as libc::c_int);
                            if rc != 0 {
                                break 's_89;
                            }
                            toggle = (*my).mc_toggle;
                        }
                    } else if (*ni).mn_flags as libc::c_int & F_SUBDATA != 0 {
                        let mut db = MDB_db {
                            md_pad: 0,
                            md_flags: 0,
                            md_depth: 0,
                            md_branch_pages: 0,
                            md_leaf_pages: 0,
                            md_overflow_pages: 0,
                            md_entries: 0,
                            md_root: 0,
                        };
                        if mp != leaf {
                            mc.mc_pg[mc.mc_top as usize] = leaf;
                            mdb_page_copy(leaf, mp, (*(*my).mc_env).me_psize);
                            mp = leaf;
                            ni = (mp as *mut libc::c_char)
                                .offset(
                                    *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                        .as_mut_ptr()
                                        .offset(i as isize)
                                        as libc::c_int as isize,
                                )
                                .offset(
                                    (if 0 as libc::c_int != 0 {
                                        PAGEHDRSZ as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                        }
                        memcpy(
                            &mut db as *mut MDB_db as *mut libc::c_void,
                            ((*ni).mn_data)
                                .as_mut_ptr()
                                .offset((*ni).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void,
                            ::core::mem::size_of::<MDB_db>() as libc::c_ulong,
                        );
                        (*my).mc_toggle = toggle;
                        rc = mdb_env_cwalk(
                            my,
                            &mut db.md_root,
                            (*ni).mn_flags as libc::c_int & F_DUPDATA,
                        );
                        if rc != 0 {
                            break 's_89;
                        }
                        toggle = (*my).mc_toggle;
                        memcpy(
                            ((*ni).mn_data)
                                .as_mut_ptr()
                                .offset((*ni).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void,
                            &mut db as *mut MDB_db as *const libc::c_void,
                            ::core::mem::size_of::<MDB_db>() as libc::c_ulong,
                        );
                    }
                    i = i.wrapping_add(1);
                    i;
                }
            }
        } else {
            mc.mc_ki[mc.mc_top as usize] = (mc.mc_ki[mc.mc_top as usize]).wrapping_add(1);
            mc.mc_ki[mc.mc_top as usize];
            if (mc.mc_ki[mc.mc_top as usize] as libc::c_uint) < n {
                let mut pg_1: pgno_t = 0;
                loop {
                    ni = (mp as *mut libc::c_char)
                        .offset(
                            *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                .as_mut_ptr()
                                .offset(mc.mc_ki[mc.mc_top as usize] as isize)
                                as libc::c_int as isize,
                        )
                        .offset(
                            (if 0 as libc::c_int != 0 {
                                PAGEHDRSZ as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    pg_1 = (*ni).mn_lo as pgno_t
                        | ((*ni).mn_hi as pgno_t) << 16 as libc::c_int
                        | (if (if -(1 as libc::c_int) as pgno_t
                            > 0xffffffff as libc::c_uint as pgno_t
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) != 0
                        {
                            ((*ni).mn_flags as pgno_t)
                                << (if -(1 as libc::c_int) as pgno_t
                                    > 0xffffffff as libc::c_uint as pgno_t
                                {
                                    32 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                        } else {
                            0 as libc::c_int as pgno_t
                        });
                    rc = mdb_page_get(&mut mc, pg_1, &mut mp, NULL as *mut libc::c_int);
                    if rc != 0 {
                        break 's_89;
                    }
                    mc.mc_top = (mc.mc_top).wrapping_add(1);
                    mc.mc_top;
                    mc.mc_snum = (mc.mc_snum).wrapping_add(1);
                    mc.mc_snum;
                    mc.mc_ki[mc.mc_top as usize] = 0 as libc::c_int as indx_t;
                    if !((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                        & 0x1 as libc::c_int
                        == 0x1 as libc::c_int)
                    {
                        break;
                    }
                    mdb_page_copy(mc.mc_pg[mc.mc_top as usize], mp, (*(*my).mc_env).me_psize);
                }
                mc.mc_pg[mc.mc_top as usize] = mp;
                continue;
            }
        }
        if (*my).mc_wlen[toggle as usize] >= MDB_WBUF {
            rc = mdb_env_cthr_toggle(my, 1 as libc::c_int);
            if rc != 0 {
                break;
            }
            toggle = (*my).mc_toggle;
        }
        mo = ((*my).mc_wbuf[toggle as usize]).offset((*my).mc_wlen[toggle as usize] as isize)
            as *mut MDB_page;
        mdb_page_copy(mo, mp, (*(*my).mc_env).me_psize);
        let fresh74 = (*my).mc_next_pgno;
        (*my).mc_next_pgno = ((*my).mc_next_pgno).wrapping_add(1);
        (*mo).mp_p.p_pgno = fresh74;
        (*my).mc_wlen[toggle as usize] = ((*my).mc_wlen[toggle as usize] as libc::c_uint)
            .wrapping_add((*(*my).mc_env).me_psize)
            as libc::c_int as libc::c_int;
        if mc.mc_top != 0 {
            ni = (mc.mc_pg[(mc.mc_top as libc::c_int - 1 as libc::c_int) as usize]
                as *mut libc::c_char)
                .offset(
                    *((*(mc.mc_pg[(mc.mc_top as libc::c_int - 1 as libc::c_int) as usize]
                        as *mut libc::c_void as *mut MDB_page2))
                        .mp2_ptrs)
                        .as_mut_ptr()
                        .offset(
                            mc.mc_ki[(mc.mc_top as libc::c_int - 1 as libc::c_int) as usize]
                                as isize,
                        ) as libc::c_int as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            (*ni).mn_lo = ((*mo).mp_p.p_pgno & 0xffff as libc::c_int as pgno_t) as libc::c_ushort;
            (*ni).mn_hi = ((*mo).mp_p.p_pgno >> 16 as libc::c_int) as libc::c_ushort;
            if if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                32 as libc::c_int
            } else {
                0 as libc::c_int
            } != 0
            {
                (*ni).mn_flags = ((*mo).mp_p.p_pgno
                    >> (if -(1 as libc::c_int) as pgno_t > 0xffffffff as libc::c_uint as pgno_t {
                        32 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })) as libc::c_ushort;
            }
            mdb_cursor_pop(&mut mc);
        } else {
            *pg = (*mo).mp_p.p_pgno;
            break;
        }
    }
    free(buf as *mut libc::c_void);
    return rc;
}
#[cold]
unsafe extern "C" fn mdb_env_copyfd1(mut env: *mut MDB_env, mut fd: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut mm = 0 as *mut MDB_meta;
    let mut mp = 0 as *mut MDB_page;
    let mut my = {
        let mut init = mdb_copy {
            mc_env: 0 as *mut MDB_env,
            mc_txn: 0 as *mut MDB_txn,
            mc_mutex: _opaque_pthread_mutex_t {
                __sig: 0,
                __opaque: [0; 56],
            },
            mc_cond: _opaque_pthread_cond_t {
                __sig: 0,
                __opaque: [0; 40],
            },
            mc_wbuf: [0 as *mut libc::c_char; 2],
            mc_over: [0 as *mut libc::c_char; 2],
            mc_wlen: [0; 2],
            mc_olen: [0; 2],
            mc_next_pgno: 0,
            mc_fd: 0,
            mc_toggle: 0,
            mc_new: 0,
            mc_error: 0,
        };
        init
    };
    let mut txn = NULL as *mut MDB_txn;
    let mut thr = 0 as *mut _opaque_pthread_t;
    let mut root: pgno_t = 0;
    let mut new_root: pgno_t = 0;
    let mut rc = MDB_SUCCESS;
    rc = pthread_mutex_init(&mut my.mc_mutex, NULL as *const pthread_mutexattr_t);
    if rc != 0 as libc::c_int {
        return rc;
    }
    rc = pthread_cond_init(&mut my.mc_cond, NULL as *const pthread_condattr_t);
    if !(rc != 0 as libc::c_int) {
        let mut p = 0 as *mut libc::c_void;
        rc = posix_memalign(
            &mut p,
            (*env).me_os_psize as size_t,
            (MDB_WBUF * 2 as libc::c_int) as size_t,
        );
        if !(rc != 0 as libc::c_int) {
            my.mc_wbuf[0 as libc::c_int as usize] = p as *mut libc::c_char;
            memset(
                my.mc_wbuf[0 as libc::c_int as usize] as *mut libc::c_void,
                0 as libc::c_int,
                (MDB_WBUF * 2 as libc::c_int) as libc::c_ulong,
            );
            my.mc_wbuf[1 as libc::c_int as usize] =
                (my.mc_wbuf[0 as libc::c_int as usize]).offset(MDB_WBUF as isize);
            my.mc_next_pgno = NUM_METAS as pgno_t;
            my.mc_env = env;
            my.mc_fd = fd;
            rc = pthread_create(
                &mut thr,
                NULL as *const pthread_attr_t,
                Some(
                    mdb_env_copythr as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                &mut my as *mut mdb_copy as *mut libc::c_void,
            );
            if !(rc != 0) {
                rc = mdb_txn_begin(
                    env,
                    NULL as *mut MDB_txn,
                    MDB_RDONLY as libc::c_uint,
                    &mut txn,
                );
                if !(rc != 0) {
                    mp = my.mc_wbuf[0 as libc::c_int as usize] as *mut MDB_page;
                    memset(
                        mp as *mut libc::c_void,
                        0 as libc::c_int,
                        (NUM_METAS as libc::c_uint).wrapping_mul((*env).me_psize) as libc::c_ulong,
                    );
                    (*mp).mp_p.p_pgno = 0 as libc::c_int as pgno_t;
                    (*mp).mp_flags = P_META as uint16_t;
                    mm = (mp as *mut libc::c_char).offset(PAGEHDRSZ as libc::c_uint as isize)
                        as *mut libc::c_void as *mut MDB_meta;
                    mdb_env_init_meta0(env, mm);
                    (*mm).mm_address = (*(*env).me_metas[0 as libc::c_int as usize]).mm_address;
                    mp = (my.mc_wbuf[0 as libc::c_int as usize]).offset((*env).me_psize as isize)
                        as *mut MDB_page;
                    (*mp).mp_p.p_pgno = 1 as libc::c_int as pgno_t;
                    (*mp).mp_flags = P_META as uint16_t;
                    *((mp as *mut libc::c_char).offset(PAGEHDRSZ as libc::c_uint as isize)
                        as *mut libc::c_void as *mut MDB_meta) = *mm;
                    mm = (mp as *mut libc::c_char).offset(PAGEHDRSZ as libc::c_uint as isize)
                        as *mut libc::c_void as *mut MDB_meta;
                    new_root = (*((*txn).mt_dbs).offset(MAIN_DBI as isize)).md_root;
                    root = new_root;
                    if root != P_INVALID {
                        let mut freecount = 0 as libc::c_int as MDB_ID;
                        let mut mc = MDB_cursor {
                            mc_next: 0 as *mut MDB_cursor,
                            mc_backup: 0 as *mut MDB_cursor,
                            mc_xcursor: 0 as *mut MDB_xcursor,
                            mc_txn: 0 as *mut MDB_txn,
                            mc_dbi: 0,
                            mc_db: 0 as *mut MDB_db,
                            mc_dbx: 0 as *mut MDB_dbx,
                            mc_dbflag: 0 as *mut libc::c_uchar,
                            mc_snum: 0,
                            mc_top: 0,
                            mc_flags: 0,
                            mc_pg: [0 as *mut MDB_page; 32],
                            mc_ki: [0; 32],
                        };
                        let mut key = MDB_val {
                            mv_size: 0,
                            mv_data: 0 as *mut libc::c_void,
                        };
                        let mut data = MDB_val {
                            mv_size: 0,
                            mv_data: 0 as *mut libc::c_void,
                        };
                        mdb_cursor_init(
                            &mut mc,
                            txn,
                            FREE_DBI as MDB_dbi,
                            NULL as *mut MDB_xcursor,
                        );
                        loop {
                            rc = mdb_cursor_get(&mut mc, &mut key, &mut data, MDB_NEXT);
                            if !(rc == 0 as libc::c_int) {
                                break;
                            }
                            freecount = freecount.wrapping_add(*(data.mv_data as *mut MDB_ID));
                        }
                        if rc != MDB_NOTFOUND {
                            current_block = 4695762913923507500;
                        } else {
                            freecount = freecount.wrapping_add(
                                ((*((*txn).mt_dbs).offset(FREE_DBI as isize)).md_branch_pages)
                                    .wrapping_add(
                                        (*((*txn).mt_dbs).offset(FREE_DBI as isize)).md_leaf_pages,
                                    )
                                    .wrapping_add(
                                        (*((*txn).mt_dbs).offset(FREE_DBI as isize))
                                            .md_overflow_pages,
                                    ),
                            );
                            new_root = ((*txn).mt_next_pgno)
                                .wrapping_sub(1 as libc::c_int as pgno_t)
                                .wrapping_sub(freecount);
                            (*mm).mm_last_pg = new_root;
                            (*mm).mm_dbs[MAIN_DBI as usize] =
                                *((*txn).mt_dbs).offset(MAIN_DBI as isize);
                            (*mm).mm_dbs[MAIN_DBI as usize].md_root = new_root;
                            current_block = 652864300344834934;
                        }
                    } else {
                        (*mm).mm_dbs[MAIN_DBI as usize].md_flags =
                            (*((*txn).mt_dbs).offset(MAIN_DBI as isize)).md_flags;
                        current_block = 652864300344834934;
                    }
                    match current_block {
                        4695762913923507500 => {}
                        _ => {
                            if root != P_INVALID
                                || (*mm).mm_dbs[MAIN_DBI as usize].md_flags as libc::c_int != 0
                            {
                                ::core::ptr::write_volatile(
                                    &mut (*mm).mm_txnid as *mut txnid_t,
                                    1 as libc::c_int as txnid_t,
                                );
                            }
                            my.mc_wlen[0 as libc::c_int as usize] = ((*env).me_psize)
                                .wrapping_mul(NUM_METAS as libc::c_uint)
                                as libc::c_int;
                            my.mc_txn = txn;
                            rc = mdb_env_cwalk(&mut my, &mut root, 0 as libc::c_int);
                            if rc == MDB_SUCCESS && root != new_root {
                                rc = MDB_INCOMPATIBLE;
                            }
                        }
                    }
                }
                if rc != 0 {
                    ::core::ptr::write_volatile(&mut my.mc_error as *mut libc::c_int, rc);
                }
                mdb_env_cthr_toggle(&mut my, 1 as libc::c_int | MDB_EOF);
                rc = pthread_join(thr, NULL as *mut *mut libc::c_void);
                _mdb_txn_abort(txn);
            }
        }
        free(my.mc_wbuf[0 as libc::c_int as usize] as *mut libc::c_void);
        pthread_cond_destroy(&mut my.mc_cond);
    }
    pthread_mutex_destroy(&mut my.mc_mutex);
    return if rc != 0 { rc } else { my.mc_error };
}
#[cold]
unsafe extern "C" fn mdb_env_copyfd0(mut env: *mut MDB_env, mut fd: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut txn = NULL as *mut MDB_txn;
    let mut wmutex = NULL as mdb_mutexref_t;
    let mut rc: libc::c_int = 0;
    let mut wsize: mdb_size_t = 0;
    let mut w3: mdb_size_t = 0;
    let mut ptr = 0 as *mut libc::c_char;
    let mut len: ssize_t = 0;
    let mut w2: size_t = 0;
    rc = mdb_txn_begin(
        env,
        NULL as *mut MDB_txn,
        MDB_RDONLY as libc::c_uint,
        &mut txn,
    );
    if rc != 0 {
        return rc;
    }
    if !((*env).me_txns).is_null() {
        mdb_txn_end(txn, MDB_END_RESET_TMP as libc::c_int as libc::c_uint);
        wmutex = ((*env).me_wmutex).as_mut_ptr();
        rc = mdb_sem_wait(wmutex);
        if rc != 0 && {
            rc = mdb_mutex_failed(env, wmutex, rc);
            rc != 0
        } {
            current_block = 2682935144166745456;
        } else {
            rc = mdb_txn_renew0(txn);
            if rc != 0 {
                let mut sb = {
                    let mut init = sembuf {
                        sem_num: 0 as libc::c_int as libc::c_ushort,
                        sem_op: 1 as libc::c_int as libc::c_short,
                        sem_flg: SEM_UNDO as libc::c_short,
                    };
                    init
                };
                sb.sem_num = (*wmutex).semnum as libc::c_ushort;
                *(*wmutex).locked = 0 as libc::c_int;
                semop((*wmutex).semid, &mut sb, 1 as libc::c_int as size_t);
                current_block = 2682935144166745456;
            } else {
                current_block = 12349973810996921269;
            }
        }
    } else {
        current_block = 12349973810996921269;
    }
    match current_block {
        12349973810996921269 => {
            wsize = ((*env).me_psize).wrapping_mul(NUM_METAS as libc::c_uint) as mdb_size_t;
            ptr = (*env).me_map;
            w2 = wsize;
            while w2 > 0 as libc::c_int as size_t {
                len = write(fd, ptr as *const libc::c_void, w2);
                rc = (len >= 0 as libc::c_int as ssize_t) as libc::c_int;
                if rc == 0 {
                    rc = *__error();
                    break;
                } else if len > 0 as libc::c_int as ssize_t {
                    rc = MDB_SUCCESS;
                    ptr = ptr.offset(len as isize);
                    w2 = w2.wrapping_sub(len as size_t);
                } else {
                    rc = EIO;
                    break;
                }
            }
            if !wmutex.is_null() {
                let mut sb_0 = {
                    let mut init = sembuf {
                        sem_num: 0 as libc::c_int as libc::c_ushort,
                        sem_op: 1 as libc::c_int as libc::c_short,
                        sem_flg: SEM_UNDO as libc::c_short,
                    };
                    init
                };
                sb_0.sem_num = (*wmutex).semnum as libc::c_ushort;
                *(*wmutex).locked = 0 as libc::c_int;
                semop((*wmutex).semid, &mut sb_0, 1 as libc::c_int as size_t);
            }
            if !(rc != 0) {
                w3 = (*txn).mt_next_pgno * (*env).me_psize as pgno_t;
                let mut fsize = 0 as libc::c_int as mdb_size_t;
                rc = mdb_fsize((*env).me_fd, &mut fsize);
                if !(rc != 0) {
                    if w3 > fsize {
                        w3 = fsize;
                    }
                    wsize = w3.wrapping_sub(wsize);
                    while wsize > 0 as libc::c_int as mdb_size_t {
                        if wsize > MAX_WRITE as mdb_size_t {
                            w2 = MAX_WRITE as size_t;
                        } else {
                            w2 = wsize;
                        }
                        len = write(fd, ptr as *const libc::c_void, w2);
                        rc = (len >= 0 as libc::c_int as ssize_t) as libc::c_int;
                        if rc == 0 {
                            rc = *__error();
                            break;
                        } else if len > 0 as libc::c_int as ssize_t {
                            rc = MDB_SUCCESS;
                            ptr = ptr.offset(len as isize);
                            wsize = wsize.wrapping_sub(len as mdb_size_t);
                        } else {
                            rc = EIO;
                            break;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    _mdb_txn_abort(txn);
    return rc;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_copyfd2(
    mut env: *mut MDB_env,
    mut fd: libc::c_int,
    mut flags: libc::c_uint,
) -> libc::c_int {
    if flags & MDB_CP_COMPACT as libc::c_uint != 0 {
        return mdb_env_copyfd1(env, fd);
    } else {
        return mdb_env_copyfd0(env, fd);
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_copyfd(mut env: *mut MDB_env, mut fd: libc::c_int) -> libc::c_int {
    return mdb_env_copyfd2(env, fd, 0 as libc::c_int as libc::c_uint);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_copy2(
    mut env: *mut MDB_env,
    mut path: *const libc::c_char,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut fname = MDB_name {
        mn_len: 0,
        mn_alloced: 0,
        mn_val: 0 as *mut mdb_nchar_t,
    };
    let mut newfd = INVALID_HANDLE_VALUE;
    rc = mdb_fname_init(path, (*env).me_flags | MDB_NOLOCK as uint32_t, &mut fname);
    if rc == MDB_SUCCESS {
        rc = mdb_fopen(
            env,
            &mut fname,
            MDB_O_COPY,
            0o666 as libc::c_int as mdb_mode_t,
            &mut newfd,
        );
        if fname.mn_alloced != 0 {
            free(fname.mn_val as *mut libc::c_void);
        }
    }
    if rc == MDB_SUCCESS {
        rc = mdb_env_copyfd2(env, newfd, flags);
        if close(newfd) < 0 as libc::c_int && rc == MDB_SUCCESS {
            rc = *__error();
        }
    }
    return rc;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_copy(
    mut env: *mut MDB_env,
    mut path: *const libc::c_char,
) -> libc::c_int {
    return mdb_env_copy2(env, path, 0 as libc::c_int as libc::c_uint);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_set_flags(
    mut env: *mut MDB_env,
    mut flag: libc::c_uint,
    mut onoff: libc::c_int,
) -> libc::c_int {
    if flag & !CHANGEABLE as libc::c_uint != 0 {
        return EINVAL;
    }
    if onoff != 0 {
        (*env).me_flags |= flag;
    } else {
        (*env).me_flags &= !flag;
    }
    return MDB_SUCCESS;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_get_flags(
    mut env: *mut MDB_env,
    mut arg: *mut libc::c_uint,
) -> libc::c_int {
    if env.is_null() || arg.is_null() {
        return EINVAL;
    }
    *arg = (*env).me_flags & (CHANGEABLE | CHANGELESS) as uint32_t;
    return MDB_SUCCESS;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_set_userctx(
    mut env: *mut MDB_env,
    mut ctx: *mut libc::c_void,
) -> libc::c_int {
    if env.is_null() {
        return EINVAL;
    }
    (*env).me_userctx = ctx;
    return MDB_SUCCESS;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_get_userctx(mut env: *mut MDB_env) -> *mut libc::c_void {
    return if !env.is_null() {
        (*env).me_userctx
    } else {
        NULL as *mut libc::c_void
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_set_assert(
    mut env: *mut MDB_env,
    mut func: Option<MDB_assert_func>,
) -> libc::c_int {
    if env.is_null() {
        return EINVAL;
    }
    (*env).me_assert_func = func;
    return MDB_SUCCESS;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_get_path(
    mut env: *mut MDB_env,
    mut arg: *mut *const libc::c_char,
) -> libc::c_int {
    if env.is_null() || arg.is_null() {
        return EINVAL;
    }
    *arg = (*env).me_path;
    return MDB_SUCCESS;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_get_fd(
    mut env: *mut MDB_env,
    mut arg: *mut mdb_filehandle_t,
) -> libc::c_int {
    if env.is_null() || arg.is_null() {
        return EINVAL;
    }
    *arg = (*env).me_fd;
    return MDB_SUCCESS;
}
#[cold]
unsafe extern "C" fn mdb_stat0(
    mut env: *mut MDB_env,
    mut db: *mut MDB_db,
    mut arg: *mut MDB_stat,
) -> libc::c_int {
    (*arg).ms_psize = (*env).me_psize;
    (*arg).ms_depth = (*db).md_depth as libc::c_uint;
    (*arg).ms_branch_pages = (*db).md_branch_pages;
    (*arg).ms_leaf_pages = (*db).md_leaf_pages;
    (*arg).ms_overflow_pages = (*db).md_overflow_pages;
    (*arg).ms_entries = (*db).md_entries;
    return MDB_SUCCESS;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_stat(
    mut env: *mut MDB_env,
    mut arg: *mut MDB_stat,
) -> libc::c_int {
    let mut meta = 0 as *mut MDB_meta;
    if env.is_null() || arg.is_null() {
        return EINVAL;
    }
    meta = mdb_env_pick_meta(env);
    return mdb_stat0(
        env,
        &mut *((*meta).mm_dbs).as_mut_ptr().offset(MAIN_DBI as isize),
        arg,
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_info(
    mut env: *mut MDB_env,
    mut arg: *mut MDB_envinfo,
) -> libc::c_int {
    let mut meta = 0 as *mut MDB_meta;
    if env.is_null() || arg.is_null() {
        return EINVAL;
    }
    meta = mdb_env_pick_meta(env);
    (*arg).me_mapaddr = (*meta).mm_address;
    (*arg).me_last_pgno = (*meta).mm_last_pg;
    (*arg).me_last_txnid = (*meta).mm_txnid;
    (*arg).me_mapsize = (*env).me_mapsize;
    (*arg).me_maxreaders = (*env).me_maxreaders;
    (*arg).me_numreaders = if !((*env).me_txns).is_null() {
        (*(*env).me_txns).mt1.mtb.mtb_numreaders
    } else {
        0 as libc::c_int as libc::c_uint
    };
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_default_cmp(mut txn: *mut MDB_txn, mut dbi: MDB_dbi) {
    let mut f = (*((*txn).mt_dbs).offset(dbi as isize)).md_flags;
    let ref mut fresh75 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_cmp;
    *fresh75 = if f as libc::c_int & MDB_REVERSEKEY != 0 {
        Some(mdb_cmp_memnr as MDB_cmp_func)
    } else if f as libc::c_int & MDB_INTEGERKEY != 0 {
        Some(mdb_cmp_cint as MDB_cmp_func)
    } else {
        Some(mdb_cmp_memn as MDB_cmp_func)
    };
    let ref mut fresh76 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_dcmp;
    *fresh76 = if f as libc::c_int & MDB_DUPSORT == 0 {
        None
    } else if f as libc::c_int & MDB_INTEGERDUP != 0 {
        if f as libc::c_int & MDB_DUPFIXED != 0 {
            Some(mdb_cmp_int as MDB_cmp_func)
        } else {
            Some(mdb_cmp_cint as MDB_cmp_func)
        }
    } else if f as libc::c_int & MDB_REVERSEDUP != 0 {
        Some(mdb_cmp_memnr as MDB_cmp_func)
    } else {
        Some(mdb_cmp_memn as MDB_cmp_func)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mdb_dbi_open(
    mut txn: *mut MDB_txn,
    mut name: *const libc::c_char,
    mut flags: libc::c_uint,
    mut dbi: *mut MDB_dbi,
) -> libc::c_int {
    let mut key = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut i: MDB_dbi = 0;
    let mut mc = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut dummy = MDB_db {
        md_pad: 0,
        md_flags: 0,
        md_depth: 0,
        md_branch_pages: 0,
        md_leaf_pages: 0,
        md_overflow_pages: 0,
        md_entries: 0,
        md_root: 0,
    };
    let mut rc: libc::c_int = 0;
    let mut dbflag: libc::c_int = 0;
    let mut exact: libc::c_int = 0;
    let mut unused = 0 as libc::c_int as libc::c_uint;
    let mut seq: libc::c_uint = 0;
    let mut namedup = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if flags & !VALID_FLAGS as libc::c_uint != 0 {
        return EINVAL;
    }
    if (*txn).mt_flags & MDB_TXN_BLOCKED as libc::c_uint != 0 {
        return MDB_BAD_TXN;
    }
    if name.is_null() {
        *dbi = MAIN_DBI as MDB_dbi;
        if flags & PERSISTENT_FLAGS as libc::c_uint != 0 {
            let mut f2 = (flags & PERSISTENT_FLAGS as libc::c_uint) as uint16_t;
            if (*((*txn).mt_dbs).offset(MAIN_DBI as isize)).md_flags as libc::c_int
                | f2 as libc::c_int
                != (*((*txn).mt_dbs).offset(MAIN_DBI as isize)).md_flags as libc::c_int
            {
                let ref mut fresh77 = (*((*txn).mt_dbs).offset(MAIN_DBI as isize)).md_flags;
                *fresh77 = (*fresh77 as libc::c_int | f2 as libc::c_int) as uint16_t;
                (*txn).mt_flags |= MDB_TXN_DIRTY as libc::c_uint;
            }
        }
        mdb_default_cmp(txn, MAIN_DBI as MDB_dbi);
        return MDB_SUCCESS;
    }
    if ((*((*txn).mt_dbxs).offset(MAIN_DBI as isize)).md_cmp).is_none() {
        mdb_default_cmp(txn, MAIN_DBI as MDB_dbi);
    }
    len = strlen(name);
    i = CORE_DBS as MDB_dbi;
    while i < (*txn).mt_numdbs {
        if (*((*txn).mt_dbxs).offset(i as isize)).md_name.mv_size == 0 {
            if unused == 0 {
                unused = i;
            }
        } else if len == (*((*txn).mt_dbxs).offset(i as isize)).md_name.mv_size
            && strncmp(
                name,
                (*((*txn).mt_dbxs).offset(i as isize)).md_name.mv_data as *const libc::c_char,
                len,
            ) == 0
        {
            *dbi = i;
            return MDB_SUCCESS;
        }
        i = i.wrapping_add(1);
        i;
    }
    if unused == 0 && (*txn).mt_numdbs >= (*(*txn).mt_env).me_maxdbs {
        return MDB_DBS_FULL;
    }
    if (*((*txn).mt_dbs).offset(MAIN_DBI as isize)).md_flags as libc::c_int
        & (MDB_DUPSORT | MDB_INTEGERKEY)
        != 0
    {
        return if flags & MDB_CREATE as libc::c_uint != 0 {
            MDB_INCOMPATIBLE
        } else {
            MDB_NOTFOUND
        };
    }
    dbflag = DB_NEW | DB_VALID | DB_USRVALID;
    exact = 0 as libc::c_int;
    key.mv_size = len;
    key.mv_data = name as *mut libc::c_void;
    mdb_cursor_init(&mut mc, txn, MAIN_DBI as MDB_dbi, NULL as *mut MDB_xcursor);
    rc = mdb_cursor_set(&mut mc, &mut key, &mut data, MDB_SET, &mut exact);
    if rc == MDB_SUCCESS {
        let mut node = (mc.mc_pg[mc.mc_top as usize] as *mut libc::c_char)
            .offset(
                *((*(mc.mc_pg[mc.mc_top as usize] as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(mc.mc_ki[mc.mc_top as usize] as isize) as libc::c_int
                    as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    PAGEHDRSZ as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if (*node).mn_flags as libc::c_int & (F_DUPDATA | F_SUBDATA) != F_SUBDATA {
            return MDB_INCOMPATIBLE;
        }
    } else {
        if rc != MDB_NOTFOUND || flags & MDB_CREATE as libc::c_uint == 0 {
            return rc;
        }
        if (*txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint
            == 0x20000 as libc::c_int as libc::c_uint
        {
            return EACCES;
        }
    }
    namedup = strdup(name);
    if namedup.is_null() {
        return ENOMEM;
    }
    if rc != 0 {
        data.mv_size = ::core::mem::size_of::<MDB_db>() as libc::c_ulong;
        data.mv_data = &mut dummy as *mut MDB_db as *mut libc::c_void;
        memset(
            &mut dummy as *mut MDB_db as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<MDB_db>() as libc::c_ulong,
        );
        dummy.md_root = P_INVALID;
        dummy.md_flags = (flags & PERSISTENT_FLAGS as libc::c_uint) as uint16_t;
        let mut dummy_0 = MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        };
        let mut tracked = 0 as *mut MDB_cursor;
        let mut tp: *mut *mut MDB_cursor =
            &mut *((*mc.mc_txn).mt_cursors).offset(mc.mc_dbi as isize) as *mut *mut MDB_cursor;
        if mc.mc_flags & C_SUB as libc::c_uint != 0 {
            dummy_0.mc_flags = C_INITIALIZED as libc::c_uint;
            dummy_0.mc_xcursor = &mut mc as *mut MDB_cursor as *mut MDB_xcursor;
            tracked = &mut dummy_0;
        } else {
            tracked = &mut mc;
        }
        (*tracked).mc_next = *tp;
        *tp = tracked;
        rc = _mdb_cursor_put(
            &mut mc,
            &mut key,
            &mut data,
            0x2 as libc::c_int as libc::c_uint,
        );
        *tp = (*tracked).mc_next;
        dbflag |= DB_DIRTY;
    }
    if rc != 0 {
        free(namedup as *mut libc::c_void);
    } else {
        let mut slot = if unused != 0 {
            unused
        } else {
            (*txn).mt_numdbs
        };
        let ref mut fresh78 = (*((*txn).mt_dbxs).offset(slot as isize)).md_name.mv_data;
        *fresh78 = namedup as *mut libc::c_void;
        (*((*txn).mt_dbxs).offset(slot as isize)).md_name.mv_size = len;
        let ref mut fresh79 = (*((*txn).mt_dbxs).offset(slot as isize)).md_rel;
        *fresh79 =
            ::core::mem::transmute::<libc::intptr_t, Option<MDB_rel_func>>(NULL as libc::intptr_t);
        *((*txn).mt_dbflags).offset(slot as isize) = dbflag as libc::c_uchar;
        let ref mut fresh80 = *((*(*txn).mt_env).me_dbiseqs).offset(slot as isize);
        *fresh80 = (*fresh80).wrapping_add(1);
        seq = *fresh80;
        *((*txn).mt_dbiseqs).offset(slot as isize) = seq;
        memcpy(
            &mut *((*txn).mt_dbs).offset(slot as isize) as *mut MDB_db as *mut libc::c_void,
            data.mv_data,
            ::core::mem::size_of::<MDB_db>() as libc::c_ulong,
        );
        *dbi = slot;
        mdb_default_cmp(txn, slot);
        if unused == 0 {
            (*txn).mt_numdbs = ((*txn).mt_numdbs).wrapping_add(1);
            (*txn).mt_numdbs;
        }
    }
    return rc;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_stat(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut arg: *mut MDB_stat,
) -> libc::c_int {
    if arg.is_null()
        || !(!txn.is_null()
            && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x8 as libc::c_int != 0)
    {
        return EINVAL;
    }
    if (*txn).mt_flags & MDB_TXN_BLOCKED as libc::c_uint != 0 {
        return MDB_BAD_TXN;
    }
    if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & DB_STALE != 0 {
        let mut mc = MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        };
        let mut mx = MDB_xcursor {
            mx_cursor: MDB_cursor {
                mc_next: 0 as *mut MDB_cursor,
                mc_backup: 0 as *mut MDB_cursor,
                mc_xcursor: 0 as *mut MDB_xcursor,
                mc_txn: 0 as *mut MDB_txn,
                mc_dbi: 0,
                mc_db: 0 as *mut MDB_db,
                mc_dbx: 0 as *mut MDB_dbx,
                mc_dbflag: 0 as *mut libc::c_uchar,
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [0 as *mut MDB_page; 32],
                mc_ki: [0; 32],
            },
            mx_db: MDB_db {
                md_pad: 0,
                md_flags: 0,
                md_depth: 0,
                md_branch_pages: 0,
                md_leaf_pages: 0,
                md_overflow_pages: 0,
                md_entries: 0,
                md_root: 0,
            },
            mx_dbx: MDB_dbx {
                md_name: MDB_val {
                    mv_size: 0,
                    mv_data: 0 as *mut libc::c_void,
                },
                md_cmp: None,
                md_dcmp: None,
                md_rel: None,
                md_relctx: 0 as *mut libc::c_void,
            },
            mx_dbflag: 0,
        };
        mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
    }
    return mdb_stat0(
        (*txn).mt_env,
        &mut *((*txn).mt_dbs).offset(dbi as isize),
        arg,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mdb_dbi_close(mut env: *mut MDB_env, mut dbi: MDB_dbi) {
    let mut ptr = 0 as *mut libc::c_char;
    if dbi < CORE_DBS as MDB_dbi || dbi >= (*env).me_maxdbs {
        return;
    }
    ptr = (*((*env).me_dbxs).offset(dbi as isize)).md_name.mv_data as *mut libc::c_char;
    if !ptr.is_null() {
        let ref mut fresh81 = (*((*env).me_dbxs).offset(dbi as isize)).md_name.mv_data;
        *fresh81 = NULL as *mut libc::c_void;
        (*((*env).me_dbxs).offset(dbi as isize)).md_name.mv_size = 0 as libc::c_int as size_t;
        *((*env).me_dbflags).offset(dbi as isize) = 0 as libc::c_int as uint16_t;
        let ref mut fresh82 = *((*env).me_dbiseqs).offset(dbi as isize);
        *fresh82 = (*fresh82).wrapping_add(1);
        let _ = *fresh82;
        free(ptr as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mdb_dbi_flags(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut flags: *mut libc::c_uint,
) -> libc::c_int {
    if !(!txn.is_null()
        && dbi < (*txn).mt_numdbs
        && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int != 0)
    {
        return EINVAL;
    }
    *flags = ((*((*txn).mt_dbs).offset(dbi as isize)).md_flags as libc::c_int & PERSISTENT_FLAGS)
        as libc::c_uint;
    return MDB_SUCCESS;
}
unsafe extern "C" fn mdb_drop0(mut mc: *mut MDB_cursor, mut subs: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    rc = mdb_page_search(mc, NULL as *mut MDB_val, MDB_PS_FIRST);
    if rc == MDB_SUCCESS {
        let mut txn = (*mc).mc_txn;
        let mut ni = 0 as *mut MDB_node;
        let mut mx = MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        };
        let mut i: libc::c_uint = 0;
        if (*mc).mc_flags & C_SUB as libc::c_uint != 0
            || subs == 0 && (*(*mc).mc_db).md_overflow_pages == 0
        {
            mdb_cursor_pop(mc);
        }
        mdb_cursor_copy(mc, &mut mx);
        's_29: loop {
            if !((*mc).mc_snum as libc::c_int > 0 as libc::c_int) {
                current_block = 14447253356787937536;
                break;
            }
            let mut mp = (*mc).mc_pg[(*mc).mc_top as usize];
            let mut n = ((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_lower as libc::c_uint)
                .wrapping_sub(
                    (PAGEHDRSZ as libc::c_uint).wrapping_sub(if 0 as libc::c_int != 0 {
                        PAGEHDRSZ as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                >> 1 as libc::c_int;
            if (*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                & 0x2 as libc::c_int
                == 0x2 as libc::c_int
            {
                i = 0 as libc::c_int as libc::c_uint;
                while i < n {
                    ni = (mp as *mut libc::c_char)
                        .offset(
                            *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                .as_mut_ptr()
                                .offset(i as isize) as libc::c_int
                                as isize,
                        )
                        .offset(
                            (if 0 as libc::c_int != 0 {
                                PAGEHDRSZ as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    if (*ni).mn_flags as libc::c_int & F_BIGDATA != 0 {
                        let mut omp = 0 as *mut MDB_page;
                        let mut pg: pgno_t = 0;
                        memcpy(
                            &mut pg as *mut pgno_t as *mut libc::c_void,
                            ((*ni).mn_data)
                                .as_mut_ptr()
                                .offset((*ni).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void,
                            ::core::mem::size_of::<pgno_t>() as libc::c_ulong,
                        );
                        rc = mdb_page_get(mc, pg, &mut omp, NULL as *mut libc::c_int);
                        if rc != 0 as libc::c_int {
                            current_block = 10762829585299754077;
                            break 's_29;
                        }
                        if (*(omp as *mut libc::c_void as *mut MDB_page2)).mp2_flags as libc::c_int
                            & 0x4 as libc::c_int
                            == 0x4 as libc::c_int
                        {
                        } else {
                            mdb_assert_fail(
                                (*(*mc).mc_txn).mt_env,
                                b"IS_OVERFLOW(omp)\0" as *const u8 as *const libc::c_char,
                                (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(
                                    b"mdb_drop0\0",
                                ))
                                .as_ptr(),
                                b"mdb.c\0" as *const u8 as *const libc::c_char,
                                11089 as libc::c_int,
                            );
                        };
                        rc = mdb_midl_append_range(
                            &mut (*txn).mt_free_pgs,
                            pg,
                            (*omp).mp_pb.pb_pages,
                        );
                        if rc != 0 {
                            current_block = 10762829585299754077;
                            break 's_29;
                        }
                        (*(*mc).mc_db).md_overflow_pages = ((*(*mc).mc_db).md_overflow_pages)
                            .wrapping_sub((*omp).mp_pb.pb_pages as pgno_t);
                        if (*(*mc).mc_db).md_overflow_pages == 0 && subs == 0 {
                            break;
                        }
                    } else if subs != 0 && (*ni).mn_flags as libc::c_int & F_SUBDATA != 0 {
                        mdb_xcursor_init1(mc, ni);
                        rc = mdb_drop0(&mut (*(*mc).mc_xcursor).mx_cursor, 0 as libc::c_int);
                        if rc != 0 {
                            current_block = 10762829585299754077;
                            break 's_29;
                        }
                    }
                    i = i.wrapping_add(1);
                    i;
                }
                if subs == 0 && (*(*mc).mc_db).md_overflow_pages == 0 {
                    current_block = 16366064998044352055;
                } else {
                    current_block = 14434620278749266018;
                }
            } else {
                rc = mdb_midl_need(&mut (*txn).mt_free_pgs, n);
                if rc != 0 as libc::c_int {
                    current_block = 10762829585299754077;
                    break;
                }
                i = 0 as libc::c_int as libc::c_uint;
                while i < n {
                    let mut pg_0: pgno_t = 0;
                    ni = (mp as *mut libc::c_char)
                        .offset(
                            *((*(mp as *mut libc::c_void as *mut MDB_page2)).mp2_ptrs)
                                .as_mut_ptr()
                                .offset(i as isize) as libc::c_int
                                as isize,
                        )
                        .offset(
                            (if 0 as libc::c_int != 0 {
                                PAGEHDRSZ as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    pg_0 = (*ni).mn_lo as pgno_t
                        | ((*ni).mn_hi as pgno_t) << 16 as libc::c_int
                        | (if (if -(1 as libc::c_int) as pgno_t
                            > 0xffffffff as libc::c_uint as pgno_t
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) != 0
                        {
                            ((*ni).mn_flags as pgno_t)
                                << (if -(1 as libc::c_int) as pgno_t
                                    > 0xffffffff as libc::c_uint as pgno_t
                                {
                                    32 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                        } else {
                            0 as libc::c_int as pgno_t
                        });
                    let mut xidl = (*txn).mt_free_pgs;
                    let ref mut fresh83 = *xidl.offset(0 as libc::c_int as isize);
                    *fresh83 = (*fresh83).wrapping_add(1);
                    let mut xlen = *fresh83;
                    *xidl.offset(xlen as isize) = pg_0;
                    i = i.wrapping_add(1);
                    i;
                }
                current_block = 14434620278749266018;
            }
            match current_block {
                14434620278749266018 => {
                    if (*mc).mc_top == 0 {
                        current_block = 14447253356787937536;
                        break;
                    }
                    (*mc).mc_ki[(*mc).mc_top as usize] = i as indx_t;
                    rc = mdb_cursor_sibling(mc, 1 as libc::c_int);
                    if !(rc != 0) {
                        continue;
                    }
                    if rc != MDB_NOTFOUND {
                        current_block = 10762829585299754077;
                        break;
                    }
                }
                _ => {}
            }
            mdb_cursor_pop(mc);
            (*mc).mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
            i = 1 as libc::c_int as libc::c_uint;
            while i < (*mc).mc_snum as libc::c_uint {
                (*mc).mc_ki[i as usize] = 0 as libc::c_int as indx_t;
                (*mc).mc_pg[i as usize] = mx.mc_pg[i as usize];
                i = i.wrapping_add(1);
                i;
            }
        }
        match current_block {
            14447253356787937536 => {
                rc = mdb_midl_append(&mut (*txn).mt_free_pgs, (*(*mc).mc_db).md_root);
            }
            _ => {}
        }
        if rc != 0 {
            (*txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
        }
    } else if rc == MDB_NOTFOUND {
        rc = MDB_SUCCESS;
    }
    (*mc).mc_flags &= !C_INITIALIZED as libc::c_uint;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_drop(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut del: libc::c_int,
) -> libc::c_int {
    let mut mc = 0 as *mut MDB_cursor;
    let mut m2 = 0 as *mut MDB_cursor;
    let mut rc: libc::c_int = 0;
    if del as libc::c_uint > 1 as libc::c_int as libc::c_uint
        || !(!txn.is_null()
            && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int != 0)
    {
        return EINVAL;
    }
    if (*txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint
        == 0x20000 as libc::c_int as libc::c_uint
    {
        return EACCES;
    }
    if *((*txn).mt_dbiseqs).offset(dbi as isize)
        != *((*(*txn).mt_env).me_dbiseqs).offset(dbi as isize)
    {
        return MDB_BAD_DBI;
    }
    rc = mdb_cursor_open(txn, dbi, &mut mc);
    if rc != 0 {
        return rc;
    }
    rc = mdb_drop0(mc, (*(*mc).mc_db).md_flags as libc::c_int & MDB_DUPSORT);
    m2 = *((*txn).mt_cursors).offset(dbi as isize);
    while !m2.is_null() {
        (*m2).mc_flags &= !(C_INITIALIZED | C_EOF) as libc::c_uint;
        m2 = (*m2).mc_next;
    }
    if !(rc != 0) {
        if del != 0 && dbi >= CORE_DBS as MDB_dbi {
            rc = mdb_del0(
                txn,
                MAIN_DBI as MDB_dbi,
                &mut (*(*mc).mc_dbx).md_name,
                NULL as *mut MDB_val,
                F_SUBDATA as libc::c_uint,
            );
            if rc == 0 {
                *((*txn).mt_dbflags).offset(dbi as isize) = DB_STALE as libc::c_uchar;
                mdb_dbi_close((*txn).mt_env, dbi);
            } else {
                (*txn).mt_flags |= MDB_TXN_ERROR as libc::c_uint;
            }
        } else {
            let ref mut fresh84 = *((*txn).mt_dbflags).offset(dbi as isize);
            *fresh84 = (*fresh84 as libc::c_int | DB_DIRTY) as libc::c_uchar;
            (*((*txn).mt_dbs).offset(dbi as isize)).md_depth = 0 as libc::c_int as uint16_t;
            (*((*txn).mt_dbs).offset(dbi as isize)).md_branch_pages = 0 as libc::c_int as pgno_t;
            (*((*txn).mt_dbs).offset(dbi as isize)).md_leaf_pages = 0 as libc::c_int as pgno_t;
            (*((*txn).mt_dbs).offset(dbi as isize)).md_overflow_pages = 0 as libc::c_int as pgno_t;
            (*((*txn).mt_dbs).offset(dbi as isize)).md_entries = 0 as libc::c_int as mdb_size_t;
            (*((*txn).mt_dbs).offset(dbi as isize)).md_root = P_INVALID;
            (*txn).mt_flags |= MDB_TXN_DIRTY as libc::c_uint;
        }
    }
    mdb_cursor_close(mc);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_set_compare(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut cmp: Option<MDB_cmp_func>,
) -> libc::c_int {
    if !(!txn.is_null()
        && dbi < (*txn).mt_numdbs
        && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int != 0)
    {
        return EINVAL;
    }
    let ref mut fresh85 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_cmp;
    *fresh85 = cmp;
    return MDB_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_set_dupsort(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut cmp: Option<MDB_cmp_func>,
) -> libc::c_int {
    if !(!txn.is_null()
        && dbi < (*txn).mt_numdbs
        && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int != 0)
    {
        return EINVAL;
    }
    let ref mut fresh86 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_dcmp;
    *fresh86 = cmp;
    return MDB_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_set_relfunc(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut rel: Option<MDB_rel_func>,
) -> libc::c_int {
    if !(!txn.is_null()
        && dbi < (*txn).mt_numdbs
        && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int != 0)
    {
        return EINVAL;
    }
    let ref mut fresh87 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_rel;
    *fresh87 = rel;
    return MDB_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn mdb_set_relctx(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut ctx: *mut libc::c_void,
) -> libc::c_int {
    if !(!txn.is_null()
        && dbi < (*txn).mt_numdbs
        && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int != 0)
    {
        return EINVAL;
    }
    let ref mut fresh88 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_relctx;
    *fresh88 = ctx;
    return MDB_SUCCESS;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_env_get_maxkeysize(mut _env: *mut MDB_env) -> libc::c_int {
    return if 0 as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        511 as libc::c_int
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_reader_list(
    mut env: *mut MDB_env,
    mut func: Option<MDB_msg_func>,
    mut ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut rdrs: libc::c_uint = 0;
    let mut mr = 0 as *mut MDB_reader;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut rc = 0 as libc::c_int;
    let mut first = 1 as libc::c_int;
    if env.is_null() || func.is_none() {
        return -(1 as libc::c_int);
    }
    if ((*env).me_txns).is_null() {
        return func.expect("non-null function pointer")(
            b"(no reader locks)\n\0" as *const u8 as *const libc::c_char,
            ctx,
        );
    }
    rdrs = (*(*env).me_txns).mt1.mtb.mtb_numreaders;
    mr = ((*(*env).me_txns).mti_readers).as_mut_ptr();
    i = 0 as libc::c_int as libc::c_uint;
    while i < rdrs {
        if (*mr.offset(i as isize)).mru.mrx.mrb_pid != 0 {
            let mut txnid = (*mr.offset(i as isize)).mru.mrx.mrb_txnid;
            sprintf(
                buf.as_mut_ptr(),
                if txnid == -(1 as libc::c_int) as txnid_t {
                    b"%10d %zx -\n\0" as *const u8 as *const libc::c_char
                } else {
                    b"%10d %zx %zu\n\0" as *const u8 as *const libc::c_char
                },
                (*mr.offset(i as isize)).mru.mrx.mrb_pid,
                (*mr.offset(i as isize)).mru.mrx.mrb_tid as size_t,
                txnid,
            );
            if first != 0 {
                first = 0 as libc::c_int;
                rc = func.expect("non-null function pointer")(
                    b"    pid     thread     txnid\n\0" as *const u8 as *const libc::c_char,
                    ctx,
                );
                if rc < 0 as libc::c_int {
                    break;
                }
            }
            rc = func.expect("non-null function pointer")(buf.as_mut_ptr(), ctx);
            if rc < 0 as libc::c_int {
                break;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if first != 0 {
        rc = func.expect("non-null function pointer")(
            b"(no active readers)\n\0" as *const u8 as *const libc::c_char,
            ctx,
        );
    }
    return rc;
}
#[cold]
unsafe extern "C" fn mdb_pid_insert(mut ids: *mut pid_t, mut pid: pid_t) -> libc::c_int {
    let mut base = 0 as libc::c_int as libc::c_uint;
    let mut cursor = 1 as libc::c_int as libc::c_uint;
    let mut val = 0 as libc::c_int;
    let mut n = *ids.offset(0 as libc::c_int as isize) as libc::c_uint;
    while (0 as libc::c_int as libc::c_uint) < n {
        let mut pivot = n >> 1 as libc::c_int;
        cursor = base
            .wrapping_add(pivot)
            .wrapping_add(1 as libc::c_int as libc::c_uint);
        val = pid - *ids.offset(cursor as isize);
        if val < 0 as libc::c_int {
            n = pivot;
        } else if val > 0 as libc::c_int {
            base = cursor;
            n = n.wrapping_sub(pivot.wrapping_add(1 as libc::c_int as libc::c_uint));
        } else {
            return -(1 as libc::c_int);
        }
    }
    if val > 0 as libc::c_int {
        cursor = cursor.wrapping_add(1);
        cursor;
    }
    let ref mut fresh89 = *ids.offset(0 as libc::c_int as isize);
    *fresh89 += 1;
    let _ = *fresh89;
    n = *ids.offset(0 as libc::c_int as isize) as libc::c_uint;
    while n > cursor {
        *ids.offset(n as isize) =
            *ids.offset(n.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        n = n.wrapping_sub(1);
        n;
    }
    *ids.offset(n as isize) = pid;
    return 0 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn mdb_reader_check(
    mut env: *mut MDB_env,
    mut dead: *mut libc::c_int,
) -> libc::c_int {
    if env.is_null() {
        return EINVAL;
    }
    if !dead.is_null() {
        *dead = 0 as libc::c_int;
    }
    return if !((*env).me_txns).is_null() {
        mdb_reader_check0(env, 0 as libc::c_int, dead)
    } else {
        MDB_SUCCESS
    };
}
#[cold]
unsafe extern "C" fn mdb_reader_check0(
    mut env: *mut MDB_env,
    mut rlocked: libc::c_int,
    mut dead: *mut libc::c_int,
) -> libc::c_int {
    let mut rmutex = if rlocked != 0 {
        NULL as *mut mdb_mutex
    } else {
        ((*env).me_rmutex).as_mut_ptr()
    };
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut rdrs: libc::c_uint = 0;
    let mut mr = 0 as *mut MDB_reader;
    let mut pids = 0 as *mut pid_t;
    let mut pid: pid_t = 0;
    let mut rc = MDB_SUCCESS;
    let mut count = 0 as libc::c_int;
    rdrs = (*(*env).me_txns).mt1.mtb.mtb_numreaders;
    pids = malloc(
        (rdrs.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<pid_t>() as libc::c_ulong),
    ) as *mut pid_t;
    if pids.is_null() {
        return ENOMEM;
    }
    *pids.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    mr = ((*(*env).me_txns).mti_readers).as_mut_ptr();
    i = 0 as libc::c_int as libc::c_uint;
    while i < rdrs {
        pid = (*mr.offset(i as isize)).mru.mrx.mrb_pid;
        if pid != 0 && pid != (*env).me_pid {
            if mdb_pid_insert(pids, pid) == 0 as libc::c_int {
                if mdb_reader_pid(env, Pidcheck, pid) == 0 {
                    j = i;
                    if !rmutex.is_null() {
                        rc = mdb_sem_wait(rmutex);
                        if rc != 0 as libc::c_int {
                            rc = mdb_mutex_failed(env, rmutex, rc);
                            if rc != 0 {
                                break;
                            }
                            rdrs = 0 as libc::c_int as libc::c_uint;
                        } else if mdb_reader_pid(env, Pidcheck, pid) != 0 {
                            j = rdrs;
                        }
                    }
                    while j < rdrs {
                        if (*mr.offset(j as isize)).mru.mrx.mrb_pid == pid {
                            ::core::ptr::write_volatile(
                                &mut (*mr.offset(j as isize)).mru.mrx.mrb_pid as *mut pid_t,
                                0 as libc::c_int,
                            );
                            count += 1;
                            count;
                        }
                        j = j.wrapping_add(1);
                        j;
                    }
                    if !rmutex.is_null() {
                        let mut sb = {
                            let mut init = sembuf {
                                sem_num: 0 as libc::c_int as libc::c_ushort,
                                sem_op: 1 as libc::c_int as libc::c_short,
                                sem_flg: SEM_UNDO as libc::c_short,
                            };
                            init
                        };
                        sb.sem_num = (*rmutex).semnum as libc::c_ushort;
                        *(*rmutex).locked = 0 as libc::c_int;
                        semop((*rmutex).semid, &mut sb, 1 as libc::c_int as size_t);
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    free(pids as *mut libc::c_void);
    if !dead.is_null() {
        *dead = count;
    }
    return rc;
}
#[cold]
unsafe extern "C" fn mdb_mutex_failed(
    mut env: *mut MDB_env,
    mut mutex: mdb_mutexref_t,
    mut rc: libc::c_int,
) -> libc::c_int {
    let mut rlocked: libc::c_int = 0;
    let mut rc2: libc::c_int = 0;
    let mut meta = 0 as *mut MDB_meta;
    if rc == MDB_OWNERDEAD {
        rc = MDB_SUCCESS;
        rlocked = (mutex == ((*env).me_rmutex).as_mut_ptr()) as libc::c_int;
        if rlocked == 0 {
            meta = mdb_env_pick_meta(env);
            ::core::ptr::write_volatile(
                &mut (*(*env).me_txns).mt1.mtb.mtb_txnid as *mut txnid_t,
                (*meta).mm_txnid,
            );
            if !((*env).me_txn).is_null() {
                (*env).me_flags |= MDB_FATAL_ERROR;
                (*env).me_txn = NULL as *mut MDB_txn;
                rc = MDB_PANIC;
            }
        }
        rc2 = mdb_reader_check0(env, rlocked, NULL as *mut libc::c_int);
        if rc2 == 0 as libc::c_int {
            rc2 = 0 as libc::c_int;
        }
        if rc != 0 || {
            rc = rc2;
            rc != 0
        } {
            let mut sb = {
                let mut init = sembuf {
                    sem_num: 0 as libc::c_int as libc::c_ushort,
                    sem_op: 1 as libc::c_int as libc::c_short,
                    sem_flg: SEM_UNDO as libc::c_short,
                };
                init
            };
            sb.sem_num = (*mutex).semnum as libc::c_ushort;
            *(*mutex).locked = 0 as libc::c_int;
            semop((*mutex).semid, &mut sb, 1 as libc::c_int as size_t);
        }
    }
    return rc;
}
