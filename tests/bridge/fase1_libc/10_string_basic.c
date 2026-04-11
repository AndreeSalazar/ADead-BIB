#include <stdio.h>
#include <string.h>

int main() {
    int pass = 0, fail = 0;

    // strlen
    if (strlen("hello") == 5) { pass++; printf("PASS: strlen\n"); }
    else { fail++; printf("FAIL: strlen\n"); }

    if (strlen("") == 0) { pass++; printf("PASS: strlen empty\n"); }
    else { fail++; printf("FAIL: strlen empty\n"); }

    // strcmp
    if (strcmp("abc", "abc") == 0) { pass++; printf("PASS: strcmp equal\n"); }
    else { fail++; printf("FAIL: strcmp equal\n"); }

    if (strcmp("abc", "abd") < 0) { pass++; printf("PASS: strcmp less\n"); }
    else { fail++; printf("FAIL: strcmp less\n"); }

    // strncmp
    if (strncmp("hello world", "hello earth", 5) == 0) { pass++; printf("PASS: strncmp\n"); }
    else { fail++; printf("FAIL: strncmp\n"); }

    // strstr
    char *found = strstr("hello world", "world");
    if (found && strcmp(found, "world") == 0) { pass++; printf("PASS: strstr\n"); }
    else { fail++; printf("FAIL: strstr\n"); }

    // strstr not found
    if (strstr("hello", "xyz") == 0) { pass++; printf("PASS: strstr not found\n"); }
    else { fail++; printf("FAIL: strstr not found\n"); }

    printf("\n=== string_basic: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
