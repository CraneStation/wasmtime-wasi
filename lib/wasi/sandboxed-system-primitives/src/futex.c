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

#include <assert.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>

#include <wasmtime_ssp.h>

#include "futex.h"
#include "locking.h"
#include "queue.h"

// A set of waiting threads.
struct futex_queue {
  TAILQ_HEAD(, futex_waiter) fq_list;
  unsigned int fq_count;
};

// Condition variables.
struct futex_condvar {
  // Address of the condition variable.
  _Atomic(__wasi_condvar_t) * fc_address;
  // The lock the waiters should be moved to when signalled.
  struct futex_lock *fc_lock;
  // Threads waiting on the condition variable.
  struct futex_queue fc_waiters;
  // Number of threads blocked on this condition variable, or being
  // blocked on the lock after being requeued.
  unsigned int fc_waitcount;
  // Global list pointers.
  LIST_ENTRY(futex_condvar) fc_next;
};

// Read-write locks.
struct futex_lock {
  // Address of the lock.
  _Atomic(__wasi_lock_t) * fl_address;
  // Current owner of the lock. LOCK_UNMANAGED if the lock is currently
  // not owned by the kernel. LOCK_OWNER_UNKNOWN in case the owner is
  // not known (e.g., when the lock is read-locked).
  __wasi_tid_t fl_owner;
#define LOCK_UNMANAGED 0x0
#define LOCK_OWNER_UNKNOWN 0x1
  // Writers blocked on the lock.
  struct futex_queue fl_writers;
  // Readers blocked on the lock.
  struct futex_queue fl_readers;
  // Number of threads blocked on this lock + condition variables.
  unsigned int fl_waitcount;
  // Global list pointers.
  LIST_ENTRY(futex_lock) fl_next;
};

struct futex_waiter {
  // Thread ID.
  __wasi_tid_t fw_tid;
  // Condition variable used for waiting.
  struct cond fw_wait;
  // Queue this waiter is currently placed in.
  struct futex_queue *fw_queue;
  // List pointers of fw_queue.
  TAILQ_ENTRY(futex_waiter) fw_next;
};

// Global data structures.
static struct mutex futex_global_lock = MUTEX_INITIALIZER;
#define REQUIRES_FUTEX_LOCK REQUIRES_EXCLUSIVE(futex_global_lock)

static LIST_HEAD(, futex_lock)
    futex_lock_list = LIST_HEAD_INITIALIZER(&futex_lock_list);
static LIST_HEAD(, futex_condvar)
    futex_condvar_list = LIST_HEAD_INITIALIZER(&futex_condvar_list);

// Utility functions.
static void futex_lock_assert(const struct futex_lock *) REQUIRES_FUTEX_LOCK;
static struct futex_lock *futex_lock_lookup_locked(_Atomic(__wasi_lock_t) *)
    REQUIRES_FUTEX_LOCK;
static void futex_lock_release(struct futex_lock *fl)
    UNLOCKS(futex_global_lock);
static __wasi_errno_t futex_lock_tryrdlock(struct futex_lock *)
    REQUIRES_FUTEX_LOCK;
static void futex_lock_unmanage(struct futex_lock *) REQUIRES_FUTEX_LOCK;
static void futex_lock_update_owner(struct futex_lock *) REQUIRES_FUTEX_LOCK;
static void futex_lock_wake_up_next(struct futex_lock *) REQUIRES_FUTEX_LOCK;
static unsigned int futex_queue_count(const struct futex_queue *)
    REQUIRES_FUTEX_LOCK;
static void futex_queue_init(struct futex_queue *) REQUIRES_FUTEX_LOCK;
static void futex_queue_requeue(struct futex_queue *, struct futex_queue *,
                                unsigned int) REQUIRES_FUTEX_LOCK;
static __wasi_errno_t futex_queue_sleep(struct futex_queue *,
                                          struct futex_lock *, __wasi_tid_t,
                                          __wasi_clockid_t,
                                          __wasi_timestamp_t,
                                          bool) REQUIRES_FUTEX_LOCK;
static __wasi_tid_t futex_queue_tid_best(const struct futex_queue *)
    REQUIRES_FUTEX_LOCK;
static void futex_queue_wake_up_all(struct futex_queue *) REQUIRES_FUTEX_LOCK;
static void futex_queue_wake_up_best(struct futex_queue *) REQUIRES_FUTEX_LOCK;

// futex_condvar operations.

static void futex_condvar_assert(const struct futex_condvar *fc)
    REQUIRES_FUTEX_LOCK {
  assert(fc->fc_waitcount >= futex_queue_count(&fc->fc_waiters) &&
         "Total number of waiters cannot be smaller than the wait queue");
  futex_lock_assert(fc->fc_lock);
}

static bool futex_condvar_lookup(const _Atomic(__wasi_condvar_t) * address,
                                 struct futex_condvar **fcret)
    TRYLOCKS_EXCLUSIVE(true, futex_global_lock) {
  mutex_lock(&futex_global_lock);
  struct futex_condvar *fc;
  LIST_FOREACH(fc, &futex_condvar_list, fc_next) {
    if (fc->fc_address == address) {
      futex_condvar_assert(fc);
      *fcret = fc;
      return true;
    }
  }
  mutex_unlock(&futex_global_lock);
  return false;
}

static __wasi_errno_t futex_condvar_lookup_or_create(
    _Atomic(__wasi_condvar_t) * condvar, _Atomic(__wasi_lock_t) * lock,
    struct futex_condvar **fcret) TRYLOCKS_EXCLUSIVE(0, futex_global_lock) {
  mutex_lock(&futex_global_lock);
  struct futex_condvar *fc;
  LIST_FOREACH(fc, &futex_condvar_list, fc_next) {
    if (fc->fc_address != condvar)
      continue;
    struct futex_lock *fl = fc->fc_lock;
    if (fl->fl_address != lock) {
      // Condition variable is owned by a different lock.
      mutex_unlock(&futex_global_lock);
      return __WASI_EINVAL;
    }

    // Found fully matching condition variable.
    futex_condvar_assert(fc);
    *fcret = fc;
    return 0;
  }

  // None found. Create new condition variable object.
  fc = malloc(sizeof(*fc));
  if (fc == NULL) {
    mutex_unlock(&futex_global_lock);
    return __WASI_ENOMEM;
  }
  fc->fc_address = condvar;
  fc->fc_lock = futex_lock_lookup_locked(lock);
  if (fc->fc_lock == NULL) {
    free(fc);
    mutex_unlock(&futex_global_lock);
    return __WASI_ENOMEM;
  }
  futex_queue_init(&fc->fc_waiters);
  fc->fc_waitcount = 0;
  LIST_INSERT_HEAD(&futex_condvar_list, fc, fc_next);
  *fcret = fc;
  return 0;
}

static void futex_condvar_release(struct futex_condvar *fc)
    UNLOCKS(futex_global_lock) {
  futex_condvar_assert(fc);
  struct futex_lock *fl = fc->fc_lock;
  if (fc->fc_waitcount == 0) {
    // Condition variable has no waiters. Deallocate it.
    LIST_REMOVE(fc, fc_next);
    free(fc);
  }
  futex_lock_release(fl);
}

static void futex_condvar_unmanage(struct futex_condvar *fc)
    REQUIRES_FUTEX_LOCK {
  if (futex_queue_count(&fc->fc_waiters) == 0)
    atomic_store(fc->fc_address, __WASI_CONDVAR_HAS_NO_WAITERS);
}

// futex_lock operations.

static void futex_lock_assert(const struct futex_lock *fl) {
  // A futex lock can only be kernel-managed if it has waiters. Vice
  // versa: if a futex lock has waiters, it must be kernel-managed.
  assert((fl->fl_owner == LOCK_UNMANAGED) ==
             (futex_queue_count(&fl->fl_readers) == 0 &&
              futex_queue_count(&fl->fl_writers) == 0) &&
         "Managed locks must have waiting threads");
  assert((fl->fl_waitcount != 0 || fl->fl_owner == LOCK_UNMANAGED) &&
         "Lock with no waiters must be unmanaged");
}

static bool futex_lock_lookup(_Atomic(__wasi_lock_t) * lock,
                              struct futex_lock **fl)
    TRYLOCKS_EXCLUSIVE(true, futex_global_lock) {
  mutex_lock(&futex_global_lock);
  *fl = futex_lock_lookup_locked(lock);
  if (*fl == NULL) {
    mutex_unlock(&futex_global_lock);
    return false;
  }
  return true;
}

static struct futex_lock *futex_lock_lookup_locked(_Atomic(__wasi_lock_t) *
                                                   lock) {
  struct futex_lock *fl;
  LIST_FOREACH(fl, &futex_lock_list, fl_next) {
    if (fl->fl_address == lock) {
      // Found matching lock object.
      futex_lock_assert(fl);
      return fl;
    }
  }

  // None found. Create new lock object.
  fl = malloc(sizeof(*fl));
  if (fl == NULL)
    return NULL;
  fl->fl_address = lock;
  fl->fl_owner = LOCK_UNMANAGED;
  futex_queue_init(&fl->fl_readers);
  futex_queue_init(&fl->fl_writers);
  fl->fl_waitcount = 0;
  LIST_INSERT_HEAD(&futex_lock_list, fl, fl_next);
  return fl;
}

static __wasi_errno_t futex_lock_rdlock(struct futex_lock *fl,
                                          __wasi_tid_t tid,
                                          __wasi_clockid_t clock_id,
                                          __wasi_timestamp_t timeout,
                                          bool abstime) REQUIRES_FUTEX_LOCK {
  __wasi_errno_t error = futex_lock_tryrdlock(fl);
  if (error == __WASI_EBUSY) {
    // Suspend execution.
    assert(fl->fl_owner != LOCK_UNMANAGED &&
           "Attempted to sleep on an unmanaged lock");
    error =
        futex_queue_sleep(&fl->fl_readers, fl, tid, clock_id, timeout, abstime);
  }
  if (error != 0)
    futex_lock_unmanage(fl);
  return error;
}

static void futex_lock_unmanage(struct futex_lock *fl) {
  if (futex_queue_count(&fl->fl_readers) == 0 &&
      futex_queue_count(&fl->fl_writers) == 0) {
    // Lock should be unmanaged.
    fl->fl_owner = LOCK_UNMANAGED;

    // Clear kernel-managed bit.
    __wasi_lock_t old = atomic_load(fl->fl_address);
    while (!atomic_compare_exchange_weak(fl->fl_address, &old,
                                         old & ~__WASI_LOCK_KERNEL_MANAGED))
      ;
  }
}

static void futex_lock_release(struct futex_lock *fl) {
  futex_lock_assert(fl);
  if (fl->fl_waitcount == 0) {
    // Lock object is unreferenced. Deallocate it.
    assert(fl->fl_owner == LOCK_UNMANAGED &&
           "Attempted to free a managed lock");
    LIST_REMOVE(fl, fl_next);
    free(fl);
  }
  mutex_unlock(&futex_global_lock);
}

static void futex_lock_set_owner(struct futex_lock *fl, __wasi_lock_t lock) {
  // Lock has no explicit owner.
  if ((lock & ~__WASI_LOCK_WRLOCKED) == 0) {
    fl->fl_owner = LOCK_OWNER_UNKNOWN;
    return;
  }
  lock &= ~(__WASI_LOCK_WRLOCKED | __WASI_LOCK_KERNEL_MANAGED);

  // Don't allow userspace to silently unlock.
  if (lock == LOCK_UNMANAGED) {
    fl->fl_owner = LOCK_OWNER_UNKNOWN;
    return;
  }
  fl->fl_owner = lock;
}

static __wasi_errno_t futex_lock_unlock(
    struct futex_lock *fl, __wasi_tid_t tid) REQUIRES_FUTEX_LOCK {
  // Validate that this thread is allowed to unlock.
  futex_lock_update_owner(fl);
  if (fl->fl_owner != LOCK_UNMANAGED && fl->fl_owner != tid)
    return __WASI_EPERM;
  futex_lock_wake_up_next(fl);
  return 0;
}

static void futex_lock_update_owner(struct futex_lock *fl) {
  if (fl->fl_owner == LOCK_OWNER_UNKNOWN)
    futex_lock_set_owner(fl, atomic_load(fl->fl_address));
}

static __wasi_errno_t futex_lock_tryrdlock(struct futex_lock *fl) {
  if (fl->fl_owner != LOCK_UNMANAGED) {
    // Lock is already acquired.
    return __WASI_EBUSY;
  }

  __wasi_lock_t old = __WASI_LOCK_UNLOCKED;
  for (;;) {
    if ((old & __WASI_LOCK_KERNEL_MANAGED) != 0) {
      // Userspace lock is kernel-managed, even though we don't have an
      // entry for it.
      return __WASI_EINVAL;
    }

    if ((old & __WASI_LOCK_WRLOCKED) == 0) {
      if (atomic_compare_exchange_weak(fl->fl_address, &old, old + 1))
        return 0;
    } else {
      if (atomic_compare_exchange_weak(fl->fl_address, &old,
                                       old | __WASI_LOCK_KERNEL_MANAGED)) {
        futex_lock_set_owner(fl, old);
        return __WASI_EBUSY;
      }
    }
  }
}

static __wasi_errno_t futex_lock_trywrlock(
    struct futex_lock *fl, __wasi_tid_t tid,
    bool force_kernel_managed) REQUIRES_FUTEX_LOCK {
  if (fl->fl_owner == tid) {
    // Attempted to acquire lock recursively.
    return __WASI_EDEADLK;
  }
  if (fl->fl_owner != LOCK_UNMANAGED) {
    // Lock is already acquired.
    return __WASI_EBUSY;
  }

  __wasi_lock_t old = __WASI_LOCK_UNLOCKED;
  for (;;) {
    if ((old & __WASI_LOCK_KERNEL_MANAGED) != 0) {
      // Userspace lock is kernel-managed, even though we don't have an
      // entry for it.
      return __WASI_EINVAL;
    }
    if (old == (tid | __WASI_LOCK_WRLOCKED)) {
      // Attempted to acquire lock recursively.
      return __WASI_EDEADLK;
    }

    if (old == __WASI_LOCK_UNLOCKED) {
      __wasi_lock_t new = tid | __WASI_LOCK_WRLOCKED;
      if (force_kernel_managed)
        new |= __WASI_LOCK_KERNEL_MANAGED;
      if (atomic_compare_exchange_weak(fl->fl_address, &old, new)) {
        if (force_kernel_managed)
          fl->fl_owner = tid;
        return 0;
      }
    } else {
      if (atomic_compare_exchange_weak(fl->fl_address, &old,
                                       old | __WASI_LOCK_KERNEL_MANAGED)) {
        futex_lock_set_owner(fl, old);
        return __WASI_EBUSY;
      }
    }
  }
}

static void futex_lock_wake_up_next(struct futex_lock *fl) {
  // Determine which thread(s) to wake up. Prefer waking up
  // writers over readers to prevent write starvation.
  if (futex_queue_count(&fl->fl_writers) > 0) {
    // Transfer ownership to a single write-locker.
    if (futex_queue_count(&fl->fl_writers) > 1 ||
        futex_queue_count(&fl->fl_readers) > 0) {
      // Lock should remain managed afterwards.
      __wasi_tid_t tid = futex_queue_tid_best(&fl->fl_writers);
      atomic_store(fl->fl_address,
                   tid | __WASI_LOCK_WRLOCKED | __WASI_LOCK_KERNEL_MANAGED);

      futex_queue_wake_up_best(&fl->fl_writers);
      fl->fl_owner = tid;
    } else {
      // Lock can become unmanaged afterwards.
      atomic_store(fl->fl_address, futex_queue_tid_best(&fl->fl_writers) |
                                       __WASI_LOCK_WRLOCKED);

      futex_queue_wake_up_best(&fl->fl_writers);
      fl->fl_owner = LOCK_UNMANAGED;
    }
  } else {
    // Transfer ownership to all read-lockers (if any).
    atomic_store(fl->fl_address, futex_queue_count(&fl->fl_readers));

    // Wake up all threads.
    futex_queue_wake_up_all(&fl->fl_readers);
    fl->fl_owner = LOCK_UNMANAGED;
  }
}

static __wasi_errno_t futex_lock_wrlock(struct futex_lock *fl,
                                          __wasi_tid_t tid,
                                          __wasi_clockid_t clock_id,
                                          __wasi_timestamp_t timeout,
                                          bool abstime) REQUIRES_FUTEX_LOCK {
  __wasi_errno_t error = futex_lock_trywrlock(fl, tid, false);
  if (error == __WASI_EBUSY) {
    assert(fl->fl_owner != LOCK_UNMANAGED &&
           "Attempted to sleep on an unmanaged lock");
    error =
        futex_queue_sleep(&fl->fl_writers, fl, tid, clock_id, timeout, abstime);
  }
  if (error != 0)
    futex_lock_unmanage(fl);
  return error;
}

// futex_queue operations.

static __wasi_tid_t futex_queue_tid_best(const struct futex_queue *fq) {
  return TAILQ_FIRST(&fq->fq_list)->fw_tid;
}

static unsigned int futex_queue_count(const struct futex_queue *fq) {
  return fq->fq_count;
}

static void futex_queue_init(struct futex_queue *fq) {
  TAILQ_INIT(&fq->fq_list);
  fq->fq_count = 0;
}

static __wasi_errno_t futex_queue_sleep(
    struct futex_queue *fq, struct futex_lock *fl, __wasi_tid_t tid,
    __wasi_clockid_t clock_id, __wasi_timestamp_t timeout, bool abstime) {
  // Initialize futex_waiter object.
  struct futex_waiter fw = {
      .fw_tid = tid,
      .fw_queue = fq,
  };
  switch (clock_id) {
    case __WASI_CLOCK_MONOTONIC:
      cond_init_monotonic(&fw.fw_wait);
      break;
    case __WASI_CLOCK_REALTIME:
      cond_init_realtime(&fw.fw_wait);
      break;
    default:
      return __WASI_ENOTSUP;
  }

  // Place object in the queue.
  TAILQ_INSERT_TAIL(&fq->fq_list, &fw, fw_next);
  ++fq->fq_count;

  ++fl->fl_waitcount;
  futex_lock_assert(fl);
  bool timedout;
  do {
    timedout =
        cond_timedwait(&fw.fw_wait, &futex_global_lock, timeout, abstime);
  } while (!timedout && fw.fw_queue == fq);
  if (fw.fw_queue != fq) {
    while (fw.fw_queue != NULL)
      cond_wait(&fw.fw_wait, &futex_global_lock);
  }
  futex_lock_assert(fl);
  --fl->fl_waitcount;
  cond_destroy(&fw.fw_wait);

  fq = fw.fw_queue;
  if (fq == NULL) {
    // Thread got dequeued, so we've slept successfully.
    return 0;
  } else {
    // Thread is still enqueued. Remove it.
    TAILQ_REMOVE(&fq->fq_list, &fw, fw_next);
    --fq->fq_count;
    return __WASI_ETIMEDOUT;
  }
}

// Moves up to nwaiters waiters from one queue to another.
static void futex_queue_requeue(struct futex_queue *fqfrom,
                                struct futex_queue *fqto,
                                unsigned int nwaiters) {
  // Move waiters to the target queue.
  while (nwaiters-- > 0 && !TAILQ_EMPTY(&fqfrom->fq_list)) {
    struct futex_waiter *fw = TAILQ_FIRST(&fqfrom->fq_list);
    TAILQ_REMOVE(&fqfrom->fq_list, fw, fw_next);
    --fqfrom->fq_count;

    fw->fw_queue = fqto;
    TAILQ_INSERT_TAIL(&fqto->fq_list, fw, fw_next);
    ++fqto->fq_count;
  }
}

// Wakes up all waiters in a queue.
static void futex_queue_wake_up_all(struct futex_queue *fq) {
  struct futex_waiter *fw;
  TAILQ_FOREACH(fw, &fq->fq_list, fw_next) {
    fw->fw_queue = NULL;
    cond_signal(&fw->fw_wait);
  }

  TAILQ_INIT(&fq->fq_list);
  fq->fq_count = 0;
}

// Wakes up the best waiter (i.e., the waiter having the highest
// priority) in a queue.
static void futex_queue_wake_up_best(struct futex_queue *fq) {
  struct futex_waiter *fw;

  fw = TAILQ_FIRST(&fq->fq_list);
  fw->fw_queue = NULL;
  cond_signal(&fw->fw_wait);

  TAILQ_REMOVE(&fq->fq_list, fw, fw_next);
  --fq->fq_count;
}

static __wasi_errno_t futex_op_condvar_wait(
    __wasi_tid_t tid, _Atomic(__wasi_condvar_t) * condvar,
    __wasi_scope_t condvar_scope, _Atomic(__wasi_lock_t) * lock,
    __wasi_scope_t lock_scope, __wasi_clockid_t clock_id,
    __wasi_timestamp_t timeout, bool abstime) {
  if (lock_scope != __WASI_SCOPE_PRIVATE ||
      condvar_scope != __WASI_SCOPE_PRIVATE)
    return __WASI_ENOTSUP;

  // Lookup condition variable object.
  struct futex_condvar *fc;
  __wasi_errno_t error = futex_condvar_lookup_or_create(condvar, lock, &fc);
  if (error != 0)
    return error;
  struct futex_lock *fl = fc->fc_lock;

  // Set the condition variable to something other than
  // __WASI_CONDVAR_HAS_NO_WAITERS to make userspace threads
  // call into the kernel to perform wakeups.
  atomic_store(condvar, ~__WASI_CONDVAR_HAS_NO_WAITERS);

  // Drop the lock.
  error = futex_lock_unlock(fl, tid);
  if (error != 0) {
    futex_condvar_unmanage(fc);
    futex_condvar_release(fc);
    return error;
  }

  // Go to sleep.
  ++fc->fc_waitcount;
  error = futex_queue_sleep(&fc->fc_waiters, fc->fc_lock, tid, clock_id,
                            timeout, abstime);
  if (error != 0) {
    // We observed a timeout. Reacquire the lock.
    futex_condvar_unmanage(fc);
    __wasi_errno_t error2 =
        futex_lock_wrlock(fl, tid, __WASI_CLOCK_REALTIME, UINT64_MAX, true);
    if (error2 != 0)
      error = error2;
  }
  --fc->fc_waitcount;
  futex_condvar_release(fc);
  return error;
}

__wasi_errno_t futex_op_condvar_signal(_Atomic(__wasi_condvar_t) * condvar,
                                         __wasi_scope_t scope,
                                         __wasi_nthreads_t nwaiters) {
  if (scope != __WASI_SCOPE_PRIVATE)
    return __WASI_ENOTSUP;

  if (nwaiters == 0) {
    // No threads to wake up.
    return 0;
  }

  struct futex_condvar *fc;
  if (!futex_condvar_lookup(condvar, &fc))
    return __WASI_ENOENT;
  struct futex_lock *fl = fc->fc_lock;

  // Attempt to acquire the lock on behalf of the first waiting thread.
  // Already set the kernel managed flag on the lock if there are
  // additional threads that we are going to wake up.
  struct futex_queue *fq = &fc->fc_waiters;
  __wasi_errno_t error = futex_lock_trywrlock(
      fl, futex_queue_tid_best(fq), nwaiters > 1 && futex_queue_count(fq) > 1);
  if (error == 0) {
    // Wake up the first thread and requeue the other threads to the
    // lock, so they can be woken up when the first thread unlocks.
    futex_queue_wake_up_best(fq);
    futex_queue_requeue(fq, &fl->fl_writers, nwaiters - 1);
  } else if (error == __WASI_EBUSY) {
    // Lock is already locked. Requeue all threads to the lock.
    assert(fl->fl_owner != LOCK_UNMANAGED &&
           "Attempted to sleep on an unmanaged lock");
    futex_queue_requeue(fq, &fl->fl_writers, nwaiters);
  } else {
    futex_condvar_release(fc);
    return error;
  }

  // Clear userspace condition variable if all waiters are gone.
  futex_condvar_unmanage(fc);
  futex_condvar_release(fc);
  return 0;
}

static __wasi_errno_t futex_op_lock_rdlock(
    __wasi_tid_t tid, _Atomic(__wasi_lock_t) * lock, __wasi_scope_t scope,
    __wasi_clockid_t clock_id, __wasi_timestamp_t timeout, bool abstime) {
  if (scope != __WASI_SCOPE_PRIVATE)
    return __WASI_ENOTSUP;

  struct futex_lock *fl;
  if (!futex_lock_lookup(lock, &fl))
    return __WASI_ENOMEM;
  __wasi_errno_t error =
      futex_lock_rdlock(fl, tid, clock_id, timeout, abstime);
  futex_lock_release(fl);
  return error;
}

static __wasi_errno_t futex_op_lock_wrlock(
    __wasi_tid_t tid, _Atomic(__wasi_lock_t) * lock, __wasi_scope_t scope,
    __wasi_clockid_t clock_id, __wasi_timestamp_t timeout, bool abstime) {
  if (scope != __WASI_SCOPE_PRIVATE)
    return __WASI_ENOTSUP;

  struct futex_lock *fl;
  if (!futex_lock_lookup(lock, &fl))
    return __WASI_ENOMEM;
  __wasi_errno_t error =
      futex_lock_wrlock(fl, tid, clock_id, timeout, abstime);
  futex_lock_release(fl);
  return error;
}

__wasi_errno_t futex_op_lock_unlock(__wasi_tid_t tid,
                                      _Atomic(__wasi_lock_t) * lock,
                                      __wasi_scope_t scope) {
  if (scope != __WASI_SCOPE_PRIVATE)
    return __WASI_ENOTSUP;

  struct futex_lock *fl;
  if (!futex_lock_lookup(lock, &fl))
    return __WASI_ENOMEM;
  __wasi_errno_t error = futex_lock_unlock(fl, tid);
  futex_lock_release(fl);
  return error;
}

bool futex_op_poll(__wasi_tid_t tid, const __wasi_subscription_t *in,
                   __wasi_event_t *out, size_t nsubscriptions,
                   size_t *nevents) {
  // Intercept all calls to poll() that want to sleep on a futex, either
  // with or without a timeout.
  if (!(nsubscriptions == 1 ||
        (nsubscriptions == 2 && in[1].type == __WASI_EVENTTYPE_CLOCK)))
    return false;

  // Simply use a very high timeout value for waits without a timeout.
  __wasi_clockid_t clock_id =
      nsubscriptions == 1 ? __WASI_CLOCK_REALTIME : in[1].clock.clock_id;
  __wasi_timestamp_t timeout =
      nsubscriptions == 1 ? UINT64_MAX : in[1].clock.timeout;
  bool abstime = nsubscriptions == 1 ||
                 (in[1].clock.flags & __WASI_SUBSCRIPTION_CLOCK_ABSTIME) != 0;

  switch (in[0].type) {
    case __WASI_EVENTTYPE_CONDVAR:
      out->error = futex_op_condvar_wait(
          tid, in[0].condvar.condvar, in[0].condvar.condvar_scope,
          in[0].condvar.lock, in[0].condvar.lock_scope, clock_id, timeout,
          abstime);
      break;
    case __WASI_EVENTTYPE_LOCK_RDLOCK:
      out->error =
          futex_op_lock_rdlock(tid, in[0].lock.lock, in[0].lock.lock_scope,
                               clock_id, timeout, abstime);
      break;
    case __WASI_EVENTTYPE_LOCK_WRLOCK:
      out->error =
          futex_op_lock_wrlock(tid, in[0].lock.lock, in[0].lock.lock_scope,
                               clock_id, timeout, abstime);
      break;
    default:
      return false;
  }

  // If the wait timed out, return the clock event instead.
  if (out->error == __WASI_ETIMEDOUT) {
    out->userdata = in[1].userdata;
    out->error = 0;
    out->type = in[1].type;
  } else {
    out->userdata = in[0].userdata;
    out->type = in[0].type;
  }
  *nevents = 1;
  return true;
}
