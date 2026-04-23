// ============================================================
// fastos_stdatomic.rs — <stdatomic.h> implementation (C11)
// ============================================================
// Atomic operations for lock-free programming (C11 §7.17)
// Requires x86-64 LOCK prefix and CMPXCHG instructions
// ============================================================

pub const ATOMIC_FUNCTIONS: &[&str] = &[
    "atomic_init",
    "atomic_load", "atomic_load_explicit",
    "atomic_store", "atomic_store_explicit",
    "atomic_exchange", "atomic_exchange_explicit",
    "atomic_compare_exchange_strong", "atomic_compare_exchange_strong_explicit",
    "atomic_compare_exchange_weak", "atomic_compare_exchange_weak_explicit",
    "atomic_fetch_add", "atomic_fetch_add_explicit",
    "atomic_fetch_sub", "atomic_fetch_sub_explicit",
    "atomic_fetch_or", "atomic_fetch_or_explicit",
    "atomic_fetch_and", "atomic_fetch_and_explicit",
    "atomic_fetch_xor", "atomic_fetch_xor_explicit",
    "atomic_flag_test_and_set", "atomic_flag_test_and_set_explicit",
    "atomic_flag_clear", "atomic_flag_clear_explicit",
    "atomic_thread_fence", "atomic_signal_fence",
    "atomic_is_lock_free",
];

pub const ATOMIC_MACROS: &[(&str, &str)] = &[
    ("ATOMIC_BOOL_LOCK_FREE", "2"),
    ("ATOMIC_CHAR_LOCK_FREE", "2"),
    ("ATOMIC_SHORT_LOCK_FREE", "2"),
    ("ATOMIC_INT_LOCK_FREE", "2"),
    ("ATOMIC_LONG_LOCK_FREE", "2"),
    ("ATOMIC_LLONG_LOCK_FREE", "2"),
    ("ATOMIC_POINTER_LOCK_FREE", "2"),
    ("ATOMIC_FLAG_INIT", "{0}"),
    ("ATOMIC_VAR_INIT", "(value)"),
];

pub const ATOMIC_TYPES: &[&str] = &[
    "atomic_bool", "atomic_char", "atomic_schar", "atomic_uchar",
    "atomic_short", "atomic_ushort",
    "atomic_int", "atomic_uint",
    "atomic_long", "atomic_ulong",
    "atomic_llong", "atomic_ullong",
    "atomic_intptr_t", "atomic_uintptr_t",
    "atomic_size_t", "atomic_ptrdiff_t",
    "atomic_intmax_t", "atomic_uintmax_t",
    "atomic_flag",
    "memory_order",
];

pub const MEMORY_ORDER_MACROS: &[(&str, &str)] = &[
    ("memory_order_relaxed", "0"),
    ("memory_order_consume", "1"),
    ("memory_order_acquire", "2"),
    ("memory_order_release", "3"),
    ("memory_order_acq_rel", "4"),
    ("memory_order_seq_cst", "5"),
];

pub fn is_atomic_symbol(name: &str) -> bool {
    ATOMIC_FUNCTIONS.contains(&name)
        || ATOMIC_MACROS.iter().any(|(n, _)| *n == name)
        || ATOMIC_TYPES.contains(&name)
        || MEMORY_ORDER_MACROS.iter().any(|(n, _)| *n == name)
}
