// ADead-BIB v3.4 - Condiciones y Control de Flujo
// if / else if / else sin paréntesis en condición

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

    // else if chain
    printf("\nClasificacion de x:\n")
    if x < 0 {
        printf("x es negativo\n")
    } else if x == 0 {
        printf("x es cero\n")
    } else if x < 10 {
        printf("x es pequeño\n")
    } else {
        printf("x es grande\n")
    }

    return 0
}
