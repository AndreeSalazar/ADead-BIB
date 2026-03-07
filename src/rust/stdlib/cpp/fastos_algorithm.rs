// ============================================================
// fastos_algorithm.rs — <algorithm> implementation
// ============================================================
// std::sort, find, copy, transform, reverse, etc.
// ============================================================

pub const ALGORITHM_FUNCTIONS: &[&str] = &[
    "sort", "stable_sort", "partial_sort", "nth_element",
    "find", "find_if", "find_if_not",
    "count", "count_if",
    "copy", "copy_if", "copy_n", "copy_backward",
    "move", "move_backward",
    "fill", "fill_n",
    "transform",
    "replace", "replace_if", "replace_copy", "replace_copy_if",
    "swap", "swap_ranges", "iter_swap",
    "reverse", "reverse_copy",
    "rotate", "rotate_copy",
    "unique", "unique_copy",
    "remove", "remove_if", "remove_copy", "remove_copy_if",
    "lower_bound", "upper_bound", "equal_range", "binary_search",
    "merge", "inplace_merge",
    "min", "max", "minmax",
    "min_element", "max_element", "minmax_element",
    "accumulate", "inner_product",
    "partial_sum", "adjacent_difference",
    "for_each", "for_each_n",
    "all_of", "any_of", "none_of",
    "equal", "mismatch", "lexicographical_compare",
    "is_sorted", "is_sorted_until",
    "next_permutation", "prev_permutation",
    "partition", "stable_partition", "is_partitioned",
    "set_union", "set_intersection", "set_difference", "set_symmetric_difference",
    "includes",
    "generate", "generate_n",
    "iota",
    "clamp",
];

pub fn is_algorithm_symbol(name: &str) -> bool {
    ALGORITHM_FUNCTIONS.contains(&name)
}
