// ============================================================
// fastos_stdio.rs — <stdio.h> implementation
// ============================================================
// printf, scanf, fprintf, fopen, fclose, fread, fwrite
// Implementado sobre syscall write/read directo
// SIN libc — SIN linker externo
// ============================================================

/// Symbols exported by <stdio.h>
pub const STDIO_FUNCTIONS: &[&str] = &[
    // Core I/O
    "printf", "scanf", "fprintf", "fscanf", "sprintf", "snprintf", "sscanf",
    // File operations
    "fopen", "fclose", "fread", "fwrite", "freopen",
    // Character/line I/O
    "fgets", "fputs", "fgetc", "fputc", "getc", "putc",
    "puts", "putchar", "getchar", "ungetc",
    // Positioning
    "fseek", "ftell", "rewind", "fgetpos", "fsetpos",
    // Status
    "feof", "ferror", "clearerr", "fflush", "perror",
    // File management
    "remove", "rename", "tmpfile", "tmpnam",
    // Variadic
    "vprintf", "vfprintf", "vsprintf", "vsnprintf",
    // Buffering
    "setbuf", "setvbuf",
];

pub const STDIO_MACROS: &[(&str, &str)] = &[
    ("stdin", "((FILE*)0)"),
    ("stdout", "((FILE*)1)"),
    ("stderr", "((FILE*)2)"),
    ("EOF", "(-1)"),
    ("BUFSIZ", "8192"),
    ("FILENAME_MAX", "4096"),
    ("FOPEN_MAX", "16"),
    ("SEEK_SET", "0"),
    ("SEEK_CUR", "1"),
    ("SEEK_END", "2"),
];

pub const STDIO_TYPES: &[&str] = &["FILE", "fpos_t"];

/// Check if a symbol belongs to stdio
pub fn is_stdio_symbol(name: &str) -> bool {
    STDIO_FUNCTIONS.contains(&name)
        || STDIO_MACROS.iter().any(|(n, _)| *n == name)
        || STDIO_TYPES.contains(&name)
}
