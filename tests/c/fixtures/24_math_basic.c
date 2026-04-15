// ============================================================
// Test 24: <math.h> Básico — trig, exp, log, pow, sqrt, floor, ceil
// ============================================================
// ADead-BIB Test Canon — C99 §7.12
// Verifica: funciones matemáticas (requiere float codegen)
// ============================================================

#include <stdio.h>
#include <math.h>

int main() {
    // --- Constantes ---
    double pi = 3.14159265358979323846;
    double e = 2.71828182845904523536;

    // --- Trigonometría ---
    printf("sin(0)=%.1f\n", sin(0.0));
    printf("cos(0)=%.1f\n", cos(0.0));
    printf("tan(0)=%.1f\n", tan(0.0));

    // --- Inversas ---
    printf("asin(1)=%.4f\n", asin(1.0));
    printf("acos(0)=%.4f\n", acos(0.0));
    printf("atan(1)=%.4f\n", atan(1.0));
    printf("atan2(1,1)=%.4f\n", atan2(1.0, 1.0));

    // --- Exponencial / log ---
    printf("exp(1)=%.4f\n", exp(1.0));
    printf("log(e)=%.4f\n", log(e));
    printf("log10(100)=%.1f\n", log10(100.0));

    // --- Potencia / raíz ---
    printf("pow(2,10)=%.0f\n", pow(2.0, 10.0));
    printf("sqrt(144)=%.0f\n", sqrt(144.0));
    printf("sqrt(2)=%.4f\n", sqrt(2.0));

    // --- Redondeo ---
    printf("floor(3.7)=%.0f\n", floor(3.7));
    printf("floor(-3.7)=%.0f\n", floor(-3.7));
    printf("ceil(3.2)=%.0f\n", ceil(3.2));
    printf("ceil(-3.2)=%.0f\n", ceil(-3.2));

    // --- Valor absoluto ---
    printf("fabs(-5.5)=%.1f\n", fabs(-5.5));
    printf("fabs(5.5)=%.1f\n", fabs(5.5));

    // --- Módulo ---
    printf("fmod(10.5,3)=%.1f\n", fmod(10.5, 3.0));

    // --- Hiperbólicas ---
    printf("sinh(1)=%.4f\n", sinh(1.0));
    printf("cosh(1)=%.4f\n", cosh(1.0));
    printf("tanh(1)=%.4f\n", tanh(1.0));

    // --- ldexp / frexp ---
    printf("ldexp(1,10)=%.0f\n", ldexp(1.0, 10));

    int exp;
    double mantissa = frexp(1024.0, &exp);
    printf("frexp(1024)=%.1f * 2^%d\n", mantissa, exp);

    // --- modf ---
    double ipart;
    double fpart = modf(3.75, &ipart);
    printf("modf(3.75)=int:%.0f frac:%.2f\n", ipart, fpart);

    return 0;
}
