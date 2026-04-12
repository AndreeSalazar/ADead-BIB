// ADead-BIB Test: string.h search functions (strchr, strrchr, strstr, strtok, strpbrk, strspn, strcspn)
#include <stdio.h>
#include <string.h>

int main() {
    int pass = 0, fail = 0;

    // strlen
    int len = strlen("Hello");
    if (len == 5) { pass++; printf("PASS: strlen=%d\n", len); }
    else { fail++; printf("FAIL: strlen=%d\n", len); }

    // strcpy + strcmp
    char buf[64];
    strcpy(buf, "Hello World");
    if (strcmp(buf, "Hello World") == 0) { pass++; printf("PASS: strcpy+strcmp\n"); }
    else { fail++; printf("FAIL: strcpy+strcmp\n"); }

    // strncpy
    char dst[16];
    memset(dst, 0, 16);
    strncpy(dst, "Hello", 3);
    if (dst[0] == 'H' && dst[1] == 'e' && dst[2] == 'l') { pass++; printf("PASS: strncpy\n"); }
    else { fail++; printf("FAIL: strncpy\n"); }

    // strcat
    char cat[64];
    strcpy(cat, "Hello");
    strcat(cat, " World");
    if (strcmp(cat, "Hello World") == 0) { pass++; printf("PASS: strcat\n"); }
    else { fail++; printf("FAIL: strcat '%s'\n", cat); }

    // strncat
    char ncat[64];
    strcpy(ncat, "ABC");
    strncat(ncat, "DEFGHIJ", 3);
    if (strcmp(ncat, "ABCDEF") == 0) { pass++; printf("PASS: strncat\n"); }
    else { fail++; printf("FAIL: strncat '%s'\n", ncat); }

    // strncmp
    if (strncmp("Hello", "Help", 3) == 0) { pass++; printf("PASS: strncmp equal prefix\n"); }
    else { fail++; printf("FAIL: strncmp\n"); }

    // strchr
    char *p = strchr("Hello World", 'W');
    if (p && *p == 'W') { pass++; printf("PASS: strchr found 'W'\n"); }
    else { fail++; printf("FAIL: strchr\n"); }

    // strrchr
    p = strrchr("Hello World", 'l');
    if (p && *p == 'l' && (p - "Hello World") == 9) { pass++; printf("PASS: strrchr\n"); }
    else { fail++; printf("FAIL: strrchr\n"); }

    // strstr
    p = strstr("Hello World", "World");
    if (p && strcmp(p, "World") == 0) { pass++; printf("PASS: strstr\n"); }
    else { fail++; printf("FAIL: strstr\n"); }

    // strtok
    char tokbuf[64];
    strcpy(tokbuf, "one,two,three");
    char *tok = strtok(tokbuf, ",");
    if (tok && strcmp(tok, "one") == 0) { pass++; printf("PASS: strtok first='%s'\n", tok); }
    else { fail++; printf("FAIL: strtok first\n"); }

    tok = strtok(0, ",");
    if (tok && strcmp(tok, "two") == 0) { pass++; printf("PASS: strtok second='%s'\n", tok); }
    else { fail++; printf("FAIL: strtok second\n"); }

    // strpbrk
    p = strpbrk("Hello World", "aeiou");
    if (p && *p == 'e') { pass++; printf("PASS: strpbrk found '%c'\n", *p); }
    else { fail++; printf("FAIL: strpbrk\n"); }

    // strspn
    int sp = strspn("aabbc", "ab");
    if (sp == 4) { pass++; printf("PASS: strspn=%d\n", sp); }
    else { fail++; printf("FAIL: strspn=%d\n", sp); }

    // strcspn
    sp = strcspn("Hello World", " ");
    if (sp == 5) { pass++; printf("PASS: strcspn=%d\n", sp); }
    else { fail++; printf("FAIL: strcspn=%d\n", sp); }

    // strerror
    char *err = strerror(0);
    if (err) { pass++; printf("PASS: strerror(0)='%s'\n", err); }
    else { fail++; printf("FAIL: strerror null\n"); }

    printf("\n=== string_search: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
