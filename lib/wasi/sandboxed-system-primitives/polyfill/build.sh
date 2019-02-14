#!/bin/bash
set -euo pipefail

EMCC=emcc

# TODO: Remove the clang include once Emscripten supports <stdatomic.h>

"$EMCC" ../src/*.c \
    -DWASMTIME_SSP_WASI_API \
    -DWASMTIME_SSP_STATIC_CURFDS \
    -I../include \
    -Iclang \
    polyfill.c \
    -s WARN_ON_UNDEFINED_SYMBOLS=0 \
    -s EXPORTED_FUNCTIONS="['_main', '___wasi_clock_res_get', '___wasi_clock_time_get', '___wasi_fd_close', '___wasi_fd_datasync', '___wasi_fd_dup', '___wasi_fd_pread', '___wasi_fd_pwrite', '___wasi_fd_read', '___wasi_fd_replace', '___wasi_fd_seek', '___wasi_fd_stat_get', '___wasi_fd_stat_put', '___wasi_fd_sync', '___wasi_fd_write', '___wasi_file_advise', '___wasi_file_allocate', '___wasi_file_create', '___wasi_file_link', '___wasi_file_open', '___wasi_file_readdir', '___wasi_file_readlink', '___wasi_file_rename', '___wasi_file_stat_fget', '___wasi_file_stat_fput', '___wasi_file_stat_get', '___wasi_file_stat_put', '___wasi_file_symlink', '___wasi_file_unlink', '___wasi_poll_oneoff', '___wasi_proc_exit', '___wasi_proc_raise', '___wasi_random_get', '___wasi_sched_yield', '___wasi_sock_recv', '___wasi_sock_send', '___wasi_sock_shutdown']" \
    --pre-js wasi.js \
    -o polyfill.html
