// ADead-BIB Test: math.h trigonometric functions (sin, cos, tan, asin, acos, atan, atan2, sinh, cosh, tanh)
#include <stdio.h>
#include <math.h>

int main() {
    int pass = 0, fail = 0;
    double pi = 3.14159265358979323846;

    // sin(0) = 0
    double s = sin(0.0);
    if (s > -0.01 && s < 0.01) { pass++; printf("PASS: sin(0)=%.4f\n", s); }
    else { fail++; printf("FAIL: sin(0)=%.4f\n", s); }

    // sin(pi/2) ≈ 1.0
    s = sin(pi / 2.0);
    if (s > 0.99 && s < 1.01) { pass++; printf("PASS: sin(pi/2)=%.4f\n", s); }
    else { fail++; printf("FAIL: sin(pi/2)=%.4f\n", s); }

    // cos(0) = 1
    double c = cos(0.0);
    if (c > 0.99 && c < 1.01) { pass++; printf("PASS: cos(0)=%.4f\n", c); }
    else { fail++; printf("FAIL: cos(0)=%.4f\n", c); }

    // cos(pi) ≈ -1.0
    c = cos(pi);
    if (c > -1.01 && c < -0.99) { pass++; printf("PASS: cos(pi)=%.4f\n", c); }
    else { fail++; printf("FAIL: cos(pi)=%.4f\n", c); }

    // tan(0) = 0
    double t = tan(0.0);
    if (t > -0.01 && t < 0.01) { pass++; printf("PASS: tan(0)=%.4f\n", t); }
    else { fail++; printf("FAIL: tan(0)=%.4f\n", t); }

    // asin(1) ≈ pi/2
    double as = asin(1.0);
    if (as > 1.57 && as < 1.58) { pass++; printf("PASS: asin(1)=%.4f\n", as); }
    else { fail++; printf("FAIL: asin(1)=%.4f\n", as); }

    // acos(0) ≈ pi/2
    double ac = acos(0.0);
    if (ac > 1.57 && ac < 1.58) { pass++; printf("PASS: acos(0)=%.4f\n", ac); }
    else { fail++; printf("FAIL: acos(0)=%.4f\n", ac); }

    // atan(1) ≈ pi/4
    double at = atan(1.0);
    if (at > 0.78 && at < 0.79) { pass++; printf("PASS: atan(1)=%.4f\n", at); }
    else { fail++; printf("FAIL: atan(1)=%.4f\n", at); }

    // atan2(1,1) ≈ pi/4
    double at2 = atan2(1.0, 1.0);
    if (at2 > 0.78 && at2 < 0.79) { pass++; printf("PASS: atan2(1,1)=%.4f\n", at2); }
    else { fail++; printf("FAIL: atan2(1,1)=%.4f\n", at2); }

    // sinh(0) = 0
    double sh = sinh(0.0);
    if (sh > -0.01 && sh < 0.01) { pass++; printf("PASS: sinh(0)=%.4f\n", sh); }
    else { fail++; printf("FAIL: sinh(0)=%.4f\n", sh); }

    // cosh(0) = 1
    double ch = cosh(0.0);
    if (ch > 0.99 && ch < 1.01) { pass++; printf("PASS: cosh(0)=%.4f\n", ch); }
    else { fail++; printf("FAIL: cosh(0)=%.4f\n", ch); }

    // tanh(0) = 0
    double th = tanh(0.0);
    if (th > -0.01 && th < 0.01) { pass++; printf("PASS: tanh(0)=%.4f\n", th); }
    else { fail++; printf("FAIL: tanh(0)=%.4f\n", th); }

    // ldexp(1.0, 10) = 1024
    double ld = ldexp(1.0, 10);
    if (ld > 1023.0 && ld < 1025.0) { pass++; printf("PASS: ldexp(1,10)=%.0f\n", ld); }
    else { fail++; printf("FAIL: ldexp=%.0f\n", ld); }

    printf("\n=== math_trig: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
