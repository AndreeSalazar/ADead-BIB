// ============================================================
// ADead-BIB — MODO 2: Aplicación con Tipado C+Python
// ============================================================
// Este modo genera ejecutables PE/ELF con sintaxis tipada.
// Ideal para: aplicaciones, herramientas, scripts compilados.
//
// Compilar Windows: adeadc MODE2_app_typed.adB -o app.exe
// Compilar Linux:   adeadc MODE2_app_typed.adB -o app --linux
//
// Características:
//   - Tipos C: int, char, short, long, float, double
//   - Punteros: int*, char*, void*
//   - Arrays: int arr[10]
//   - Structs y clases
//   - Funciones con tipos de retorno
//   - Control de flujo: if, while, for, switch
// ============================================================

// ---- Tipos Básicos ----
int main() {
    // Enteros con tamaño explícito
    char c = 65           // 1 byte  (I8)
    short s = 1000        // 2 bytes (I16)
    int i = 42            // 4 bytes (I32)
    long l = 100000       // 8 bytes (I64)
    
    // Flotantes
    float f = 3.14        // 4 bytes (F32)
    double d = 2.71828    // 8 bytes (F64)
    
    // Booleano
    bool flag = true      // 1 byte
    
    printf("=== ADead-BIB Tipos ===\n")
    printf("char c = ")
    printf(c)
    printf("\n")
    
    printf("int i = ")
    printf(i)
    printf("\n")
    
    printf("long l = ")
    printf(l)
    printf("\n")
    
    return 0
}

// ---- Función con tipos ----
int add(int a, int b) {
    return a + b
}

int factorial(int n) {
    if (n <= 1) {
        return 1
    }
    return n * factorial(n - 1)
}

// ---- Punteros ----
void swap(int* a, int* b) {
    int temp = *a
    *a = *b
    *b = temp
}

int* find_max(int* arr, int len) {
    int* max_ptr = arr
    for (int i = 1; i < len; i = i + 1) {
        if (arr[i] > *max_ptr) {
            max_ptr = &arr[i]
        }
    }
    return max_ptr
}

// ---- Arrays ----
void array_demo() {
    int nums[5] = {10, 20, 30, 40, 50}
    
    printf("Array: ")
    for (int i = 0; i < 5; i = i + 1) {
        printf(nums[i])
        printf(" ")
    }
    printf("\n")
    
    // Suma de array
    int sum = 0
    for (int i = 0; i < 5; i = i + 1) {
        sum = sum + nums[i]
    }
    printf("Suma: ")
    printf(sum)
    printf("\n")
}

// ---- Operadores Bitwise ----
void bitwise_demo() {
    int a = 0b1100    // 12
    int b = 0b1010    // 10
    
    printf("\nBitwise Operations:\n")
    
    int and_r = a & b   // 8
    int or_r = a | b    // 14
    int xor_r = a ^ b   // 6
    int shl_r = a << 2  // 48
    int shr_r = a >> 2  // 3
    int not_r = ~a      // -13
    
    printf("12 & 10 = ")
    printf(and_r)
    printf("\n")
    
    printf("12 | 10 = ")
    printf(or_r)
    printf("\n")
    
    printf("12 ^ 10 = ")
    printf(xor_r)
    printf("\n")
    
    printf("12 << 2 = ")
    printf(shl_r)
    printf("\n")
    
    printf("12 >> 2 = ")
    printf(shr_r)
    printf("\n")
}

// ---- Control de Flujo ----
void control_flow_demo() {
    printf("\nControl Flow:\n")
    
    // If-else
    int x = 10
    if (x > 5) {
        printf("x > 5\n")
    } else {
        printf("x <= 5\n")
    }
    
    // While
    int count = 0
    while (count < 3) {
        printf("count = ")
        printf(count)
        printf("\n")
        count = count + 1
    }
    
    // For
    printf("For loop: ")
    for (int i = 0; i < 5; i = i + 1) {
        printf(i)
        printf(" ")
    }
    printf("\n")
}
