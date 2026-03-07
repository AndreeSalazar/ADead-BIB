#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int binary_search(int *arr, int n, int target) { int lo=0,hi=n-1; while(lo<=hi){int m=lo+(hi-lo)/2; if(arr[m]==target)return m; if(arr[m]<target)lo=m+1; else hi=m-1;} return -1; }
void bubble_sort(int *arr, int n) { for(int i=0;i<n-1;i++) for(int j=0;j<n-i-1;j++) if(arr[j]>arr[j+1]){int t=arr[j];arr[j]=arr[j+1];arr[j+1]=t;} }
int gcd(int a,int b) { while(b){int t=b;b=a%b;a=t;} return a; }
int fibonacci(int n) { if(n<=1)return n; int a=0,b=1; for(int i=2;i<=n;i++){int t=a+b;a=b;b=t;} return b; }
int main() { int arr[5]; arr[0]=3;arr[1]=1;arr[2]=4;arr[3]=1;arr[4]=5; bubble_sort(arr,5); printf("gcd=%d fib=%d\n",gcd(12,8),fibonacci(10)); return 0; }