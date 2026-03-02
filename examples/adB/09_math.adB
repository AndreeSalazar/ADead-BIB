// ADead-BIB v3.4 - Funciones Matemáticas
// Implementación de funciones matemáticas puramente con tipos C

// Valor absoluto
int abs_val(int x) {
    if x < 0 {
        return 0 - x
    }
    return x
}

// Máximo de dos valores
int max_val(int a, int b) {
    if a > b {
        return a
    }
    return b
}

// Mínimo de dos valores
int min_val(int a, int b) {
    if a < b {
        return a
    }
    return b
}

// Factorial recursivo
int factorial(int n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

// Potencia: base^exp (iterativa)
int potencia(int base, int exp) {
    int result = 1
    int i = 0
    while i < exp {
        result *= base
        i += 1
    }
    return result
}

// Divisible exactamente
bool divisible(int n, int d) {
    return n % d == 0
}

// Mayor común divisor (Euclides)
int gcd(int a, int b) {
    while b != 0 {
        int temp = b
        b = a % b
        a = temp
    }
    return a
}

int main() {
    printf("=== Funciones Matematicas ===\n\n")

    printf("abs(-5)      = ")
    printf(abs_val(0 - 5))
    printf("\n")

    printf("max(10, 20)  = ")
    printf(max_val(10, 20))
    printf("\n")

    printf("min(10, 20)  = ")
    printf(min_val(10, 20))
    printf("\n")

    printf("factorial(6) = ")
    printf(factorial(6))
    printf("\n")

    printf("potencia(2,8)= ")
    printf(potencia(2, 8))
    printf("\n")

    printf("gcd(48, 18)  = ")
    printf(gcd(48, 18))
    printf("\n")

    return 0
}
