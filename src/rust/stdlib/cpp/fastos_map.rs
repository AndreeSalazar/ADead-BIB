// ============================================================
// fastos_map.rs — <map> implementation
// ============================================================
// std::map<K,V> — Red-Black Tree ordered map
// ============================================================

pub const MAP_METHODS: &[&str] = &[
    "operator[]", "at",
    "insert", "emplace", "erase",
    "find", "count", "contains",
    "lower_bound", "upper_bound", "equal_range",
    "size", "empty", "clear",
    "begin", "end", "rbegin", "rend",
    "swap",
];

pub fn is_map_symbol(name: &str) -> bool {
    name == "map" || name == "multimap"
        || name == "unordered_map" || name == "unordered_multimap"
        || MAP_METHODS.contains(&name)
}
