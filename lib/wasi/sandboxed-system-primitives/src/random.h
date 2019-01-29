// Copyright (c) 2016 Nuxi, https://nuxi.nl/
//
// SPDX-License-Identifier: BSD-2-Clause

#ifndef RANDOM_H
#define RANDOM_H

#include <stdint.h>

void random_buf(void *, size_t);
uintmax_t random_uniform(uintmax_t);

#endif
