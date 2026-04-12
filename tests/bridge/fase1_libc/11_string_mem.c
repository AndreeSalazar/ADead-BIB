// ADead-BIB Test: string.h memory functions (memcpy, memmove, memset, memcmp, memchr)
#include <stdio.h>
#include <string.h>

int main() {
    int pass = 0, fail = 0;

    // memset
    char buf[32];
    memset(buf, 'A', 10);
    buf[10] = '\0';
    if (buf[0] == 'A' && buf[9] == 'A' && buf[10] == '\0') {
        pass++; printf("PASS: memset '%s'\n", buf);
    } else { fail++; printf("FAIL: memset\n"); }

    // memcpy
    char src[16];
    char dst[16];
    memset(src, 0, 16);
    strcpy(src, "Hello");
    memcpy(dst, src, 6);
    if (strcmp(dst, "Hello") == 0) { pass++; printf("PASS: memcpy '%s'\n", dst); }
    else { fail++; printf("FAIL: memcpy '%s'\n", dst); }

    // memmove (overlapping)
    char overlap[32];
    strcpy(overlap, "ABCDEFGHIJ");
    memmove(overlap + 2, overlap, 8);
    // Expected: "ABABCDEFGH"
    if (overlap[0] == 'A' && overlap[1] == 'B' && overlap[2] == 'A' && overlap[3] == 'B') {
        pass++; printf("PASS: memmove overlap\n");
    } else { fail++; printf("FAIL: memmove overlap\n"); }

    // memcmp
    char a[8];
    char b[8];
    strcpy(a, "abc");
    strcpy(b, "abc");
    if (memcmp(a, b, 3) == 0) { pass++; printf("PASS: memcmp equal\n"); }
    else { fail++; printf("FAIL: memcmp equal\n"); }

    strcpy(b, "abd");
    if (memcmp(a, b, 3) < 0) { pass++; printf("PASS: memcmp less\n"); }
    else { fail++; printf("FAIL: memcmp less\n"); }

    // memchr
    char haystack[16];
    strcpy(haystack, "Hello World");
    char *found = (char*)memchr(haystack, 'W', 11);
    if (found && *found == 'W') { pass++; printf("PASS: memchr found 'W'\n"); }
    else { fail++; printf("FAIL: memchr\n"); }

    found = (char*)memchr(haystack, 'Z', 11);
    if (found == 0) { pass++; printf("PASS: memchr not found 'Z'\n"); }
    else { fail++; printf("FAIL: memchr found 'Z'?\n"); }

    printf("\n=== string_mem: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
