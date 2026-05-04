/* adeb_types.h — Tipos C99 fundamentales (canon ADead-BIB) */
#ifndef ADEB_TYPES_H
#define ADEB_TYPES_H

typedef signed char        int8_t;
typedef short              int16_t;
typedef int                int32_t;
typedef long long          int64_t;

typedef unsigned char      uint8_t;
typedef unsigned short     uint16_t;
typedef unsigned int       uint32_t;
typedef unsigned long long uint64_t;

typedef uint64_t           size_t;
typedef int64_t            ptrdiff_t;
typedef int64_t            intptr_t;
typedef uint64_t           uintptr_t;

#ifndef NULL
#define NULL ((void*)0)
#endif

#define ADEB_TRUE  1
#define ADEB_FALSE 0

#endif /* ADEB_TYPES_H */
