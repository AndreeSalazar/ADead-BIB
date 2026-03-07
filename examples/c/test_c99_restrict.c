#include <stdio.h>
void vector_add(int *a, int *b, int *c, int n) {
    for (int i = 0; i < n; i++) c[i] = a[i] + b[i];
}
void scalar_mul(int *arr, int scalar, int n) {
    for (int i = 0; i < n; i++) arr[i] *= scalar;
}
void copy_array(int *dst, const int *src, int n) {
    for (int i = 0; i < n; i++) dst[i] = src[i];
}
int main() {
    int a[4]; a[0]=1; a[1]=2; a[2]=3; a[3]=4;
    int b[4]; b[0]=10; b[1]=20; b[2]=30; b[3]=40;
    int c[4];
    vector_add(a, b, c, 4);
    printf("vadd: %d %d %d %d\n", c[0], c[1], c[2], c[3]);
    scalar_mul(a, 3, 4);
    printf("smul: %d %d %d %d\n", a[0], a[1], a[2], a[3]);
    int d[4];
    copy_array(d, c, 4);
    printf("copy: %d %d %d %d\n", d[0], d[1], d[2], d[3]);
    return 0;
}