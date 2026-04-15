// ============================================================
// Test 23: <string.h> Completo — todas las funciones de string y memoria
// ============================================================
// ADead-BIB Test Canon — C99 §7.24
// Verifica: str*, mem*, strerror
// ============================================================

#include <stdio.h>
#include <string.h>

int main() {
    // --- strlen ---
    printf("strlen('')=%d\n", (int)strlen(""));
    printf("strlen('hello')=%d\n", (int)strlen("hello"));
    printf("strlen('ADead-BIB')=%d\n", (int)strlen("ADead-BIB"));

    // --- strcpy / strncpy ---
    char buf[64];
    strcpy(buf, "hello");
    printf("strcpy: %s\n", buf);

    char buf2[10];
    strncpy(buf2, "hello world", 9);
    buf2[9] = '\0';
    printf("strncpy: %s\n", buf2);

    // --- strcat / strncat ---
    char cat[64];
    strcpy(cat, "hello");
    strcat(cat, " ");
    strcat(cat, "world");
    printf("strcat: %s\n", cat);

    char ncat[32];
    strcpy(ncat, "hello");
    strncat(ncat, " world!!!", 6);
    printf("strncat: %s\n", ncat);

    // --- strcmp / strncmp ---
    printf("strcmp eq=%d\n", strcmp("abc", "abc"));
    printf("strcmp lt=%d\n", (strcmp("abc", "abd") < 0));
    printf("strcmp gt=%d\n", (strcmp("abd", "abc") > 0));
    printf("strncmp=%d\n", strncmp("hello world", "hello there", 5));

    // --- strchr / strrchr ---
    const char *s = "hello world";
    const char *ch = strchr(s, 'o');
    printf("strchr 'o': %s\n", ch);

    const char *rch = strrchr(s, 'o');
    printf("strrchr 'o': %s\n", rch);

    const char *null_ch = strchr(s, 'z');
    printf("strchr 'z': %p\n", (void *)null_ch);

    // --- strstr ---
    const char *sub = strstr("hello world", "world");
    printf("strstr 'world': %s\n", sub);

    const char *nosub = strstr("hello world", "xyz");
    printf("strstr 'xyz': %p\n", (void *)nosub);

    // --- strtok ---
    char tok_buf[64];
    strcpy(tok_buf, "one,two,three,four");
    char *token = strtok(tok_buf, ",");
    int tok_count = 0;
    while (token) {
        printf("token[%d]=%s\n", tok_count, token);
        token = strtok((char *)0, ",");
        tok_count++;
    }

    // --- memset ---
    char mem[16];
    memset(mem, 'X', 10);
    mem[10] = '\0';
    printf("memset: %s\n", mem);

    // --- memcpy ---
    char src[] = "ABCDEFGH";
    char dst[16];
    memcpy(dst, src, 8);
    dst[8] = '\0';
    printf("memcpy: %s\n", dst);

    // --- memcmp ---
    printf("memcmp eq=%d\n", memcmp("abcd", "abcd", 4));
    printf("memcmp ne=%d\n", (memcmp("abcd", "abce", 4) != 0));

    // --- memmove (overlapping) ---
    char overlap[32] = "ABCDEFGHIJ";
    memmove(overlap + 2, overlap, 8);
    overlap[10] = '\0';
    printf("memmove: %s\n", overlap);

    return 0;
}
