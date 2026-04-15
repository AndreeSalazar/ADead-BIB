// ============================================================
// Test 12: Aritmética de Punteros — p+n, p-n, p-q, array via ptr
// ============================================================
// ADead-BIB Test Canon — C99 §6.5.6
// Verifica: incremento, decremento, distancia, arrays como ptrs
// ============================================================

#include <stdio.h>

int sum_via_ptr(int *arr, int n) {
    int sum = 0;
    int *end = arr + n;
    while (arr < end) {
        sum += *arr;
        arr++;
    }
    return sum;
}

void fill_sequence(int *arr, int n) {
    int i;
    for (i = 0; i < n; i++) {
        *(arr + i) = i * i;
    }
}

int count_char(const char *s, char c) {
    int count = 0;
    while (*s) {
        if (*s == c) count++;
        s++;
    }
    return count;
}

int main() {
    // --- Array como puntero ---
    int arr[5] = {10, 20, 30, 40, 50};
    int *p = arr;

    printf("arr[0]=%d *p=%d\n", arr[0], *p);
    printf("arr[2]=%d *(p+2)=%d\n", arr[2], *(p + 2));
    printf("arr[4]=%d *(p+4)=%d\n", arr[4], *(p + 4));

    // --- Incremento ---
    p = arr;
    printf("*p=%d ", *p);
    p++;
    printf("*p++=%d ", *p);
    p++;
    printf("*p++=%d\n", *p);

    // --- Decremento ---
    p = arr + 4;
    printf("*p=%d ", *p);
    p--;
    printf("*p--=%d ", *p);
    p--;
    printf("*p--=%d\n", *p);

    // --- Distancia entre punteros ---
    int *start = &arr[0];
    int *end = &arr[4];
    long diff = end - start;
    printf("distance=%ld\n", diff);

    // --- Comparación de punteros ---
    printf("start<end=%d start==start=%d\n", (start < end), (start == start));

    // --- Sum via pointer iteration ---
    printf("sum=%d\n", sum_via_ptr(arr, 5));

    // --- Fill via pointer offset ---
    int squares[5];
    fill_sequence(squares, 5);
    printf("squares: %d %d %d %d %d\n",
           squares[0], squares[1], squares[2], squares[3], squares[4]);

    // --- p[i] es *(p+i) ---
    p = arr;
    printf("p[0]=%d p[1]=%d p[2]=%d\n", p[0], p[1], p[2]);
    printf("0[p]=%d 1[p]=%d 2[p]=%d\n", 0[p], 1[p], 2[p]);

    // --- Char pointer arithmetic ---
    const char *msg = "hello world";
    printf("count_l=%d count_o=%d\n", count_char(msg, 'l'), count_char(msg, 'o'));

    // --- Pointer difference en bytes (sizeof awareness) ---
    char ca[10] = {0};
    char *cp1 = &ca[0];
    char *cp2 = &ca[5];
    long char_diff = cp2 - cp1;
    printf("char_diff=%ld int_elem_size=%d\n", char_diff, (int)sizeof(int));

    return 0;
}
// Expected:
// arr[0]=10 *p=10
// arr[2]=30 *(p+2)=30
// arr[4]=50 *(p+4)=50
// *p=10 *p++=20 *p++=30
// *p=50 *p--=40 *p--=30
// distance=4
// start<end=1 start==start=1
// sum=150
// squares: 0 1 4 9 16
// p[0]=10 p[1]=20 p[2]=30
// 0[p]=10 1[p]=20 2[p]=30
// count_l=3 count_o=2
// char_diff=5 int_elem_size=4
