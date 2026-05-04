/* adeb_stdlib.h — General utilities */
#ifndef ADEB_STDLIB_H
#define ADEB_STDLIB_H

#include "adeb_types.h"

extern void* malloc(size_t size);
extern void* calloc(size_t n, size_t size);
extern void* realloc(void* ptr, size_t size);
extern void  free(void* ptr);
extern void  exit(int status);
extern void  abort(void);

extern int  atoi(const char* s);
extern long atol(const char* s);
extern double atof(const char* s);

extern void qsort(void* base, size_t n, size_t size,
                  int (*cmp)(const void*, const void*));
extern void* bsearch(const void* key, const void* base, size_t n, size_t size,
                     int (*cmp)(const void*, const void*));

extern int rand(void);
extern void srand(unsigned int seed);

#endif /* ADEB_STDLIB_H */
