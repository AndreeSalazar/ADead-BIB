#include <stdio.h>
int add(int a, int b) { return a+b; }
int sub(int a, int b) { return a-b; }
int mul(int a, int b) { return a*b; }
int div_i(int a, int b) { return a/b; }
int mod_i(int a, int b) { return a%b; }
int neg(int a) { return -a; }
int apply_twice(int (*f)(int,int), int a, int b) { return f(f(a,b),b); }
int main() { printf("%d %d %d %d %d %d\n",add(1,2),sub(5,3),mul(3,4),div_i(10,3),mod_i(10,3),neg(7)); return 0; }