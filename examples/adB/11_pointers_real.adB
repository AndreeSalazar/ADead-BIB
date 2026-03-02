// ADead-BIB v3.4 - Punteros + Bitwise + Memoria
// Punteros reales, operaciones bitwise y sizeof

int main() {
    printf("=== Punteros + Bitwise + Memoria ===\n")

    // ---- Punteros ----
    int x = 42
    int* px = &x

    printf("x = ")
    printf(x)
    printf("\n")

    *px = 100
    printf("Despues de *px = 100: x = ")
    printf(x)
    printf("\n")

    // Puntero a otro puntero
    int y = 7
    int* py = &y
    printf("\ny = ")
    printf(*py)
    printf("\n")

    // ---- Operaciones Bitwise ----
    int a = 12
    int b = 10

    printf("\nOperaciones Bitwise (a=12, b=10):\n")

    // AND bitwise
    int and_r = a & b
    printf("a & b  = ")
    printf(and_r)
    printf("\n")

    // OR bitwise
    int or_r = a | b
    printf("a | b  = ")
    printf(or_r)
    printf("\n")

    // XOR bitwise
    int xor_r = a ^ b
    printf("a ^ b  = ")
    printf(xor_r)
    printf("\n")

    // Left shift
    int shl_r = a << 2
    printf("a << 2 = ")
    printf(shl_r)
    printf("\n")

    // Right shift
    int shr_r = a >> 2
    printf("a >> 2 = ")
    printf(shr_r)
    printf("\n")

    // Bitwise NOT
    int not_r = ~a
    printf("~a     = ")
    printf(not_r)
    printf("\n")

    // ---- Memory info ----
    printf("\nMemoria y ABI:\n")
    printf("sizeof(char)  = ")
    printf(sizeof(char))
    printf(" byte\n")
    printf("sizeof(short) = ")
    printf(sizeof(short))
    printf(" bytes\n")
    printf("sizeof(int)   = ")
    printf(sizeof(int))
    printf(" bytes\n")
    printf("sizeof(long)  = ")
    printf(sizeof(long))
    printf(" bytes\n")

    return 0
}
