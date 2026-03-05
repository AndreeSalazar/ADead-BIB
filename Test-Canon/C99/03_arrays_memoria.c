// ============================================================
// Canon C99 — §6.5.2.1 Arrays y Memoria Contigua
// ============================================================
// Intención: Un array es memoria contigua. arr[i] equivale
// exactamente a *(arr + i). No hay bounds checking,
// no hay abstracción — es acceso directo a memoria.
//
// C99 §6.5.2.1: "The definition of the subscript operator []
// is that E1[E2] is identical to (*((E1)+(E2)))"
// ============================================================

#include <stdio.h>

// --- Funciones que operan sobre arrays ---

int array_sum(int arr[], int len) {
    int total = 0;
    int i;
    for (i = 0; i < len; i++) {
        total = total + arr[i];
    }
    return total;
}

int array_max(int arr[], int len) {
    int max = arr[0];
    int i;
    for (i = 1; i < len; i++) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}

int array_min(int arr[], int len) {
    int min = arr[0];
    int i;
    for (i = 1; i < len; i++) {
        if (arr[i] < min) {
            min = arr[i];
        }
    }
    return min;
}

void array_reverse(int arr[], int len) {
    int i = 0;
    int j = len - 1;
    while (i < j) {
        int temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
        i++;
        j--;
    }
}

void bubble_sort(int arr[], int len) {
    int i;
    int j;
    for (i = 0; i < len - 1; i++) {
        for (j = 0; j < len - 1 - i; j++) {
            if (arr[j] > arr[j + 1]) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

int linear_search(int arr[], int len, int target) {
    int i;
    for (i = 0; i < len; i++) {
        if (arr[i] == target) {
            return i;
        }
    }
    return -1;
}

void print_array(int arr[], int len) {
    printf("  [");
    int i;
    for (i = 0; i < len; i++) {
        printf("%d", arr[i]);
        if (i < len - 1) printf(", ");
    }
    printf("]\n");
}

int main() {
    printf("=== Canon C99: Arrays y Memoria ===\n\n");

    // --- Array con inicialización manual ---
    int nums[6];
    nums[0] = 42;
    nums[1] = 17;
    nums[2] = 89;
    nums[3] = 3;
    nums[4] = 55;
    nums[5] = 21;

    printf("Array original:");
    print_array(nums, 6);

    // --- Operaciones ---
    printf("sum = %d\n", array_sum(nums, 6));
    printf("max = %d\n", array_max(nums, 6));
    printf("min = %d\n", array_min(nums, 6));

    // --- Búsqueda ---
    int idx = linear_search(nums, 6, 89);
    printf("search(89) = index %d\n", idx);
    int not_found = linear_search(nums, 6, 999);
    printf("search(999) = index %d\n", not_found);

    // --- Reverse ---
    array_reverse(nums, 6);
    printf("Reversed:");
    print_array(nums, 6);

    // --- Sort ---
    bubble_sort(nums, 6);
    printf("Sorted:");
    print_array(nums, 6);

    // --- Array de caracteres (string manual) ---
    char word[4];
    word[0] = 'A';
    word[1] = 'B';
    word[2] = 'C';
    word[3] = '\0';
    printf("\nchar array: %s\n", word);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (array_sum(nums, 6) == 227)    { pass++; } else { printf("FAIL: sum\n"); }
    total++; if (array_max(nums, 6) == 89)     { pass++; } else { printf("FAIL: max\n"); }
    total++; if (array_min(nums, 6) == 3)      { pass++; } else { printf("FAIL: min\n"); }
    total++; if (nums[0] == 3)                 { pass++; } else { printf("FAIL: sorted[0]\n"); }
    total++; if (nums[5] == 89)                { pass++; } else { printf("FAIL: sorted[5]\n"); }
    total++; if (idx == 2)                     { pass++; } else { printf("FAIL: search\n"); }
    total++; if (not_found == -1)              { pass++; } else { printf("FAIL: not found\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
