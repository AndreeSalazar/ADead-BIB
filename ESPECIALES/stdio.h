// ============================================================
// ADead-BIB — ESPECIALES/stdio.h
// Standard I/O — ADead-BIB implementation layer
// Compatible with C89/C99/C11/C++11
// ============================================================

#ifndef _ADEAD_STDIO_H
#define _ADEAD_STDIO_H

typedef unsigned long long size_t;
typedef long long fpos_t;
typedef struct { long long _handle; int _mode; int _flags; } FILE;

// Standard streams
extern FILE* stdin;
extern FILE* stdout;
extern FILE* stderr;

// EOF
#define EOF (-1)
#define NULL 0

// Buffer modes
#define _IOFBF  0
#define _IOLBF  1
#define _IONBF  2

// Seek modes
#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

// Max path / name
#define FILENAME_MAX  260
#define FOPEN_MAX     512
#define L_tmpnam      260
#define TMP_MAX       32767
#define BUFSIZ        512

// ── Formatted output ─────────────────────────────────────────
extern int  printf(const char* fmt, ...);
extern int  fprintf(FILE* stream, const char* fmt, ...);
extern int  sprintf(char* buf, const char* fmt, ...);
extern int  snprintf(char* buf, size_t n, const char* fmt, ...);
extern int  vprintf(const char* fmt, void* args);
extern int  vfprintf(FILE* stream, const char* fmt, void* args);
extern int  vsprintf(char* buf, const char* fmt, void* args);
extern int  vsnprintf(char* buf, size_t n, const char* fmt, void* args);

// ── Formatted input ──────────────────────────────────────────
extern int  scanf(const char* fmt, ...);
extern int  fscanf(FILE* stream, const char* fmt, ...);
extern int  sscanf(const char* buf, const char* fmt, ...);

// ── Character I/O ─────────────────────────────────────────────
extern int  putchar(int c);
extern int  getchar();
extern int  putc(int c, FILE* stream);
extern int  getc(FILE* stream);
extern int  fputc(int c, FILE* stream);
extern int  fgetc(FILE* stream);
extern int  puts(const char* s);
extern char* gets(char* s);
extern char* fgets(char* s, int n, FILE* stream);
extern int  fputs(const char* s, FILE* stream);
extern int  ungetc(int c, FILE* stream);

// ── File operations ──────────────────────────────────────────
extern FILE* fopen(const char* path, const char* mode);
extern FILE* freopen(const char* path, const char* mode, FILE* stream);
extern int   fclose(FILE* stream);
extern int   fflush(FILE* stream);
extern size_t fread(void* buf, size_t sz, size_t n, FILE* stream);
extern size_t fwrite(const void* buf, size_t sz, size_t n, FILE* stream);
extern int   fseek(FILE* stream, long offset, int whence);
extern long  ftell(FILE* stream);
extern void  rewind(FILE* stream);
extern int   fgetpos(FILE* stream, fpos_t* pos);
extern int   fsetpos(FILE* stream, const fpos_t* pos);
extern int   feof(FILE* stream);
extern int   ferror(FILE* stream);
extern void  clearerr(FILE* stream);
extern int   remove(const char* path);
extern int   rename(const char* old_name, const char* new_name);
extern FILE* tmpfile();
extern char* tmpnam(char* s);
extern void  setbuf(FILE* stream, char* buf);
extern int   setvbuf(FILE* stream, char* buf, int mode, size_t size);
extern void  perror(const char* msg);

#endif // _ADEAD_STDIO_H
