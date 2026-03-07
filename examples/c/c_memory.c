#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main() { int *arr = (int*)malloc(10 * sizeof(int)); for(int i = 0; i < 10; i++) arr[i] = i * i;
arr = (int*)realloc(arr, 20 * sizeof(int)); void *z = calloc(5, sizeof(int)); memset(z, 0, 20);
printf("arr[5]=%d\n", arr[5]); free(arr); free(z); return 0; }