// ============================================================
// fastos_string_cpp.rs — <string> implementation
// ============================================================
// std::string — Dynamic string with SSO potential
// ============================================================

pub const STRING_METHODS: &[&str] = &[
    "length", "size", "empty", "clear",
    "c_str", "data",
    "operator[]", "at",
    "operator+", "operator+=", "operator==", "operator!=",
    "operator<", "operator>", "operator<=", "operator>=",
    "find", "rfind", "find_first_of", "find_last_of",
    "find_first_not_of", "find_last_not_of",
    "substr", "append", "insert", "erase", "replace",
    "compare", "copy",
    "begin", "end", "rbegin", "rend",
    "front", "back",
    "push_back", "pop_back",
    "resize", "reserve", "capacity",
    "swap",
];

pub const STRING_CONSTANTS: &[(&str, &str)] = &[
    ("npos", "SIZE_MAX"),
];

pub fn is_string_cpp_symbol(name: &str) -> bool {
    name == "string" || name == "basic_string" || name == "wstring"
        || STRING_METHODS.contains(&name)
        || STRING_CONSTANTS.iter().any(|(n, _)| *n == name)
}
