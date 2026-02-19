// ADead-BIB v3.1 - Funciones
// Definición y llamada de funciones

// Función simple sin parámetros
void saludar() {
    printf("Hola desde una funcion!\n")
}

// Función con parámetros
void mostrar_numero(int n) {
    printf("El numero es: ")
    printf(n)
    printf("\n")
}

// Función con retorno
int sumar(int a, int b) {
    return a + b
}

int multiplicar(int x, int y) {
    return x * y
}

// Función principal
int main() {
    printf("=== Funciones ===\n")
    
    saludar()
    
    mostrar_numero(42)
    
    int resultado = sumar(10, 20)
    printf("10 + 20 = ")
    printf(resultado)
    printf("\n")
    
    int producto = multiplicar(5, 6)
    printf("5 * 6 = ")
    printf(producto)
    printf("\n")
    
    return 0
}
