// To implement `proc_exit`, we define a custom exception object
// that we can throw to unwind the stack and carry the exit value.
function WASIExit(return_value, message, fileName, lineNumber) {
    let instance = new Error(message, fileName, lineNumber);
    instance.return_value = return_value;
    Object.setPrototypeOf(instance, Object.getPrototypeOf(this));
    if (Error.captureStackTrace) {
        Error.captureStackTrace(instance, WASIExit);
    }
    return instance;
}

WASIExit.prototype = Object.create(Error.prototype, {
    constructor: {
      value: Error,
      enumerable: false,
      writable: true,
      configurable: true
    }
});

if (Object.setPrototypeOf) {
    Object.setPrototypeOf(WASIExit, Error);
} else {
    WASIExit.__proto__ = Error;
}

function handleWASIExit(e) {
    if (e.return_value != 0) {
        console.log('program exited with non-zero exit status ' + e.return_value);
    }
}

// There are two heaps in play, the guest heap, which belongs to the WASI-using
// program, and the host heap, which belongs to the Emscripten-compiled polyfill
// library. The following declare support for the guest heap in a similar manner
// to Emscripten's heap.

var GUEST_HEAP,
/** @type {ArrayBuffer} */
  GUEST_buffer,
/** @type {Int8Array} */
  GUEST_HEAP8,
/** @type {Uint8Array} */
  GUEST_HEAPU8,
/** @type {Int16Array} */
  GUEST_HEAP16,
/** @type {Uint16Array} */
  GUEST_HEAPU16,
/** @type {Int32Array} */
  GUEST_HEAP32,
/** @type {Uint32Array} */
  GUEST_HEAPU32,
/** @type {Float32Array} */
  GUEST_HEAPF32,
/** @type {Float64Array} */
  GUEST_HEAPF64;

function updateGuestBuffer(buf) {
  Module['GUEST_buffer'] = GUEST_buffer = buf;
}

function updateGuestBufferViews() {
  Module['GUEST_HEAP8'] = GUEST_HEAP8 = new Int8Array(GUEST_buffer);
  Module['GUEST_HEAP16'] = GUEST_HEAP16 = new Int16Array(GUEST_buffer);
  Module['GUEST_HEAP32'] = GUEST_HEAP32 = new Int32Array(GUEST_buffer);
  Module['GUEST_HEAPU8'] = GUEST_HEAPU8 = new Uint8Array(GUEST_buffer);
  Module['GUEST_HEAPU16'] = GUEST_HEAPU16 = new Uint16Array(GUEST_buffer);
  Module['GUEST_HEAPU32'] = GUEST_HEAPU32 = new Uint32Array(GUEST_buffer);
  Module['GUEST_HEAPF32'] = GUEST_HEAPF32 = new Float32Array(GUEST_buffer);
  Module['GUEST_HEAPF64'] = GUEST_HEAPF64 = new Float64Array(GUEST_buffer);
}

function copyin_bytes(src, len) {
    let dst = _malloc(len);
    for (let i = 0; i < len; ++i) {
        HEAP8[dst + i] = GUEST_HEAP8[src + i];
    }
    return dst;
}

function copyout_bytes(dst, src, len) {
    for (let i = 0; i < len; ++i) {
        GUEST_HEAP8[dst + i] = HEAP8[src + i];
    }
    _free(src);
}

function copyout_i32(dst, src) {
    GUEST_HEAP32[dst>>2] = HEAP32[src>>2];
    _free(src);
}

function copyout_i64(dst, src) {
    GUEST_HEAP32[dst>>2] = HEAP32[src>>2];
    GUEST_HEAP32[(dst + 4)>>2] = HEAP32[(src + 4)>>2];
    _free(src);
}

function translate_ciovs(iovs, iovs_len) {
    host_iovs = _malloc(8 * iovs_len);
    for (let i = 0; i < iovs_len; ++i) {
        let ptr = GUEST_HEAP32[(iovs + i * 8 + 0) >> 2];
        let len = GUEST_HEAP32[(iovs + i * 8 + 4) >> 2];
        let buf = copyin_bytes(ptr, len);
        HEAP32[(host_iovs + i * 8 + 0)>>2] = buf;
        HEAP32[(host_iovs + i * 8 + 4)>>2] = len;
    }
    return host_iovs;
}

function free_ciovs(host_iovs, iovs_len) {
    for (let i = 0; i < iovs_len; ++i) {
        let buf = HEAP32[(host_iovs + i * 8 + 0) >> 2];
        _free(buf);
    }
    _free(host_iovs);
}

function translate_iovs(iovs, iovs_len) {
    host_iovs = _malloc(8 * iovs_len);
    for (let i = 0; i < iovs_len; ++i) {
        let len = GUEST_HEAP32[(iovs + i * 8 + 4) >> 2];
        let buf = _malloc(len);
        HEAP32[(host_iovs + i * 8 + 0)>>2] = buf;
        HEAP32[(host_iovs + i * 8 + 4)>>2] = len;
    }
    return host_iovs;
}

function free_iovs(host_iovs, iovs_len, iovs) {
    for (let i = 0; i < iovs_len; ++i) {
        let buf = HEAP32[(host_iovs + i * 8 + 0) >> 2];
        let len = HEAP32[(host_iovs + i * 8 + 4) >> 2];
        let ptr = GUEST_HEAP32[(host_iovs + i * 8 + 0) >> 2];
        copyout_bytes(ptr, buf, len);
    }
    _free(host_iovs);
}

var WASIPolyfill = {

clock_res_get: function(clock_id, resolution) {
    let host_resolution = _malloc(8);
    let ret = ___wasi_clock_res_get(clock_id, host_resolution);
    copyout_i64(resolution, host_resolution);
    return ret;
},

clock_time_get: function(clock_id, precision, time) {
    let host_time = _malloc(8);
    let ret = ___wasi_clock_time_get(clock_id, precision, host_time);
    copyout_i64(time, host_time);
    return ret;
},

fd_close: function(fd) {
    return ___wasi_fd_close(fd);
},

fd_datasync: function(fd) {
    return ___wasi_fd_datasync(fd);
},

fd_pread: function(fd, iovs, iovs_len, offset, nread) {
    let host_iovs = translate_iovs(iovs, iovs_len);
    let host_nread = _malloc(4);
    let ret = ___wasi_fd_pread(fd, host_iovs, iovs_len, offset, host_nread);
    copyout_i32(nread, host_nread);
    free_iovs(host_iovs, iovs_len);
    return ret;
},

fd_pwrite: function(fd, iovs, iovs_len, offset, nwritten) {
    let host_iovs = translate_ciovs(iovs, iovs_len);
    let host_nwritten = _malloc(4);
    let ret = ___wasi_fd_pwrite(fd, host_iovs, iovs_len, offset, host_nwritten);
    copyout_i32(nwritten, host_nwritten);
    free_ciovs(host_iovs, iovs_len);
    return ret;
},

fd_read: function(fd, iovs, iovs_len, ri_flags, nread) {
    let host_iovs = translate_iovs(iovs, iovs_len);
    let host_nread = _malloc(4);
    let ret = ___wasi_fd_read(fd, host_iovs, iovs_len, ri_flags, host_nread);
    copyout_i32(nread, host_nread);
    free_iovs(host_iovs, iovs_len);
    return ret;
},

fd_renumber: function(from, to) {
    return ___wasi_fd_renumber(from, to);
},

fd_seek: function(fd, offset, whence, newoffset) {
    let host_newoffset = _malloc(8);
    let ret = ___wasi_fd_seek(fd, offset, whence, host_newoffset);
    copyout_i64(newoffset, host_newoffset);
    return ret;
},

fd_tell: function(fd, newoffset) {
    let host_newoffset = _malloc(8);
    let ret = ___wasi_fd_seek(fd, host_newoffset);
    copyout_i64(newoffset, host_newoffset);
    return ret;
},

fd_fdstat_get: function(fd, buf) {
    let host_buf = _malloc(24); // sizeof __wasi_fdstat_t
    let ret = ___wasi_fd_fdstat_get(fd, host_buf);
    copyout_bytes(buf, host_buf, 24);
    return ret;
},

fd_fdstat_set_flags: function(fd, flags) {
    return ___wasi_fd_fdstat_set_flags(fd, flags);
},

fd_fdstat_set_rights: function(fd, fs_rights_base, fs_rights_inheriting) {
    return ___wasi_fd_fdstat_set_rights(fd, fs_rights_base, fs_rights_inheriting);
},

fd_sync: function(fd) {
    return ___wasi_fd_sync(fd);
},

fd_write: function(fd, iovs, iovs_len, si_flags, nwritten) {
    let host_iovs = translate_ciovs(iovs, iovs_len);
    let host_nwritten = _malloc(4);
    let ret = ___wasi_fd_write(fd, host_iovs, iovs_len, si_flags, host_nwritten);
    copyout_i32(nwritten, host_nwritten);
    free_ciovs(host_iovs, iovs_len);
    return ret;
},

fd_advise: function(fd, offset, len, advice) {
    return ___wasi_fd_advise(fd, offset, len, advice);
},

fd_allocate: function(fd, offset, len) {
    return ___wasi_fd_allocate(fd, offset, len);
},

path_create_directory: function(fd, path, path_len) {
    let host_path = copyin_bytes(path, path_len);
    let ret = ___wasi_path_create_directory(fd, host_path, path_len);
    _free(host_path);
    return ret;
},

path_link: function(fd0, path0, path_len0, fd1, path1, path_len1) {
    let host_path0 = copyin_bytes(path0, path_len0);
    let host_path1 = copyin_bytes(path1, path_len1);
    let ret = ___wasi_path_link(fd, host_path0, path_len0, fd1, host_path1, path1_len);
    _free(host_path1);
    _free(host_path0);
    return ret;
},

path_open: function(dirfd, dirflags, path, path_len, oflags, fs_rights_base, fs_rights_inheriting, fs_flags, fd) {
    let host_path = copyin_bytes(path, path_len);
    let host_fd = _malloc(4);
    let ret = ___wasi_path_open(dirfd, dirflags, host_path, path_len, oflags, fs_rights_base, fs_rights_inheriting, fs_flags, host_fd);
    copyout_i32(fd, host_fd);
    _free(host_path);
    return ret;
},

fd_readdir: function(fd, buf, buf_len, cookie, buf_used) {
    let host_buf = _malloc(buf_len);
    let host_buf_used = _malloc(4);
    let ret = ___wasi_fd_readdir(fd, buf, buf_len, cookie, host_buf_used);
    copyout_i32(buf_used, host_buf_used);
    copyout_bytes(buf, host_buf, buf_len);
    return ret;
},

path_readlink: function(fd, path, path_len, buf, buf_len, buf_used) {
    let host_path = copyin_bytes(path, path_len);
    let host_buf = _malloc(buf_len);
    let host_buf_used = _malloc(4);
    let ret = ___wasi_path_readlink(fd, path, path_len, buf, buf_len, host_buf_used);
    copyout_i32(buf_used, host_buf_used);
    copyout_bytes(buf, host_buf, buf_len);
    _free(host_path);
    return ret;
},

path_rename: function(fd0, path0, path_len0, fd1, path1, path_len1) {
    let host_path0 = copyin_bytes(path0, path_len0);
    let host_path1 = copyin_bytes(path1, path_len1);
    let ret = ___wasi_path_rename(fd, host_path0, path_len0, fd1, host_path1, path1_len);
    _free(host_path1);
    _free(host_path0);
    return ret;
},

fd_filestat_get: function(fd, buf) {
    let host_buf = _malloc(56); // sizeof __wasi_filestat_t
    let ret = ___wasi_fd_filestat_get(host_buf);
    copyout_bytes(buf, host_buf, 56);
    return ret;
},

fd_filestat_set_size: function(fd, size) {
    return ___wasi_fd_filestat_set_size(fd, size);
},

fd_filestat_set_times: function(fd, st_atim, st_mtim, fstflags) {
    return ___wasi_fd_filestat_set_times(fd, st_atim, st_mtim, fstflags);
},

path_filestat_get: function(fd, path, path_len, buf) {
    let host_path = copyin_bytes(path, path_len);
    let host_buf = _malloc(56); // sizeof __wasi_filestat_t
    let ret = ___wasi_path_filestat_get(fd, host_path, path_len, host_buf);
    copyout_bytes(buf, host_buf, 56);
    _free(host_path);
    return ret;
},

path_filestat_set_times: function(fd, path, path_len, st_atim, st_mtim, flags) {
    let host_path = copyin_bytes(path, path_len);
    let ret = ___wasi_path_filestat_set_times(fd, host_path, st_atim, st_mtim, fstflags);
    _free(host_path);
    return ret;
},

path_symlink: function(path0, path_len0, fd, path1, path_len1) {
    let host_path0 = copyin_bytes(path0, path0_len);
    let host_path1 = copyin_bytes(path1, path1_len);
    let ret = ___wasi_path_symlink(host_path0, path_len0, fd, host_path1, path_len1);
    _free(host_path1);
    _free(host_path0);
    return ret;
},

path_unlink_file: function(fd, path, path_len, flags) {
    let host_path = copyin_bytes(path, path_len);
    let ret = ___wasi_path_unlink_file(fd, host_path, path_len, flags);
    _free(host_path);
    return ret;
},

path_remove_directory: function(fd, path, path_len, flags) {
    let host_path = copyin_bytes(path, path_len);
    let ret = ___wasi_path_remove_directory(fd, host_path, path_len, flags);
    _free(host_path);
    return ret;
},

poll_oneoff: function(in_, out, nsubscriptions, nevents) {
    let host_in = copyin_bytes(in_, nsubscriptions * 56); // sizeof __wasi_subscription_t
    let host_out = _malloc(nsubscriptions * 32); // sizeof __wasi_event_t
    let host_nevents = _malloc(4);
    let ret = ___wasi_poll_oneoff(host_in, host_out, host_nevents);
    copyout_bytes(out, host_out, nsubscriptions * 32);
    copyout_i32(nevents, host_nevents);
    _free(host_in);
    return ret;
},

proc_exit: function(rval) {
    let message;
    if (rval == 0) {
        message = "success";
    } else {
        message = "error code " + rval;
    }
    throw new WASIExit(rval, message);
},

proc_raise: function(sig) {
    if (sig == 18 || // SIGSTOP
        sig == 19 || // SIGTSTP
        sig == 20 || // SIGTTIN
        sig == 21 || // SIGTTOU
        sig == 22 || // SIGURG
        sig == 16 || // SIGCHLD
        sig == 13)   // SIGPIPE
    {
      return 0;
    }

    let message = "raised signal " + sig;
    throw new WASIExit(128 + sig, message);
},

random_get: function(buf, buf_len) {
    let host_buf = _malloc(buf_len);
    let ret = __wasi_random_get(host_buf, buf_len);
    copyout_bytes(buf, host_buf, buf_len);
    return ret;
},

sched_yield: function() {
    return __wasi_sched_yield();
},

fd_shutdown: function(sock, how) {
    return __wasi_fd_shutdown(sock, how);
}

};
