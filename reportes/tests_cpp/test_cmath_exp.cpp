#include <cmath>
#include <iostream>

int main() {
    // Exponenciales — el caso que falla
    double a = std::exp(1.0);
    double b = std::exp2(3.0);
    double c = std::expm1(0.5);
    double d = std::log(2.718);
    double e = std::log2(8.0);
    double f = std::log10(100.0);
    double g = std::log1p(0.5);
    double h = std::pow(2.0, 10.0);
    double i = std::sqrt(144.0);
    double j = std::cbrt(27.0);
    double k = std::hypot(3.0, 4.0);

    // Hiperbólicas
    double l = std::sinh(1.0);
    double m = std::cosh(1.0);
    double n = std::tanh(0.5);
    double o = std::asinh(1.0);
    double p = std::acosh(2.0);
    double q = std::atanh(0.5);

    // Extras
    double r = std::fma(2.0, 3.0, 4.0);
    double s = std::erf(1.0);
    double t = std::erfc(1.0);
    double u = std::tgamma(5.0);
    double v = std::lgamma(5.0);

    std::cout << "exp(1) = " << a << std::endl;
    std::cout << "pow(2,10) = " << h << std::endl;
    return 0;
}
