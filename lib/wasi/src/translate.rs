use cast;
use cast::From as _0;
use host;
use std::mem::{align_of, size_of};
use std::slice;
use wasm32;
use wasmtime_runtime::{Export, VMContext};

/// Translate a wasm pointer into a native pointer.
///
/// This is unsafe due to trusting the contents of vmctx. The pointer result
/// is bounds and alignment checked.
unsafe fn decode_ptr(
    ptr: wasm32::uintptr_t,
    len: usize,
    align: usize,
    vmctx: &mut VMContext,
) -> Result<*mut u8, host::wasi_errno_t> {
    match vmctx.lookup_global_export("memory") {
        Some(Export::Memory {
            definition,
            vmctx: _,
            memory: _,
        }) => {
            if len > 0 {
                // Check for overflow within the access.
                let last = match (ptr as usize).checked_add(len - 1) {
                    Some(sum) => sum,
                    None => {
                        println!("!!! overflow");
                        return Err(host::WASI_EFAULT as host::wasi_errno_t);
                    }
                };
                // Check for out of bounds.
                if last >= (*definition).current_length {
                    println!("!!! out of bounds");
                    return Err(host::WASI_EFAULT as host::wasi_errno_t);
                }
            }
            // Check alignment.
            if (ptr as usize) % align != 0 {
                println!("!!! bad alignment: {} % {}", ptr, align);
                return Err(host::WASI_EINVAL as host::wasi_errno_t);
            }
            // Ok, translate the address.
            Ok((((*definition).base as usize) + (ptr as usize)) as *mut u8)
        }
        // No export named "wasi_memory", or the export isn't a memory.
        // FIXME: Is EINVAL the best code here?
        x => {
            println!(
                "!!! no export named wasi_memory, or the export isn't a mem: {:?}",
                x
            );
            Err(host::WASI_EINVAL as host::wasi_errno_t)
        }
    }
}

unsafe fn decode_ptr_to<T>(
    ptr: wasm32::uintptr_t,
    vmctx: &mut VMContext,
) -> Result<*mut T, host::wasi_errno_t> {
    decode_ptr(ptr, size_of::<T>(), align_of::<T>(), vmctx).map(|ptr| ptr as *mut T)
}

unsafe fn decode_pointee<T>(
    ptr: wasm32::uintptr_t,
    vmctx: &mut VMContext,
) -> Result<T, host::wasi_errno_t> {
    let ptr = decode_ptr_to::<T>(ptr, vmctx)?;

    // Size and alignment are checked by `decode_ptr_to`.
    Ok(ptr.read())
}

pub unsafe fn encode_pointee<T>(
    ptr: wasm32::uintptr_t,
    t: T,
    vmctx: &mut VMContext,
) -> Result<(), host::wasi_errno_t> {
    let ptr = decode_ptr_to::<T>(ptr, vmctx)?;

    // Size and alignment are checked by `decode_ptr_to`.
    Ok(ptr.write(t))
}

pub unsafe fn decode_slice_of<T>(
    ptr: wasm32::uintptr_t,
    len: wasm32::size_t,
    vmctx: &mut VMContext,
) -> Result<(*mut T, usize), host::wasi_errno_t> {
    let len = cast::usize(len);

    let ptr = decode_ptr(
        ptr,
        size_of::<T>().checked_mul(len).unwrap(),
        align_of::<T>(),
        vmctx,
    )? as *mut T;

    Ok((ptr, len))
}

pub fn decode_usize(len: wasm32::size_t) -> usize {
    cast::usize(len)
}

pub unsafe fn decode_filesize(filesize: wasm32::wasi_filesize_t) -> host::wasi_filesize_t {
    filesize
}

pub unsafe fn decode_lookup(
    lookup_ptr: wasm32::uintptr_t,
    vmctx: &mut VMContext,
) -> Result<host::wasi_lookup_t, host::wasi_errno_t> {
    let lookup = decode_pointee::<wasm32::wasi_lookup_t>(lookup_ptr, vmctx)?;
    Ok(host::wasi_lookup_t {
        fd: decode_fd(lookup.fd),
        flags: decode_lookupflags(lookup.flags),
    })
}

pub fn decode_fd(fd: wasm32::wasi_fd_t) -> host::wasi_fd_t {
    fd
}

pub fn decode_filedelta(filedelta: wasm32::wasi_filedelta_t) -> host::wasi_filedelta_t {
    filedelta
}

pub fn decode_whence(whence: wasm32::wasi_whence_t) -> host::wasi_whence_t {
    whence
}

pub fn decode_clockid(_clockid: wasm32::wasi_clockid_t) -> host::wasi_clockid_t {
    unimplemented!("decode_clockid");
}

pub fn decode_timestamp(timestamp: wasm32::wasi_timestamp_t) -> host::wasi_timestamp_t {
    timestamp
}

pub fn decode_exitcode(exitcode: wasm32::wasi_exitcode_t) -> host::wasi_exitcode_t {
    exitcode
}

pub fn decode_lookupflags(lookupflags: wasm32::wasi_lookupflags_t) -> host::wasi_lookupflags_t {
    lookupflags
}

pub fn decode_oflags(_oflags: wasm32::wasi_oflags_t) -> host::wasi_oflags_t {
    unimplemented!("decode_oflags");
}

#[allow(dead_code)]
pub fn decode_mflags(mflags: wasm32::wasi_mflags_t) -> host::wasi_mflags_t {
    mflags
}

#[allow(dead_code)]
pub fn decode_mprot(mprot: wasm32::wasi_mprot_t) -> host::wasi_mprot_t {
    mprot
}

pub fn decode_advice(advice: wasm32::wasi_advice_t) -> host::wasi_advice_t {
    advice
}

pub fn decode_dircookie(dircookie: wasm32::wasi_dircookie_t) -> host::wasi_dircookie_t {
    dircookie
}

pub fn decode_fdsflags(fdsflags: wasm32::wasi_fdsflags_t) -> host::wasi_fdsflags_t {
    fdsflags
}

pub fn decode_filetype(filetype: wasm32::wasi_filetype_t) -> host::wasi_filetype_t {
    filetype
}

pub fn decode_fsflags(fsflags: wasm32::wasi_fsflags_t) -> host::wasi_fsflags_t {
    fsflags
}

pub fn decode_sdflags(sdflags: wasm32::wasi_sdflags_t) -> host::wasi_sdflags_t {
    sdflags
}

pub fn decode_ulflags(ulflags: wasm32::wasi_ulflags_t) -> host::wasi_ulflags_t {
    ulflags
}

pub fn decode_fdstat(
    _fdstat_ptr: wasm32::uintptr_t,
    _vmctx: &mut VMContext,
) -> host::wasi_fdstat_t {
    unimplemented!("decode_oflags");
}

pub unsafe fn decode_char_slice(
    ptr: wasm32::uintptr_t,
    len: wasm32::size_t,
    vmctx: &mut VMContext,
) -> Result<(*mut host::char, usize), host::wasi_errno_t> {
    println!("decode_char_slice len={}", len);
    decode_slice_of::<host::char>(ptr, len, vmctx)
}

pub unsafe fn decode_ciovec(
    ciovec: &wasm32::wasi_ciovec_t,
    vmctx: &mut VMContext,
) -> Result<host::wasi_ciovec_t, host::wasi_errno_t> {
    let len = cast::usize(ciovec.buf_len);
    Ok(host::wasi_ciovec_t {
        buf: decode_ptr(ciovec.buf, len, 1, vmctx)? as *const host::void,
        buf_len: len,
    })
}

pub unsafe fn decode_iovec(
    iovec: &wasm32::wasi_iovec_t,
    vmctx: &mut VMContext,
) -> Result<host::wasi_iovec_t, host::wasi_errno_t> {
    let len = cast::usize(iovec.buf_len);
    Ok(host::wasi_iovec_t {
        buf: decode_ptr(iovec.buf, len, 1, vmctx)? as *mut host::void,
        buf_len: len,
    })
}

pub unsafe fn decode_ciovec_slice(
    ptr: wasm32::uintptr_t,
    len: wasm32::size_t,
    vmctx: &mut VMContext,
) -> Result<Vec<host::wasi_ciovec_t>, host::wasi_errno_t> {
    let slice = decode_slice_of::<wasm32::wasi_ciovec_t>(ptr, len, vmctx)?;
    let slice = slice::from_raw_parts(slice.0, slice.1);
    slice.iter().map(|iov| decode_ciovec(iov, vmctx)).collect()
}

pub unsafe fn decode_iovec_slice(
    ptr: wasm32::uintptr_t,
    len: wasm32::size_t,
    vmctx: &mut VMContext,
) -> Result<Vec<host::wasi_iovec_t>, host::wasi_errno_t> {
    let slice = decode_slice_of::<wasm32::wasi_iovec_t>(ptr, len, vmctx)?;
    let slice = slice::from_raw_parts(slice.0, slice.1);
    slice.iter().map(|iov| decode_iovec(iov, vmctx)).collect()
}

pub unsafe fn decode_subscription(
    _subscription: wasm32::wasi_subscription_t,
    _vmctx: &mut VMContext,
) -> host::wasi_subscription_t {
    unimplemented!("decode_subscription");
}

pub unsafe fn decode_subscription_slice(
    ptr: wasm32::uintptr_t,
    len: wasm32::size_t,
    vmctx: &mut VMContext,
) -> Result<Vec<host::wasi_subscription_t>, host::wasi_errno_t> {
    let slice = decode_slice_of::<wasm32::wasi_subscription_t>(ptr, len, vmctx)?;
    let slice = slice::from_raw_parts(slice.0, slice.1);
    Ok(slice
        .iter()
        .map(|subscription| decode_subscription(*subscription, vmctx))
        .collect())
}

pub unsafe fn decode_event(
    _event: wasm32::wasi_event_t,
    _vmctx: &mut VMContext,
) -> host::wasi_event_t {
    unimplemented!("decode_event");
}

pub unsafe fn decode_event_slice(
    ptr: wasm32::uintptr_t,
    len: wasm32::size_t,
    vmctx: &mut VMContext,
) -> Result<Vec<host::wasi_event_t>, host::wasi_errno_t> {
    let slice = decode_slice_of::<wasm32::wasi_event_t>(ptr, len, vmctx)?;
    let slice = slice::from_raw_parts(slice.0, slice.1);
    Ok(slice
        .iter()
        .map(|event| decode_event(*event, vmctx))
        .collect())
}

pub unsafe fn encode_event_slice(
    _ptr: wasm32::uintptr_t,
    _host: Vec<host::wasi_event_t>,
    _vmctx: &mut VMContext,
) -> Result<(), host::wasi_errno_t> {
    unimplemented!("encode_event_slice");
}

pub unsafe fn decode_fd_byref(
    _fd_ptr: wasm32::uintptr_t,
    _vmctx: &mut VMContext,
) -> Result<host::wasi_fd_t, host::wasi_errno_t> {
    unimplemented!("decode_fd_byref");
}

pub unsafe fn encode_fd_byref(
    _fd_ptr: wasm32::uintptr_t,
    _fd: host::wasi_fd_t,
    _vmctx: &mut VMContext,
) -> Result<(), host::wasi_errno_t> {
    unimplemented!("encode_fd_byref");
}

pub unsafe fn decode_timestamp_byref(
    timestamp_ptr: wasm32::uintptr_t,
    vmctx: &mut VMContext,
) -> Result<host::wasi_timestamp_t, host::wasi_errno_t> {
    decode_pointee::<wasm32::wasi_timestamp_t>(timestamp_ptr, vmctx)
        .map(host::wasi_timestamp_t::cast)
}

pub unsafe fn encode_timestamp_byref(
    timestamp_ptr: wasm32::uintptr_t,
    host_timestamp: host::wasi_timestamp_t,
    vmctx: &mut VMContext,
) -> Result<(), host::wasi_errno_t> {
    encode_pointee::<wasm32::wasi_timestamp_t>(
        timestamp_ptr,
        wasm32::wasi_timestamp_t::cast(host_timestamp),
        vmctx,
    )
}

pub unsafe fn decode_filesize_byref(
    filesize_ptr: wasm32::uintptr_t,
    vmctx: &mut VMContext,
) -> Result<host::wasi_filesize_t, host::wasi_errno_t> {
    decode_pointee::<wasm32::wasi_filesize_t>(filesize_ptr, vmctx).map(host::wasi_filesize_t::cast)
}

pub unsafe fn encode_filesize_byref(
    filesize_ptr: wasm32::uintptr_t,
    host_filesize: host::wasi_filesize_t,
    vmctx: &mut VMContext,
) -> Result<(), host::wasi_errno_t> {
    encode_pointee::<wasm32::wasi_filesize_t>(
        filesize_ptr,
        wasm32::wasi_filesize_t::cast(host_filesize),
        vmctx,
    )
}

pub unsafe fn decode_usize_byref(
    usize_ptr: wasm32::uintptr_t,
    vmctx: &mut VMContext,
) -> Result<usize, host::wasi_errno_t> {
    decode_pointee::<wasm32::size_t>(usize_ptr, vmctx).map(decode_usize)
}

pub unsafe fn encode_usize_byref(
    usize_ptr: wasm32::uintptr_t,
    host_usize: usize,
    vmctx: &mut VMContext,
) -> Result<(), host::wasi_errno_t> {
    encode_pointee::<wasm32::size_t>(usize_ptr, wasm32::size_t::cast(host_usize).unwrap(), vmctx)
}

pub unsafe fn decode_fdstat_byref(
    _fdstat_ptr: wasm32::uintptr_t,
    _vmctx: &mut VMContext,
) -> Result<host::wasi_fdstat_t, host::wasi_errno_t> {
    unimplemented!("decode_fdstat_byref");
}

pub unsafe fn encode_fdstat_byref(
    _fdstat_ptr: wasm32::uintptr_t,
    _host_fdstat: host::wasi_fdstat_t,
    _vmctx: &mut VMContext,
) -> Result<(), host::wasi_errno_t> {
    unimplemented!("encode_fdstat_byref");
}

pub unsafe fn decode_filestat_byref(
    _filestat_ptr: wasm32::uintptr_t,
    _vmctx: &mut VMContext,
) -> Result<host::wasi_filestat_t, host::wasi_errno_t> {
    unimplemented!("decode_filestat_byref");
}

pub unsafe fn encode_filestat_byref(
    _filestat_ptr: wasm32::uintptr_t,
    _host_filestat: host::wasi_filestat_t,
    _vmctx: &mut VMContext,
) -> Result<(), host::wasi_errno_t> {
    unimplemented!("encode_filestat_byref");
}

pub unsafe fn decode_recv_out_byref(
    _recv_out_ptr: wasm32::uintptr_t,
    _vmctx: &mut VMContext,
) -> Result<host::wasi_recv_out_t, host::wasi_errno_t> {
    unimplemented!("decode_recv_out_byref");
}

pub unsafe fn encode_recv_out_byref(
    _recv_out_ptr: wasm32::uintptr_t,
    _host_recv_out: host::wasi_recv_out_t,
    _vmctx: &mut VMContext,
) -> Result<(), host::wasi_errno_t> {
    unimplemented!("encode_recv_out_byref");
}

pub unsafe fn decode_send_out_byref(
    _send_out_ptr: wasm32::uintptr_t,
    _vmctx: &mut VMContext,
) -> Result<host::wasi_send_out_t, host::wasi_errno_t> {
    unimplemented!("decode_send_out_byref");
}

pub unsafe fn encode_send_out_byref(
    _send_out_ptr: wasm32::uintptr_t,
    _host_send_out: host::wasi_send_out_t,
    _vmctx: &mut VMContext,
) -> Result<(), host::wasi_errno_t> {
    unimplemented!("encode_send_out_byref");
}

pub unsafe fn decode_send_in_byref(
    _send_in_ptr: wasm32::uintptr_t,
    _vmctx: &mut VMContext,
) -> Result<host::wasi_send_in_t, host::wasi_errno_t> {
    unimplemented!("decode_send_in_byref");
}

pub unsafe fn decode_recv_in_byref(
    _recv_in_ptr: wasm32::uintptr_t,
    _vmctx: &mut VMContext,
) -> Result<host::wasi_recv_in_t, host::wasi_errno_t> {
    unimplemented!("decode_recv_in_byref");
}

pub fn encode_errno(e: host::wasi_errno_t) -> wasm32::wasi_errno_t {
    assert!(e <= wasm32::WASI_ENOTCAPABLE);
    e
}
