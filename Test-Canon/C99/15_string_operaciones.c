// ============================================================
// Canon C99 — §7.21 Strings (char arrays)
// ============================================================
// Intención: Un string en C es un array de char terminado
// en '\0'. No es un objeto — es memoria cruda con una
// convención de terminación.
//
// C99 §7.1.1: "A string is a contiguous sequence of
// characters terminated by and including the first null
// character."
//
// Todas las operaciones de string son loops sobre char*.
// ============================================================

#include <stdio.h>

// --- strlen: contar hasta encontrar '\0' ---
int my_strlen(const char *s) {
    int len = 0;
    while (s[len] != '\0') {
        len++;
    }
    return len;
}

// --- strcmp: comparar carácter por carácter ---
int my_strcmp(const char *a, const char *b) {
    int i = 0;
    while (a[i] != '\0' && b[i] != '\0') {
        if (a[i] != b[i]) {
            return a[i] - b[i];
        }
        i++;
    }
    return a[i] - b[i];
}

// --- strcpy: copiar hasta '\0' ---
void my_strcpy(char *dst, const char *src) {
    int i = 0;
    while (src[i] != '\0') {
        dst[i] = src[i];
        i++;
    }
    dst[i] = '\0';
}

// --- strcat: concatenar ---
void my_strcat(char *dst, const char *src) {
    int i = my_strlen(dst);
    int j = 0;
    while (src[j] != '\0') {
        dst[i] = src[j];
        i++;
        j++;
    }
    dst[i] = '\0';
}

// --- strchr: buscar carácter ---
int my_strchr(const char *s, char c) {
    int i = 0;
    while (s[i] != '\0') {
        if (s[i] == c) return i;
        i++;
    }
    return -1;
}

// --- toupper manual ---
char my_toupper(char c) {
    if (c >= 'a' && c <= 'z') {
        return c - 32;
    }
    return c;
}

// --- is_digit ---
int my_isdigit(char c) {
    return c >= '0' && c <= '9';
}

// --- atoi simple ---
int my_atoi(const char *s) {
    int result = 0;
    int sign = 1;
    int i = 0;

    if (s[0] == '-') {
        sign = -1;
        i = 1;
    }

    while (s[i] != '\0' && my_isdigit(s[i])) {
        result = result * 10 + (s[i] - '0');
        i++;
    }
    return result * sign;
}

int main() {
    printf("=== Canon C99: String Operaciones ===\n\n");

    // --- strlen ---
    printf("strlen:\n");
    printf("  \"Hello\" = %d\n", my_strlen("Hello"));
    printf("  \"\" = %d\n", my_strlen(""));
    printf("  \"ADead-BIB\" = %d\n", my_strlen("ADead-BIB"));

    // --- strcmp ---
    printf("\nstrcmp:\n");
    printf("  \"abc\" vs \"abc\" = %d\n", my_strcmp("abc", "abc"));
    printf("  \"abc\" vs \"abd\" = %d (negative)\n", my_strcmp("abc", "abd"));
    printf("  \"abd\" vs \"abc\" = %d (positive)\n", my_strcmp("abd", "abc"));

    // --- strcpy ---
    char buf[32];
    my_strcpy(buf, "Hello");
    printf("\nstrcpy: \"%s\"\n", buf);

    // --- strcat ---
    my_strcat(buf, " World");
    printf("strcat: \"%s\"\n", buf);

    // --- strchr ---
    printf("\nstrchr:\n");
    printf("  'l' in \"Hello\" = index %d\n", my_strchr("Hello", 'l'));
    printf("  'z' in \"Hello\" = index %d\n", my_strchr("Hello", 'z'));

    // --- toupper ---
    printf("\ntoupper:\n");
    printf("  'a' → '%c'\n", my_toupper('a'));
    printf("  'z' → '%c'\n", my_toupper('z'));
    printf("  'A' → '%c'\n", my_toupper('A'));
    printf("  '5' → '%c'\n", my_toupper('5'));

    // --- atoi ---
    printf("\natoi:\n");
    printf("  \"42\" = %d\n", my_atoi("42"));
    printf("  \"-17\" = %d\n", my_atoi("-17"));
    printf("  \"12345\" = %d\n", my_atoi("12345"));
    printf("  \"0\" = %d\n", my_atoi("0"));

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (my_strlen("Hello") == 5)        { pass++; } else { printf("FAIL: strlen\n"); }
    total++; if (my_strlen("") == 0)             { pass++; } else { printf("FAIL: strlen empty\n"); }
    total++; if (my_strcmp("abc", "abc") == 0)    { pass++; } else { printf("FAIL: strcmp eq\n"); }
    total++; if (my_strcmp("abc", "abd") < 0)     { pass++; } else { printf("FAIL: strcmp lt\n"); }
    total++; if (my_strchr("Hello", 'l') == 2)   { pass++; } else { printf("FAIL: strchr\n"); }
    total++; if (my_strchr("Hello", 'z') == -1)  { pass++; } else { printf("FAIL: strchr miss\n"); }
    total++; if (my_toupper('a') == 'A')         { pass++; } else { printf("FAIL: toupper\n"); }
    total++; if (my_atoi("42") == 42)            { pass++; } else { printf("FAIL: atoi\n"); }
    total++; if (my_atoi("-17") == -17)          { pass++; } else { printf("FAIL: atoi neg\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
