// ============================================================
// Test 15: Punteros a Función — callbacks, dispatch, vtable pattern
// ============================================================
// ADead-BIB Test Canon — C99 §6.5.2.2
// Verifica: fn ptr decl, call via ptr, arrays de fn ptrs, typedef
// ============================================================

#include <stdio.h>

// --- Funciones simples para apuntar ---
int add(int a, int b) { return a + b; }
int sub(int a, int b) { return a - b; }
int mul(int a, int b) { return a * b; }
int divide(int a, int b) { return (b != 0) ? a / b : 0; }
int modulo(int a, int b) { return (b != 0) ? a % b : 0; }

// --- typedef para fn ptr ---
typedef int (*BinaryOp)(int, int);

// --- Comparadores para sort ---
int compare_asc(const void *a, const void *b) {
    return *(const int *)a - *(const int *)b;
}

int compare_desc(const void *a, const void *b) {
    return *(const int *)b - *(const int *)a;
}

// --- apply: recibe fn ptr como parámetro ---
int apply(BinaryOp op, int a, int b) {
    return op(a, b);
}

// --- map: aplica fn a cada elemento ---
void map_array(int *arr, int n, int (*fn)(int, int), int operand) {
    int i;
    for (i = 0; i < n; i++) {
        arr[i] = fn(arr[i], operand);
    }
}

// --- Dispatch table ---
typedef struct {
    const char *name;
    BinaryOp func;
} Operation;

int dispatch(Operation *ops, int n_ops, const char *name, int a, int b) {
    int i;
    for (i = 0; i < n_ops; i++) {
        int j = 0;
        const char *s1 = ops[i].name;
        const char *s2 = name;
        while (s1[j] && s2[j] && s1[j] == s2[j]) j++;
        if (s1[j] == '\0' && s2[j] == '\0') {
            return ops[i].func(a, b);
        }
    }
    return 0;
}

// --- Retornar function pointer ---
BinaryOp get_operation(char op) {
    switch (op) {
        case '+': return add;
        case '-': return sub;
        case '*': return mul;
        case '/': return divide;
        default:  return add;
    }
}

int main() {
    // --- Fn ptr básico ---
    int (*fp)(int, int) = add;
    printf("fp(3,4)=%d\n", fp(3, 4));

    fp = sub;
    printf("fp(10,3)=%d\n", fp(10, 3));

    // --- typedef fn ptr ---
    BinaryOp op = mul;
    printf("op(6,7)=%d\n", op(6, 7));

    // --- apply ---
    printf("apply(add,10,20)=%d\n", apply(add, 10, 20));
    printf("apply(mul,5,6)=%d\n", apply(mul, 5, 6));

    // --- Array de fn ptrs ---
    BinaryOp ops[4] = {add, sub, mul, divide};
    const char *names[4] = {"add", "sub", "mul", "div"};
    int i;
    for (i = 0; i < 4; i++) {
        printf("%s(10,3)=%d\n", names[i], ops[i](10, 3));
    }

    // --- map_array ---
    int data[5] = {1, 2, 3, 4, 5};
    map_array(data, 5, mul, 10);
    printf("mapped: %d %d %d %d %d\n",
           data[0], data[1], data[2], data[3], data[4]);

    // --- Dispatch table ---
    Operation calc_ops[5];
    calc_ops[0].name = "add"; calc_ops[0].func = add;
    calc_ops[1].name = "sub"; calc_ops[1].func = sub;
    calc_ops[2].name = "mul"; calc_ops[2].func = mul;
    calc_ops[3].name = "div"; calc_ops[3].func = divide;
    calc_ops[4].name = "mod"; calc_ops[4].func = modulo;

    printf("dispatch add=%d\n", dispatch(calc_ops, 5, "add", 100, 30));
    printf("dispatch mul=%d\n", dispatch(calc_ops, 5, "mul", 6, 7));

    // --- get_operation (retorna fn ptr) ---
    BinaryOp plus = get_operation('+');
    BinaryOp times = get_operation('*');
    printf("plus(5,3)=%d times(5,3)=%d\n", plus(5, 3), times(5, 3));

    return 0;
}
// Expected:
// fp(3,4)=7
// fp(10,3)=7
// op(6,7)=42
// apply(add,10,20)=30
// apply(mul,5,6)=30
// add(10,3)=13
// sub(10,3)=7
// mul(10,3)=30
// div(10,3)=3
// mapped: 10 20 30 40 50
// dispatch add=130
// dispatch mul=42
// plus(5,3)=8 times(5,3)=15
