// Test: <math.h> — Math functions
// Expected: Compile + Run OK

#include <math.h>
#include <stdio.h>

int main() {
    printf("=== math.h test ===\n");

    double s = sin(3.14159 / 2.0);
    double c = cos(0.0);
    double sq = sqrt(144.0);
    double pw = pow(2.0, 10.0);
    double fl = floor(3.7);
    double ce = ceil(3.2);
    double ab = fabs(-42.5);
    double lg = log(2.718281828);

    printf("sin(pi/2)=%.2f cos(0)=%.2f\n", s, c);
    printf("sqrt(144)=%.1f pow(2,10)=%.0f\n", sq, pw);
    printf("floor(3.7)=%.1f ceil(3.2)=%.1f\n", fl, ce);
    printf("fabs(-42.5)=%.1f log(e)=%.2f\n", ab, lg);

    printf("=== math.h OK ===\n");
    return 0;
}
