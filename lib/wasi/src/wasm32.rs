//! WASI types as defined in wasm32. This file was originally generated
//! by running bindgen over wasi_types.h with a wasm32 target, and the
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
pub const WASI_ADVICE_DONTNEED: u32 = 1;
pub const WASI_ADVICE_NOREUSE: u32 = 2;
pub const WASI_ADVICE_NORMAL: u32 = 3;
pub const WASI_ADVICE_RANDOM: u32 = 4;
pub const WASI_ADVICE_SEQUENTIAL: u32 = 5;
pub const WASI_ADVICE_WILLNEED: u32 = 6;
pub const WASI_AT_ARGDATA: u32 = 256;
pub const WASI_AT_ARGDATALEN: u32 = 257;
pub const WASI_AT_BASE: u32 = 7;
pub const WASI_AT_CANARY: u32 = 258;
pub const WASI_AT_CANARYLEN: u32 = 259;
pub const WASI_AT_NCPUS: u32 = 260;
pub const WASI_AT_NULL: u32 = 0;
pub const WASI_AT_PAGESZ: u32 = 6;
pub const WASI_AT_PHDR: u32 = 3;
pub const WASI_AT_PHNUM: u32 = 4;
pub const WASI_AT_PID: u32 = 263;
pub const WASI_AT_SYSINFO_EHDR: u32 = 262;
pub const WASI_AT_TID: u32 = 261;
pub const WASI_CLOCK_MONOTONIC: u32 = 1;
pub const WASI_CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
pub const WASI_CLOCK_REALTIME: u32 = 3;
pub const WASI_CLOCK_THREAD_CPUTIME_ID: u32 = 4;
pub const WASI_CONDVAR_HAS_NO_WAITERS: u32 = 0;
pub const WASI_DIRCOOKIE_START: u32 = 0;
pub const WASI_ESUCCESS: wasi_errno_t = 0;
pub const WASI_E2BIG: wasi_errno_t = 1;
pub const WASI_EACCES: wasi_errno_t = 2;
pub const WASI_EADDRINUSE: wasi_errno_t = 3;
pub const WASI_EADDRNOTAVAIL: wasi_errno_t = 4;
pub const WASI_EAFNOSUPPORT: wasi_errno_t = 5;
pub const WASI_EAGAIN: wasi_errno_t = 6;
pub const WASI_EALREADY: wasi_errno_t = 7;
pub const WASI_EBADF: wasi_errno_t = 8;
pub const WASI_EBADMSG: wasi_errno_t = 9;
pub const WASI_EBUSY: wasi_errno_t = 10;
pub const WASI_ECANCELED: wasi_errno_t = 11;
pub const WASI_ECHILD: wasi_errno_t = 12;
pub const WASI_ECONNABORTED: wasi_errno_t = 13;
pub const WASI_ECONNREFUSED: wasi_errno_t = 14;
pub const WASI_ECONNRESET: wasi_errno_t = 15;
pub const WASI_EDEADLK: wasi_errno_t = 16;
pub const WASI_EDESTADDRREQ: wasi_errno_t = 17;
pub const WASI_EDOM: wasi_errno_t = 18;
pub const WASI_EDQUOT: wasi_errno_t = 19;
pub const WASI_EEXIST: wasi_errno_t = 20;
pub const WASI_EFAULT: wasi_errno_t = 21;
pub const WASI_EFBIG: wasi_errno_t = 22;
pub const WASI_EHOSTUNREACH: wasi_errno_t = 23;
pub const WASI_EIDRM: wasi_errno_t = 24;
pub const WASI_EILSEQ: wasi_errno_t = 25;
pub const WASI_EINPROGRESS: wasi_errno_t = 26;
pub const WASI_EINTR: wasi_errno_t = 27;
pub const WASI_EINVAL: wasi_errno_t = 28;
pub const WASI_EIO: wasi_errno_t = 29;
pub const WASI_EISCONN: wasi_errno_t = 30;
pub const WASI_EISDIR: wasi_errno_t = 31;
pub const WASI_ELOOP: wasi_errno_t = 32;
pub const WASI_EMFILE: wasi_errno_t = 33;
pub const WASI_EMLINK: wasi_errno_t = 34;
pub const WASI_EMSGSIZE: wasi_errno_t = 35;
pub const WASI_EMULTIHOP: wasi_errno_t = 36;
pub const WASI_ENAMETOOLONG: wasi_errno_t = 37;
pub const WASI_ENETDOWN: wasi_errno_t = 38;
pub const WASI_ENETRESET: wasi_errno_t = 39;
pub const WASI_ENETUNREACH: wasi_errno_t = 40;
pub const WASI_ENFILE: wasi_errno_t = 41;
pub const WASI_ENOBUFS: wasi_errno_t = 42;
pub const WASI_ENODEV: wasi_errno_t = 43;
pub const WASI_ENOENT: wasi_errno_t = 44;
pub const WASI_ENOEXEC: wasi_errno_t = 45;
pub const WASI_ENOLCK: wasi_errno_t = 46;
pub const WASI_ENOLINK: wasi_errno_t = 47;
pub const WASI_ENOMEM: wasi_errno_t = 48;
pub const WASI_ENOMSG: wasi_errno_t = 49;
pub const WASI_ENOPROTOOPT: wasi_errno_t = 50;
pub const WASI_ENOSPC: wasi_errno_t = 51;
pub const WASI_ENOSYS: wasi_errno_t = 52;
pub const WASI_ENOTCONN: wasi_errno_t = 53;
pub const WASI_ENOTDIR: wasi_errno_t = 54;
pub const WASI_ENOTEMPTY: wasi_errno_t = 55;
pub const WASI_ENOTRECOVERABLE: wasi_errno_t = 56;
pub const WASI_ENOTSOCK: wasi_errno_t = 57;
pub const WASI_ENOTSUP: wasi_errno_t = 58;
pub const WASI_ENOTTY: wasi_errno_t = 59;
pub const WASI_ENXIO: wasi_errno_t = 60;
pub const WASI_EOVERFLOW: wasi_errno_t = 61;
pub const WASI_EOWNERDEAD: wasi_errno_t = 62;
pub const WASI_EPERM: wasi_errno_t = 63;
pub const WASI_EPIPE: wasi_errno_t = 64;
pub const WASI_EPROTO: wasi_errno_t = 65;
pub const WASI_EPROTONOSUPPORT: wasi_errno_t = 66;
pub const WASI_EPROTOTYPE: wasi_errno_t = 67;
pub const WASI_ERANGE: wasi_errno_t = 68;
pub const WASI_EROFS: wasi_errno_t = 69;
pub const WASI_ESPIPE: wasi_errno_t = 70;
pub const WASI_ESRCH: wasi_errno_t = 71;
pub const WASI_ESTALE: wasi_errno_t = 72;
pub const WASI_ETIMEDOUT: wasi_errno_t = 73;
pub const WASI_ETXTBSY: wasi_errno_t = 74;
pub const WASI_EXDEV: wasi_errno_t = 75;
pub const WASI_ENOTCAPABLE: wasi_errno_t = 76;
pub const WASI_EVENT_FD_READWRITE_HANGUP: u32 = 1;
pub const WASI_EVENTTYPE_CLOCK: u32 = 1;
pub const WASI_EVENTTYPE_CONDVAR: u32 = 2;
pub const WASI_EVENTTYPE_FD_READ: u32 = 3;
pub const WASI_EVENTTYPE_FD_WRITE: u32 = 4;
pub const WASI_EVENTTYPE_LOCK_RDLOCK: u32 = 5;
pub const WASI_EVENTTYPE_LOCK_WRLOCK: u32 = 6;
pub const WASI_EVENTTYPE_PROC_TERMINATE: u32 = 7;
pub const WASI_PROCESS_CHILD: u32 = 4294967295;
pub const WASI_MAP_ANON_FD: u32 = 4294967295;
pub const WASI_FDFLAG_APPEND: u32 = 1;
pub const WASI_FDFLAG_DSYNC: u32 = 2;
pub const WASI_FDFLAG_NONBLOCK: u32 = 4;
pub const WASI_FDFLAG_RSYNC: u32 = 8;
pub const WASI_FDFLAG_SYNC: u32 = 16;
pub const WASI_FDSTAT_FLAGS: u32 = 1;
pub const WASI_FDSTAT_RIGHTS: u32 = 2;
pub const WASI_FILETYPE_UNKNOWN: u32 = 0;
pub const WASI_FILETYPE_BLOCK_DEVICE: u32 = 16;
pub const WASI_FILETYPE_CHARACTER_DEVICE: u32 = 17;
pub const WASI_FILETYPE_DIRECTORY: u32 = 32;
pub const WASI_FILETYPE_PROCESS: u32 = 80;
pub const WASI_FILETYPE_REGULAR_FILE: u32 = 96;
pub const WASI_FILETYPE_SHARED_MEMORY: u32 = 112;
pub const WASI_FILETYPE_SOCKET_DGRAM: u32 = 128;
pub const WASI_FILETYPE_SOCKET_STREAM: u32 = 130;
pub const WASI_FILETYPE_SYMBOLIC_LINK: u32 = 144;
pub const WASI_FILESTAT_ATIM: u32 = 1;
pub const WASI_FILESTAT_ATIM_NOW: u32 = 2;
pub const WASI_FILESTAT_MTIM: u32 = 4;
pub const WASI_FILESTAT_MTIM_NOW: u32 = 8;
pub const WASI_FILESTAT_SIZE: u32 = 16;
pub const WASI_LOCK_UNLOCKED: u32 = 0;
pub const WASI_LOCK_WRLOCKED: u32 = 1073741824;
pub const WASI_LOCK_KERNEL_MANAGED: u32 = 2147483648;
pub const WASI_LOCK_BOGUS: u32 = 2147483648;
pub const WASI_LOOKUP_SYMLINK_FOLLOW: u32 = 1;
pub const WASI_MAP_ANON: u32 = 1;
pub const WASI_MAP_FIXED: u32 = 2;
pub const WASI_MAP_PRIVATE: u32 = 4;
pub const WASI_MAP_SHARED: u32 = 8;
pub const WASI_PROT_EXEC: u32 = 1;
pub const WASI_PROT_WRITE: u32 = 2;
pub const WASI_PROT_READ: u32 = 4;
pub const WASI_MS_ASYNC: u32 = 1;
pub const WASI_MS_INVALIDATE: u32 = 2;
pub const WASI_MS_SYNC: u32 = 4;
pub const WASI_O_CREAT: u32 = 1;
pub const WASI_O_DIRECTORY: u32 = 2;
pub const WASI_O_EXCL: u32 = 4;
pub const WASI_O_TRUNC: u32 = 8;
pub const WASI_SOCK_RECV_PEEK: u32 = 4;
pub const WASI_SOCK_RECV_WAITALL: u32 = 16;
pub const WASI_RIGHT_FD_DATASYNC: u32 = 1;
pub const WASI_RIGHT_FD_READ: u32 = 2;
pub const WASI_RIGHT_FD_SEEK: u32 = 4;
pub const WASI_RIGHT_FD_STAT_PUT_FLAGS: u32 = 8;
pub const WASI_RIGHT_FD_SYNC: u32 = 16;
pub const WASI_RIGHT_FD_TELL: u32 = 32;
pub const WASI_RIGHT_FD_WRITE: u32 = 64;
pub const WASI_RIGHT_FILE_ADVISE: u32 = 128;
pub const WASI_RIGHT_FILE_ALLOCATE: u32 = 256;
pub const WASI_RIGHT_FILE_CREATE_DIRECTORY: u32 = 512;
pub const WASI_RIGHT_FILE_CREATE_FILE: u32 = 1024;
pub const WASI_RIGHT_FILE_LINK_SOURCE: u32 = 4096;
pub const WASI_RIGHT_FILE_LINK_TARGET: u32 = 8192;
pub const WASI_RIGHT_FILE_OPEN: u32 = 16384;
pub const WASI_RIGHT_FILE_READDIR: u32 = 32768;
pub const WASI_RIGHT_FILE_READLINK: u32 = 65536;
pub const WASI_RIGHT_FILE_RENAME_SOURCE: u32 = 131072;
pub const WASI_RIGHT_FILE_RENAME_TARGET: u32 = 262144;
pub const WASI_RIGHT_FILE_STAT_FGET: u32 = 524288;
pub const WASI_RIGHT_FILE_STAT_FPUT_SIZE: u32 = 1048576;
pub const WASI_RIGHT_FILE_STAT_FPUT_TIMES: u32 = 2097152;
pub const WASI_RIGHT_FILE_STAT_GET: u32 = 4194304;
pub const WASI_RIGHT_FILE_STAT_PUT_TIMES: u32 = 8388608;
pub const WASI_RIGHT_FILE_SYMLINK: u32 = 16777216;
pub const WASI_RIGHT_FILE_UNLINK: u32 = 33554432;
pub const WASI_RIGHT_MEM_MAP: u32 = 67108864;
pub const WASI_RIGHT_MEM_MAP_EXEC: u32 = 134217728;
pub const WASI_RIGHT_POLL_FD_READWRITE: u32 = 268435456;
pub const WASI_RIGHT_POLL_PROC_TERMINATE: u32 = 1073741824;
pub const WASI_RIGHT_PROC_EXEC: u64 = 4294967296;
pub const WASI_RIGHT_SOCK_SHUTDOWN: u64 = 549755813888;
pub const WASI_SOCK_RECV_FDS_TRUNCATED: u32 = 1;
pub const WASI_SOCK_RECV_DATA_TRUNCATED: u32 = 8;
pub const WASI_SCOPE_PRIVATE: u32 = 4;
pub const WASI_SCOPE_SHARED: u32 = 8;
pub const WASI_SHUT_RD: u32 = 1;
pub const WASI_SHUT_WR: u32 = 2;
pub const WASI_SIGABRT: u32 = 1;
pub const WASI_SIGALRM: u32 = 2;
pub const WASI_SIGBUS: u32 = 3;
pub const WASI_SIGCHLD: u32 = 4;
pub const WASI_SIGCONT: u32 = 5;
pub const WASI_SIGFPE: u32 = 6;
pub const WASI_SIGHUP: u32 = 7;
pub const WASI_SIGILL: u32 = 8;
pub const WASI_SIGINT: u32 = 9;
pub const WASI_SIGKILL: u32 = 10;
pub const WASI_SIGPIPE: u32 = 11;
pub const WASI_SIGQUIT: u32 = 12;
pub const WASI_SIGSEGV: u32 = 13;
pub const WASI_SIGSTOP: u32 = 14;
pub const WASI_SIGSYS: u32 = 15;
pub const WASI_SIGTERM: u32 = 16;
pub const WASI_SIGTRAP: u32 = 17;
pub const WASI_SIGTSTP: u32 = 18;
pub const WASI_SIGTTIN: u32 = 19;
pub const WASI_SIGTTOU: u32 = 20;
pub const WASI_SIGURG: u32 = 21;
pub const WASI_SIGUSR1: u32 = 22;
pub const WASI_SIGUSR2: u32 = 23;
pub const WASI_SIGVTALRM: u32 = 24;
pub const WASI_SIGXCPU: u32 = 25;
pub const WASI_SIGXFSZ: u32 = 26;
pub const WASI_SUBSCRIPTION_CLOCK_ABSTIME: u32 = 1;
pub const WASI_SUBSCRIPTION_FD_READWRITE_POLL: u32 = 1;
pub const WASI_UNLINK_REMOVEDIR: u32 = 1;
pub const WASI_WHENCE_CUR: wasi_whence_t = 1;
pub const WASI_WHENCE_END: wasi_whence_t = 2;
pub const WASI_WHENCE_SET: wasi_whence_t = 3;
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
pub type wasi_advice_t = u8;
pub type wasi_auxtype_t = u32;
pub type wasi_clockid_t = u32;
pub type wasi_condvar_t = u32;
pub type wasi_device_t = u64;
pub type wasi_dircookie_t = u64;
pub type wasi_errno_t = u16;
pub type wasi_eventrwflags_t = u16;
pub type wasi_eventtype_t = u8;
pub type wasi_exitcode_t = u32;
pub type wasi_fd_t = u32;
pub type wasi_fdflags_t = u16;
pub type wasi_fdsflags_t = u16;
pub type wasi_filedelta_t = i64;
pub type wasi_filesize_t = u64;
pub type wasi_filetype_t = u8;
pub type wasi_fsflags_t = u16;
pub type wasi_inode_t = u64;
pub type wasi_linkcount_t = u32;
pub type wasi_lock_t = u32;
pub type wasi_lookupflags_t = u32;
pub type wasi_mflags_t = u8;
pub type wasi_mprot_t = u8;
pub type wasi_msflags_t = u8;
pub type wasi_nthreads_t = u32;
pub type wasi_oflags_t = u16;
pub type wasi_riflags_t = u16;
pub type wasi_rights_t = u64;
pub type wasi_roflags_t = u16;
pub type wasi_scope_t = u8;
pub type wasi_sdflags_t = u8;
pub type wasi_siflags_t = u16;
pub type wasi_signal_t = u8;
pub type wasi_subclockflags_t = u16;
pub type wasi_subrwflags_t = u16;
pub type wasi_tid_t = u32;
pub type wasi_timestamp_t = u64;
pub type wasi_ulflags_t = u8;
pub type wasi_userdata_t = u64;
pub type wasi_whence_t = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_dirent_t {
    pub d_next: wasi_dircookie_t,
    pub d_ino: wasi_inode_t,
    pub d_namlen: u32,
    pub d_type: wasi_filetype_t,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_wasi_dirent_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_dirent_t>(),
        24usize,
        concat!("Size of: ", stringify!(wasi_dirent_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_dirent_t>())).d_next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_dirent_t),
            "::",
            stringify!(d_next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_dirent_t>())).d_ino as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_dirent_t),
            "::",
            stringify!(d_ino)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_dirent_t>())).d_namlen as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_dirent_t),
            "::",
            stringify!(d_namlen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_dirent_t>())).d_type as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_dirent_t),
            "::",
            stringify!(d_type)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wasi_event_t {
    pub userdata: wasi_userdata_t,
    pub error: wasi_errno_t,
    pub type_: wasi_eventtype_t,
    pub __bindgen_padding_0: u32,
    pub __bindgen_anon_1: wasi_event_t__bindgen_ty_1,
}
#[allow(non_snake_case)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union wasi_event_t__bindgen_ty_1 {
    pub fd_readwrite: wasi_event_t__bindgen_ty_1__bindgen_ty_1,
    pub proc_terminate: wasi_event_t__bindgen_ty_1__bindgen_ty_2,
    _bindgen_union_align: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_event_t__bindgen_ty_1__bindgen_ty_1 {
    pub nbytes: wasi_filesize_t,
    pub unused: [i8; 4usize],
    pub flags: wasi_eventrwflags_t,
    pub __bindgen_padding_0: u16,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_event_t__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<wasi_event_t__bindgen_ty_1__bindgen_ty_1>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(wasi_event_t__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_event_t__bindgen_ty_1__bindgen_ty_1>())).nbytes as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_event_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(nbytes)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_event_t__bindgen_ty_1__bindgen_ty_1>())).unused as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_event_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(unused)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_event_t__bindgen_ty_1__bindgen_ty_1>())).flags as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_event_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_event_t__bindgen_ty_1__bindgen_ty_2 {
    pub unused: [i8; 4usize],
    pub signal: wasi_signal_t,
    pub exitcode: wasi_exitcode_t,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_event_t__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<wasi_event_t__bindgen_ty_1__bindgen_ty_2>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(wasi_event_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_event_t__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(wasi_event_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_event_t__bindgen_ty_1__bindgen_ty_2>())).unused as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_event_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(unused)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_event_t__bindgen_ty_1__bindgen_ty_2>())).signal as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_event_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(signal)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_event_t__bindgen_ty_1__bindgen_ty_2>())).exitcode
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_event_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(exitcode)
        )
    );
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_event_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<wasi_event_t__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(wasi_event_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_event_t__bindgen_ty_1>())).fd_readwrite as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_event_t__bindgen_ty_1),
            "::",
            stringify!(fd_readwrite)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_event_t__bindgen_ty_1>())).proc_terminate as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_event_t__bindgen_ty_1),
            "::",
            stringify!(proc_terminate)
        )
    );
}
#[test]
fn bindgen_test_layout_wasi_event_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_event_t>(),
        32usize,
        concat!("Size of: ", stringify!(wasi_event_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_event_t>())).userdata as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_event_t),
            "::",
            stringify!(userdata)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_event_t>())).error as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_event_t),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_event_t>())).type_ as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_event_t),
            "::",
            stringify!(type_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_fdstat_t {
    pub fs_filetype: wasi_filetype_t,
    pub fs_flags: wasi_fdflags_t,
    pub __bindgen_padding_0: u32,
    pub fs_rights_base: wasi_rights_t,
    pub fs_rights_inheriting: wasi_rights_t,
}
#[test]
fn bindgen_test_layout_wasi_fdstat_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_fdstat_t>(),
        24usize,
        concat!("Size of: ", stringify!(wasi_fdstat_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_fdstat_t>())).fs_filetype as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_fdstat_t),
            "::",
            stringify!(fs_filetype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_fdstat_t>())).fs_flags as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_fdstat_t),
            "::",
            stringify!(fs_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_fdstat_t>())).fs_rights_base as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_fdstat_t),
            "::",
            stringify!(fs_rights_base)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_fdstat_t>())).fs_rights_inheriting as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_fdstat_t),
            "::",
            stringify!(fs_rights_inheriting)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_filestat_t {
    pub st_dev: wasi_device_t,
    pub st_ino: wasi_inode_t,
    pub st_filetype: wasi_filetype_t,
    pub st_nlink: wasi_linkcount_t,
    pub st_size: wasi_filesize_t,
    pub st_atim: wasi_timestamp_t,
    pub st_mtim: wasi_timestamp_t,
    pub st_ctim: wasi_timestamp_t,
}
#[test]
fn bindgen_test_layout_wasi_filestat_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_filestat_t>(),
        56usize,
        concat!("Size of: ", stringify!(wasi_filestat_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_filestat_t>())).st_dev as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_filestat_t),
            "::",
            stringify!(st_dev)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_filestat_t>())).st_ino as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_filestat_t),
            "::",
            stringify!(st_ino)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_filestat_t>())).st_filetype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_filestat_t),
            "::",
            stringify!(st_filetype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_filestat_t>())).st_nlink as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_filestat_t),
            "::",
            stringify!(st_nlink)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_filestat_t>())).st_size as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_filestat_t),
            "::",
            stringify!(st_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_filestat_t>())).st_atim as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_filestat_t),
            "::",
            stringify!(st_atim)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_filestat_t>())).st_mtim as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_filestat_t),
            "::",
            stringify!(st_mtim)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_filestat_t>())).st_ctim as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_filestat_t),
            "::",
            stringify!(st_ctim)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_lookup_t {
    pub fd: wasi_fd_t,
    pub flags: wasi_lookupflags_t,
}
#[test]
fn bindgen_test_layout_wasi_lookup_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_lookup_t>(),
        8usize,
        concat!("Size of: ", stringify!(wasi_lookup_t))
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_lookup_t>(),
        4usize,
        concat!("Alignment of ", stringify!(wasi_lookup_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_lookup_t>())).fd as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_lookup_t),
            "::",
            stringify!(fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_lookup_t>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_lookup_t),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wasi_auxv_t {
    pub a_type: wasi_auxtype_t,
    pub __bindgen_anon_1: wasi_auxv_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union wasi_auxv_t__bindgen_ty_1 {
    pub a_val: size_t,
    pub a_ptr: uintptr_t, // *mut ::std::os::raw::c_void
    _bindgen_union_align: u32,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_auxv_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<wasi_auxv_t__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(wasi_auxv_t__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_auxv_t__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(wasi_auxv_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_auxv_t__bindgen_ty_1>())).a_val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_auxv_t__bindgen_ty_1),
            "::",
            stringify!(a_val)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_auxv_t__bindgen_ty_1>())).a_ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_auxv_t__bindgen_ty_1),
            "::",
            stringify!(a_ptr)
        )
    );
}
#[test]
fn bindgen_test_layout_wasi_auxv_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_auxv_t>(),
        8usize,
        concat!("Size of: ", stringify!(wasi_auxv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_auxv_t>(),
        4usize,
        concat!("Alignment of ", stringify!(wasi_auxv_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_auxv_t>())).a_type as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_auxv_t),
            "::",
            stringify!(a_type)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_ciovec_t {
    pub buf: uintptr_t, // *const ::std::os::raw::c_void
    pub buf_len: size_t,
}
#[test]
fn bindgen_test_layout_wasi_ciovec_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_ciovec_t>(),
        8usize,
        concat!("Size of: ", stringify!(wasi_ciovec_t))
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_ciovec_t>(),
        4usize,
        concat!("Alignment of ", stringify!(wasi_ciovec_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_ciovec_t>())).buf as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_ciovec_t),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_ciovec_t>())).buf_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_ciovec_t),
            "::",
            stringify!(buf_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_iovec_t {
    pub buf: uintptr_t, // *mut ::std::os::raw::c_void
    pub buf_len: size_t,
}
#[test]
fn bindgen_test_layout_wasi_iovec_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_iovec_t>(),
        8usize,
        concat!("Size of: ", stringify!(wasi_iovec_t))
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_iovec_t>(),
        4usize,
        concat!("Alignment of ", stringify!(wasi_iovec_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_iovec_t>())).buf as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_iovec_t),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_iovec_t>())).buf_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_iovec_t),
            "::",
            stringify!(buf_len)
        )
    );
}
// TODO: Figure out if we need this.
//pub type wasi_processentry_t =
//    ::std::option::Option<
//        unsafe extern "C" fn(
//            auxv: uintptr_t, // *const wasi_auxv_t
//        )
//    >;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_recv_in_t {
    pub ri_data: uintptr_t, // *const wasi_iovec_t
    pub ri_data_len: size_t,
    pub ri_fds: uintptr_t, // *mut wasi_fd_t
    pub ri_fds_len: size_t,
    pub ri_flags: wasi_riflags_t,
}
#[test]
fn bindgen_test_layout_wasi_recv_in_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_recv_in_t>(),
        20usize,
        concat!("Size of: ", stringify!(wasi_recv_in_t))
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_recv_in_t>(),
        4usize,
        concat!("Alignment of ", stringify!(wasi_recv_in_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_recv_in_t>())).ri_data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_recv_in_t),
            "::",
            stringify!(ri_data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_recv_in_t>())).ri_data_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_recv_in_t),
            "::",
            stringify!(ri_data_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_recv_in_t>())).ri_fds as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_recv_in_t),
            "::",
            stringify!(ri_fds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_recv_in_t>())).ri_fds_len as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_recv_in_t),
            "::",
            stringify!(ri_fds_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_recv_in_t>())).ri_flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_recv_in_t),
            "::",
            stringify!(ri_flags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wasi_recv_out_t {
    pub ro_datalen: size_t,
    pub ro_fdslen: size_t,
    pub ro_unused: [i8; 40usize],
    pub ro_flags: wasi_roflags_t,
}
#[test]
fn bindgen_test_layout_wasi_recv_out_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_recv_out_t>(),
        52usize,
        concat!("Size of: ", stringify!(wasi_recv_out_t))
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_recv_out_t>(),
        4usize,
        concat!("Alignment of ", stringify!(wasi_recv_out_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_recv_out_t>())).ro_datalen as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_recv_out_t),
            "::",
            stringify!(ro_datalen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_recv_out_t>())).ro_fdslen as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_recv_out_t),
            "::",
            stringify!(ro_fdslen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_recv_out_t>())).ro_unused as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_recv_out_t),
            "::",
            stringify!(ro_unused)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_recv_out_t>())).ro_flags as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_recv_out_t),
            "::",
            stringify!(ro_flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_send_in_t {
    pub si_data: uintptr_t, // *const wasi_ciovec_t
    pub si_data_len: size_t,
    pub si_fds: uintptr_t, // *const wasi_fd_t
    pub si_fds_len: size_t,
    pub si_flags: wasi_siflags_t,
}
#[test]
fn bindgen_test_layout_wasi_send_in_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_send_in_t>(),
        20usize,
        concat!("Size of: ", stringify!(wasi_send_in_t))
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_send_in_t>(),
        4usize,
        concat!("Alignment of ", stringify!(wasi_send_in_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_send_in_t>())).si_data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_send_in_t),
            "::",
            stringify!(si_data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_send_in_t>())).si_data_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_send_in_t),
            "::",
            stringify!(si_data_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_send_in_t>())).si_fds as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_send_in_t),
            "::",
            stringify!(si_fds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_send_in_t>())).si_fds_len as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_send_in_t),
            "::",
            stringify!(si_fds_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_send_in_t>())).si_flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_send_in_t),
            "::",
            stringify!(si_flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_send_out_t {
    pub so_datalen: size_t,
}
#[test]
fn bindgen_test_layout_wasi_send_out_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_send_out_t>(),
        4usize,
        concat!("Size of: ", stringify!(wasi_send_out_t))
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_send_out_t>(),
        4usize,
        concat!("Alignment of ", stringify!(wasi_send_out_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_send_out_t>())).so_datalen as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_send_out_t),
            "::",
            stringify!(so_datalen)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wasi_subscription_t {
    pub userdata: wasi_userdata_t,
    pub unused: u16,
    pub type_: wasi_eventtype_t,
    pub __bindgen_padding_0: u32,
    pub __bindgen_anon_1: wasi_subscription_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union wasi_subscription_t__bindgen_ty_1 {
    pub clock: wasi_subscription_t__bindgen_ty_1__bindgen_ty_1,
    pub condvar: wasi_subscription_t__bindgen_ty_1__bindgen_ty_2,
    pub fd_readwrite: wasi_subscription_t__bindgen_ty_1__bindgen_ty_3,
    pub lock: wasi_subscription_t__bindgen_ty_1__bindgen_ty_4,
    pub proc_terminate: wasi_subscription_t__bindgen_ty_1__bindgen_ty_5,
    _bindgen_union_align: [u64; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_subscription_t__bindgen_ty_1__bindgen_ty_1 {
    pub identifier: wasi_userdata_t,
    pub clock_id: wasi_clockid_t,
    pub __bindgen_padding_0: u32,
    pub timeout: wasi_timestamp_t,
    pub precision: wasi_timestamp_t,
    pub flags: wasi_subclockflags_t,
    pub __bindgen_padding_1: [u16; 3usize],
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_1>(),
        40usize,
        concat!(
            "Size of: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_1>())).identifier
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(identifier)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_1>())).clock_id
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(clock_id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_1>())).timeout
                as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(timeout)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_1>())).precision
                as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(precision)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_1>())).flags
                as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_subscription_t__bindgen_ty_1__bindgen_ty_2 {
    pub condvar: uintptr_t, // *mut wasi_condvar_t
    pub lock: uintptr_t,    // *mut wasi_lock_t
    pub condvar_scope: wasi_scope_t,
    pub lock_scope: wasi_scope_t,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_2>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_2>())).condvar
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(condvar)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_2>())).lock
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(lock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_2>()))
                .condvar_scope as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(condvar_scope)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_2>())).lock_scope
                as *const _ as usize
        },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(lock_scope)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_subscription_t__bindgen_ty_1__bindgen_ty_3 {
    pub fd: wasi_fd_t,
    pub flags: wasi_subrwflags_t,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_3>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_3>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_3>())).fd
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(fd)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_3>())).flags
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_subscription_t__bindgen_ty_1__bindgen_ty_4 {
    pub lock: uintptr_t, // *mut wasi_lock_t
    pub lock_scope: wasi_scope_t,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t__bindgen_ty_1__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_4>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_4>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_4>())).lock
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(lock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_4>())).lock_scope
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(lock_scope)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_subscription_t__bindgen_ty_1__bindgen_ty_5 {
    pub fd: wasi_fd_t,
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t__bindgen_ty_1__bindgen_ty_5() {
    assert_eq!(
        ::std::mem::size_of::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_5>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_5>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1__bindgen_ty_5>())).fd
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1__bindgen_ty_5),
            "::",
            stringify!(fd)
        )
    );
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<wasi_subscription_t__bindgen_ty_1>(),
        40usize,
        concat!("Size of: ", stringify!(wasi_subscription_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1>())).clock as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(clock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1>())).condvar as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(condvar)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1>())).fd_readwrite as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(fd_readwrite)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1>())).lock as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(lock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wasi_subscription_t__bindgen_ty_1>())).proc_terminate as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(proc_terminate)
        )
    );
}
#[allow(non_snake_case)]
#[test]
fn bindgen_test_layout_wasi_subscription_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_subscription_t>(),
        56usize,
        concat!("Size of: ", stringify!(wasi_subscription_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_subscription_t>())).userdata as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t),
            "::",
            stringify!(userdata)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_subscription_t>())).unused as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t),
            "::",
            stringify!(unused)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_subscription_t>())).type_ as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_subscription_t),
            "::",
            stringify!(type_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_tcb_t {
    pub parent: uintptr_t, // *mut ::std::os::raw::c_void
}
#[test]
fn bindgen_test_layout_wasi_tcb_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_tcb_t>(),
        4usize,
        concat!("Size of: ", stringify!(wasi_tcb_t))
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_tcb_t>(),
        4usize,
        concat!("Alignment of ", stringify!(wasi_tcb_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_tcb_t>())).parent as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_tcb_t),
            "::",
            stringify!(parent)
        )
    );
}
pub type wasi_threadentry_t = uintptr_t;
//pub type wasi_threadentry_t = ::std::option::Option<
//    unsafe extern "C" fn(
//        tid: wasi_tid_t,
//        aux: uintptr_t, // *mut ::std::os::raw::c_void
//    ),
//>;
#[test]
fn bindgen_test_layout_wasi_threadentry_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_threadentry_t>(),
        4usize,
        concat!("Size of: ", stringify!(wasi_threadentry_t))
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_threadentry_t>(),
        4usize,
        concat!("Alignment of ", stringify!(wasi_threadentry_t))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wasi_threadattr_t {
    pub entry_point: wasi_threadentry_t,
    pub stack: uintptr_t, // *mut ::std::os::raw::c_void
    pub stack_len: size_t,
    pub argument: uintptr_t, // *mut ::std::os::raw::c_void
}
#[test]
fn bindgen_test_layout_wasi_threadattr_t() {
    assert_eq!(
        ::std::mem::size_of::<wasi_threadattr_t>(),
        16usize,
        concat!("Size of: ", stringify!(wasi_threadattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<wasi_threadattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(wasi_threadattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_threadattr_t>())).entry_point as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_threadattr_t),
            "::",
            stringify!(entry_point)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_threadattr_t>())).stack as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_threadattr_t),
            "::",
            stringify!(stack)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_threadattr_t>())).stack_len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_threadattr_t),
            "::",
            stringify!(stack_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wasi_threadattr_t>())).argument as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(wasi_threadattr_t),
            "::",
            stringify!(argument)
        )
    );
}

pub fn strerror(errno: wasi_errno_t) -> &'static str {
    match errno {
        WASI_ESUCCESS => "WASI_ESUCCESS",
        WASI_E2BIG => "WASI_E2BIG",
        WASI_EACCES => "WASI_EACCES",
        WASI_EADDRINUSE => "WASI_EADDRINUSE",
        WASI_EADDRNOTAVAIL => "WASI_EADDRNOTAVAIL",
        WASI_EAFNOSUPPORT => "WASI_EAFNOSUPPORT",
        WASI_EAGAIN => "WASI_EAGAIN",
        WASI_EALREADY => "WASI_EALREADY",
        WASI_EBADF => "WASI_EBADF",
        WASI_EBADMSG => "WASI_EBADMSG",
        WASI_EBUSY => "WASI_EBUSY",
        WASI_ECANCELED => "WASI_ECANCELED",
        WASI_ECHILD => "WASI_ECHILD",
        WASI_ECONNABORTED => "WASI_ECONNABORTED",
        WASI_ECONNREFUSED => "WASI_ECONNREFUSED",
        WASI_ECONNRESET => "WASI_ECONNRESET",
        WASI_EDEADLK => "WASI_EDEADLK",
        WASI_EDESTADDRREQ => "WASI_EDESTADDRREQ",
        WASI_EDOM => "WASI_EDOM",
        WASI_EDQUOT => "WASI_EDQUOT",
        WASI_EEXIST => "WASI_EEXIST",
        WASI_EFAULT => "WASI_EFAULT",
        WASI_EFBIG => "WASI_EFBIG",
        WASI_EHOSTUNREACH => "WASI_EHOSTUNREACH",
        WASI_EIDRM => "WASI_EIDRM",
        WASI_EILSEQ => "WASI_EILSEQ",
        WASI_EINPROGRESS => "WASI_EINPROGRESS",
        WASI_EINTR => "WASI_EINTR",
        WASI_EINVAL => "WASI_EINVAL",
        WASI_EIO => "WASI_EIO",
        WASI_EISCONN => "WASI_EISCONN",
        WASI_EISDIR => "WASI_EISDIR",
        WASI_ELOOP => "WASI_ELOOP",
        WASI_EMFILE => "WASI_EMFILE",
        WASI_EMLINK => "WASI_EMLINK",
        WASI_EMSGSIZE => "WASI_EMSGSIZE",
        WASI_EMULTIHOP => "WASI_EMULTIHOP",
        WASI_ENAMETOOLONG => "WASI_ENAMETOOLONG",
        WASI_ENETDOWN => "WASI_ENETDOWN",
        WASI_ENETRESET => "WASI_ENETRESET",
        WASI_ENETUNREACH => "WASI_ENETUNREACH",
        WASI_ENFILE => "WASI_ENFILE",
        WASI_ENOBUFS => "WASI_ENOBUFS",
        WASI_ENODEV => "WASI_ENODEV",
        WASI_ENOENT => "WASI_ENOENT",
        WASI_ENOEXEC => "WASI_ENOEXEC",
        WASI_ENOLCK => "WASI_ENOLCK",
        WASI_ENOLINK => "WASI_ENOLINK",
        WASI_ENOMEM => "WASI_ENOMEM",
        WASI_ENOMSG => "WASI_ENOMSG",
        WASI_ENOPROTOOPT => "WASI_ENOPROTOOPT",
        WASI_ENOSPC => "WASI_ENOSPC",
        WASI_ENOSYS => "WASI_ENOSYS",
        WASI_ENOTCONN => "WASI_ENOTCONN",
        WASI_ENOTDIR => "WASI_ENOTDIR",
        WASI_ENOTEMPTY => "WASI_ENOTEMPTY",
        WASI_ENOTRECOVERABLE => "WASI_ENOTRECOVERABLE",
        WASI_ENOTSOCK => "WASI_ENOTSOCK",
        WASI_ENOTSUP => "WASI_ENOTSUP",
        WASI_ENOTTY => "WASI_ENOTTY",
        WASI_ENXIO => "WASI_ENXIO",
        WASI_EOVERFLOW => "WASI_EOVERFLOW",
        WASI_EOWNERDEAD => "WASI_EOWNERDEAD",
        WASI_EPERM => "WASI_EPERM",
        WASI_EPIPE => "WASI_EPIPE",
        WASI_EPROTO => "WASI_EPROTO",
        WASI_EPROTONOSUPPORT => "WASI_EPROTONOSUPPORT",
        WASI_EPROTOTYPE => "WASI_EPROTOTYPE",
        WASI_ERANGE => "WASI_ERANGE",
        WASI_EROFS => "WASI_EROFS",
        WASI_ESPIPE => "WASI_ESPIPE",
        WASI_ESRCH => "WASI_ESRCH",
        WASI_ESTALE => "WASI_ESTALE",
        WASI_ETIMEDOUT => "WASI_ETIMEDOUT",
        WASI_ETXTBSY => "WASI_ETXTBSY",
        WASI_EXDEV => "WASI_EXDEV",
        WASI_ENOTCAPABLE => "WASI_ENOTCAPABLE",
        other => panic!("Undefined errno value {:?}", other),
    }
}

pub fn whence_to_str(whence: wasi_whence_t) -> &'static str {
    match whence {
        WASI_WHENCE_CUR => "WASI_WHENCE_CUR",
        WASI_WHENCE_END => "WASI_WHENCE_END",
        WASI_WHENCE_SET => "WASI_WHENCE_SET",
        other => panic!("Undefined whence value {:?}", other),
    }
}
