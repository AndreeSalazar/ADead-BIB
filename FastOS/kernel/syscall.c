/*
 * FastOS v2.0 — System Call Interface
 * Hybrid syscall design inspired by Windows NT and Linux
 * 
 * Syscall numbers:
 * - 0x000-0x0FF: Process/Thread (Linux-style)
 * - 0x100-0x1FF: Memory (Linux-style)
 * - 0x200-0x2FF: File I/O (POSIX-style)
 * - 0x300-0x3FF: Network (BSD sockets)
 * - 0x400-0x4FF: Graphics/Display (FastOS native)
 * - 0x500-0x5FF: Device I/O (Windows-style)
 * - 0x600-0x6FF: Security/BG (FastOS native)
 * - 0xF00-0xFFF: FastOS extensions
 */

#include "../include/kernel.h"
#include "../include/types.h"

/* ============================================================
 * Syscall Numbers — Hybrid Windows/Linux/FastOS
 * ============================================================ */

/* Process/Thread (Linux-inspired) */
#define SYS_EXIT            0x001
#define SYS_FORK            0x002
#define SYS_EXEC            0x003
#define SYS_WAIT            0x004
#define SYS_GETPID          0x005
#define SYS_GETPPID         0x006
#define SYS_KILL            0x007
#define SYS_YIELD           0x008
#define SYS_SLEEP           0x009
#define SYS_THREAD_CREATE   0x00A
#define SYS_THREAD_EXIT     0x00B
#define SYS_THREAD_JOIN     0x00C

/* Memory (Linux-inspired) */
#define SYS_MMAP            0x100
#define SYS_MUNMAP          0x101
#define SYS_MPROTECT        0x102
#define SYS_BRK             0x103
#define SYS_SBRK            0x104

/* File I/O (POSIX) */
#define SYS_OPEN            0x200
#define SYS_CLOSE           0x201
#define SYS_READ            0x202
#define SYS_WRITE           0x203
#define SYS_LSEEK           0x204
#define SYS_STAT            0x205
#define SYS_FSTAT           0x206
#define SYS_MKDIR           0x207
#define SYS_RMDIR           0x208
#define SYS_UNLINK          0x209
#define SYS_RENAME          0x20A
#define SYS_READDIR         0x20B
#define SYS_GETCWD          0x20C
#define SYS_CHDIR           0x20D
#define SYS_DUP             0x20E
#define SYS_DUP2            0x20F
#define SYS_PIPE            0x210
#define SYS_IOCTL           0x211

/* Network (BSD sockets) */
#define SYS_SOCKET          0x300
#define SYS_BIND            0x301
#define SYS_LISTEN          0x302
#define SYS_ACCEPT          0x303
#define SYS_CONNECT         0x304
#define SYS_SEND            0x305
#define SYS_RECV            0x306
#define SYS_SENDTO          0x307
#define SYS_RECVFROM        0x308
#define SYS_SHUTDOWN        0x309
#define SYS_GETSOCKOPT      0x30A
#define SYS_SETSOCKOPT      0x30B

/* Graphics/Display (FastOS native) */
#define SYS_FB_OPEN         0x400
#define SYS_FB_CLOSE        0x401
#define SYS_FB_MAP          0x402
#define SYS_FB_FLIP         0x403
#define SYS_FB_SETMODE      0x404
#define SYS_FB_GETINFO      0x405
#define SYS_GPU_INIT        0x410
#define SYS_GPU_SUBMIT      0x411
#define SYS_GPU_WAIT        0x412

/* Device I/O (Windows-inspired) */
#define SYS_DEV_OPEN        0x500
#define SYS_DEV_CLOSE       0x501
#define SYS_DEV_READ        0x502
#define SYS_DEV_WRITE       0x503
#define SYS_DEV_IOCTL       0x504
#define SYS_DEV_ENUM        0x505

/* Security/BG (FastOS native) */
#define SYS_BG_VERIFY       0x600
#define SYS_BG_GETMAP       0x601
#define SYS_BG_SETPOLICY    0x602
#define SYS_SETUID          0x610
#define SYS_GETUID          0x611
#define SYS_SETGID          0x612
#define SYS_GETGID          0x613

/* FastOS Extensions */
#define SYS_FASTOS_INFO     0xF00
#define SYS_FASTOS_DEBUG    0xF01
#define SYS_FASTOS_PERF     0xF02

/* ============================================================
 * Syscall Handler Type
 * ============================================================ */

typedef int64_t (*syscall_handler_t)(uint64_t arg1, uint64_t arg2, 
                                      uint64_t arg3, uint64_t arg4,
                                      uint64_t arg5, uint64_t arg6);

/* ============================================================
 * Syscall Implementations
 * ============================================================ */

/* Process syscalls */
static int64_t sys_exit(uint64_t code, uint64_t a2, uint64_t a3, 
                        uint64_t a4, uint64_t a5, uint64_t a6) {
    (void)a2; (void)a3; (void)a4; (void)a5; (void)a6;
    kprintf("[SYSCALL] exit(%lld)\n", code);
    /* TODO: Implement process termination */
    return 0;
}

static int64_t sys_getpid(uint64_t a1, uint64_t a2, uint64_t a3,
                          uint64_t a4, uint64_t a5, uint64_t a6) {
    (void)a1; (void)a2; (void)a3; (void)a4; (void)a5; (void)a6;
    /* TODO: Return actual PID */
    return 1;  /* Init process */
}

static int64_t sys_yield(uint64_t a1, uint64_t a2, uint64_t a3,
                         uint64_t a4, uint64_t a5, uint64_t a6) {
    (void)a1; (void)a2; (void)a3; (void)a4; (void)a5; (void)a6;
    /* TODO: Yield to scheduler */
    return 0;
}

/* Memory syscalls */
static int64_t sys_mmap(uint64_t addr, uint64_t length, uint64_t prot,
                        uint64_t flags, uint64_t fd, uint64_t offset) {
    (void)addr; (void)prot; (void)flags; (void)fd; (void)offset;
    kprintf("[SYSCALL] mmap(addr=0x%llX, len=%lld)\n", addr, length);
    /* TODO: Implement memory mapping */
    return -1;
}

static int64_t sys_munmap(uint64_t addr, uint64_t length, uint64_t a3,
                          uint64_t a4, uint64_t a5, uint64_t a6) {
    (void)a3; (void)a4; (void)a5; (void)a6;
    kprintf("[SYSCALL] munmap(addr=0x%llX, len=%lld)\n", addr, length);
    /* TODO: Implement memory unmapping */
    return 0;
}

/* File I/O syscalls */
static int64_t sys_open(uint64_t path, uint64_t flags, uint64_t mode,
                        uint64_t a4, uint64_t a5, uint64_t a6) {
    (void)a4; (void)a5; (void)a6;
    const char *pathname = (const char*)path;
    kprintf("[SYSCALL] open(\"%s\", flags=0x%llX, mode=0x%llX)\n", 
            pathname, flags, mode);
    /* TODO: Implement file open */
    return -1;
}

static int64_t sys_close(uint64_t fd, uint64_t a2, uint64_t a3,
                         uint64_t a4, uint64_t a5, uint64_t a6) {
    (void)a2; (void)a3; (void)a4; (void)a5; (void)a6;
    kprintf("[SYSCALL] close(fd=%lld)\n", fd);
    /* TODO: Implement file close */
    return 0;
}

static int64_t sys_read(uint64_t fd, uint64_t buf, uint64_t count,
                        uint64_t a4, uint64_t a5, uint64_t a6) {
    (void)a4; (void)a5; (void)a6;
    kprintf("[SYSCALL] read(fd=%lld, buf=0x%llX, count=%lld)\n", 
            fd, buf, count);
    /* TODO: Implement file read */
    return -1;
}

static int64_t sys_write(uint64_t fd, uint64_t buf, uint64_t count,
                         uint64_t a4, uint64_t a5, uint64_t a6) {
    (void)a4; (void)a5; (void)a6;
    
    /* Handle stdout/stderr */
    if (fd == 1 || fd == 2) {
        const char *str = (const char*)buf;
        for (uint64_t i = 0; i < count; i++) {
            /* Output to terminal */
            kprintf("%c", str[i]);
        }
        return count;
    }
    
    kprintf("[SYSCALL] write(fd=%lld, buf=0x%llX, count=%lld)\n", 
            fd, buf, count);
    return -1;
}

/* BG Security syscalls */
static int64_t sys_bg_verify(uint64_t binary, uint64_t size, uint64_t level,
                             uint64_t a4, uint64_t a5, uint64_t a6) {
    (void)a4; (void)a5; (void)a6;
    kprintf("[SYSCALL] bg_verify(binary=0x%llX, size=%lld, level=%lld)\n",
            binary, size, level);
    /* TODO: Call BG verification */
    return 0;
}

/* FastOS info syscall */
static int64_t sys_fastos_info(uint64_t info_type, uint64_t buf, uint64_t size,
                               uint64_t a4, uint64_t a5, uint64_t a6) {
    (void)a4; (void)a5; (void)a6;
    
    if (info_type == 0 && buf && size >= 32) {
        /* Return OS name */
        const char *name = "FastOS v2.0 (ADead-BIB)";
        char *dest = (char*)buf;
        int i = 0;
        while (name[i] && i < (int)size - 1) {
            dest[i] = name[i];
            i++;
        }
        dest[i] = '\0';
        return i;
    }
    
    return -1;
}

/* ============================================================
 * Syscall Table
 * ============================================================ */

#define SYSCALL_MAX 0x1000

static syscall_handler_t syscall_table[SYSCALL_MAX] = {
    /* Process */
    [SYS_EXIT]      = sys_exit,
    [SYS_GETPID]    = sys_getpid,
    [SYS_YIELD]     = sys_yield,
    
    /* Memory */
    [SYS_MMAP]      = sys_mmap,
    [SYS_MUNMAP]    = sys_munmap,
    
    /* File I/O */
    [SYS_OPEN]      = sys_open,
    [SYS_CLOSE]     = sys_close,
    [SYS_READ]      = sys_read,
    [SYS_WRITE]     = sys_write,
    
    /* Security */
    [SYS_BG_VERIFY] = sys_bg_verify,
    
    /* FastOS */
    [SYS_FASTOS_INFO] = sys_fastos_info,
};

/* ============================================================
 * Syscall Dispatcher
 * ============================================================ */

int64_t syscall_dispatch(uint64_t num, uint64_t arg1, uint64_t arg2,
                         uint64_t arg3, uint64_t arg4, uint64_t arg5,
                         uint64_t arg6) {
    if (num >= SYSCALL_MAX || syscall_table[num] == NULL) {
        kprintf("[SYSCALL] Unknown syscall: 0x%llX\n", num);
        return -1;
    }
    
    return syscall_table[num](arg1, arg2, arg3, arg4, arg5, arg6);
}

/* ============================================================
 * Syscall Entry Point (called from assembly)
 * 
 * x86-64 syscall convention (Linux-compatible):
 *   RAX = syscall number
 *   RDI = arg1
 *   RSI = arg2
 *   RDX = arg3
 *   R10 = arg4
 *   R8  = arg5
 *   R9  = arg6
 *   Return in RAX
 * ============================================================ */

/* This would be called from the SYSCALL handler in assembly */
int64_t syscall_entry(uint64_t rax, uint64_t rdi, uint64_t rsi,
                      uint64_t rdx, uint64_t r10, uint64_t r8, uint64_t r9) {
    return syscall_dispatch(rax, rdi, rsi, rdx, r10, r8, r9);
}

/* ============================================================
 * Initialize Syscall Interface
 * ============================================================ */

void syscall_init(void) {
    kprintf("[SYSCALL] Initializing syscall interface...\n");
    
    /* Setup MSRs for SYSCALL/SYSRET */
    /* STAR: Ring 0 and Ring 3 segments */
    uint64_t star = ((uint64_t)0x0008 << 32) |  /* Kernel CS */
                    ((uint64_t)0x0010 << 48);   /* User CS */
    wrmsr(0xC0000081, star);
    
    /* LSTAR: Syscall entry point */
    /* wrmsr(0xC0000082, (uint64_t)syscall_entry_asm); */
    
    /* SFMASK: Flags to clear on syscall */
    wrmsr(0xC0000084, 0x200);  /* Clear IF */
    
    kprintf("[SYSCALL] Syscall interface ready\n");
    kprintf("[SYSCALL] Registered handlers:\n");
    kprintf("  - Process: exit, getpid, yield\n");
    kprintf("  - Memory: mmap, munmap\n");
    kprintf("  - File I/O: open, close, read, write\n");
    kprintf("  - Security: bg_verify\n");
    kprintf("  - FastOS: fastos_info\n");
}
