#include <stdio.h>
int sum3(int a, int b, int c) { return a + b + c; }
int sum5(int a, int b, int c, int d, int e) { return a + b + c + d + e; }
int max3(int a, int b, int c) {
    int m = a;
    if (b > m) m = b;
    if (c > m) m = c;
    return m;
}
int min3(int a, int b, int c) {
    int m = a;
    if (b < m) m = b;
    if (c < m) m = c;
    return m;
}
void print_many(const char *a, const char *b, const char *c) {
    printf("%s %s %s\n", a, b, c);
}
int main() {
    printf("sum3(1,2,3) = %d\n", sum3(1, 2, 3));
    printf("sum5(1,2,3,4,5) = %d\n", sum5(1, 2, 3, 4, 5));
    printf("max3(3,7,2) = %d\n", max3(3, 7, 2));
    printf("min3(3,7,2) = %d\n", min3(3, 7, 2));
    print_many("hello", "from", "variadics");
    return 0;
}