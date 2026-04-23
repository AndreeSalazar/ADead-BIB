// ============================================================
// fastos_setjmp.rs — <setjmp.h> implementation
// ============================================================
// Non-local jumps: setjmp, longjmp, jmp_buf
// Required for error handling in C programs without C++ exceptions
// ============================================================

pub const SETJMP_FUNCTIONS: &[&str] = &[
    "setjmp", "longjmp", "_setjmp", "_longjmp",
];

pub const SETJMP_TYPES: &[&str] = &["jmp_buf", "sigjmp_buf"];

pub fn is_setjmp_symbol(name: &str) -> bool {
    SETJMP_FUNCTIONS.contains(&name)
        || SETJMP_TYPES.contains(&name)
}
