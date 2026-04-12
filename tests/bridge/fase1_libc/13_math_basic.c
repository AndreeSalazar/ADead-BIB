// ADead-BIB Test: math.h basic functions (sqrt, pow, fabs, ceil, floor, round, fmod, fmin, fmax)
#include <stdio.h>
#include <math.h>

int main() {
    int pass = 0, fail = 0;

    // fabs
    double fa = fabs(-3.14);
    if (fa > 3.13 && fa < 3.15) { pass++; printf("PASS: fabs(-3.14)=%.2f\n", fa); }
    else { fail++; printf("FAIL: fabs=%.2f\n", fa); }

    // sqrt
    double sq = sqrt(144.0);
    if (sq > 11.99 && sq < 12.01) { pass++; printf("PASS: sqrt(144)=%.2f\n", sq); }
    else { fail++; printf("FAIL: sqrt=%.2f\n", sq); }

    // pow
    double pw = pow(2.0, 10.0);
    if (pw > 1023.0 && pw < 1025.0) { pass++; printf("PASS: pow(2,10)=%.0f\n", pw); }
    else { fail++; printf("FAIL: pow=%.0f\n", pw); }

    // ceil
    double ce = ceil(3.2);
    if (ce > 3.99 && ce < 4.01) { pass++; printf("PASS: ceil(3.2)=%.0f\n", ce); }
    else { fail++; printf("FAIL: ceil=%.0f\n", ce); }

    // floor
    double fl = floor(3.8);
    if (fl > 2.99 && fl < 3.01) { pass++; printf("PASS: floor(3.8)=%.0f\n", fl); }
    else { fail++; printf("FAIL: floor=%.0f\n", fl); }

    // round
    double rn = round(3.5);
    if (rn > 3.99 && rn < 4.01) { pass++; printf("PASS: round(3.5)=%.0f\n", rn); }
    else { fail++; printf("FAIL: round=%.0f\n", rn); }

    // fmod
    double fm = fmod(10.0, 3.0);
    if (fm > 0.99 && fm < 1.01) { pass++; printf("PASS: fmod(10,3)=%.2f\n", fm); }
    else { fail++; printf("FAIL: fmod=%.2f\n", fm); }

    // fmin / fmax
    double mn = fmin(3.0, 7.0);
    double mx = fmax(3.0, 7.0);
    if (mn > 2.99 && mn < 3.01) { pass++; printf("PASS: fmin(3,7)=%.0f\n", mn); }
    else { fail++; printf("FAIL: fmin=%.0f\n", mn); }
    if (mx > 6.99 && mx < 7.01) { pass++; printf("PASS: fmax(3,7)=%.0f\n", mx); }
    else { fail++; printf("FAIL: fmax=%.0f\n", mx); }

    // exp / log
    double ex = exp(1.0);
    if (ex > 2.71 && ex < 2.72) { pass++; printf("PASS: exp(1)=%.4f\n", ex); }
    else { fail++; printf("FAIL: exp=%.4f\n", ex); }

    double lg = log(ex);
    if (lg > 0.99 && lg < 1.01) { pass++; printf("PASS: log(e)=%.4f\n", lg); }
    else { fail++; printf("FAIL: log=%.4f\n", lg); }

    // log10
    double l10 = log10(1000.0);
    if (l10 > 2.99 && l10 < 3.01) { pass++; printf("PASS: log10(1000)=%.2f\n", l10); }
    else { fail++; printf("FAIL: log10=%.2f\n", l10); }

    // hypot
    double hy = hypot(3.0, 4.0);
    if (hy > 4.99 && hy < 5.01) { pass++; printf("PASS: hypot(3,4)=%.2f\n", hy); }
    else { fail++; printf("FAIL: hypot=%.2f\n", hy); }

    printf("\n=== math_basic: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
