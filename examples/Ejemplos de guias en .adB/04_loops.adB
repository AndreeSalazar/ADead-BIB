// ADead-BIB v3.1 - Loops (For, While, Do-While)
// Estructuras de repetición

int main() {
    printf("=== Loops ===\n")
    
    // While loop
    printf("While loop (0-4):\n")
    int i = 0
    while i < 5 {
        printf("  i = ")
        printf(i)
        printf("\n")
        i = i + 1
    }
    
    // For loop (usando while internamente)
    printf("For loop (0-2):\n")
    int j = 0
    while j < 3 {
        printf("  j = ")
        printf(j)
        printf("\n")
        j = j + 1
    }
    
    // Contador con incremento
    printf("Contador con += :\n")
    int k = 0
    while k < 10 {
        k = k + 2
    }
    printf("  k final = ")
    printf(k)
    printf("\n")
    
    return 0
}
