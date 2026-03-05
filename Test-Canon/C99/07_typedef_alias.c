// ============================================================
// Canon C99 — §6.7.7 Typedef y Alias de Tipos
// ============================================================
// Intención: typedef crea un alias — un nombre nuevo para
// un tipo existente. No crea un tipo nuevo.
// Es esencial para function pointers y tipos complejos.
//
// C99 §6.7.7: "A typedef declaration does not introduce
// a new type, only a synonym for the type so specified."
// ============================================================

#include <stdio.h>

// --- Typedef para tipos simples ---
typedef int i32;
typedef unsigned int u32;
typedef long long i64;
typedef unsigned char byte;
typedef char *string;

// --- Typedef para struct ---
typedef struct {
    int x;
    int y;
} Vec2;

typedef struct {
    Vec2 min;
    Vec2 max;
} BoundingBox;

// --- Typedef para function pointer ---
typedef int (*BinaryOp)(int, int);
typedef void (*PrintFunc)(int);

// --- Funciones para function pointers ---
int add(int a, int b) { return a + b; }
int sub(int a, int b) { return a - b; }
int mul(int a, int b) { return a * b; }

void print_result(int x) {
    printf("  result = %d\n", x);
}

int apply(BinaryOp op, int a, int b) {
    return op(a, b);
}

void apply_and_print(BinaryOp op, PrintFunc printer, int a, int b) {
    int result = op(a, b);
    printer(result);
}

int main() {
    printf("=== Canon C99: Typedef y Alias ===\n\n");

    // --- Tipos simples ---
    i32 x = 42;
    u32 y = 1000;
    i64 big = 1000000000;
    byte b = 0xFF;

    printf("i32 x = %d\n", x);
    printf("u32 y = %u\n", y);
    printf("i64 big = %lld\n", big);
    printf("byte b = 0x%02X\n", b);

    // --- Struct typedef ---
    Vec2 pos;
    pos.x = 10;
    pos.y = 20;
    printf("\nVec2 pos = (%d, %d)\n", pos.x, pos.y);

    BoundingBox box;
    box.min.x = 0;
    box.min.y = 0;
    box.max.x = 100;
    box.max.y = 50;
    printf("BoundingBox: (%d,%d)-(%d,%d)\n",
        box.min.x, box.min.y, box.max.x, box.max.y);

    // --- Function pointer typedef ---
    printf("\nFunction pointers:\n");

    BinaryOp op = add;
    printf("  add(10, 20) = %d\n", apply(op, 10, 20));

    op = sub;
    printf("  sub(50, 8)  = %d\n", apply(op, 50, 8));

    op = mul;
    printf("  mul(6, 7)   = %d\n", apply(op, 6, 7));

    // --- apply_and_print ---
    printf("\napply_and_print:\n");
    apply_and_print(add, print_result, 100, 200);
    apply_and_print(mul, print_result, 3, 14);

    // --- Array de function pointers ---
    BinaryOp ops[3];
    ops[0] = add;
    ops[1] = sub;
    ops[2] = mul;

    printf("\nArray de funciones:\n");
    int i;
    for (i = 0; i < 3; i++) {
        printf("  ops[%d](10, 5) = %d\n", i, ops[i](10, 5));
    }

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (x == 42)                { pass++; } else { printf("FAIL: i32\n"); }
    total++; if (b == 255)               { pass++; } else { printf("FAIL: byte\n"); }
    total++; if (apply(add, 10, 20) == 30) { pass++; } else { printf("FAIL: add\n"); }
    total++; if (apply(sub, 50, 8) == 42)  { pass++; } else { printf("FAIL: sub\n"); }
    total++; if (apply(mul, 6, 7) == 42)   { pass++; } else { printf("FAIL: mul\n"); }
    total++; if (ops[0](10, 5) == 15)      { pass++; } else { printf("FAIL: ops[0]\n"); }
    total++; if (ops[2](10, 5) == 50)      { pass++; } else { printf("FAIL: ops[2]\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
