#include <math.h>

int main() {
    // Exponenciales
    double a = exp(1.0);
    double b = exp2(3.0);
    double c = expm1(0.5);
    double d = log(2.718);
    double e = log2(8.0);
    double f = log10(100.0);
    double g = log1p(0.5);

    // Potencias
    double h = pow(2.0, 10.0);
    double i = sqrt(144.0);
    double j = cbrt(27.0);
    double k = hypot(3.0, 4.0);

    // Trigonometría
    double l = sin(M_PI / 2.0);
    double m = cos(0.0);
    double n = tan(M_PI / 4.0);
    double o = asin(1.0);
    double p = acos(0.0);
    double q = atan(1.0);
    double r = atan2(1.0, 1.0);

    // Hiperbólicas
    double s = sinh(1.0);
    double t = cosh(1.0);
    double u = tanh(0.5);
    double v = asinh(1.0);
    double w = acosh(2.0);
    double x = atanh(0.5);

    // Redondeo
    double y = ceil(2.3);
    double z = floor(2.7);
    double aa = round(2.5);
    double bb = trunc(2.9);
    double cc = nearbyint(2.5);
    double dd = rint(2.5);
    long ee = lround(2.5);
    long long ff = llround(2.5);
    long gg = lrint(2.5);

    // Clasificación
    int hh = fpclassify(1.0);
    int ii = isnan(0.0 / 0.0);
    int jj = isinf(1.0 / 0.0);
    int kk = isfinite(1.0);
    int ll = isnormal(1.0);
    int mm = signbit(-1.0);

    // Funciones extras
    double nn = fma(2.0, 3.0, 4.0);
    double oo = fdim(5.0, 3.0);
    double pp = fmax(1.0, 2.0);
    double qq = fmin(1.0, 2.0);
    double rr = remainder(5.3, 2.0);
    double ss = copysign(-1.0, 1.0);
    double tt = nextafter(1.0, 2.0);
    double uu = scalbn(1.0, 3);
    double vv = ldexp(1.0, 10);
    int exp_val;
    double ww = frexp(8.0, &exp_val);
    double int_part;
    double xx = modf(3.14, &int_part);
    double yy = erf(1.0);
    double zz = erfc(1.0);
    double aaa = tgamma(5.0);
    double bbb = lgamma(5.0);

    printf("exp(1) = %f\n", a);
    return 0;
}
