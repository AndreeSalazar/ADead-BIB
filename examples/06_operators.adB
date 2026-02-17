// ADead-BIB v3.1 - Operadores
// Aritméticos, relacionales, lógicos, bitwise

int main() {
    printf("=== Operadores ===\n")
    
    int a = 10
    int b = 3
    
    // Operadores aritméticos
    printf("Aritmeticos:\n")
    printf("  10 + 3 = ")
    printf(a + b)
    printf("\n")
    
    printf("  10 - 3 = ")
    printf(a - b)
    printf("\n")
    
    printf("  10 * 3 = ")
    printf(a * b)
    printf("\n")
    
    printf("  10 / 3 = ")
    printf(a / b)
    printf("\n")
    
    // Operadores de asignación compuesta
    printf("Asignacion compuesta:\n")
    int x = 5
    x = x + 3
    printf("  x += 3: ")
    printf(x)
    printf("\n")
    
    x = x - 2
    printf("  x -= 2: ")
    printf(x)
    printf("\n")
    
    x = x * 2
    printf("  x *= 2: ")
    printf(x)
    printf("\n")
    
    return 0
}
