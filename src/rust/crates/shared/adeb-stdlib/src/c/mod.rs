// ============================================================
// ADead-BIB C99 Standard Library
// ============================================================
// Implementaciones propias — Sin libc externa
// Cada módulo implementa un header C99 estándar
// usando syscalls directos o instrucciones x87/SSE2
// ============================================================
// FastOS Kernel Modules (Nuevos v7.1)
//   fastos_kernel → <kernel.h> <fastos.h> — API del kernel
//   fastos_io     → <fastos_io.h>          — I/O x86-64
//   fastos_asm    → __builtin_*, asm volatile, __attribute__
// ============================================================

pub mod fastos_stdio;
pub mod fastos_stdlib;
pub mod fastos_string;
pub mod fastos_math;
pub mod fastos_time;
pub mod fastos_assert;
pub mod fastos_errno;
pub mod fastos_limits;
pub mod fastos_types;
pub mod fastos_ctype;   // ctype.h → isalpha, isdigit, toupper, tolower... (C99 §7.4)
pub mod fastos_signal;  // signal.h → signal, raise, SIGABRT, SIGFPE... (C99 §7.14)
pub mod fastos_wchar;   // wchar.h + wctype.h → wcslen, wcscpy, towupper... (C99 §7.24-25)

// ── FastOS Kernel (ADead-BIB v7.1) ──────────────────────────
pub mod fastos_kernel;  // kernel.h, fastos.h → kprintf, kmalloc, process_t, KERNEL_PANIC...
pub mod fastos_io;      // fastos_io.h        → inb/outb, cli/sti, read_cr3, PIC/PIT
pub mod fastos_asm;     // built-ins          → __builtin_va_list, __attribute__, asm volatile

// ── C11 Headers ─────────────────────────────────────────────
pub mod fastos_setjmp;    // setjmp.h  → setjmp, longjmp, jmp_buf
pub mod fastos_fenv;      // fenv.h    → fegetround, fesetround, feclearexcept... (C99 §7.6)
pub mod fastos_inttypes;  // inttypes.h → PRId64, PRIu32, imaxabs, strtoimax... (C99 §7.8)
pub mod fastos_stdatomic; // stdatomic.h → atomic_int, atomic_load, atomic_store... (C11 §7.17)
pub mod fastos_threads;   // threads.h → thrd_create, mtx_lock, cnd_wait... (C11 §7.26)
pub mod fastos_complex;   // complex.h → cabs, carg, cexp, cpow... (C99 §7.3)
