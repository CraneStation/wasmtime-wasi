// Copyright (c) 2016 Nuxi, https://nuxi.nl/
//
// SPDX-License-Identifier: BSD-2-Clause

#ifndef FUTEX_H
#define FUTEX_H

#include <stdbool.h>
#include <stddef.h>

#include <wasi_types.h>

wasi_errno_t futex_op_condvar_signal(_Atomic(wasi_condvar_t) *,
                                         wasi_scope_t, wasi_nthreads_t);
wasi_errno_t futex_op_lock_unlock(wasi_tid_t,
                                      _Atomic(wasi_lock_t) *,
                                      wasi_scope_t);
bool futex_op_poll(wasi_tid_t, const wasi_subscription_t *,
                   wasi_event_t *, size_t, size_t *);

#endif
