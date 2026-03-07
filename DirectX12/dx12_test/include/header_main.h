
/* header_main.h — ADead-BIB Universal Header v7.0 */
/* Un solo include. Todo disponible. Sin linker. */

/* === fastos_types.h === */
typedef signed char int8_t;
typedef short int16_t;
typedef int int32_t;
typedef long int64_t;
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long uint64_t;
typedef unsigned long size_t;
typedef long ssize_t;
typedef long ptrdiff_t;
typedef long intptr_t;
typedef unsigned long uintptr_t;
typedef int bool;

/* === fastos_limits.h === */

/* === fastos_stdio.h === */
typedef struct _IO_FILE FILE;
extern FILE *stdin;
extern FILE *stdout;
extern FILE *stderr;
int printf(const char *format, ...);
int fprintf(FILE *stream, const char *format, ...);
int sprintf(char *str, const char *format, ...);
int snprintf(char *str, size_t size, const char *format, ...);
int scanf(const char *format, ...);
int fscanf(FILE *stream, const char *format, ...);
int sscanf(const char *str, const char *format, ...);
int puts(const char *s);
int fputs(const char *s, FILE *stream);
int putchar(int c);
int getchar(void);
char *fgets(char *s, int size, FILE *stream);
FILE *fopen(const char *path, const char *mode);
int fclose(FILE *stream);
int fflush(FILE *stream);
size_t fread(void *ptr, size_t size, size_t nmemb, FILE *stream);
size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream);
int fseek(FILE *stream, long offset, int whence);
long ftell(FILE *stream);
void rewind(FILE *stream);
int feof(FILE *stream);
int ferror(FILE *stream);
int remove(const char *path);
int rename(const char *old, const char *new_name);
void perror(const char *s);

/* === fastos_stdlib.h === */
void *malloc(size_t size);
void *calloc(size_t nmemb, size_t size);
void *realloc(void *ptr, size_t size);
void free(void *ptr);
void abort(void);
void exit(int status);
int atexit(void (*function)(void));
char *getenv(const char *name);
int system(const char *command);
int abs(int j);
long labs(long j);
int atoi(const char *nptr);
long atol(const char *nptr);
double atof(const char *nptr);
long strtol(const char *nptr, char **endptr, int base);
double strtod(const char *nptr, char **endptr);
int rand(void);
void srand(unsigned int seed);
void qsort(void *base, size_t nmemb, size_t size, int (*compar)(const void *, const void *));
void *bsearch(const void *key, const void *base, size_t nmemb, size_t size, int (*compar)(const void *, const void *));

/* === fastos_string.h === */
void *memcpy(void *dest, const void *src, size_t n);
void *memmove(void *dest, const void *src, size_t n);
void *memset(void *s, int c, size_t n);
int memcmp(const void *s1, const void *s2, size_t n);
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

/* === fastos_math.h === */
double sin(double x);
double cos(double x);
double tan(double x);
double asin(double x);
double acos(double x);
double atan(double x);
double atan2(double y, double x);
double exp(double x);
double log(double x);
double log2(double x);
double log10(double x);
double pow(double base, double exponent);
double sqrt(double x);
double cbrt(double x);
double ceil(double x);
double floor(double x);
double round(double x);
double fabs(double x);
double fmod(double x, double y);
double hypot(double x, double y);
float sinf(float x);
float cosf(float x);
float sqrtf(float x);
float powf(float base, float exponent);
float fabsf(float x);

/* === fastos_time.h === */
typedef long time_t;
typedef long clock_t;
time_t time(time_t *tloc);
clock_t clock(void);
double difftime(time_t time1, time_t time0);

/* === fastos_assert.h === */

/* === fastos_errno.h === */
extern int errno;
char *strerror(int errnum);

/* TREE SHAKING: ADead-BIB includes only what you use. */
/* Hello World with this header → 2KB binary. */

/* === DirectX 12 Headers (fastos) === */
#include <fastos_windows.h>
#include <fastos_wrl.h>
#include <fastos_d3d12.h>
#include <fastos_dxgi.h>
