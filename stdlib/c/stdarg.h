/*
 * ADead-BIB Standard Library
 * stdarg.h - Variable arguments
 */

#ifndef _ADEAD_STDARG_H
#define _ADEAD_STDARG_H

/* Variable argument list type */
typedef __builtin_va_list va_list;

/* Variable argument macros */
#define va_start(ap, last) __builtin_va_start(ap, last)
#define va_arg(ap, type)   __builtin_va_arg(ap, type)
#define va_end(ap)         __builtin_va_end(ap)
#define va_copy(dest, src) __builtin_va_copy(dest, src)

#endif /* _ADEAD_STDARG_H */
