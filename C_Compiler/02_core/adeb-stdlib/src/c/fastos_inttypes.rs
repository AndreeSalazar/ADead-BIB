// ============================================================
// fastos_inttypes.rs — <inttypes.h> implementation
// ============================================================
// Format macros for fixed-width integers (C99 §7.8)
// PRId32, PRIu64, PRIx16, SCNd32, etc.
// Also: imaxabs, imaxdiv, strtoimax, strtoumax
// ============================================================

pub const INTTYPES_FUNCTIONS: &[&str] = &[
    "imaxabs", "imaxdiv", "strtoimax", "strtoumax",
    "wcstoimax", "wcstoumax",
];

pub const INTTYPES_MACROS: &[(&str, &str)] = &[
    // Printf format macros — decimal
    ("PRId8", "\"d\""),
    ("PRId16", "\"d\""),
    ("PRId32", "\"d\""),
    ("PRId64", "\"lld\""),
    ("PRIi8", "\"i\""),
    ("PRIi16", "\"i\""),
    ("PRIi32", "\"i\""),
    ("PRIi64", "\"lli\""),
    // Printf format macros — unsigned
    ("PRIu8", "\"u\""),
    ("PRIu16", "\"u\""),
    ("PRIu32", "\"u\""),
    ("PRIu64", "\"llu\""),
    // Printf format macros — hex
    ("PRIx8", "\"x\""),
    ("PRIx16", "\"x\""),
    ("PRIx32", "\"x\""),
    ("PRIx64", "\"llx\""),
    ("PRIX8", "\"X\""),
    ("PRIX16", "\"X\""),
    ("PRIX32", "\"X\""),
    ("PRIX64", "\"llX\""),
    // Printf format macros — octal
    ("PRIo8", "\"o\""),
    ("PRIo16", "\"o\""),
    ("PRIo32", "\"o\""),
    ("PRIo64", "\"llo\""),
    // Printf macros — pointer-sized
    ("PRIdPTR", "\"lld\""),
    ("PRIuPTR", "\"llu\""),
    ("PRIxPTR", "\"llx\""),
    ("PRIdMAX", "\"lld\""),
    ("PRIuMAX", "\"llu\""),
    // Scanf format macros
    ("SCNd8", "\"hhd\""),
    ("SCNd16", "\"hd\""),
    ("SCNd32", "\"d\""),
    ("SCNd64", "\"lld\""),
    ("SCNu8", "\"hhu\""),
    ("SCNu16", "\"hu\""),
    ("SCNu32", "\"u\""),
    ("SCNu64", "\"llu\""),
    ("SCNx8", "\"hhx\""),
    ("SCNx16", "\"hx\""),
    ("SCNx32", "\"x\""),
    ("SCNx64", "\"llx\""),
];

pub const INTTYPES_TYPES: &[&str] = &["imaxdiv_t"];

pub fn is_inttypes_symbol(name: &str) -> bool {
    INTTYPES_FUNCTIONS.contains(&name)
        || INTTYPES_MACROS.iter().any(|(n, _)| *n == name)
        || INTTYPES_TYPES.contains(&name)
}
