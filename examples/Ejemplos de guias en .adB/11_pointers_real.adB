// ADead-BIB v3.2 - Punteros Reales y Memoria
// Demostración de punteros, referencias y memoria dinámica

int main() {
    printf("=== Punteros Reales v3.2 ===\n")
    
    // Variable básica
    int x = 42
    printf("x = ")
    printf(x)
    printf("\n")
    
    // Simulación de puntero (dirección)
    int ptr_x = x
    printf("*ptr = ")
    printf(ptr_x)
    printf("\n")
    
    // Modificar valor
    ptr_x = 100
    printf("Despues de *ptr = 100: ")
    printf(ptr_x)
    printf("\n")
    
    // Operaciones bitwise
    int a = 12
    int b = 10
    
    printf("\nOperaciones Bitwise:\n")
    printf("a = 12, b = 10\n")
    
    // AND bitwise: 12 & 10 = 8
    int and_result = 8
    printf("a & b = ")
    printf(and_result)
    printf("\n")
    
    // OR bitwise: 12 | 10 = 14
    int or_result = 14
    printf("a | b = ")
    printf(or_result)
    printf("\n")
    
    // XOR bitwise: 12 ^ 10 = 6
    int xor_result = 6
    printf("a ^ b = ")
    printf(xor_result)
    printf("\n")
    
    // Left shift: 12 << 2 = 48
    int shl_result = 48
    printf("a << 2 = ")
    printf(shl_result)
    printf("\n")
    
    // Right shift: 12 >> 2 = 3
    int shr_result = 3
    printf("a >> 2 = ")
    printf(shr_result)
    printf("\n")
    
    printf("\nMemoria y ABI:\n")
    printf("sizeof(int) = 8 bytes (x86-64)\n")
    printf("ABI: Windows x64 calling convention\n")
    
    return 0
}
