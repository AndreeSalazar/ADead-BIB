// ============================================================
// Test 07: Arrays Básicos — 1D, 2D, inicialización, iteración
// ============================================================
// ADead-BIB Test Canon — C99 §6.7.6.2
// Verifica: declaración, init, acceso, strings, multidimensional
// ============================================================

#include <stdio.h>

int sum_array(int arr[], int n) {
    int sum = 0;
    int i;
    for (i = 0; i < n; i++) {
        sum += arr[i];
    }
    return sum;
}

int find_max(int arr[], int n) {
    int max = arr[0];
    int i;
    for (i = 1; i < n; i++) {
        if (arr[i] > max) max = arr[i];
    }
    return max;
}

int find_min(int arr[], int n) {
    int min = arr[0];
    int i;
    for (i = 1; i < n; i++) {
        if (arr[i] < min) min = arr[i];
    }
    return min;
}

void reverse_array(int arr[], int n) {
    int i;
    for (i = 0; i < n / 2; i++) {
        int tmp = arr[i];
        arr[i] = arr[n - 1 - i];
        arr[n - 1 - i] = tmp;
    }
}

void bubble_sort(int arr[], int n) {
    int i, j;
    for (i = 0; i < n - 1; i++) {
        for (j = 0; j < n - 1 - i; j++) {
            if (arr[j] > arr[j + 1]) {
                int tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
            }
        }
    }
}

int main() {
    // --- Array 1D con inicialización ---
    int data[5] = {10, 20, 30, 40, 50};
    printf("sum=%d max=%d min=%d\n", sum_array(data, 5), find_max(data, 5), find_min(data, 5));

    // --- Array sin tamaño explícito ---
    int auto_arr[] = {5, 3, 1, 4, 2};
    int auto_sz = sizeof(auto_arr) / sizeof(auto_arr[0]);
    printf("auto_sz=%d\n", auto_sz);

    // --- Reverse ---
    reverse_array(auto_arr, auto_sz);
    printf("reversed: %d %d %d %d %d\n",
           auto_arr[0], auto_arr[1], auto_arr[2], auto_arr[3], auto_arr[4]);

    // --- Sort ---
    int unsorted[] = {64, 25, 12, 22, 11};
    bubble_sort(unsorted, 5);
    printf("sorted: %d %d %d %d %d\n",
           unsorted[0], unsorted[1], unsorted[2], unsorted[3], unsorted[4]);

    // --- Array 2D ---
    int matrix[3][3] = {
        {1, 2, 3},
        {4, 5, 6},
        {7, 8, 9}
    };
    int trace = matrix[0][0] + matrix[1][1] + matrix[2][2];
    printf("trace=%d\n", trace);

    // --- Array 2D iteración ---
    int sum2d = 0;
    int i, j;
    for (i = 0; i < 3; i++) {
        for (j = 0; j < 3; j++) {
            sum2d += matrix[i][j];
        }
    }
    printf("sum2d=%d\n", sum2d);

    // --- Array de chars (string) ---
    char name[] = "ADead-BIB";
    int len = 0;
    while (name[len] != '\0') len++;
    printf("name=%s len=%d\n", name, len);

    // --- Array parcialmente inicializado ---
    int partial[10] = {1, 2, 3};
    printf("partial[0]=%d partial[3]=%d partial[9]=%d\n",
           partial[0], partial[3], partial[9]);

    return 0;
}
// Expected:
// sum=150 max=50 min=10
// auto_sz=5
// reversed: 2 4 1 3 5
// sorted: 11 12 22 25 64
// trace=15
// sum2d=45
// name=ADead-BIB len=9
// partial[0]=1 partial[3]=0 partial[9]=0
