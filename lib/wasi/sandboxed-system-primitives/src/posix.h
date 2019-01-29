// Copyright (c) 2016-2018 Nuxi, https://nuxi.nl/
//
// SPDX-License-Identifier: BSD-2-Clause

#ifndef POSIX_H
#define POSIX_H

#include <stdbool.h>
#include <stddef.h>

#include "locking.h"

struct fd_entry;
struct syscalls;

struct fd_table {
  struct rwlock lock;
  struct fd_entry *entries;
  size_t size;
  size_t used;
};

#ifdef WASMTIME_UNMODIFIED
// Only provide this in C mode, as _Thread_local may not be provided by
// the compiler in C++ mode.
#ifndef __cplusplus
extern _Thread_local wasi_tid_t curtid;
#endif

extern struct syscalls posix_syscalls;
#endif

void fd_table_init(struct fd_table *);
bool fd_table_insert_existing(struct fd_table *, wasi_fd_t, int);

#endif
