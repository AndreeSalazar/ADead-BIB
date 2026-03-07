#include <stdio.h>
#include <math.h>
double deg2rad(double d) { return d * 3.14159265358979 / 180.0; }
double lerp(double a, double b, double t) { return a + (b - a) * t; }
int main() { printf("sin=%f cos=%f sqrt=%f\n", sin(deg2rad(30)), cos(deg2rad(60)), sqrt(2.0)); return 0; }