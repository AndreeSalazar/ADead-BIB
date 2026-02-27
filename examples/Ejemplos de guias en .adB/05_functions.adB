// ADead-BIB v3.4 - Funciones y Recursión
// Funciones C-style con tipos explícitos

// Sin parámetros
void saludar() {
    printf("Hola desde una funcion!\n")
}

// Con parámetros
void mostrar(int n) {
    printf("El numero es: ")
    printf(n)
    printf("\n")
}

// Con retorno
int sumar(int a, int b) {
    return a + b
}

int multiplicar(int x, int y) {
    return x * y
}

// Recursión
int factorial(int n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

// Función como parámetro de cálculo (valor de retorno usado)
int cuadrado(int x) {
    return x * x
}

int main() {
    printf("=== Funciones ===\n\n")

    saludar()
    mostrar(42)

    int s = sumar(10, 20)
    printf("10 + 20 = ")
    printf(s)
    printf("\n")

    int p = multiplicar(5, 6)
    printf("5 * 6 = ")
    printf(p)
    printf("\n")

    printf("\nRecursion:\n")
    int i = 1
    while i <= 6 {
        printf("factorial(")
        printf(i)
        printf(") = ")
        printf(factorial(i))
        printf("\n")
        i += 1
    }

    printf("\nCuadrados:\n")
    int j = 1
    while j <= 5 {
        printf("cuadrado(")
        printf(j)
        printf(") = ")
        printf(cuadrado(j))
        printf("\n")
        j += 1
    }

    return 0
}
