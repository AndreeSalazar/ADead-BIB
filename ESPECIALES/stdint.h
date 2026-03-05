// ============================================================
// ADead-BIB — ESPECIALES/stdint.h
// Standard integer types (ISO C99/C11/C++11 compatible)
// For use with ADead-BIB native compiler
// ============================================================

#ifndef _ADEAD_STDINT_H
#define _ADEAD_STDINT_H

// ── Exact-width types ────────────────────────────────────────
typedef signed char            int8_t;
typedef unsigned char          uint8_t;
typedef signed short           int16_t;
typedef unsigned short         uint16_t;
typedef signed int             int32_t;
typedef unsigned int           uint32_t;
typedef signed long long       int64_t;
typedef unsigned long long     uint64_t;
typedef signed long long       int_fast8_t;
typedef unsigned long long     uint_fast8_t;
typedef signed long long       int_fast16_t;
typedef unsigned long long     uint_fast16_t;
typedef signed long long       int_fast32_t;
typedef unsigned long long     uint_fast32_t;
typedef signed long long       int_fast64_t;
typedef unsigned long long     uint_fast64_t;
typedef signed long long       intptr_t;
typedef unsigned long long     uintptr_t;
typedef signed long long       intmax_t;
typedef unsigned long long     uintmax_t;
typedef unsigned long long     size_t;
typedef signed long long       ptrdiff_t;

// ── Min / Max limits ──────────────────────────────────────────
#define INT8_MIN    (-128)
#define INT8_MAX    (127)
#define UINT8_MAX   (255U)
#define INT16_MIN   (-32768)
#define INT16_MAX   (32767)
#define UINT16_MAX  (65535U)
#define INT32_MIN   (-2147483648)
#define INT32_MAX   (2147483647)
#define UINT32_MAX  (4294967295U)
#define INT64_MIN   (-9223372036854775808LL)
#define INT64_MAX   (9223372036854775807LL)
#define UINT64_MAX  (18446744073709551615ULL)
#define SIZE_MAX    UINT64_MAX
#define PTRDIFF_MIN INT64_MIN
#define PTRDIFF_MAX INT64_MAX
#define INTPTR_MIN  INT64_MIN
#define INTPTR_MAX  INT64_MAX
#define UINTPTR_MAX UINT64_MAX
#define INTMAX_MIN  INT64_MIN
#define INTMAX_MAX  INT64_MAX
#define UINTMAX_MAX UINT64_MAX

// ── Literal macros ────────────────────────────────────────────
#define INT8_C(v)   (v)
#define INT16_C(v)  (v)
#define INT32_C(v)  (v)
#define INT64_C(v)  (v##LL)
#define UINT8_C(v)  (v##U)
#define UINT16_C(v) (v##U)
#define UINT32_C(v) (v##U)
#define UINT64_C(v) (v##ULL)
#define INTMAX_C(v) (v##LL)
#define UINTMAX_C(v)(v##ULL)

#endif // _ADEAD_STDINT_H
