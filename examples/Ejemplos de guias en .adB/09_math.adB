// ADead-BIB v3.1 - Funciones Matemáticas
// Operaciones matemáticas básicas

int abs_val(int x) {
    if x < 0 {
        return 0 - x
    }
    return x
}

int max_val(int a, int b) {
    if a > b {
        return a
    }
    return b
}

int min_val(int a, int b) {
    if a < b {
        return a
    }
    return b
}

int factorial(int n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

int main() {
    printf("=== Funciones Matematicas ===\n")
    
    printf("abs(-5) = ")
    printf(abs_val(0 - 5))
    printf("\n")
    
    printf("max(10, 20) = ")
    printf(max_val(10, 20))
    printf("\n")
    
    printf("min(10, 20) = ")
    printf(min_val(10, 20))
    printf("\n")
    
    printf("factorial(5) = ")
    printf(factorial(5))
    printf("\n")
    
    return 0
}
