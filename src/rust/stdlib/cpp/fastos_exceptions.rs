// ============================================================
// fastos_exceptions.rs — <exception> + <stdexcept> implementation
// ============================================================
// try/catch/throw C++98
// std::exception, std::runtime_error, std::logic_error, etc.
// ============================================================

pub const EXCEPTION_CLASSES: &[&str] = &[
    "exception",
    "bad_exception",
    "bad_alloc",
    "bad_cast",
    "bad_typeid",
    "runtime_error",
    "range_error",
    "overflow_error",
    "underflow_error",
    "logic_error",
    "domain_error",
    "invalid_argument",
    "length_error",
    "out_of_range",
];

pub const EXCEPTION_FUNCTIONS: &[&str] = &[
    "what",
    "current_exception",
    "rethrow_exception",
    "make_exception_ptr",
    "throw_with_nested",
    "rethrow_if_nested",
    "terminate",
    "set_terminate",
    "get_terminate",
    "uncaught_exception",
    "uncaught_exceptions",
];

pub fn is_exception_symbol(name: &str) -> bool {
    EXCEPTION_CLASSES.contains(&name) || EXCEPTION_FUNCTIONS.contains(&name)
}
