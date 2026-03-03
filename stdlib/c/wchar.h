/*
 * ADead-BIB Standard Library
 * wchar.h - Wide Character Handling
 * 
 * Based on: C99/C11
 */

#ifndef _ADEAD_WCHAR_H
#define _ADEAD_WCHAR_H

#include "stddef.h"
#include "stdarg.h"
#include "stdio.h"

/* Wide character type */
#ifndef __WCHAR_TYPE__
typedef int wchar_t;
#endif

typedef int wint_t;
typedef struct { int __count; unsigned int __value; } mbstate_t;

/* WEOF */
#define WEOF ((wint_t)-1)

/* Wide string functions */
wchar_t* wcscpy(wchar_t* dest, const wchar_t* src);
wchar_t* wcsncpy(wchar_t* dest, const wchar_t* src, size_t n);
wchar_t* wcscat(wchar_t* dest, const wchar_t* src);
wchar_t* wcsncat(wchar_t* dest, const wchar_t* src, size_t n);
int wcscmp(const wchar_t* s1, const wchar_t* s2);
int wcsncmp(const wchar_t* s1, const wchar_t* s2, size_t n);
int wcscoll(const wchar_t* s1, const wchar_t* s2);
size_t wcsxfrm(wchar_t* dest, const wchar_t* src, size_t n);
wchar_t* wcschr(const wchar_t* s, wchar_t c);
wchar_t* wcsrchr(const wchar_t* s, wchar_t c);
size_t wcscspn(const wchar_t* s, const wchar_t* reject);
size_t wcsspn(const wchar_t* s, const wchar_t* accept);
wchar_t* wcspbrk(const wchar_t* s, const wchar_t* accept);
wchar_t* wcsstr(const wchar_t* haystack, const wchar_t* needle);
wchar_t* wcstok(wchar_t* s, const wchar_t* delim, wchar_t** ptr);
size_t wcslen(const wchar_t* s);

/* Wide memory functions */
wchar_t* wmemcpy(wchar_t* dest, const wchar_t* src, size_t n);
wchar_t* wmemmove(wchar_t* dest, const wchar_t* src, size_t n);
int wmemcmp(const wchar_t* s1, const wchar_t* s2, size_t n);
wchar_t* wmemchr(const wchar_t* s, wchar_t c, size_t n);
wchar_t* wmemset(wchar_t* s, wchar_t c, size_t n);

/* Multibyte/wide conversion */
int mbsinit(const mbstate_t* ps);
size_t mbrlen(const char* s, size_t n, mbstate_t* ps);
size_t mbrtowc(wchar_t* pwc, const char* s, size_t n, mbstate_t* ps);
size_t wcrtomb(char* s, wchar_t wc, mbstate_t* ps);
size_t mbsrtowcs(wchar_t* dest, const char** src, size_t len, mbstate_t* ps);
size_t wcsrtombs(char* dest, const wchar_t** src, size_t len, mbstate_t* ps);

/* Wide character I/O */
wint_t fgetwc(FILE* stream);
wint_t fputwc(wchar_t c, FILE* stream);
wint_t getwc(FILE* stream);
wint_t putwc(wchar_t c, FILE* stream);
wint_t getwchar(void);
wint_t putwchar(wchar_t c);
wchar_t* fgetws(wchar_t* s, int n, FILE* stream);
int fputws(const wchar_t* s, FILE* stream);
wint_t ungetwc(wint_t c, FILE* stream);

/* Wide formatted I/O */
int wprintf(const wchar_t* format, ...);
int fwprintf(FILE* stream, const wchar_t* format, ...);
int swprintf(wchar_t* s, size_t n, const wchar_t* format, ...);
int vwprintf(const wchar_t* format, va_list ap);
int vfwprintf(FILE* stream, const wchar_t* format, va_list ap);
int vswprintf(wchar_t* s, size_t n, const wchar_t* format, va_list ap);

int wscanf(const wchar_t* format, ...);
int fwscanf(FILE* stream, const wchar_t* format, ...);
int swscanf(const wchar_t* s, const wchar_t* format, ...);

/* Wide number conversion */
double wcstod(const wchar_t* nptr, wchar_t** endptr);
float wcstof(const wchar_t* nptr, wchar_t** endptr);
long double wcstold(const wchar_t* nptr, wchar_t** endptr);
long wcstol(const wchar_t* nptr, wchar_t** endptr, int base);
unsigned long wcstoul(const wchar_t* nptr, wchar_t** endptr, int base);
long long wcstoll(const wchar_t* nptr, wchar_t** endptr, int base);
unsigned long long wcstoull(const wchar_t* nptr, wchar_t** endptr, int base);

/* Wide time */
size_t wcsftime(wchar_t* s, size_t maxsize, const wchar_t* format, const struct tm* timeptr);

#endif /* _ADEAD_WCHAR_H */
