// ============================================================
// fastos_stdlib.rs — <stdlib.h> implementation
// ============================================================
// malloc, free, calloc, realloc, exit, atoi, rand, qsort
// Implementado sobre syscall mmap/munmap directo
// SIN libc — SIN linker externo
// ============================================================

pub const STDLIB_FUNCTIONS: &[&str] = &[
    "malloc", "calloc", "realloc", "free",
    "exit", "abort", "_Exit",
    "atoi", "atol", "atoll", "atof",
    "strtol", "strtoll", "strtoul", "strtoull", "strtod", "strtof",
    "rand", "srand",
    "qsort", "bsearch",
    "abs", "labs", "llabs",
    "div", "ldiv", "lldiv",
    "getenv", "system",
    "atexit",
    // C11
    "aligned_alloc",
    // Multi-byte
    "mbstowcs", "wcstombs", "mbtowc", "wctomb",
    // MSVC extensions — memory
    "_aligned_malloc", "_aligned_free",
    // MSVC extensions — threads
    "_beginthread", "_endthread", "_beginthreadex", "_endthreadex",
    // MSVC extensions — string
    "_stricmp", "_strnicmp",
    "_snprintf_s", "sprintf_s", "strcpy_s", "strncpy_s", "strcat_s",
    "_strdup", "_strlwr", "_strupr",
    "_itoa", "_ltoa", "_ui64toa",
    // MSVC extensions — file I/O
    "_open", "_close", "_read", "_write", "_lseek",
    "_stat", "_fstat",
    "_mkdir", "_rmdir", "_chdir", "_getcwd",
    "_findfirst", "_findnext", "_findclose",
    "_access",
    "_fullpath", "_makepath", "_splitpath",
    // MSVC extensions — misc
    "_putenv", "_sleep",
    "_setmode", "_fileno", "_isatty",
];

pub const STDLIB_MACROS: &[(&str, &str)] = &[
    ("NULL", "((void*)0)"),
    ("EXIT_SUCCESS", "0"),
    ("EXIT_FAILURE", "1"),
    ("RAND_MAX", "2147483647"),
    ("MB_CUR_MAX", "4"),
    ("MB_LEN_MAX", "16"),
];

pub const STDLIB_TYPES: &[&str] = &["div_t", "ldiv_t", "lldiv_t", "wchar_t"];

pub fn is_stdlib_symbol(name: &str) -> bool {
    STDLIB_FUNCTIONS.contains(&name)
        || STDLIB_MACROS.iter().any(|(n, _)| *n == name)
        || STDLIB_TYPES.contains(&name)
}
