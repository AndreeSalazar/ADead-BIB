// ADead-BIB v3.4 - Operadores
// Aritméticos, compound assignment, bitwise, relacionales

int main() {
    printf("=== Operadores ===\n")

    int a = 10
    int b = 3

    // Aritméticos
    printf("Aritmeticos (a=10, b=3):\n")
    printf("  a + b = ")
    printf(a + b)
    printf("\n")

    printf("  a - b = ")
    printf(a - b)
    printf("\n")

    printf("  a * b = ")
    printf(a * b)
    printf("\n")

    printf("  a / b = ")
    printf(a / b)
    printf("\n")

    printf("  a %% b = ")
    printf(a % b)
    printf("\n")

    // Compound assignment
    printf("\nCompound Assignment:\n")
    int x = 10
    x += 5
    printf("  10 += 5 -> ")
    printf(x)
    printf("\n")

    x -= 3
    printf("  15 -= 3 -> ")
    printf(x)
    printf("\n")

    x *= 2
    printf("  12 *= 2 -> ")
    printf(x)
    printf("\n")

    x /= 4
    printf("  24 /= 4 -> ")
    printf(x)
    printf("\n")

    x %= 4
    printf("  6 %%= 4 -> ")
    printf(x)
    printf("\n")

    // Bitwise
    printf("\nBitwise (a=10, b=3):\n")
    printf("  a & b  = ")
    printf(a & b)
    printf("\n")

    printf("  a | b  = ")
    printf(a | b)
    printf("\n")

    printf("  a ^ b  = ")
    printf(a ^ b)
    printf("\n")

    printf("  a << 1 = ")
    printf(a << 1)
    printf("\n")

    printf("  a >> 1 = ")
    printf(a >> 1)
    printf("\n")

    return 0
}
