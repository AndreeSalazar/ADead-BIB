/* adeb_math.h — Floating point math */
#ifndef ADEB_MATH_H
#define ADEB_MATH_H

#define M_PI    3.14159265358979323846
#define M_E     2.71828182845904523536
#define M_SQRT2 1.41421356237309504880

extern double sin(double x);
extern double cos(double x);
extern double tan(double x);
extern double asin(double x);
extern double acos(double x);
extern double atan(double x);
extern double atan2(double y, double x);

extern double sqrt(double x);
extern double pow(double x, double y);
extern double exp(double x);
extern double log(double x);
extern double log2(double x);
extern double log10(double x);

extern double floor(double x);
extern double ceil(double x);
extern double fabs(double x);
extern double fmod(double x, double y);

#endif /* ADEB_MATH_H */
