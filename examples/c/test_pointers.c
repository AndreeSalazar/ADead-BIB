#include <stdio.h>
void increment(int *p) { *p = *p + 1; }
int main() { int x = 10; increment(&x); printf("x=%d\n", x); int arr[3]; arr[0]=10;arr[1]=20;arr[2]=30; printf("a1=%d\n",arr[1]); return 0; }