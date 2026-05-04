/* adeb_string.h — Memory & string primitives (puente con ASM-BIB) */
#ifndef ADEB_STRING_H
#define ADEB_STRING_H

#include "adeb_types.h"

extern void*  memcpy(void* dst, const void* src, size_t n);
extern void*  memmove(void* dst, const void* src, size_t n);
extern void*  memset(void* dst, int c, size_t n);
extern int    memcmp(const void* a, const void* b, size_t n);
extern void*  memchr(const void* s, int c, size_t n);

extern size_t strlen(const char* s);
extern char*  strcpy(char* dst, const char* src);
extern char*  strncpy(char* dst, const char* src, size_t n);
extern char*  strcat(char* dst, const char* src);
extern int    strcmp(const char* a, const char* b);
extern int    strncmp(const char* a, const char* b, size_t n);
extern char*  strchr(const char* s, int c);
extern char*  strstr(const char* hay, const char* needle);
extern char*  strdup(const char* s);

#endif /* ADEB_STRING_H */
