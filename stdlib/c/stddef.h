/*
 * ADead-BIB Standard Library
 * stddef.h - Common definitions
 */

#ifndef _ADEAD_STDDEF_H
#define _ADEAD_STDDEF_H

/* Null pointer constant */
#ifndef NULL
#define NULL ((void*)0)
#endif

/* Size type */
typedef unsigned long long size_t;

/* Signed size type */
typedef signed long long ssize_t;

/* Pointer difference type */
typedef signed long long ptrdiff_t;

/* Wide character type */
typedef int wchar_t;

/* Max alignment type */
typedef long double max_align_t;

/* Offset of member in struct */
#define offsetof(type, member) ((size_t)&((type*)0)->member)

#endif /* _ADEAD_STDDEF_H */
