// ============================================================
// Test 03: <ctype.h> — Real-world Usage: Token Parser
// ============================================================
// ADead-BIB Test Canon — Aplicación práctica
// Un mini-parser que usa ctype.h para clasificar tokens
// en una cadena. Demuestra el uso real de is* y to*.
// Expected: Compila OK — todos los tokens clasificados
// ============================================================
#include <stdio.h>
#include <ctype.h>
#include <string.h>

// Cuenta caracteres por categoría usando ctype
void classify_string(const char *s) {
    int letters = 0, digits = 0, spaces = 0, puncts = 0, others = 0;
    int len = strlen(s);

    for (int i = 0; i < len; i++) {
        unsigned char c = (unsigned char)s[i];
        if (isalpha(c))      letters++;
        else if (isdigit(c)) digits++;
        else if (isspace(c)) spaces++;
        else if (ispunct(c)) puncts++;
        else                 others++;
    }

    printf("Input: \"%s\"\n", s);
    printf("  Letters: %d, Digits: %d, Spaces: %d, Punct: %d, Other: %d\n",
           letters, digits, spaces, puncts, others);
}

// Convierte una cadena a uppercase usando toupper
void to_upper_str(const char *src, char *dst) {
    int i = 0;
    while (src[i]) {
        dst[i] = (char)toupper((unsigned char)src[i]);
        i++;
    }
    dst[i] = '\0';
}

// Parsea un número hexadecimal simple
int parse_hex(const char *s) {
    int result = 0;
    for (int i = 0; s[i]; i++) {
        unsigned char c = (unsigned char)s[i];
        if (!isxdigit(c)) break;
        result *= 16;
        if (isdigit(c))
            result += c - '0';
        else
            result += tolower(c) - 'a' + 10;
    }
    return result;
}

int main() {
    // Test 1: Classify a mixed string
    classify_string("Hello, World! 123");

    // Test 2: Convert to uppercase
    char upper[64];
    to_upper_str("adead-bib v8.0", upper);
    printf("Uppercase: %s\n", upper);

    // Test 3: Parse hex
    int hex_val = parse_hex("1A2F");
    printf("Hex '1A2F' = %d (expected 6703)\n", hex_val);

    // Test 4: Validate identifier
    const char *ident = "my_var123";
    int valid = 1;
    if (!isalpha((unsigned char)ident[0]) && ident[0] != '_') {
        valid = 0;
    }
    for (int i = 1; ident[i]; i++) {
        if (!isalnum((unsigned char)ident[i]) && ident[i] != '_') {
            valid = 0;
            break;
        }
    }
    printf("Identifier '%s' valid: %d (expected 1)\n", ident, valid);

    return 0;
}
// Expected output:
// Input: "Hello, World! 123"
//   Letters: 10, Digits: 3, Spaces: 2, Punct: 2, Other: 0
// Uppercase: ADEAD-BIB V8.0
// Hex '1A2F' = 6703 (expected 6703)
// Identifier 'my_var123' valid: 1 (expected 1)
