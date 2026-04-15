// ============================================================
// Test 02: Operaciones Aritméticas — +, -, *, /, %, unary
// ============================================================
// ADead-BIB Test Canon — C99 §6.5
// Verifica: todas las operaciones aritméticas básicas
// ============================================================

#include <stdio.h>

int main() {
    // --- Suma ---
    int a = 10 + 5;
    int b = -3 + 8;
    int c = 0 + 0;

    // --- Resta ---
    int d = 10 - 5;
    int e = 5 - 10;
    int f = 0 - 0;

    // --- Multiplicación ---
    int g = 6 * 7;
    int h = -3 * 4;
    int i = 0 * 999;

    // --- División ---
    int j = 20 / 4;
    int k = 7 / 2;
    int l = -7 / 2;

    // --- Módulo ---
    int m = 10 % 3;
    int n = 7 % 7;
    int o = -10 % 3;

    // --- Unary ---
    int p = -42;
    int q = +42;
    int r = -(-42);

    // --- Incremento/Decremento ---
    int x = 0;
    x++;
    ++x;
    x--;
    --x;
    int pre = ++x;
    int post = x++;

    // --- Compound assignment ---
    int y = 100;
    y += 10;
    y -= 5;
    y *= 2;
    y /= 3;
    y %= 7;

    printf("a=%d b=%d c=%d\n", a, b, c);
    printf("d=%d e=%d f=%d\n", d, e, f);
    printf("g=%d h=%d i=%d\n", g, h, i);
    printf("j=%d k=%d l=%d\n", j, k, l);
    printf("m=%d n=%d o=%d\n", m, n, o);
    printf("p=%d q=%d r=%d\n", p, q, r);
    printf("pre=%d post=%d x=%d\n", pre, post, x);
    printf("y=%d\n", y);

    return 0;
}
// Expected output:
// a=15 b=5 c=0
// d=5 e=-5 f=0
// g=42 h=-12 i=0
// j=5 k=3 l=-3
// m=1 n=0 o=-1
// p=-42 q=42 r=42
// pre=1 post=1 x=2
// y=4
