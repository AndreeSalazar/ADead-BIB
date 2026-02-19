// ADead-BIB v3.1 - Punteros (Conceptual)
// Simulación de punteros con referencias

int main() {
    printf("=== Punteros (Conceptual) ===\n")
    
    // Variable original
    int valor = 42
    printf("valor = ")
    printf(valor)
    printf("\n")
    
    // Simulación de puntero (referencia)
    // En ADead-BIB, usamos variables para simular
    int ptr_valor = valor
    printf("*ptr = ")
    printf(ptr_valor)
    printf("\n")
    
    // Modificar a través de "puntero"
    ptr_valor = 100
    printf("Despues de *ptr = 100:\n")
    printf("  ptr_valor = ")
    printf(ptr_valor)
    printf("\n")
    
    // Dirección simulada (offset)
    int direccion = 1000
    printf("Direccion simulada: ")
    printf(direccion)
    printf("\n")
    
    return 0
}
