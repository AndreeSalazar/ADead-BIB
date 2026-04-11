#include <stdio.h>
#include <limits.h>
#include <stdint.h>

int main() {
    int pass = 0, fail = 0;

    // INT_MAX / INT_MIN
    if (INT_MAX == 2147483647) { pass++; printf("PASS: INT_MAX\n"); }
    else { fail++; printf("FAIL: INT_MAX\n"); }

    // CHAR_BIT
    if (CHAR_BIT == 8) { pass++; printf("PASS: CHAR_BIT\n"); }
    else { fail++; printf("FAIL: CHAR_BIT\n"); }

    // sizeof checks
    if (sizeof(char) == 1) { pass++; printf("PASS: sizeof(char)\n"); }
    else { fail++; printf("FAIL: sizeof(char)\n"); }

    if (sizeof(short) == 2) { pass++; printf("PASS: sizeof(short)\n"); }
    else { fail++; printf("FAIL: sizeof(short)\n"); }

    if (sizeof(int) == 4) { pass++; printf("PASS: sizeof(int)\n"); }
    else { fail++; printf("FAIL: sizeof(int)\n"); }

    // stdint types
    if (sizeof(int8_t) == 1) { pass++; printf("PASS: sizeof(int8_t)\n"); }
    else { fail++; printf("FAIL: sizeof(int8_t)\n"); }

    if (sizeof(int32_t) == 4) { pass++; printf("PASS: sizeof(int32_t)\n"); }
    else { fail++; printf("FAIL: sizeof(int32_t)\n"); }

    // INT32_MAX
    if (INT32_MAX == 2147483647) { pass++; printf("PASS: INT32_MAX\n"); }
    else { fail++; printf("FAIL: INT32_MAX\n"); }

    printf("\n=== limits_types: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
