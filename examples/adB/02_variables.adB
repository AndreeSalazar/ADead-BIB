// ADead-BIB v3.4 - Variables y Tipos
// Tipos C explícitos: int, long, float, double, bool, char

int main() {
    printf("=== Variables y Tipos ===\n")

    // Enteros con tipos C explícitos
    int x = 42
    int y = 10
    long l = 100000
    char c = 65      // 'A' en ASCII

    printf("int  x = ")
    printf(x)
    printf("\n")

    printf("int  y = ")
    printf(y)
    printf("\n")

    printf("long l = ")
    printf(l)
    printf("\n")

    printf("char c = ")
    printf(c)
    printf("\n")

    // Operaciones
    int suma     = x + y
    int resta    = x - y
    int producto = x * y
    int division = x / y
    int modulo   = x % y

    printf("\nx + y = ")
    printf(suma)
    printf("\n")

    printf("x - y = ")
    printf(resta)
    printf("\n")

    printf("x * y = ")
    printf(producto)
    printf("\n")

    printf("x / y = ")
    printf(division)
    printf("\n")

    printf("x %% y = ")
    printf(modulo)
    printf("\n")

    return 0
}
