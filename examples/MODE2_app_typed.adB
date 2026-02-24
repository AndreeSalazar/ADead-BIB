// ============================================================
// ADead-BIB — MODO 2: Aplicación con Tipado C+Python
// ============================================================
// Este modo genera ejecutables PE/ELF con sintaxis tipada.
// Ideal para: aplicaciones, herramientas CLI, scripts compilados.
//
// Compilar Windows: adeadc MODE2_app_typed.adB -o app.exe
// Compilar Linux:   adeadc MODE2_app_typed.adB -o app --linux
//
// Sintaxis: C-style sin punto y coma, llaves para bloques
// ============================================================

// ---- Función con tipos ----
int add(int a, int b) {
    return a + b
}

int multiply(int x, int y) {
    return x * y
}

// ---- Factorial recursivo ----
int factorial(int n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

// ---- Demo de control de flujo ----
void control_flow_demo() {
    printf("Control Flow Demo:\n")
    
    int x = 10
    if x > 5 {
        printf("x > 5: verdadero\n")
    } else {
        printf("x > 5: falso\n")
    }
    
    // While loop
    printf("While loop:\n")
    int count = 0
    while count < 3 {
        printf("  count = ")
        printf(count)
        printf("\n")
        count = count + 1
    }
}

// ---- Demo de operaciones matemáticas ----
void math_demo() {
    printf("\nMath Operations:\n")
    
    int a = 12
    int b = 10
    
    int suma = a + b
    printf("12 + 10 = ")
    printf(suma)
    printf("\n")
    
    int resta = a - b
    printf("12 - 10 = ")
    printf(resta)
    printf("\n")
    
    int mult = a * b
    printf("12 * 10 = ")
    printf(mult)
    printf("\n")
    
    int div = a / b
    printf("12 / 10 = ")
    printf(div)
    printf("\n")
}

// ---- Demo de nuevas características v3.4 ----
void new_features_demo() {
    printf("\nNew Features v3.4:\n")
    
    // Compound assignments (+=, -=, *=, /=)
    int counter = 100
    counter += 50
    printf("100 += 50 = ")
    printf(counter)
    printf("\n")
    
    counter -= 30
    printf("150 -= 30 = ")
    printf(counter)
    printf("\n")
    
    counter *= 2
    printf("120 *= 2 = ")
    printf(counter)
    printf("\n")
    
    counter /= 4
    printf("240 /= 4 = ")
    printf(counter)
    printf("\n")
    
    // else if chains
    int score = 85
    printf("\nScore ")
    printf(score)
    if score >= 90 {
        printf(" = A\n")
    } else if score >= 80 {
        printf(" = B\n")
    } else if score >= 70 {
        printf(" = C\n")
    } else {
        printf(" = F\n")
    }
    
    // do-while loop
    printf("\ndo-while countdown: ")
    int n = 5
    do {
        printf(n)
        printf(" ")
        n -= 1
    } while n > 0
    printf("\n")
}

// ---- Función principal ----
int main() {
    printf("=== ADead-BIB v3.4 - Tipos C ===\n\n")
    
    // Variables con tipos
    int i = 42
    printf("int i = ")
    printf(i)
    printf("\n")
    
    int l = 100000
    printf("long l = ")
    printf(l)
    printf("\n")
    
    // Llamar funciones
    int resultado = add(10, 20)
    printf("\nadd(10, 20) = ")
    printf(resultado)
    printf("\n")
    
    int producto = multiply(5, 6)
    printf("multiply(5, 6) = ")
    printf(producto)
    printf("\n")
    
    int fact5 = factorial(5)
    printf("factorial(5) = ")
    printf(fact5)
    printf("\n")
    
    // Demos
    control_flow_demo()
    math_demo()
    new_features_demo()
    
    printf("\n=== Compilado sin NASM, sin LLVM ===\n")
    return 0
}
