#include <stdio.h>

int main() {
    printf("START\n");
    int a = 1;
    int b = 2;
    int c = 3;
    int d = 4;
    int e = 5;
    int f = 6;
    int g = 7;
    int h = 8;
    int i = 9;
    int j = 10;
    int k = 11;
    int l = 12;
    int m = 13;
    int n = 14;
    int o = 15;
    int p = 16;
    int q = 17;
    int r = 18;
    int s = 19;
    int t = 20;
    int sum = a+b+c+d+e+f+g+h+i+j+k+l+m+n+o+p+q+r+s+t;
    printf("sum=%d (expected 210)\n", sum);
    
    int arr1[] = {5, 3, 8, 1, 9, 2, 7, 4, 6, 10};
    int arr2[] = {3, 1, 4, 1, 5, 9, 2, 6};
    int v0 = arr1[0];
    int v9 = arr1[9];
    printf("arr1[0]=%d(5) arr1[9]=%d(10)\n", v0, v9);
    printf("DONE\n");
    return 0;
}
