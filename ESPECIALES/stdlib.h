// ============================================================
// ADead-BIB — ESPECIALES/stdlib.h
// Standard Library — ADead-BIB implementation layer
// Compatible with C89/C99/C11/C++11/C++17
// ============================================================

#ifndef _ADEAD_STDLIB_H
#define _ADEAD_STDLIB_H

typedef unsigned long long size_t;
typedef long long          ptrdiff_t;
typedef long long          intmax_t;
typedef unsigned long long uintmax_t;

#define NULL        0
#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1
#define RAND_MAX    32767

// ── Memory management ────────────────────────────────────────
extern void*  malloc(size_t size);
extern void*  calloc(size_t count, size_t size);
extern void*  realloc(void* ptr, size_t size);
extern void   free(void* ptr);
extern void*  aligned_alloc(size_t alignment, size_t size);

// ── Process control ──────────────────────────────────────────
extern void   exit(int status);
extern void   abort();
extern void   _exit(int status);
extern int    atexit(void (*func)());
extern int    at_quick_exit(void (*func)());
extern void   quick_exit(int status);

// ── Arithmetic ───────────────────────────────────────────────
extern int    abs(int x);
extern long   labs(long x);
extern long long llabs(long long x);
extern int    div_impl(int n, int d);     // div() returns struct, simplified
extern long   ldiv_impl(long n, long d);

// ── Random numbers ───────────────────────────────────────────
extern int    rand();
extern void   srand(unsigned int seed);
extern int    rand_r(unsigned int* seed);

// ── String/number conversions ────────────────────────────────
extern int      atoi(const char* s);
extern long     atol(const char* s);
extern long long atoll(const char* s);
extern double   atof(const char* s);
extern long     strtol(const char* s, char** endptr, int base);
extern unsigned long strtoul(const char* s, char** endptr, int base);
extern long long strtoll(const char* s, char** endptr, int base);
extern unsigned long long strtoull(const char* s, char** endptr, int base);
extern double   strtod(const char* s, char** endptr);
extern float    strtof(const char* s, char** endptr);

// ── Environment ──────────────────────────────────────────────
extern char*  getenv(const char* name);
extern int    putenv(char* string);
extern int    setenv(const char* name, const char* value, int overwrite);
extern int    unsetenv(const char* name);
extern int    system(const char* command);

// ── Sorting and searching ─────────────────────────────────────
extern void   qsort(void* base, size_t n, size_t size, int (*compar)(const void*, const void*));
extern void*  bsearch(const void* key, const void* base, size_t n, size_t size, int (*compar)(const void*, const void*));

// ── Multibyte / wide chars (stubs) ────────────────────────────
extern int    mblen(const char* s, size_t n);
extern size_t mbstowcs(wchar_t* dest, const char* src, size_t n);
extern size_t wcstombs(char* dest, const wchar_t* src, size_t n);
extern int    mbtowc(wchar_t* pwc, const char* s, size_t n);
extern int    wctomb(char* s, wchar_t wchar);

// ── Min/max helpers (not in standard C but common) ───────────
static inline int   min_i(int a, int b)   { return a < b ? a : b; }
static inline int   max_i(int a, int b)   { return a > b ? a : b; }
static inline long  min_l(long a, long b) { return a < b ? a : b; }
static inline long  max_l(long a, long b) { return a > b ? a : b; }

// ── Swap utility (not in C stdlib but useful) ─────────────────
#define SWAP(a, b, T) do { T _tmp = (a); (a) = (b); (b) = _tmp; } while(0)

#endif // _ADEAD_STDLIB_H
