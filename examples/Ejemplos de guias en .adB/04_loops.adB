// ADead-BIB v3.4 - Loops (while, do-while, break, continue)
// Estructuras de repetición — sin paréntesis en condición

int main() {
    printf("=== Loops ===\n")

    // While con compound assignment (+=)
    printf("While loop (0-4):\n")
    int i = 0
    while i < 5 {
        printf("  i = ")
        printf(i)
        printf("\n")
        i += 1
    }

    // While con paso de 2
    printf("\nWhile paso 2 (0, 2, 4, 6, 8):\n")
    int j = 0
    while j < 10 {
        printf("  j = ")
        printf(j)
        printf("\n")
        j += 2
    }

    // do-while: ejecuta al menos una vez
    printf("\ndo-while (countdown 3..1):\n")
    int k = 3
    do {
        printf("  k = ")
        printf(k)
        printf("\n")
        k -= 1
    } while k > 0

    // while con break
    printf("\nWhile con break en i==3:\n")
    int n = 0
    while n < 10 {
        if n == 3 {
            break
        }
        printf("  n = ")
        printf(n)
        printf("\n")
        n += 1
    }

    return 0
}
