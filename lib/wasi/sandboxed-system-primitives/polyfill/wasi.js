// To implement `proc_exit`, we define a custom exception object
// that we can throw to unwind the stack and carry the exit value.
function WASIExit(return_value, message, fileName, lineNumber) {
    var instance = new Error(message, fileName, lineNumber);
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

function memcpy_to_guest_from_host(dst, src, len) {
    for (var i = 0; i < len; ++i) {
        GUEST_HEAP8[dst + i] = HEAP8[src + i];
    }
}

function memcpy_host_to_guest(dst, src, len) {
    for (var i = 0; i < len; ++i) {
        HEAP8[dst + i] = GUEST_HEAP8[src + i];
    }
}

function translate_ciovs(iovs, iovs_len) {
    host_iovs = _malloc(8 * iovs_len);
    for (var i = 0; i < iovs_len; ++i) {
        let ptr = GUEST_HEAP32[(iovs + i * 8 + 0) >> 2];
        let len = GUEST_HEAP32[(iovs + i * 8 + 4) >> 2];
        let buf = _malloc(len);
        memcpy_host_to_guest(buf, ptr, len);
        HEAP32[(host_iovs + i * 8 + 0)>>2] = buf;
        HEAP32[(host_iovs + i * 8 + 4)>>2] = len;
    }
    return host_iovs;
}

function free_ciovs(host_iovs, iovs_len) {
    for (var i = 0; i < iovs_len; ++i) {
        let buf = HEAP32[(host_iovs + i * 8 + 0) >> 2];
        _free(buf);
    }
    _free(host_iovs);
}

function translate_iovs(iovs, iovs_len) {
    host_iovs = _malloc(8 * iovs_len);
    for (var i = 0; i < iovs_len; ++i) {
        let len = GUEST_HEAP32[(iovs + i * 8 + 4) >> 2];
        let buf = _malloc(len);
        HEAP32[(host_iovs + i * 8 + 0)>>2] = buf;
        HEAP32[(host_iovs + i * 8 + 4)>>2] = len;
    }
    return host_iovs;
}

function free_iovs(host_iovs, iovs_len, iovs) {
    for (var i = 0; i < iovs_len; ++i) {
        let buf = HEAP32[(host_iovs + i * 8 + 0) >> 2];
        let len = HEAP32[(host_iovs + i * 8 + 4) >> 2];
        let ptr = GUEST_HEAP32[(host_iovs + i * 8 + 0) >> 2];
        memcpy_to_guest_from_host(ptr, buf, len);
        _free(buf);
    }
    _free(host_iovs);
}

var WASIPolyfill = {

clock_res_get: function(clock_id, resolution) {
    var host_resolution = _malloc(8);
    var ret = ___wasi_clock_res_get(clock_id, host_resolution);
    memcpy_to_guest_from_host(resolution, host_resolution, 8);
    _free(host_resolution);
    return ret;
},

clock_time_get: function(clock_id, precision, time) {
    var host_time = _malloc(8);
    var ret = ___wasi_clock_time_get(clock_id, precision, host_time);
    memcpy_to_guest_from_host(time, host_time, 8);
    _free(host_time);
    return ret;
},

fd_close: function(fd) {
    return ___wasi_fd_close(fd);
},

fd_datasync: function(fd) {
    return ___wasi_fd_datasync(fd);
},

fd_pread: function(fd, iovs, iovs_len, offset, nread) {
    var host_iovs = translate_iovs(iovs, iovs_len);
    var host_nread = _malloc(4);
    var ret = ___wasi_fd_pread(fd, host_iovs, iovs_len, offset, host_nread);
    memcpy_to_guest_from_host(nread, host_nread, 4);
    _free(host_nread);
    free_iovs(host_iovs, iovs_len);
    return ret;
},

fd_pwrite: function(fd, iovs, iovs_len, offset, nwritten) {
    var host_iovs = translate_ciovs(iovs, iovs_len);
    var host_nwritten = _malloc(4);
    var ret = ___wasi_fd_write(fd, host_iovs, iovs_len, offset, host_nwritten);
    memcpy_to_guest_from_host(nwritten, host_nwritten, 4);
    _free(host_nwritten);
    free_ciovs(host_iovs, iovs_len);
    return ret;
},

fd_read: function(fd, iovs, iovs_len, nread) {
    var host_iovs = translate_iovs(iovs, iovs_len);
    var host_nread = _malloc(4);
    var ret = ___wasi_fd_read(fd, host_iovs, iovs_len, host_nread);
    memcpy_to_guest_from_host(nread, host_nread, 4);
    _free(host_nread);
    free_iovs(host_iovs, iovs_len);
    return ret;
},

fd_renumber: function(from, to) {
    return ___wasi_fd_renumber(from, to);
},

fd_seek: function(fd, offset, whence, newoffset) {
    var host_newoffset = _malloc(8);
    var ret = ___wasi_fd_seek(fd, offset, whence, host_newoffset);
    memcpy_to_guest_from_host(newoffset, host_newoffset, 8);
    _free(host_newoffset);
    return ret;
},

fd_tell: function(fd, newoffset) {
    var host_newoffset = _malloc(8);
    var ret = ___wasi_fd_seek(fd, host_newoffset);
    memcpy_to_guest_from_host(newoffset, host_newoffset, 8);
    _free(host_newoffset);
    return ret;
},

fd_stat_get: function(fd, buf) {
    var host_buf = _malloc(24);
    var ret = ___wasi_fd_stat_get(fd, host_buf);
    memcpy_to_guest_from_host(buf, host_buf, 24);
    _free(host_buf);
    return ret;
},

fd_stat_set_flags: function(fd, flags) {
    return ___wasi_fd_stat_get(fd, flags);
},

fd_stat_set_rights: function(fd, fs_rights_base, fs_rights_inheriting) {
    return ___wasi_fd_stat_get(fd, fs_rights_base, fs_rights_inheriting);
},

fd_stat_put: function(fd, buf, flags) {
    var host_buf = _malloc(24);
    memcpy_to_host_from_guest(host_buf, buf, 24);
    var ret = ___wasi_fd_stat_get(fd, host_buf);
    _free(host_buf);
    return ret;
},

fd_sync: function(fd) {
    return ___wasi_fd_sync(fd);
},

fd_write: function(fd, iovs, iovs_len, nwritten) {
    var host_iovs = translate_ciovs(iovs, iovs_len);
    var host_nwritten = _malloc(4);
    var ret = ___wasi_fd_write(fd, host_iovs, iovs_len, host_nwritten);
    memcpy_to_guest_from_host(nwritten, host_nwritten, 4);
    _free(host_nwritten);
    free_ciovs(host_iovs, iovs_len);
    return ret;
},

file_advise: function(fd, offset, len, advice) {
    return ___wasi_file_advise(fd, offset, len, advice);
},

file_allocate: function(fd, offset, len) {
    return ___wasi_file_allocate(fd, offset, len);
},

file_mkdir: function(fd, path, path_len) {
    let host_path = _malloc(path_len);
    memcpy_to_host_from_guest(host_path, path, path_len);
    var ret = ___wasi_file_mkdir(fd, host_path, path_len);
    _free(host_path);
    return ret;
},

file_link: function(fd0, path0, path_len0, fd1, path1, path_len1) {
    let host_path0 = _malloc(path_len0);
    memcpy_to_host_from_guest(host_path0, path0, path_len0);
    let host_path1 = _malloc(path_len1);
    memcpy_to_host_from_guest(host_path1, path1, path_len1);
    var ret = ___wasi_file_link(fd, host_path0, path_len0, fd1, host_path1, path1_len);
    _free(host_path1);
    _free(host_path0);
    return ret;
},

file_open: function(dirfd, path, path_len, oflags, fs_rights_base, fs_rights_inheriting, fs_flags, fd) {
    let host_dirfd = _malloc(8);
    memcpy_to_host_from_guest(host_dirfd, dirfd, 8);
    let host_path = _malloc(path_len);
    memcpy_to_host_from_guest(host_path, path, path_len);
    _free(host_dirfd);
    _free(host_path);
    return 52; // ENOSYS
},

file_readdir: function(fd, buf, buf_len, cookie, buf_used) {
    console.log("unimplemented WASI syscall: file_readdir");
    return 52; // ENOSYS
},

file_readlink: function(fd, path, path_len, buf, buf_len, buf_used) {
    console.log("unimplemented WASI syscall: file_readlink");
    return 52; // ENOSYS
},

file_rename: function(fd0, path0, path_len0, fd1, path1, path_len1) {
    console.log("unimplemented WASI syscall: file_rename");
    return 52; // ENOSYS
},

file_fstat_get: function(fd, buf) {
    console.log("unimplemented WASI syscall: file_fstat_get");
    return 52; // ENOSYS
},

file_fstat_set_size: function(fd, buf, flags) {
    console.log("unimplemented WASI syscall: file_fstat_set_size");
    return 52; // ENOSYS
},

file_fstat_set_times: function(fd, buf, flags) {
    console.log("unimplemented WASI syscall: file_fstat_set_times");
    return 52; // ENOSYS
},

file_stat_get: function(fd, path, path_len, buf) {
    console.log("unimplemented WASI syscall: file_stat_get");
    return 52; // ENOSYS
},

file_stat_set_times: function(fd, path, path_len, st_atim, st_mtim, flags) {
    console.log("unimplemented WASI syscall: file_stat_set_times");
    return 52; // ENOSYS
},

file_symlink: function(path0, path_len0, fd, path1, path_len1) {
    console.log("unimplemented WASI syscall: file_symlink");
    return 52; // ENOSYS
},

file_unlink: function(fd, path, path_len, flags) {
    console.log("unimplemented WASI syscall: file_unlink");
    return 52; // ENOSYS
},

poll_oneoff: function(in_, out, nsubscriptions, nevents) {
    console.log("unimplemented WASI syscall: poll_oneoff");
    return 52; // ENOSYS
},

proc_exit: function(rval) {
    var message;
    if (rval == 0) {
        message = "success";
    } else {
        message = "error code " + rval;
    }
    throw new WASIExit(rval, message);
},

proc_raise: function(sig) {
    console.log("unimplemented WASI syscall: proc_raise");
    return 52; // ENOSYS
},

random_get: function(buf, buf_len) {
    var host_buf = _malloc(buf_len);
    var ret = __wasi_random_get(host_buf, buf_len);
    memcpy_to_guest_from_host(buf, host_buf, buf_len);
    _free(host_buf);
    return ret;
},

sched_yield: function() {
    return __wasi_sched_yield();
},

sock_recv: function(sock, ri_data, ri_data_len, ri_flags, ro_datalen, ro_flags) {
    console.log("unimplemented WASI syscall: sock_recv");
    return 52; // ENOSYS
},

sock_send: function(sock, si_data, si_data_len, si_flags, so_datalen) {
    console.log("unimplemented WASI syscall: sock_send");
    return 52; // ENOSYS
},

sock_shutdown: function(sock, how) {
    return __wasi_sock_shutdown(sock, how);
}

};
