// ADead-BIB v3.4 - Punteros y Referencias
// Punteros reales con &, * y tipos puntero

int main() {
    printf("=== Punteros y Referencias ===\n")

    // Variable original
    int valor = 42
    printf("valor = ")
    printf(valor)
    printf("\n")

    // Puntero: int* apunta a int
    int* ptr = &valor
    printf("*ptr = ")
    printf(*ptr)
    printf("\n")

    // Modificar a través del puntero
    *ptr = 100
    printf("\nDespues de *ptr = 100:\n")
    printf("  valor = ")
    printf(valor)
    printf("\n")
    printf("  *ptr  = ")
    printf(*ptr)
    printf("\n")

    // sizeof
    printf("\nsizeof(int) = ")
    printf(sizeof(int))
    printf(" bytes\n")

    printf("sizeof(int*) = ")
    printf(sizeof(int*))
    printf(" bytes\n")

    return 0
}
