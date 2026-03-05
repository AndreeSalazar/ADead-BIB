// ============================================================
// Canon C99 — §6.5.2.2 Punteros a Función
// ============================================================
// Intención: Un function pointer contiene la dirección de
// una función. Llamar a través de él es un `call reg`
// directo — cero overhead de abstracción.
//
// Uso canónico: callbacks, dispatch tables, strategy pattern
// en C puro sin vtables.
// ============================================================

#include <stdio.h>

// --- Operaciones aritméticas ---
int op_add(int a, int b) { return a + b; }
int op_sub(int a, int b) { return a - b; }
int op_mul(int a, int b) { return a * b; }
int op_div(int a, int b) { if (b == 0) return 0; return a / b; }
int op_mod(int a, int b) { if (b == 0) return 0; return a % b; }

// --- Tipo para function pointer ---
typedef int (*operation_fn)(int, int);

// --- Dispatch table ---
struct Operation {
    char symbol;
    operation_fn fn;
};

// --- Callback ---
typedef void (*callback_fn)(int, int, int);

void print_result(int a, int b, int result) {
    printf("  %d op %d = %d\n", a, b, result);
}

int apply(operation_fn fn, int a, int b) {
    return fn(a, b);
}

void apply_with_callback(operation_fn op, callback_fn cb, int a, int b) {
    int result = op(a, b);
    cb(a, b, result);
}

// --- Array de function pointers ---
int apply_all(operation_fn ops[], int count, int initial, int operand) {
    int result = initial;
    int i;
    for (i = 0; i < count; i++) {
        result = ops[i](result, operand);
    }
    return result;
}

// --- Comparator para sort ---
typedef int (*compare_fn)(int, int);

int cmp_asc(int a, int b) { return a - b; }
int cmp_desc(int a, int b) { return b - a; }

void sort_with(int arr[], int len, compare_fn cmp) {
    int i;
    int j;
    for (i = 0; i < len - 1; i++) {
        for (j = 0; j < len - 1 - i; j++) {
            if (cmp(arr[j], arr[j + 1]) > 0) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

int main() {
    printf("=== Canon C99: Punteros a Función ===\n\n");

    // --- Uso básico ---
    operation_fn fn = op_add;
    printf("fn = add: %d\n", fn(10, 20));

    fn = op_mul;
    printf("fn = mul: %d\n", fn(6, 7));

    // --- apply() ---
    printf("\napply:\n");
    printf("  add(10,20) = %d\n", apply(op_add, 10, 20));
    printf("  sub(50,8)  = %d\n", apply(op_sub, 50, 8));
    printf("  mul(6,7)   = %d\n", apply(op_mul, 6, 7));
    printf("  div(100,3) = %d\n", apply(op_div, 100, 3));
    printf("  mod(17,5)  = %d\n", apply(op_mod, 17, 5));

    // --- Callback ---
    printf("\nCallbacks:\n");
    apply_with_callback(op_add, print_result, 100, 200);
    apply_with_callback(op_mul, print_result, 12, 12);

    // --- Dispatch table ---
    printf("\nDispatch table:\n");
    struct Operation ops_table[4];
    ops_table[0].symbol = '+'; ops_table[0].fn = op_add;
    ops_table[1].symbol = '-'; ops_table[1].fn = op_sub;
    ops_table[2].symbol = '*'; ops_table[2].fn = op_mul;
    ops_table[3].symbol = '/'; ops_table[3].fn = op_div;

    int i;
    for (i = 0; i < 4; i++) {
        printf("  20 %c 5 = %d\n", ops_table[i].symbol, ops_table[i].fn(20, 5));
    }

    // --- Array de fns encadenados ---
    operation_fn chain[3];
    chain[0] = op_add;
    chain[1] = op_mul;
    chain[2] = op_sub;
    int chained = apply_all(chain, 3, 10, 5);
    printf("\nChain (((10+5)*5)-5) = %d\n", chained);

    // --- Sort con comparator ---
    int arr[5];
    arr[0] = 42; arr[1] = 17; arr[2] = 89; arr[3] = 3; arr[4] = 55;

    sort_with(arr, 5, cmp_asc);
    printf("\nSorted asc: ");
    for (i = 0; i < 5; i++) printf("%d ", arr[i]);
    printf("\n");

    sort_with(arr, 5, cmp_desc);
    printf("Sorted desc: ");
    for (i = 0; i < 5; i++) printf("%d ", arr[i]);
    printf("\n");

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (apply(op_add, 10, 20) == 30) { pass++; } else { printf("FAIL: add\n"); }
    total++; if (apply(op_mul, 6, 7) == 42)   { pass++; } else { printf("FAIL: mul\n"); }
    total++; if (apply(op_mod, 17, 5) == 2)   { pass++; } else { printf("FAIL: mod\n"); }
    total++; if (chained == 70)               { pass++; } else { printf("FAIL: chain\n"); }
    total++; if (arr[0] == 89)                { pass++; } else { printf("FAIL: desc[0]\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
