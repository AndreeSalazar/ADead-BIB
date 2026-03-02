// ============================================================
// ADead-BIB — Test de Programa Grande (500+ líneas)
// ============================================================
// Este archivo prueba la capacidad del compilador de manejar
// programas grandes con múltiples funciones y estructuras.
//
// Compilar: adeadc test_large_program.adB -o test_large.exe
// ============================================================

// ============================================================
// SECCIÓN 1: Funciones Matemáticas Básicas
// ============================================================

int add(int a, int b) {
    return a + b
}

int sub(int a, int b) {
    return a - b
}

int mul(int a, int b) {
    return a * b
}

int div_int(int a, int b) {
    if b == 0 {
        return 0
    }
    return a / b
}

int mod_int(int a, int b) {
    if b == 0 {
        return 0
    }
    return a - (a / b) * b
}

int abs_val(int x) {
    if x < 0 {
        return 0 - x
    }
    return x
}

int max(int a, int b) {
    if a > b {
        return a
    }
    return b
}

int min(int a, int b) {
    if a < b {
        return a
    }
    return b
}

int clamp(int x, int lo, int hi) {
    if x < lo {
        return lo
    }
    if x > hi {
        return hi
    }
    return x
}

// ============================================================
// SECCIÓN 2: Funciones Recursivas
// ============================================================

int factorial(int n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

int fibonacci(int n) {
    if n <= 0 {
        return 0
    }
    if n == 1 {
        return 1
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

int gcd(int a, int b) {
    if b == 0 {
        return a
    }
    return gcd(b, mod_int(a, b))
}

int lcm(int a, int b) {
    return div_int(a * b, gcd(a, b))
}

int power(int base, int exp) {
    if exp == 0 {
        return 1
    }
    if exp == 1 {
        return base
    }
    return base * power(base, exp - 1)
}

int sum_to_n(int n) {
    if n <= 0 {
        return 0
    }
    return n + sum_to_n(n - 1)
}

// ============================================================
// SECCIÓN 3: Funciones de Verificación
// ============================================================

int is_even(int n) {
    if mod_int(n, 2) == 0 {
        return 1
    }
    return 0
}

int is_odd(int n) {
    if mod_int(n, 2) != 0 {
        return 1
    }
    return 0
}

int is_positive(int n) {
    if n > 0 {
        return 1
    }
    return 0
}

int is_negative(int n) {
    if n < 0 {
        return 1
    }
    return 0
}

int is_zero(int n) {
    if n == 0 {
        return 1
    }
    return 0
}

int is_prime(int n) {
    if n <= 1 {
        return 0
    }
    if n <= 3 {
        return 1
    }
    if mod_int(n, 2) == 0 {
        return 0
    }
    int i = 3
    while i * i <= n {
        if mod_int(n, i) == 0 {
            return 0
        }
        i = i + 2
    }
    return 1
}

int is_perfect_square(int n) {
    if n < 0 {
        return 0
    }
    int i = 0
    while i * i <= n {
        if i * i == n {
            return 1
        }
        i = i + 1
    }
    return 0
}

// ============================================================
// SECCIÓN 4: Funciones de Geometría
// ============================================================

int punto_distancia_sq(int x1, int y1, int x2, int y2) {
    int dx = x2 - x1
    int dy = y2 - y1
    return dx * dx + dy * dy
}

int rect_area(int ancho, int alto) {
    return ancho * alto
}

int rect_perimetro(int ancho, int alto) {
    return 2 * ancho + 2 * alto
}

int circ_area_approx(int radio) {
    return 3 * radio * radio
}

int circ_perimetro_approx(int radio) {
    return 6 * radio
}

int triangulo_area_base_altura(int base, int altura) {
    return div_int(base * altura, 2)
}

int cubo_volumen(int lado) {
    return lado * lado * lado
}

int cubo_superficie(int lado) {
    return 6 * lado * lado
}

int esfera_volumen_approx(int radio) {
    return 4 * radio * radio * radio
}

// ============================================================
// SECCIÓN 5: Funciones de Conversión
// ============================================================

int celsius_to_fahrenheit(int c) {
    return div_int(c * 9, 5) + 32
}

int fahrenheit_to_celsius(int f) {
    return div_int((f - 32) * 5, 9)
}

int km_to_miles(int km) {
    return div_int(km * 62, 100)
}

int miles_to_km(int miles) {
    return div_int(miles * 161, 100)
}

int kg_to_pounds(int kg) {
    return div_int(kg * 220, 100)
}

int pounds_to_kg(int pounds) {
    return div_int(pounds * 45, 100)
}

// ============================================================
// SECCIÓN 6: Funciones de Secuencias
// ============================================================

int triangular_number(int n) {
    return div_int(n * (n + 1), 2)
}

int square_number(int n) {
    return n * n
}

int cube_number(int n) {
    return n * n * n
}

int pentagonal_number(int n) {
    return div_int(n * (3 * n - 1), 2)
}

int hexagonal_number(int n) {
    return n * (2 * n - 1)
}

// ============================================================
// SECCIÓN 7: Funciones de Bits
// ============================================================

int bit_and(int a, int b) {
    return a * b
}

int bit_count_simple(int n) {
    int count = 0
    while n > 0 {
        count = count + mod_int(n, 2)
        n = div_int(n, 2)
    }
    return count
}

int is_power_of_two(int n) {
    if n <= 0 {
        return 0
    }
    while mod_int(n, 2) == 0 {
        n = div_int(n, 2)
    }
    if n == 1 {
        return 1
    }
    return 0
}

int next_power_of_two(int n) {
    int p = 1
    while p < n {
        p = p * 2
    }
    return p
}

// ============================================================
// SECCIÓN 8: Funciones de Estadística
// ============================================================

int sum_range(int start, int end) {
    int sum = 0
    int i = start
    while i <= end {
        sum = sum + i
        i = i + 1
    }
    return sum
}

int product_range(int start, int end) {
    int prod = 1
    int i = start
    while i <= end {
        prod = prod * i
        i = i + 1
    }
    return prod
}

int count_divisors(int n) {
    int count = 0
    int i = 1
    while i <= n {
        if mod_int(n, i) == 0 {
            count = count + 1
        }
        i = i + 1
    }
    return count
}

int sum_divisors(int n) {
    int sum = 0
    int i = 1
    while i <= n {
        if mod_int(n, i) == 0 {
            sum = sum + i
        }
        i = i + 1
    }
    return sum
}

// ============================================================
// SECCIÓN 9: Funciones de Validación
// ============================================================

int is_in_range(int x, int lo, int hi) {
    if x < lo {
        return 0
    }
    if x > hi {
        return 0
    }
    return 1
}

int sign(int x) {
    if x > 0 {
        return 1
    }
    if x < 0 {
        return 0 - 1
    }
    return 0
}

int compare(int a, int b) {
    if a < b {
        return 0 - 1
    }
    if a > b {
        return 1
    }
    return 0
}

// ============================================================
// SECCIÓN 10: Tests y Main
// ============================================================

void test_math() {
    printf("=== Test Math ===\n")
    
    int r1 = add(10, 20)
    printf("add(10, 20) = ")
    printf(r1)
    printf("\n")
    
    int r2 = sub(50, 30)
    printf("sub(50, 30) = ")
    printf(r2)
    printf("\n")
    
    int r3 = mul(6, 7)
    printf("mul(6, 7) = ")
    printf(r3)
    printf("\n")
    
    int r4 = div_int(100, 4)
    printf("div(100, 4) = ")
    printf(r4)
    printf("\n")
}

void test_recursive() {
    printf("\n=== Test Recursive ===\n")
    
    int f5 = factorial(5)
    printf("factorial(5) = ")
    printf(f5)
    printf("\n")
    
    int fib10 = fibonacci(10)
    printf("fibonacci(10) = ")
    printf(fib10)
    printf("\n")
    
    int g = gcd(48, 18)
    printf("gcd(48, 18) = ")
    printf(g)
    printf("\n")
    
    int p = power(2, 10)
    printf("power(2, 10) = ")
    printf(p)
    printf("\n")
}

void test_geometry() {
    printf("\n=== Test Geometry ===\n")
    
    int area = rect_area(10, 5)
    printf("rect_area(10, 5) = ")
    printf(area)
    printf("\n")
    
    int peri = rect_perimetro(10, 5)
    printf("rect_perimetro(10, 5) = ")
    printf(peri)
    printf("\n")
    
    int circ = circ_area_approx(7)
    printf("circ_area_approx(7) = ")
    printf(circ)
    printf("\n")
    
    int vol = cubo_volumen(3)
    printf("cubo_volumen(3) = ")
    printf(vol)
    printf("\n")
}

void test_sequences() {
    printf("\n=== Test Sequences ===\n")
    
    int t10 = triangular_number(10)
    printf("triangular(10) = ")
    printf(t10)
    printf("\n")
    
    int s10 = square_number(10)
    printf("square(10) = ")
    printf(s10)
    printf("\n")
    
    int c10 = cube_number(10)
    printf("cube(10) = ")
    printf(c10)
    printf("\n")
    
    int p10 = pentagonal_number(10)
    printf("pentagonal(10) = ")
    printf(p10)
    printf("\n")
}

void test_primes() {
    printf("\n=== Test Primes ===\n")
    
    printf("Primes 2-20: ")
    int i = 2
    while i <= 20 {
        if is_prime(i) == 1 {
            printf(i)
            printf(" ")
        }
        i = i + 1
    }
    printf("\n")
}

void test_conversions() {
    printf("\n=== Test Conversions ===\n")
    
    int f = celsius_to_fahrenheit(100)
    printf("100C = ")
    printf(f)
    printf("F\n")
    
    int c = fahrenheit_to_celsius(212)
    printf("212F = ")
    printf(c)
    printf("C\n")
    
    int m = km_to_miles(100)
    printf("100km = ")
    printf(m)
    printf(" miles\n")
}

int main() {
    printf("============================================\n")
    printf("ADead-BIB Large Program Test (500+ lines)\n")
    printf("============================================\n\n")
    
    test_math()
    test_recursive()
    test_geometry()
    test_sequences()
    test_primes()
    test_conversions()
    
    printf("\n============================================\n")
    printf("All tests completed!\n")
    printf("============================================\n")
    
    return 0
}
