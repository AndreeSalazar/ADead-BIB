#include <iostream>
int gcd(int a, int b) { while (b) { int t = b; b = a % b; a = t; } return a; }
int lcm(int a, int b) { return a / gcd(a, b) * b; }
int main() { printf("gcd=%d lcm=%d\n", gcd(12, 8), lcm(12, 8)); return 0; }