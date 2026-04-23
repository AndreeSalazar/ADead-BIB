// ============================================================
// fastos_signal.rs — <signal.h> implementation
// ============================================================
// Signal handling via msvcrt.dll IAT imports
// ============================================================

pub const SIGNAL_FUNCTIONS: &[&str] = &[
    "signal", "raise",
];

pub const SIGNAL_MACROS: &[(&str, &str)] = &[
    ("SIG_DFL", "((void(*)(int))0)"),
    ("SIG_IGN", "((void(*)(int))1)"),
    ("SIG_ERR", "((void(*)(int))-1)"),
    ("SIGABRT", "22"),
    ("SIGFPE", "8"),
    ("SIGILL", "4"),
    ("SIGINT", "2"),
    ("SIGSEGV", "11"),
    ("SIGTERM", "15"),
];

pub const SIGNAL_TYPES: &[&str] = &["sig_atomic_t"];

pub fn is_signal_symbol(name: &str) -> bool {
    SIGNAL_FUNCTIONS.contains(&name)
        || SIGNAL_MACROS.iter().any(|(n, _)| *n == name)
        || SIGNAL_TYPES.contains(&name)
}
