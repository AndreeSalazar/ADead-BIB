// ADead-BIB FFI Cross-Language Demo
// ==================================
// Ejemplo de funciones exportables para uso desde otros lenguajes
// Author: Eddi Andreé Salazar Matos

// ============================================
// FUNCIONES EXPORTADAS (ABI C compatible)
// ============================================

#[export("C")]
int add(int a, int b) {
    return a + b
}

#[export("C")]
int multiply(int a, int b) {
    return a * b
}

#[export("C")]
int power(int base, int exp) {
    int result = 1
    int i = 0
    while i < exp {
        result = result * base
        i = i + 1
    }
    return result
}

#[export("C")]
int gcd(int a, int b) {
    while b != 0 {
        int temp = b
        b = a - (a / b) * b
        a = temp
    }
    return a
}

// ============================================
// DEMO PRINCIPAL
// ============================================

int main() {
    printf("=== ADead-BIB FFI Cross-Language Demo ===\n")
    printf("Funciones exportables para Python, Java, C, Rust\n\n")
    
    printf("add(15, 25) = ")
    printf(add(15, 25))
    printf("\n")
    
    printf("multiply(7, 8) = ")
    printf(multiply(7, 8))
    printf("\n")
    
    printf("power(2, 10) = ")
    printf(power(2, 10))
    printf("\n")
    
    printf("gcd(48, 18) = ")
    printf(gcd(48, 18))
    printf("\n")
    
    printf("\n=== Listo para FFI ===\n")
    return 0
}
