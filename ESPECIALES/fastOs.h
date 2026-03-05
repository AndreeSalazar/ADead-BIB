// ============================================================
// ADead-BIB — ESPECIALES/fastOs.h
// FastOS System Interface — Native ADead-BIB Header
// .exe → .po (FastOS Portable Object) pipeline support
// ============================================================
// FastOS is the ADead-BIB native OS target.
// .po files are FastOS Portable Objects — the analogue of
// ELF or PE for the FastOS kernel.
// This header exposes the FastOS syscall interface, loader
// ABI, and all built-in primitives available on FastOS.
// ============================================================

#ifndef _ADEAD_FASTOS_H
#define _ADEAD_FASTOS_H

// ── FastOS version ───────────────────────────────────────────
#define FASTOS_VERSION_MAJOR    1
#define FASTOS_VERSION_MINOR    0
#define FASTOS_VERSION_PATCH    0
#define FASTOS_VERSION          "1.0.0"
#define FASTOS_ARCH_X86_64      1

// ── Base types ───────────────────────────────────────────────
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;
typedef signed char        i8;
typedef signed short       i16;
typedef signed int         i32;
typedef signed long long   i64;
typedef unsigned long long usize;
typedef signed long long   isize;
typedef unsigned long long ptr_t;
typedef void*              rawptr;
typedef char*              cstr;
typedef const char*        ccstr;

// ── Boolean ──────────────────────────────────────────────────
typedef u8  bool_t;
#define PO_TRUE    1
#define PO_FALSE   0

// ── FastOS .po file format constants ─────────────────────────
#define PO_MAGIC            0x504F4144  // 'POAD' (Portable Object ADead)
#define PO_VERSION          1
#define PO_ARCH_X86_64      0x01
#define PO_TYPE_EXEC        0x01        // Executable
#define PO_TYPE_DLL         0x02        // Shared library
#define PO_TYPE_DRIVER      0x03        // FastOS kernel driver
#define PO_SECTION_CODE     0x01
#define PO_SECTION_DATA     0x02
#define PO_SECTION_BSS      0x03
#define PO_SECTION_RODATA   0x04
#define PO_SECTION_RELOC    0x05
#define PO_SECTION_SYMTAB   0x06
#define PO_SECTION_STRTAB   0x07
#define PO_FLAG_READABLE    0x01
#define PO_FLAG_WRITABLE    0x02
#define PO_FLAG_EXECUTABLE  0x04

// ── FastOS syscall numbers ────────────────────────────────────
#define SYS_EXIT            0x00
#define SYS_WRITE           0x01
#define SYS_READ            0x02
#define SYS_OPEN            0x03
#define SYS_CLOSE           0x04
#define SYS_SEEK            0x05
#define SYS_STAT            0x06
#define SYS_ALLOC           0x07
#define SYS_FREE            0x08
#define SYS_MMAP            0x09
#define SYS_MUNMAP          0x0A
#define SYS_SPAWN           0x0B
#define SYS_WAIT            0x0C
#define SYS_KILL            0x0D
#define SYS_GETPID          0x0E
#define SYS_SLEEP           0x0F
#define SYS_TIME            0x10
#define SYS_YIELD           0x11
#define SYS_SOCKET          0x12
#define SYS_BIND            0x13
#define SYS_LISTEN          0x14
#define SYS_ACCEPT          0x15
#define SYS_CONNECT         0x16
#define SYS_SEND            0x17
#define SYS_RECV            0x18
#define SYS_CLOSESOCK       0x19
#define SYS_MKDIR           0x1A
#define SYS_RMDIR           0x1B
#define SYS_UNLINK          0x1C
#define SYS_RENAME          0x1D
#define SYS_READDIR         0x1E
#define SYS_GETCWD          0x1F
#define SYS_CHDIR           0x20
#define SYS_ENV_GET         0x21
#define SYS_ENV_SET         0x22
#define SYS_THREAD_SPAWN    0x23
#define SYS_THREAD_JOIN     0x24
#define SYS_MUTEX_CREATE    0x25
#define SYS_MUTEX_LOCK      0x26
#define SYS_MUTEX_UNLOCK    0x27
#define SYS_MUTEX_DESTROY   0x28

// ── FastOS file flags ─────────────────────────────────────────
#define FO_READ             0x01
#define FO_WRITE            0x02
#define FO_CREATE           0x04
#define FO_TRUNC            0x08
#define FO_APPEND           0x10
#define FO_EXCL             0x20
#define SEEK_SET            0
#define SEEK_CUR            1
#define SEEK_END            2

// ── FastOS standard file descriptors ─────────────────────────
#define FD_STDIN            0
#define FD_STDOUT           1
#define FD_STDERR           2

// ── FastOS error codes ────────────────────────────────────────
#define E_OK                0
#define E_PERM              1
#define E_NOENT             2
#define E_IO                5
#define E_NOMEM             12
#define E_ACCES             13
#define E_BUSY              16
#define E_EXIST             17
#define E_NODEV             19
#define E_INVAL             22
#define E_RANGE             34
#define E_NOSYS             38
#define E_TIMEOUT           110
#define E_CONNREFUSED       111

// ── .po Header struct (FastOS Portable Object format) ─────────
struct PoHeader {
    u32  magic;          // PO_MAGIC = 0x504F4144
    u8   version;        // PO_VERSION = 1
    u8   arch;           // PO_ARCH_X86_64 = 1
    u8   type;           // PO_TYPE_EXEC / DLL / DRIVER
    u8   reserved;
    u64  entry_point;    // Virtual address of entry
    u64  load_base;      // Preferred load base (0 = ASLR)
    u32  section_count;
    u32  symbol_count;
    u64  sections_offset;
    u64  symbols_offset;
    u64  strings_offset;
    u64  reloc_offset;
    u64  image_size;
    u64  stack_size;
    char name[64];       // Module name
};

struct PoSection {
    char   name[16];
    u32    flags;        // PO_FLAG_*
    u64    virtual_addr;
    u64    file_offset;
    u64    size;
    u32    alignment;
    u32    reserved;
};

struct PoSymbol {
    u32  name_offset;    // Offset into string table
    u64  value;          // Address or value
    u32  section_index;
    u8   binding;        // 0=local, 1=global, 2=weak
    u8   type;           // 0=notype, 1=func, 2=data
    u16  reserved;
};

// ── FastOS syscall interface (low-level) ──────────────────────
// These map directly to the FastOS kernel ABI.
// ADead-BIB emits the appropriate syscall instructions.
extern i64  __fastos_syscall0(u64 number);
extern i64  __fastos_syscall1(u64 number, u64 a1);
extern i64  __fastos_syscall2(u64 number, u64 a1, u64 a2);
extern i64  __fastos_syscall3(u64 number, u64 a1, u64 a2, u64 a3);
extern i64  __fastos_syscall4(u64 number, u64 a1, u64 a2, u64 a3, u64 a4);
extern i64  __fastos_syscall5(u64 number, u64 a1, u64 a2, u64 a3, u64 a4, u64 a5);
extern i64  __fastos_syscall6(u64 number, u64 a1, u64 a2, u64 a3, u64 a4, u64 a5, u64 a6);

// ── FastOS high-level C API ───────────────────────────────────
extern void     fo_exit(i32 code);
extern i64      fo_write(i32 fd, const void* buf, usize len);
extern i64      fo_read(i32 fd, void* buf, usize len);
extern i32      fo_open(ccstr path, u32 flags);
extern i32      fo_close(i32 fd);
extern i64      fo_seek(i32 fd, i64 offset, i32 whence);
extern void*    fo_alloc(usize size);
extern void     fo_free(void* ptr);
extern void*    fo_realloc(void* ptr, usize size);
extern void*    fo_mmap(void* hint, usize size, u32 flags);
extern i32      fo_munmap(void* addr, usize size);
extern i32      fo_spawn(ccstr path, char** argv, char** envp);
extern i32      fo_wait(i32 pid, i32* status);
extern i32      fo_kill(i32 pid, i32 sig);
extern i32      fo_getpid();
extern void     fo_sleep(u32 ms);
extern u64      fo_time();
extern void     fo_yield();
extern i32      fo_mkdir(ccstr path);
extern i32      fo_rmdir(ccstr path);
extern i32      fo_unlink(ccstr path);
extern i32      fo_rename(ccstr old_path, ccstr new_path);
extern i32      fo_getcwd(char* buf, usize size);
extern i32      fo_chdir(ccstr path);
extern ccstr    fo_env_get(ccstr key);
extern i32      fo_env_set(ccstr key, ccstr value);
extern void     fo_puts(ccstr str);
extern void     fo_putc(char c);
extern i32      fo_getc();
extern i64      fo_printf(ccstr fmt, ...);

// ── FastOS thread API ─────────────────────────────────────────
typedef u64 fo_thread_t;
typedef u64 fo_mutex_t;
typedef void* (*fo_thread_fn)(void*);
extern i32      fo_thread_spawn(fo_thread_t* tid, fo_thread_fn fn, void* arg);
extern i32      fo_thread_join(fo_thread_t tid);
extern fo_mutex_t fo_mutex_create();
extern i32      fo_mutex_lock(fo_mutex_t m);
extern i32      fo_mutex_unlock(fo_mutex_t m);
extern void     fo_mutex_destroy(fo_mutex_t m);

// ── Compiler / ABI hints ──────────────────────────────────────
#define FO_NORETURN         __attribute__((noreturn))
#define FO_PACKED           __attribute__((packed))
#define FO_ALIGNED(n)       __attribute__((aligned(n)))
#define FO_SECTION(s)       __attribute__((section(s)))
#define FO_EXPORT           __attribute__((visibility("default")))
#define FO_WEAK             __attribute__((weak))
#define FO_LIKELY(x)        __builtin_expect(!!(x), 1)
#define FO_UNLIKELY(x)      __builtin_expect(!!(x), 0)

// ── FastOS entry point ────────────────────────────────────────
// ADead-BIB programs targeting FastOS must define:
//   int fo_main(int argc, char** argv);
// The .po loader calls fo_main. Normal 'main' is also supported
// as an alias for compatibility.
#define FASTOS_MAIN         fo_main

#endif // _ADEAD_FASTOS_H
