// Copyright (c) 2016 Nuxi, https://nuxi.nl/
//
// SPDX-License-Identifier: BSD-2-Clause

#ifndef REFCOUNT_H
#define REFCOUNT_H

#include <assert.h>
#include <stdatomic.h>
#include <stdbool.h>

#include "locking.h"

// Simple reference counter.
struct LOCKABLE refcount {
  atomic_uint count;
};

#define PRODUCES(...) LOCKS_SHARED(__VA_ARGS__) NO_LOCK_ANALYSIS
#define CONSUMES(...) UNLOCKS(__VA_ARGS__) NO_LOCK_ANALYSIS

// Initialize the reference counter.
static void refcount_init(struct refcount *r, unsigned int count) PRODUCES(*r) {
  atomic_init(&r->count, count);
}

// Increment the reference counter.
static inline void refcount_acquire(struct refcount *r) PRODUCES(*r) {
  atomic_fetch_add_explicit(&r->count, 1, memory_order_acquire);
}

// Decrement the reference counter, returning whether the reference
// dropped to zero.
static inline bool refcount_release(struct refcount *r) CONSUMES(*r) {
  int old = atomic_fetch_sub_explicit(&r->count, 1, memory_order_release);
  assert(old != 0 && "Reference count becoming negative");
  return old == 1;
}

#endif
