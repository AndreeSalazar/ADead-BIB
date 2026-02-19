// ADead-BIB ABI Principal v3.2
// Definiciones ABI estándar para interoperabilidad universal
// Author: Eddi Andreé Salazar Matos

// ============================================
// TIPOS EXPORTADOS (compatibles con C ABI)
// ============================================

// Función exportada: sumar dos enteros
#[export("C")]
int adead_sum(int a, int b) {
    return a + b
}

// Función exportada: multiplicar
#[export("C")]
int adead_mul(int a, int b) {
    return a * b
}

// Función exportada: máximo
#[export("C")]
int adead_max(int a, int b) {
    if a > b {
        return a
    }
    return b
}

// Función exportada: mínimo
#[export("C")]
int adead_min(int a, int b) {
    if a < b {
        return a
    }
    return b
}

// Función exportada: valor absoluto
#[export("C")]
int adead_abs(int x) {
    if x < 0 {
        return 0 - x
    }
    return x
}

// Función exportada: factorial
#[export("C")]
int adead_factorial(int n) {
    if n <= 1 {
        return 1
    }
    return n * adead_factorial(n - 1)
}

// Función exportada: fibonacci
#[export("C")]
int adead_fib(int n) {
    if n <= 1 {
        return n
    }
    return adead_fib(n - 1) + adead_fib(n - 2)
}

// ============================================
// FUNCIONES DE MEMORIA
// ============================================

// Función exportada: sizeof tipo
#[export("C")]
int adead_sizeof_int() {
    return 8
}

// ============================================
// FUNCIONES DE UTILIDAD
// ============================================

// Función exportada: versión
#[export("C")]
int adead_version() {
    return 320
}

// Función principal para pruebas
int main() {
    printf("=== ADead-BIB ABI Test ===\n")
    
    printf("adead_sum(10, 20) = ")
    printf(adead_sum(10, 20))
    printf("\n")
    
    printf("adead_mul(5, 6) = ")
    printf(adead_mul(5, 6))
    printf("\n")
    
    printf("adead_max(15, 8) = ")
    printf(adead_max(15, 8))
    printf("\n")
    
    printf("adead_abs(-42) = ")
    printf(adead_abs(0 - 42))
    printf("\n")
    
    printf("adead_factorial(5) = ")
    printf(adead_factorial(5))
    printf("\n")
    
    printf("adead_version() = ")
    printf(adead_version())
    printf("\n")
    
    return 0
}
