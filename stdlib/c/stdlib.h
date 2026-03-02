/*
 * ADead-BIB Standard Library
 * stdlib.h - General utilities
 */

#ifndef _ADEAD_STDLIB_H
#define _ADEAD_STDLIB_H

#include "stddef.h"

/* Exit codes */
#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1

/* Random number limits */
#define RAND_MAX 32767

/* Memory allocation */
void* malloc(size_t size);
void* calloc(size_t nmemb, size_t size);
void* realloc(void* ptr, size_t size);
void free(void* ptr);
void* aligned_alloc(size_t alignment, size_t size);

/* Program termination */
void abort(void);
void exit(int status);
void _Exit(int status);
int atexit(void (*func)(void));
int at_quick_exit(void (*func)(void));
void quick_exit(int status);

/* Environment */
char* getenv(const char* name);
int system(const char* command);

/* String conversion */
int atoi(const char* nptr);
long atol(const char* nptr);
long long atoll(const char* nptr);
double atof(const char* nptr);

long strtol(const char* nptr, char** endptr, int base);
long long strtoll(const char* nptr, char** endptr, int base);
unsigned long strtoul(const char* nptr, char** endptr, int base);
unsigned long long strtoull(const char* nptr, char** endptr, int base);
double strtod(const char* nptr, char** endptr);
float strtof(const char* nptr, char** endptr);
long double strtold(const char* nptr, char** endptr);

/* Integer arithmetic */
int abs(int j);
long labs(long j);
long long llabs(long long j);

typedef struct { int quot; int rem; } div_t;
typedef struct { long quot; long rem; } ldiv_t;
typedef struct { long long quot; long long rem; } lldiv_t;

div_t div(int numer, int denom);
ldiv_t ldiv(long numer, long denom);
lldiv_t lldiv(long long numer, long long denom);

/* Random numbers */
int rand(void);
void srand(unsigned int seed);

/* Searching and sorting */
void* bsearch(const void* key, const void* base, size_t nmemb, size_t size,
              int (*compar)(const void*, const void*));
void qsort(void* base, size_t nmemb, size_t size,
           int (*compar)(const void*, const void*));

/* Multibyte/wide character conversion */
int mblen(const char* s, size_t n);
int mbtowc(wchar_t* pwc, const char* s, size_t n);
int wctomb(char* s, wchar_t wc);
size_t mbstowcs(wchar_t* dest, const char* src, size_t n);
size_t wcstombs(char* dest, const wchar_t* src, size_t n);

#endif /* _ADEAD_STDLIB_H */
