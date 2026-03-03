/*
 * ADead-BIB Standard Library
 * math.h - Mathematical Functions
 * 
 * Based on: C99/C11 standard, musl libc
 * PDP-11 heritage: FP11 floating point unit semantics
 */

#ifndef _ADEAD_MATH_H
#define _ADEAD_MATH_H

/* Constants */
#define M_E         2.71828182845904523536
#define M_LOG2E     1.44269504088896340736
#define M_LOG10E    0.434294481903251827651
#define M_LN2       0.693147180559945309417
#define M_LN10      2.30258509299404568402
#define M_PI        3.14159265358979323846
#define M_PI_2      1.57079632679489661923
#define M_PI_4      0.785398163397448309616
#define M_1_PI      0.318309886183790671538
#define M_2_PI      0.636619772367581343076
#define M_2_SQRTPI  1.12837916709551257390
#define M_SQRT2     1.41421356237309504880
#define M_SQRT1_2   0.707106781186547524401

/* Infinity and NaN */
#define INFINITY    __builtin_inf()
#define NAN         __builtin_nan("")
#define HUGE_VAL    __builtin_huge_val()
#define HUGE_VALF   __builtin_huge_valf()
#define HUGE_VALL   __builtin_huge_vall()

/* Classification macros */
#define FP_NAN       0
#define FP_INFINITE  1
#define FP_ZERO      2
#define FP_SUBNORMAL 3
#define FP_NORMAL    4

#define fpclassify(x) __builtin_fpclassify(FP_NAN, FP_INFINITE, FP_NORMAL, FP_SUBNORMAL, FP_ZERO, x)
#define isfinite(x)   __builtin_isfinite(x)
#define isinf(x)      __builtin_isinf(x)
#define isnan(x)      __builtin_isnan(x)
#define isnormal(x)   __builtin_isnormal(x)
#define signbit(x)    __builtin_signbit(x)

/* Trigonometric functions */
double sin(double x);
double cos(double x);
double tan(double x);
double asin(double x);
double acos(double x);
double atan(double x);
double atan2(double y, double x);

float sinf(float x);
float cosf(float x);
float tanf(float x);
float asinf(float x);
float acosf(float x);
float atanf(float x);
float atan2f(float y, float x);

/* Hyperbolic functions */
double sinh(double x);
double cosh(double x);
double tanh(double x);
double asinh(double x);
double acosh(double x);
double atanh(double x);

float sinhf(float x);
float coshf(float x);
float tanhf(float x);

/* Exponential and logarithmic */
double exp(double x);
double exp2(double x);
double expm1(double x);
double log(double x);
double log10(double x);
double log2(double x);
double log1p(double x);
double ldexp(double x, int exp);
double frexp(double x, int* exp);

float expf(float x);
float exp2f(float x);
float logf(float x);
float log10f(float x);
float log2f(float x);

/* Power functions */
double pow(double base, double exp);
double sqrt(double x);
double cbrt(double x);
double hypot(double x, double y);

float powf(float base, float exp);
float sqrtf(float x);
float cbrtf(float x);
float hypotf(float x, float y);

/* Rounding and remainder */
double ceil(double x);
double floor(double x);
double trunc(double x);
double round(double x);
double nearbyint(double x);
double rint(double x);
long lrint(double x);
long long llrint(double x);
long lround(double x);
long long llround(double x);

float ceilf(float x);
float floorf(float x);
float truncf(float x);
float roundf(float x);

double fmod(double x, double y);
double remainder(double x, double y);
double remquo(double x, double y, int* quo);

float fmodf(float x, float y);

/* Floating-point manipulation */
double copysign(double x, double y);
double nan(const char* tagp);
double nextafter(double x, double y);
double nexttoward(double x, long double y);

float copysignf(float x, float y);
float nanf(const char* tagp);

/* Min, max, difference */
double fmax(double x, double y);
double fmin(double x, double y);
double fdim(double x, double y);

float fmaxf(float x, float y);
float fminf(float x, float y);
float fdimf(float x, float y);

/* Other */
double fabs(double x);
double fma(double x, double y, double z);
double scalbn(double x, int n);
double scalbln(double x, long n);
int ilogb(double x);
double logb(double x);

float fabsf(float x);
float fmaf(float x, float y, float z);

/* Error functions */
double erf(double x);
double erfc(double x);
double lgamma(double x);
double tgamma(double x);

/* Bessel functions (POSIX) */
double j0(double x);
double j1(double x);
double jn(int n, double x);
double y0(double x);
double y1(double x);
double yn(int n, double x);

#endif /* _ADEAD_MATH_H */
