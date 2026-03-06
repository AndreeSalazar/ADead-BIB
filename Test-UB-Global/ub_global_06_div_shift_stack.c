#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

void div_literal() {
    int x = 10 / 0;             // UB: DivisionByZero
    printf("%d\n", x);
}

void div_modulo() {
    int x = 10 % 0;             // UB: mismo que division
    printf("%d\n", x);
}

void div_variable(int b) {
    int a = 100;
    int c = a / b;              // UB: si b = 0
    printf("%d\n", c);
}

void div_calculado() {
    int arr[5] = {1, 2, 0, 4, 5};
    int divisor = arr[2];       // = 0
    int result = 100 / divisor; // UB: divisor = 0
    printf("%d\n", result);
}

void div_intmin() {
    int x = INT_MIN;
    int y = x / -1;             // UB: resultado > INT_MAX
    printf("%d\n", y);
}

void shift_negativo() {
    int x = 8;
    int y = x >> -1;            // UB: shift negativo
    printf("%d\n", y);
}

void shift_mayor_width() {
    int x = 1;
    int y = x << 32;            // UB: 32 >= sizeof(int)*8
    int z = x << 31;            // UB en signed: shift into sign bit
    printf("%d %d\n", y, z);
}

void shift_variable(int bits) {
    int x = 1;
    int y = x << bits;          // UB: si bits >= 32 o bits < 0
    printf("%d\n", y);
}

void shift_negativo_valor() {
    int x = -1;
    int y = x << 1;             // UB: left shift de signed negativo
    printf("%d\n", y);
}

void recursion_infinita() {
    recursion_infinita();       // UB: stack overflow
}

void b_infinita();
void a_infinita() { b_infinita(); }
void b_infinita() { a_infinita(); } // UB: ciclo infinito

int fibonacci_roto(int n) {
    if (n < 0) return 0;        // caso base nunca alcanzado
    return fibonacci_roto(n+1) + fibonacci_roto(n+2); // UB: n crece
}

void string_literal_modify() {
    char *str = "hello";
    str[0] = 'H';               // UB: string literal en ROM
}

void sequence_point() {
    int i = 0;
    int j = i++ + i++;          // UB: orden de evaluacion
    printf("%d\n", j);
}

void ptr_comparison() {
    int a = 1;
    int b = 2;
    int *pa = &a;
    int *pb = &b;
    if (pa < pb) {              // UB: comparar punteros a objetos distintos
        printf("a antes que b\n");
    }
}

void memcpy_overlap() {
    char buf[20] = "hello world";
    memcpy(buf + 2, buf, 10);   // UB: source y dest se solapan
    printf("%s\n", buf);
}

int* vla_escape(int n) {
    int arr[n];                 // VLA
    return arr;                 // UB: VLA destruido al salir
}

void div_limpio(int a, int b) {
    if (b == 0) {               // check siempre ✅
        printf("div_limpio: division por cero evitada\n");
        return;
    }
    if (a == INT_MIN && b == -1) {  // caso especial ✅
        printf("div_limpio: INT_MIN/-1 evitado\n");
        return;
    }
    printf("div_limpio: %d / %d = %d\n", a, b, a / b);
}

void shift_limpio(int x, int bits) {
    if (bits < 0 || bits >= 31) {   // check ✅ (31 para signed)
        printf("shift_limpio: shift inválido evitado\n");
        return;
    }
    if (x < 0) {                     // no shift signed negativo ✅
        printf("shift_limpio: shift de negativo evitado\n");
        return;
    }
    int result = x << bits;
    printf("shift_limpio: %d << %d = %d\n", x, bits, result);
}

int fibonacci_limpio(int n) {
    if (n <= 0) return 0;       // caso base ✅
    if (n == 1) return 1;       // caso base ✅
    if (n > 40) return -1;      // limite de seguridad ✅
    return fibonacci_limpio(n-1) + fibonacci_limpio(n-2);
}

void string_mutable_limpio() {
    char str[] = "hello";       // array, no literal ✅
    str[0] = 'H';               // seguro ✅
    printf("string_limpio: %s\n", str);
}

void memcpy_limpio() {
    char src[20] = "hello world";
    char dst[20] = {0};
    memcpy(dst, src, strlen(src) + 1);  // no overlap ✅
    printf("memcpy_limpio: %s\n", dst);
}

int main() {
    div_limpio(100, 3);
    div_limpio(100, 0);
    div_limpio(INT_MIN, -1);
    shift_limpio(1, 4);
    shift_limpio(1, 35);
    shift_limpio(-1, 2);
    printf("fibonacci(10) = %d\n", fibonacci_limpio(10));
    string_mutable_limpio();
    memcpy_limpio();

    printf("\nmain: ok\n");
    return 0;
}
