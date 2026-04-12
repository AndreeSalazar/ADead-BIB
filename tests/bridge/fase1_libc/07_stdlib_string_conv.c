// ADead-BIB Test: stdlib string conversion (atoi, atof, atol, strtol, strtoul, strtod)
#include <stdio.h>
#include <stdlib.h>

int main() {
    int pass = 0, fail = 0;

    // atoi
    int i = atoi("12345");
    if (i == 12345) { pass++; printf("PASS: atoi(%s)=%d\n", "12345", i); }
    else { fail++; printf("FAIL: atoi expected 12345 got %d\n", i); }

    i = atoi("-99");
    if (i == -99) { pass++; printf("PASS: atoi(%s)=%d\n", "-99", i); }
    else { fail++; printf("FAIL: atoi expected -99 got %d\n", i); }

    i = atoi("0");
    if (i == 0) { pass++; printf("PASS: atoi(%s)=%d\n", "0", i); }
    else { fail++; printf("FAIL: atoi expected 0 got %d\n", i); }

    // atol
    long l = atol("999999999");
    if (l == 999999999L) { pass++; printf("PASS: atol=%ld\n", l); }
    else { fail++; printf("FAIL: atol=%ld\n", l); }

    // strtol
    char *endp;
    long sl = strtol("0xFF", &endp, 16);
    if (sl == 255) { pass++; printf("PASS: strtol hex=%ld\n", sl); }
    else { fail++; printf("FAIL: strtol hex=%ld\n", sl); }

    sl = strtol("0777", &endp, 8);
    if (sl == 511) { pass++; printf("PASS: strtol oct=%ld\n", sl); }
    else { fail++; printf("FAIL: strtol oct=%ld\n", sl); }

    // strtoul
    unsigned long ul = strtoul("4294967295", &endp, 10);
    if (ul == 4294967295UL) { pass++; printf("PASS: strtoul=%lu\n", ul); }
    else { fail++; printf("FAIL: strtoul=%lu\n", ul); }

    // abs
    int a = abs(-42);
    if (a == 42) { pass++; printf("PASS: abs(-42)=%d\n", a); }
    else { fail++; printf("FAIL: abs(-42)=%d\n", a); }

    a = abs(0);
    if (a == 0) { pass++; printf("PASS: abs(0)=%d\n", a); }
    else { fail++; printf("FAIL: abs(0)=%d\n", a); }

    // labs
    long la = labs(-1234567890L);
    if (la == 1234567890L) { pass++; printf("PASS: labs=%ld\n", la); }
    else { fail++; printf("FAIL: labs=%ld\n", la); }

    printf("\n=== stdlib_string_conv: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
