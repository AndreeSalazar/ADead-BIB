// ============================================================
// fastos_threads.rs — <threads.h> implementation (C11)
// ============================================================
// C11 threading API (§7.26)
// Maps to kernel32.dll CreateThread/CriticalSection on Windows
// ============================================================

pub const THREADS_FUNCTIONS: &[&str] = &[
    // Thread management
    "thrd_create", "thrd_equal", "thrd_current",
    "thrd_sleep", "thrd_yield", "thrd_exit", "thrd_detach", "thrd_join",
    // Mutex
    "mtx_init", "mtx_lock", "mtx_timedlock", "mtx_trylock",
    "mtx_unlock", "mtx_destroy",
    // Condition variable
    "cnd_init", "cnd_signal", "cnd_broadcast",
    "cnd_wait", "cnd_timedwait", "cnd_destroy",
    // Thread-specific storage
    "tss_create", "tss_get", "tss_set", "tss_delete",
    // Call once
    "call_once",
];

pub const THREADS_MACROS: &[(&str, &str)] = &[
    ("thrd_success", "0"),
    ("thrd_nomem", "1"),
    ("thrd_timedout", "2"),
    ("thrd_busy", "3"),
    ("thrd_error", "4"),
    ("mtx_plain", "0"),
    ("mtx_recursive", "1"),
    ("mtx_timed", "2"),
    ("ONCE_FLAG_INIT", "{0}"),
    ("TSS_DTOR_ITERATIONS", "4"),
];

pub const THREADS_TYPES: &[&str] = &[
    "thrd_t", "thrd_start_t", "mtx_t", "cnd_t",
    "tss_t", "tss_dtor_t", "once_flag",
];

pub fn is_threads_symbol(name: &str) -> bool {
    THREADS_FUNCTIONS.contains(&name)
        || THREADS_MACROS.iter().any(|(n, _)| *n == name)
        || THREADS_TYPES.contains(&name)
}
