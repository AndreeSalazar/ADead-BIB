// Test: <string.h> — String manipulation
// Expected: Compile + Run OK

#include <string.h>
#include <stdio.h>

int main() {
    printf("=== string.h test ===\n");

    // strlen
    const char *s = "hello world";
    int len = strlen(s);
    printf("strlen(\"%s\")=%d\n", s, len);

    // strcpy
    char dst[32];
    strcpy(dst, "copied");
    printf("strcpy: %s\n", dst);

    // strcmp
    int cmp = strcmp("abc", "abc");
    printf("strcmp(\"abc\",\"abc\")=%d\n", cmp);

    // memset
    char buf[8];
    memset(buf, 'X', 7);
    buf[7] = '\0';
    printf("memset: %s\n", buf);

    printf("=== string.h OK ===\n");
    return 0;
}
