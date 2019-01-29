// Copyright (c) 2016 Nuxi, https://nuxi.nl/
//
// SPDX-License-Identifier: BSD-2-Clause

#ifndef RIGHTS_H
#define RIGHTS_H

#define RIGHTS_ALL                                                             \
  (WASI_RIGHT_FD_DATASYNC | WASI_RIGHT_FD_READ |                       \
   WASI_RIGHT_FD_SEEK | WASI_RIGHT_FD_STAT_PUT_FLAGS |                 \
   WASI_RIGHT_FD_SYNC | WASI_RIGHT_FD_TELL | WASI_RIGHT_FD_WRITE | \
   WASI_RIGHT_FILE_ADVISE | WASI_RIGHT_FILE_ALLOCATE |                 \
   WASI_RIGHT_FILE_CREATE_DIRECTORY | WASI_RIGHT_FILE_CREATE_FILE |    \
   WASI_RIGHT_FILE_LINK_SOURCE | WASI_RIGHT_FILE_LINK_TARGET |         \
   WASI_RIGHT_FILE_OPEN | WASI_RIGHT_FILE_READDIR |                    \
   WASI_RIGHT_FILE_READLINK | WASI_RIGHT_FILE_RENAME_SOURCE |          \
   WASI_RIGHT_FILE_RENAME_TARGET | WASI_RIGHT_FILE_STAT_FGET |         \
   WASI_RIGHT_FILE_STAT_FPUT_SIZE | WASI_RIGHT_FILE_STAT_FPUT_TIMES |  \
   WASI_RIGHT_FILE_STAT_GET | WASI_RIGHT_FILE_STAT_PUT_TIMES |         \
   WASI_RIGHT_FILE_SYMLINK | WASI_RIGHT_FILE_UNLINK |                  \
   WASI_RIGHT_MEM_MAP | WASI_RIGHT_MEM_MAP_EXEC |                      \
   WASI_RIGHT_POLL_FD_READWRITE | WASI_RIGHT_POLL_PROC_TERMINATE |     \
   WASI_RIGHT_PROC_EXEC | WASI_RIGHT_SOCK_SHUTDOWN)

// Block and character device interaction is outside the scope of
// CloudABI. Simply allow everything.
#define RIGHTS_BLOCK_DEVICE_BASE RIGHTS_ALL
#define RIGHTS_BLOCK_DEVICE_INHERITING RIGHTS_ALL
#define RIGHTS_CHARACTER_DEVICE_BASE RIGHTS_ALL
#define RIGHTS_CHARACTER_DEVICE_INHERITING RIGHTS_ALL

// Only allow directory operations on directories. Directories can only
// yield file descriptors to other directories and files.
#define RIGHTS_DIRECTORY_BASE                                              \
  (WASI_RIGHT_FD_STAT_PUT_FLAGS | WASI_RIGHT_FD_SYNC |             \
   WASI_RIGHT_FILE_ADVISE | WASI_RIGHT_FILE_CREATE_DIRECTORY |     \
   WASI_RIGHT_FILE_CREATE_FILE | WASI_RIGHT_FILE_LINK_SOURCE |     \
   WASI_RIGHT_FILE_LINK_TARGET | WASI_RIGHT_FILE_OPEN |            \
   WASI_RIGHT_FILE_READDIR | WASI_RIGHT_FILE_READLINK |            \
   WASI_RIGHT_FILE_RENAME_SOURCE | WASI_RIGHT_FILE_RENAME_TARGET | \
   WASI_RIGHT_FILE_STAT_FGET | WASI_RIGHT_FILE_STAT_FPUT_TIMES |   \
   WASI_RIGHT_FILE_STAT_GET | WASI_RIGHT_FILE_STAT_PUT_TIMES |     \
   WASI_RIGHT_FILE_SYMLINK | WASI_RIGHT_FILE_UNLINK |              \
   WASI_RIGHT_POLL_FD_READWRITE)
#define RIGHTS_DIRECTORY_INHERITING \
  (RIGHTS_DIRECTORY_BASE | RIGHTS_REGULAR_FILE_BASE)

// Operations that apply to regular files.
#define RIGHTS_REGULAR_FILE_BASE                                               \
  (WASI_RIGHT_FD_DATASYNC | WASI_RIGHT_FD_READ |                       \
   WASI_RIGHT_FD_SEEK | WASI_RIGHT_FD_STAT_PUT_FLAGS |                 \
   WASI_RIGHT_FD_SYNC | WASI_RIGHT_FD_TELL | WASI_RIGHT_FD_WRITE | \
   WASI_RIGHT_FILE_ADVISE | WASI_RIGHT_FILE_ALLOCATE |                 \
   WASI_RIGHT_FILE_STAT_FGET | WASI_RIGHT_FILE_STAT_FPUT_SIZE |        \
   WASI_RIGHT_FILE_STAT_FPUT_TIMES | WASI_RIGHT_MEM_MAP |              \
   WASI_RIGHT_MEM_MAP_EXEC | WASI_RIGHT_POLL_FD_READWRITE |            \
   WASI_RIGHT_PROC_EXEC)
#define RIGHTS_REGULAR_FILE_INHERITING 0

// Operations that apply to shared memory objects.
#define RIGHTS_SHARED_MEMORY_BASE                                       \
  (WASI_RIGHT_FD_READ | WASI_RIGHT_FD_WRITE |                   \
   WASI_RIGHT_FILE_STAT_FGET | WASI_RIGHT_FILE_STAT_FPUT_SIZE | \
   WASI_RIGHT_MEM_MAP | WASI_RIGHT_MEM_MAP_EXEC)
#define RIGHTS_SHARED_MEMORY_INHERITING 0

// Operations that apply to sockets and socket pairs.
#define RIGHTS_SOCKET_BASE                                     \
  (WASI_RIGHT_FD_READ | WASI_RIGHT_FD_STAT_PUT_FLAGS | \
   WASI_RIGHT_FD_WRITE | WASI_RIGHT_FILE_STAT_FGET |   \
   WASI_RIGHT_POLL_FD_READWRITE | WASI_RIGHT_SOCK_SHUTDOWN)
#define RIGHTS_SOCKET_INHERITING RIGHTS_ALL

// Operations that apply to TTYs.
#define RIGHTS_TTY_BASE                                        \
  (WASI_RIGHT_FD_READ | WASI_RIGHT_FD_STAT_PUT_FLAGS | \
   WASI_RIGHT_FD_WRITE | WASI_RIGHT_FILE_STAT_FGET |   \
   WASI_RIGHT_POLL_FD_READWRITE)
#define RIGHTS_TTY_INHERITING 0

#endif
