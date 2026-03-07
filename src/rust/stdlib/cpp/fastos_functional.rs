// ============================================================
// fastos_functional.rs — <functional> implementation
// ============================================================
// std::function, std::bind, std::hash
// ============================================================

pub const FUNCTIONAL_TYPES: &[&str] = &[
    "function",
    "hash",
    "less", "greater", "less_equal", "greater_equal",
    "equal_to", "not_equal_to",
    "plus", "minus", "multiplies", "divides", "modulus", "negate",
    "logical_and", "logical_or", "logical_not",
    "bit_and", "bit_or", "bit_xor", "bit_not",
    "reference_wrapper",
];

pub const FUNCTIONAL_FUNCTIONS: &[&str] = &[
    "bind", "ref", "cref",
    "mem_fn", "not_fn",
    "invoke",
];

pub fn is_functional_symbol(name: &str) -> bool {
    FUNCTIONAL_TYPES.contains(&name) || FUNCTIONAL_FUNCTIONS.contains(&name)
}
