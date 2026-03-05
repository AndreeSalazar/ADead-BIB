// ============================================================
// ADead-BIB — ESPECIALES/math.h
// Math Functions — ADead-BIB implementation layer
// Compatible with C89/C99/C11/C++11
// ============================================================

#ifndef _ADEAD_MATH_H
#define _ADEAD_MATH_H

// ── Constants ─────────────────────────────────────────────────
#define M_PI        3.14159265358979323846
#define M_PI_2      1.57079632679489661923
#define M_PI_4      0.78539816339744830962
#define M_E         2.71828182845904523536
#define M_LOG2E     1.44269504088896340736
#define M_LOG10E    0.43429448190325182765
#define M_LN2       0.69314718055994530942
#define M_LN10      2.30258509299404568402
#define M_SQRT2     1.41421356237309504880
#define M_SQRT1_2   0.70710678118654752440
#define M_TAU       6.28318530717958647692
#define INFINITY    __builtin_inff()
#define NAN         __builtin_nanf("")
#define HUGE_VALF   __builtin_inff()
#define HUGE_VAL    __builtin_inf()
#define HUGE_VALL   __builtin_infl()
#define M_2_PI      0.63661977236758134308
#define M_2_SQRTPI  1.12837916709551257390

// ── Rounding ──────────────────────────────────────────────────
extern double floor(double x);
extern double ceil(double x);
extern double round(double x);
extern double trunc(double x);
extern double fmod(double x, double y);
extern float  floorf(float x);
extern float  ceilf(float x);
extern float  roundf(float x);
extern float  truncf(float x);
extern float  fmodf(float x, float y);

// ── Power / exponential ───────────────────────────────────────
extern double pow(double base, double exp);
extern double exp(double x);
extern double exp2(double x);
extern double expm1(double x);
extern double log(double x);
extern double log2(double x);
extern double log10(double x);
extern double log1p(double x);
extern double sqrt(double x);
extern double cbrt(double x);
extern double hypot(double x, double y);
extern float  powf(float base, float exp);
extern float  expf(float x);
extern float  logf(float x);
extern float  sqrtf(float x);
extern float  hypotf(float x, float y);

// ── Trig ─────────────────────────────────────────────────────
extern double sin(double x);
extern double cos(double x);
extern double tan(double x);
extern double asin(double x);
extern double acos(double x);
extern double atan(double x);
extern double atan2(double y, double x);
extern double sinh(double x);
extern double cosh(double x);
extern double tanh(double x);
extern double asinh(double x);
extern double acosh(double x);
extern double atanh(double x);
extern float  sinf(float x);
extern float  cosf(float x);
extern float  tanf(float x);
extern float  asinf(float x);
extern float  acosf(float x);
extern float  atanf(float x);
extern float  atan2f(float y, float x);

// ── Absolute value / sign ─────────────────────────────────────
extern double fabs(double x);
extern float  fabsf(float x);
extern double copysign(double mag, double sgn);
extern float  copysignf(float mag, float sgn);
extern double fdim(double x, double y);
extern double fmax(double x, double y);
extern double fmin(double x, double y);
extern float  fmaxf(float x, float y);
extern float  fminf(float x, float y);

// ── Classification ────────────────────────────────────────────
extern int    isnan(double x);
extern int    isinf(double x);
extern int    isfinite(double x);
extern int    isnormal(double x);
extern int    signbit(double x);
extern int    fpclassify(double x);

// ── Integer math ─────────────────────────────────────────────
extern int    abs(int x);
extern long   labs(long x);
extern long long llabs(long long x);

// ── FP manipulation ──────────────────────────────────────────
extern double frexp(double x, int* exp);
extern double ldexp(double x, int exp);
extern double modf(double x, double* iptr);
extern double scalbn(double x, int n);
extern double scalbln(double x, long n);
extern int    ilogb(double x);
extern double logb(double x);
extern double nextafter(double x, double y);
extern double nexttoward(double x, long double y);
extern double remquo(double x, double y, int* quo);
extern double remainder(double x, double y);
extern double fma(double x, double y, double z);

#endif // _ADEAD_MATH_H
