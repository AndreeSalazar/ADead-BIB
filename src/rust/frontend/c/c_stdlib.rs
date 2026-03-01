// ============================================================
// ADead-BIB Built-in C Standard Library Headers
// ============================================================
// Provides C standard library declarations as built-in strings.
// When #include <stdio.h> is found, we inject these declarations
// directly — no filesystem, no sysroot, no GCC headers needed.
//
// ADead-BIB owns the headers. 💀🦈
//
// Covers ALL standard C headers + POSIX + FastOS extensions:
//   Fase 1: musl base (stdio, stdlib, string, math, etc.)
//   Fase 2: System (unistd, fcntl, sys/types, etc.)
//   Fase 3: Network (sys/socket, netinet, arpa, etc.)
//   Fase 4: Threading (pthread)
//   Fase 5: FastOS-specific (vulkan, wayland, freetype, etc.)
// ============================================================

/// Common prologue injected before any header declarations.
/// Defines fundamental types that all headers depend on.
pub const COMMON_PROLOGUE: &str = r#"
typedef unsigned long size_t;
typedef long ssize_t;
typedef long ptrdiff_t;
typedef unsigned int wchar_t;
typedef long intptr_t;
typedef unsigned long uintptr_t;
typedef long off_t;
typedef int pid_t;
typedef unsigned int uid_t;
typedef unsigned int gid_t;
typedef unsigned int mode_t;
typedef long time_t;
typedef long clock_t;
typedef int clockid_t;
typedef unsigned long ino_t;
typedef unsigned long dev_t;
typedef unsigned long nlink_t;
typedef long blksize_t;
typedef long blkcnt_t;

typedef signed char int8_t;
typedef short int16_t;
typedef int int32_t;
typedef long int64_t;
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long uint64_t;

typedef struct __va_list_tag { int x; } va_list;
"#;

/// Look up a header by name and return its declarations.
/// Returns None if the header is unknown.
pub fn get_header(name: &str) -> Option<&'static str> {
    match name {
        // ==========================================
        // C Standard Library (C99/C11)
        // ==========================================
        "stdio.h" => Some(HEADER_STDIO),
        "stdlib.h" => Some(HEADER_STDLIB),
        "string.h" => Some(HEADER_STRING),
        "strings.h" => Some(HEADER_STRINGS),
        "math.h" => Some(HEADER_MATH),
        "ctype.h" => Some(HEADER_CTYPE),
        "stdint.h" | "inttypes.h" => Some(HEADER_STDINT),
        "stdbool.h" => Some(HEADER_STDBOOL),
        "stddef.h" => Some(HEADER_STDDEF),
        "stdarg.h" => Some(HEADER_STDARG),
        "limits.h" => Some(HEADER_LIMITS),
        "float.h" => Some(HEADER_FLOAT),
        "errno.h" => Some(HEADER_ERRNO),
        "assert.h" => Some(HEADER_ASSERT),
        "signal.h" => Some(HEADER_SIGNAL),
        "setjmp.h" => Some(HEADER_SETJMP),
        "time.h" => Some(HEADER_TIME),
        "locale.h" => Some(HEADER_LOCALE),

        // ==========================================
        // POSIX / System
        // ==========================================
        "unistd.h" => Some(HEADER_UNISTD),
        "fcntl.h" => Some(HEADER_FCNTL),
        "sys/types.h" => Some(HEADER_SYS_TYPES),
        "sys/stat.h" => Some(HEADER_SYS_STAT),
        "sys/mman.h" => Some(HEADER_SYS_MMAN),
        "sys/ioctl.h" => Some(HEADER_SYS_IOCTL),
        "sys/wait.h" => Some(HEADER_SYS_WAIT),
        "sys/time.h" => Some(HEADER_SYS_TIME),
        "sys/select.h" => Some(HEADER_SYS_SELECT),
        "dirent.h" => Some(HEADER_DIRENT),
        "dlfcn.h" => Some(HEADER_DLFCN),
        "pthread.h" => Some(HEADER_PTHREAD),
        "semaphore.h" => Some(HEADER_SEMAPHORE),

        // ==========================================
        // Network
        // ==========================================
        "sys/socket.h" => Some(HEADER_SYS_SOCKET),
        "netinet/in.h" => Some(HEADER_NETINET_IN),
        "arpa/inet.h" => Some(HEADER_ARPA_INET),
        "netdb.h" => Some(HEADER_NETDB),
        "poll.h" => Some(HEADER_POLL),
        "sys/epoll.h" => Some(HEADER_SYS_EPOLL),

        // ==========================================
        // Compresión (Fase 3)
        // ==========================================
        "zlib.h" => Some(HEADER_ZLIB),
        "lz4.h" => Some(HEADER_LZ4),
        "zstd.h" => Some(HEADER_ZSTD),

        // ==========================================
        // Imágenes (Fase 3)
        // ==========================================
        "png.h" => Some(HEADER_PNG),
        "jpeglib.h" => Some(HEADER_JPEG),
        "webp/encode.h" | "webp/decode.h" => Some(HEADER_WEBP),

        // ==========================================
        // Audio (Fase 4)
        // ==========================================
        "vorbis/codec.h" | "vorbis/vorbisfile.h" => Some(HEADER_VORBIS),
        "opus/opus.h" | "opus.h" => Some(HEADER_OPUS),
        "FLAC/stream_decoder.h" | "FLAC/all.h" => Some(HEADER_FLAC),

        // ==========================================
        // GPU / Gráficos (Fase 2)
        // ==========================================
        "vulkan/vulkan.h" => Some(HEADER_VULKAN),
        "EGL/egl.h" => Some(HEADER_EGL),
        "xf86drm.h" | "libdrm/drm.h" => Some(HEADER_DRM),
        "wayland-client.h" => Some(HEADER_WAYLAND),

        // ==========================================
        // Fuentes/Texto (Fase 2)
        // ==========================================
        "ft2build.h" | "freetype/freetype.h" => Some(HEADER_FREETYPE),
        "hb.h" | "harfbuzz/hb.h" => Some(HEADER_HARFBUZZ),

        // ==========================================
        // Base de datos (Fase 3)
        // ==========================================
        "sqlite3.h" => Some(HEADER_SQLITE3),

        // ==========================================
        // Red/Security (Fase 3)
        // ==========================================
        "curl/curl.h" => Some(HEADER_CURL),
        "openssl/ssl.h" => Some(HEADER_OPENSSL),

        // ==========================================
        // Input / Hardware (Fase 2)
        // ==========================================
        "linux/input.h" | "libinput.h" => Some(HEADER_LIBINPUT),
        "xkbcommon/xkbcommon.h" => Some(HEADER_XKBCOMMON),
        "libudev.h" => Some(HEADER_LIBUDEV),
        "libusb.h" | "libusb-1.0/libusb.h" => Some(HEADER_LIBUSB),

        // ==========================================
        // Multimedia / FFmpeg (Fase 4)
        // ==========================================
        "libavcodec/avcodec.h" => Some(HEADER_AVCODEC),
        "libavformat/avformat.h" => Some(HEADER_AVFORMAT),
        "libavutil/avutil.h" => Some(HEADER_AVUTIL),
        "libswscale/swscale.h" => Some(HEADER_SWSCALE),

        // ==========================================
        // XML/JSON/Config (Fase 3)
        // ==========================================
        "expat.h" => Some(HEADER_EXPAT),

        _ => None,
    }
}

// ================================================================
//  C Standard Library Headers
// ================================================================

const HEADER_STDIO: &str = r#"
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
int putc(int c, FILE *stream);
int fputc(int c, FILE *stream);
int getchar(void);
int getc(FILE *stream);
int fgetc(FILE *stream);
char *fgets(char *s, int size, FILE *stream);
int ungetc(int c, FILE *stream);
FILE *fopen(const char *path, const char *mode);
FILE *freopen(const char *path, const char *mode, FILE *stream);
FILE *fdopen(int fd, const char *mode);
int fclose(FILE *stream);
int fflush(FILE *stream);
size_t fread(void *ptr, size_t size, size_t nmemb, FILE *stream);
size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream);
int fseek(FILE *stream, long offset, int whence);
long ftell(FILE *stream);
void rewind(FILE *stream);
int feof(FILE *stream);
int ferror(FILE *stream);
void clearerr(FILE *stream);
int fileno(FILE *stream);
int remove(const char *path);
int rename(const char *old, const char *new_name);
FILE *tmpfile(void);
char *tmpnam(char *s);
void perror(const char *s);
int vprintf(const char *format, va_list ap);
int vfprintf(FILE *stream, const char *format, va_list ap);
int vsprintf(char *str, const char *format, va_list ap);
int vsnprintf(char *str, size_t size, const char *format, va_list ap);
int setvbuf(FILE *stream, char *buf, int mode, size_t size);
void setbuf(FILE *stream, char *buf);
"#;

const HEADER_STDLIB: &str = r#"
void *malloc(size_t size);
void *calloc(size_t nmemb, size_t size);
void *realloc(void *ptr, size_t size);
void free(void *ptr);
void *aligned_alloc(size_t alignment, size_t size);
void abort(void);
void exit(int status);
void _exit(int status);
void _Exit(int status);
int atexit(void (*function)(void));
int at_quick_exit(void (*function)(void));
void quick_exit(int status);
char *getenv(const char *name);
int setenv(const char *name, const char *value, int overwrite);
int unsetenv(const char *name);
int putenv(char *string);
int system(const char *command);
int abs(int j);
long labs(long j);
long long llabs(long long j);
int atoi(const char *nptr);
long atol(const char *nptr);
long long atoll(const char *nptr);
double atof(const char *nptr);
long strtol(const char *nptr, char **endptr, int base);
unsigned long strtoul(const char *nptr, char **endptr, int base);
long long strtoll(const char *nptr, char **endptr, int base);
unsigned long long strtoull(const char *nptr, char **endptr, int base);
double strtod(const char *nptr, char **endptr);
float strtof(const char *nptr, char **endptr);
int rand(void);
void srand(unsigned int seed);
void qsort(void *base, size_t nmemb, size_t size, int (*compar)(const void *, const void *));
void *bsearch(const void *key, const void *base, size_t nmemb, size_t size, int (*compar)(const void *, const void *));
int mkstemp(char *template_str);
char *mkdtemp(char *template_str);
char *realpath(const char *path, char *resolved_path);
"#;

const HEADER_STRING: &str = r#"
void *memcpy(void *dest, const void *src, size_t n);
void *memmove(void *dest, const void *src, size_t n);
void *memset(void *s, int c, size_t n);
int memcmp(const void *s1, const void *s2, size_t n);
void *memchr(const void *s, int c, size_t n);
size_t strlen(const char *s);
size_t strnlen(const char *s, size_t maxlen);
char *strcpy(char *dest, const char *src);
char *strncpy(char *dest, const char *src, size_t n);
char *strcat(char *dest, const char *src);
char *strncat(char *dest, const char *src, size_t n);
int strcmp(const char *s1, const char *s2);
int strncmp(const char *s1, const char *s2, size_t n);
int strcasecmp(const char *s1, const char *s2);
int strncasecmp(const char *s1, const char *s2, size_t n);
char *strchr(const char *s, int c);
char *strrchr(const char *s, int c);
char *strstr(const char *haystack, const char *needle);
char *strtok(char *str, const char *delim);
char *strtok_r(char *str, const char *delim, char **saveptr);
char *strdup(const char *s);
char *strndup(const char *s, size_t n);
size_t strspn(const char *s, const char *accept);
size_t strcspn(const char *s, const char *reject);
char *strpbrk(const char *s, const char *accept);
char *strerror(int errnum);
int strerror_r(int errnum, char *buf, size_t buflen);
"#;

const HEADER_STRINGS: &str = r#"
int strcasecmp(const char *s1, const char *s2);
int strncasecmp(const char *s1, const char *s2, size_t n);
void bcopy(const void *src, void *dest, size_t n);
void bzero(void *s, size_t n);
int ffs(int i);
"#;

const HEADER_MATH: &str = r#"
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
double asinh(double x);
double acosh(double x);
double atanh(double x);
double exp(double x);
double exp2(double x);
double expm1(double x);
double log(double x);
double log2(double x);
double log10(double x);
double log1p(double x);
double pow(double base, double exponent);
double sqrt(double x);
double cbrt(double x);
double hypot(double x, double y);
double fabs(double x);
double ceil(double x);
double floor(double x);
double round(double x);
double trunc(double x);
double fmod(double x, double y);
double remainder(double x, double y);
double fmax(double x, double y);
double fmin(double x, double y);
double copysign(double x, double y);
double nan(const char *tagp);
int isnan(double x);
int isinf(double x);
int isfinite(double x);
int isnormal(double x);
int signbit(double x);
double ldexp(double x, int exp);
double frexp(double x, int *exp);
double modf(double x, double *iptr);
double scalbn(double x, int n);
int ilogb(double x);
double logb(double x);
double nextafter(double x, double y);
double erf(double x);
double erfc(double x);
double lgamma(double x);
double tgamma(double x);

float sinf(float x);
float cosf(float x);
float tanf(float x);
float sqrtf(float x);
float powf(float base, float exponent);
float fabsf(float x);
float ceilf(float x);
float floorf(float x);
float roundf(float x);
float fmodf(float x, float y);
float fmaxf(float x, float y);
float fminf(float x, float y);
float logf(float x);
float log2f(float x);
float expf(float x);
float atan2f(float y, float x);
"#;

const HEADER_CTYPE: &str = r#"
int isalpha(int c);
int isdigit(int c);
int isalnum(int c);
int isspace(int c);
int isupper(int c);
int islower(int c);
int isprint(int c);
int isgraph(int c);
int iscntrl(int c);
int ispunct(int c);
int isxdigit(int c);
int isblank(int c);
int isascii(int c);
int toupper(int c);
int tolower(int c);
int toascii(int c);
"#;

const HEADER_STDINT: &str = r#"
typedef signed char int8_t;
typedef short int16_t;
typedef int int32_t;
typedef long int64_t;
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long uint64_t;
typedef long intmax_t;
typedef unsigned long uintmax_t;
typedef long intptr_t;
typedef unsigned long uintptr_t;
typedef int int_least8_t;
typedef int int_least16_t;
typedef int int_least32_t;
typedef long int_least64_t;
typedef unsigned int uint_least8_t;
typedef unsigned int uint_least16_t;
typedef unsigned int uint_least32_t;
typedef unsigned long uint_least64_t;
typedef int int_fast8_t;
typedef int int_fast16_t;
typedef int int_fast32_t;
typedef long int_fast64_t;
typedef unsigned int uint_fast8_t;
typedef unsigned int uint_fast16_t;
typedef unsigned int uint_fast32_t;
typedef unsigned long uint_fast64_t;
"#;

const HEADER_STDBOOL: &str = r#"
typedef int bool;
"#;

const HEADER_STDDEF: &str = r#"
typedef unsigned long size_t;
typedef long ptrdiff_t;
typedef unsigned int wchar_t;
"#;

const HEADER_STDARG: &str = r#"
typedef struct __va_list_tag { int x; } va_list;
void va_start(va_list ap, ...);
void va_end(va_list ap);
void va_copy(va_list dest, va_list src);
"#;

const HEADER_LIMITS: &str = r#"
int __CHAR_BIT__;
int __SCHAR_MIN__;
int __SCHAR_MAX__;
int __UCHAR_MAX__;
int __SHRT_MIN__;
int __SHRT_MAX__;
int __USHRT_MAX__;
int __INT_MIN__;
int __INT_MAX__;
unsigned int __UINT_MAX__;
long __LONG_MIN__;
long __LONG_MAX__;
unsigned long __ULONG_MAX__;
long long __LLONG_MIN__;
long long __LLONG_MAX__;
unsigned long long __ULLONG_MAX__;
"#;

const HEADER_FLOAT: &str = r#"
int __FLT_RADIX__;
int __FLT_DIG__;
int __DBL_DIG__;
int __LDBL_DIG__;
float __FLT_MIN__;
float __FLT_MAX__;
float __FLT_EPSILON__;
double __DBL_MIN__;
double __DBL_MAX__;
double __DBL_EPSILON__;
"#;

const HEADER_ERRNO: &str = r#"
extern int errno;
int *__errno_location(void);
"#;

const HEADER_ASSERT: &str = r#"
void __assert_fail(const char *assertion, const char *file, unsigned int line, const char *function);
"#;

const HEADER_SIGNAL: &str = r#"
typedef void (*sighandler_t)(int);
typedef unsigned long sigset_t;

struct sigaction {
    sighandler_t sa_handler;
    unsigned long sa_flags;
    sigset_t sa_mask;
};

sighandler_t signal(int signum, sighandler_t handler);
int sigaction(int signum, const struct sigaction *act, struct sigaction *oldact);
int kill(int pid, int sig);
int raise(int sig);
int sigemptyset(sigset_t *set);
int sigfillset(sigset_t *set);
int sigaddset(sigset_t *set, int signum);
int sigdelset(sigset_t *set, int signum);
int sigismember(const sigset_t *set, int signum);
int sigprocmask(int how, const sigset_t *set, sigset_t *oldset);
int sigpending(sigset_t *set);
int sigsuspend(const sigset_t *mask);
"#;

const HEADER_SETJMP: &str = r#"
typedef long jmp_buf[8];
int setjmp(jmp_buf env);
void longjmp(jmp_buf env, int val);
"#;

const HEADER_TIME: &str = r#"
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
    long tv_sec;
    long tv_nsec;
};

time_t time(time_t *tloc);
struct tm *localtime(const time_t *timep);
struct tm *gmtime(const time_t *timep);
struct tm *localtime_r(const time_t *timep, struct tm *result);
struct tm *gmtime_r(const time_t *timep, struct tm *result);
time_t mktime(struct tm *tm);
double difftime(time_t time1, time_t time0);
size_t strftime(char *s, size_t max, const char *format, const struct tm *tm);
char *asctime(const struct tm *tm);
char *ctime(const time_t *timep);
clock_t clock(void);
int clock_gettime(clockid_t clk_id, struct timespec *tp);
int clock_settime(clockid_t clk_id, const struct timespec *tp);
int clock_getres(clockid_t clk_id, struct timespec *res);
int nanosleep(const struct timespec *req, struct timespec *rem);
unsigned int sleep(unsigned int seconds);
int usleep(unsigned int usec);
"#;

const HEADER_LOCALE: &str = r#"
struct lconv {
    char *decimal_point;
    char *thousands_sep;
    char *grouping;
    char *int_curr_symbol;
    char *currency_symbol;
};

char *setlocale(int category, const char *locale);
struct lconv *localeconv(void);
"#;

// ================================================================
//  POSIX / System Headers
// ================================================================

const HEADER_UNISTD: &str = r#"
int close(int fd);
ssize_t read(int fd, void *buf, size_t count);
ssize_t write(int fd, const void *buf, size_t count);
off_t lseek(int fd, off_t offset, int whence);
int dup(int oldfd);
int dup2(int oldfd, int newfd);
int pipe(int pipefd[2]);
int unlink(const char *pathname);
int rmdir(const char *pathname);
int link(const char *oldpath, const char *newpath);
int symlink(const char *target, const char *linkpath);
ssize_t readlink(const char *pathname, char *buf, size_t bufsiz);
int chdir(const char *path);
char *getcwd(char *buf, size_t size);
int mkdir(const char *pathname, mode_t mode);
int access(int fd, int mode);
pid_t fork(void);
pid_t getpid(void);
pid_t getppid(void);
uid_t getuid(void);
uid_t geteuid(void);
gid_t getgid(void);
gid_t getegid(void);
int setuid(uid_t uid);
int setgid(gid_t gid);
int execve(const char *pathname, char *const argv[], char *const envp[]);
int execvp(const char *file, char *const argv[]);
int execv(const char *pathname, char *const argv[]);
void _exit(int status);
unsigned int alarm(unsigned int seconds);
int pause(void);
int chown(const char *pathname, uid_t owner, gid_t group);
int fchown(int fd, uid_t owner, gid_t group);
int truncate(const char *path, off_t length);
int ftruncate(int fd, off_t length);
long sysconf(int name);
int isatty(int fd);
char *ttyname(int fd);
int fsync(int fd);
int fdatasync(int fd);
ssize_t pread(int fd, void *buf, size_t count, off_t offset);
ssize_t pwrite(int fd, const void *buf, size_t count, off_t offset);
"#;

const HEADER_FCNTL: &str = r#"
int open(const char *pathname, int flags, ...);
int openat(int dirfd, const char *pathname, int flags, ...);
int creat(const char *pathname, mode_t mode);
int fcntl(int fd, int cmd, ...);
"#;

const HEADER_SYS_TYPES: &str = r#"
typedef unsigned long size_t;
typedef long ssize_t;
typedef long off_t;
typedef int pid_t;
typedef unsigned int uid_t;
typedef unsigned int gid_t;
typedef unsigned int mode_t;
typedef unsigned long ino_t;
typedef unsigned long dev_t;
typedef unsigned long nlink_t;
"#;

const HEADER_SYS_STAT: &str = r#"
struct stat {
    dev_t st_dev;
    ino_t st_ino;
    mode_t st_mode;
    nlink_t st_nlink;
    uid_t st_uid;
    gid_t st_gid;
    dev_t st_rdev;
    off_t st_size;
    blksize_t st_blksize;
    blkcnt_t st_blocks;
    time_t st_atime;
    time_t st_mtime;
    time_t st_ctime;
};

int stat(const char *pathname, struct stat *statbuf);
int fstat(int fd, struct stat *statbuf);
int lstat(const char *pathname, struct stat *statbuf);
int chmod(const char *pathname, mode_t mode);
int fchmod(int fd, mode_t mode);
mode_t umask(mode_t mask);
int mkdir(const char *pathname, mode_t mode);
int mkfifo(const char *pathname, mode_t mode);
"#;

const HEADER_SYS_MMAN: &str = r#"
void *mmap(void *addr, size_t length, int prot, int flags, int fd, off_t offset);
int munmap(void *addr, size_t length);
int mprotect(void *addr, size_t len, int prot);
int msync(void *addr, size_t length, int flags);
int mlock(const void *addr, size_t len);
int munlock(const void *addr, size_t len);
int mlockall(int flags);
int munlockall(void);
int madvise(void *addr, size_t length, int advice);
int shm_open(const char *name, int oflag, mode_t mode);
int shm_unlink(const char *name);
"#;

const HEADER_SYS_IOCTL: &str = r#"
int ioctl(int fd, unsigned long request, ...);
"#;

const HEADER_SYS_WAIT: &str = r#"
pid_t wait(int *wstatus);
pid_t waitpid(pid_t pid, int *wstatus, int options);
"#;

const HEADER_SYS_TIME: &str = r#"
struct timeval {
    long tv_sec;
    long tv_usec;
};

struct itimerval {
    struct timeval it_interval;
    struct timeval it_value;
};

int gettimeofday(struct timeval *tv, void *tz);
int settimeofday(const struct timeval *tv, const void *tz);
int getitimer(int which, struct itimerval *curr_value);
int setitimer(int which, const struct itimerval *new_value, struct itimerval *old_value);
int utimes(const char *filename, const struct timeval times[2]);
"#;

const HEADER_SYS_SELECT: &str = r#"
typedef struct fd_set { unsigned long fds_bits[16]; } fd_set;

int select(int nfds, fd_set *readfds, fd_set *writefds, fd_set *exceptfds, struct timeval *timeout);
int pselect(int nfds, fd_set *readfds, fd_set *writefds, fd_set *exceptfds, const struct timespec *timeout, const sigset_t *sigmask);
"#;

const HEADER_DIRENT: &str = r#"
struct dirent {
    unsigned long d_ino;
    unsigned short d_reclen;
    unsigned char d_type;
    char d_name[256];
};

typedef struct __dirstream DIR;
DIR *opendir(const char *name);
struct dirent *readdir(DIR *dirp);
int readdir_r(DIR *dirp, struct dirent *entry, struct dirent **result);
int closedir(DIR *dirp);
void rewinddir(DIR *dirp);
void seekdir(DIR *dirp, long loc);
long telldir(DIR *dirp);
int scandir(const char *dirp, struct dirent ***namelist, int (*filter)(const struct dirent *), int (*compar)(const struct dirent **, const struct dirent **));
"#;

const HEADER_DLFCN: &str = r#"
void *dlopen(const char *filename, int flags);
int dlclose(void *handle);
void *dlsym(void *handle, const char *symbol);
char *dlerror(void);
"#;

const HEADER_PTHREAD: &str = r#"
typedef unsigned long pthread_t;
typedef struct { long __data[8]; } pthread_attr_t;
typedef struct { long __data[10]; } pthread_mutex_t;
typedef struct { long __data[1]; } pthread_mutexattr_t;
typedef struct { long __data[12]; } pthread_cond_t;
typedef struct { long __data[1]; } pthread_condattr_t;
typedef struct { long __data[14]; } pthread_rwlock_t;
typedef struct { long __data[2]; } pthread_rwlockattr_t;
typedef int pthread_once_t;
typedef unsigned int pthread_key_t;
typedef struct { long __data[8]; } pthread_barrier_t;
typedef struct { long __data[1]; } pthread_barrierattr_t;
typedef volatile int pthread_spinlock_t;

int pthread_create(pthread_t *thread, const pthread_attr_t *attr, void *(*start_routine)(void *), void *arg);
int pthread_join(pthread_t thread, void **retval);
int pthread_detach(pthread_t thread);
void pthread_exit(void *retval);
pthread_t pthread_self(void);
int pthread_equal(pthread_t t1, pthread_t t2);
int pthread_cancel(pthread_t thread);
int pthread_attr_init(pthread_attr_t *attr);
int pthread_attr_destroy(pthread_attr_t *attr);
int pthread_attr_setdetachstate(pthread_attr_t *attr, int detachstate);
int pthread_attr_setstacksize(pthread_attr_t *attr, size_t stacksize);

int pthread_mutex_init(pthread_mutex_t *mutex, const pthread_mutexattr_t *attr);
int pthread_mutex_destroy(pthread_mutex_t *mutex);
int pthread_mutex_lock(pthread_mutex_t *mutex);
int pthread_mutex_trylock(pthread_mutex_t *mutex);
int pthread_mutex_unlock(pthread_mutex_t *mutex);

int pthread_cond_init(pthread_cond_t *cond, const pthread_condattr_t *attr);
int pthread_cond_destroy(pthread_cond_t *cond);
int pthread_cond_signal(pthread_cond_t *cond);
int pthread_cond_broadcast(pthread_cond_t *cond);
int pthread_cond_wait(pthread_cond_t *cond, pthread_mutex_t *mutex);
int pthread_cond_timedwait(pthread_cond_t *cond, pthread_mutex_t *mutex, const struct timespec *abstime);

int pthread_rwlock_init(pthread_rwlock_t *rwlock, const pthread_rwlockattr_t *attr);
int pthread_rwlock_destroy(pthread_rwlock_t *rwlock);
int pthread_rwlock_rdlock(pthread_rwlock_t *rwlock);
int pthread_rwlock_wrlock(pthread_rwlock_t *rwlock);
int pthread_rwlock_unlock(pthread_rwlock_t *rwlock);
int pthread_rwlock_tryrdlock(pthread_rwlock_t *rwlock);
int pthread_rwlock_trywrlock(pthread_rwlock_t *rwlock);

int pthread_key_create(pthread_key_t *key, void (*destructor)(void *));
int pthread_key_delete(pthread_key_t key);
void *pthread_getspecific(pthread_key_t key);
int pthread_setspecific(pthread_key_t key, const void *value);

int pthread_once(pthread_once_t *once_control, void (*init_routine)(void));

int pthread_barrier_init(pthread_barrier_t *barrier, const pthread_barrierattr_t *attr, unsigned int count);
int pthread_barrier_destroy(pthread_barrier_t *barrier);
int pthread_barrier_wait(pthread_barrier_t *barrier);

int pthread_spin_init(pthread_spinlock_t *lock, int pshared);
int pthread_spin_destroy(pthread_spinlock_t *lock);
int pthread_spin_lock(pthread_spinlock_t *lock);
int pthread_spin_trylock(pthread_spinlock_t *lock);
int pthread_spin_unlock(pthread_spinlock_t *lock);

int pthread_setname_np(pthread_t thread, const char *name);
int pthread_getname_np(pthread_t thread, char *name, size_t len);
"#;

const HEADER_SEMAPHORE: &str = r#"
typedef struct { long __data[4]; } sem_t;

int sem_init(sem_t *sem, int pshared, unsigned int value);
int sem_destroy(sem_t *sem);
int sem_wait(sem_t *sem);
int sem_trywait(sem_t *sem);
int sem_timedwait(sem_t *sem, const struct timespec *abs_timeout);
int sem_post(sem_t *sem);
int sem_getvalue(sem_t *sem, int *sval);
sem_t *sem_open(const char *name, int oflag, ...);
int sem_close(sem_t *sem);
int sem_unlink(const char *name);
"#;

// ================================================================
//  Network Headers
// ================================================================

const HEADER_SYS_SOCKET: &str = r#"
typedef unsigned int socklen_t;
typedef unsigned short sa_family_t;

struct sockaddr {
    sa_family_t sa_family;
    char sa_data[14];
};

struct sockaddr_storage {
    sa_family_t ss_family;
    char __ss_padding[126];
};

struct msghdr {
    void *msg_name;
    socklen_t msg_namelen;
    struct iovec *msg_iov;
    size_t msg_iovlen;
    void *msg_control;
    size_t msg_controllen;
    int msg_flags;
};

struct iovec {
    void *iov_base;
    size_t iov_len;
};

int socket(int domain, int type_val, int protocol);
int bind(int sockfd, const struct sockaddr *addr, socklen_t addrlen);
int listen(int sockfd, int backlog);
int accept(int sockfd, struct sockaddr *addr, socklen_t *addrlen);
int connect(int sockfd, const struct sockaddr *addr, socklen_t addrlen);
ssize_t send(int sockfd, const void *buf, size_t len, int flags);
ssize_t recv(int sockfd, void *buf, size_t len, int flags);
ssize_t sendto(int sockfd, const void *buf, size_t len, int flags, const struct sockaddr *dest_addr, socklen_t addrlen);
ssize_t recvfrom(int sockfd, void *buf, size_t len, int flags, struct sockaddr *src_addr, socklen_t *addrlen);
ssize_t sendmsg(int sockfd, const struct msghdr *msg, int flags);
ssize_t recvmsg(int sockfd, struct msghdr *msg, int flags);
int shutdown(int sockfd, int how);
int getsockopt(int sockfd, int level, int optname, void *optval, socklen_t *optlen);
int setsockopt(int sockfd, int level, int optname, const void *optval, socklen_t optlen);
int getsockname(int sockfd, struct sockaddr *addr, socklen_t *addrlen);
int getpeername(int sockfd, struct sockaddr *addr, socklen_t *addrlen);
int socketpair(int domain, int type_val, int protocol, int sv[2]);
"#;

const HEADER_NETINET_IN: &str = r#"
typedef unsigned int in_addr_t;
typedef unsigned short in_port_t;

struct in_addr {
    in_addr_t s_addr;
};

struct in6_addr {
    unsigned char s6_addr[16];
};

struct sockaddr_in {
    sa_family_t sin_family;
    in_port_t sin_port;
    struct in_addr sin_addr;
    unsigned char sin_zero[8];
};

struct sockaddr_in6 {
    sa_family_t sin6_family;
    in_port_t sin6_port;
    unsigned int sin6_flowinfo;
    struct in6_addr sin6_addr;
    unsigned int sin6_scope_id;
};

unsigned short htons(unsigned short hostshort);
unsigned short ntohs(unsigned short netshort);
unsigned int htonl(unsigned int hostlong);
unsigned int ntohl(unsigned int netlong);
"#;

const HEADER_ARPA_INET: &str = r#"
in_addr_t inet_addr(const char *cp);
char *inet_ntoa(struct in_addr in);
int inet_pton(int af, const char *src, void *dst);
const char *inet_ntop(int af, const void *src, char *dst, socklen_t size);
int inet_aton(const char *cp, struct in_addr *inp);
"#;

const HEADER_NETDB: &str = r#"
struct hostent {
    char *h_name;
    char **h_aliases;
    int h_addrtype;
    int h_length;
    char **h_addr_list;
};

struct addrinfo {
    int ai_flags;
    int ai_family;
    int ai_socktype;
    int ai_protocol;
    socklen_t ai_addrlen;
    struct sockaddr *ai_addr;
    char *ai_canonname;
    struct addrinfo *ai_next;
};

struct hostent *gethostbyname(const char *name);
struct hostent *gethostbyaddr(const void *addr, socklen_t len, int type_val);
int getaddrinfo(const char *node, const char *service, const struct addrinfo *hints, struct addrinfo **res);
void freeaddrinfo(struct addrinfo *res);
const char *gai_strerror(int errcode);
int getnameinfo(const struct sockaddr *sa, socklen_t salen, char *host, socklen_t hostlen, char *serv, socklen_t servlen, int flags);
"#;

const HEADER_POLL: &str = r#"
struct pollfd {
    int fd;
    short events;
    short revents;
};

int poll(struct pollfd *fds, unsigned long nfds, int timeout);
int ppoll(struct pollfd *fds, unsigned long nfds, const struct timespec *tmo_p, const sigset_t *sigmask);
"#;

const HEADER_SYS_EPOLL: &str = r#"
typedef union epoll_data {
    void *ptr;
    int fd;
    unsigned int u32;
    unsigned long u64;
} epoll_data_t;

struct epoll_event {
    unsigned int events;
    epoll_data_t data;
};

int epoll_create(int size);
int epoll_create1(int flags);
int epoll_ctl(int epfd, int op, int fd, struct epoll_event *event);
int epoll_wait(int epfd, struct epoll_event *events, int maxevents, int timeout);
int epoll_pwait(int epfd, struct epoll_event *events, int maxevents, int timeout, const sigset_t *sigmask);
"#;

// ================================================================
//  Compresión Headers
// ================================================================

const HEADER_ZLIB: &str = r#"
typedef void *voidp;
typedef unsigned char Byte;
typedef unsigned int uInt;
typedef unsigned long uLong;
typedef long z_off_t;

typedef struct z_stream_s {
    const unsigned char *next_in;
    unsigned int avail_in;
    unsigned long total_in;
    unsigned char *next_out;
    unsigned int avail_out;
    unsigned long total_out;
    const char *msg;
    void *state;
} z_stream;

typedef z_stream *z_streamp;

const char *zlibVersion(void);
int deflateInit(z_streamp strm, int level);
int deflate(z_streamp strm, int flush);
int deflateEnd(z_streamp strm);
int inflateInit(z_streamp strm);
int inflate(z_streamp strm, int flush);
int inflateEnd(z_streamp strm);
uLong compressBound(uLong sourceLen);
int compress(unsigned char *dest, uLong *destLen, const unsigned char *source, uLong sourceLen);
int compress2(unsigned char *dest, uLong *destLen, const unsigned char *source, uLong sourceLen, int level);
int uncompress(unsigned char *dest, uLong *destLen, const unsigned char *source, uLong sourceLen);
uLong crc32(uLong crc, const unsigned char *buf, uInt len);
uLong adler32(uLong adler, const unsigned char *buf, uInt len);
"#;

const HEADER_LZ4: &str = r#"
int LZ4_compress_default(const char *src, char *dst, int srcSize, int dstCapacity);
int LZ4_decompress_safe(const char *src, char *dst, int compressedSize, int dstCapacity);
int LZ4_compressBound(int inputSize);
int LZ4_compress_fast(const char *src, char *dst, int srcSize, int dstCapacity, int acceleration);
int LZ4_compress_HC(const char *src, char *dst, int srcSize, int dstCapacity, int compressionLevel);
int LZ4_versionNumber(void);
const char *LZ4_versionString(void);
"#;

const HEADER_ZSTD: &str = r#"
typedef struct ZSTD_CCtx_s ZSTD_CCtx;
typedef struct ZSTD_DCtx_s ZSTD_DCtx;

unsigned ZSTD_versionNumber(void);
const char *ZSTD_versionString(void);
size_t ZSTD_compress(void *dst, size_t dstCapacity, const void *src, size_t srcSize, int compressionLevel);
size_t ZSTD_decompress(void *dst, size_t dstCapacity, const void *src, size_t compressedSize);
unsigned long long ZSTD_getFrameContentSize(const void *src, size_t srcSize);
size_t ZSTD_compressBound(size_t srcSize);
unsigned ZSTD_isError(size_t code);
const char *ZSTD_getErrorName(size_t code);
int ZSTD_minCLevel(void);
int ZSTD_maxCLevel(void);
int ZSTD_defaultCLevel(void);
ZSTD_CCtx *ZSTD_createCCtx(void);
size_t ZSTD_freeCCtx(ZSTD_CCtx *cctx);
ZSTD_DCtx *ZSTD_createDCtx(void);
size_t ZSTD_freeDCtx(ZSTD_DCtx *dctx);
"#;

// ================================================================
//  Imágenes Headers
// ================================================================

const HEADER_PNG: &str = r#"
typedef struct png_struct_def png_struct;
typedef png_struct *png_structp;
typedef struct png_info_def png_info;
typedef png_info *png_infop;
typedef unsigned char png_byte;
typedef unsigned int png_uint_32;
typedef int png_int_32;
typedef unsigned short png_uint_16;
typedef unsigned char *png_bytep;
typedef unsigned char **png_bytepp;
typedef const char *png_const_charp;

png_structp png_create_read_struct(const char *user_png_ver, void *error_ptr, void *error_fn, void *warn_fn);
png_structp png_create_write_struct(const char *user_png_ver, void *error_ptr, void *error_fn, void *warn_fn);
png_infop png_create_info_struct(png_structp png_ptr);
void png_destroy_read_struct(png_structp *png_ptr_ptr, png_infop *info_ptr_ptr, png_infop *end_info_ptr_ptr);
void png_destroy_write_struct(png_structp *png_ptr_ptr, png_infop *info_ptr_ptr);
void png_init_io(png_structp png_ptr, FILE *fp);
void png_read_info(png_structp png_ptr, png_infop info_ptr);
void png_read_image(png_structp png_ptr, png_bytepp image);
void png_read_end(png_structp png_ptr, png_infop info_ptr);
void png_write_info(png_structp png_ptr, png_infop info_ptr);
void png_write_image(png_structp png_ptr, png_bytepp image);
void png_write_end(png_structp png_ptr, png_infop info_ptr);
png_uint_32 png_get_image_width(png_structp png_ptr, png_infop info_ptr);
png_uint_32 png_get_image_height(png_structp png_ptr, png_infop info_ptr);
png_byte png_get_bit_depth(png_structp png_ptr, png_infop info_ptr);
png_byte png_get_color_type(png_structp png_ptr, png_infop info_ptr);
void png_set_IHDR(png_structp png_ptr, png_infop info_ptr, png_uint_32 width, png_uint_32 height, int bit_depth, int color_type, int interlace_method, int compression_method, int filter_method);
"#;

const HEADER_JPEG: &str = r#"
typedef struct jpeg_decompress_struct {
    int image_width;
    int image_height;
    int num_components;
    int output_width;
    int output_height;
    int output_components;
} jpeg_decompress_struct;

typedef struct jpeg_compress_struct {
    int image_width;
    int image_height;
    int input_components;
    int in_color_space;
} jpeg_compress_struct;

typedef struct jpeg_error_mgr { int msg_code; } jpeg_error_mgr;

struct jpeg_error_mgr *jpeg_std_error(struct jpeg_error_mgr *err);
void jpeg_create_decompress(jpeg_decompress_struct *cinfo);
void jpeg_create_compress(jpeg_compress_struct *cinfo);
int jpeg_read_header(jpeg_decompress_struct *cinfo, int require_image);
int jpeg_start_decompress(jpeg_decompress_struct *cinfo);
unsigned int jpeg_read_scanlines(jpeg_decompress_struct *cinfo, unsigned char **scanlines, unsigned int max_lines);
int jpeg_finish_decompress(jpeg_decompress_struct *cinfo);
void jpeg_destroy_decompress(jpeg_decompress_struct *cinfo);
void jpeg_destroy_compress(jpeg_compress_struct *cinfo);
void jpeg_stdio_src(jpeg_decompress_struct *cinfo, FILE *infile);
void jpeg_stdio_dest(jpeg_compress_struct *cinfo, FILE *outfile);
void jpeg_set_defaults(jpeg_compress_struct *cinfo);
void jpeg_set_quality(jpeg_compress_struct *cinfo, int quality, int force_baseline);
void jpeg_start_compress(jpeg_compress_struct *cinfo, int write_all_tables);
unsigned int jpeg_write_scanlines(jpeg_compress_struct *cinfo, unsigned char **scanlines, unsigned int num_lines);
void jpeg_finish_compress(jpeg_compress_struct *cinfo);
"#;

const HEADER_WEBP: &str = r#"
uint8_t *WebPDecodeRGBA(const uint8_t *data, size_t data_size, int *width, int *height);
uint8_t *WebPDecodeRGB(const uint8_t *data, size_t data_size, int *width, int *height);
uint8_t *WebPDecodeBGRA(const uint8_t *data, size_t data_size, int *width, int *height);
int WebPGetInfo(const uint8_t *data, size_t data_size, int *width, int *height);
size_t WebPEncodeRGBA(const uint8_t *rgba, int width, int height, int stride, float quality_factor, uint8_t **output);
size_t WebPEncodeRGB(const uint8_t *rgb, int width, int height, int stride, float quality_factor, uint8_t **output);
size_t WebPEncodeLosslessRGBA(const uint8_t *rgba, int width, int height, int stride, uint8_t **output);
void WebPFree(void *ptr);
int WebPGetDecoderVersion(void);
int WebPGetEncoderVersion(void);
"#;

// ================================================================
//  Audio Headers
// ================================================================

const HEADER_VORBIS: &str = r#"
typedef struct OggVorbis_File OggVorbis_File;
typedef struct vorbis_info {
    int version;
    int channels;
    long rate;
    long bitrate_upper;
    long bitrate_nominal;
    long bitrate_lower;
} vorbis_info;

typedef struct vorbis_comment {
    char **user_comments;
    int *comment_lengths;
    int comments;
    char *vendor;
} vorbis_comment;

int ov_open_callbacks(void *datasource, OggVorbis_File *vf, const char *initial, long ibytes, void *callbacks);
int ov_fopen(const char *path, OggVorbis_File *vf);
int ov_clear(OggVorbis_File *vf);
vorbis_info *ov_info(OggVorbis_File *vf, int link);
vorbis_comment *ov_comment(OggVorbis_File *vf, int link);
long ov_read(OggVorbis_File *vf, char *buffer, int length, int bigendianp, int word, int sgned, int *bitstream);
int ov_seekable(OggVorbis_File *vf);
long ov_pcm_total(OggVorbis_File *vf, int i);
double ov_time_total(OggVorbis_File *vf, int i);
int ov_pcm_seek(OggVorbis_File *vf, long pos);
int ov_time_seek(OggVorbis_File *vf, double pos);
double ov_time_tell(OggVorbis_File *vf);
long ov_pcm_tell(OggVorbis_File *vf);
"#;

const HEADER_OPUS: &str = r#"
typedef struct OpusEncoder OpusEncoder;
typedef struct OpusDecoder OpusDecoder;

OpusEncoder *opus_encoder_create(int Fs, int channels, int application, int *error);
int opus_encode(OpusEncoder *st, const short *pcm, int frame_size, unsigned char *data, int max_data_bytes);
int opus_encode_float(OpusEncoder *st, const float *pcm, int frame_size, unsigned char *data, int max_data_bytes);
void opus_encoder_destroy(OpusEncoder *st);
int opus_encoder_ctl(OpusEncoder *st, int request, ...);

OpusDecoder *opus_decoder_create(int Fs, int channels, int *error);
int opus_decode(OpusDecoder *st, const unsigned char *data, int len, short *pcm, int frame_size, int decode_fec);
int opus_decode_float(OpusDecoder *st, const unsigned char *data, int len, float *pcm, int frame_size, int decode_fec);
void opus_decoder_destroy(OpusDecoder *st);

const char *opus_strerror(int error);
const char *opus_get_version_string(void);
int opus_packet_get_bandwidth(const unsigned char *data);
int opus_packet_get_nb_channels(const unsigned char *data);
int opus_packet_get_nb_frames(const unsigned char *data, int len);
int opus_packet_get_samples_per_frame(const unsigned char *data, int Fs);
"#;

const HEADER_FLAC: &str = r#"
typedef struct FLAC__StreamDecoder FLAC__StreamDecoder;
typedef struct FLAC__StreamEncoder FLAC__StreamEncoder;

typedef enum {
    FLAC__STREAM_DECODER_SEARCH_FOR_METADATA = 0,
    FLAC__STREAM_DECODER_READ_METADATA,
    FLAC__STREAM_DECODER_SEARCH_FOR_FRAME_SYNC,
    FLAC__STREAM_DECODER_READ_FRAME,
    FLAC__STREAM_DECODER_END_OF_STREAM,
    FLAC__STREAM_DECODER_ABORTED
} FLAC__StreamDecoderState;

FLAC__StreamDecoder *FLAC__stream_decoder_new(void);
void FLAC__stream_decoder_delete(FLAC__StreamDecoder *decoder);
int FLAC__stream_decoder_init_file(FLAC__StreamDecoder *decoder, const char *filename, void *write_callback, void *metadata_callback, void *error_callback, void *client_data);
int FLAC__stream_decoder_process_single(FLAC__StreamDecoder *decoder);
int FLAC__stream_decoder_process_until_end_of_stream(FLAC__StreamDecoder *decoder);
int FLAC__stream_decoder_finish(FLAC__StreamDecoder *decoder);
FLAC__StreamDecoderState FLAC__stream_decoder_get_state(const FLAC__StreamDecoder *decoder);
unsigned FLAC__stream_decoder_get_channels(const FLAC__StreamDecoder *decoder);
unsigned FLAC__stream_decoder_get_sample_rate(const FLAC__StreamDecoder *decoder);
unsigned FLAC__stream_decoder_get_bits_per_sample(const FLAC__StreamDecoder *decoder);
unsigned long long FLAC__stream_decoder_get_total_samples(const FLAC__StreamDecoder *decoder);
"#;

// ================================================================
//  GPU / Gráficos Headers
// ================================================================

const HEADER_VULKAN: &str = r#"
typedef struct VkInstance_T *VkInstance;
typedef struct VkPhysicalDevice_T *VkPhysicalDevice;
typedef struct VkDevice_T *VkDevice;
typedef struct VkQueue_T *VkQueue;
typedef struct VkCommandBuffer_T *VkCommandBuffer;
typedef struct VkCommandPool_T *VkCommandPool;
typedef struct VkBuffer_T *VkBuffer;
typedef struct VkImage_T *VkImage;
typedef struct VkImageView_T *VkImageView;
typedef struct VkShaderModule_T *VkShaderModule;
typedef struct VkPipeline_T *VkPipeline;
typedef struct VkPipelineLayout_T *VkPipelineLayout;
typedef struct VkRenderPass_T *VkRenderPass;
typedef struct VkFramebuffer_T *VkFramebuffer;
typedef struct VkSemaphore_T *VkSemaphore;
typedef struct VkFence_T *VkFence;
typedef struct VkDeviceMemory_T *VkDeviceMemory;
typedef struct VkDescriptorSet_T *VkDescriptorSet;
typedef struct VkDescriptorSetLayout_T *VkDescriptorSetLayout;
typedef struct VkDescriptorPool_T *VkDescriptorPool;
typedef struct VkSampler_T *VkSampler;
typedef struct VkSurfaceKHR_T *VkSurfaceKHR;
typedef struct VkSwapchainKHR_T *VkSwapchainKHR;
typedef unsigned int VkFlags;
typedef unsigned int VkBool32;
typedef unsigned long VkDeviceSize;
typedef int VkResult;

typedef struct VkApplicationInfo {
    int sType;
    const void *pNext;
    const char *pApplicationName;
    unsigned int applicationVersion;
    const char *pEngineName;
    unsigned int engineVersion;
    unsigned int apiVersion;
} VkApplicationInfo;

typedef struct VkInstanceCreateInfo {
    int sType;
    const void *pNext;
    VkFlags flags;
    const VkApplicationInfo *pApplicationInfo;
    unsigned int enabledLayerCount;
    const char *const *ppEnabledLayerNames;
    unsigned int enabledExtensionCount;
    const char *const *ppEnabledExtensionNames;
} VkInstanceCreateInfo;

VkResult vkCreateInstance(const VkInstanceCreateInfo *pCreateInfo, const void *pAllocator, VkInstance *pInstance);
void vkDestroyInstance(VkInstance instance, const void *pAllocator);
VkResult vkEnumeratePhysicalDevices(VkInstance instance, unsigned int *pPhysicalDeviceCount, VkPhysicalDevice *pPhysicalDevices);
VkResult vkCreateDevice(VkPhysicalDevice physicalDevice, const void *pCreateInfo, const void *pAllocator, VkDevice *pDevice);
void vkDestroyDevice(VkDevice device, const void *pAllocator);
void vkGetDeviceQueue(VkDevice device, unsigned int queueFamilyIndex, unsigned int queueIndex, VkQueue *pQueue);
VkResult vkQueueSubmit(VkQueue queue, unsigned int submitCount, const void *pSubmits, VkFence fence);
VkResult vkQueueWaitIdle(VkQueue queue);
VkResult vkDeviceWaitIdle(VkDevice device);
VkResult vkAllocateMemory(VkDevice device, const void *pAllocateInfo, const void *pAllocator, VkDeviceMemory *pMemory);
void vkFreeMemory(VkDevice device, VkDeviceMemory memory, const void *pAllocator);
VkResult vkMapMemory(VkDevice device, VkDeviceMemory memory, VkDeviceSize offset, VkDeviceSize size, VkFlags flags, void **ppData);
void vkUnmapMemory(VkDevice device, VkDeviceMemory memory);
VkResult vkCreateBuffer(VkDevice device, const void *pCreateInfo, const void *pAllocator, VkBuffer *pBuffer);
void vkDestroyBuffer(VkDevice device, VkBuffer buffer, const void *pAllocator);
VkResult vkCreateImage(VkDevice device, const void *pCreateInfo, const void *pAllocator, VkImage *pImage);
void vkDestroyImage(VkDevice device, VkImage image, const void *pAllocator);
VkResult vkCreateCommandPool(VkDevice device, const void *pCreateInfo, const void *pAllocator, VkCommandPool *pCommandPool);
void vkDestroyCommandPool(VkDevice device, VkCommandPool commandPool, const void *pAllocator);
VkResult vkAllocateCommandBuffers(VkDevice device, const void *pAllocateInfo, VkCommandBuffer *pCommandBuffers);
VkResult vkBeginCommandBuffer(VkCommandBuffer commandBuffer, const void *pBeginInfo);
VkResult vkEndCommandBuffer(VkCommandBuffer commandBuffer);
void vkCmdBindPipeline(VkCommandBuffer commandBuffer, int pipelineBindPoint, VkPipeline pipeline);
void vkCmdDraw(VkCommandBuffer commandBuffer, unsigned int vertexCount, unsigned int instanceCount, unsigned int firstVertex, unsigned int firstInstance);
void vkCmdDrawIndexed(VkCommandBuffer commandBuffer, unsigned int indexCount, unsigned int instanceCount, unsigned int firstIndex, int vertexOffset, unsigned int firstInstance);
void vkCmdDispatch(VkCommandBuffer commandBuffer, unsigned int groupCountX, unsigned int groupCountY, unsigned int groupCountZ);
void vkCmdCopyBuffer(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkBuffer dstBuffer, unsigned int regionCount, const void *pRegions);
void vkCmdBeginRenderPass(VkCommandBuffer commandBuffer, const void *pRenderPassBegin, int contents);
void vkCmdEndRenderPass(VkCommandBuffer commandBuffer);
VkResult vkCreateShaderModule(VkDevice device, const void *pCreateInfo, const void *pAllocator, VkShaderModule *pShaderModule);
void vkDestroyShaderModule(VkDevice device, VkShaderModule shaderModule, const void *pAllocator);
VkResult vkCreateGraphicsPipelines(VkDevice device, void *pipelineCache, unsigned int createInfoCount, const void *pCreateInfos, const void *pAllocator, VkPipeline *pPipelines);
VkResult vkCreateComputePipelines(VkDevice device, void *pipelineCache, unsigned int createInfoCount, const void *pCreateInfos, const void *pAllocator, VkPipeline *pPipelines);
void vkDestroyPipeline(VkDevice device, VkPipeline pipeline, const void *pAllocator);
VkResult vkCreateSemaphore(VkDevice device, const void *pCreateInfo, const void *pAllocator, VkSemaphore *pSemaphore);
VkResult vkCreateFence(VkDevice device, const void *pCreateInfo, const void *pAllocator, VkFence *pFence);
VkResult vkWaitForFences(VkDevice device, unsigned int fenceCount, const VkFence *pFences, VkBool32 waitAll, unsigned long timeout);
VkResult vkResetFences(VkDevice device, unsigned int fenceCount, const VkFence *pFences);
"#;

const HEADER_EGL: &str = r#"
typedef void *EGLDisplay;
typedef void *EGLSurface;
typedef void *EGLContext;
typedef void *EGLConfig;
typedef unsigned int EGLBoolean;
typedef int EGLint;

EGLDisplay eglGetDisplay(void *native_display);
EGLBoolean eglInitialize(EGLDisplay dpy, EGLint *major, EGLint *minor);
EGLBoolean eglTerminate(EGLDisplay dpy);
EGLBoolean eglChooseConfig(EGLDisplay dpy, const EGLint *attrib_list, EGLConfig *configs, EGLint config_size, EGLint *num_config);
EGLSurface eglCreateWindowSurface(EGLDisplay dpy, EGLConfig config, void *win, const EGLint *attrib_list);
EGLContext eglCreateContext(EGLDisplay dpy, EGLConfig config, EGLContext share_context, const EGLint *attrib_list);
EGLBoolean eglMakeCurrent(EGLDisplay dpy, EGLSurface draw, EGLSurface read, EGLContext ctx);
EGLBoolean eglSwapBuffers(EGLDisplay dpy, EGLSurface surface);
EGLBoolean eglDestroyContext(EGLDisplay dpy, EGLContext ctx);
EGLBoolean eglDestroySurface(EGLDisplay dpy, EGLSurface surface);
void *eglGetProcAddress(const char *procname);
EGLint eglGetError(void);
"#;

const HEADER_DRM: &str = r#"
typedef struct _drmModeRes {
    int count_fbs;
    unsigned int *fbs;
    int count_crtcs;
    unsigned int *crtcs;
    int count_connectors;
    unsigned int *connectors;
    int count_encoders;
    unsigned int *encoders;
    unsigned int min_width;
    unsigned int max_width;
    unsigned int min_height;
    unsigned int max_height;
} drmModeRes;

typedef struct _drmModeConnector {
    unsigned int connector_id;
    unsigned int encoder_id;
    unsigned int connector_type;
    unsigned int connector_type_id;
    int connection;
    unsigned int mmWidth;
    unsigned int mmHeight;
    int count_modes;
    void *modes;
    int count_props;
    unsigned int *props;
    unsigned long *prop_values;
    int count_encoders;
    unsigned int *encoders;
} drmModeConnector;

int drmOpen(const char *name, const char *busid);
int drmClose(int fd);
drmModeRes *drmModeGetResources(int fd);
void drmModeFreeResources(drmModeRes *ptr);
drmModeConnector *drmModeGetConnector(int fd, unsigned int connectorId);
void drmModeFreeConnector(drmModeConnector *ptr);
int drmModeSetCrtc(int fd, unsigned int crtcId, unsigned int bufferId, unsigned int x, unsigned int y, unsigned int *connectors, int count, void *mode);
int drmModePageFlip(int fd, unsigned int crtc_id, unsigned int fb_id, unsigned int flags, void *user_data);
int drmModeAddFB(int fd, unsigned int width, unsigned int height, unsigned char depth, unsigned char bpp, unsigned int pitch, unsigned int bo_handle, unsigned int *buf_id);
int drmModeRmFB(int fd, unsigned int bufferId);
int drmIoctl(int fd, unsigned long request, void *arg);
"#;

const HEADER_WAYLAND: &str = r#"
typedef struct wl_display wl_display;
typedef struct wl_registry wl_registry;
typedef struct wl_compositor wl_compositor;
typedef struct wl_surface wl_surface;
typedef struct wl_shell wl_shell;
typedef struct wl_shell_surface wl_shell_surface;
typedef struct wl_seat wl_seat;
typedef struct wl_keyboard wl_keyboard;
typedef struct wl_pointer wl_pointer;
typedef struct wl_shm wl_shm;
typedef struct wl_shm_pool wl_shm_pool;
typedef struct wl_buffer wl_buffer;
typedef struct wl_callback wl_callback;
typedef struct wl_output wl_output;
typedef struct wl_event_queue wl_event_queue;

wl_display *wl_display_connect(const char *name);
void wl_display_disconnect(wl_display *display);
int wl_display_get_fd(wl_display *display);
int wl_display_dispatch(wl_display *display);
int wl_display_dispatch_pending(wl_display *display);
int wl_display_roundtrip(wl_display *display);
int wl_display_flush(wl_display *display);
wl_registry *wl_display_get_registry(wl_display *display);
int wl_registry_add_listener(wl_registry *registry, const void *listener, void *data);
void *wl_registry_bind(wl_registry *registry, unsigned int name, const void *interface, unsigned int version);
wl_surface *wl_compositor_create_surface(wl_compositor *compositor);
void wl_surface_destroy(wl_surface *surface);
void wl_surface_attach(wl_surface *surface, wl_buffer *buffer, int x, int y);
void wl_surface_damage(wl_surface *surface, int x, int y, int width, int height);
void wl_surface_commit(wl_surface *surface);
wl_callback *wl_surface_frame(wl_surface *surface);
wl_shm_pool *wl_shm_create_pool(wl_shm *shm, int fd, int size);
wl_buffer *wl_shm_pool_create_buffer(wl_shm_pool *pool, int offset, int width, int height, int stride, unsigned int format);
void wl_shm_pool_destroy(wl_shm_pool *pool);
void wl_buffer_destroy(wl_buffer *buffer);
"#;

// ================================================================
//  Fuentes/Texto Headers
// ================================================================

const HEADER_FREETYPE: &str = r#"
typedef struct FT_LibraryRec_ *FT_Library;
typedef struct FT_FaceRec_ *FT_Face;
typedef int FT_Error;
typedef long FT_Long;
typedef unsigned long FT_ULong;
typedef int FT_Int;
typedef unsigned int FT_UInt;
typedef short FT_Short;
typedef unsigned short FT_UShort;
typedef signed long FT_Fixed;
typedef signed long FT_Pos;

typedef struct FT_Vector_ {
    FT_Pos x;
    FT_Pos y;
} FT_Vector;

typedef struct FT_BBox_ {
    FT_Pos xMin;
    FT_Pos yMin;
    FT_Pos xMax;
    FT_Pos yMax;
} FT_BBox;

typedef struct FT_Bitmap_ {
    unsigned int rows;
    unsigned int width;
    int pitch;
    unsigned char *buffer;
    unsigned short num_grays;
    unsigned char pixel_mode;
} FT_Bitmap;

typedef struct FT_GlyphSlotRec_ {
    FT_Library library;
    FT_Face face;
    FT_Vector advance;
    int format;
    FT_Bitmap bitmap;
    int bitmap_left;
    int bitmap_top;
} *FT_GlyphSlot;

FT_Error FT_Init_FreeType(FT_Library *alibrary);
FT_Error FT_Done_FreeType(FT_Library library);
FT_Error FT_New_Face(FT_Library library, const char *filepathname, FT_Long face_index, FT_Face *aface);
FT_Error FT_New_Memory_Face(FT_Library library, const unsigned char *file_base, FT_Long file_size, FT_Long face_index, FT_Face *aface);
FT_Error FT_Done_Face(FT_Face face);
FT_Error FT_Set_Pixel_Sizes(FT_Face face, FT_UInt pixel_width, FT_UInt pixel_height);
FT_Error FT_Set_Char_Size(FT_Face face, FT_Long char_width, FT_Long char_height, FT_UInt horz_resolution, FT_UInt vert_resolution);
FT_UInt FT_Get_Char_Index(FT_Face face, FT_ULong charcode);
FT_Error FT_Load_Glyph(FT_Face face, FT_UInt glyph_index, int load_flags);
FT_Error FT_Load_Char(FT_Face face, FT_ULong char_code, int load_flags);
FT_Error FT_Render_Glyph(FT_GlyphSlot slot, int render_mode);
"#;

const HEADER_HARFBUZZ: &str = r#"
typedef struct hb_buffer_t hb_buffer_t;
typedef struct hb_font_t hb_font_t;
typedef struct hb_face_t hb_face_t;
typedef struct hb_blob_t hb_blob_t;
typedef unsigned int hb_codepoint_t;
typedef int hb_bool_t;
typedef unsigned int hb_tag_t;
typedef int hb_position_t;
typedef unsigned int hb_mask_t;

typedef struct hb_glyph_info_t {
    hb_codepoint_t codepoint;
    hb_mask_t mask;
    unsigned int cluster;
} hb_glyph_info_t;

typedef struct hb_glyph_position_t {
    hb_position_t x_advance;
    hb_position_t y_advance;
    hb_position_t x_offset;
    hb_position_t y_offset;
} hb_glyph_position_t;

hb_buffer_t *hb_buffer_create(void);
void hb_buffer_destroy(hb_buffer_t *buffer);
void hb_buffer_add_utf8(hb_buffer_t *buffer, const char *text, int text_length, unsigned int item_offset, int item_length);
void hb_buffer_set_direction(hb_buffer_t *buffer, int direction);
void hb_buffer_set_script(hb_buffer_t *buffer, int script);
void hb_buffer_set_language(hb_buffer_t *buffer, void *language);
void hb_buffer_guess_segment_properties(hb_buffer_t *buffer);
unsigned int hb_buffer_get_length(hb_buffer_t *buffer);
hb_glyph_info_t *hb_buffer_get_glyph_infos(hb_buffer_t *buffer, unsigned int *length);
hb_glyph_position_t *hb_buffer_get_glyph_positions(hb_buffer_t *buffer, unsigned int *length);
void hb_shape(hb_font_t *font, hb_buffer_t *buffer, const void *features, unsigned int num_features);
hb_blob_t *hb_blob_create(const char *data, unsigned int length, int mode, void *user_data, void *destroy);
hb_face_t *hb_face_create(hb_blob_t *blob, unsigned int index);
void hb_face_destroy(hb_face_t *face);
hb_font_t *hb_font_create(hb_face_t *face);
void hb_font_destroy(hb_font_t *font);
void hb_font_set_scale(hb_font_t *font, int x_scale, int y_scale);
"#;

// ================================================================
//  Base de datos Headers
// ================================================================

const HEADER_SQLITE3: &str = r#"
typedef struct sqlite3 sqlite3;
typedef struct sqlite3_stmt sqlite3_stmt;
typedef long long sqlite3_int64;
typedef unsigned long long sqlite3_uint64;
typedef void (*sqlite3_destructor_type)(void *);

int sqlite3_open(const char *filename, sqlite3 **ppDb);
int sqlite3_open_v2(const char *filename, sqlite3 **ppDb, int flags, const char *zVfs);
int sqlite3_close(sqlite3 *db);
int sqlite3_close_v2(sqlite3 *db);
int sqlite3_exec(sqlite3 *db, const char *sql, int (*callback)(void *, int, char **, char **), void *arg, char **errmsg);
int sqlite3_prepare_v2(sqlite3 *db, const char *zSql, int nByte, sqlite3_stmt **ppStmt, const char **pzTail);
int sqlite3_step(sqlite3_stmt *pStmt);
int sqlite3_finalize(sqlite3_stmt *pStmt);
int sqlite3_reset(sqlite3_stmt *pStmt);
int sqlite3_bind_int(sqlite3_stmt *pStmt, int i, int iValue);
int sqlite3_bind_int64(sqlite3_stmt *pStmt, int i, sqlite3_int64 iValue);
int sqlite3_bind_double(sqlite3_stmt *pStmt, int i, double rValue);
int sqlite3_bind_text(sqlite3_stmt *pStmt, int i, const char *zData, int nData, sqlite3_destructor_type xDel);
int sqlite3_bind_blob(sqlite3_stmt *pStmt, int i, const void *zData, int nData, sqlite3_destructor_type xDel);
int sqlite3_bind_null(sqlite3_stmt *pStmt, int i);
int sqlite3_column_count(sqlite3_stmt *pStmt);
int sqlite3_column_type(sqlite3_stmt *pStmt, int iCol);
int sqlite3_column_int(sqlite3_stmt *pStmt, int iCol);
sqlite3_int64 sqlite3_column_int64(sqlite3_stmt *pStmt, int iCol);
double sqlite3_column_double(sqlite3_stmt *pStmt, int iCol);
const unsigned char *sqlite3_column_text(sqlite3_stmt *pStmt, int iCol);
const void *sqlite3_column_blob(sqlite3_stmt *pStmt, int iCol);
int sqlite3_column_bytes(sqlite3_stmt *pStmt, int iCol);
const char *sqlite3_column_name(sqlite3_stmt *pStmt, int N);
int sqlite3_changes(sqlite3 *db);
sqlite3_int64 sqlite3_last_insert_rowid(sqlite3 *db);
const char *sqlite3_errmsg(sqlite3 *db);
int sqlite3_errcode(sqlite3 *db);
void sqlite3_free(void *ptr);
const char *sqlite3_libversion(void);
int sqlite3_libversion_number(void);
"#;

// ================================================================
//  Red / Security Headers
// ================================================================

const HEADER_CURL: &str = r#"
typedef void CURL;
typedef void CURLM;
typedef int CURLcode;
typedef int CURLMcode;
typedef int CURLoption;
typedef int CURLINFO;

typedef struct curl_slist {
    char *data;
    struct curl_slist *next;
} curl_slist;

CURLcode curl_global_init(long flags);
void curl_global_cleanup(void);
CURL *curl_easy_init(void);
void curl_easy_cleanup(CURL *curl);
CURLcode curl_easy_setopt(CURL *curl, CURLoption option, ...);
CURLcode curl_easy_perform(CURL *curl);
CURLcode curl_easy_getinfo(CURL *curl, CURLINFO info, ...);
void curl_easy_reset(CURL *curl);
CURL *curl_easy_duphandle(CURL *curl);
const char *curl_easy_strerror(CURLcode code);
char *curl_easy_escape(CURL *curl, const char *string, int length);
char *curl_easy_unescape(CURL *curl, const char *string, int length, int *outlength);
void curl_free(void *p);
struct curl_slist *curl_slist_append(struct curl_slist *list, const char *string);
void curl_slist_free_all(struct curl_slist *list);
CURLM *curl_multi_init(void);
CURLMcode curl_multi_cleanup(CURLM *multi);
CURLMcode curl_multi_add_handle(CURLM *multi, CURL *curl);
CURLMcode curl_multi_remove_handle(CURLM *multi, CURL *curl);
CURLMcode curl_multi_perform(CURLM *multi, int *running_handles);
const char *curl_version(void);
"#;

const HEADER_OPENSSL: &str = r#"
typedef struct ssl_st SSL;
typedef struct ssl_ctx_st SSL_CTX;
typedef struct ssl_method_st SSL_METHOD;
typedef struct bio_st BIO;
typedef struct bio_method_st BIO_METHOD;
typedef struct evp_md_st EVP_MD;
typedef struct evp_md_ctx_st EVP_MD_CTX;
typedef struct evp_cipher_st EVP_CIPHER;
typedef struct evp_cipher_ctx_st EVP_CIPHER_CTX;
typedef struct rsa_st RSA;
typedef struct evp_pkey_st EVP_PKEY;
typedef struct x509_st X509;

int SSL_library_init(void);
void SSL_load_error_strings(void);
const SSL_METHOD *TLS_method(void);
const SSL_METHOD *TLS_client_method(void);
const SSL_METHOD *TLS_server_method(void);
SSL_CTX *SSL_CTX_new(const SSL_METHOD *method);
void SSL_CTX_free(SSL_CTX *ctx);
SSL *SSL_new(SSL_CTX *ctx);
void SSL_free(SSL *ssl);
int SSL_set_fd(SSL *ssl, int fd);
int SSL_connect(SSL *ssl);
int SSL_accept(SSL *ssl);
int SSL_read(SSL *ssl, void *buf, int num);
int SSL_write(SSL *ssl, const void *buf, int num);
int SSL_shutdown(SSL *ssl);
int SSL_get_error(const SSL *ssl, int ret);
long SSL_CTX_set_options(SSL_CTX *ctx, long options);
int SSL_CTX_use_certificate_file(SSL_CTX *ctx, const char *file, int type_val);
int SSL_CTX_use_PrivateKey_file(SSL_CTX *ctx, const char *file, int type_val);
int SSL_CTX_check_private_key(const SSL_CTX *ctx);

unsigned char *SHA256(const unsigned char *d, size_t n, unsigned char *md);
unsigned char *SHA512(const unsigned char *d, size_t n, unsigned char *md);
EVP_MD_CTX *EVP_MD_CTX_new(void);
void EVP_MD_CTX_free(EVP_MD_CTX *ctx);
int EVP_DigestInit_ex(EVP_MD_CTX *ctx, const EVP_MD *type_val, void *impl);
int EVP_DigestUpdate(EVP_MD_CTX *ctx, const void *d, size_t cnt);
int EVP_DigestFinal_ex(EVP_MD_CTX *ctx, unsigned char *md, unsigned int *s);
const EVP_MD *EVP_sha256(void);
const EVP_MD *EVP_sha512(void);
"#;

// ================================================================
//  Input / Hardware Headers
// ================================================================

const HEADER_LIBINPUT: &str = r#"
typedef struct libinput libinput;
typedef struct libinput_device libinput_device;
typedef struct libinput_event libinput_event;
typedef struct libinput_event_pointer libinput_event_pointer;
typedef struct libinput_event_keyboard libinput_event_keyboard;
typedef struct libinput_event_touch libinput_event_touch;
typedef struct libinput_seat libinput_seat;

typedef struct libinput_interface {
    int (*open_restricted)(const char *path, int flags, void *user_data);
    void (*close_restricted)(int fd, void *user_data);
} libinput_interface;

libinput *libinput_udev_create_context(const libinput_interface *interface, void *user_data, void *udev);
libinput *libinput_path_create_context(const libinput_interface *interface, void *user_data);
libinput_device *libinput_path_add_device(libinput *libinput, const char *path);
void libinput_path_remove_device(libinput_device *device);
void libinput_unref(libinput *libinput);
int libinput_get_fd(libinput *libinput);
int libinput_dispatch(libinput *libinput);
libinput_event *libinput_get_event(libinput *libinput);
int libinput_event_get_type(libinput_event *event);
void libinput_event_destroy(libinput_event *event);
libinput_device *libinput_event_get_device(libinput_event *event);
libinput_event_pointer *libinput_event_get_pointer_event(libinput_event *event);
libinput_event_keyboard *libinput_event_get_keyboard_event(libinput_event *event);
double libinput_event_pointer_get_dx(libinput_event_pointer *event);
double libinput_event_pointer_get_dy(libinput_event_pointer *event);
unsigned int libinput_event_pointer_get_button(libinput_event_pointer *event);
unsigned int libinput_event_keyboard_get_key(libinput_event_keyboard *event);
int libinput_event_keyboard_get_key_state(libinput_event_keyboard *event);
"#;

const HEADER_XKBCOMMON: &str = r#"
typedef struct xkb_context xkb_context;
typedef struct xkb_keymap xkb_keymap;
typedef struct xkb_state xkb_state;
typedef unsigned int xkb_keycode_t;
typedef unsigned int xkb_keysym_t;
typedef unsigned int xkb_layout_index_t;
typedef unsigned int xkb_mod_index_t;

xkb_context *xkb_context_new(int flags);
void xkb_context_unref(xkb_context *context);
xkb_keymap *xkb_keymap_new_from_string(xkb_context *context, const char *string, int format, int flags);
xkb_keymap *xkb_keymap_new_from_names(xkb_context *context, const void *names, int flags);
void xkb_keymap_unref(xkb_keymap *keymap);
xkb_state *xkb_state_new(xkb_keymap *keymap);
void xkb_state_unref(xkb_state *state);
xkb_keysym_t xkb_state_key_get_one_sym(xkb_state *state, xkb_keycode_t key);
int xkb_state_key_get_utf8(xkb_state *state, xkb_keycode_t key, char *buffer, size_t size);
unsigned int xkb_state_key_get_utf32(xkb_state *state, xkb_keycode_t key);
int xkb_state_update_key(xkb_state *state, xkb_keycode_t key, int direction);
int xkb_state_mod_name_is_active(xkb_state *state, const char *name, int type_val);
xkb_keysym_t xkb_keysym_from_name(const char *name, int flags);
int xkb_keysym_get_name(xkb_keysym_t keysym, char *buffer, size_t size);
"#;

const HEADER_LIBUDEV: &str = r#"
typedef struct udev udev;
typedef struct udev_device udev_device;
typedef struct udev_enumerate udev_enumerate;
typedef struct udev_monitor udev_monitor;
typedef struct udev_list_entry udev_list_entry;

udev *udev_new(void);
udev *udev_ref(udev *udev);
udev *udev_unref(udev *udev);
udev_device *udev_device_new_from_syspath(udev *udev, const char *syspath);
udev_device *udev_device_new_from_devnum(udev *udev, char type_val, dev_t devnum);
udev_device *udev_device_new_from_subsystem_sysname(udev *udev, const char *subsystem, const char *sysname);
udev_device *udev_device_ref(udev_device *udev_device);
udev_device *udev_device_unref(udev_device *udev_device);
const char *udev_device_get_devpath(udev_device *udev_device);
const char *udev_device_get_subsystem(udev_device *udev_device);
const char *udev_device_get_devtype(udev_device *udev_device);
const char *udev_device_get_syspath(udev_device *udev_device);
const char *udev_device_get_sysname(udev_device *udev_device);
const char *udev_device_get_devnode(udev_device *udev_device);
const char *udev_device_get_property_value(udev_device *udev_device, const char *key);
const char *udev_device_get_sysattr_value(udev_device *udev_device, const char *sysattr);
udev_device *udev_device_get_parent(udev_device *udev_device);
udev_enumerate *udev_enumerate_new(udev *udev);
udev_enumerate *udev_enumerate_unref(udev_enumerate *udev_enumerate);
int udev_enumerate_add_match_subsystem(udev_enumerate *udev_enumerate, const char *subsystem);
int udev_enumerate_scan_devices(udev_enumerate *udev_enumerate);
udev_list_entry *udev_enumerate_get_list_entry(udev_enumerate *udev_enumerate);
udev_list_entry *udev_list_entry_get_next(udev_list_entry *list_entry);
const char *udev_list_entry_get_name(udev_list_entry *list_entry);
udev_monitor *udev_monitor_new_from_netlink(udev *udev, const char *name);
udev_monitor *udev_monitor_unref(udev_monitor *udev_monitor);
int udev_monitor_filter_add_match_subsystem_devtype(udev_monitor *udev_monitor, const char *subsystem, const char *devtype);
int udev_monitor_enable_receiving(udev_monitor *udev_monitor);
int udev_monitor_get_fd(udev_monitor *udev_monitor);
udev_device *udev_monitor_receive_device(udev_monitor *udev_monitor);
"#;

const HEADER_LIBUSB: &str = r#"
typedef struct libusb_context libusb_context;
typedef struct libusb_device libusb_device;
typedef struct libusb_device_handle libusb_device_handle;

typedef struct libusb_device_descriptor {
    unsigned char bLength;
    unsigned char bDescriptorType;
    unsigned short bcdUSB;
    unsigned char bDeviceClass;
    unsigned char bDeviceSubClass;
    unsigned char bDeviceProtocol;
    unsigned char bMaxPacketSize0;
    unsigned short idVendor;
    unsigned short idProduct;
    unsigned short bcdDevice;
    unsigned char iManufacturer;
    unsigned char iProduct;
    unsigned char iSerialNumber;
    unsigned char bNumConfigurations;
} libusb_device_descriptor;

int libusb_init(libusb_context **ctx);
void libusb_exit(libusb_context *ctx);
ssize_t libusb_get_device_list(libusb_context *ctx, libusb_device ***list);
void libusb_free_device_list(libusb_device **list, int unref_devices);
int libusb_get_device_descriptor(libusb_device *dev, libusb_device_descriptor *desc);
int libusb_open(libusb_device *dev, libusb_device_handle **dev_handle);
void libusb_close(libusb_device_handle *dev_handle);
libusb_device_handle *libusb_open_device_with_vid_pid(libusb_context *ctx, unsigned short vendor_id, unsigned short product_id);
int libusb_claim_interface(libusb_device_handle *dev_handle, int interface_number);
int libusb_release_interface(libusb_device_handle *dev_handle, int interface_number);
int libusb_bulk_transfer(libusb_device_handle *dev_handle, unsigned char endpoint, unsigned char *data, int length, int *actual_length, unsigned int timeout);
int libusb_control_transfer(libusb_device_handle *dev_handle, unsigned char request_type, unsigned char bRequest, unsigned short wValue, unsigned short wIndex, unsigned char *data, unsigned short wLength, unsigned int timeout);
int libusb_interrupt_transfer(libusb_device_handle *dev_handle, unsigned char endpoint, unsigned char *data, int length, int *actual_length, unsigned int timeout);
const char *libusb_strerror(int errcode);
"#;

// ================================================================
//  Multimedia / FFmpeg Headers
// ================================================================

const HEADER_AVCODEC: &str = r#"
typedef struct AVCodec AVCodec;
typedef struct AVCodecContext AVCodecContext;
typedef struct AVPacket AVPacket;
typedef struct AVFrame AVFrame;
typedef struct AVCodecParameters AVCodecParameters;
typedef int AVCodecID;
typedef int AVPixelFormat;
typedef int AVSampleFormat;

const AVCodec *avcodec_find_decoder(AVCodecID id);
const AVCodec *avcodec_find_encoder(AVCodecID id);
const AVCodec *avcodec_find_decoder_by_name(const char *name);
const AVCodec *avcodec_find_encoder_by_name(const char *name);
AVCodecContext *avcodec_alloc_context3(const AVCodec *codec);
void avcodec_free_context(AVCodecContext **avctx);
int avcodec_open2(AVCodecContext *avctx, const AVCodec *codec, void **options);
int avcodec_close(AVCodecContext *avctx);
int avcodec_send_packet(AVCodecContext *avctx, const AVPacket *avpkt);
int avcodec_receive_frame(AVCodecContext *avctx, AVFrame *frame);
int avcodec_send_frame(AVCodecContext *avctx, const AVFrame *frame);
int avcodec_receive_packet(AVCodecContext *avctx, AVPacket *avpkt);
int avcodec_parameters_to_context(AVCodecContext *codec, const AVCodecParameters *par);
int avcodec_parameters_from_context(AVCodecParameters *par, const AVCodecContext *codec);
AVPacket *av_packet_alloc(void);
void av_packet_free(AVPacket **pkt);
void av_packet_unref(AVPacket *pkt);
AVFrame *av_frame_alloc(void);
void av_frame_free(AVFrame **frame);
void av_frame_unref(AVFrame *frame);
unsigned avcodec_version(void);
const char *avcodec_configuration(void);
"#;

const HEADER_AVFORMAT: &str = r#"
typedef struct AVFormatContext AVFormatContext;
typedef struct AVStream AVStream;
typedef struct AVOutputFormat AVOutputFormat;
typedef struct AVInputFormat AVInputFormat;
typedef struct AVIOContext AVIOContext;

AVFormatContext *avformat_alloc_context(void);
void avformat_free_context(AVFormatContext *s);
int avformat_open_input(AVFormatContext **ps, const char *url, const AVInputFormat *fmt, void **options);
void avformat_close_input(AVFormatContext **s);
int avformat_find_stream_info(AVFormatContext *ic, void **options);
int av_find_best_stream(AVFormatContext *ic, int type_val, int wanted_stream_nb, int related_stream, const void **decoder_ret, int flags);
int av_read_frame(AVFormatContext *s, void *pkt);
int av_seek_frame(AVFormatContext *s, int stream_index, long timestamp, int flags);
int avformat_write_header(AVFormatContext *s, void **options);
int av_write_frame(AVFormatContext *s, void *pkt);
int av_interleaved_write_frame(AVFormatContext *s, void *pkt);
int av_write_trailer(AVFormatContext *s);
const AVOutputFormat *av_guess_format(const char *short_name, const char *filename, const char *mime_type);
AVStream *avformat_new_stream(AVFormatContext *s, const void *c);
void av_dump_format(AVFormatContext *ic, int index, const char *url, int is_output);
unsigned avformat_version(void);
"#;

const HEADER_AVUTIL: &str = r#"
void *av_malloc(size_t size);
void *av_mallocz(size_t size);
void *av_realloc(void *ptr, size_t size);
void av_free(void *ptr);
void av_freep(void *ptr);
char *av_strdup(const char *s);
int av_log2(unsigned int v);
int av_strerror(int errnum, char *errbuf, size_t errbuf_size);
long av_rescale_q(long a, void *bq, void *cq);
long av_gettime(void);
long av_gettime_relative(void);
unsigned avutil_version(void);
const char *avutil_configuration(void);
const char *av_get_pix_fmt_name(int pix_fmt);
int av_get_bytes_per_sample(int sample_fmt);
const char *av_get_sample_fmt_name(int sample_fmt);
int av_samples_get_buffer_size(int *linesize, int nb_channels, int nb_samples, int sample_fmt, int align);
int av_image_get_buffer_size(int pix_fmt, int width, int height, int align);
int av_image_fill_arrays(unsigned char *dst_data[4], int dst_linesize[4], const unsigned char *src, int pix_fmt, int width, int height, int align);
"#;

const HEADER_SWSCALE: &str = r#"
typedef struct SwsContext SwsContext;

struct SwsContext *sws_getContext(int srcW, int srcH, int srcFormat, int dstW, int dstH, int dstFormat, int flags, void *srcFilter, void *dstFilter, const double *param);
void sws_freeContext(struct SwsContext *swsContext);
int sws_scale(struct SwsContext *c, const unsigned char *const srcSlice[], const int srcStride[], int srcSliceY, int srcSliceH, unsigned char *const dst[], const int dstStride[]);
unsigned swscale_version(void);
const char *swscale_configuration(void);
"#;

// ================================================================
//  XML/JSON Headers
// ================================================================

const HEADER_EXPAT: &str = r#"
typedef struct XML_ParserStruct *XML_Parser;
typedef char XML_Char;
typedef char XML_LChar;
typedef int XML_Bool;
typedef unsigned char XML_Index;
typedef unsigned long XML_Size;
typedef int XML_Status;
typedef int XML_Error;

typedef void (*XML_StartElementHandler)(void *userData, const XML_Char *name, const XML_Char **atts);
typedef void (*XML_EndElementHandler)(void *userData, const XML_Char *name);
typedef void (*XML_CharacterDataHandler)(void *userData, const XML_Char *s, int len);

XML_Parser XML_ParserCreate(const XML_Char *encoding);
XML_Parser XML_ParserCreateNS(const XML_Char *encoding, XML_Char namespaceSeparator);
void XML_ParserFree(XML_Parser parser);
XML_Status XML_Parse(XML_Parser parser, const char *s, int len, int isFinal);
void XML_SetUserData(XML_Parser parser, void *userData);
void XML_SetElementHandler(XML_Parser parser, XML_StartElementHandler start, XML_EndElementHandler end);
void XML_SetCharacterDataHandler(XML_Parser parser, XML_CharacterDataHandler handler);
XML_Error XML_GetErrorCode(XML_Parser parser);
const XML_LChar *XML_ErrorString(XML_Error code);
long XML_GetCurrentLineNumber(XML_Parser parser);
long XML_GetCurrentColumnNumber(XML_Parser parser);
int XML_GetCurrentByteIndex(XML_Parser parser);
XML_Bool XML_ParserReset(XML_Parser parser, const XML_Char *encoding);
void XML_SetEncoding(XML_Parser parser, const XML_Char *encoding);
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_prologue_exists() {
        assert!(!COMMON_PROLOGUE.is_empty());
        assert!(COMMON_PROLOGUE.contains("size_t"));
        assert!(COMMON_PROLOGUE.contains("uint8_t"));
    }

    #[test]
    fn test_get_known_headers() {
        assert!(get_header("stdio.h").is_some());
        assert!(get_header("stdlib.h").is_some());
        assert!(get_header("string.h").is_some());
        assert!(get_header("math.h").is_some());
        assert!(get_header("pthread.h").is_some());
        assert!(get_header("vulkan/vulkan.h").is_some());
        assert!(get_header("sqlite3.h").is_some());
        assert!(get_header("curl/curl.h").is_some());
        assert!(get_header("zlib.h").is_some());
        assert!(get_header("png.h").is_some());
    }

    #[test]
    fn test_unknown_header_returns_none() {
        assert!(get_header("imaginary.h").is_none());
        assert!(get_header("not_a_real_header.h").is_none());
    }

    #[test]
    fn test_stdio_has_printf() {
        let stdio = get_header("stdio.h").unwrap();
        assert!(stdio.contains("printf"));
        assert!(stdio.contains("fprintf"));
        assert!(stdio.contains("fopen"));
        assert!(stdio.contains("fclose"));
        assert!(stdio.contains("fread"));
        assert!(stdio.contains("fwrite"));
    }

    #[test]
    fn test_stdlib_has_malloc() {
        let stdlib = get_header("stdlib.h").unwrap();
        assert!(stdlib.contains("malloc"));
        assert!(stdlib.contains("free"));
        assert!(stdlib.contains("calloc"));
        assert!(stdlib.contains("realloc"));
        assert!(stdlib.contains("exit"));
        assert!(stdlib.contains("atoi"));
    }

    #[test]
    fn test_string_has_memcpy() {
        let string = get_header("string.h").unwrap();
        assert!(string.contains("memcpy"));
        assert!(string.contains("memset"));
        assert!(string.contains("strlen"));
        assert!(string.contains("strcmp"));
        assert!(string.contains("strcpy"));
        assert!(string.contains("strdup"));
    }

    #[test]
    fn test_math_has_trig() {
        let math = get_header("math.h").unwrap();
        assert!(math.contains("sin"));
        assert!(math.contains("cos"));
        assert!(math.contains("sqrt"));
        assert!(math.contains("pow"));
        assert!(math.contains("log"));
        assert!(math.contains("floor"));
        assert!(math.contains("ceil"));
    }

    #[test]
    fn test_vulkan_has_core() {
        let vk = get_header("vulkan/vulkan.h").unwrap();
        assert!(vk.contains("VkInstance"));
        assert!(vk.contains("VkDevice"));
        assert!(vk.contains("vkCreateInstance"));
        assert!(vk.contains("vkCmdDraw"));
        assert!(vk.contains("VkPipeline"));
    }

    #[test]
    fn test_pthread_has_threading() {
        let pt = get_header("pthread.h").unwrap();
        assert!(pt.contains("pthread_create"));
        assert!(pt.contains("pthread_mutex_lock"));
        assert!(pt.contains("pthread_cond_wait"));
        assert!(pt.contains("pthread_rwlock"));
        assert!(pt.contains("pthread_barrier"));
    }

    #[test]
    fn test_all_fastos_categories() {
        // Fase 1 — Base: musl types in prologue
        assert!(COMMON_PROLOGUE.contains("size_t"));

        // Fase 2 — Display
        assert!(get_header("vulkan/vulkan.h").is_some());
        assert!(get_header("wayland-client.h").is_some());
        assert!(get_header("ft2build.h").is_some());
        assert!(get_header("libinput.h").is_some());

        // Fase 3 — User basic
        assert!(get_header("png.h").is_some());
        assert!(get_header("jpeglib.h").is_some());
        assert!(get_header("zlib.h").is_some());
        assert!(get_header("curl/curl.h").is_some());
        assert!(get_header("sqlite3.h").is_some());

        // Fase 4 — Multimedia
        assert!(get_header("libavcodec/avcodec.h").is_some());
        assert!(get_header("vorbis/codec.h").is_some());
        assert!(get_header("opus/opus.h").is_some());

        // Compresión
        assert!(get_header("lz4.h").is_some());
        assert!(get_header("zstd.h").is_some());

        // Network
        assert!(get_header("sys/socket.h").is_some());
        assert!(get_header("netinet/in.h").is_some());
        assert!(get_header("sys/epoll.h").is_some());

        // Hardware
        assert!(get_header("libudev.h").is_some());
        assert!(get_header("libusb-1.0/libusb.h").is_some());
    }
}
