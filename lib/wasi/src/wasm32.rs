//! WASI types as defined in wasm32. This file was originally generated
//! by running bindgen over __wasi_types.h with a wasm32 target, and the
//! content still largely reflects that, however it's been modified to be
//! host-independent.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -2147483648;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 2147483647;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 4294967295;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const INTPTR_MIN: i32 = -2147483648;
pub const INTPTR_MAX: u32 = 2147483647;
pub const UINTPTR_MAX: u32 = 4294967295;
pub const PTRDIFF_MIN: i32 = -2147483648;
pub const PTRDIFF_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: u32 = 4294967295;
pub const WINT_MIN: i32 = -2147483648;
pub const WINT_MAX: i32 = 2147483647;
pub const __WASI_ADVICE_DONTNEED: u32 = 1;
pub const __WASI_ADVICE_NOREUSE: u32 = 2;
pub const __WASI_ADVICE_NORMAL: u32 = 3;
pub const __WASI_ADVICE_RANDOM: u32 = 4;
pub const __WASI_ADVICE_SEQUENTIAL: u32 = 5;
pub const __WASI_ADVICE_WILLNEED: u32 = 6;
pub const __WASI_AT_ARGDATA: u32 = 256;
pub const __WASI_AT_ARGDATALEN: u32 = 257;
pub const __WASI_AT_BASE: u32 = 7;
pub const __WASI_AT_CANARY: u32 = 258;
pub const __WASI_AT_CANARYLEN: u32 = 259;
pub const __WASI_AT_NCPUS: u32 = 260;
pub const __WASI_AT_NULL: u32 = 0;
pub const __WASI_AT_PAGESZ: u32 = 6;
pub const __WASI_AT_PHDR: u32 = 3;
pub const __WASI_AT_PHNUM: u32 = 4;
pub const __WASI_AT_PID: u32 = 263;
pub const __WASI_AT_SYSINFO_EHDR: u32 = 262;
pub const __WASI_AT_TID: u32 = 261;
pub const __WASI_CLOCK_MONOTONIC: u32 = 1;
pub const __WASI_CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
pub const __WASI_CLOCK_REALTIME: u32 = 3;
pub const __WASI_CLOCK_THREAD_CPUTIME_ID: u32 = 4;
pub const __WASI_CONDVAR_HAS_NO_WAITERS: u32 = 0;
pub const __WASI_DIRCOOKIE_START: u32 = 0;
pub const __WASI_ESUCCESS: __wasi_errno_t = 0;
pub const __WASI_E2BIG: __wasi_errno_t = 1;
pub const __WASI_EACCES: __wasi_errno_t = 2;
pub const __WASI_EADDRINUSE: __wasi_errno_t = 3;
pub const __WASI_EADDRNOTAVAIL: __wasi_errno_t = 4;
pub const __WASI_EAFNOSUPPORT: __wasi_errno_t = 5;
pub const __WASI_EAGAIN: __wasi_errno_t = 6;
pub const __WASI_EALREADY: __wasi_errno_t = 7;
pub const __WASI_EBADF: __wasi_errno_t = 8;
pub const __WASI_EBADMSG: __wasi_errno_t = 9;
pub const __WASI_EBUSY: __wasi_errno_t = 10;
pub const __WASI_ECANCELED: __wasi_errno_t = 11;
pub const __WASI_ECHILD: __wasi_errno_t = 12;
pub const __WASI_ECONNABORTED: __wasi_errno_t = 13;
pub const __WASI_ECONNREFUSED: __wasi_errno_t = 14;
pub const __WASI_ECONNRESET: __wasi_errno_t = 15;
pub const __WASI_EDEADLK: __wasi_errno_t = 16;
pub const __WASI_EDESTADDRREQ: __wasi_errno_t = 17;
pub const __WASI_EDOM: __wasi_errno_t = 18;
pub const __WASI_EDQUOT: __wasi_errno_t = 19;
pub const __WASI_EEXIST: __wasi_errno_t = 20;
pub const __WASI_EFAULT: __wasi_errno_t = 21;
pub const __WASI_EFBIG: __wasi_errno_t = 22;
pub const __WASI_EHOSTUNREACH: __wasi_errno_t = 23;
pub const __WASI_EIDRM: __wasi_errno_t = 24;
pub const __WASI_EILSEQ: __wasi_errno_t = 25;
pub const __WASI_EINPROGRESS: __wasi_errno_t = 26;
pub const __WASI_EINTR: __wasi_errno_t = 27;
pub const __WASI_EINVAL: __wasi_errno_t = 28;
pub const __WASI_EIO: __wasi_errno_t = 29;
pub const __WASI_EISCONN: __wasi_errno_t = 30;
pub const __WASI_EISDIR: __wasi_errno_t = 31;
pub const __WASI_ELOOP: __wasi_errno_t = 32;
pub const __WASI_EMFILE: __wasi_errno_t = 33;
pub const __WASI_EMLINK: __wasi_errno_t = 34;
pub const __WASI_EMSGSIZE: __wasi_errno_t = 35;
pub const __WASI_EMULTIHOP: __wasi_errno_t = 36;
pub const __WASI_ENAMETOOLONG: __wasi_errno_t = 37;
pub const __WASI_ENETDOWN: __wasi_errno_t = 38;
pub const __WASI_ENETRESET: __wasi_errno_t = 39;
pub const __WASI_ENETUNREACH: __wasi_errno_t = 40;
pub const __WASI_ENFILE: __wasi_errno_t = 41;
pub const __WASI_ENOBUFS: __wasi_errno_t = 42;
pub const __WASI_ENODEV: __wasi_errno_t = 43;
pub const __WASI_ENOENT: __wasi_errno_t = 44;
pub const __WASI_ENOEXEC: __wasi_errno_t = 45;
pub const __WASI_ENOLCK: __wasi_errno_t = 46;
pub const __WASI_ENOLINK: __wasi_errno_t = 47;
pub const __WASI_ENOMEM: __wasi_errno_t = 48;
pub const __WASI_ENOMSG: __wasi_errno_t = 49;
pub const __WASI_ENOPROTOOPT: __wasi_errno_t = 50;
pub const __WASI_ENOSPC: __wasi_errno_t = 51;
pub const __WASI_ENOSYS: __wasi_errno_t = 52;
pub const __WASI_ENOTCONN: __wasi_errno_t = 53;
pub const __WASI_ENOTDIR: __wasi_errno_t = 54;
pub const __WASI_ENOTEMPTY: __wasi_errno_t = 55;
pub const __WASI_ENOTRECOVERABLE: __wasi_errno_t = 56;
pub const __WASI_ENOTSOCK: __wasi_errno_t = 57;
pub const __WASI_ENOTSUP: __wasi_errno_t = 58;
pub const __WASI_ENOTTY: __wasi_errno_t = 59;
pub const __WASI_ENXIO: __wasi_errno_t = 60;
pub const __WASI_EOVERFLOW: __wasi_errno_t = 61;
pub const __WASI_EOWNERDEAD: __wasi_errno_t = 62;
pub const __WASI_EPERM: __wasi_errno_t = 63;
pub const __WASI_EPIPE: __wasi_errno_t = 64;
pub const __WASI_EPROTO: __wasi_errno_t = 65;
pub const __WASI_EPROTONOSUPPORT: __wasi_errno_t = 66;
pub const __WASI_EPROTOTYPE: __wasi_errno_t = 67;
pub const __WASI_ERANGE: __wasi_errno_t = 68;
pub const __WASI_EROFS: __wasi_errno_t = 69;
pub const __WASI_ESPIPE: __wasi_errno_t = 70;
pub const __WASI_ESRCH: __wasi_errno_t = 71;
pub const __WASI_ESTALE: __wasi_errno_t = 72;
pub const __WASI_ETIMEDOUT: __wasi_errno_t = 73;
pub const __WASI_ETXTBSY: __wasi_errno_t = 74;
pub const __WASI_EXDEV: __wasi_errno_t = 75;
pub const __WASI_ENOTCAPABLE: __wasi_errno_t = 76;
pub const __WASI_EVENT_FD_READWRITE_HANGUP: u32 = 1;
pub const __WASI_EVENTTYPE_CLOCK: u32 = 1;
pub const __WASI_EVENTTYPE_CONDVAR: u32 = 2;
pub const __WASI_EVENTTYPE_FD_READ: u32 = 3;
pub const __WASI_EVENTTYPE_FD_WRITE: u32 = 4;
pub const __WASI_EVENTTYPE_LOCK_RDLOCK: u32 = 5;
pub const __WASI_EVENTTYPE_LOCK_WRLOCK: u32 = 6;
pub const __WASI_EVENTTYPE_PROC_TERMINATE: u32 = 7;
pub const __WASI_PROCESS_CHILD: u32 = 4294967295;
pub const __WASI_MAP_ANON_FD: u32 = 4294967295;
pub const __WASI_FDFLAG_APPEND: u32 = 1;
pub const __WASI_FDFLAG_DSYNC: u32 = 2;
pub const __WASI_FDFLAG_NONBLOCK: u32 = 4;
pub const __WASI_FDFLAG_RSYNC: u32 = 8;
pub const __WASI_FDFLAG_SYNC: u32 = 16;
pub const __WASI_FDSTAT_FLAGS: u32 = 1;
pub const __WASI_FDSTAT_RIGHTS: u32 = 2;
pub const __WASI_FILETYPE_UNKNOWN: u32 = 0;
pub const __WASI_FILETYPE_BLOCK_DEVICE: u32 = 16;
pub const __WASI_FILETYPE_CHARACTER_DEVICE: u32 = 17;
pub const __WASI_FILETYPE_DIRECTORY: u32 = 32;
pub const __WASI_FILETYPE_PROCESS: u32 = 80;
pub const __WASI_FILETYPE_REGULAR_FILE: u32 = 96;
pub const __WASI_FILETYPE_SHARED_MEMORY: u32 = 112;
pub const __WASI_FILETYPE_SOCKET_DGRAM: u32 = 128;
pub const __WASI_FILETYPE_SOCKET_STREAM: u32 = 130;
pub const __WASI_FILETYPE_SYMBOLIC_LINK: u32 = 144;
pub const __WASI_FILESTAT_ATIM: u32 = 1;
pub const __WASI_FILESTAT_ATIM_NOW: u32 = 2;
pub const __WASI_FILESTAT_MTIM: u32 = 4;
pub const __WASI_FILESTAT_MTIM_NOW: u32 = 8;
pub const __WASI_FILESTAT_SIZE: u32 = 16;
pub const __WASI_LOCK_UNLOCKED: u32 = 0;
pub const __WASI_LOCK_WRLOCKED: u32 = 1073741824;
pub const __WASI_LOCK_KERNEL_MANAGED: u32 = 2147483648;
pub const __WASI_LOCK_BOGUS: u32 = 2147483648;
pub const __WASI_LOOKUP_SYMLINK_FOLLOW: u32 = 1;
pub const __WASI_MAP_ANON: u32 = 1;
pub const __WASI_MAP_FIXED: u32 = 2;
pub const __WASI_MAP_PRIVATE: u32 = 4;
pub const __WASI_MAP_SHARED: u32 = 8;
pub const __WASI_PROT_EXEC: u32 = 1;
pub const __WASI_PROT_WRITE: u32 = 2;
pub const __WASI_PROT_READ: u32 = 4;
pub const __WASI_MS_ASYNC: u32 = 1;
pub const __WASI_MS_INVALIDATE: u32 = 2;
pub const __WASI_MS_SYNC: u32 = 4;
pub const __WASI_O_CREAT: u32 = 1;
pub const __WASI_O_DIRECTORY: u32 = 2;
pub const __WASI_O_EXCL: u32 = 4;
pub const __WASI_O_TRUNC: u32 = 8;
pub const __WASI_SOCK_RECV_PEEK: u32 = 4;
pub const __WASI_SOCK_RECV_WAITALL: u32 = 16;
pub const __WASI_RIGHT_FD_DATASYNC: u32 = 1;
pub const __WASI_RIGHT_FD_READ: u32 = 2;
pub const __WASI_RIGHT_FD_SEEK: u32 = 4;
pub const __WASI_RIGHT_FD_STAT_PUT_FLAGS: u32 = 8;
pub const __WASI_RIGHT_FD_SYNC: u32 = 16;
pub const __WASI_RIGHT_FD_TELL: u32 = 32;
pub const __WASI_RIGHT_FD_WRITE: u32 = 64;
pub const __WASI_RIGHT_FILE_ADVISE: u32 = 128;
pub const __WASI_RIGHT_FILE_ALLOCATE: u32 = 256;
pub const __WASI_RIGHT_FILE_CREATE_DIRECTORY: u32 = 512;
pub const __WASI_RIGHT_FILE_CREATE_FILE: u32 = 1024;
pub const __WASI_RIGHT_FILE_LINK_SOURCE: u32 = 4096;
pub const __WASI_RIGHT_FILE_LINK_TARGET: u32 = 8192;
pub const __WASI_RIGHT_FILE_OPEN: u32 = 16384;
pub const __WASI_RIGHT_FILE_READDIR: u32 = 32768;
pub const __WASI_RIGHT_FILE_READLINK: u32 = 65536;
pub const __WASI_RIGHT_FILE_RENAME_SOURCE: u32 = 131072;
pub const __WASI_RIGHT_FILE_RENAME_TARGET: u32 = 262144;
pub const __WASI_RIGHT_FILE_STAT_FGET: u32 = 524288;
pub const __WASI_RIGHT_FILE_STAT_FPUT_SIZE: u32 = 1048576;
pub const __WASI_RIGHT_FILE_STAT_FPUT_TIMES: u32 = 2097152;
pub const __WASI_RIGHT_FILE_STAT_GET: u32 = 4194304;
pub const __WASI_RIGHT_FILE_STAT_PUT_TIMES: u32 = 8388608;
pub const __WASI_RIGHT_FILE_SYMLINK: u32 = 16777216;
pub const __WASI_RIGHT_FILE_UNLINK: u32 = 33554432;
pub const __WASI_RIGHT_MEM_MAP: u32 = 67108864;
pub const __WASI_RIGHT_MEM_MAP_EXEC: u32 = 134217728;
pub const __WASI_RIGHT_POLL_FD_READWRITE: u32 = 268435456;
pub const __WASI_RIGHT_POLL_PROC_TERMINATE: u32 = 1073741824;
pub const __WASI_RIGHT_PROC_EXEC: u64 = 4294967296;
pub const __WASI_RIGHT_SOCK_SHUTDOWN: u64 = 549755813888;
pub const __WASI_SOCK_RECV_FDS_TRUNCATED: u32 = 1;
pub const __WASI_SOCK_RECV_DATA_TRUNCATED: u32 = 8;
pub const __WASI_SCOPE_PRIVATE: u32 = 4;
pub const __WASI_SCOPE_SHARED: u32 = 8;
pub const __WASI_SHUT_RD: u32 = 1;
pub const __WASI_SHUT_WR: u32 = 2;
pub const __WASI_SIGABRT: u32 = 1;
pub const __WASI_SIGALRM: u32 = 2;
pub const __WASI_SIGBUS: u32 = 3;
pub const __WASI_SIGCHLD: u32 = 4;
pub const __WASI_SIGCONT: u32 = 5;
pub const __WASI_SIGFPE: u32 = 6;
pub const __WASI_SIGHUP: u32 = 7;
pub const __WASI_SIGILL: u32 = 8;
pub const __WASI_SIGINT: u32 = 9;
pub const __WASI_SIGKILL: u32 = 10;
pub const __WASI_SIGPIPE: u32 = 11;
pub const __WASI_SIGQUIT: u32 = 12;
pub const __WASI_SIGSEGV: u32 = 13;
pub const __WASI_SIGSTOP: u32 = 14;
pub const __WASI_SIGSYS: u32 = 15;
pub const __WASI_SIGTERM: u32 = 16;
pub const __WASI_SIGTRAP: u32 = 17;
pub const __WASI_SIGTSTP: u32 = 18;
pub const __WASI_SIGTTIN: u32 = 19;
pub const __WASI_SIGTTOU: u32 = 20;
pub const __WASI_SIGURG: u32 = 21;
pub const __WASI_SIGUSR1: u32 = 22;
pub const __WASI_SIGUSR2: u32 = 23;
pub const __WASI_SIGVTALRM: u32 = 24;
pub const __WASI_SIGXCPU: u32 = 25;
pub const __WASI_SIGXFSZ: u32 = 26;
pub const __WASI_SUBSCRIPTION_CLOCK_ABSTIME: u32 = 1;
pub const __WASI_SUBSCRIPTION_FD_READWRITE_POLL: u32 = 1;
pub const __WASI_UNLINK_REMOVEDIR: u32 = 1;
pub const __WASI_WHENCE_CUR: __wasi_whence_t = 1;
pub const __WASI_WHENCE_END: __wasi_whence_t = 2;
pub const __WASI_WHENCE_SET: __wasi_whence_t = 3;
pub type wchar_t = i32;
pub type size_t = u32;
pub type intptr_t = i32;
pub type uintptr_t = u32;
pub type long = i32;
pub type unsigned_long = u32;
pub type __u_char = u8;
pub type __u_short = u16;
pub type __u_int = u32;
pub type __u_long = unsigned_long;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __quad_t = i64;
pub type __u_quad_t = u64;
pub type __intmax_t = i64;
pub type __uintmax_t = u64;
pub type __dev_t = __u_quad_t;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = __u_quad_t;
pub type __ino64_t = __u_quad_t;
pub type __mode_t = u32;
pub type __nlink_t = __u_quad_t;
pub type __off_t = __quad_t;
pub type __off64_t = __quad_t;
pub type __pid_t = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [i32; 2usize],
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__fsid_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = __quad_t;
pub type __rlim_t = __u_quad_t;
pub type __rlim64_t = __u_quad_t;
pub type __id_t = u32;
pub type __time_t = __quad_t;
pub type __useconds_t = u32;
pub type __suseconds_t = __quad_t;
pub type __daddr_t = i32;
pub type __key_t = i32;
pub type __clockid_t = i32;
pub type __timer_t = uintptr_t; // *mut ::std::os::raw::c_void
pub type __blksize_t = __quad_t;
pub type __blkcnt_t = __quad_t;
pub type __blkcnt64_t = __quad_t;
pub type __fsblkcnt_t = __u_quad_t;
pub type __fsblkcnt64_t = __u_quad_t;
pub type __fsfilcnt_t = __u_quad_t;
pub type __fsfilcnt64_t = __u_quad_t;
pub type __fsword_t = __quad_t;
pub type __ssize_t = i32;
pub type __syscall_slong_t = __quad_t;
pub type __syscall_ulong_t = __u_quad_t;
pub type __loff_t = __off64_t;
pub type __caddr_t = uintptr_t; // *mut i8
pub type __intptr_t = i32;
pub type __uintptr_t = i32;
pub type __socklen_t = u32;
pub type __sig_atomic_t = i32;
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type int_fast16_t = i32;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u32;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type __wasi_advice_t = u8;
pub type __wasi_auxtype_t = u32;
pub type __wasi_clockid_t = u32;
pub type __wasi_condvar_t = u32;
pub type __wasi_device_t = u64;
pub type __wasi_dircookie_t = u64;
pub type __wasi_errno_t = u16;
pub type __wasi_eventrwflags_t = u16;
pub type __wasi_eventtype_t = u8;
pub type __wasi_exitcode_t = u32;
pub type __wasi_fd_t = u32;
pub type __wasi_fdflags_t = u16;
pub type __wasi_fdsflags_t = u16;
pub type __wasi_filedelta_t = i64;
pub type __wasi_filesize_t = u64;
pub type __wasi_filetype_t = u8;
pub type __wasi_fsflags_t = u16;
pub type __wasi_inode_t = u64;
pub type __wasi_linkcount_t = u32;
pub type __wasi_lock_t = u32;
pub type __wasi_lookupflags_t = u32;
pub type __wasi_mflags_t = u8;
pub type __wasi_mprot_t = u8;
pub type __wasi_msflags_t = u8;
pub type __wasi_nthreads_t = u32;
pub type __wasi_oflags_t = u16;
pub type __wasi_riflags_t = u16;
pub type __wasi_rights_t = u64;
pub type __wasi_roflags_t = u16;
pub type __wasi_scope_t = u8;
pub type __wasi_sdflags_t = u8;
pub type __wasi_siflags_t = u16;
pub type __wasi_signal_t = u8;
pub type __wasi_subclockflags_t = u16;
pub type __wasi_subrwflags_t = u16;
pub type __wasi_tid_t = u32;
pub type __wasi_timestamp_t = u64;
pub type __wasi_ulflags_t = u8;
pub type __wasi_userdata_t = u64;
pub type __wasi_whence_t = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_dirent_t {
    pub d_next: __wasi_dircookie_t,
    pub d_ino: __wasi_inode_t,
    pub d_namlen: u32,
    pub d_type: __wasi_filetype_t,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_wasi_dirent_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_dirent_t>(),
        24usize,
        concat!("Size of: ", stringify!(__wasi_dirent_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_dirent_t>())).d_next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_dirent_t),
            "::",
            stringify!(d_next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_dirent_t>())).d_ino as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_dirent_t),
            "::",
            stringify!(d_ino)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_dirent_t>())).d_namlen as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_dirent_t),
            "::",
            stringify!(d_namlen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_dirent_t>())).d_type as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_dirent_t),
            "::",
            stringify!(d_type)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __wasi_event_t {
    pub userdata: __wasi_userdata_t,
    pub error: __wasi_errno_t,
    pub type_: __wasi_eventtype_t,
    pub __bindgen_padding_0: u32,
    pub __bindgen_anon_1: __wasi_event_t__bindgen_ty_1,
}
#[allow(non_snake_case)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union __wasi_event_t__bindgen_ty_1 {
    pub fd_readwrite: __wasi_event_t__bindgen_ty_1__bindgen_ty_1,
    pub proc_terminate: __wasi_event_t__bindgen_ty_1__bindgen_ty_2,
    _bindgen_union_align: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_event_t__bindgen_ty_1__bindgen_ty_1 {
    pub nbytes: __wasi_filesize_t,
    pub unused: [i8; 4usize],
    pub flags: __wasi_eventrwflags_t,
    pub __bindgen_padding_0: u16,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_event_t__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_event_t__bindgen_ty_1__bindgen_ty_1>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(__wasi_event_t__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_event_t__bindgen_ty_1__bindgen_ty_1>())).nbytes
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_event_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(nbytes)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_event_t__bindgen_ty_1__bindgen_ty_1>())).unused
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_event_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(unused)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_event_t__bindgen_ty_1__bindgen_ty_1>())).flags as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_event_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_event_t__bindgen_ty_1__bindgen_ty_2 {
    pub unused: [i8; 4usize],
    pub signal: __wasi_signal_t,
    pub exitcode: __wasi_exitcode_t,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_event_t__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_event_t__bindgen_ty_1__bindgen_ty_2>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(__wasi_event_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_event_t__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__wasi_event_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_event_t__bindgen_ty_1__bindgen_ty_2>())).unused
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_event_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(unused)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_event_t__bindgen_ty_1__bindgen_ty_2>())).signal
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_event_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(signal)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_event_t__bindgen_ty_1__bindgen_ty_2>())).exitcode
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_event_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(exitcode)
        )
    );
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_event_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_event_t__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(__wasi_event_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_event_t__bindgen_ty_1>())).fd_readwrite as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_event_t__bindgen_ty_1),
            "::",
            stringify!(fd_readwrite)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_event_t__bindgen_ty_1>())).proc_terminate as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_event_t__bindgen_ty_1),
            "::",
            stringify!(proc_terminate)
        )
    );
}
#[test]
fn bindgen_test_layout_wasi_event_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_event_t>(),
        32usize,
        concat!("Size of: ", stringify!(__wasi_event_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_event_t>())).userdata as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_event_t),
            "::",
            stringify!(userdata)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_event_t>())).error as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_event_t),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_event_t>())).type_ as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_event_t),
            "::",
            stringify!(type_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_fdstat_t {
    pub fs_filetype: __wasi_filetype_t,
    pub fs_flags: __wasi_fdflags_t,
    pub __bindgen_padding_0: u32,
    pub fs_rights_base: __wasi_rights_t,
    pub fs_rights_inheriting: __wasi_rights_t,
}
#[test]
fn bindgen_test_layout_wasi_fdstat_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_fdstat_t>(),
        24usize,
        concat!("Size of: ", stringify!(__wasi_fdstat_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_fdstat_t>())).fs_filetype as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_fdstat_t),
            "::",
            stringify!(fs_filetype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_fdstat_t>())).fs_flags as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_fdstat_t),
            "::",
            stringify!(fs_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_fdstat_t>())).fs_rights_base as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_fdstat_t),
            "::",
            stringify!(fs_rights_base)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_fdstat_t>())).fs_rights_inheriting as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_fdstat_t),
            "::",
            stringify!(fs_rights_inheriting)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_filestat_t {
    pub st_dev: __wasi_device_t,
    pub st_ino: __wasi_inode_t,
    pub st_filetype: __wasi_filetype_t,
    pub st_nlink: __wasi_linkcount_t,
    pub st_size: __wasi_filesize_t,
    pub st_atim: __wasi_timestamp_t,
    pub st_mtim: __wasi_timestamp_t,
    pub st_ctim: __wasi_timestamp_t,
}
#[test]
fn bindgen_test_layout_wasi_filestat_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_filestat_t>(),
        56usize,
        concat!("Size of: ", stringify!(__wasi_filestat_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_filestat_t>())).st_dev as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_filestat_t),
            "::",
            stringify!(st_dev)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_filestat_t>())).st_ino as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_filestat_t),
            "::",
            stringify!(st_ino)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_filestat_t>())).st_filetype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_filestat_t),
            "::",
            stringify!(st_filetype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_filestat_t>())).st_nlink as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_filestat_t),
            "::",
            stringify!(st_nlink)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_filestat_t>())).st_size as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_filestat_t),
            "::",
            stringify!(st_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_filestat_t>())).st_atim as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_filestat_t),
            "::",
            stringify!(st_atim)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_filestat_t>())).st_mtim as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_filestat_t),
            "::",
            stringify!(st_mtim)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_filestat_t>())).st_ctim as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_filestat_t),
            "::",
            stringify!(st_ctim)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_lookup_t {
    pub fd: __wasi_fd_t,
    pub flags: __wasi_lookupflags_t,
}
#[test]
fn bindgen_test_layout_wasi_lookup_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_lookup_t>(),
        8usize,
        concat!("Size of: ", stringify!(__wasi_lookup_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_lookup_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__wasi_lookup_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_lookup_t>())).fd as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_lookup_t),
            "::",
            stringify!(fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_lookup_t>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_lookup_t),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __wasi_auxv_t {
    pub a_type: __wasi_auxtype_t,
    pub __bindgen_anon_1: __wasi_auxv_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __wasi_auxv_t__bindgen_ty_1 {
    pub a_val: size_t,
    pub a_ptr: uintptr_t, // *mut ::std::os::raw::c_void
    _bindgen_union_align: u32,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_auxv_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_auxv_t__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(__wasi_auxv_t__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_auxv_t__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(__wasi_auxv_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_auxv_t__bindgen_ty_1>())).a_val as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_auxv_t__bindgen_ty_1),
            "::",
            stringify!(a_val)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_auxv_t__bindgen_ty_1>())).a_ptr as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_auxv_t__bindgen_ty_1),
            "::",
            stringify!(a_ptr)
        )
    );
}
#[test]
fn bindgen_test_layout_wasi_auxv_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_auxv_t>(),
        8usize,
        concat!("Size of: ", stringify!(__wasi_auxv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_auxv_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__wasi_auxv_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_auxv_t>())).a_type as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_auxv_t),
            "::",
            stringify!(a_type)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_ciovec_t {
    pub buf: uintptr_t, // *const ::std::os::raw::c_void
    pub buf_len: size_t,
}
#[test]
fn bindgen_test_layout_wasi_ciovec_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_ciovec_t>(),
        8usize,
        concat!("Size of: ", stringify!(__wasi_ciovec_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_ciovec_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__wasi_ciovec_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_ciovec_t>())).buf as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_ciovec_t),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_ciovec_t>())).buf_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_ciovec_t),
            "::",
            stringify!(buf_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_iovec_t {
    pub buf: uintptr_t, // *mut ::std::os::raw::c_void
    pub buf_len: size_t,
}
#[test]
fn bindgen_test_layout_wasi_iovec_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_iovec_t>(),
        8usize,
        concat!("Size of: ", stringify!(__wasi_iovec_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_iovec_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__wasi_iovec_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_iovec_t>())).buf as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_iovec_t),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_iovec_t>())).buf_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_iovec_t),
            "::",
            stringify!(buf_len)
        )
    );
}
// TODO: Figure out if we need this.
//pub type __wasi_processentry_t =
//    ::std::option::Option<
//        unsafe extern "C" fn(
//            auxv: uintptr_t, // *const __wasi_auxv_t
//        )
//    >;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_recv_in_t {
    pub ri_data: uintptr_t, // *const __wasi_iovec_t
    pub ri_data_len: size_t,
    pub ri_fds: uintptr_t, // *mut __wasi_fd_t
    pub ri_fds_len: size_t,
    pub ri_flags: __wasi_riflags_t,
}
#[test]
fn bindgen_test_layout_wasi_recv_in_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_recv_in_t>(),
        20usize,
        concat!("Size of: ", stringify!(__wasi_recv_in_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_recv_in_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__wasi_recv_in_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_recv_in_t>())).ri_data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_recv_in_t),
            "::",
            stringify!(ri_data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_recv_in_t>())).ri_data_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_recv_in_t),
            "::",
            stringify!(ri_data_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_recv_in_t>())).ri_fds as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_recv_in_t),
            "::",
            stringify!(ri_fds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_recv_in_t>())).ri_fds_len as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_recv_in_t),
            "::",
            stringify!(ri_fds_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_recv_in_t>())).ri_flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_recv_in_t),
            "::",
            stringify!(ri_flags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __wasi_recv_out_t {
    pub ro_datalen: size_t,
    pub ro_fdslen: size_t,
    pub ro_unused: [i8; 40usize],
    pub ro_flags: __wasi_roflags_t,
}
#[test]
fn bindgen_test_layout_wasi_recv_out_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_recv_out_t>(),
        52usize,
        concat!("Size of: ", stringify!(__wasi_recv_out_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_recv_out_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__wasi_recv_out_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_recv_out_t>())).ro_datalen as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_recv_out_t),
            "::",
            stringify!(ro_datalen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_recv_out_t>())).ro_fdslen as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_recv_out_t),
            "::",
            stringify!(ro_fdslen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_recv_out_t>())).ro_unused as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_recv_out_t),
            "::",
            stringify!(ro_unused)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_recv_out_t>())).ro_flags as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_recv_out_t),
            "::",
            stringify!(ro_flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_send_in_t {
    pub si_data: uintptr_t, // *const __wasi_ciovec_t
    pub si_data_len: size_t,
    pub si_fds: uintptr_t, // *const __wasi_fd_t
    pub si_fds_len: size_t,
    pub si_flags: __wasi_siflags_t,
}
#[test]
fn bindgen_test_layout_wasi_send_in_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_send_in_t>(),
        20usize,
        concat!("Size of: ", stringify!(__wasi_send_in_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_send_in_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__wasi_send_in_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_send_in_t>())).si_data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_send_in_t),
            "::",
            stringify!(si_data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_send_in_t>())).si_data_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_send_in_t),
            "::",
            stringify!(si_data_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_send_in_t>())).si_fds as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_send_in_t),
            "::",
            stringify!(si_fds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_send_in_t>())).si_fds_len as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_send_in_t),
            "::",
            stringify!(si_fds_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_send_in_t>())).si_flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_send_in_t),
            "::",
            stringify!(si_flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_send_out_t {
    pub so_datalen: size_t,
}
#[test]
fn bindgen_test_layout_wasi_send_out_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_send_out_t>(),
        4usize,
        concat!("Size of: ", stringify!(__wasi_send_out_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_send_out_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__wasi_send_out_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_send_out_t>())).so_datalen as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_send_out_t),
            "::",
            stringify!(so_datalen)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __wasi_subscription_t {
    pub userdata: __wasi_userdata_t,
    pub unused: u16,
    pub type_: __wasi_eventtype_t,
    pub __bindgen_padding_0: u32,
    pub __bindgen_anon_1: __wasi_subscription_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __wasi_subscription_t__bindgen_ty_1 {
    pub clock: __wasi_subscription_t__bindgen_ty_1__bindgen_ty_1,
    pub condvar: __wasi_subscription_t__bindgen_ty_1__bindgen_ty_2,
    pub fd_readwrite: __wasi_subscription_t__bindgen_ty_1__bindgen_ty_3,
    pub lock: __wasi_subscription_t__bindgen_ty_1__bindgen_ty_4,
    pub proc_terminate: __wasi_subscription_t__bindgen_ty_1__bindgen_ty_5,
    _bindgen_union_align: [u64; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_subscription_t__bindgen_ty_1__bindgen_ty_1 {
    pub identifier: __wasi_userdata_t,
    pub clock_id: __wasi_clockid_t,
    pub __bindgen_padding_0: u32,
    pub timeout: __wasi_timestamp_t,
    pub precision: __wasi_timestamp_t,
    pub flags: __wasi_subclockflags_t,
    pub __bindgen_padding_1: [u16; 3usize],
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_1>(),
        40usize,
        concat!(
            "Size of: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_1>())).identifier
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(identifier)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_1>())).clock_id
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(clock_id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_1>())).timeout
                as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(timeout)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_1>())).precision
                as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(precision)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_1>())).flags
                as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_subscription_t__bindgen_ty_1__bindgen_ty_2 {
    pub condvar: uintptr_t, // *mut __wasi_condvar_t
    pub lock: uintptr_t,    // *mut __wasi_lock_t
    pub condvar_scope: __wasi_scope_t,
    pub lock_scope: __wasi_scope_t,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_2>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_2>())).condvar
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(condvar)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_2>())).lock
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(lock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_2>()))
                .condvar_scope as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(condvar_scope)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_2>())).lock_scope
                as *const _ as usize
        },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(lock_scope)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_subscription_t__bindgen_ty_1__bindgen_ty_3 {
    pub fd: __wasi_fd_t,
    pub flags: __wasi_subrwflags_t,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_3>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_3>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_3>())).fd
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(fd)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_3>())).flags
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_subscription_t__bindgen_ty_1__bindgen_ty_4 {
    pub lock: uintptr_t, // *mut __wasi_lock_t
    pub lock_scope: __wasi_scope_t,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t__bindgen_ty_1__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_4>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_4>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_4>())).lock
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(lock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_4>())).lock_scope
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(lock_scope)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_subscription_t__bindgen_ty_1__bindgen_ty_5 {
    pub fd: __wasi_fd_t,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t__bindgen_ty_1__bindgen_ty_5() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_5>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_5>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1__bindgen_ty_5>())).fd
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1__bindgen_ty_5),
            "::",
            stringify!(fd)
        )
    );
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_subscription_t__bindgen_ty_1>(),
        40usize,
        concat!("Size of: ", stringify!(__wasi_subscription_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1>())).clock as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(clock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1>())).condvar as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(condvar)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1>())).fd_readwrite as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(fd_readwrite)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1>())).lock as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(lock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__wasi_subscription_t__bindgen_ty_1>())).proc_terminate
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(proc_terminate)
        )
    );
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_subscription_t>(),
        56usize,
        concat!("Size of: ", stringify!(__wasi_subscription_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_subscription_t>())).userdata as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t),
            "::",
            stringify!(userdata)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_subscription_t>())).unused as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t),
            "::",
            stringify!(unused)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_subscription_t>())).type_ as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_subscription_t),
            "::",
            stringify!(type_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_tcb_t {
    pub parent: uintptr_t, // *mut ::std::os::raw::c_void
}
#[test]
fn bindgen_test_layout_wasi_tcb_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_tcb_t>(),
        4usize,
        concat!("Size of: ", stringify!(__wasi_tcb_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_tcb_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__wasi_tcb_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_tcb_t>())).parent as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_tcb_t),
            "::",
            stringify!(parent)
        )
    );
}
pub type __wasi_threadentry_t = uintptr_t;
//pub type __wasi_threadentry_t = ::std::option::Option<
//    unsafe extern "C" fn(
//        tid: __wasi_tid_t,
//        aux: uintptr_t, // *mut ::std::os::raw::c_void
//    ),
//>;
#[test]
fn bindgen_test_layout_wasi_threadentry_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_threadentry_t>(),
        4usize,
        concat!("Size of: ", stringify!(__wasi_threadentry_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_threadentry_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__wasi_threadentry_t))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_threadattr_t {
    pub entry_point: __wasi_threadentry_t,
    pub stack: uintptr_t, // *mut ::std::os::raw::c_void
    pub stack_len: size_t,
    pub argument: uintptr_t, // *mut ::std::os::raw::c_void
}
#[test]
fn bindgen_test_layout_wasi_threadattr_t() {
    assert_eq!(
        ::std::mem::size_of::<__wasi_threadattr_t>(),
        16usize,
        concat!("Size of: ", stringify!(__wasi_threadattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__wasi_threadattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__wasi_threadattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_threadattr_t>())).entry_point as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_threadattr_t),
            "::",
            stringify!(entry_point)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_threadattr_t>())).stack as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_threadattr_t),
            "::",
            stringify!(stack)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_threadattr_t>())).stack_len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_threadattr_t),
            "::",
            stringify!(stack_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__wasi_threadattr_t>())).argument as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__wasi_threadattr_t),
            "::",
            stringify!(argument)
        )
    );
}

pub fn strerror(errno: __wasi_errno_t) -> &'static str {
    match errno {
        __WASI_ESUCCESS => "__WASI_ESUCCESS",
        __WASI_E2BIG => "__WASI_E2BIG",
        __WASI_EACCES => "__WASI_EACCES",
        __WASI_EADDRINUSE => "__WASI_EADDRINUSE",
        __WASI_EADDRNOTAVAIL => "__WASI_EADDRNOTAVAIL",
        __WASI_EAFNOSUPPORT => "__WASI_EAFNOSUPPORT",
        __WASI_EAGAIN => "__WASI_EAGAIN",
        __WASI_EALREADY => "__WASI_EALREADY",
        __WASI_EBADF => "__WASI_EBADF",
        __WASI_EBADMSG => "__WASI_EBADMSG",
        __WASI_EBUSY => "__WASI_EBUSY",
        __WASI_ECANCELED => "__WASI_ECANCELED",
        __WASI_ECHILD => "__WASI_ECHILD",
        __WASI_ECONNABORTED => "__WASI_ECONNABORTED",
        __WASI_ECONNREFUSED => "__WASI_ECONNREFUSED",
        __WASI_ECONNRESET => "__WASI_ECONNRESET",
        __WASI_EDEADLK => "__WASI_EDEADLK",
        __WASI_EDESTADDRREQ => "__WASI_EDESTADDRREQ",
        __WASI_EDOM => "__WASI_EDOM",
        __WASI_EDQUOT => "__WASI_EDQUOT",
        __WASI_EEXIST => "__WASI_EEXIST",
        __WASI_EFAULT => "__WASI_EFAULT",
        __WASI_EFBIG => "__WASI_EFBIG",
        __WASI_EHOSTUNREACH => "__WASI_EHOSTUNREACH",
        __WASI_EIDRM => "__WASI_EIDRM",
        __WASI_EILSEQ => "__WASI_EILSEQ",
        __WASI_EINPROGRESS => "__WASI_EINPROGRESS",
        __WASI_EINTR => "__WASI_EINTR",
        __WASI_EINVAL => "__WASI_EINVAL",
        __WASI_EIO => "__WASI_EIO",
        __WASI_EISCONN => "__WASI_EISCONN",
        __WASI_EISDIR => "__WASI_EISDIR",
        __WASI_ELOOP => "__WASI_ELOOP",
        __WASI_EMFILE => "__WASI_EMFILE",
        __WASI_EMLINK => "__WASI_EMLINK",
        __WASI_EMSGSIZE => "__WASI_EMSGSIZE",
        __WASI_EMULTIHOP => "__WASI_EMULTIHOP",
        __WASI_ENAMETOOLONG => "__WASI_ENAMETOOLONG",
        __WASI_ENETDOWN => "__WASI_ENETDOWN",
        __WASI_ENETRESET => "__WASI_ENETRESET",
        __WASI_ENETUNREACH => "__WASI_ENETUNREACH",
        __WASI_ENFILE => "__WASI_ENFILE",
        __WASI_ENOBUFS => "__WASI_ENOBUFS",
        __WASI_ENODEV => "__WASI_ENODEV",
        __WASI_ENOENT => "__WASI_ENOENT",
        __WASI_ENOEXEC => "__WASI_ENOEXEC",
        __WASI_ENOLCK => "__WASI_ENOLCK",
        __WASI_ENOLINK => "__WASI_ENOLINK",
        __WASI_ENOMEM => "__WASI_ENOMEM",
        __WASI_ENOMSG => "__WASI_ENOMSG",
        __WASI_ENOPROTOOPT => "__WASI_ENOPROTOOPT",
        __WASI_ENOSPC => "__WASI_ENOSPC",
        __WASI_ENOSYS => "__WASI_ENOSYS",
        __WASI_ENOTCONN => "__WASI_ENOTCONN",
        __WASI_ENOTDIR => "__WASI_ENOTDIR",
        __WASI_ENOTEMPTY => "__WASI_ENOTEMPTY",
        __WASI_ENOTRECOVERABLE => "__WASI_ENOTRECOVERABLE",
        __WASI_ENOTSOCK => "__WASI_ENOTSOCK",
        __WASI_ENOTSUP => "__WASI_ENOTSUP",
        __WASI_ENOTTY => "__WASI_ENOTTY",
        __WASI_ENXIO => "__WASI_ENXIO",
        __WASI_EOVERFLOW => "__WASI_EOVERFLOW",
        __WASI_EOWNERDEAD => "__WASI_EOWNERDEAD",
        __WASI_EPERM => "__WASI_EPERM",
        __WASI_EPIPE => "__WASI_EPIPE",
        __WASI_EPROTO => "__WASI_EPROTO",
        __WASI_EPROTONOSUPPORT => "__WASI_EPROTONOSUPPORT",
        __WASI_EPROTOTYPE => "__WASI_EPROTOTYPE",
        __WASI_ERANGE => "__WASI_ERANGE",
        __WASI_EROFS => "__WASI_EROFS",
        __WASI_ESPIPE => "__WASI_ESPIPE",
        __WASI_ESRCH => "__WASI_ESRCH",
        __WASI_ESTALE => "__WASI_ESTALE",
        __WASI_ETIMEDOUT => "__WASI_ETIMEDOUT",
        __WASI_ETXTBSY => "__WASI_ETXTBSY",
        __WASI_EXDEV => "__WASI_EXDEV",
        __WASI_ENOTCAPABLE => "__WASI_ENOTCAPABLE",
        other => panic!("Undefined errno value {:?}", other),
    }
}

pub fn whence_to_str(whence: __wasi_whence_t) -> &'static str {
    match whence {
        __WASI_WHENCE_CUR => "__WASI_WHENCE_CUR",
        __WASI_WHENCE_END => "__WASI_WHENCE_END",
        __WASI_WHENCE_SET => "__WASI_WHENCE_SET",
        other => panic!("Undefined whence value {:?}", other),
    }
}
