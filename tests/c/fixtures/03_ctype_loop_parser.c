// ============================================================
// Test 03: <ctype.h> — Real-world Usage: Classify + Convert
// ============================================================
// ADead-BIB Test Canon — Aplicación práctica
// Demuestra uso real de isalpha, isdigit, toupper, isxdigit
// Expected: Compila OK + Ejecuta OK
// ============================================================
#include <stdio.h>
#include <ctype.h>

int count_alpha(const char *s) {
    int n = 0;
    while (*s) {
        if (isalpha(*s)) n++;
        s++;
    }
    return n;
}

int count_digits(const char *s) {
    int n = 0;
    while (*s) {
        if (isdigit(*s)) n++;
        s++;
    }
    return n;
}

int parse_hex_digit(char c) {
    if (isdigit(c)) return c - '0';
    if (isxdigit(c)) return tolower(c) - 'a' + 10;
    return -1;
}

int main() {
    const char *test = "Hello 42!";
    int a = count_alpha(test);
    int d = count_digits(test);
    printf("alpha=%d digits=%d\n", a, d);

    int h1 = parse_hex_digit('A');
    int h2 = parse_hex_digit('f');
    int h3 = parse_hex_digit('3');
    printf("hex: A=%d f=%d 3=%d\n", h1, h2, h3);

    int up = toupper('a');
    int lo = tolower('Z');
    printf("toupper(a)=%d tolower(Z)=%d\n", up, lo);

    return 0;
}
