/*
 * ADead-BIB Standard Library
 * complex.h - Complex Number Arithmetic
 * 
 * Based on: C99/C11
 */

#ifndef _ADEAD_COMPLEX_H
#define _ADEAD_COMPLEX_H

/* Complex type */
#define complex _Complex
#define _Complex_I (__extension__ 1.0fi)
#define I _Complex_I

/* Type-generic macros */
#define CMPLX(x, y)  ((double complex)((double)(x) + _Complex_I * (double)(y)))
#define CMPLXF(x, y) ((float complex)((float)(x) + _Complex_I * (float)(y)))
#define CMPLXL(x, y) ((long double complex)((long double)(x) + _Complex_I * (long double)(y)))

/* Trigonometric */
double complex cacos(double complex z);
double complex casin(double complex z);
double complex catan(double complex z);
double complex ccos(double complex z);
double complex csin(double complex z);
double complex ctan(double complex z);

float complex cacosf(float complex z);
float complex casinf(float complex z);
float complex catanf(float complex z);
float complex ccosf(float complex z);
float complex csinf(float complex z);
float complex ctanf(float complex z);

/* Hyperbolic */
double complex cacosh(double complex z);
double complex casinh(double complex z);
double complex catanh(double complex z);
double complex ccosh(double complex z);
double complex csinh(double complex z);
double complex ctanh(double complex z);

float complex cacoshf(float complex z);
float complex casinhf(float complex z);
float complex catanhf(float complex z);
float complex ccoshf(float complex z);
float complex csinhf(float complex z);
float complex ctanhf(float complex z);

/* Exponential and logarithmic */
double complex cexp(double complex z);
double complex clog(double complex z);

float complex cexpf(float complex z);
float complex clogf(float complex z);

/* Power and absolute */
double cabs(double complex z);
double complex cpow(double complex x, double complex y);
double complex csqrt(double complex z);

float cabsf(float complex z);
float complex cpowf(float complex x, float complex y);
float complex csqrtf(float complex z);

/* Manipulation */
double carg(double complex z);
double cimag(double complex z);
double creal(double complex z);
double complex conj(double complex z);
double complex cproj(double complex z);

float cargf(float complex z);
float cimagf(float complex z);
float crealf(float complex z);
float complex conjf(float complex z);
float complex cprojf(float complex z);

#endif /* _ADEAD_COMPLEX_H */
