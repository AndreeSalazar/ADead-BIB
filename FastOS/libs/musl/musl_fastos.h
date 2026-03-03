/*
 * FastOS v2.0 — musl libc Integration
 * Based on: musl-libc.org
 * 
 * Provides standard C library functions for FastOS applications
 * Integrated with Rust safety layer for memory operations
 * 
 * Compile: adB cc musl_fastos.c -o musl.po --lib
 */

#ifndef MUSL_FASTOS_H
#define MUSL_FASTOS_H

#include "../../include/types.h"

/* ============================================================
 * Standard Types (C99/C11)
 * ============================================================ */

typedef long ssize_t;
typedef unsigned long size_t;
typedef long ptrdiff_t;
typedef long intptr_t;
typedef unsigned long uintptr_t;

typedef long off_t;
typedef int pid_t;
typedef unsigned int uid_t;
typedef unsigned int gid_t;
typedef unsigned int mode_t;
typedef long time_t;
typedef long clock_t;

/* NULL */
#ifndef NULL
#define NULL ((void*)0)
#endif

/* Boolean */
#ifndef __cplusplus
typedef _Bool bool;
#define true 1
#define false 0
#endif

/* ============================================================
 * String Functions
 * ============================================================ */

size_t strlen(const char *s);
char *strcpy(char *dest, const char *src);
char *strncpy(char *dest, const char *src, size_t n);
char *strcat(char *dest, const char *src);
char *strncat(char *dest, const char *src, size_t n);
int strcmp(const char *s1, const char *s2);
int strncmp(const char *s1, const char *s2, size_t n);
char *strchr(const char *s, int c);
char *strrchr(const char *s, int c);
char *strstr(const char *haystack, const char *needle);
char *strdup(const char *s);
char *strtok(char *str, const char *delim);

/* ============================================================
 * Memory Functions
 * ============================================================ */

void *memcpy(void *dest, const void *src, size_t n);
void *memmove(void *dest, const void *src, size_t n);
void *memset(void *s, int c, size_t n);
int memcmp(const void *s1, const void *s2, size_t n);
void *memchr(const void *s, int c, size_t n);

/* ============================================================
 * Memory Allocation (integrated with Rust allocator)
 * ============================================================ */

void *malloc(size_t size);
void *calloc(size_t nmemb, size_t size);
void *realloc(void *ptr, size_t size);
void free(void *ptr);
void *aligned_alloc(size_t alignment, size_t size);

/* ============================================================
 * I/O Functions
 * ============================================================ */

typedef struct _FILE FILE;

extern FILE *stdin;
extern FILE *stdout;
extern FILE *stderr;

FILE *fopen(const char *pathname, const char *mode);
FILE *fdopen(int fd, const char *mode);
int fclose(FILE *stream);
size_t fread(void *ptr, size_t size, size_t nmemb, FILE *stream);
size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream);
int fseek(FILE *stream, long offset, int whence);
long ftell(FILE *stream);
void rewind(FILE *stream);
int fflush(FILE *stream);
int feof(FILE *stream);
int ferror(FILE *stream);
void clearerr(FILE *stream);

int printf(const char *format, ...);
int fprintf(FILE *stream, const char *format, ...);
int sprintf(char *str, const char *format, ...);
int snprintf(char *str, size_t size, const char *format, ...);
int vprintf(const char *format, __builtin_va_list ap);
int vfprintf(FILE *stream, const char *format, __builtin_va_list ap);
int vsprintf(char *str, const char *format, __builtin_va_list ap);
int vsnprintf(char *str, size_t size, const char *format, __builtin_va_list ap);

int scanf(const char *format, ...);
int fscanf(FILE *stream, const char *format, ...);
int sscanf(const char *str, const char *format, ...);

int getchar(void);
int fgetc(FILE *stream);
char *fgets(char *s, int size, FILE *stream);
int putchar(int c);
int fputc(int c, FILE *stream);
int fputs(const char *s, FILE *stream);
int puts(const char *s);

/* ============================================================
 * File Descriptor I/O
 * ============================================================ */

#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

#define O_RDONLY    0x0000
#define O_WRONLY    0x0001
#define O_RDWR      0x0002
#define O_CREAT     0x0040
#define O_EXCL      0x0080
#define O_TRUNC     0x0200
#define O_APPEND    0x0400

int open(const char *pathname, int flags, ...);
int close(int fd);
ssize_t read(int fd, void *buf, size_t count);
ssize_t write(int fd, const void *buf, size_t count);
off_t lseek(int fd, off_t offset, int whence);
int dup(int oldfd);
int dup2(int oldfd, int newfd);
int pipe(int pipefd[2]);

/* ============================================================
 * Process Functions
 * ============================================================ */

void exit(int status);
void _exit(int status);
int atexit(void (*function)(void));
void abort(void);

pid_t getpid(void);
pid_t getppid(void);
pid_t fork(void);
int execve(const char *pathname, char *const argv[], char *const envp[]);
int execv(const char *pathname, char *const argv[]);
int execvp(const char *file, char *const argv[]);
pid_t wait(int *wstatus);
pid_t waitpid(pid_t pid, int *wstatus, int options);

/* ============================================================
 * Environment
 * ============================================================ */

extern char **environ;

char *getenv(const char *name);
int setenv(const char *name, const char *value, int overwrite);
int unsetenv(const char *name);
int putenv(char *string);

/* ============================================================
 * Time Functions
 * ============================================================ */

struct tm {
    int tm_sec;
    int tm_min;
    int tm_hour;
    int tm_mday;
    int tm_mon;
    int tm_year;
    int tm_wday;
    int tm_yday;
    int tm_isdst;
};

struct timespec {
    time_t tv_sec;
    long tv_nsec;
};

time_t time(time_t *tloc);
struct tm *localtime(const time_t *timep);
struct tm *gmtime(const time_t *timep);
time_t mktime(struct tm *tm);
char *ctime(const time_t *timep);
char *asctime(const struct tm *tm);
size_t strftime(char *s, size_t max, const char *format, const struct tm *tm);
int nanosleep(const struct timespec *req, struct timespec *rem);
unsigned int sleep(unsigned int seconds);
int usleep(unsigned int usec);

/* ============================================================
 * Math Functions
 * ============================================================ */

double sin(double x);
double cos(double x);
double tan(double x);
double asin(double x);
double acos(double x);
double atan(double x);
double atan2(double y, double x);
double sinh(double x);
double cosh(double x);
double tanh(double x);
double exp(double x);
double log(double x);
double log10(double x);
double pow(double x, double y);
double sqrt(double x);
double ceil(double x);
double floor(double x);
double fabs(double x);
double fmod(double x, double y);

/* ============================================================
 * Integer Conversion
 * ============================================================ */

int atoi(const char *nptr);
long atol(const char *nptr);
long long atoll(const char *nptr);
double atof(const char *nptr);
long strtol(const char *nptr, char **endptr, int base);
unsigned long strtoul(const char *nptr, char **endptr, int base);
long long strtoll(const char *nptr, char **endptr, int base);
unsigned long long strtoull(const char *nptr, char **endptr, int base);
double strtod(const char *nptr, char **endptr);

/* ============================================================
 * Character Classification
 * ============================================================ */

int isalnum(int c);
int isalpha(int c);
int isblank(int c);
int iscntrl(int c);
int isdigit(int c);
int isgraph(int c);
int islower(int c);
int isprint(int c);
int ispunct(int c);
int isspace(int c);
int isupper(int c);
int isxdigit(int c);
int tolower(int c);
int toupper(int c);

/* ============================================================
 * Error Handling
 * ============================================================ */

extern int errno;

#define EPERM           1
#define ENOENT          2
#define ESRCH           3
#define EINTR           4
#define EIO             5
#define ENXIO           6
#define E2BIG           7
#define ENOEXEC         8
#define EBADF           9
#define ECHILD          10
#define EAGAIN          11
#define ENOMEM          12
#define EACCES          13
#define EFAULT          14
#define EBUSY           16
#define EEXIST          17
#define EXDEV           18
#define ENODEV          19
#define ENOTDIR         20
#define EISDIR          21
#define EINVAL          22
#define ENFILE          23
#define EMFILE          24
#define ENOTTY          25
#define EFBIG           27
#define ENOSPC          28
#define ESPIPE          29
#define EROFS           30
#define EMLINK          31
#define EPIPE           32
#define EDOM            33
#define ERANGE          34
#define ENOSYS          38

char *strerror(int errnum);
void perror(const char *s);

/* ============================================================
 * Assertions
 * ============================================================ */

#ifdef NDEBUG
#define assert(expr) ((void)0)
#else
#define assert(expr) \
    ((expr) ? (void)0 : __assert_fail(#expr, __FILE__, __LINE__, __func__))
#endif

void __assert_fail(const char *expr, const char *file, int line, const char *func);

/* ============================================================
 * FastOS Extensions
 * ============================================================ */

/* Rust safety wrappers */
void *musl_safe_malloc(size_t size);
void *musl_safe_memcpy(void *dest, const void *src, size_t n);
void *musl_safe_memmove(void *dest, const void *src, size_t n);

/* Binary Guardian integration */
int musl_bg_verify(void *ptr, size_t size);

#endif /* MUSL_FASTOS_H */
