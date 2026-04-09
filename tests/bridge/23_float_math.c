// ADead-BIB Bridge Test 23 — Float & Double Arithmetic
// Level: INTERMEDIATE
// Tests: float/double ops, precision, conversions, Newton sqrt, edge cases

#include <stdio.h>

int main() {
    printf("=== ADead-BIB Bridge Test 23: Float Math ===\n");
    int pass = 0, fail = 0;

    // Float addition
    float fa = 1.5f + 2.5f;
    if (fa > 3.99f && fa < 4.01f) { pass++; } else { fail++; printf("FAIL: float add %f\n", fa); }

    // Float subtraction
    float fs = 10.0f - 3.25f;
    if (fs > 6.74f && fs < 6.76f) { pass++; } else { fail++; printf("FAIL: float sub %f\n", fs); }

    // Float multiplication
    float fm = 3.0f * 4.5f;
    if (fm > 13.49f && fm < 13.51f) { pass++; } else { fail++; printf("FAIL: float mul %f\n", fm); }

    // Float division
    float fd = 10.0f / 4.0f;
    if (fd > 2.49f && fd < 2.51f) { pass++; } else { fail++; printf("FAIL: float div %f\n", fd); }

    // Double precision: 1.0/3.0 * 3.0 ~= 1.0
    double dp = (1.0 / 3.0) * 3.0;
    double eps = dp - 1.0;
    if (eps < 0.0) eps = -eps;
    if (eps < 1e-10) { pass++; } else { fail++; printf("FAIL: double precision %e\n", eps); }

    // Float to int conversion
    int fi = (int)3.14f;
    if (fi == 3) { pass++; } else { fail++; printf("FAIL: (int)3.14f = %d\n", fi); }

    // Int to float conversion
    float itf = (float)42;
    if (itf > 41.99f && itf < 42.01f) { pass++; } else { fail++; printf("FAIL: (float)42 = %f\n", itf); }

    // Negative float
    float neg = -7.5f;
    float negabs = neg < 0.0f ? -neg : neg;
    if (negabs > 7.49f && negabs < 7.51f) { pass++; } else { fail++; printf("FAIL: neg abs %f\n", negabs); }

    // Newton's method sqrt(25.0) ~= 5.0
    double x = 25.0;
    double guess = x / 2.0;
    for (int i = 0; i < 20; i++) {
        guess = (guess + x / guess) / 2.0;
    }
    double sqrtErr = guess - 5.0;
    if (sqrtErr < 0.0) sqrtErr = -sqrtErr;
    if (sqrtErr < 1e-10) { pass++; } else { fail++; printf("FAIL: sqrt(25)=%f\n", guess); }

    // Newton's method sqrt(2.0) ~= 1.41421356...
    x = 2.0;
    guess = x / 2.0;
    for (int i = 0; i < 20; i++) {
        guess = (guess + x / guess) / 2.0;
    }
    double sqrt2Err = guess - 1.41421356237;
    if (sqrt2Err < 0.0) sqrt2Err = -sqrt2Err;
    if (sqrt2Err < 1e-8) { pass++; } else { fail++; printf("FAIL: sqrt(2)=%f\n", guess); }

    // Float overflow: large * large → very large (not checking inf, just > threshold)
    float big = 1.0e30f;
    float bigger = big * 1000.0f;
    if (bigger > 1.0e32f) { pass++; } else { fail++; printf("FAIL: float overflow %e\n", bigger); }

    // Double mixed arithmetic
    double dm = 2.5 + 3 * 1.5 - 0.5;
    double dmErr = dm - 6.5;
    if (dmErr < 0.0) dmErr = -dmErr;
    if (dmErr < 1e-10) { pass++; } else { fail++; printf("FAIL: mixed arith %f\n", dm); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 23: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
