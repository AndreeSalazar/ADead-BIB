// ADead-BIB v3.0 - Control de Flujo (C-style)
// if/else, while con sintaxis C

int main() {
    printf("=== Control de Flujo C-style ===\n")
    
    int x = 10
    int y = 5
    
    if x > y {
        printf("x es mayor que y\n")
    } else {
        printf("x no es mayor que y\n")
    }
    
    printf("Contando con while:\n")
    int i = 0
    while i < 3 {
        printf("i = ")
        printf(i)
        printf("\n")
        i = i + 1
    }
    
    printf("Fin!\n")
    return 0
}
