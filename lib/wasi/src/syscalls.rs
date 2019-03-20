use crate::host::fd_table;
use crate::instantiate::WASIState;
use cranelift_codegen::ir::types::{Type, I32, I64};
use host;
use translate::*;
use wasm32;
use wasmtime_runtime::VMContext;

fn return_encoded_errno(e: host::__wasi_errno_t) -> wasm32::__wasi_errno_t {
    let errno = encode_errno(e);
    trace!("    -> errno={}", wasm32::strerror(errno));
    errno
}

unsafe fn get_curfds(vmctx: *mut VMContext) -> *mut fd_table {
    (&mut *(&mut *vmctx)
        .host_state()
        .downcast_mut::<WASIState>()
        .unwrap()
        .curfds) as *mut fd_table
}

pub trait AbiRet {
    type Abi;
    fn convert(self) -> Self::Abi;
    fn codegen_tys() -> Vec<Type>;
}

pub trait AbiParam {
    type Abi;
    fn convert(arg: Self::Abi) -> Self;
    fn codegen_ty() -> Type;
}

macro_rules! cast32 {
    ($($i:ident)*) => ($(
        impl AbiRet for $i {
            type Abi = i32;

            fn convert(self) -> Self::Abi {
                self as i32
            }

            fn codegen_tys() -> Vec<Type> { vec![I32] }
        }

        impl AbiParam for $i {
            type Abi = i32;

            fn convert(param: i32) -> Self {
                param as $i
            }

            fn codegen_ty() -> Type { I32 }
        }
    )*)
}

macro_rules! cast64 {
    ($($i:ident)*) => ($(
        impl AbiRet for $i {
            type Abi = i64;

            fn convert(self) -> Self::Abi {
                self as i64
            }

            fn codegen_tys() -> Vec<Type> { vec![I64] }
        }

        impl AbiParam for $i {
            type Abi = i64;

            fn convert(param: i64) -> Self {
                param as $i
            }

            fn codegen_ty() -> Type { I64 }
        }
    )*)
}

cast32!(i8 i16 i32 u8 u16 u32);
cast64!(i64 u64);

impl AbiRet for () {
    type Abi = ();
    fn convert(self) {}
    fn codegen_tys() -> Vec<Type> {
        Vec::new()
    }
}

macro_rules! syscalls {
    ($(pub unsafe extern "C" fn $name:ident($ctx:ident: *mut VMContext $(, $arg:ident: $ty:ty)*,) -> $ret:ty {
        $($body:tt)*
    })*) => ($(
        pub mod $name {
            use super::*;

            /// Returns the codegen types of all the parameters to the shim
            /// generated
            pub fn params() -> Vec<Type> {
                vec![$(<$ty as AbiParam>::codegen_ty()),*]
            }

            /// Returns the codegen types of all the results of the shim
            /// generated
            pub fn results() -> Vec<Type> {
                <$ret as AbiRet>::codegen_tys()
            }

            /// The actual function pointer to the shim for a syscall.
            ///
            /// NB: ideally we'd expose `shim` below, but it seems like there's
            /// a compiler bug which prvents that from being cast to a `usize`.
            pub static SHIM: unsafe extern "C" fn(
                *mut VMContext,
                $(<$ty as AbiParam>::Abi),*
            ) -> <$ret as AbiRet>::Abi = shim;

            unsafe extern "C" fn shim(
                $ctx: *mut VMContext,
                $($arg: <$ty as AbiParam>::Abi,)*
            ) -> <$ret as AbiRet>::Abi {
                let r = super::$name($ctx, $(<$ty as AbiParam>::convert($arg),)*);
                <$ret as AbiRet>::convert(r)
            }
        }

        pub unsafe extern "C" fn $name($ctx: *mut VMContext, $($arg: $ty,)*) -> $ret {
            $($body)*
        }
    )*)
}

syscalls! {

    pub unsafe extern "C" fn clock_res_get(
        vmctx: *mut VMContext,
        clock_id: wasm32::__wasi_clockid_t,
        resolution: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "clock_res_get(clock_id={:?}, resolution={:#x?})",
            clock_id,
            resolution,
        );

        let vmctx = &mut *vmctx;
        let clock_id = decode_clockid(clock_id);
        let mut host_resolution = match decode_timestamp_byref(vmctx, resolution) {
            Ok(host_resolution) => host_resolution,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_clock_res_get(clock_id, &mut host_resolution);

        trace!("     | *resolution={:?}", host_resolution);
        encode_timestamp_byref(vmctx, resolution, host_resolution).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn clock_time_get(
        vmctx: *mut VMContext,
        clock_id: wasm32::__wasi_clockid_t,
        precision: wasm32::__wasi_timestamp_t,
        time: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "clock_time_get(clock_id={:?}, precision={:?}, time={:#x?})",
            clock_id,
            precision,
            time,
        );

        let vmctx = &mut *vmctx;
        let clock_id = decode_clockid(clock_id);
        let precision = decode_timestamp(precision);
        let mut host_time = match decode_timestamp_byref(vmctx, time) {
            Ok(host_time) => host_time,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_clock_time_get(clock_id, precision, &mut host_time);

        trace!("     | *time={:?}", host_time);
        encode_timestamp_byref(vmctx, time, host_time).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn fd_close(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
    ) -> wasm32::__wasi_errno_t {
        trace!("fd_close(fd={:?})", fd);

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);

        let e = host::wasmtime_ssp_fd_close(curfds, fd);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn fd_datasync(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
    ) -> wasm32::__wasi_errno_t {
        trace!("fd_datasync(fd={:?})", fd);

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);

        let e = host::wasmtime_ssp_fd_datasync(curfds, fd);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn fd_pread(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        iovs: wasm32::uintptr_t,
        iovs_len: wasm32::size_t,
        offset: wasm32::__wasi_filesize_t,
        nread: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "fd_pread(fd={:?}, iovs={:#x?}, iovs_len={:?}, offset={}, nread={:#x?})",
            fd,
            iovs,
            iovs_len,
            offset,
            nread
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let iovs = match decode_iovec_slice(vmctx, iovs, iovs_len) {
            Ok(iovs) => iovs,
            Err(e) => return return_encoded_errno(e),
        };
        let offset = decode_filesize(offset);
        let mut host_nread = match decode_usize_byref(vmctx, nread) {
            Ok(host_nread) => host_nread,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_fd_pread(
            curfds,
            fd,
            iovs.as_ptr(),
            iovs.len(),
            offset,
            &mut host_nread,
        );

        trace!("     | *nread={:?}", host_nread);
        encode_usize_byref(vmctx, nread, host_nread).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn fd_pwrite(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        iovs: wasm32::uintptr_t,
        iovs_len: wasm32::size_t,
        offset: wasm32::__wasi_filesize_t,
        nwritten: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "fd_pwrite(fd={:?}, iovs={:#x?}, iovs_len={:?}, offset={}, nwritten={:#x?})",
            fd,
            iovs,
            iovs_len,
            offset,
            nwritten
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let iovs = match decode_ciovec_slice(vmctx, iovs, iovs_len) {
            Ok(iovs) => iovs,
            Err(e) => return return_encoded_errno(e),
        };
        let offset = decode_filesize(offset);
        let mut host_nwritten = match decode_usize_byref(vmctx, nwritten) {
            Ok(host_nwritten) => host_nwritten,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_fd_pwrite(
            curfds,
            fd,
            iovs.as_ptr(),
            iovs.len(),
            offset,
            &mut host_nwritten,
        );

        trace!("     | *nwritten={:?}", host_nwritten);
        encode_usize_byref(vmctx, nwritten, host_nwritten).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn fd_read(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        iovs: wasm32::uintptr_t,
        iovs_len: wasm32::size_t,
        nread: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "fd_read(fd={:?}, iovs={:#x?}, iovs_len={:?}, nread={:#x?})",
            fd,
            iovs,
            iovs_len,
            nread
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let iovs = match decode_iovec_slice(vmctx, iovs, iovs_len) {
            Ok(iovs) => iovs,
            Err(e) => return return_encoded_errno(e),
        };
        let mut host_nread = match decode_usize_byref(vmctx, nread) {
            Ok(host_nread) => host_nread,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_fd_read(curfds, fd, iovs.as_ptr(), iovs.len(), &mut host_nread);

        trace!("     | *nread={:?}", host_nread);
        encode_usize_byref(vmctx, nread, host_nread).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn fd_renumber(
        vmctx: *mut VMContext,
        from: wasm32::__wasi_fd_t,
        to: wasm32::__wasi_fd_t,
    ) -> wasm32::__wasi_errno_t {
        trace!("fd_renumber(from={:?}, to={:?})", from, to);

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let from = decode_fd(from);
        let to = decode_fd(to);

        let e = host::wasmtime_ssp_fd_renumber(curfds, from, to);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn fd_seek(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        offset: wasm32::__wasi_filedelta_t,
        whence: wasm32::__wasi_whence_t,
        newoffset: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "fd_seek(fd={:?}, offset={:?}, whence={}, newoffset={:#x?})",
            fd,
            offset,
            wasm32::whence_to_str(whence),
            newoffset
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let offset = decode_filedelta(offset);
        let whence = decode_whence(whence);
        let mut host_newoffset = match decode_filesize_byref(vmctx, newoffset) {
            Ok(host_newoffset) => host_newoffset,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_fd_seek(curfds, fd, offset, whence, &mut host_newoffset);

        trace!("     | *newoffset={:?}", host_newoffset);
        encode_filesize_byref(vmctx, newoffset, host_newoffset).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn fd_tell(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        newoffset: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!("fd_tell(fd={:?}, newoffset={:#x?})", fd, newoffset);

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let mut host_newoffset = match decode_filesize_byref(vmctx, newoffset) {
            Ok(host_newoffset) => host_newoffset,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_fd_tell(curfds, fd, &mut host_newoffset);

        trace!("     | *newoffset={:?}", host_newoffset);
        encode_filesize_byref(vmctx, newoffset, host_newoffset).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn fd_stat_get(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        buf: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!("fd_stat_get(fd={:?}, buf={:#x?})", fd, buf);

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let mut host_buf = match decode_fdstat_byref(vmctx, buf) {
            Ok(host_buf) => host_buf,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_fd_stat_get(curfds, fd, &mut host_buf);

        trace!("     | *buf={:?}", host_buf);
        encode_fdstat_byref(vmctx, buf, host_buf).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn fd_stat_set_flags(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        flags: wasm32::__wasi_fdflags_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "fd_stat_set_flags(fd={:?}, flags={:#x?})",
            fd,
            flags
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let flags = decode_fdflags(flags);

        let e = host::wasmtime_ssp_fd_stat_set_flags(curfds, fd, flags);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn fd_stat_set_rights(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        fs_rights_base: wasm32::__wasi_rights_t,
        fs_rights_inheriting: wasm32::__wasi_rights_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "fd_stat_set_rights(fd={:?}, fs_rights_base={:#x?}, fs_rights_inheriting={:#x?})",
            fd,
            fs_rights_base,
            fs_rights_inheriting
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let fs_rights_base = decode_rights(fs_rights_base);
        let fs_rights_inheriting = decode_rights(fs_rights_inheriting);

        let e = host::wasmtime_ssp_fd_stat_set_rights(curfds, fd, fs_rights_base, fs_rights_inheriting);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn fd_sync(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
    ) -> wasm32::__wasi_errno_t {
        trace!("fd_sync(fd={:?})", fd);

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);

        let e = host::wasmtime_ssp_fd_sync(curfds, fd);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn fd_write(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        iovs: wasm32::uintptr_t,
        iovs_len: wasm32::size_t,
        nwritten: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "fd_write(fd={:?}, iovs={:#x?}, iovs_len={:?}, nwritten={:#x?})",
            fd,
            iovs,
            iovs_len,
            nwritten
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let iovs = match decode_ciovec_slice(vmctx, iovs, iovs_len) {
            Ok(iovs) => iovs,
            Err(e) => return return_encoded_errno(e),
        };
        let mut host_nwritten = match decode_usize_byref(vmctx, nwritten) {
            Ok(host_nwritten) => host_nwritten,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_fd_write(curfds, fd, iovs.as_ptr(), iovs.len(), &mut host_nwritten);

        trace!("     | *nwritten={:?}", host_nwritten);
        encode_usize_byref(vmctx, nwritten, host_nwritten).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_advise(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        offset: wasm32::__wasi_filesize_t,
        len: wasm32::__wasi_filesize_t,
        advice: wasm32::__wasi_advice_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "file_advise(fd={:?}, offset={}, len={}, advice={:?})",
            fd,
            offset,
            len,
            advice
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let offset = decode_filesize(offset);
        let len = decode_filesize(len);
        let advice = decode_advice(advice);

        let e = host::wasmtime_ssp_file_advise(curfds, fd, offset, len, advice);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_allocate(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        offset: wasm32::__wasi_filesize_t,
        len: wasm32::__wasi_filesize_t,
    ) -> wasm32::__wasi_errno_t {
        trace!("file_allocate(fd={:?}, offset={}, len={})", fd, offset, len);

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let offset = decode_filesize(offset);
        let len = decode_filesize(len);

        let e = host::wasmtime_ssp_file_allocate(curfds, fd, offset, len);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_create_directory(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        path: wasm32::uintptr_t,
        path_len: wasm32::size_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "file_create_directory(fd={:?}, path={:#x?}, path_len={})",
            fd,
            path,
            path_len,
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let (path, path_len) = match decode_char_slice(vmctx, path, path_len) {
            Ok((path, path_len)) => (path, path_len),
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_file_create_directory(curfds, fd, path, path_len);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_link(
        vmctx: *mut VMContext,
        fd0: wasm32::__wasi_fd_t,
        flags0: wasm32::__wasi_lookupflags_t,
        path0: wasm32::uintptr_t,
        path_len0: wasm32::size_t,
        fd1: wasm32::__wasi_fd_t,
        path1: wasm32::uintptr_t,
        path_len1: wasm32::size_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "file_link(fd0={:?}, flags0={:?}, path0={:#x?}, path_len0={}, fd1={:?}, path1={:#x?}, path_len1={})",
            fd0,
            flags0,
            path0,
            path_len0,
            fd1,
            path1,
            path_len1
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd0 = decode_fd(fd0);
        let flags0 = decode_lookupflags(flags0);
        let (path0, path_len0) = match decode_char_slice(vmctx, path0, path_len0) {
            Ok((path0, path_len0)) => (path0, path_len0),
            Err(e) => return return_encoded_errno(e),
        };
        let fd1 = decode_fd(fd1);
        let (path1, path_len1) = match decode_char_slice(vmctx, path1, path_len1) {
            Ok((path1, path_len1)) => (path1, path_len1),
            Err(e) => return return_encoded_errno(e),
        };

        let e =
            host::wasmtime_ssp_file_link(curfds, fd0, flags0, path0, path_len0, fd1, path1, path_len1);

        return_encoded_errno(e)
    }

    // TODO: When multi-value happens, switch to that instead of passing
    // the `fd` by reference?
    pub unsafe extern "C" fn file_open(
        vmctx: *mut VMContext,
        dirfd: wasm32::__wasi_fd_t,
        dirflags: wasm32::__wasi_lookupflags_t,
        path: wasm32::uintptr_t,
        path_len: wasm32::size_t,
        oflags: wasm32::__wasi_oflags_t,
        fs_rights_base: wasm32::__wasi_rights_t,
        fs_rights_inheriting: wasm32::__wasi_rights_t,
        fs_flags: wasm32::__wasi_fdflags_t,
        fd: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "file_open(dirfd={:?}, dirflags={:?}, path={:#x?}, path_len={:?}, oflags={:#x?}, fs_rights_base={:#x?}, fs_rights_inheriting={:#x?}, fs_flags={:#x?}, fd={:#x?})",
            dirfd,
            dirflags,
            path,
            path_len,
            oflags,
            fs_rights_base,
            fs_rights_inheriting,
            fs_flags,
            fd
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let dirfd = decode_fd(dirfd);
        let dirflags = decode_lookupflags(dirflags);
        let (path, path_len) = match decode_char_slice(vmctx, path, path_len) {
            Ok((path, path_len)) => (path, path_len),
            Err(e) => return return_encoded_errno(e),
        };
        let oflags = decode_oflags(oflags);
        let fs_rights_base = decode_rights(fs_rights_base);
        let fs_rights_inheriting = decode_rights(fs_rights_inheriting);
        let fs_flags = decode_fdflags(fs_flags);
        let mut host_fd = match decode_fd_byref(vmctx, fd) {
            Ok(host_fd) => host_fd,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_file_open(
            curfds,
            dirfd,
            dirflags,
            path,
            path_len,
            oflags,
            fs_rights_base,
            fs_rights_inheriting,
            fs_flags,
            &mut host_fd,
        );

        trace!("     | *fd={:?}", host_fd);
        encode_fd_byref(vmctx, fd, host_fd).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_readdir(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        buf: wasm32::uintptr_t,
        buf_len: wasm32::size_t,
        cookie: wasm32::__wasi_dircookie_t,
        buf_used: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "file_readdir(fd={:?}, buf={:#x?}, buf_len={}, cookie={:#x?}, buf_used={:#x?})",
            fd,
            buf,
            buf_len,
            cookie,
            buf_used,
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let (buf, buf_len) = match decode_char_slice(vmctx, buf, buf_len) {
            Ok((buf, buf_len)) => (buf, buf_len),
            Err(e) => return return_encoded_errno(e),
        };
        let cookie = decode_dircookie(cookie);
        let mut host_buf_used = match decode_usize_byref(vmctx, buf_used) {
            Ok(host_buf_used) => host_buf_used,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_file_readdir(
            curfds,
            fd,
            buf as *mut host::void,
            buf_len,
            cookie,
            &mut host_buf_used,
        );

        trace!("     | *buf_used={:?}", host_buf_used);
        encode_usize_byref(vmctx, buf_used, host_buf_used).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_readlink(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        path: wasm32::uintptr_t,
        path_len: wasm32::size_t,
        buf: wasm32::uintptr_t,
        buf_len: wasm32::size_t,
        buf_used: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "file_readlink(fd={:?}, path={:#x?}, path_len={:?}, buf={:#x?}, buf_len={}, buf_used={:#x?})",
            fd,
            path,
            path_len,
            buf,
            buf_len,
            buf_used,
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let (path, path_len) = match decode_char_slice(vmctx, path, path_len) {
            Ok((path, path_len)) => (path, path_len),
            Err(e) => return return_encoded_errno(e),
        };
        let (buf, buf_len) = match decode_char_slice(vmctx, buf, buf_len) {
            Ok((buf, buf_len)) => (buf, buf_len),
            Err(e) => return return_encoded_errno(e),
        };
        let mut host_buf_used = match decode_usize_byref(vmctx, buf_used) {
            Ok(host_buf_used) => host_buf_used,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_file_readlink(
            curfds,
            fd,
            path,
            path_len,
            buf,
            buf_len,
            &mut host_buf_used,
        );

        trace!("     | *buf_used={:?}", host_buf_used);
        encode_usize_byref(vmctx, buf_used, host_buf_used).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_rename(
        vmctx: *mut VMContext,
        fd0: wasm32::__wasi_fd_t,
        path0: wasm32::uintptr_t,
        path_len0: wasm32::size_t,
        fd1: wasm32::__wasi_fd_t,
        path1: wasm32::uintptr_t,
        path_len1: wasm32::size_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "file_rename(fd0={:?}, path0={:#x?}, path_len0={:?}, fd1={:?}, path1={:#x?}, path_len1={:?})",
            fd0,
            path0,
            path_len0,
            fd1,
            path1,
            path_len1,
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd0 = decode_fd(fd0);
        let (path0, path_len0) = match decode_char_slice(vmctx, path0, path_len0) {
            Ok((path0, path_len0)) => (path0, path_len0),
            Err(e) => return return_encoded_errno(e),
        };
        let fd1 = decode_fd(fd1);
        let (path1, path_len1) = match decode_char_slice(vmctx, path1, path_len1) {
            Ok((path1, path_len1)) => (path1, path_len1),
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_file_rename(curfds, fd0, path0, path_len0, fd1, path1, path_len1);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_fstat_get(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        buf: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!("file_fstat_get(fd={:?}, buf={:#x?})", fd, buf);

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let mut host_buf = match decode_filestat_byref(vmctx, buf) {
            Ok(host_buf) => host_buf,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_file_fstat_get(curfds, fd, &mut host_buf);

        trace!("     | *buf={:?}", host_buf);
        encode_filestat_byref(vmctx, buf, host_buf).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_fstat_set_times(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        st_atim: wasm32::__wasi_timestamp_t,
        st_mtim: wasm32::__wasi_timestamp_t,
        fstflags: wasm32::__wasi_fstflags_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "file_fstat_set_times(fd={:?}, st_atim={}, st_mtim={}, fstflags={:#x?})",
            fd,
            st_atim, st_mtim,
            fstflags
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let st_atim = decode_timestamp(st_atim);
        let st_mtim = decode_timestamp(st_mtim);
        let fstflags = decode_fstflags(fstflags);

        let e = host::wasmtime_ssp_file_fstat_set_times(curfds, fd, st_atim, st_mtim, fstflags);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_fstat_set_size(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        size: wasm32::__wasi_filesize_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "file_fstat_set_size(fd={:?}, size={})",
            fd,
            size
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let size = decode_filesize(size);

        let e = host::wasmtime_ssp_file_fstat_set_size(curfds, fd, size);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_stat_get(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        flags: wasm32::__wasi_lookupflags_t,
        path: wasm32::uintptr_t,
        path_len: wasm32::size_t,
        buf: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "file_stat_get(fd={:?}, flags={:?}, path={:#x?}, path_len={}, buf={:#x?})",
            fd,
            flags,
            path,
            path_len,
            buf
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let flags = decode_lookupflags(flags);
        let (path, path_len) = match decode_char_slice(vmctx, path, path_len) {
            Ok((path, path_len)) => (path, path_len),
            Err(e) => return return_encoded_errno(e),
        };
        let mut host_buf = match decode_filestat_byref(vmctx, buf) {
            Ok(host_buf) => host_buf,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_file_stat_get(curfds, fd, flags, path, path_len, &mut host_buf);

        trace!("     | *buf={:?}", host_buf);
        encode_filestat_byref(vmctx, buf, host_buf).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_stat_set_times(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        flags: wasm32::__wasi_lookupflags_t,
        path: wasm32::uintptr_t,
        path_len: wasm32::size_t,
        st_atim: wasm32::__wasi_timestamp_t,
        st_mtim: wasm32::__wasi_timestamp_t,
        fstflags: wasm32::__wasi_fstflags_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "file_stat_set_times(fd={:?}, flags={:?}, path={:#x?}, path_len={}, st_atim={}, st_mtim={}, fstflags={:#x?})",
            fd,
            flags,
            path,
            path_len,
            st_atim, st_mtim,
            fstflags
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let flags = decode_lookupflags(flags);
        let (path, path_len) = match decode_char_slice(vmctx, path, path_len) {
            Ok((path, path_len)) => (path, path_len),
            Err(e) => return return_encoded_errno(e),
        };
        let st_atim = decode_timestamp(st_atim);
        let st_mtim = decode_timestamp(st_mtim);
        let fstflags = decode_fstflags(fstflags);

        let e = host::wasmtime_ssp_file_stat_set_times(curfds, fd, flags, path, path_len, st_atim, st_mtim, fstflags);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_symlink(
        vmctx: *mut VMContext,
        path0: wasm32::uintptr_t,
        path_len0: wasm32::size_t,
        fd: wasm32::__wasi_fd_t,
        path1: wasm32::uintptr_t,
        path_len1: wasm32::size_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "file_symlink(path0={:#x?}, path_len0={}, fd={:?}, path1={:#x?}, path_len1={})",
            path0,
            path_len0,
            fd,
            path1,
            path_len1
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let (path0, path_len0) = match decode_char_slice(vmctx, path0, path_len0) {
            Ok((path0, path_len0)) => (path0, path_len0),
            Err(e) => return return_encoded_errno(e),
        };
        let fd = decode_fd(fd);
        let (path1, path_len1) = match decode_char_slice(vmctx, path1, path_len1) {
            Ok((path1, path_len1)) => (path1, path_len1),
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_file_symlink(curfds, path0, path_len0, fd, path1, path_len1);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn file_unlink(
        vmctx: *mut VMContext,
        fd: wasm32::__wasi_fd_t,
        path: wasm32::uintptr_t,
        path_len: wasm32::size_t,
        flags: wasm32::__wasi_ulflags_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "file_unlink(fd={:?}, path={:#x?}, path_len={}, flags={:#x?})",
            fd,
            path,
            path_len,
            flags
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let fd = decode_fd(fd);
        let (path, path_len) = match decode_char_slice(vmctx, path, path_len) {
            Ok((path, path_len)) => (path, path_len),
            Err(e) => return return_encoded_errno(e),
        };
        let flags = decode_ulflags(flags);

        let e = host::wasmtime_ssp_file_unlink(curfds, fd, path, path_len, flags);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn poll_oneoff(
        vmctx: *mut VMContext,
        in_: wasm32::uintptr_t,
        out: wasm32::uintptr_t,
        nsubscriptions: wasm32::size_t,
        nevents: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "poll_oneoff(in={:#x?}, out={:#x?}, nsubscriptions={}, nevents={:#x?})",
            in_,
            out,
            nsubscriptions,
            nevents,
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let in_ = match decode_subscription_slice(vmctx, in_, nsubscriptions) {
            Ok(in_) => in_,
            Err(e) => return return_encoded_errno(e),
        };
        let mut host_out = match decode_event_slice(vmctx, out, nsubscriptions) {
            Ok(out) => out,
            Err(e) => return return_encoded_errno(e),
        };
        let mut host_nevents = match decode_usize_byref(vmctx, nevents) {
            Ok(host_nevents) => host_nevents,
            Err(e) => return return_encoded_errno(e),
        };

        assert!(in_.len() == host_out.len());

        let e = host::wasmtime_ssp_poll_oneoff(
            curfds,
            in_.as_ptr(),
            host_out.as_mut_ptr(),
            in_.len(),
            &mut host_nevents,
        );

        trace!("     | *nevents={:?}", host_nevents);
        encode_usize_byref(vmctx, nevents, host_nevents).unwrap();

        if log_enabled!(log::Level::Trace) {
            for (index, _event) in host_out.iter().enumerate() {
                // TODO: Format the output for tracing.
                trace!("     | *out[{}]=...", index);
            }
        }
        encode_event_slice(vmctx, out, host_out).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn proc_exit(_vmctx: *mut VMContext, rval: u32,) -> () {
        trace!("proc_exec(rval={:?})", rval);

        let rval = decode_exitcode(rval);

        // TODO: Rather than call __wasi_proc_exit here, we should trigger a
        // stack unwind similar to a trap.

        panic!("__wasi_proc_exit({})", rval)
    }

    pub unsafe extern "C" fn proc_raise(
        _vmctx: *mut VMContext,
        _sig: wasm32::__wasi_signal_t,
    ) -> wasm32::__wasi_errno_t {
        unimplemented!("__wasi_proc_raise");
    }

    pub unsafe extern "C" fn random_get(
        vmctx: *mut VMContext,
        buf: wasm32::uintptr_t,
        buf_len: wasm32::size_t,
    ) -> wasm32::__wasi_errno_t {
        trace!("random_get(buf={:#x?}, buf_len={:?})", buf, buf_len);

        let vmctx = &mut *vmctx;
        let (buf, buf_len) = match decode_char_slice(vmctx, buf, buf_len) {
            Ok((buf, buf_len)) => (buf, buf_len),
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_random_get(buf as *mut host::void, buf_len);

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn sched_yield(_vmctx: *mut VMContext,) -> wasm32::__wasi_errno_t {
        let e = host::wasmtime_ssp_sched_yield();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn sock_recv(
        vmctx: *mut VMContext,
        sock: wasm32::__wasi_fd_t,
        ri_data: wasm32::uintptr_t,
        ri_data_len: wasm32::size_t,
        ri_flags: wasm32::__wasi_riflags_t,
        ro_datalen: wasm32::uintptr_t,
        ro_flags: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "sock_recv(sock={:?}, ri_data={:#x?}, ri_data_len={}, ri_flags={:#x?}, ro_datalen={:#x?}, ro_flags={:#x?})",
            sock,
            ri_data, ri_data_len, ri_flags,
            ro_datalen, ro_flags
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let sock = decode_fd(sock);
        let ri_data = match decode_iovec_slice(vmctx, ri_data, ri_data_len) {
            Ok(ri_data) => ri_data,
            Err(e) => return return_encoded_errno(e),
        };
        let ri_flags = decode_riflags(ri_flags);
        let mut host_ro_datalen = match decode_usize_byref(vmctx, ro_datalen) {
            Ok(host_ro_datalen) => host_ro_datalen,
            Err(e) => return return_encoded_errno(e),
        };
        let mut host_ro_flags = match decode_roflags_byref(vmctx, ro_flags) {
            Ok(host_ro_flags) => host_ro_flags,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_sock_recv(curfds, sock, ri_data.as_ptr(), ri_data.len(), ri_flags,
                                             &mut host_ro_datalen, &mut host_ro_flags);

        // TODO: Format the output for tracing.
        trace!("     | *ro_datalen={}", host_ro_datalen);
        trace!("     | *ro_flags={}", host_ro_flags);
        encode_usize_byref(vmctx, ro_datalen, host_ro_datalen).unwrap();
        encode_roflags_byref(vmctx, ro_flags, host_ro_flags).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn sock_send(
        vmctx: *mut VMContext,
        sock: wasm32::__wasi_fd_t,
        si_data: wasm32::uintptr_t,
        si_data_len: wasm32::size_t,
        si_flags: wasm32::__wasi_siflags_t,
        so_datalen: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t {
        trace!(
            "sock_send(sock={:?}, si_data={:#x?}, si_data_len={}, si_flags={:#x?}, so_datalen={:#x?})",
            sock,
            si_data, si_data_len, si_flags, so_datalen,
        );

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let sock = decode_fd(sock);
        let si_data = match decode_ciovec_slice(vmctx, si_data, si_data_len) {
            Ok(si_data) => si_data,
            Err(e) => return return_encoded_errno(e),
        };
        let si_flags = decode_siflags(si_flags);
        let mut host_so_datalen = match decode_usize_byref(vmctx, so_datalen) {
            Ok(so_datalen) => so_datalen,
            Err(e) => return return_encoded_errno(e),
        };

        let e = host::wasmtime_ssp_sock_send(curfds, sock, si_data.as_ptr(), si_data.len(), si_flags, &mut host_so_datalen);

        trace!("     | *so_datalen={:?}", host_so_datalen);
        encode_usize_byref(vmctx, so_datalen, host_so_datalen).unwrap();

        return_encoded_errno(e)
    }

    pub unsafe extern "C" fn sock_shutdown(
        vmctx: *mut VMContext,
        sock: wasm32::__wasi_fd_t,
        how: wasm32::__wasi_sdflags_t,
    ) -> wasm32::__wasi_errno_t {
        trace!("sock_shutdown(sock={:?}, how={:?})", sock, how);

        let vmctx = &mut *vmctx;
        let curfds = get_curfds(vmctx);
        let sock = decode_fd(sock);
        let how = decode_sdflags(how);

        let e = host::wasmtime_ssp_sock_shutdown(curfds, sock, how);

        return_encoded_errno(e)
    }
}
