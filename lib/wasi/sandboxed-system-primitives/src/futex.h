// Part of the Wasmtime Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://github.com/CraneStation/wasmtime/blob/master/LICENSE for license information.
//
// Significant parts of this file are derived from cloudabi-utils. See
// https://github.com/CraneStation/wasmtime/blob/master/lib/wasi/sandboxed-system-primitives/src/LICENSE
// for license information.
//
// The upstream file contains the following copyright notice:
//
// Copyright (c) 2016 Nuxi, https://nuxi.nl/

#ifndef FUTEX_H
#define FUTEX_H

#include <stdbool.h>
#include <stddef.h>

#include <wasmtime_ssp.h>

__wasi_errno_t futex_op_condvar_signal(_Atomic(__wasi_condvar_t) *,
                                         __wasi_scope_t, __wasi_nthreads_t);
__wasi_errno_t futex_op_lock_unlock(__wasi_tid_t,
                                      _Atomic(__wasi_lock_t) *,
                                      __wasi_scope_t);
bool futex_op_poll(__wasi_tid_t, const __wasi_subscription_t *,
                   __wasi_event_t *, size_t, size_t *);

#endif
