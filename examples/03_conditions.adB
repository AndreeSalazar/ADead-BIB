// ADead-BIB v3.1 - Condiciones y Operadores Lógicos
// if/else, operadores relacionales y lógicos

int main() {
    printf("=== Condiciones ===\n")
    
    int x = 10
    int y = 5
    int z = 10
    
    // Operadores relacionales
    if x > y {
        printf("x > y: verdadero\n")
    }
    
    if x < y {
        printf("x < y: verdadero\n")
    } else {
        printf("x < y: falso\n")
    }
    
    if x == z {
        printf("x == z: verdadero\n")
    }
    
    if x != y {
        printf("x != y: verdadero\n")
    }
    
    if x >= y {
        printf("x >= y: verdadero\n")
    }
    
    if y <= x {
        printf("y <= x: verdadero\n")
    }
    
    return 0
}
