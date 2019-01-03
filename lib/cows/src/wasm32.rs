//! CloudABI types as defined in wasm32. This file was originally generated
//! by running bindgen over cloudabi_types.h with a wasm32 target, and the
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
pub const CLOUDABI_ADVICE_DONTNEED: u32 = 1;
pub const CLOUDABI_ADVICE_NOREUSE: u32 = 2;
pub const CLOUDABI_ADVICE_NORMAL: u32 = 3;
pub const CLOUDABI_ADVICE_RANDOM: u32 = 4;
pub const CLOUDABI_ADVICE_SEQUENTIAL: u32 = 5;
pub const CLOUDABI_ADVICE_WILLNEED: u32 = 6;
pub const CLOUDABI_AT_ARGDATA: u32 = 256;
pub const CLOUDABI_AT_ARGDATALEN: u32 = 257;
pub const CLOUDABI_AT_BASE: u32 = 7;
pub const CLOUDABI_AT_CANARY: u32 = 258;
pub const CLOUDABI_AT_CANARYLEN: u32 = 259;
pub const CLOUDABI_AT_NCPUS: u32 = 260;
pub const CLOUDABI_AT_NULL: u32 = 0;
pub const CLOUDABI_AT_PAGESZ: u32 = 6;
pub const CLOUDABI_AT_PHDR: u32 = 3;
pub const CLOUDABI_AT_PHNUM: u32 = 4;
pub const CLOUDABI_AT_PID: u32 = 263;
pub const CLOUDABI_AT_SYSINFO_EHDR: u32 = 262;
pub const CLOUDABI_AT_TID: u32 = 261;
pub const CLOUDABI_CLOCK_MONOTONIC: u32 = 1;
pub const CLOUDABI_CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
pub const CLOUDABI_CLOCK_REALTIME: u32 = 3;
pub const CLOUDABI_CLOCK_THREAD_CPUTIME_ID: u32 = 4;
pub const CLOUDABI_CONDVAR_HAS_NO_WAITERS: u32 = 0;
pub const CLOUDABI_DIRCOOKIE_START: u32 = 0;
pub const CLOUDABI_ESUCCESS: cloudabi_errno_t = 0;
pub const CLOUDABI_E2BIG: cloudabi_errno_t = 1;
pub const CLOUDABI_EACCES: cloudabi_errno_t = 2;
pub const CLOUDABI_EADDRINUSE: cloudabi_errno_t = 3;
pub const CLOUDABI_EADDRNOTAVAIL: cloudabi_errno_t = 4;
pub const CLOUDABI_EAFNOSUPPORT: cloudabi_errno_t = 5;
pub const CLOUDABI_EAGAIN: cloudabi_errno_t = 6;
pub const CLOUDABI_EALREADY: cloudabi_errno_t = 7;
pub const CLOUDABI_EBADF: cloudabi_errno_t = 8;
pub const CLOUDABI_EBADMSG: cloudabi_errno_t = 9;
pub const CLOUDABI_EBUSY: cloudabi_errno_t = 10;
pub const CLOUDABI_ECANCELED: cloudabi_errno_t = 11;
pub const CLOUDABI_ECHILD: cloudabi_errno_t = 12;
pub const CLOUDABI_ECONNABORTED: cloudabi_errno_t = 13;
pub const CLOUDABI_ECONNREFUSED: cloudabi_errno_t = 14;
pub const CLOUDABI_ECONNRESET: cloudabi_errno_t = 15;
pub const CLOUDABI_EDEADLK: cloudabi_errno_t = 16;
pub const CLOUDABI_EDESTADDRREQ: cloudabi_errno_t = 17;
pub const CLOUDABI_EDOM: cloudabi_errno_t = 18;
pub const CLOUDABI_EDQUOT: cloudabi_errno_t = 19;
pub const CLOUDABI_EEXIST: cloudabi_errno_t = 20;
pub const CLOUDABI_EFAULT: cloudabi_errno_t = 21;
pub const CLOUDABI_EFBIG: cloudabi_errno_t = 22;
pub const CLOUDABI_EHOSTUNREACH: cloudabi_errno_t = 23;
pub const CLOUDABI_EIDRM: cloudabi_errno_t = 24;
pub const CLOUDABI_EILSEQ: cloudabi_errno_t = 25;
pub const CLOUDABI_EINPROGRESS: cloudabi_errno_t = 26;
pub const CLOUDABI_EINTR: cloudabi_errno_t = 27;
pub const CLOUDABI_EINVAL: cloudabi_errno_t = 28;
pub const CLOUDABI_EIO: cloudabi_errno_t = 29;
pub const CLOUDABI_EISCONN: cloudabi_errno_t = 30;
pub const CLOUDABI_EISDIR: cloudabi_errno_t = 31;
pub const CLOUDABI_ELOOP: cloudabi_errno_t = 32;
pub const CLOUDABI_EMFILE: cloudabi_errno_t = 33;
pub const CLOUDABI_EMLINK: cloudabi_errno_t = 34;
pub const CLOUDABI_EMSGSIZE: cloudabi_errno_t = 35;
pub const CLOUDABI_EMULTIHOP: cloudabi_errno_t = 36;
pub const CLOUDABI_ENAMETOOLONG: cloudabi_errno_t = 37;
pub const CLOUDABI_ENETDOWN: cloudabi_errno_t = 38;
pub const CLOUDABI_ENETRESET: cloudabi_errno_t = 39;
pub const CLOUDABI_ENETUNREACH: cloudabi_errno_t = 40;
pub const CLOUDABI_ENFILE: cloudabi_errno_t = 41;
pub const CLOUDABI_ENOBUFS: cloudabi_errno_t = 42;
pub const CLOUDABI_ENODEV: cloudabi_errno_t = 43;
pub const CLOUDABI_ENOENT: cloudabi_errno_t = 44;
pub const CLOUDABI_ENOEXEC: cloudabi_errno_t = 45;
pub const CLOUDABI_ENOLCK: cloudabi_errno_t = 46;
pub const CLOUDABI_ENOLINK: cloudabi_errno_t = 47;
pub const CLOUDABI_ENOMEM: cloudabi_errno_t = 48;
pub const CLOUDABI_ENOMSG: cloudabi_errno_t = 49;
pub const CLOUDABI_ENOPROTOOPT: cloudabi_errno_t = 50;
pub const CLOUDABI_ENOSPC: cloudabi_errno_t = 51;
pub const CLOUDABI_ENOSYS: cloudabi_errno_t = 52;
pub const CLOUDABI_ENOTCONN: cloudabi_errno_t = 53;
pub const CLOUDABI_ENOTDIR: cloudabi_errno_t = 54;
pub const CLOUDABI_ENOTEMPTY: cloudabi_errno_t = 55;
pub const CLOUDABI_ENOTRECOVERABLE: cloudabi_errno_t = 56;
pub const CLOUDABI_ENOTSOCK: cloudabi_errno_t = 57;
pub const CLOUDABI_ENOTSUP: cloudabi_errno_t = 58;
pub const CLOUDABI_ENOTTY: cloudabi_errno_t = 59;
pub const CLOUDABI_ENXIO: cloudabi_errno_t = 60;
pub const CLOUDABI_EOVERFLOW: cloudabi_errno_t = 61;
pub const CLOUDABI_EOWNERDEAD: cloudabi_errno_t = 62;
pub const CLOUDABI_EPERM: cloudabi_errno_t = 63;
pub const CLOUDABI_EPIPE: cloudabi_errno_t = 64;
pub const CLOUDABI_EPROTO: cloudabi_errno_t = 65;
pub const CLOUDABI_EPROTONOSUPPORT: cloudabi_errno_t = 66;
pub const CLOUDABI_EPROTOTYPE: cloudabi_errno_t = 67;
pub const CLOUDABI_ERANGE: cloudabi_errno_t = 68;
pub const CLOUDABI_EROFS: cloudabi_errno_t = 69;
pub const CLOUDABI_ESPIPE: cloudabi_errno_t = 70;
pub const CLOUDABI_ESRCH: cloudabi_errno_t = 71;
pub const CLOUDABI_ESTALE: cloudabi_errno_t = 72;
pub const CLOUDABI_ETIMEDOUT: cloudabi_errno_t = 73;
pub const CLOUDABI_ETXTBSY: cloudabi_errno_t = 74;
pub const CLOUDABI_EXDEV: cloudabi_errno_t = 75;
pub const CLOUDABI_ENOTCAPABLE: cloudabi_errno_t = 76;
pub const CLOUDABI_EVENT_FD_READWRITE_HANGUP: u32 = 1;
pub const CLOUDABI_EVENTTYPE_CLOCK: u32 = 1;
pub const CLOUDABI_EVENTTYPE_CONDVAR: u32 = 2;
pub const CLOUDABI_EVENTTYPE_FD_READ: u32 = 3;
pub const CLOUDABI_EVENTTYPE_FD_WRITE: u32 = 4;
pub const CLOUDABI_EVENTTYPE_LOCK_RDLOCK: u32 = 5;
pub const CLOUDABI_EVENTTYPE_LOCK_WRLOCK: u32 = 6;
pub const CLOUDABI_EVENTTYPE_PROC_TERMINATE: u32 = 7;
pub const CLOUDABI_PROCESS_CHILD: u32 = 4294967295;
pub const CLOUDABI_MAP_ANON_FD: u32 = 4294967295;
pub const CLOUDABI_FDFLAG_APPEND: u32 = 1;
pub const CLOUDABI_FDFLAG_DSYNC: u32 = 2;
pub const CLOUDABI_FDFLAG_NONBLOCK: u32 = 4;
pub const CLOUDABI_FDFLAG_RSYNC: u32 = 8;
pub const CLOUDABI_FDFLAG_SYNC: u32 = 16;
pub const CLOUDABI_FDSTAT_FLAGS: u32 = 1;
pub const CLOUDABI_FDSTAT_RIGHTS: u32 = 2;
pub const CLOUDABI_FILETYPE_UNKNOWN: u32 = 0;
pub const CLOUDABI_FILETYPE_BLOCK_DEVICE: u32 = 16;
pub const CLOUDABI_FILETYPE_CHARACTER_DEVICE: u32 = 17;
pub const CLOUDABI_FILETYPE_DIRECTORY: u32 = 32;
pub const CLOUDABI_FILETYPE_PROCESS: u32 = 80;
pub const CLOUDABI_FILETYPE_REGULAR_FILE: u32 = 96;
pub const CLOUDABI_FILETYPE_SHARED_MEMORY: u32 = 112;
pub const CLOUDABI_FILETYPE_SOCKET_DGRAM: u32 = 128;
pub const CLOUDABI_FILETYPE_SOCKET_STREAM: u32 = 130;
pub const CLOUDABI_FILETYPE_SYMBOLIC_LINK: u32 = 144;
pub const CLOUDABI_FILESTAT_ATIM: u32 = 1;
pub const CLOUDABI_FILESTAT_ATIM_NOW: u32 = 2;
pub const CLOUDABI_FILESTAT_MTIM: u32 = 4;
pub const CLOUDABI_FILESTAT_MTIM_NOW: u32 = 8;
pub const CLOUDABI_FILESTAT_SIZE: u32 = 16;
pub const CLOUDABI_LOCK_UNLOCKED: u32 = 0;
pub const CLOUDABI_LOCK_WRLOCKED: u32 = 1073741824;
pub const CLOUDABI_LOCK_KERNEL_MANAGED: u32 = 2147483648;
pub const CLOUDABI_LOCK_BOGUS: u32 = 2147483648;
pub const CLOUDABI_LOOKUP_SYMLINK_FOLLOW: u32 = 1;
pub const CLOUDABI_MAP_ANON: u32 = 1;
pub const CLOUDABI_MAP_FIXED: u32 = 2;
pub const CLOUDABI_MAP_PRIVATE: u32 = 4;
pub const CLOUDABI_MAP_SHARED: u32 = 8;
pub const CLOUDABI_PROT_EXEC: u32 = 1;
pub const CLOUDABI_PROT_WRITE: u32 = 2;
pub const CLOUDABI_PROT_READ: u32 = 4;
pub const CLOUDABI_MS_ASYNC: u32 = 1;
pub const CLOUDABI_MS_INVALIDATE: u32 = 2;
pub const CLOUDABI_MS_SYNC: u32 = 4;
pub const CLOUDABI_O_CREAT: u32 = 1;
pub const CLOUDABI_O_DIRECTORY: u32 = 2;
pub const CLOUDABI_O_EXCL: u32 = 4;
pub const CLOUDABI_O_TRUNC: u32 = 8;
pub const CLOUDABI_SOCK_RECV_PEEK: u32 = 4;
pub const CLOUDABI_SOCK_RECV_WAITALL: u32 = 16;
pub const CLOUDABI_RIGHT_FD_DATASYNC: u32 = 1;
pub const CLOUDABI_RIGHT_FD_READ: u32 = 2;
pub const CLOUDABI_RIGHT_FD_SEEK: u32 = 4;
pub const CLOUDABI_RIGHT_FD_STAT_PUT_FLAGS: u32 = 8;
pub const CLOUDABI_RIGHT_FD_SYNC: u32 = 16;
pub const CLOUDABI_RIGHT_FD_TELL: u32 = 32;
pub const CLOUDABI_RIGHT_FD_WRITE: u32 = 64;
pub const CLOUDABI_RIGHT_FILE_ADVISE: u32 = 128;
pub const CLOUDABI_RIGHT_FILE_ALLOCATE: u32 = 256;
pub const CLOUDABI_RIGHT_FILE_CREATE_DIRECTORY: u32 = 512;
pub const CLOUDABI_RIGHT_FILE_CREATE_FILE: u32 = 1024;
pub const CLOUDABI_RIGHT_FILE_LINK_SOURCE: u32 = 4096;
pub const CLOUDABI_RIGHT_FILE_LINK_TARGET: u32 = 8192;
pub const CLOUDABI_RIGHT_FILE_OPEN: u32 = 16384;
pub const CLOUDABI_RIGHT_FILE_READDIR: u32 = 32768;
pub const CLOUDABI_RIGHT_FILE_READLINK: u32 = 65536;
pub const CLOUDABI_RIGHT_FILE_RENAME_SOURCE: u32 = 131072;
pub const CLOUDABI_RIGHT_FILE_RENAME_TARGET: u32 = 262144;
pub const CLOUDABI_RIGHT_FILE_STAT_FGET: u32 = 524288;
pub const CLOUDABI_RIGHT_FILE_STAT_FPUT_SIZE: u32 = 1048576;
pub const CLOUDABI_RIGHT_FILE_STAT_FPUT_TIMES: u32 = 2097152;
pub const CLOUDABI_RIGHT_FILE_STAT_GET: u32 = 4194304;
pub const CLOUDABI_RIGHT_FILE_STAT_PUT_TIMES: u32 = 8388608;
pub const CLOUDABI_RIGHT_FILE_SYMLINK: u32 = 16777216;
pub const CLOUDABI_RIGHT_FILE_UNLINK: u32 = 33554432;
pub const CLOUDABI_RIGHT_MEM_MAP: u32 = 67108864;
pub const CLOUDABI_RIGHT_MEM_MAP_EXEC: u32 = 134217728;
pub const CLOUDABI_RIGHT_POLL_FD_READWRITE: u32 = 268435456;
pub const CLOUDABI_RIGHT_POLL_PROC_TERMINATE: u32 = 1073741824;
pub const CLOUDABI_RIGHT_PROC_EXEC: u64 = 4294967296;
pub const CLOUDABI_RIGHT_SOCK_SHUTDOWN: u64 = 549755813888;
pub const CLOUDABI_SOCK_RECV_FDS_TRUNCATED: u32 = 1;
pub const CLOUDABI_SOCK_RECV_DATA_TRUNCATED: u32 = 8;
pub const CLOUDABI_SCOPE_PRIVATE: u32 = 4;
pub const CLOUDABI_SCOPE_SHARED: u32 = 8;
pub const CLOUDABI_SHUT_RD: u32 = 1;
pub const CLOUDABI_SHUT_WR: u32 = 2;
pub const CLOUDABI_SIGABRT: u32 = 1;
pub const CLOUDABI_SIGALRM: u32 = 2;
pub const CLOUDABI_SIGBUS: u32 = 3;
pub const CLOUDABI_SIGCHLD: u32 = 4;
pub const CLOUDABI_SIGCONT: u32 = 5;
pub const CLOUDABI_SIGFPE: u32 = 6;
pub const CLOUDABI_SIGHUP: u32 = 7;
pub const CLOUDABI_SIGILL: u32 = 8;
pub const CLOUDABI_SIGINT: u32 = 9;
pub const CLOUDABI_SIGKILL: u32 = 10;
pub const CLOUDABI_SIGPIPE: u32 = 11;
pub const CLOUDABI_SIGQUIT: u32 = 12;
pub const CLOUDABI_SIGSEGV: u32 = 13;
pub const CLOUDABI_SIGSTOP: u32 = 14;
pub const CLOUDABI_SIGSYS: u32 = 15;
pub const CLOUDABI_SIGTERM: u32 = 16;
pub const CLOUDABI_SIGTRAP: u32 = 17;
pub const CLOUDABI_SIGTSTP: u32 = 18;
pub const CLOUDABI_SIGTTIN: u32 = 19;
pub const CLOUDABI_SIGTTOU: u32 = 20;
pub const CLOUDABI_SIGURG: u32 = 21;
pub const CLOUDABI_SIGUSR1: u32 = 22;
pub const CLOUDABI_SIGUSR2: u32 = 23;
pub const CLOUDABI_SIGVTALRM: u32 = 24;
pub const CLOUDABI_SIGXCPU: u32 = 25;
pub const CLOUDABI_SIGXFSZ: u32 = 26;
pub const CLOUDABI_SUBSCRIPTION_CLOCK_ABSTIME: u32 = 1;
pub const CLOUDABI_SUBSCRIPTION_FD_READWRITE_POLL: u32 = 1;
pub const CLOUDABI_UNLINK_REMOVEDIR: u32 = 1;
pub const CLOUDABI_WHENCE_CUR: cloudabi_whence_t = 1;
pub const CLOUDABI_WHENCE_END: cloudabi_whence_t = 2;
pub const CLOUDABI_WHENCE_SET: cloudabi_whence_t = 3;
pub type wchar_t = i32;
pub type size_t = u32;
pub type intptr_t = i32;
pub type long = i32;
pub type unsigned_long = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
    pub __bindgen_padding_0: [u32; 2usize],
    pub __clang_max_align_nonce2: f64,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce2 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
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
pub type __timer_t = intptr_t; // *mut ::std::os::raw::c_void
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
pub type __caddr_t = intptr_t; // *mut i8
pub type __intptr_t = i32;
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
pub type cloudabi_advice_t = u8;
pub type cloudabi_auxtype_t = u32;
pub type cloudabi_clockid_t = u32;
pub type cloudabi_condvar_t = u32;
pub type cloudabi_device_t = u64;
pub type cloudabi_dircookie_t = u64;
pub type cloudabi_errno_t = u16;
pub type cloudabi_eventrwflags_t = u16;
pub type cloudabi_eventtype_t = u8;
pub type cloudabi_exitcode_t = u32;
pub type cloudabi_fd_t = u32;
pub type cloudabi_fdflags_t = u16;
pub type cloudabi_fdsflags_t = u16;
pub type cloudabi_filedelta_t = i64;
pub type cloudabi_filesize_t = u64;
pub type cloudabi_filetype_t = u8;
pub type cloudabi_fsflags_t = u16;
pub type cloudabi_inode_t = u64;
pub type cloudabi_linkcount_t = u32;
pub type cloudabi_lock_t = u32;
pub type cloudabi_lookupflags_t = u32;
pub type cloudabi_mflags_t = u8;
pub type cloudabi_mprot_t = u8;
pub type cloudabi_msflags_t = u8;
pub type cloudabi_nthreads_t = u32;
pub type cloudabi_oflags_t = u16;
pub type cloudabi_riflags_t = u16;
pub type cloudabi_rights_t = u64;
pub type cloudabi_roflags_t = u16;
pub type cloudabi_scope_t = u8;
pub type cloudabi_sdflags_t = u8;
pub type cloudabi_siflags_t = u16;
pub type cloudabi_signal_t = u8;
pub type cloudabi_subclockflags_t = u16;
pub type cloudabi_subrwflags_t = u16;
pub type cloudabi_tid_t = u32;
pub type cloudabi_timestamp_t = u64;
pub type cloudabi_ulflags_t = u8;
pub type cloudabi_userdata_t = u64;
pub type cloudabi_whence_t = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_dirent_t {
    pub d_next: cloudabi_dircookie_t,
    pub d_ino: cloudabi_inode_t,
    pub d_namlen: u32,
    pub d_type: cloudabi_filetype_t,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_cloudabi_dirent_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_dirent_t>(),
        24usize,
        concat!("Size of: ", stringify!(cloudabi_dirent_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_dirent_t>())).d_next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_dirent_t),
            "::",
            stringify!(d_next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_dirent_t>())).d_ino as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_dirent_t),
            "::",
            stringify!(d_ino)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_dirent_t>())).d_namlen as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_dirent_t),
            "::",
            stringify!(d_namlen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_dirent_t>())).d_type as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_dirent_t),
            "::",
            stringify!(d_type)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cloudabi_event_t {
    pub userdata: cloudabi_userdata_t,
    pub error: cloudabi_errno_t,
    pub type_: cloudabi_eventtype_t,
    pub __bindgen_padding_0: u32,
    pub __bindgen_anon_1: cloudabi_event_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cloudabi_event_t__bindgen_ty_1 {
    pub fd_readwrite: cloudabi_event_t__bindgen_ty_1__bindgen_ty_1,
    pub proc_terminate: cloudabi_event_t__bindgen_ty_1__bindgen_ty_2,
    _bindgen_union_align: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_event_t__bindgen_ty_1__bindgen_ty_1 {
    pub nbytes: cloudabi_filesize_t,
    pub unused: [i8; 4usize],
    pub flags: cloudabi_eventrwflags_t,
    pub __bindgen_padding_0: u16,
}
#[test]
fn bindgen_test_layout_cloudabi_event_t__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_event_t__bindgen_ty_1__bindgen_ty_1>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(cloudabi_event_t__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_event_t__bindgen_ty_1__bindgen_ty_1>())).nbytes
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_event_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(nbytes)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_event_t__bindgen_ty_1__bindgen_ty_1>())).unused
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_event_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(unused)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_event_t__bindgen_ty_1__bindgen_ty_1>())).flags
                as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_event_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_event_t__bindgen_ty_1__bindgen_ty_2 {
    pub unused: [i8; 4usize],
    pub signal: cloudabi_signal_t,
    pub exitcode: cloudabi_exitcode_t,
}
#[test]
fn bindgen_test_layout_cloudabi_event_t__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_event_t__bindgen_ty_1__bindgen_ty_2>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(cloudabi_event_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_event_t__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(cloudabi_event_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_event_t__bindgen_ty_1__bindgen_ty_2>())).unused
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_event_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(unused)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_event_t__bindgen_ty_1__bindgen_ty_2>())).signal
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_event_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(signal)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_event_t__bindgen_ty_1__bindgen_ty_2>())).exitcode
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_event_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(exitcode)
        )
    );
}
#[test]
fn bindgen_test_layout_cloudabi_event_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_event_t__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(cloudabi_event_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_event_t__bindgen_ty_1>())).fd_readwrite as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_event_t__bindgen_ty_1),
            "::",
            stringify!(fd_readwrite)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_event_t__bindgen_ty_1>())).proc_terminate as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_event_t__bindgen_ty_1),
            "::",
            stringify!(proc_terminate)
        )
    );
}
#[test]
fn bindgen_test_layout_cloudabi_event_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_event_t>(),
        32usize,
        concat!("Size of: ", stringify!(cloudabi_event_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_event_t>())).userdata as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_event_t),
            "::",
            stringify!(userdata)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_event_t>())).error as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_event_t),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_event_t>())).type_ as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_event_t),
            "::",
            stringify!(type_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_fdstat_t {
    pub fs_filetype: cloudabi_filetype_t,
    pub fs_flags: cloudabi_fdflags_t,
    pub __bindgen_padding_0: u32,
    pub fs_rights_base: cloudabi_rights_t,
    pub fs_rights_inheriting: cloudabi_rights_t,
}
#[test]
fn bindgen_test_layout_cloudabi_fdstat_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_fdstat_t>(),
        24usize,
        concat!("Size of: ", stringify!(cloudabi_fdstat_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_fdstat_t>())).fs_filetype as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_fdstat_t),
            "::",
            stringify!(fs_filetype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_fdstat_t>())).fs_flags as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_fdstat_t),
            "::",
            stringify!(fs_flags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_fdstat_t>())).fs_rights_base as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_fdstat_t),
            "::",
            stringify!(fs_rights_base)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_fdstat_t>())).fs_rights_inheriting as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_fdstat_t),
            "::",
            stringify!(fs_rights_inheriting)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_filestat_t {
    pub st_dev: cloudabi_device_t,
    pub st_ino: cloudabi_inode_t,
    pub st_filetype: cloudabi_filetype_t,
    pub st_nlink: cloudabi_linkcount_t,
    pub st_size: cloudabi_filesize_t,
    pub st_atim: cloudabi_timestamp_t,
    pub st_mtim: cloudabi_timestamp_t,
    pub st_ctim: cloudabi_timestamp_t,
}
#[test]
fn bindgen_test_layout_cloudabi_filestat_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_filestat_t>(),
        56usize,
        concat!("Size of: ", stringify!(cloudabi_filestat_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_filestat_t>())).st_dev as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_filestat_t),
            "::",
            stringify!(st_dev)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_filestat_t>())).st_ino as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_filestat_t),
            "::",
            stringify!(st_ino)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_filestat_t>())).st_filetype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_filestat_t),
            "::",
            stringify!(st_filetype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_filestat_t>())).st_nlink as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_filestat_t),
            "::",
            stringify!(st_nlink)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_filestat_t>())).st_size as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_filestat_t),
            "::",
            stringify!(st_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_filestat_t>())).st_atim as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_filestat_t),
            "::",
            stringify!(st_atim)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_filestat_t>())).st_mtim as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_filestat_t),
            "::",
            stringify!(st_mtim)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_filestat_t>())).st_ctim as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_filestat_t),
            "::",
            stringify!(st_ctim)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_lookup_t {
    pub fd: cloudabi_fd_t,
    pub flags: cloudabi_lookupflags_t,
}
#[test]
fn bindgen_test_layout_cloudabi_lookup_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_lookup_t>(),
        8usize,
        concat!("Size of: ", stringify!(cloudabi_lookup_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_lookup_t>(),
        4usize,
        concat!("Alignment of ", stringify!(cloudabi_lookup_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_lookup_t>())).fd as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_lookup_t),
            "::",
            stringify!(fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_lookup_t>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_lookup_t),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cloudabi_auxv_t {
    pub a_type: cloudabi_auxtype_t,
    pub __bindgen_anon_1: cloudabi_auxv_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cloudabi_auxv_t__bindgen_ty_1 {
    pub a_val: size_t,
    pub a_ptr: intptr_t, // *mut ::std::os::raw::c_void
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_cloudabi_auxv_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_auxv_t__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(cloudabi_auxv_t__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_auxv_t__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(cloudabi_auxv_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_auxv_t__bindgen_ty_1>())).a_val as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_auxv_t__bindgen_ty_1),
            "::",
            stringify!(a_val)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_auxv_t__bindgen_ty_1>())).a_ptr as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_auxv_t__bindgen_ty_1),
            "::",
            stringify!(a_ptr)
        )
    );
}
#[test]
fn bindgen_test_layout_cloudabi_auxv_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_auxv_t>(),
        8usize,
        concat!("Size of: ", stringify!(cloudabi_auxv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_auxv_t>(),
        4usize,
        concat!("Alignment of ", stringify!(cloudabi_auxv_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_auxv_t>())).a_type as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_auxv_t),
            "::",
            stringify!(a_type)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_ciovec_t {
    pub buf: intptr_t, // *const ::std::os::raw::c_void
    pub buf_len: size_t,
}
#[test]
fn bindgen_test_layout_cloudabi_ciovec_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_ciovec_t>(),
        8usize,
        concat!("Size of: ", stringify!(cloudabi_ciovec_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_ciovec_t>(),
        4usize,
        concat!("Alignment of ", stringify!(cloudabi_ciovec_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_ciovec_t>())).buf as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_ciovec_t),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_ciovec_t>())).buf_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_ciovec_t),
            "::",
            stringify!(buf_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_iovec_t {
    pub buf: intptr_t, // *mut ::std::os::raw::c_void
    pub buf_len: size_t,
}
#[test]
fn bindgen_test_layout_cloudabi_iovec_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_iovec_t>(),
        8usize,
        concat!("Size of: ", stringify!(cloudabi_iovec_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_iovec_t>(),
        4usize,
        concat!("Alignment of ", stringify!(cloudabi_iovec_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_iovec_t>())).buf as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_iovec_t),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_iovec_t>())).buf_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_iovec_t),
            "::",
            stringify!(buf_len)
        )
    );
}
// TODO: Figure out if we need this.
//pub type cloudabi_processentry_t =
//    ::std::option::Option<
//        unsafe extern "C" fn(
//            auxv: intptr_t, // *const cloudabi_auxv_t
//        )
//    >;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_recv_in_t {
    pub ri_data: intptr_t, // *const cloudabi_iovec_t
    pub ri_data_len: size_t,
    pub ri_fds: intptr_t, // *mut cloudabi_fd_t
    pub ri_fds_len: size_t,
    pub ri_flags: cloudabi_riflags_t,
}
#[test]
fn bindgen_test_layout_cloudabi_recv_in_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_recv_in_t>(),
        20usize,
        concat!("Size of: ", stringify!(cloudabi_recv_in_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_recv_in_t>(),
        4usize,
        concat!("Alignment of ", stringify!(cloudabi_recv_in_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_recv_in_t>())).ri_data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_recv_in_t),
            "::",
            stringify!(ri_data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_recv_in_t>())).ri_data_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_recv_in_t),
            "::",
            stringify!(ri_data_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_recv_in_t>())).ri_fds as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_recv_in_t),
            "::",
            stringify!(ri_fds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_recv_in_t>())).ri_fds_len as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_recv_in_t),
            "::",
            stringify!(ri_fds_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_recv_in_t>())).ri_flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_recv_in_t),
            "::",
            stringify!(ri_flags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cloudabi_recv_out_t {
    pub ro_datalen: size_t,
    pub ro_fdslen: size_t,
    pub ro_unused: [i8; 40usize],
    pub ro_flags: cloudabi_roflags_t,
}
#[test]
fn bindgen_test_layout_cloudabi_recv_out_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_recv_out_t>(),
        52usize,
        concat!("Size of: ", stringify!(cloudabi_recv_out_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_recv_out_t>(),
        4usize,
        concat!("Alignment of ", stringify!(cloudabi_recv_out_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_recv_out_t>())).ro_datalen as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_recv_out_t),
            "::",
            stringify!(ro_datalen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_recv_out_t>())).ro_fdslen as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_recv_out_t),
            "::",
            stringify!(ro_fdslen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_recv_out_t>())).ro_unused as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_recv_out_t),
            "::",
            stringify!(ro_unused)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_recv_out_t>())).ro_flags as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_recv_out_t),
            "::",
            stringify!(ro_flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_send_in_t {
    pub si_data: intptr_t, // *const cloudabi_ciovec_t
    pub si_data_len: size_t,
    pub si_fds: intptr_t, // *const cloudabi_fd_t
    pub si_fds_len: size_t,
    pub si_flags: cloudabi_siflags_t,
}
#[test]
fn bindgen_test_layout_cloudabi_send_in_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_send_in_t>(),
        20usize,
        concat!("Size of: ", stringify!(cloudabi_send_in_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_send_in_t>(),
        4usize,
        concat!("Alignment of ", stringify!(cloudabi_send_in_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_send_in_t>())).si_data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_send_in_t),
            "::",
            stringify!(si_data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_send_in_t>())).si_data_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_send_in_t),
            "::",
            stringify!(si_data_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_send_in_t>())).si_fds as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_send_in_t),
            "::",
            stringify!(si_fds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_send_in_t>())).si_fds_len as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_send_in_t),
            "::",
            stringify!(si_fds_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_send_in_t>())).si_flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_send_in_t),
            "::",
            stringify!(si_flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_send_out_t {
    pub so_datalen: size_t,
}
#[test]
fn bindgen_test_layout_cloudabi_send_out_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_send_out_t>(),
        4usize,
        concat!("Size of: ", stringify!(cloudabi_send_out_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_send_out_t>(),
        4usize,
        concat!("Alignment of ", stringify!(cloudabi_send_out_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_send_out_t>())).so_datalen as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_send_out_t),
            "::",
            stringify!(so_datalen)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cloudabi_subscription_t {
    pub userdata: cloudabi_userdata_t,
    pub unused: u16,
    pub type_: cloudabi_eventtype_t,
    pub __bindgen_padding_0: u32,
    pub __bindgen_anon_1: cloudabi_subscription_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cloudabi_subscription_t__bindgen_ty_1 {
    pub clock: cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1,
    pub condvar: cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2,
    pub fd_readwrite: cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_3,
    pub lock: cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_4,
    pub proc_terminate: cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_5,
    _bindgen_union_align: [u64; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1 {
    pub identifier: cloudabi_userdata_t,
    pub clock_id: cloudabi_clockid_t,
    pub __bindgen_padding_0: u32,
    pub timeout: cloudabi_timestamp_t,
    pub precision: cloudabi_timestamp_t,
    pub flags: cloudabi_subclockflags_t,
    pub __bindgen_padding_1: [u16; 3usize],
}
#[test]
fn bindgen_test_layout_cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1>(),
        40usize,
        concat!(
            "Size of: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1>()))
                .identifier as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(identifier)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1>())).clock_id
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(clock_id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1>())).timeout
                as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(timeout)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1>()))
                .precision as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(precision)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1>())).flags
                as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2 {
    pub condvar: intptr_t, // *mut cloudabi_condvar_t
    pub lock: intptr_t,    // *mut cloudabi_lock_t
    pub condvar_scope: cloudabi_scope_t,
    pub lock_scope: cloudabi_scope_t,
}
#[test]
fn bindgen_test_layout_cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2>())).condvar
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(condvar)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2>())).lock
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(lock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2>()))
                .condvar_scope as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(condvar_scope)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2>()))
                .lock_scope as *const _ as usize
        },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(lock_scope)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_3 {
    pub fd: cloudabi_fd_t,
    pub flags: cloudabi_subrwflags_t,
}
#[test]
fn bindgen_test_layout_cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_3>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_3>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_3>())).fd
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(fd)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_3>())).flags
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_4 {
    pub lock: intptr_t, // *mut cloudabi_lock_t
    pub lock_scope: cloudabi_scope_t,
}
#[test]
fn bindgen_test_layout_cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_4>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_4>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_4>())).lock
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(lock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_4>()))
                .lock_scope as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(lock_scope)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_5 {
    pub fd: cloudabi_fd_t,
}
#[test]
fn bindgen_test_layout_cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_5() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_5>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_5>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_5>())).fd
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1__bindgen_ty_5),
            "::",
            stringify!(fd)
        )
    );
}
#[test]
fn bindgen_test_layout_cloudabi_subscription_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_subscription_t__bindgen_ty_1>(),
        40usize,
        concat!(
            "Size of: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1>())).clock as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(clock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1>())).condvar as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(condvar)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1>())).fd_readwrite
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(fd_readwrite)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1>())).lock as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(lock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t__bindgen_ty_1>())).proc_terminate
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t__bindgen_ty_1),
            "::",
            stringify!(proc_terminate)
        )
    );
}
#[test]
fn bindgen_test_layout_cloudabi_subscription_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_subscription_t>(),
        56usize,
        concat!("Size of: ", stringify!(cloudabi_subscription_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_subscription_t>())).userdata as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t),
            "::",
            stringify!(userdata)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_subscription_t>())).unused as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t),
            "::",
            stringify!(unused)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_subscription_t>())).type_ as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_subscription_t),
            "::",
            stringify!(type_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_tcb_t {
    pub parent: intptr_t, // *mut ::std::os::raw::c_void
}
#[test]
fn bindgen_test_layout_cloudabi_tcb_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_tcb_t>(),
        4usize,
        concat!("Size of: ", stringify!(cloudabi_tcb_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_tcb_t>(),
        4usize,
        concat!("Alignment of ", stringify!(cloudabi_tcb_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_tcb_t>())).parent as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_tcb_t),
            "::",
            stringify!(parent)
        )
    );
}
pub type cloudabi_threadentry_t = ::std::option::Option<
    unsafe extern "C" fn(
        tid: cloudabi_tid_t,
        aux: intptr_t, // *mut ::std::os::raw::c_void
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cloudabi_threadattr_t {
    pub entry_point: cloudabi_threadentry_t,
    pub stack: intptr_t, // *mut ::std::os::raw::c_void
    pub stack_len: size_t,
    pub argument: intptr_t, // *mut ::std::os::raw::c_void
}
#[test]
fn bindgen_test_layout_cloudabi_threadattr_t() {
    assert_eq!(
        ::std::mem::size_of::<cloudabi_threadattr_t>(),
        16usize,
        concat!("Size of: ", stringify!(cloudabi_threadattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cloudabi_threadattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(cloudabi_threadattr_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cloudabi_threadattr_t>())).entry_point as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_threadattr_t),
            "::",
            stringify!(entry_point)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_threadattr_t>())).stack as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_threadattr_t),
            "::",
            stringify!(stack)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_threadattr_t>())).stack_len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_threadattr_t),
            "::",
            stringify!(stack_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cloudabi_threadattr_t>())).argument as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(cloudabi_threadattr_t),
            "::",
            stringify!(argument)
        )
    );
}

pub fn strerror(errno: cloudabi_errno_t) -> &'static str {
    match errno {
        CLOUDABI_ESUCCESS => "CLOUDABI_ESUCCESS",
        CLOUDABI_E2BIG => "CLOUDABI_E2BIG",
        CLOUDABI_EACCES => "CLOUDABI_EACCES",
        CLOUDABI_EADDRINUSE => "CLOUDABI_EADDRINUSE",
        CLOUDABI_EADDRNOTAVAIL => "CLOUDABI_EADDRNOTAVAIL",
        CLOUDABI_EAFNOSUPPORT => "CLOUDABI_EAFNOSUPPORT",
        CLOUDABI_EAGAIN => "CLOUDABI_EAGAIN",
        CLOUDABI_EALREADY => "CLOUDABI_EALREADY",
        CLOUDABI_EBADF => "CLOUDABI_EBADF",
        CLOUDABI_EBADMSG => "CLOUDABI_EBADMSG",
        CLOUDABI_EBUSY => "CLOUDABI_EBUSY",
        CLOUDABI_ECANCELED => "CLOUDABI_ECANCELED",
        CLOUDABI_ECHILD => "CLOUDABI_ECHILD",
        CLOUDABI_ECONNABORTED => "CLOUDABI_ECONNABORTED",
        CLOUDABI_ECONNREFUSED => "CLOUDABI_ECONNREFUSED",
        CLOUDABI_ECONNRESET => "CLOUDABI_ECONNRESET",
        CLOUDABI_EDEADLK => "CLOUDABI_EDEADLK",
        CLOUDABI_EDESTADDRREQ => "CLOUDABI_EDESTADDRREQ",
        CLOUDABI_EDOM => "CLOUDABI_EDOM",
        CLOUDABI_EDQUOT => "CLOUDABI_EDQUOT",
        CLOUDABI_EEXIST => "CLOUDABI_EEXIST",
        CLOUDABI_EFAULT => "CLOUDABI_EFAULT",
        CLOUDABI_EFBIG => "CLOUDABI_EFBIG",
        CLOUDABI_EHOSTUNREACH => "CLOUDABI_EHOSTUNREACH",
        CLOUDABI_EIDRM => "CLOUDABI_EIDRM",
        CLOUDABI_EILSEQ => "CLOUDABI_EILSEQ",
        CLOUDABI_EINPROGRESS => "CLOUDABI_EINPROGRESS",
        CLOUDABI_EINTR => "CLOUDABI_EINTR",
        CLOUDABI_EINVAL => "CLOUDABI_EINVAL",
        CLOUDABI_EIO => "CLOUDABI_EIO",
        CLOUDABI_EISCONN => "CLOUDABI_EISCONN",
        CLOUDABI_EISDIR => "CLOUDABI_EISDIR",
        CLOUDABI_ELOOP => "CLOUDABI_ELOOP",
        CLOUDABI_EMFILE => "CLOUDABI_EMFILE",
        CLOUDABI_EMLINK => "CLOUDABI_EMLINK",
        CLOUDABI_EMSGSIZE => "CLOUDABI_EMSGSIZE",
        CLOUDABI_EMULTIHOP => "CLOUDABI_EMULTIHOP",
        CLOUDABI_ENAMETOOLONG => "CLOUDABI_ENAMETOOLONG",
        CLOUDABI_ENETDOWN => "CLOUDABI_ENETDOWN",
        CLOUDABI_ENETRESET => "CLOUDABI_ENETRESET",
        CLOUDABI_ENETUNREACH => "CLOUDABI_ENETUNREACH",
        CLOUDABI_ENFILE => "CLOUDABI_ENFILE",
        CLOUDABI_ENOBUFS => "CLOUDABI_ENOBUFS",
        CLOUDABI_ENODEV => "CLOUDABI_ENODEV",
        CLOUDABI_ENOENT => "CLOUDABI_ENOENT",
        CLOUDABI_ENOEXEC => "CLOUDABI_ENOEXEC",
        CLOUDABI_ENOLCK => "CLOUDABI_ENOLCK",
        CLOUDABI_ENOLINK => "CLOUDABI_ENOLINK",
        CLOUDABI_ENOMEM => "CLOUDABI_ENOMEM",
        CLOUDABI_ENOMSG => "CLOUDABI_ENOMSG",
        CLOUDABI_ENOPROTOOPT => "CLOUDABI_ENOPROTOOPT",
        CLOUDABI_ENOSPC => "CLOUDABI_ENOSPC",
        CLOUDABI_ENOSYS => "CLOUDABI_ENOSYS",
        CLOUDABI_ENOTCONN => "CLOUDABI_ENOTCONN",
        CLOUDABI_ENOTDIR => "CLOUDABI_ENOTDIR",
        CLOUDABI_ENOTEMPTY => "CLOUDABI_ENOTEMPTY",
        CLOUDABI_ENOTRECOVERABLE => "CLOUDABI_ENOTRECOVERABLE",
        CLOUDABI_ENOTSOCK => "CLOUDABI_ENOTSOCK",
        CLOUDABI_ENOTSUP => "CLOUDABI_ENOTSUP",
        CLOUDABI_ENOTTY => "CLOUDABI_ENOTTY",
        CLOUDABI_ENXIO => "CLOUDABI_ENXIO",
        CLOUDABI_EOVERFLOW => "CLOUDABI_EOVERFLOW",
        CLOUDABI_EOWNERDEAD => "CLOUDABI_EOWNERDEAD",
        CLOUDABI_EPERM => "CLOUDABI_EPERM",
        CLOUDABI_EPIPE => "CLOUDABI_EPIPE",
        CLOUDABI_EPROTO => "CLOUDABI_EPROTO",
        CLOUDABI_EPROTONOSUPPORT => "CLOUDABI_EPROTONOSUPPORT",
        CLOUDABI_EPROTOTYPE => "CLOUDABI_EPROTOTYPE",
        CLOUDABI_ERANGE => "CLOUDABI_ERANGE",
        CLOUDABI_EROFS => "CLOUDABI_EROFS",
        CLOUDABI_ESPIPE => "CLOUDABI_ESPIPE",
        CLOUDABI_ESRCH => "CLOUDABI_ESRCH",
        CLOUDABI_ESTALE => "CLOUDABI_ESTALE",
        CLOUDABI_ETIMEDOUT => "CLOUDABI_ETIMEDOUT",
        CLOUDABI_ETXTBSY => "CLOUDABI_ETXTBSY",
        CLOUDABI_EXDEV => "CLOUDABI_EXDEV",
        CLOUDABI_ENOTCAPABLE => "CLOUDABI_ENOTCAPABLE",
        other => panic!("Undefined errno value {:?}", other),
    }
}

pub fn whence_to_str(whence: cloudabi_whence_t) -> &'static str {
    match whence {
        CLOUDABI_WHENCE_CUR => "CLOUDABI_WHENCE_CUR",
        CLOUDABI_WHENCE_END => "CLOUDABI_WHENCE_END",
        CLOUDABI_WHENCE_SET => "CLOUDABI_WHENCE_SET",
        other => panic!("Undefined whence value {:?}", other),
    }
}
