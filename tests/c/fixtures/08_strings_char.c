// ============================================================
// Test 08: Strings y Caracteres — literales, operaciones manuales
// ============================================================
// ADead-BIB Test Canon — C99 §6.4.5
// Verifica: string literals, char arrays, escape sequences
// ============================================================

#include <stdio.h>
#include <string.h>

int my_strlen(const char *s) {
    int len = 0;
    while (s[len] != '\0') {
        len++;
    }
    return len;
}

void my_strcpy(char *dst, const char *src) {
    int i = 0;
    while (src[i] != '\0') {
        dst[i] = src[i];
        i++;
    }
    dst[i] = '\0';
}

int my_strcmp(const char *a, const char *b) {
    while (*a && *b && *a == *b) {
        a++;
        b++;
    }
    return *a - *b;
}

void my_strcat(char *dst, const char *src) {
    while (*dst) dst++;
    while (*src) {
        *dst = *src;
        dst++;
        src++;
    }
    *dst = '\0';
}

char my_toupper(char c) {
    if (c >= 'a' && c <= 'z') return c - 32;
    return c;
}

char my_tolower(char c) {
    if (c >= 'A' && c <= 'Z') return c + 32;
    return c;
}

void to_upper_str(char *s) {
    while (*s) {
        *s = my_toupper(*s);
        s++;
    }
}

int main() {
    // --- String literals ---
    const char *hello = "Hello, World!";
    printf("str=%s\n", hello);
    printf("len=%d\n", my_strlen(hello));

    // --- Char array ---
    char buf[64];
    my_strcpy(buf, "ADead");
    my_strcat(buf, "-BIB");
    printf("buf=%s\n", buf);

    // --- Comparación ---
    int cmp1 = my_strcmp("abc", "abc");
    int cmp2 = my_strcmp("abc", "abd");
    int cmp3 = my_strcmp("abd", "abc");
    printf("cmp: eq=%d lt=%d gt=%d\n", cmp1, (cmp2 < 0), (cmp3 > 0));

    // --- Escape sequences ---
    char tab = '\t';
    char newline = '\n';
    char null_ch = '\0';
    char backslash = '\\';
    char quote = '\"';
    printf("escapes: tab='%c' bs='%c' q='%c'\n", tab, backslash, quote);

    // --- Hex/octal char ---
    char hex_a = '\x41';
    char oct_a = '\101';
    printf("hex=%c oct=%c\n", hex_a, oct_a);

    // --- toupper/tolower ---
    char upper_buf[32];
    my_strcpy(upper_buf, "hello world");
    to_upper_str(upper_buf);
    printf("upper=%s\n", upper_buf);

    // --- String con stdlib ---
    printf("strlen=%d\n", (int)strlen("test"));
    char dst[32];
    strcpy(dst, "hello");
    strcat(dst, " world");
    printf("stdlib=%s\n", dst);

    // --- Búsqueda ---
    const char *found = strstr("hello world", "world");
    printf("strstr=%s\n", found);

    return 0;
}
