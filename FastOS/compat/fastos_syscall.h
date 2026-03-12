/*
 * FastOS v2.2 — Compatibility Layer: Native Syscalls
 * compat/fastos_syscall.h
 *
 * Las syscalls nativas de FastOS. TODO lo demás se traduce a estas.
 * Win32 → aquí. POSIX → aquí. Sin CRT. Sin DLL. Sin libc.
 *
 * Compilar con ADead-BIB:
 *   adb cc app.c --target fastos -o app.po
 *
 * Autor: Eddi Andreé Salazar Matos — Perú — GPL v2
 * ADead-BIB — Binary Is Binary — Po:506F4F53 — BG:APPROVE
 */

#ifndef _FASTOS_SYSCALL_H
#define _FASTOS_SYSCALL_H

#include "../include/types.h"

/* ══════════════════════════════════════════════════════
 * § 1. Po Magic & Constants
 * ══════════════════════════════════════════════════════ */

#define FASTOS_PO_MAGIC     0x506F4F53  /* "PoOS" little-endian */
#define FASTOS_VERSION       0x0202     /* v2.2 */
#define FASTOS_MAX_FD        64         /* max open file descriptors */
#define FASTOS_MAX_PATH      256        /* max path length */
#define FASTOS_PAGE_SIZE     4096       /* memory page size */

/* ══════════════════════════════════════════════════════
 * § 2. Syscall Numbers
 * ══════════════════════════════════════════════════════ */

#define SYSCALL_EXIT          1
#define SYSCALL_READ          3
#define SYSCALL_WRITE         4
#define SYSCALL_OPEN          5
#define SYSCALL_CLOSE         6
#define SYSCALL_MMAP         90
#define SYSCALL_MUNMAP       91
#define SYSCALL_BG_CHECK    201
#define SYSCALL_PO_EXEC     202
#define SYSCALL_PROC_SPAWN  210
#define SYSCALL_PROC_EXIT   211
#define SYSCALL_PROC_YIELD  212
#define SYSCALL_TIME_GET    220
#define SYSCALL_TIME_SLEEP  221

/* ══════════════════════════════════════════════════════
 * § 3. File System Flags
 * ══════════════════════════════════════════════════════ */

#define FS_READ       0x01
#define FS_WRITE      0x02
#define FS_READWRITE  0x03
#define FS_CREATE     0x04
#define FS_TRUNCATE   0x08
#define FS_APPEND     0x10

/* Seek modes */
#define FS_SEEK_SET   0
#define FS_SEEK_CUR   1
#define FS_SEEK_END   2

/* ══════════════════════════════════════════════════════
 * § 4. Error Codes
 * ══════════════════════════════════════════════════════ */

#define FASTOS_OK             0
#define FASTOS_ERROR         -1
#define FASTOS_ENOENT        -2   /* No such file or directory */
#define FASTOS_EACCES        -3   /* Permission denied (BG block) */
#define FASTOS_ENOMEM        -4   /* Out of memory */
#define FASTOS_EBADF         -5   /* Bad file descriptor */
#define FASTOS_EEXIST        -6   /* File exists */
#define FASTOS_EINVAL        -7   /* Invalid argument */
#define FASTOS_EIO           -8   /* I/O error */
#define FASTOS_ENOSYS        -9   /* Syscall not implemented */
#define FASTOS_EBGDENY      -10   /* Binary Guardian denied */

/* ══════════════════════════════════════════════════════
 * § 5. Memory Protection Flags
 * ══════════════════════════════════════════════════════ */

#define MEM_READ      0x01
#define MEM_WRITE     0x02
#define MEM_EXEC      0x04
#define MEM_USER      0x08

/* ══════════════════════════════════════════════════════
 * § 6. Binary Guardian Levels
 * ══════════════════════════════════════════════════════ */

#define BG_LEVEL_REBUILD     1   /* Auto re-build check */
#define BG_LEVEL_FIREWALL    2   /* + Human firewall */
#define BG_LEVEL_PREEXEC     3   /* + Pre-execution scan */
#define BG_LEVEL_DEADMAN     4   /* + Dead Man's Switch */

#define BG_APPROVE           0
#define BG_DENY              1
#define BG_WARN              2

/* ══════════════════════════════════════════════════════
 * § 7. Structures
 * ══════════════════════════════════════════════════════ */

/* File descriptor info */
typedef struct {
    int      fd;
    uint32_t flags;
    uint64_t position;
    uint64_t size;
} fastos_file_t;

/* Process info */
typedef struct {
    uint32_t pid;
    uint32_t state;      /* 1=running, 2=sleeping, 3=zombie, 4=dead */
    uint64_t entry;      /* entry point */
    uint64_t stack;      /* stack base */
    uint32_t stack_size; /* stack size */
} fastos_proc_t;

/* System info (returned by sys_info) */
typedef struct {
    uint32_t cpu_family;
    uint32_t cpu_model;
    uint32_t cpu_cores;
    uint32_t ram_mb;
    uint32_t avx2;       /* 1 if AVX2 available */
    uint32_t sse42;      /* 1 if SSE4.2 available */
    uint32_t aes_ni;     /* 1 if AES-NI available */
    uint32_t bg_level;   /* current BG security level */
    uint64_t tsc_freq;   /* TSC frequency (approx) */
    char     vendor[16]; /* CPU vendor string */
} fastos_sysinfo_t;

/* Time value */
typedef struct {
    uint64_t seconds;
    uint32_t milliseconds;
    uint32_t ticks;      /* PIT ticks since boot */
} fastos_time_t;

/* ══════════════════════════════════════════════════════
 * § 8. Syscall API — File System (Po Format I/O)
 * ══════════════════════════════════════════════════════ */

/*
 * fs_open — Open a file
 * @path:  file path (Po filesystem)
 * @flags: FS_READ, FS_WRITE, FS_CREATE, etc.
 * @return: file descriptor (>= 0) or error code (< 0)
 */
int fs_open(const char *path, uint32_t flags);

/*
 * fs_read — Read from a file
 * @fd:  file descriptor from fs_open
 * @buf: destination buffer
 * @len: number of bytes to read
 * @return: bytes read (>= 0) or error code (< 0)
 */
ssize_t fs_read(int fd, void *buf, size_t len);

/*
 * fs_write — Write to a file
 * @fd:  file descriptor
 * @buf: source buffer
 * @len: number of bytes to write
 * @return: bytes written (>= 0) or error code (< 0)
 */
ssize_t fs_write(int fd, const void *buf, size_t len);

/*
 * fs_close — Close a file descriptor
 * @fd: file descriptor to close
 * @return: FASTOS_OK or error code
 */
int fs_close(int fd);

/*
 * fs_seek — Seek within a file
 * @fd:     file descriptor
 * @offset: byte offset
 * @whence: FS_SEEK_SET, FS_SEEK_CUR, FS_SEEK_END
 * @return: new position or error code
 */
int64_t fs_seek(int fd, int64_t offset, int whence);

/* ══════════════════════════════════════════════════════
 * § 9. Syscall API — Memory
 * ══════════════════════════════════════════════════════ */

/*
 * mem_alloc — Allocate memory from FastOS heap
 * @size: number of bytes
 * @return: pointer to allocated memory, or NULL on failure
 *
 * Uses the kernel heap. No libc malloc. No CRT.
 * BG verifies the calling binary before allocation.
 */
void *mem_alloc(size_t size);

/*
 * mem_free — Free allocated memory
 * @ptr: pointer previously returned by mem_alloc
 */
void mem_free(void *ptr);

/*
 * mem_map — Map memory pages (like mmap)
 * @addr:  hint address (0 = kernel chooses)
 * @size:  bytes to map (rounded up to FASTOS_PAGE_SIZE)
 * @prot:  MEM_READ | MEM_WRITE | MEM_EXEC
 * @return: mapped address or NULL on failure
 */
void *mem_map(void *addr, size_t size, uint32_t prot);

/*
 * mem_unmap — Unmap memory pages
 * @addr: address returned by mem_map
 * @size: size of mapping
 * @return: FASTOS_OK or error code
 */
int mem_unmap(void *addr, size_t size);

/* ══════════════════════════════════════════════════════
 * § 10. Syscall API — Process
 * ══════════════════════════════════════════════════════ */

/*
 * proc_spawn — Create a new process/thread
 * @fn:         entry point function
 * @stack_size: stack size (0 = default 64KB)
 * @return: PID (> 0) or error code (< 0)
 */
int proc_spawn(void (*fn)(void), uint32_t stack_size);

/*
 * proc_exit — Exit the current process
 * @code: exit code
 */
void proc_exit(int code);

/*
 * proc_yield — Yield CPU to scheduler
 */
void proc_yield(void);

/* ══════════════════════════════════════════════════════
 * § 11. Syscall API — Security (Binary Guardian)
 * ══════════════════════════════════════════════════════ */

/*
 * bg_check — Query Binary Guardian status/permission
 * @level: BG_LEVEL_REBUILD through BG_LEVEL_DEADMAN
 * @return: BG_APPROVE, BG_DENY, or BG_WARN
 *
 * Every syscall internally calls bg_check. Apps can also
 * call it directly to verify their own security state.
 */
int bg_check(int level);

/* ══════════════════════════════════════════════════════
 * § 12. Syscall API — Time
 * ══════════════════════════════════════════════════════ */

/*
 * time_get — Get current time
 * @t: pointer to fastos_time_t to fill
 * @return: FASTOS_OK or error code
 */
int time_get(fastos_time_t *t);

/*
 * time_sleep — Sleep for milliseconds
 * @ms: milliseconds to sleep (uses PIT)
 */
void time_sleep(uint32_t ms);

/*
 * rdtsc — Read Time Stamp Counter directly
 * @return: 64-bit TSC value
 * Uses Ryzen 5 5600X invariant TSC for high-precision timing.
 */
uint64_t rdtsc(void);

/* ══════════════════════════════════════════════════════
 * § 13. Syscall API — System Info
 * ══════════════════════════════════════════════════════ */

/*
 * sys_info — Get system information
 * @info: pointer to fastos_sysinfo_t to fill
 * @return: FASTOS_OK or error code
 */
int sys_info(fastos_sysinfo_t *info);

/* ══════════════════════════════════════════════════════
 * § 14. Syscall API — I/O (VGA + Serial)
 * ══════════════════════════════════════════════════════ */

/*
 * vga_putchar — Write a character to VGA text mode
 * @ch:    character (CP437)
 * @attr:  color attribute (high nibble = bg, low = fg)
 * @row:   row (0-24)
 * @col:   column (0-79)
 */
void vga_putchar(char ch, uint8_t attr, int row, int col);

/*
 * serial_putchar — Write a character to COM1 (0x3F8)
 * @ch: character to send
 */
void serial_putchar(char ch);

/* ══════════════════════════════════════════════════════
 * § 15. Inline Syscall Mechanism
 *
 * FastOS uses a simple syscall ABI:
 *   RAX = syscall number
 *   RDI = arg1, RSI = arg2, RDX = arg3, R10 = arg4
 *   Return in RAX
 *
 * For kernel-mode code (current), these are direct calls.
 * For usermode .Po binaries, these go through INT 0x80.
 * ══════════════════════════════════════════════════════ */

#define FASTOS_SYSCALL_ABI  "int $0x80"

static inline int64_t _fastos_syscall0(int nr) {
    int64_t ret;
    __asm__ __volatile__(FASTOS_SYSCALL_ABI
        : "=a"(ret) : "a"(nr) : "memory");
    return ret;
}

static inline int64_t _fastos_syscall1(int nr, int64_t a1) {
    int64_t ret;
    __asm__ __volatile__(FASTOS_SYSCALL_ABI
        : "=a"(ret) : "a"(nr), "D"(a1) : "memory");
    return ret;
}

static inline int64_t _fastos_syscall2(int nr, int64_t a1, int64_t a2) {
    int64_t ret;
    __asm__ __volatile__(FASTOS_SYSCALL_ABI
        : "=a"(ret) : "a"(nr), "D"(a1), "S"(a2) : "memory");
    return ret;
}

static inline int64_t _fastos_syscall3(int nr, int64_t a1, int64_t a2, int64_t a3) {
    int64_t ret;
    __asm__ __volatile__(FASTOS_SYSCALL_ABI
        : "=a"(ret) : "a"(nr), "D"(a1), "S"(a2), "d"(a3) : "memory");
    return ret;
}

#endif /* _FASTOS_SYSCALL_H */
