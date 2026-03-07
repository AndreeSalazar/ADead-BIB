#include <header_main.h>
int main() {
    printf("C libre\n");
    int arr[5];
    arr[0] = 1; arr[1] = 2; arr[2] = 3; arr[3] = 4; arr[4] = 5;
    printf("%d\n", arr[4]);
    double s = sin(3.14159 / 6.0);
    printf("sin(30)=%.4f\n", s);
    void *p = malloc(1024);
    free(p);
    printf("malloc/free ok\n");
    return 0;
}