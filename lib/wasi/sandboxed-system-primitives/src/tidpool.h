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

#ifndef TIDPOOL_H
#define TIDPOOL_H

#include <wasmtime_ssp.h>

// Allocates a new thread identifier.
__wasi_tid_t tidpool_allocate(void);

#endif
