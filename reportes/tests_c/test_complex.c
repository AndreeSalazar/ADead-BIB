#include <complex.h>
#include <stdio.h>

int main() {
    double complex z1 = 1.0 + 2.0 * I;
    double complex z2 = 3.0 + 4.0 * I;
    double complex sum = z1 + z2;
    double complex prod = z1 * z2;
    double re = creal(z1);
    double im = cimag(z1);
    double mag = cabs(z1);
    double ang = carg(z1);
    double complex conj_z = conj(z1);
    double complex sq = csqrt(z1);
    double complex ex = cexp(z1);
    double complex lg = clog(z1);
    double complex pw = cpow(z1, z2);
    double complex sn = csin(z1);
    double complex cs = ccos(z1);
    printf("z1 = %f + %fi\n", re, im);
    return 0;
}
