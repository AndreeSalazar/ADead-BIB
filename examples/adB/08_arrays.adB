// ADead-BIB v3.4 - Arrays
// Declaración y uso de arrays con tipo e índice

int main() {
    printf("=== Arrays ===\n")

    // Declaración de array con tipo y tamaño
    int arr[5] = [10, 20, 30, 40, 50]

    // Acceso por índice
    printf("Array de 5 elementos:\n")
    int i = 0
    while i < 5 {
        printf("  arr[")
        printf(i)
        printf("] = ")
        printf(arr[i])
        printf("\n")
        i += 1
    }

    // Suma de elementos con loop
    int suma = 0
    int j = 0
    while j < 5 {
        suma += arr[j]
        j += 1
    }
    printf("Suma total: ")
    printf(suma)
    printf("\n")

    // Modificar elemento
    arr[2] = 99
    printf("\nDespues de arr[2] = 99:\n")
    printf("  arr[2] = ")
    printf(arr[2])
    printf("\n")

    return 0
}
