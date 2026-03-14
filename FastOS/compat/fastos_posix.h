/*
 * FastOS v2.2 — Compatibility Layer: POSIX Subset
 * compat/fastos_posix.h
 *
 * Traduce las llamadas POSIX/Linux IMPORTANTES a syscalls FastOS nativas.
 * Sin systemd. Sin dbus. Sin X11. Sin netlink. Sin cgroups.
 *
 * App Linux original:
 *   #include <unistd.h>
 *   #include <fcntl.h>
 *   int fd = open("data.bin", O_RDONLY);
 *
 * Con FastOS compat layer:
 *   #include <fastos_posix.h>      // reemplaza unistd.h + fcntl.h
 *   int fd = open("data.bin", O_RDONLY);
 *   // ADead-BIB traduce automáticamente a fs_open()
 *
 * Resultado: binario FastOS puro. Sin glibc. Sin musl.
 *
 * Compilar con ADead-BIB:
 *   adb cc app_posix.c --target fastos -o app.po
 *
 * Autor: Eddi Andreé Salazar Matos — Perú — GPL v2
 * ADead-BIB — Binary Is Binary — Po:506F4F53 — BG:APPROVE
 */

#ifndef _FASTOS_POSIX_H
#define _FASTOS_POSIX_H

#include "fastos_syscall.h"
#include "fastos_stdlib.h"

/* ══════════════════════════════════════════════════════
 * § 1. POSIX Type Definitions
 * ══════════════════════════════════════════════════════ */

typedef int64_t  off_t;
typedef uint32_t mode_t;
typedef uint32_t pid_t;
typedef uint64_t time_t;
typedef int64_t  suseconds_t;

struct timeval {
    time_t      tv_sec;
    suseconds_t tv_usec;
};

struct timezone {
    int tz_minuteswest;
    int tz_dsttime;
};

/* ══════════════════════════════════════════════════════
 * § 2. fcntl.h — File Control Flags
 * ══════════════════════════════════════════════════════ */

#define O_RDONLY    0x0000
#define O_WRONLY    0x0001
#define O_RDWR      0x0002
#define O_CREAT     0x0040
#define O_TRUNC     0x0200
#define O_APPEND    0x0400
#define O_EXCL      0x0080

/* Seek constants */
#define SEEK_SET    0
#define SEEK_CUR    1
#define SEEK_END    2

/* Standard file descriptors */
#define STDIN_FILENO  0
#define STDOUT_FILENO 1
#define STDERR_FILENO 2

/* ══════════════════════════════════════════════════════
 * § 3. unistd.h — File I/O → fs_open / fs_read / fs_write / fs_close
 *
 * CRÍTICO — The most used POSIX calls.
 * ══════════════════════════════════════════════════════ */

/*
 * open → fs_open
 * Maps POSIX O_* flags to FastOS FS_* flags
 */
static inline int open(const char *pathname, int flags, ...) {
    uint32_t fflags = 0;

    if ((flags & O_RDWR) == O_RDWR)       fflags = FS_READWRITE;
    else if (flags & O_WRONLY)             fflags = FS_WRITE;
    else                                   fflags = FS_READ;

    if (flags & O_CREAT)   fflags = fflags | FS_CREATE;
    if (flags & O_TRUNC)   fflags = fflags | FS_TRUNCATE;
    if (flags & O_APPEND)  fflags = fflags | FS_APPEND;

    return fs_open(pathname, fflags);
}

/*
 * read → fs_read
 */
static inline ssize_t read(int fd, void *buf, size_t count) {
    return fs_read(fd, buf, count);
}

/*
 * write → fs_write
 */
static inline ssize_t write(int fd, const void *buf, size_t count) {
    return fs_write(fd, buf, count);
}

/*
 * close → fs_close
 */
static inline int close(int fd) {
    return fs_close(fd);
}

/*
 * lseek → fs_seek
 */
static inline off_t lseek(int fd, off_t offset, int whence) {
    return (off_t)fs_seek(fd, (int64_t)offset, whence);
}

/* ══════════════════════════════════════════════════════
 * § 4. stdlib.h — Memory → mem_alloc / mem_free
 *
 * CRÍTICO — malloc/free are the core memory API.
 * ══════════════════════════════════════════════════════ */

/*
 * malloc → mem_alloc
 */
static inline void *malloc(size_t size) {
    return mem_alloc(size);
}

/*
 * calloc → mem_alloc + mem_zero
 */
static inline void *calloc(size_t nmemb, size_t size) {
    size_t total = nmemb * size;
    void *ptr = mem_alloc(total);
    if (ptr != NULL) {
        mem_zero(ptr, total);
    }
    return ptr;
}

/*
 * realloc → mem_alloc + mem_copy + mem_free
 * Simplified: always allocates new block.
 */
static inline void *realloc(void *ptr, size_t size) {
    if (size == 0) {
        if (ptr != NULL) mem_free(ptr);
        return NULL;
    }
    void *newptr = mem_alloc(size);
    if (newptr != NULL && ptr != NULL) {
        mem_copy(newptr, ptr, size);  /* may read past old size — safe in FastOS heap */
        mem_free(ptr);
    }
    return newptr;
}

/*
 * free → mem_free
 */
static inline void free(void *ptr) {
    if (ptr != NULL) mem_free(ptr);
}

/* ══════════════════════════════════════════════════════
 * § 5. sys/mman.h — Memory Mapping → mem_map / mem_unmap
 *
 * IMPORTANTE — mmap for large allocations.
 * ══════════════════════════════════════════════════════ */

#define PROT_NONE   0x0
#define PROT_READ   0x1
#define PROT_WRITE  0x2
#define PROT_EXEC   0x4

#define MAP_PRIVATE   0x02
#define MAP_ANONYMOUS 0x20
#define MAP_ANON      MAP_ANONYMOUS
#define MAP_FIXED     0x10
#define MAP_FAILED    ((void *)-1)

/*
 * mmap → mem_map
 */
static inline void *mmap(void *addr, size_t length, int prot,
                          int flags, int fd, off_t offset) {
    uint32_t fprot = 0;
    if (prot & PROT_READ)  fprot = fprot | MEM_READ;
    if (prot & PROT_WRITE) fprot = fprot | MEM_WRITE;
    if (prot & PROT_EXEC)  fprot = fprot | MEM_EXEC;

    void *result = mem_map(addr, length, fprot);
    if (result == NULL) return MAP_FAILED;
    return result;
}

/*
 * munmap → mem_unmap
 */
static inline int munmap(void *addr, size_t length) {
    return mem_unmap(addr, length);
}

/* ══════════════════════════════════════════════════════
 * § 6. pthread.h — Threads → proc_spawn
 *
 * IMPORTANTE — pthread_create is the main threading call.
 * ══════════════════════════════════════════════════════ */

typedef uint32_t pthread_t;
typedef struct { int detachstate; } pthread_attr_t;

/*
 * pthread_create → proc_spawn
 */
static inline int pthread_create(
    pthread_t *thread,
    const pthread_attr_t *attr,  /* ignored */
    void *(*start_routine)(void *),
    void *arg                    /* ignored for now */
) {
    int pid = proc_spawn((void (*)(void))start_routine, 65536);
    if (pid < 0) return -1;
    if (thread != NULL) *thread = (pthread_t)pid;
    return 0;
}

/*
 * pthread_exit → proc_exit
 */
static inline void pthread_exit(void *retval) {
    proc_exit(0);
}

/* ══════════════════════════════════════════════════════
 * § 7. time.h — Time → time_get / rdtsc
 *
 * IMPORTANTE — gettimeofday, clock_gettime
 * ══════════════════════════════════════════════════════ */

/*
 * gettimeofday → time_get
 */
static inline int gettimeofday(struct timeval *tv, struct timezone *tz) {
    fastos_time_t ft;
    int ret = time_get(&ft);
    if (ret != FASTOS_OK) return -1;
    if (tv != NULL) {
        tv->tv_sec = (time_t)ft.seconds;
        tv->tv_usec = (suseconds_t)ft.milliseconds * 1000;
    }
    return 0;
}

/*
 * sleep → time_sleep (seconds)
 */
static inline unsigned int sleep(unsigned int seconds) {
    time_sleep(seconds * 1000);
    return 0;
}

/*
 * usleep → time_sleep (microseconds)
 */
static inline int usleep(uint32_t usec) {
    time_sleep(usec / 1000);
    return 0;
}

/* ══════════════════════════════════════════════════════
 * § 8. stdio.h — I/O → VGA + serial
 *
 * ÚTIL — printf familia (minimal implementation)
 * ══════════════════════════════════════════════════════ */

/*
 * puts → serial_print + newline (for kernel/debug use)
 */
static inline int puts(const char *s) {
    serial_print(s);
    __outb(COM1, '\r');
    __outb(COM1, '\n');
    return 0;
}

/*
 * putchar → serial_putchar
 */
static inline int putchar(int c) {
    serial_putchar((char)c);
    return c;
}

/*
 * printf — minimal implementation (format: %s, %d, %x, %c, %%)
 * NOTE: This is a simplified printf. For full printf, use ADead-BIB's
 * built-in implementation when compiling for Windows/Linux targets.
 *
 * In FastOS kernel mode, output goes to COM1 serial.
 */
static inline int printf(const char *fmt, ...) {
    /* Minimal: just print the format string literally for now.
     * Full va_args printf requires ADead-BIB ABI support.
     * Use serial_print() + int_to_str() for formatted output. */
    serial_print(fmt);
    return str_len(fmt);
}

/* ══════════════════════════════════════════════════════
 * § 9. process.h — Process Control
 *
 * ÚTIL — exit, _exit, getpid
 * ══════════════════════════════════════════════════════ */

/*
 * exit → proc_exit
 */
static inline void exit(int status) {
    proc_exit(status);
}

/*
 * _exit → proc_exit (no cleanup)
 */
static inline void _exit(int status) {
    proc_exit(status);
}

/*
 * getpid — returns 1 for now (shell process)
 */
static inline pid_t getpid(void) {
    return 2; /* FASTOS_PID_SHELL */
}

/* ══════════════════════════════════════════════════════
 * § 10. errno.h — Error handling
 * ══════════════════════════════════════════════════════ */

static int errno = 0;

#define ENOENT  2
#define EACCES  13
#define ENOMEM  12
#define EBADF   9
#define EEXIST  17
#define EINVAL  22
#define EIO     5
#define ENOSYS  38

/* ══════════════════════════════════════════════════════
 * § 11. mmap / munmap → heap_alloc/heap_free
 * ══════════════════════════════════════════════════════ */

#define PROT_READ   0x1
#define PROT_WRITE  0x2
#define PROT_EXEC   0x4
#define MAP_PRIVATE 0x02
#define MAP_ANONYMOUS 0x20
#define MAP_ANON    MAP_ANONYMOUS
#define MAP_FAILED  ((void *)-1)

static inline void *mmap(void *addr, size_t length, int prot, int flags,
                          int fd, off_t offset) {
    (void)addr; (void)prot; (void)flags; (void)fd; (void)offset;
    void *p = heap_alloc((uint32_t)length);
    return p ? p : MAP_FAILED;
}

static inline int munmap(void *addr, size_t length) {
    (void)length;
    heap_free(addr);
    return 0;
}

static inline void *sbrk(intptr_t increment) {
    if (increment <= 0) return (void *)-1;
    return heap_alloc((uint32_t)increment);
}

/* ══════════════════════════════════════════════════════
 * § 12. Directory operations (opendir/readdir/closedir)
 * ══════════════════════════════════════════════════════ */

typedef struct {
    int _fd;
    int _pos;
} DIR;

struct dirent {
    unsigned long d_ino;
    char          d_name[256];
};

static inline DIR *opendir(const char *name) {
    int fd = fs_open(name, 0); /* O_RDONLY */
    if (fd < 0) return (DIR *)0;
    DIR *d = (DIR *)heap_alloc(sizeof(DIR));
    if (d) { d->_fd = fd; d->_pos = 0; }
    return d;
}

static inline struct dirent *readdir(DIR *dirp) {
    (void)dirp;
    return (struct dirent *)0; /* Stub: no FS enumeration yet */
}

static inline int closedir(DIR *dirp) {
    if (!dirp) return -1;
    fs_close(dirp->_fd);
    heap_free(dirp);
    return 0;
}

/* ══════════════════════════════════════════════════════
 * § 13. Process stubs
 * ══════════════════════════════════════════════════════ */

static inline pid_t getppid(void) { return 1; /* init */ }

/* fork: not yet — use proc_spawn instead */
static inline pid_t fork(void) {
    errno = ENOSYS;
    return -1; /* FUTURO: proc_spawn() modification */
}

/* exec family: not yet — use Po loader */
static inline int execve(const char *pathname, char *const argv[], char *const envp[]) {
    (void)pathname; (void)argv; (void)envp;
    errno = ENOSYS;
    return -1; /* FUTURO: Po loader */
}

/* ══════════════════════════════════════════════════════
 * § 14. Socket stubs (FUTURO: network stack)
 * ══════════════════════════════════════════════════════ */

typedef unsigned int socklen_t;

struct sockaddr {
    unsigned short sa_family;
    char           sa_data[14];
};

#define AF_INET  2
#define SOCK_STREAM 1
#define SOCK_DGRAM  2

static inline int socket(int domain, int type, int protocol) {
    (void)domain; (void)type; (void)protocol;
    errno = ENOSYS;
    return -1;
}

static inline int bind(int sockfd, const struct sockaddr *addr, socklen_t addrlen) {
    (void)sockfd; (void)addr; (void)addrlen;
    errno = ENOSYS;
    return -1;
}

static inline int listen(int sockfd, int backlog) {
    (void)sockfd; (void)backlog;
    errno = ENOSYS;
    return -1;
}

static inline int accept(int sockfd, struct sockaddr *addr, socklen_t *addrlen) {
    (void)sockfd; (void)addr; (void)addrlen;
    errno = ENOSYS;
    return -1;
}

static inline int connect(int sockfd, const struct sockaddr *addr, socklen_t addrlen) {
    (void)sockfd; (void)addr; (void)addrlen;
    errno = ENOSYS;
    return -1;
}

typedef long ssize_t;

static inline ssize_t send(int sockfd, const void *buf, size_t len, int flags) {
    (void)sockfd; (void)buf; (void)len; (void)flags;
    errno = ENOSYS;
    return -1;
}

static inline ssize_t recv(int sockfd, void *buf, size_t len, int flags) {
    (void)sockfd; (void)buf; (void)len; (void)flags;
    errno = ENOSYS;
    return -1;
}

/* ══════════════════════════════════════════════════════
 * § 15. IGNORADOS — Basura Linux que NO se traduce
 *
 * systemd, dbus, X11, netlink, cgroups → IGNORAR
 * ══════════════════════════════════════════════════════ */

/* systemd/dbus → BLOCKED */
#define sd_notify(...)     FASTOS_EBGDENY
#define dbus_message_new(...) NULL

#endif /* _FASTOS_POSIX_H */
