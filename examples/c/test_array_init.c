#include <stdio.h>
int sum(int arr[], int n) { int s=0; for(int i=0;i<n;i++) s+=arr[i]; return s; }
int main() { int a[] = {1,2,3,4,5}; int b[3]; b[0]=10;b[1]=20;b[2]=30; printf("sa=%d sb=%d\n",sum(a,5),sum(b,3)); return 0; }