// ============================================================
// fastos_wchar.rs — <wchar.h> + <wctype.h> implementation
// ============================================================
// Wide character support via msvcrt.dll IAT imports
// ============================================================

pub const WCHAR_FUNCTIONS: &[&str] = &[
    // I/O
    "wprintf", "fwprintf", "swprintf",
    // String operations
    "wcscpy", "wcsncpy", "wcscat", "wcsncat",
    "wcscmp", "wcsncmp", "wcslen",
    // Search
    "wcschr", "wcsrchr", "wcsstr", "wcstok",
    // Conversion
    "wcstol", "wcstoul", "wcstod",
    // Multi-byte
    "mbstowcs", "wcstombs", "mbtowc", "wctomb",
    // Scanning
    "wscanf", "fwscanf", "swscanf",
    // Wide memory
    "wmemcpy", "wmemmove", "wmemset", "wmemcmp", "wmemchr",
    // Restartable multi-byte
    "mbrtowc", "wcrtomb", "mbrlen",
    // Wide character I/O
    "fgetwc", "fputwc", "fgetws", "fputws", "getwc", "putwc",
    // Time
    "wcsftime",
    // MSVC extension
    "wcsdup",
];

pub const WCTYPE_FUNCTIONS: &[&str] = &[
    "iswalpha", "iswdigit", "iswalnum", "iswspace",
    "iswupper", "iswlower", "iswprint", "iswpunct",
    "iswcntrl", "iswxdigit", "iswgraph",
    "towupper", "towlower",
];

pub const WCHAR_TYPES: &[&str] = &["wchar_t", "wint_t", "mbstate_t"];

pub const WCHAR_MACROS: &[(&str, &str)] = &[
    ("WEOF", "((wint_t)-1)"),
    ("WCHAR_MAX", "0xFFFF"),
    ("WCHAR_MIN", "0"),
];

pub fn is_wchar_symbol(name: &str) -> bool {
    WCHAR_FUNCTIONS.contains(&name)
        || WCTYPE_FUNCTIONS.contains(&name)
        || WCHAR_TYPES.contains(&name)
        || WCHAR_MACROS.iter().any(|(n, _)| *n == name)
}
