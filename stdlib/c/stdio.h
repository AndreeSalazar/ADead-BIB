/*
 * ADead-BIB Standard Library
 * stdio.h - Standard Input/Output
 */

#ifndef _ADEAD_STDIO_H
#define _ADEAD_STDIO_H

#include "stddef.h"
#include "stdarg.h"

/* File handle type */
typedef struct _FILE FILE;

/* Standard streams (provided by runtime) */
extern FILE* stdin;
extern FILE* stdout;
extern FILE* stderr;

/* End of file indicator */
#define EOF (-1)

/* Buffer sizes */
#define BUFSIZ 8192
#define FILENAME_MAX 260
#define FOPEN_MAX 20

/* Seek origins */
#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

/* File operations */
FILE* fopen(const char* filename, const char* mode);
FILE* freopen(const char* filename, const char* mode, FILE* stream);
int fclose(FILE* stream);
int fflush(FILE* stream);

/* Character I/O */
int fgetc(FILE* stream);
int fputc(int c, FILE* stream);
int getc(FILE* stream);
int putc(int c, FILE* stream);
int getchar(void);
int putchar(int c);
int ungetc(int c, FILE* stream);

/* String I/O */
char* fgets(char* s, int n, FILE* stream);
int fputs(const char* s, FILE* stream);
char* gets(char* s);  /* deprecated but included */
int puts(const char* s);

/* Formatted I/O */
int printf(const char* format, ...);
int fprintf(FILE* stream, const char* format, ...);
int sprintf(char* str, const char* format, ...);
int snprintf(char* str, size_t size, const char* format, ...);

int scanf(const char* format, ...);
int fscanf(FILE* stream, const char* format, ...);
int sscanf(const char* str, const char* format, ...);

/* Variable argument versions */
int vprintf(const char* format, va_list ap);
int vfprintf(FILE* stream, const char* format, va_list ap);
int vsprintf(char* str, const char* format, va_list ap);
int vsnprintf(char* str, size_t size, const char* format, va_list ap);

/* Direct I/O */
size_t fread(void* ptr, size_t size, size_t nmemb, FILE* stream);
size_t fwrite(const void* ptr, size_t size, size_t nmemb, FILE* stream);

/* File positioning */
int fseek(FILE* stream, long offset, int whence);
long ftell(FILE* stream);
void rewind(FILE* stream);
int fgetpos(FILE* stream, fpos_t* pos);
int fsetpos(FILE* stream, const fpos_t* pos);

/* Error handling */
void clearerr(FILE* stream);
int feof(FILE* stream);
int ferror(FILE* stream);
void perror(const char* s);

/* File removal/renaming */
int remove(const char* filename);
int rename(const char* oldname, const char* newname);

/* Temporary files */
FILE* tmpfile(void);
char* tmpnam(char* s);

/* Position type */
typedef long long fpos_t;

#endif /* _ADEAD_STDIO_H */
