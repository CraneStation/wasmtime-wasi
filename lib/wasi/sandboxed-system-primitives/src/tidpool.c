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

#include <stdatomic.h>

#include "tidpool.h"

// Start counting at two, as zero and one are reserved by the futex code
// (LOCK_MANAGED, LOCK_OWNER_UNKNOWN).
static _Atomic(__wasi_tid_t) tidpool = 2;

__wasi_tid_t tidpool_allocate(void) {
  // TODO(ed): Deal with overflows. But then again, thread identifiers
  // are 30 bits. Who ever creates more than one billion threads during
  // the lifetime of a single process? Some people do, I guess.
  return atomic_fetch_add_explicit(&tidpool, 1, memory_order_relaxed);
}
