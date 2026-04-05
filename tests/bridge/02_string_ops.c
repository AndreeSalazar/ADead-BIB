// ADead-BIB Bridge Test 02 — String Operations (ASM-BIB bridge)
// Level: BASIC
// Tests: strlen, strcpy, strcmp, strcat, strchr, memcpy, memset, memcmp
// These map to ASM-BIB asm_strlen, asm_strcpy, etc. via bridge

#include <stdio.h>
#include <string.h>

int main() {
    printf("=== ADead-BIB Bridge Test 02: String Ops ===\n");
    int pass = 0, fail = 0;

    // strlen
    const char *hello = "Hello";
    if (strlen(hello) == 5) { pass++; } else { fail++; printf("FAIL: strlen\n"); }

    // strcpy
    char buf[64];
    strcpy(buf, "World");
    if (strcmp(buf, "World") == 0) { pass++; } else { fail++; printf("FAIL: strcpy\n"); }

    // strcmp
    if (strcmp("abc", "abc") == 0) { pass++; } else { fail++; printf("FAIL: strcmp equal\n"); }
    if (strcmp("abc", "abd") < 0)  { pass++; } else { fail++; printf("FAIL: strcmp less\n"); }
    if (strcmp("abd", "abc") > 0)  { pass++; } else { fail++; printf("FAIL: strcmp greater\n"); }

    // strcat
    char dest[64];
    strcpy(dest, "Hello");
    strcat(dest, " World");
    if (strcmp(dest, "Hello World") == 0) { pass++; } else { fail++; printf("FAIL: strcat\n"); }

    // strchr
    const char *search = "Hello World";
    char *found = strchr(search, 'W');
    if (found != NULL && *found == 'W') { pass++; } else { fail++; printf("FAIL: strchr found\n"); }
    char *nf = strchr(search, 'Z');
    if (nf == NULL) { pass++; } else { fail++; printf("FAIL: strchr not found\n"); }

    // memcpy
    char src[] = "ABCDEFGH";
    char dst[16];
    memcpy(dst, src, 8);
    dst[8] = '\0';
    if (strcmp(dst, "ABCDEFGH") == 0) { pass++; } else { fail++; printf("FAIL: memcpy\n"); }

    // memset
    char zeros[16];
    memset(zeros, 0, 16);
    int all_zero = 1;
    for (int i = 0; i < 16; i++) { if (zeros[i] != 0) all_zero = 0; }
    if (all_zero) { pass++; } else { fail++; printf("FAIL: memset\n"); }

    memset(zeros, 0x41, 8);
    if (zeros[0] == 'A' && zeros[7] == 'A' && zeros[8] == 0) { pass++; } else { fail++; printf("FAIL: memset pattern\n"); }

    // memcmp
    if (memcmp("ABC", "ABC", 3) == 0) { pass++; } else { fail++; printf("FAIL: memcmp equal\n"); }
    if (memcmp("ABC", "ABD", 3) < 0)  { pass++; } else { fail++; printf("FAIL: memcmp less\n"); }
    if (memcmp("ABD", "ABC", 3) > 0)  { pass++; } else { fail++; printf("FAIL: memcmp greater\n"); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 02: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
