/* adeb_stdio.h — I/O standard (proxy a runtime/io) */
#ifndef ADEB_STDIO_H
#define ADEB_STDIO_H

#include "adeb_types.h"

typedef struct FILE FILE;
extern FILE* stdout;
extern FILE* stderr;
extern FILE* stdin;

extern int   printf(const char* fmt, ...);
extern int   fprintf(FILE* stream, const char* fmt, ...);
extern int   sprintf(char* buf, const char* fmt, ...);
extern int   puts(const char* s);
extern int   putchar(int c);
extern int   getchar(void);

extern FILE* fopen(const char* path, const char* mode);
extern int   fclose(FILE* f);
extern size_t fread(void* buf, size_t size, size_t n, FILE* f);
extern size_t fwrite(const void* buf, size_t size, size_t n, FILE* f);
extern int   fflush(FILE* f);
extern int   feof(FILE* f);

#endif /* ADEB_STDIO_H */
