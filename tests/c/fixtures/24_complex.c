// Test: <complex.h> — C99 complex numbers
// Expected: Compile OK — complex type + function declarations

#include <complex.h>
#include <stdio.h>

int main() {
    printf("=== complex.h test ===\n");

    // Complex operations (declarations only — verify they parse)
    double _Complex z1;
    double _Complex z2;

    double re = creal(z1);
    double im = cimag(z1);
    double ab = cabs(z1);
    double ar = carg(z1);
    double _Complex zc = conj(z1);
    double _Complex zs = csqrt(z1);

    printf("complex types parsed OK\n");

    printf("=== complex.h OK ===\n");
    return 0;
}
