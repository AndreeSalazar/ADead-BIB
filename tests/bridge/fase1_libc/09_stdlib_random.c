#include <stdio.h>
#include <stdlib.h>

int main() {
    int pass = 0, fail = 0;

    // srand + rand
    srand(12345);
    int r1 = rand();
    int r2 = rand();
    if (r1 != r2) { pass++; printf("PASS: rand produces different values: %d, %d\n", r1, r2); }
    else { fail++; printf("FAIL: rand same values\n"); }

    // Reproducibility
    srand(12345);
    int r1b = rand();
    if (r1 == r1b) { pass++; printf("PASS: srand reproducible\n"); }
    else { fail++; printf("FAIL: srand not reproducible\n"); }

    // Range check (0 to RAND_MAX)
    srand(999);
    int valid = 1;
    int i;
    for (i = 0; i < 100; i++) {
        int r = rand();
        if (r < 0) valid = 0;
    }
    if (valid) { pass++; printf("PASS: rand >= 0\n"); }
    else { fail++; printf("FAIL: rand negative\n"); }

    printf("\n=== stdlib_random: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
