use crate::host::fd_table;
use crate::instantiate::WASIState;
use host;
use translate::*;
use wasm32;
use wasmtime_runtime::VMContext;

fn return_encoded_errno(e: host::wasi_errno_t) -> wasm32::wasi_errno_t {
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

pub unsafe extern "C" fn clock_res_get(
    clock_id: wasm32::wasi_clockid_t,
    resolution: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "clock_res_get(clock_id={:?}, resolution={:#x?})",
        clock_id,
        resolution,
    );

    let vmctx = &mut *vmctx;
    let clock_id = decode_clockid(clock_id);
    let mut host_resolution = match decode_timestamp_byref(resolution, vmctx) {
        Ok(host_resolution) => host_resolution,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_clock_res_get(clock_id, &mut host_resolution);

    trace!("     | *resolution={:?}", host_resolution);
    encode_timestamp_byref(resolution, host_resolution, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn clock_time_get(
    clock_id: wasm32::wasi_clockid_t,
    precision: wasm32::wasi_timestamp_t,
    time: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "clock_time_get(clock_id={:?}, precision={:?}, time={:#x?})",
        clock_id,
        precision,
        time,
    );

    let vmctx = &mut *vmctx;
    let clock_id = decode_clockid(clock_id);
    let precision = decode_timestamp(precision);
    let mut host_time = match decode_timestamp_byref(time, vmctx) {
        Ok(host_time) => host_time,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_clock_time_get(clock_id, precision, &mut host_time);

    trace!("     | *time={:?}", host_time);
    encode_timestamp_byref(time, host_time, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn condvar_signal(
    _condvar: wasm32::uintptr_t,
    _scope: wasm32::wasi_scope_t,
    _nwaiters: wasm32::wasi_nthreads_t,
    _vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    unimplemented!("wasi_condvar_signal");
}

pub unsafe extern "C" fn fd_close(
    fd: wasm32::wasi_fd_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!("fd_close(fd={:?})", fd);

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let fd = decode_fd(fd);

    let e = host::wasmtime_ssp_fd_close(curfds, fd);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn fd_create1(
    type_: wasm32::wasi_filetype_t,
    fd: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!("fd_create1(type={:?}, fd={:#x?})", type_, fd);

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let type_ = decode_filetype(type_);
    let mut host_fd = match decode_fd_byref(fd, vmctx) {
        Ok(host_fd) => host_fd,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_fd_create1(curfds, type_, &mut host_fd);

    trace!("     | *fd={:?}", host_fd);
    encode_fd_byref(fd, host_fd, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn fd_create2(
    type_: wasm32::wasi_filetype_t,
    fd0: wasm32::uintptr_t,
    fd1: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "fd_create2(type={:?}, fd0={:#x?}, fd1={:#x?})",
        type_,
        fd0,
        fd1
    );

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let type_ = decode_filetype(type_);
    let mut host_fd0 = match decode_fd_byref(fd0, vmctx) {
        Ok(host_fd0) => host_fd0,
        Err(e) => return return_encoded_errno(e),
    };
    let mut host_fd1 = match decode_fd_byref(fd1, vmctx) {
        Ok(host_fd1) => host_fd1,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_fd_create2(curfds, type_, &mut host_fd0, &mut host_fd1);

    trace!("     | *fd0={:?}", host_fd0);
    encode_fd_byref(fd0, host_fd0, vmctx).unwrap();

    trace!("     | *fd1={:?}", host_fd1);
    encode_fd_byref(fd1, host_fd1, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn fd_datasync(
    fd: wasm32::wasi_fd_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!("fd_datasync(fd={:?})", fd);

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let fd = decode_fd(fd);

    let e = host::wasmtime_ssp_fd_datasync(curfds, fd);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn fd_dup(
    from: wasm32::wasi_fd_t,
    fd: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!("fd_dup(from={:?}, fd={:#x?})", from, fd);

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let from = decode_fd(from);
    let mut host_fd = match decode_fd_byref(fd, vmctx) {
        Ok(host_fd) => host_fd,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_fd_dup(curfds, from, &mut host_fd);

    trace!("     | *fd={:?}", host_fd);
    encode_fd_byref(fd, host_fd, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn fd_pread(
    fd: wasm32::wasi_fd_t,
    iovs: wasm32::uintptr_t,
    iovs_len: wasm32::size_t,
    offset: wasm32::wasi_filesize_t,
    nread: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
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
    let iovs = match decode_iovec_slice(iovs, iovs_len, vmctx) {
        Ok(iovs) => iovs,
        Err(e) => return return_encoded_errno(e),
    };
    let offset = decode_filesize(offset);
    let mut host_nread = match decode_usize_byref(nread, vmctx) {
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
    encode_usize_byref(nread, host_nread, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn fd_pwrite(
    fd: wasm32::wasi_fd_t,
    iovs: wasm32::uintptr_t,
    iovs_len: wasm32::size_t,
    offset: wasm32::wasi_filesize_t,
    nwritten: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
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
    let iovs = match decode_ciovec_slice(iovs, iovs_len, vmctx) {
        Ok(iovs) => iovs,
        Err(e) => return return_encoded_errno(e),
    };
    let offset = decode_filesize(offset);
    let mut host_nwritten = match decode_usize_byref(nwritten, vmctx) {
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
    encode_usize_byref(nwritten, host_nwritten, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn fd_read(
    fd: wasm32::wasi_fd_t,
    iovs: wasm32::uintptr_t,
    iovs_len: wasm32::size_t,
    nread: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
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
    let iovs = match decode_iovec_slice(iovs, iovs_len, vmctx) {
        Ok(iovs) => iovs,
        Err(e) => return return_encoded_errno(e),
    };
    let mut host_nread = match decode_usize_byref(nread, vmctx) {
        Ok(host_nread) => host_nread,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_fd_read(curfds, fd, iovs.as_ptr(), iovs.len(), &mut host_nread);

    trace!("     | *nread={:?}", host_nread);
    encode_usize_byref(nread, host_nread, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn fd_replace(
    from: wasm32::wasi_fd_t,
    to: wasm32::wasi_fd_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!("fd_replace(from={:?}, to={:?})", from, to);

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let from = decode_fd(from);
    let to = decode_fd(to);

    let e = host::wasmtime_ssp_fd_replace(curfds, from, to);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn fd_seek(
    fd: wasm32::wasi_fd_t,
    offset: wasm32::wasi_filedelta_t,
    whence: wasm32::wasi_whence_t,
    newoffset: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
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
    let mut host_newoffset = match decode_filesize_byref(newoffset, vmctx) {
        Ok(host_newoffset) => host_newoffset,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_fd_seek(curfds, fd, offset, whence, &mut host_newoffset);

    trace!("     | *newoffset={:?}", host_newoffset);
    encode_filesize_byref(newoffset, host_newoffset, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn fd_stat_get(
    fd: wasm32::wasi_fd_t,
    buf: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!("fd_stat_get(fd={:?}, buf={:#x?})", fd, buf);

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let fd = decode_fd(fd);
    let mut host_buf = match decode_fdstat_byref(buf, vmctx) {
        Ok(host_buf) => host_buf,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_fd_stat_get(curfds, fd, &mut host_buf);

    trace!("     | *buf={:?}", host_buf);
    encode_fdstat_byref(buf, host_buf, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn fd_stat_put(
    fd: wasm32::wasi_fd_t,
    buf: wasm32::uintptr_t,
    flags: wasm32::wasi_fdsflags_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "fd_stat_put(fd={:?}, buf={:#x?}, flags={:#x?})",
        fd,
        buf,
        flags
    );

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let fd = decode_fd(fd);
    let host_buf = match decode_fdstat_byref(buf, vmctx) {
        Ok(host_buf) => host_buf,
        Err(e) => return return_encoded_errno(e),
    };
    let flags = decode_fdsflags(flags);

    let e = host::wasmtime_ssp_fd_stat_put(curfds, fd, &host_buf, flags);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn fd_sync(
    fd: wasm32::wasi_fd_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!("fd_sync(fd={:?})", fd);

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let fd = decode_fd(fd);

    let e = host::wasmtime_ssp_fd_sync(curfds, fd);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn fd_write(
    fd: wasm32::wasi_fd_t,
    iovs: wasm32::uintptr_t,
    iovs_len: wasm32::size_t,
    nwritten: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
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
    let iovs = match decode_ciovec_slice(iovs, iovs_len, vmctx) {
        Ok(iovs) => iovs,
        Err(e) => return return_encoded_errno(e),
    };
    let mut host_nwritten = match decode_usize_byref(nwritten, vmctx) {
        Ok(host_nwritten) => host_nwritten,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_fd_write(curfds, fd, iovs.as_ptr(), iovs.len(), &mut host_nwritten);

    trace!("     | *nwritten={:?}", host_nwritten);
    encode_usize_byref(nwritten, host_nwritten, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn file_advise(
    fd: wasm32::wasi_fd_t,
    offset: wasm32::wasi_filesize_t,
    len: wasm32::wasi_filesize_t,
    advice: wasm32::wasi_advice_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
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
    fd: wasm32::wasi_fd_t,
    offset: wasm32::wasi_filesize_t,
    len: wasm32::wasi_filesize_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!("file_allocate(fd={:?}, offset={}, len={})", fd, offset, len);

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let fd = decode_fd(fd);
    let offset = decode_filesize(offset);
    let len = decode_filesize(len);

    let e = host::wasmtime_ssp_file_allocate(curfds, fd, offset, len);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn file_create(
    fd: wasm32::wasi_fd_t,
    path: wasm32::uintptr_t,
    path_len: wasm32::size_t,
    type_: wasm32::wasi_filetype_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "file_create(fd={:?}, path={:#x?}, path_len={}, type={:?})",
        fd,
        path,
        path_len,
        type_
    );

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let fd = decode_fd(fd);
    let (path, path_len) = match decode_char_slice(path, path_len, vmctx) {
        Ok((path, path_len)) => (path, path_len),
        Err(e) => return return_encoded_errno(e),
    };
    let type_ = decode_filetype(type_);

    let e = host::wasmtime_ssp_file_create(curfds, fd, path, path_len, type_);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn file_link(
    fd0: wasm32::uintptr_t,
    path0: wasm32::uintptr_t,
    path_len0: wasm32::size_t,
    fd1: wasm32::wasi_fd_t,
    path1: wasm32::uintptr_t,
    path_len1: wasm32::size_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "file_link(fd0={:?}, path0={:#x?}, path_len0={}, fd1={:?}, path1={:#x?}, path_len1={})",
        fd0,
        path0,
        path_len0,
        fd1,
        path1,
        path_len1
    );

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let fd0 = match decode_lookup(fd0, vmctx) {
        Ok(fd) => fd,
        Err(e) => return return_encoded_errno(e),
    };
    let (path0, path_len0) = match decode_char_slice(path0, path_len0, vmctx) {
        Ok((path0, path_len0)) => (path0, path_len0),
        Err(e) => return return_encoded_errno(e),
    };
    let fd1 = decode_fd(fd1);
    let (path1, path_len1) = match decode_char_slice(path1, path_len1, vmctx) {
        Ok((path1, path_len1)) => (path1, path_len1),
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_file_link(curfds, fd0, path0, path_len0, fd1, path1, path_len1);

    return_encoded_errno(e)
}

// TODO: When multi-value happens, switch to that instead of passing
// the `fd` by reference?
pub unsafe extern "C" fn file_open(
    dirfd: wasm32::uintptr_t,
    path: wasm32::uintptr_t,
    path_len: wasm32::size_t,
    oflags: wasm32::wasi_oflags_t,
    fds: wasm32::uintptr_t,
    fd: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "file_open(dirfd={:?}, path={:#x?}, path_len={:?}, oflags={:#x?}, fds={:#x?}, fd={:#x?})",
        dirfd,
        path,
        path_len,
        oflags,
        fds,
        fd
    );

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let dirfd = match decode_lookup(dirfd, vmctx) {
        Ok(dirfd) => dirfd,
        Err(e) => return return_encoded_errno(e),
    };
    let (path, path_len) = match decode_char_slice(path, path_len, vmctx) {
        Ok((path, path_len)) => (path, path_len),
        Err(e) => return return_encoded_errno(e),
    };
    let oflags = decode_oflags(oflags);
    let fds = decode_fdstat(fds, vmctx);
    let mut host_fd = match decode_fd_byref(fd, vmctx) {
        Ok(host_fd) => host_fd,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_file_open(curfds, dirfd, path, path_len, oflags, &fds, &mut host_fd);

    trace!("     | *fd={:?}", host_fd);
    encode_fd_byref(fd, host_fd, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn file_readdir(
    fd: wasm32::wasi_fd_t,
    buf: wasm32::uintptr_t,
    buf_len: wasm32::size_t,
    cookie: wasm32::wasi_dircookie_t,
    buf_used: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
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
    let (buf, buf_len) = match decode_char_slice(buf, buf_len, vmctx) {
        Ok((buf, buf_len)) => (buf, buf_len),
        Err(e) => return return_encoded_errno(e),
    };
    let cookie = decode_dircookie(cookie);
    let mut host_buf_used = match decode_usize_byref(buf_used, vmctx) {
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
    encode_usize_byref(buf_used, host_buf_used, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn file_readlink(
    fd: wasm32::wasi_fd_t,
    path: wasm32::uintptr_t,
    path_len: wasm32::size_t,
    buf: wasm32::uintptr_t,
    buf_len: wasm32::size_t,
    buf_used: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
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
    let (path, path_len) = match decode_char_slice(path, path_len, vmctx) {
        Ok((path, path_len)) => (path, path_len),
        Err(e) => return return_encoded_errno(e),
    };
    let (buf, buf_len) = match decode_char_slice(buf, buf_len, vmctx) {
        Ok((buf, buf_len)) => (buf, buf_len),
        Err(e) => return return_encoded_errno(e),
    };
    let mut host_buf_used = match decode_usize_byref(buf_used, vmctx) {
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
    encode_usize_byref(buf_used, host_buf_used, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn file_rename(
    fd0: wasm32::wasi_fd_t,
    path0: wasm32::uintptr_t,
    path_len0: wasm32::size_t,
    fd1: wasm32::wasi_fd_t,
    path1: wasm32::uintptr_t,
    path_len1: wasm32::size_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
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
    let (path0, path_len0) = match decode_char_slice(path0, path_len0, vmctx) {
        Ok((path0, path_len0)) => (path0, path_len0),
        Err(e) => return return_encoded_errno(e),
    };
    let fd1 = decode_fd(fd1);
    let (path1, path_len1) = match decode_char_slice(path1, path_len1, vmctx) {
        Ok((path1, path_len1)) => (path1, path_len1),
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_file_rename(curfds, fd0, path0, path_len0, fd1, path1, path_len1);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn file_stat_fget(
    fd: wasm32::wasi_fd_t,
    buf: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!("file_stat_fget(fd={:?}, buf={:#x?})", fd, buf);

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let fd = decode_fd(fd);
    let mut host_buf = match decode_filestat_byref(buf, vmctx) {
        Ok(host_buf) => host_buf,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_file_stat_fget(curfds, fd, &mut host_buf);

    trace!("     | *buf={:?}", host_buf);
    encode_filestat_byref(buf, host_buf, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn file_stat_fput(
    fd: wasm32::wasi_fd_t,
    buf: wasm32::uintptr_t,
    flags: wasm32::wasi_fsflags_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "file_stat_fput(fd={:?}, buf={:#x?}, flags={:#x?})",
        fd,
        buf,
        flags
    );

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let fd = decode_fd(fd);
    let host_buf = match decode_filestat_byref(buf, vmctx) {
        Ok(host_buf) => host_buf,
        Err(e) => return return_encoded_errno(e),
    };
    let flags = decode_fsflags(flags);

    let e = host::wasmtime_ssp_file_stat_fput(curfds, fd, &host_buf, flags);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn file_stat_get(
    fd: wasm32::uintptr_t,
    path: wasm32::uintptr_t,
    path_len: wasm32::size_t,
    buf: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "file_stat_get(fd={:?}, path={:#x?}, path_len={}, buf={:#x?})",
        fd,
        path,
        path_len,
        buf
    );

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let fd = match decode_lookup(fd, vmctx) {
        Ok(fd) => fd,
        Err(e) => return return_encoded_errno(e),
    };
    let (path, path_len) = match decode_char_slice(path, path_len, vmctx) {
        Ok((path, path_len)) => (path, path_len),
        Err(e) => return return_encoded_errno(e),
    };
    let mut host_buf = match decode_filestat_byref(buf, vmctx) {
        Ok(host_buf) => host_buf,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_file_stat_get(curfds, fd, path, path_len, &mut host_buf);

    trace!("     | *buf={:?}", host_buf);
    encode_filestat_byref(buf, host_buf, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn file_stat_put(
    fd: wasm32::uintptr_t,
    path: wasm32::uintptr_t,
    path_len: wasm32::size_t,
    buf: wasm32::uintptr_t,
    flags: wasm32::wasi_fsflags_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "file_stat_put(fd={:?}, path={:#x?}, path_leb={}, buf={:#x?}, flags={:#x?})",
        path,
        path_len,
        fd,
        buf,
        flags
    );

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let fd = match decode_lookup(fd, vmctx) {
        Ok(fd) => fd,
        Err(e) => return return_encoded_errno(e),
    };
    let (path, path_len) = match decode_char_slice(path, path_len, vmctx) {
        Ok((path, path_len)) => (path, path_len),
        Err(e) => return return_encoded_errno(e),
    };
    let host_buf = match decode_filestat_byref(buf, vmctx) {
        Ok(host_buf) => host_buf,
        Err(e) => return return_encoded_errno(e),
    };
    let flags = decode_fsflags(flags);

    let e = host::wasmtime_ssp_file_stat_put(curfds, fd, path, path_len, &host_buf, flags);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn file_symlink(
    path0: wasm32::uintptr_t,
    path_len0: wasm32::size_t,
    fd: wasm32::wasi_fd_t,
    path1: wasm32::uintptr_t,
    path_len1: wasm32::size_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
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
    let (path0, path_len0) = match decode_char_slice(path0, path_len0, vmctx) {
        Ok((path0, path_len0)) => (path0, path_len0),
        Err(e) => return return_encoded_errno(e),
    };
    let fd = decode_fd(fd);
    let (path1, path_len1) = match decode_char_slice(path1, path_len1, vmctx) {
        Ok((path1, path_len1)) => (path1, path_len1),
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_file_symlink(curfds, path0, path_len0, fd, path1, path_len1);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn file_unlink(
    fd: wasm32::wasi_fd_t,
    path: wasm32::uintptr_t,
    path_len: wasm32::size_t,
    flags: wasm32::wasi_ulflags_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
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
    let (path, path_len) = match decode_char_slice(path, path_len, vmctx) {
        Ok((path, path_len)) => (path, path_len),
        Err(e) => return return_encoded_errno(e),
    };
    let flags = decode_ulflags(flags);

    let e = host::wasmtime_ssp_file_unlink(curfds, fd, path, path_len, flags);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn lock_unlock(
    _lock: wasm32::uintptr_t,
    _scope: wasm32::wasi_scope_t,
    _vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    unimplemented!("wasi_lock_unlock");
}

pub unsafe extern "C" fn mem_advise(
    _mapping: wasm32::uintptr_t,
    _mapping_len: wasm32::size_t,
    _advice: wasm32::wasi_advice_t,
    _vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    unimplemented!("wasi_mem_advise");
}

pub unsafe extern "C" fn mem_map(
    addr: wasm32::uintptr_t,
    len: wasm32::size_t,
    prot: wasm32::wasi_mprot_t,
    flags: wasm32::wasi_mflags_t,
    fd: wasm32::wasi_fd_t,
    off: wasm32::wasi_filesize_t,
    mem: wasm32::uintptr_t,
    _vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "mem_map(addr={:#x?}, len={}, prot={:#x?}, flags={:#x?}, fd={:#x?}, off={:?} mem={:#x?})",
        addr,
        len,
        prot,
        flags,
        fd,
        off,
        mem
    );

    unimplemented!("wasi_mem_map");
}

pub unsafe extern "C" fn mem_protect(
    _mapping: wasm32::uintptr_t,
    _mapping_len: wasm32::size_t,
    _prot: wasm32::wasi_mprot_t,
    _vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    unimplemented!("wasi_mem_protect");
}

pub unsafe extern "C" fn mem_sync(
    _mapping: wasm32::uintptr_t,
    _mapping_len: wasm32::size_t,
    _flags: wasm32::wasi_msflags_t,
    _vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    unimplemented!("wasi_mem_sync");
}

pub unsafe extern "C" fn mem_unmap(
    mapping: wasm32::uintptr_t,
    mapping_len: wasm32::size_t,
    _vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "mem_unmap(mapping={:#x?}, mapping_len={:?})",
        mapping,
        mapping_len
    );

    unimplemented!("wasi_mem_unmap");
}

pub unsafe extern "C" fn poll(
    in_: wasm32::uintptr_t,
    out: wasm32::uintptr_t,
    nsubscriptions: wasm32::size_t,
    nevents: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "poll(in={:#x?}, out={:#x?}, nsubscriptions={}, nevents={:#x?})",
        in_,
        out,
        nsubscriptions,
        nevents,
    );

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let in_ = match decode_subscription_slice(in_, nsubscriptions, vmctx) {
        Ok(in_) => in_,
        Err(e) => return return_encoded_errno(e),
    };
    let mut host_out = match decode_event_slice(out, nsubscriptions, vmctx) {
        Ok(out) => out,
        Err(e) => return return_encoded_errno(e),
    };
    let mut host_nevents = match decode_usize_byref(nevents, vmctx) {
        Ok(host_nevents) => host_nevents,
        Err(e) => return return_encoded_errno(e),
    };

    assert!(in_.len() == host_out.len());

    let e = host::wasmtime_ssp_poll(
        curfds,
        in_.as_ptr(),
        host_out.as_mut_ptr(),
        in_.len(),
        &mut host_nevents,
    );

    trace!("     | *nevents={:?}", host_nevents);
    encode_usize_byref(nevents, host_nevents, vmctx).unwrap();

    if log_enabled!(log::Level::Trace) {
        for (index, _event) in host_out.iter().enumerate() {
            // TODO: Format the output for tracing.
            trace!("     | *out[{}]=...", index);
        }
    }
    encode_event_slice(out, host_out, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn proc_exit(rval: u32, _vmctx: *mut VMContext) -> ! {
    trace!("proc_exec(rval={:?})", rval);

    let rval = decode_exitcode(rval);

    // TODO: Rather than call wasi_proc_exit here, we should trigger a
    // stack unwind similar to a trap.

    panic!("wasi_proc_exit({})", rval)
}

pub unsafe extern "C" fn proc_raise(
    _sig: wasm32::wasi_signal_t,
    _vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    unimplemented!("wasi_proc_raise");
}

pub unsafe extern "C" fn random_get(
    buf: wasm32::uintptr_t,
    buf_len: wasm32::size_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!("random_get(buf={:#x?}, buf_len={:?})", buf, buf_len);

    let vmctx = &mut *vmctx;
    let (buf, buf_len) = match decode_char_slice(buf, buf_len, vmctx) {
        Ok((buf, buf_len)) => (buf, buf_len),
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_random_get(buf as *mut host::void, buf_len);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn sock_recv(
    sock: wasm32::wasi_fd_t,
    in_: wasm32::uintptr_t,
    out: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "sock_recv(sock={:?}, in={:#x?}, out={:#x?})",
        sock,
        in_,
        out
    );

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let sock = decode_fd(sock);
    let in_ = match decode_recv_in_byref(in_, vmctx) {
        Ok(in_) => in_,
        Err(e) => return return_encoded_errno(e),
    };
    let mut host_out = match decode_recv_out_byref(out, vmctx) {
        Ok(host_out) => host_out,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_sock_recv(curfds, sock, &in_, &mut host_out);

    // TODO: Format the output for tracing.
    trace!("     | *out=...");
    encode_recv_out_byref(out, host_out, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn sock_send(
    sock: wasm32::wasi_fd_t,
    in_: wasm32::uintptr_t,
    out: wasm32::uintptr_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!(
        "sock_send(sock={:?}, in={:#x?}, out={:#x?})",
        sock,
        in_,
        out
    );

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let sock = decode_fd(sock);
    let in_ = match decode_send_in_byref(in_, vmctx) {
        Ok(in_) => in_,
        Err(e) => return return_encoded_errno(e),
    };
    let mut host_out = match decode_send_out_byref(out, vmctx) {
        Ok(host_out) => host_out,
        Err(e) => return return_encoded_errno(e),
    };

    let e = host::wasmtime_ssp_sock_send(curfds, sock, &in_, &mut host_out);

    trace!("     | *out={:?}", host_out);
    encode_send_out_byref(out, host_out, vmctx).unwrap();

    return_encoded_errno(e)
}

pub unsafe extern "C" fn sock_shutdown(
    sock: wasm32::wasi_fd_t,
    how: wasm32::wasi_sdflags_t,
    vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    trace!("sock_shutdown(sock={:?}, how={:?})", sock, how);

    let vmctx = &mut *vmctx;
    let curfds = get_curfds(vmctx);
    let sock = decode_fd(sock);
    let how = decode_sdflags(how);

    let e = host::wasmtime_ssp_sock_shutdown(curfds, sock, how);

    return_encoded_errno(e)
}

pub unsafe extern "C" fn thread_create(
    _attr: wasm32::uintptr_t,
    _tid: wasm32::uintptr_t,
    _vmctx: *mut VMContext,
) -> wasm32::wasi_errno_t {
    unimplemented!("wasi_thread_create");
}

pub unsafe extern "C" fn thread_exit(
    _lock: wasm32::uintptr_t,
    _scope: wasm32::wasi_scope_t,
    _vmctx: *mut VMContext,
) -> ! {
    unimplemented!("wasi_thread_exit");
}

pub unsafe extern "C" fn thread_yield(_vmctx: *mut VMContext) -> wasm32::wasi_errno_t {
    unimplemented!("wasi_thread_yield");
}
