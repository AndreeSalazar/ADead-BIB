// ============================================================
// ADead-BIB C Compiler — Hello World Showcase
// ============================================================
// Demuestra TODO lo que el frontend C de ADead-BIB puede compilar:
//   - Funciones, variables, tipos
//   - Control de flujo: if/else, for, while, do-while, switch
//   - Structs, enums, typedef
//   - Punteros y arrays
//   - Operaciones bitwise
//   - Printf con format specifiers
//   - Expresiones ternarias
//   - Asignaciones compuestas (+=, -=, etc.)
//
// Compilar: adeadc hello.c -o hello.exe
// Sin GCC. Sin Clang. Solo ADead-BIB. 💀🦈
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// ==================== Structs ====================

struct Point {
    int x;
    int y;
};

struct Color {
    unsigned char r;
    unsigned char g;
    unsigned char b;
    unsigned char a;
};

struct Rect {
    struct Point origin;
    int width;
    int height;
};

// ==================== Enums ====================

enum Direction {
    DIR_NORTH = 0,
    DIR_EAST  = 1,
    DIR_SOUTH = 2,
    DIR_WEST  = 3
};

enum LogLevel {
    LOG_DEBUG,
    LOG_INFO,
    LOG_WARN,
    LOG_ERROR,
    LOG_FATAL
};

// ==================== Typedefs ====================

typedef unsigned int uint;
typedef unsigned char byte;
typedef long long i64;

// ==================== Funciones ====================

int add(int a, int b) {
    return a + b;
}

int multiply(int a, int b) {
    return a * b;
}

int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

int fibonacci(int n) {
    if (n <= 0) return 0;
    if (n == 1) return 1;
    int a = 0;
    int b = 1;
    for (int i = 2; i <= n; i++) {
        int temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

int max(int a, int b) {
    return (a > b) ? a : b;
}

int min(int a, int b) {
    return (a < b) ? a : b;
}

int clamp(int val, int lo, int hi) {
    return (val < lo) ? lo : (val > hi) ? hi : val;
}

int abs_val(int x) {
    return (x < 0) ? -x : x;
}

int gcd(int a, int b) {
    while (b != 0) {
        int temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

int is_prime(int n) {
    if (n < 2) return 0;
    if (n < 4) return 1;
    if (n % 2 == 0) return 0;
    for (int i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return 0;
    }
    return 1;
}

// ==================== Punteros ====================

void swap(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

int sum_array(int *arr, int len) {
    int total = 0;
    for (int i = 0; i < len; i++) {
        total += arr[i];
    }
    return total;
}

void reverse_array(int *arr, int len) {
    for (int i = 0; i < len / 2; i++) {
        swap(&arr[i], &arr[len - 1 - i]);
    }
}

// ==================== Strings ====================

int string_length(const char *s) {
    int len = 0;
    while (s[len] != '\0') {
        len++;
    }
    return len;
}

int string_equal(const char *a, const char *b) {
    while (*a && *b) {
        if (*a != *b) return 0;
        a++;
        b++;
    }
    return *a == *b;
}

// ==================== Bitwise ====================

unsigned int set_bit(unsigned int val, int bit) {
    return val | (1 << bit);
}

unsigned int clear_bit(unsigned int val, int bit) {
    return val & ~(1 << bit);
}

unsigned int toggle_bit(unsigned int val, int bit) {
    return val ^ (1 << bit);
}

int test_bit(unsigned int val, int bit) {
    return (val >> bit) & 1;
}

unsigned int count_bits(unsigned int val) {
    unsigned int count = 0;
    while (val) {
        count += val & 1;
        val >>= 1;
    }
    return count;
}

// ==================== Main ====================

int main() {
    printf("=== ADead-BIB C Compiler ===\n");
    printf("Sin GCC. Sin Clang. Solo ADead-BIB.\n\n");

    // --- Variables y aritmética ---
    int x = 10;
    int y = 20;
    int result = add(x, y);
    printf("add(%d, %d) = %d\n", x, y, result);
    printf("multiply(%d, %d) = %d\n", x, y, multiply(x, y));
    printf("factorial(7) = %d\n", factorial(7));
    printf("fibonacci(10) = %d\n", fibonacci(10));
    printf("gcd(48, 18) = %d\n", gcd(48, 18));

    // --- Primos ---
    printf("\nPrimos hasta 30: ");
    for (int i = 2; i <= 30; i++) {
        if (is_prime(i)) {
            printf("%d ", i);
        }
    }
    printf("\n");

    // --- Control de flujo ---
    printf("\nControl de flujo:\n");
    if (result > 25) {
        printf("  %d > 25\n", result);
    } else {
        printf("  %d <= 25\n", result);
    }

    // Switch
    enum Direction dir = DIR_EAST;
    printf("  Direction: ");
    switch (dir) {
        case DIR_NORTH: printf("North\n"); break;
        case DIR_EAST:  printf("East\n"); break;
        case DIR_SOUTH: printf("South\n"); break;
        case DIR_WEST:  printf("West\n"); break;
        default:        printf("Unknown\n"); break;
    }

    // For loop
    printf("\nFor loop (cuadrados): ");
    for (int i = 1; i <= 5; i++) {
        printf("%d ", i * i);
    }
    printf("\n");

    // While loop
    int n = 1;
    printf("While (potencias de 2): ");
    while (n <= 128) {
        printf("%d ", n);
        n *= 2;
    }
    printf("\n");

    // Do-while
    int count = 0;
    do {
        count++;
    } while (count < 5);
    printf("Do-while result: %d\n", count);

    // --- Ternario ---
    int val = 42;
    printf("\nTernario: %d es %s\n", val, (val % 2 == 0) ? "par" : "impar");
    printf("max(%d, %d) = %d\n", 15, 30, max(15, 30));
    printf("clamp(150, 0, 100) = %d\n", clamp(150, 0, 100));

    // --- Punteros ---
    printf("\nPunteros:\n");
    int a = 100;
    int b = 200;
    printf("  Antes swap: a=%d, b=%d\n", a, b);
    swap(&a, &b);
    printf("  Despues swap: a=%d, b=%d\n", a, b);

    // --- Arrays ---
    printf("\nArrays:\n");
    int arr[8];
    for (int i = 0; i < 8; i++) {
        arr[i] = (i + 1) * 10;
    }
    printf("  Array: ");
    for (int i = 0; i < 8; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
    printf("  Sum: %d\n", sum_array(arr, 8));

    reverse_array(arr, 8);
    printf("  Reversed: ");
    for (int i = 0; i < 8; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");

    // --- Structs ---
    printf("\nStructs:\n");
    struct Point p1;
    p1.x = 10;
    p1.y = 20;
    printf("  Point: (%d, %d)\n", p1.x, p1.y);

    struct Color red;
    red.r = 255;
    red.g = 0;
    red.b = 0;
    red.a = 255;
    printf("  Color: rgba(%d, %d, %d, %d)\n", red.r, red.g, red.b, red.a);

    // --- Bitwise ---
    printf("\nBitwise:\n");
    unsigned int flags = 0;
    flags = set_bit(flags, 0);
    flags = set_bit(flags, 3);
    flags = set_bit(flags, 7);
    printf("  Flags: 0x%x\n", flags);
    printf("  Bit 3: %d\n", test_bit(flags, 3));
    printf("  Bit 4: %d\n", test_bit(flags, 4));
    printf("  Popcount: %d\n", count_bits(flags));

    flags = toggle_bit(flags, 3);
    printf("  After toggle bit 3: 0x%x\n", flags);

    // --- Asignaciones compuestas ---
    printf("\nCompound assigns:\n");
    int v = 100;
    v += 50;
    printf("  += 50 → %d\n", v);
    v -= 30;
    printf("  -= 30 → %d\n", v);
    v *= 2;
    printf("  *= 2  → %d\n", v);
    v /= 3;
    printf("  /= 3  → %d\n", v);
    v %= 7;
    printf("  %%= 7  → %d\n", v);

    // --- Strings ---
    printf("\nStrings:\n");
    const char *hello = "ADead-BIB";
    printf("  String: %s\n", hello);
    printf("  Length: %d\n", string_length(hello));
    printf("  Equal test: %d\n", string_equal("abc", "abc"));
    printf("  Not equal: %d\n", string_equal("abc", "xyz"));

    // --- Memoria dinámica ---
    printf("\nMemoria dinamica:\n");
    int *heap = malloc(5 * sizeof(int));
    if (heap != NULL) {
        for (int i = 0; i < 5; i++) {
            heap[i] = i * i;
        }
        printf("  Heap array: ");
        for (int i = 0; i < 5; i++) {
            printf("%d ", heap[i]);
        }
        printf("\n");
        free(heap);
        printf("  Freed OK\n");
    }

    printf("\n=== ADead-BIB Complete ===\n");
    return 0;
}
