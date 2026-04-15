// ============================================================
// Test 22: <stdlib.h> Completo — conversiones, random, exit, abs
// ============================================================
// ADead-BIB Test Canon — C99 §7.22
// Verifica: atoi, atol, strtol, abs, rand/srand, system, getenv
// ============================================================

#include <stdio.h>
#include <stdlib.h>

int main() {
    // --- atoi ---
    int a1 = atoi("42");
    int a2 = atoi("-123");
    int a3 = atoi("0");
    int a4 = atoi("   456");
    printf("atoi: %d %d %d %d\n", a1, a2, a3, a4);

    // --- atol ---
    long l1 = atol("100000");
    long l2 = atol("-999999");
    printf("atol: %ld %ld\n", l1, l2);

    // --- strtol ---
    char *endptr;
    long s1 = strtol("12345", &endptr, 10);
    printf("strtol(10): %ld rest='%s'\n", s1, endptr);

    long s2 = strtol("FF", &endptr, 16);
    printf("strtol(16): %ld\n", s2);

    long s3 = strtol("777", &endptr, 8);
    printf("strtol(8): %ld\n", s3);

    long s4 = strtol("1010", &endptr, 2);
    printf("strtol(2): %ld\n", s4);

    long s5 = strtol("42abc", &endptr, 10);
    printf("strtol partial: %ld rest='%s'\n", s5, endptr);

    // --- abs ---
    printf("abs(42)=%d abs(-42)=%d abs(0)=%d\n", abs(42), abs(-42), abs(0));

    // --- rand/srand ---
    srand(12345);
    printf("rand: %d %d %d\n", rand() % 100, rand() % 100, rand() % 100);

    srand(12345);
    int r1 = rand();
    srand(12345);
    int r2 = rand();
    printf("deterministic: %d==%d -> %d\n", r1, r2, (r1 == r2));

    // --- getenv ---
    const char *path = getenv("PATH");
    printf("PATH exists=%d\n", (path != (char *)0));

    const char *nonexist = getenv("THIS_SHOULD_NOT_EXIST_ADEAD_BIB_TEST");
    printf("nonexist=%d\n", (nonexist == (char *)0));

    // --- NULL ---
    void *n = NULL;
    printf("NULL=%p\n", n);

    // --- EXIT codes ---
    printf("EXIT_SUCCESS=%d EXIT_FAILURE=%d\n", EXIT_SUCCESS, EXIT_FAILURE);

    return 0;
}
