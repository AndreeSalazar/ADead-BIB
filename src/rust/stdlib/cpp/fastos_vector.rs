// ============================================================
// fastos_vector.rs — <vector> implementation
// ============================================================
// std::vector<T> — Dynamic array with amortized O(1) push_back
// Rule of Three completo (copy ctor/assign/dtor)
// ============================================================

pub const VECTOR_METHODS: &[&str] = &[
    "push_back", "pop_back", "emplace_back",
    "operator[]", "at",
    "size", "capacity", "empty",
    "clear", "resize", "reserve", "shrink_to_fit",
    "begin", "end", "rbegin", "rend",
    "cbegin", "cend",
    "front", "back", "data",
    "insert", "erase", "emplace",
    "assign", "swap",
];

pub fn is_vector_symbol(name: &str) -> bool {
    name == "vector" || VECTOR_METHODS.contains(&name)
}
