#ifndef COMMON_H
#define COMMON_H

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

static const u8 expected_logo[0x30] = {
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83,
    0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63,
    0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
};

#define NO_IMPL                                                                \
  {                                                                            \
    fprintf(stderr, "NO IMPL: No impl code in %s at %s:%d\n", __func__,        \
            __FILE__, __LINE__);                                               \
    abort();                                                                   \
  }

#define UN_REACHABLE                                                           \
  {                                                                            \
    fprintf(stderr, "UNREACHABLE: Reached unreachable code in %s at %s:%d\n",  \
            __func__, __FILE__, __LINE__);                                     \
    abort();                                                                   \
  }
#endif