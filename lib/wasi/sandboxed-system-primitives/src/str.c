// Copyright (c) 2016 Nuxi, https://nuxi.nl/
//
// SPDX-License-Identifier: BSD-2-Clause

#include "config.h"

#include <errno.h>
#include <stdlib.h>
#include <string.h>

#include "str.h"

#ifdef WASMTIME_UNMODIFIED
#if !CONFIG_HAS_STRLCPY
size_t strlcpy(char *s1, const char *s2, size_t n) {
  // Copy up to n - 1 characters into the destination buffer.
  const char *begin = s2;
  while (n > 1) {
    *s1 = *s2;
    if (*s2 == '\0')
      return s2 - begin;
    ++s1;
    ++s2;
    --n;
  }

  // Nul-terminate the destination buffer if space is available.
  if (n > 0)
    *s1 = '\0';

  // Continue computing the length of s2.
  while (*s2 != '\0')
    ++s2;
  return s2 - begin;
}
#endif
#endif

char *str_nullterminate(const char *s, size_t len) {
  // Copy string.
  char *ret = strndup(s, len);
  if (ret == NULL)
    return NULL;

  // Ensure that it contains no null bytes within.
  if (strlen(ret) != len) {
    free(ret);
    errno = EILSEQ;
    return NULL;
  }
  return ret;
}
