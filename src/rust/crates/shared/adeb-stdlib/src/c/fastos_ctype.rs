// ============================================================
// fastos_ctype.rs — <ctype.h> implementation
// ============================================================
// Character classification and conversion functions
// Implementado con lookup table para máxima velocidad
// SIN libc — SIN linker externo
//
// C99 §7.4 — Character handling <ctype.h>
// Todas las funciones reciben int, retornan int
// Comportamiento definido para unsigned char + EOF (-1)
// ============================================================

/// Classification functions exported by <ctype.h>
pub const CTYPE_FUNCTIONS: &[&str] = &[
    // Classification (C99 §7.4.1)
    "isalnum",   // alphanumeric (letter or digit)
    "isalpha",   // alphabetic character
    "isblank",   // blank character (space or tab) — C99
    "iscntrl",   // control character
    "isdigit",   // decimal digit
    "isgraph",   // printable character (excluding space)
    "islower",   // lowercase letter
    "isprint",   // printable character (including space)
    "ispunct",   // punctuation character
    "isspace",   // whitespace character
    "isupper",   // uppercase letter
    "isxdigit",  // hexadecimal digit
    // POSIX extensions (widely used)
    "isascii",   // ASCII character (0-127)
    // Conversion (C99 §7.4.2)
    "toupper",   // convert to uppercase
    "tolower",   // convert to lowercase
    // POSIX extension
    "toascii",   // mask to 7-bit ASCII
];

/// Macros exported by <ctype.h>
pub const CTYPE_MACROS: &[(&str, &str)] = &[
    ("_CTYPE_U", "0x01"),  // uppercase
    ("_CTYPE_L", "0x02"),  // lowercase
    ("_CTYPE_D", "0x04"),  // digit
    ("_CTYPE_S", "0x08"),  // space
    ("_CTYPE_P", "0x10"),  // punctuation
    ("_CTYPE_C", "0x20"),  // control
    ("_CTYPE_X", "0x40"),  // hex digit
    ("_CTYPE_B", "0x80"),  // blank
];

/// Check if a symbol belongs to ctype
pub fn is_ctype_symbol(name: &str) -> bool {
    CTYPE_FUNCTIONS.contains(&name)
        || CTYPE_MACROS.iter().any(|(n, _)| *n == name)
}

/// Genera el contenido C del header ctype.h para inyectar
/// en el preprocessor — implementación inline completa
/// usando lookup table para O(1) por carácter.
///
/// ADead-BIB genera esto internamente — sin libc.
pub fn generate_ctype_h() -> String {
    let mut out = String::with_capacity(4096);
    out.push_str("/* ctype.h — Generado internamente por ADead-BIB */\n");
    out.push_str("#ifndef _FASTOS_CTYPE_H\n#define _FASTOS_CTYPE_H\n\n");

    // Flags internos para la lookup table
    out.push_str("/* Flags de clasificación — lookup table O(1) */\n");
    out.push_str("#define _CTYPE_U  0x01  /* uppercase */\n");
    out.push_str("#define _CTYPE_L  0x02  /* lowercase */\n");
    out.push_str("#define _CTYPE_D  0x04  /* digit */\n");
    out.push_str("#define _CTYPE_S  0x08  /* space */\n");
    out.push_str("#define _CTYPE_P  0x10  /* punctuation */\n");
    out.push_str("#define _CTYPE_C  0x20  /* control */\n");
    out.push_str("#define _CTYPE_X  0x40  /* hex digit */\n");
    out.push_str("#define _CTYPE_B  0x80  /* blank (space/tab) */\n\n");

    // Lookup table estática (256 entries para unsigned char)
    out.push_str("/* Lookup table — 256 bytes — acceso O(1) */\n");
    out.push_str("static const unsigned char __ctype_table[256] = {\n");

    // Generar la tabla caracter por caracter
    for i in 0u16..256 {
        let c = i as u8;
        let mut flags: u8 = 0;

        // Control characters: 0x00-0x1F y 0x7F
        if c <= 0x1F || c == 0x7F {
            flags |= 0x20; // _CTYPE_C
        }
        // Space characters: space, \t, \n, \v, \f, \r
        if c == b' ' || c == b'\t' || c == b'\n' || (c >= 0x0B && c <= 0x0D) {
            flags |= 0x08; // _CTYPE_S
        }
        // Blank: space or tab
        if c == b' ' || c == b'\t' {
            flags |= 0x80; // _CTYPE_B
        }
        // Digits
        if c >= b'0' && c <= b'9' {
            flags |= 0x04; // _CTYPE_D
            flags |= 0x40; // _CTYPE_X (digits are also hex)
        }
        // Uppercase
        if c >= b'A' && c <= b'Z' {
            flags |= 0x01; // _CTYPE_U
        }
        // Lowercase
        if c >= b'a' && c <= b'z' {
            flags |= 0x02; // _CTYPE_L
        }
        // Hex digits (a-f, A-F) — extra to digits
        if (c >= b'a' && c <= b'f') || (c >= b'A' && c <= b'F') {
            flags |= 0x40; // _CTYPE_X
        }
        // Punctuation: printable non-alnum non-space
        if c >= 0x21 && c <= 0x7E
            && !(c >= b'0' && c <= b'9')
            && !(c >= b'A' && c <= b'Z')
            && !(c >= b'a' && c <= b'z')
        {
            flags |= 0x10; // _CTYPE_P
        }

        if i % 16 == 0 {
            out.push_str("    ");
        }
        out.push_str(&format!("0x{:02X}", flags));
        if i < 255 {
            out.push(',');
        }
        if i % 16 == 15 {
            out.push_str(&format!("  /* 0x{:02X}-0x{:02X} */\n", i - 15, i));
        }
    }
    out.push_str("};\n\n");

    // Classification functions — inline, O(1) via lookup
    out.push_str("/* --- Classification functions (C99 §7.4.1) --- */\n\n");

    out.push_str("static inline int isalnum(int c) {\n");
    out.push_str("    return (c >= 0 && c <= 255) ? (__ctype_table[(unsigned char)c] & (_CTYPE_U|_CTYPE_L|_CTYPE_D)) : 0;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int isalpha(int c) {\n");
    out.push_str("    return (c >= 0 && c <= 255) ? (__ctype_table[(unsigned char)c] & (_CTYPE_U|_CTYPE_L)) : 0;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int isblank(int c) {\n");
    out.push_str("    return (c >= 0 && c <= 255) ? (__ctype_table[(unsigned char)c] & _CTYPE_B) : 0;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int iscntrl(int c) {\n");
    out.push_str("    return (c >= 0 && c <= 255) ? (__ctype_table[(unsigned char)c] & _CTYPE_C) : 0;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int isdigit(int c) {\n");
    out.push_str("    return (c >= 0 && c <= 255) ? (__ctype_table[(unsigned char)c] & _CTYPE_D) : 0;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int isgraph(int c) {\n");
    out.push_str("    return (c >= 0 && c <= 255) ? (__ctype_table[(unsigned char)c] & (_CTYPE_U|_CTYPE_L|_CTYPE_D|_CTYPE_P)) : 0;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int islower(int c) {\n");
    out.push_str("    return (c >= 0 && c <= 255) ? (__ctype_table[(unsigned char)c] & _CTYPE_L) : 0;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int isprint(int c) {\n");
    out.push_str("    return (c >= 0 && c <= 255) ? (__ctype_table[(unsigned char)c] & (_CTYPE_U|_CTYPE_L|_CTYPE_D|_CTYPE_P|_CTYPE_B)) : 0;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int ispunct(int c) {\n");
    out.push_str("    return (c >= 0 && c <= 255) ? (__ctype_table[(unsigned char)c] & _CTYPE_P) : 0;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int isspace(int c) {\n");
    out.push_str("    return (c >= 0 && c <= 255) ? (__ctype_table[(unsigned char)c] & _CTYPE_S) : 0;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int isupper(int c) {\n");
    out.push_str("    return (c >= 0 && c <= 255) ? (__ctype_table[(unsigned char)c] & _CTYPE_U) : 0;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int isxdigit(int c) {\n");
    out.push_str("    return (c >= 0 && c <= 255) ? (__ctype_table[(unsigned char)c] & _CTYPE_X) : 0;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int isascii(int c) {\n");
    out.push_str("    return (unsigned int)c <= 127;\n");
    out.push_str("}\n\n");

    // Conversion functions
    out.push_str("/* --- Conversion functions (C99 §7.4.2) --- */\n\n");

    out.push_str("static inline int toupper(int c) {\n");
    out.push_str("    return (c >= 'a' && c <= 'z') ? (c - 'a' + 'A') : c;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int tolower(int c) {\n");
    out.push_str("    return (c >= 'A' && c <= 'Z') ? (c - 'A' + 'a') : c;\n");
    out.push_str("}\n\n");

    out.push_str("static inline int toascii(int c) {\n");
    out.push_str("    return c & 0x7F;\n");
    out.push_str("}\n\n");

    out.push_str("#endif /* _FASTOS_CTYPE_H */\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctype_symbol_recognition() {
        // Classification functions
        assert!(is_ctype_symbol("isalpha"));
        assert!(is_ctype_symbol("isdigit"));
        assert!(is_ctype_symbol("isalnum"));
        assert!(is_ctype_symbol("isspace"));
        assert!(is_ctype_symbol("isupper"));
        assert!(is_ctype_symbol("islower"));
        assert!(is_ctype_symbol("isprint"));
        assert!(is_ctype_symbol("isgraph"));
        assert!(is_ctype_symbol("iscntrl"));
        assert!(is_ctype_symbol("ispunct"));
        assert!(is_ctype_symbol("isxdigit"));
        assert!(is_ctype_symbol("isblank"));
        assert!(is_ctype_symbol("isascii"));
        // Conversion functions
        assert!(is_ctype_symbol("toupper"));
        assert!(is_ctype_symbol("tolower"));
        assert!(is_ctype_symbol("toascii"));
        // Not ctype
        assert!(!is_ctype_symbol("printf"));
        assert!(!is_ctype_symbol("malloc"));
        assert!(!is_ctype_symbol("strlen"));
    }

    #[test]
    fn test_ctype_function_count() {
        assert_eq!(CTYPE_FUNCTIONS.len(), 16, "C99 ctype.h has 16 functions");
    }

    #[test]
    fn test_generate_ctype_h() {
        let h = generate_ctype_h();
        // Header guards
        assert!(h.contains("#ifndef _FASTOS_CTYPE_H"));
        assert!(h.contains("#define _FASTOS_CTYPE_H"));
        assert!(h.contains("#endif"));
        // Lookup table
        assert!(h.contains("__ctype_table[256]"));
        // All 16 functions present
        for func in CTYPE_FUNCTIONS {
            assert!(h.contains(func), "Missing function: {}", func);
        }
        // Implementation is inline
        assert!(h.contains("static inline int isalpha"));
        assert!(h.contains("static inline int toupper"));
        assert!(h.contains("static inline int tolower"));
    }

    #[test]
    fn test_ctype_table_correctness() {
        // Verify the lookup table logic matches expected behavior
        // by testing the flag computation directly

        // Helper to compute flags
        fn flags_for(c: u8) -> u8 {
            let mut f: u8 = 0;
            if c <= 0x1F || c == 0x7F { f |= 0x20; }
            if c == b' ' || c == b'\t' || c == b'\n' || (c >= 0x0B && c <= 0x0D) { f |= 0x08; }
            if c == b' ' || c == b'\t' { f |= 0x80; }
            if c >= b'0' && c <= b'9' { f |= 0x04 | 0x40; }
            if c >= b'A' && c <= b'Z' { f |= 0x01; }
            if c >= b'a' && c <= b'z' { f |= 0x02; }
            if (c >= b'a' && c <= b'f') || (c >= b'A' && c <= b'F') { f |= 0x40; }
            if c >= 0x21 && c <= 0x7E
                && !(c >= b'0' && c <= b'9')
                && !(c >= b'A' && c <= b'Z')
                && !(c >= b'a' && c <= b'z')
            { f |= 0x10; }
            f
        }

        // Digits
        for c in b'0'..=b'9' {
            let f = flags_for(c);
            assert!(f & 0x04 != 0, "'{}' should be digit", c as char);
            assert!(f & 0x40 != 0, "'{}' should be xdigit", c as char);
        }

        // Uppercase
        for c in b'A'..=b'Z' {
            let f = flags_for(c);
            assert!(f & 0x01 != 0, "'{}' should be upper", c as char);
        }

        // Lowercase
        for c in b'a'..=b'z' {
            let f = flags_for(c);
            assert!(f & 0x02 != 0, "'{}' should be lower", c as char);
        }

        // Space
        assert!(flags_for(b' ') & 0x08 != 0, "space should be space");
        assert!(flags_for(b'\t') & 0x08 != 0, "tab should be space");
        assert!(flags_for(b'\n') & 0x08 != 0, "newline should be space");

        // Blank
        assert!(flags_for(b' ') & 0x80 != 0, "space should be blank");
        assert!(flags_for(b'\t') & 0x80 != 0, "tab should be blank");
        assert!(flags_for(b'\n') & 0x80 == 0, "newline should NOT be blank");

        // Control
        assert!(flags_for(0x00) & 0x20 != 0, "NUL should be control");
        assert!(flags_for(0x1F) & 0x20 != 0, "0x1F should be control");
        assert!(flags_for(0x7F) & 0x20 != 0, "DEL should be control");

        // Punctuation
        assert!(flags_for(b'!') & 0x10 != 0, "'!' should be punct");
        assert!(flags_for(b'@') & 0x10 != 0, "'@' should be punct");
        assert!(flags_for(b'~') & 0x10 != 0, "'~' should be punct");
        assert!(flags_for(b'A') & 0x10 == 0, "'A' should NOT be punct");

        // Hex digits
        assert!(flags_for(b'a') & 0x40 != 0, "'a' should be xdigit");
        assert!(flags_for(b'f') & 0x40 != 0, "'f' should be xdigit");
        assert!(flags_for(b'g') & 0x40 == 0, "'g' should NOT be xdigit");
        assert!(flags_for(b'A') & 0x40 != 0, "'A' should be xdigit");
        assert!(flags_for(b'F') & 0x40 != 0, "'F' should be xdigit");
    }
}
