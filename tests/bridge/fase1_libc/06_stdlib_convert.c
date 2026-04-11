#include <stdio.h>
#include <stdlib.h>

int main() {
    int pass = 0, fail = 0;

    // atoi
    if (atoi("42") == 42) { pass++; printf("PASS: atoi\n"); }
    else { fail++; printf("FAIL: atoi\n"); }

    if (atoi("-100") == -100) { pass++; printf("PASS: atoi negative\n"); }
    else { fail++; printf("FAIL: atoi negative\n"); }

    // atol
    if (atol("1234567890") == 1234567890L) { pass++; printf("PASS: atol\n"); }
    else { fail++; printf("FAIL: atol\n"); }

    // strtol
    char *end;
    long val = strtol("0xFF", &end, 16);
    if (val == 255) { pass++; printf("PASS: strtol hex\n"); }
    else { fail++; printf("FAIL: strtol hex\n"); }

    val = strtol("  -42abc", &end, 10);
    if (val == -42) { pass++; printf("PASS: strtol negative\n"); }
    else { fail++; printf("FAIL: strtol negative\n"); }

    // abs
    if (abs(-7) == 7) { pass++; printf("PASS: abs\n"); }
    else { fail++; printf("FAIL: abs\n"); }

    if (abs(7) == 7) { pass++; printf("PASS: abs positive\n"); }
    else { fail++; printf("FAIL: abs positive\n"); }

    printf("\n=== stdlib_convert: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
