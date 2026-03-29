// Test: Production strings — all common string operations
// Expected: Compile + Run — real-world string patterns
// Strict: Every return value checked

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int main() {
    printf("=== PRODUCTION: Strings ===\n");
    int pass = 0, fail = 0;

    // strlen
    if (strlen("") == 0)              { pass++; } else { fail++; printf("FAIL: strlen empty\n"); }
    if (strlen("abc") == 3)           { pass++; } else { fail++; printf("FAIL: strlen abc\n"); }
    if (strlen("hello world") == 11)  { pass++; } else { fail++; printf("FAIL: strlen hello world\n"); }

    // strcmp
    if (strcmp("abc", "abc") == 0)     { pass++; } else { fail++; printf("FAIL: strcmp eq\n"); }
    if (strcmp("abc", "abd") < 0)      { pass++; } else { fail++; printf("FAIL: strcmp lt\n"); }
    if (strcmp("abd", "abc") > 0)      { pass++; } else { fail++; printf("FAIL: strcmp gt\n"); }
    if (strcmp("", "") == 0)           { pass++; } else { fail++; printf("FAIL: strcmp empty\n"); }

    // strncmp
    if (strncmp("abcdef", "abcxyz", 3) == 0) { pass++; } else { fail++; printf("FAIL: strncmp\n"); }
    if (strncmp("abc", "abd", 2) == 0)        { pass++; } else { fail++; printf("FAIL: strncmp 2\n"); }

    // strcpy + strcat
    char buf[64];
    strcpy(buf, "Hello");
    strcat(buf, ", ");
    strcat(buf, "World!");
    if (strcmp(buf, "Hello, World!") == 0) { pass++; } else { fail++; printf("FAIL: strcpy+strcat=%s\n", buf); }

    // strncpy
    char dst[8];
    strncpy(dst, "ABCDEFGHIJK", 7);
    dst[7] = '\0';
    if (strcmp(dst, "ABCDEFG") == 0) { pass++; } else { fail++; printf("FAIL: strncpy=%s\n", dst); }

    // strchr + strrchr
    const char *test = "hello.world.c";
    char *dot = strchr(test, '.');
    if (dot && strcmp(dot, ".world.c") == 0) { pass++; } else { fail++; printf("FAIL: strchr\n"); }
    char *rdot = strrchr(test, '.');
    if (rdot && strcmp(rdot, ".c") == 0) { pass++; } else { fail++; printf("FAIL: strrchr\n"); }

    // strstr
    char *found = strstr(test, "world");
    if (found && strcmp(found, "world.c") == 0) { pass++; } else { fail++; printf("FAIL: strstr\n"); }
    char *nf = strstr(test, "xyz");
    if (nf == 0) { pass++; } else { fail++; printf("FAIL: strstr not found\n"); }

    // memcpy + memcmp
    char a[16] = "ABCDEFGHIJKLMNO";
    char b[16];
    memcpy(b, a, 16);
    if (memcmp(a, b, 16) == 0) { pass++; } else { fail++; printf("FAIL: memcpy/memcmp\n"); }

    // memset
    char z[8];
    memset(z, 'X', 7);
    z[7] = '\0';
    if (strcmp(z, "XXXXXXX") == 0) { pass++; } else { fail++; printf("FAIL: memset=%s\n", z); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== PRODUCTION: Strings %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
