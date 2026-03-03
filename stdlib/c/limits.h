/*
 * ADead-BIB Standard Library
 * limits.h - Implementation Limits
 * 
 * Based on: C99/C11, x86-64 ABI
 */

#ifndef _ADEAD_LIMITS_H
#define _ADEAD_LIMITS_H

/* Char limits */
#define CHAR_BIT    8
#define SCHAR_MIN   (-128)
#define SCHAR_MAX   127
#define UCHAR_MAX   255
#define CHAR_MIN    SCHAR_MIN
#define CHAR_MAX    SCHAR_MAX

/* Short limits */
#define SHRT_MIN    (-32768)
#define SHRT_MAX    32767
#define USHRT_MAX   65535

/* Int limits */
#define INT_MIN     (-2147483647 - 1)
#define INT_MAX     2147483647
#define UINT_MAX    4294967295U

/* Long limits (64-bit on x86-64) */
#define LONG_MIN    (-9223372036854775807L - 1)
#define LONG_MAX    9223372036854775807L
#define ULONG_MAX   18446744073709551615UL

/* Long long limits */
#define LLONG_MIN   (-9223372036854775807LL - 1)
#define LLONG_MAX   9223372036854775807LL
#define ULLONG_MAX  18446744073709551615ULL

/* Other limits */
#define MB_LEN_MAX  4

/* POSIX limits */
#define PATH_MAX    4096
#define NAME_MAX    255
#define PIPE_BUF    4096
#define ARG_MAX     131072
#define OPEN_MAX    1024
#define LINE_MAX    2048

/* Size limits */
#define SSIZE_MAX   LONG_MAX
#define SIZE_MAX    ULONG_MAX

#endif /* _ADEAD_LIMITS_H */
